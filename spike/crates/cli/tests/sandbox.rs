//! The throwaway per-user profile a driven binary's keyset and receipts land in.
//!
//! Its own module, not a corner of `support`: three test binaries want a sandbox and only one of
//! them wants the case-discovery reporter beside it, so sharing one module would drag that
//! reporter's `eprintln!`s into targets with no business granting them.

#![allow(
    dead_code,
    reason = "one shared module, three test binaries: each uses only the accessors it needs, so `expect` would go unfulfilled in the ones that use fewer"
)]

use std::path::PathBuf;

/// A throwaway per-user profile: where a driven binary's keyset and receipts land.
///
/// Nothing Dorc-specific selects it. Production resolves the PLATFORM's own variables, so a
/// sandbox is made by setting exactly those and nothing else — which is what keeps the resolution
/// under test the resolution that ships (`30Rd`'s test-and-fixture fence: there is no
/// environment variable that names a fixture provider, key, store, or weaker policy).
///
/// The two ROLES stay separate directories here as they are on Windows and Linux, so a case that
/// copies state without configuration copies what an operator would.
pub(crate) struct ProfileSandbox {
    root: PathBuf,
}

impl ProfileSandbox {
    /// A fresh sandbox for this process, named for the target that made it.
    ///
    /// The lint allowance rides the FUNCTION rather than a crate preamble: this module is shared
    /// by four test binaries, three of which grant nothing, and a sandbox that could not be made
    /// is a harness fault worth a loud panic rather than a drive against whoever ran the suite.
    #[expect(
        clippy::expect_used,
        reason = "a sandbox that cannot be created must stop the target, not degrade it"
    )]
    pub(crate) fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!("dorc-profile-{name}-{}", std::process::id()));
        // A leftover from a killed run is removed rather than reused: a case asserting what a
        // CLEAN profile does would otherwise quietly assert something else.
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("config")).expect("a sandbox configuration base");
        std::fs::create_dir_all(root.join("state")).expect("a sandbox state base");
        Self { root }
    }

    /// Where a keyset would land.
    pub(crate) fn config_root(&self) -> PathBuf {
        self.root.join("config")
    }

    /// Where receipts would land.
    pub(crate) fn state_root(&self) -> PathBuf {
        self.root.join("state")
    }

    /// Point one invocation's standard roots at this sandbox.
    ///
    /// `HOME` is set as well as the XDG pair, because macOS resolves both roles from it and a
    /// drive that left it inherited would write into whoever ran the suite.
    pub(crate) fn apply(&self, command: &mut std::process::Command) {
        apply_roots_under(command, &self.root);
    }
}

/// Point one invocation's standard roots at `root`'s config/state pair.
///
/// Separate from [`ProfileSandbox`] so a caller can own a throwaway profile without owning a
/// self-removing value — a case materialized into a scratch dir already has a lifetime.
///
/// `HOME` is set as well as the XDG pair, because macOS resolves both roles from it and a drive
/// that left it inherited would write into whoever ran the suite.
pub(crate) fn apply_roots_under(command: &mut std::process::Command, root: &std::path::Path) {
    for key in ["APPDATA", "XDG_CONFIG_HOME"] {
        command.env(key, root.join("config"));
    }
    for key in ["LOCALAPPDATA", "XDG_STATE_HOME"] {
        command.env(key, root.join("state"));
    }
    command.env("HOME", root.join("home"));
}

impl Drop for ProfileSandbox {
    /// Take the sandbox with us. Default-on means every drive leaves a receipt, and a suite that
    /// grows a fresh litter per run is one nobody wants to keep running.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}
