//! The controller roots: where a keyset and a store live, resolved once and never again.
//!
//! # Values in, never an environment read here
//!
//! This crate does not read the environment. Root resolution happens at the process edge, which
//! hands the answers in as [`RootInputs`]; everything below works from that value. That is what
//! keeps the whole crate drivable by a deterministic model — and it is also the honest shape,
//! because which variables to read is a platform question the edge already has to answer.
//!
//! # Two roles, and why they may share a path
//!
//! Configuration and state are separate ROLES: keys live under one, receipts under the other, so
//! copying state without configuration preserves the intended separation. On macOS both roles
//! resolve to one application-support directory, and the versioned subdirectories preserve the
//! role without claiming a path or backup separation they do not have. The type keeps them
//! distinct even where the paths coincide, so no caller can reach for "the root".

/// Which platform's standard locations an edge resolved against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootPlatform {
    /// `%APPDATA%` and `%LOCALAPPDATA%`.
    Windows,
    /// One application-support path serving both roles.
    MacOs,
    /// `$XDG_CONFIG_HOME` / `$XDG_STATE_HOME`, or their standard fallbacks.
    OtherUnix,
}

impl RootPlatform {
    /// Which honest baseline this platform's objects are validated against.
    ///
    /// macOS is grouped with the other Unixes because what differs there is where the roots LAND,
    /// not what the filesystem can be asked. The Windows arm is explicitly weaker and stays a
    /// separate answer rather than being folded into a portable one that is true nowhere.
    #[must_use]
    pub const fn baseline(self) -> crate::store::PlatformBaseline {
        match self {
            Self::Windows => crate::store::PlatformBaseline::Windows,
            Self::MacOs | Self::OtherUnix => crate::store::PlatformBaseline::UnixLike,
        }
    }
}

/// The two role-typed platform BASES an edge resolved, before validation.
///
/// Bases rather than product roots: the edge answers where a platform keeps per-user
/// configuration and state, and every component below that is this crate's, fixed and typed. That
/// is what makes `30Rd`'s "only fixed, typed, single-component internal names beneath the
/// landing" true from the very top rather than from wherever a caller happened to stop joining.
///
/// Deliberately not a filesystem type: this crate holds what it was told, and turning either into
/// a validated location is the local I/O layer's act, under its own refusals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootInputs {
    platform: RootPlatform,
    configuration: String,
    state: String,
}

/// Why an edge could not present usable roots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootRefusal {
    /// A required base was absent or empty.
    ///
    /// There is no fallback to a working directory, a repository, a temporary directory, a cache
    /// directory, or the other role's root. A run without a place to put its durables says so.
    ControllerRootUnavailable {
        /// Which role.
        role: RootRole,
    },
    /// A base that was not absolute. A relative root would move with the process's own cwd, which
    /// a book can change.
    NotAbsolute {
        /// Which role.
        role: RootRole,
    },
}

/// Which of the two role-typed roots a value belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RootRole {
    /// Where private key material lives.
    Configuration,
    /// Where published receipts live.
    State,
}

impl RootRole {
    /// Both roles.
    pub const ALL: [Self; 2] = [Self::Configuration, Self::State];
}

impl RootInputs {
    /// Bind two resolved roots.
    ///
    /// # Errors
    /// Refuses an absent or non-absolute base, naming the role, so a caller can say which of the
    /// two the platform could not answer.
    pub fn of(
        platform: RootPlatform,
        configuration: &str,
        state: &str,
    ) -> Result<Self, RootRefusal> {
        for (role, text) in [
            (RootRole::Configuration, configuration),
            (RootRole::State, state),
        ] {
            if text.is_empty() {
                return Err(RootRefusal::ControllerRootUnavailable { role });
            }
            if !is_absolute(text) {
                return Err(RootRefusal::NotAbsolute { role });
            }
        }
        Ok(Self {
            platform,
            configuration: configuration.to_owned(),
            state: state.to_owned(),
        })
    }

    /// The platform these were resolved against.
    #[must_use]
    pub const fn platform(&self) -> RootPlatform {
        self.platform
    }

    /// The platform base for `role`, as the edge answered it.
    #[must_use]
    pub fn base(&self, role: RootRole) -> &str {
        match role {
            RootRole::Configuration => &self.configuration,
            RootRole::State => &self.state,
        }
    }

    /// The product root for `role`: the base, plus this project's one fixed component.
    ///
    /// The single component the bootstrap protocol may have to create, and the landing every
    /// later child is taken relative to.
    #[must_use]
    pub fn product_root(&self, role: RootRole) -> Option<crate::names::LocalPath> {
        crate::names::LocalPath::of_root(self.platform, self.base(role))
            .child(crate::names::PRODUCT_DIR)
    }
}

/// Whether a spelling is absolute on either family.
///
/// Both families are admitted from either platform arm, because a test drives a Windows-shaped
/// edge from a Unix host and the reverse; what is refused is a spelling that is absolute on
/// NEITHER, which is the only case that could move with a process's own working directory.
fn is_absolute(text: &str) -> bool {
    if text.starts_with('/') || text.starts_with("\\\\") {
        return true;
    }
    let mut bytes = text.bytes();
    match (bytes.next(), bytes.next(), bytes.next()) {
        (Some(drive), Some(b':'), Some(b'\\' | b'/')) => drive.is_ascii_alphabetic(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_root_that_is_absolute_on_neither_family_is_refused() {
        for relative in ["", "dorc", "./dorc", "../dorc", "C:dorc"] {
            let outcome = RootInputs::of(RootPlatform::OtherUnix, relative, "/state");
            assert!(outcome.is_err(), "{relative:?} was admitted");
        }
        assert!(RootInputs::of(RootPlatform::OtherUnix, "/config", "/state").is_ok());
        assert!(RootInputs::of(RootPlatform::Windows, "C:\\config", "D:/state").is_ok());
        assert!(RootInputs::of(RootPlatform::Windows, "\\\\host\\share", "D:/state").is_ok());
    }

    #[test]
    fn an_absent_root_names_the_role_it_is_missing_for() {
        // The refusal has to say which half the platform could not answer: a run with a
        // configuration root and no state root can still verify, and one with neither cannot.
        assert_eq!(
            RootInputs::of(RootPlatform::OtherUnix, "/config", ""),
            Err(RootRefusal::ControllerRootUnavailable {
                role: RootRole::State
            })
        );
        assert_eq!(
            RootInputs::of(RootPlatform::OtherUnix, "", "/state"),
            Err(RootRefusal::ControllerRootUnavailable {
                role: RootRole::Configuration
            })
        );
    }

    #[test]
    fn the_two_roles_stay_distinct_even_where_one_path_serves_both() {
        // The macOS shape. The bases coincide and the roles do not, so nothing may reach for
        // "the root" and get whichever one it happened to want. Below the shared product root the
        // two versioned subdirectories keep the roles apart without claiming a path or backup
        // separation they do not have.
        let shared = "/Users/x/Library/Application Support";
        let roots = RootInputs::of(RootPlatform::MacOs, shared, shared).expect("absolute");
        assert_eq!(roots.base(RootRole::Configuration), shared);
        assert_eq!(roots.base(RootRole::State), shared);
        assert_eq!(
            roots.product_root(RootRole::Configuration),
            roots.product_root(RootRole::State)
        );
        assert_ne!(RootRole::Configuration, RootRole::State);
    }

    #[test]
    fn the_product_root_is_the_base_plus_exactly_one_fixed_component() {
        // The component is this project's and is never supplied, so a base carrying its own
        // trailing name cannot silently become the product root.
        let roots = RootInputs::of(RootPlatform::OtherUnix, "/home/x/.config", "/home/x/.state")
            .expect("absolute");
        let configuration = roots
            .product_root(RootRole::Configuration)
            .expect("one ordinary component");
        assert_eq!(configuration.as_str(), "/home/x/.config/dorc");
        let windows = RootInputs::of(RootPlatform::Windows, "C:\\Roaming", "C:\\Local")
            .expect("absolute")
            .product_root(RootRole::Configuration)
            .expect("one ordinary component");
        assert_eq!(windows.as_str(), "C:\\Roaming\\dorc");
    }
}
