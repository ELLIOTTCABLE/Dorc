//! `core::escalation` — the two INDEPENDENT authority axes of context-entry probing
//! (`27C:rul-two-axis-escalation-consent`, HUMAN-TYPED 2026-07-17). Shared vocabulary (like
//! [`crate::Phase`]) so the cli edge (the admin's dial flag), `hostsim` (the injected host
//! capability), and the decision logic in `dorc_oracle::entry` all name the same two things. The
//! per-dimension decision that composes them lives in `dorc_oracle::entry` (it needs the wrapper
//! `Dimension`); this module holds only the axis vocabularies.
//!
//! The two axes are ORTHOGONAL and must never be collapsed (`27C` §1):
//!
//! 1. **Mechanical capability** ([`Capability`]) — CAN the connection effect the shift at all,
//!    with zero new credentials? A CAPABILITY test, never an identity one. A host fact
//!    (`hostsim`-injected in tests; the cli edge in reality). The probe NEVER self-acquires.
//! 2. **Consent** ([`EscalationDial`]) — GIVEN mechanical-yes, has the admin consented to pointing
//!    escalation machinery at fallible human-authored oracle code? The ternary dial, default YES
//!    for tolerance-vouched functions (the double-ended ack).

/// **Axis 1 — mechanical capability** (`27C` §1(1)): what the connection can mechanically effect
/// with zero new credentials. Capability, NEVER identity (`27C:rule-no-privilege-order`: there is
/// no privilege ordering; the only implementable predicate is "can the connection do it"). A host
/// fact, injected (`hostsim` in tests, the cli edge in reality) — the probe never SELF-acquires (no
/// prompting, no credential handling the user did not pre-establish; the acquisition-UX cell stays
/// deferred, `27C:open-cell-granted-acquire-ux`).
///
/// Coarse by design (the spike floor): the finer "which specific NOPASSWD forms succeed" is a host
/// fact the real cli edge would probe per-form; here [`NonRootNopasswd`](Capability::NonRootNopasswd)
/// stands for "the non-interactive user-dimension forms (`sudo -n`-class) succeed" and the
/// substrate/root-only dimensions (chroot, netns) do not. The per-dimension mapping lives in
/// `dorc_oracle::entry::Capability::permits` (it needs the wrapper `Dimension`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Capability {
    /// The connection is root (or root-equivalent for these mechanisms): every modeled dimension —
    /// user, fs-view (chroot), netns — can be entered. The default spike posture.
    #[default]
    Root,
    /// A non-root connection whose NON-INTERACTIVE user-dimension forms already succeed
    /// (`sudo -n`-class NOPASSWD included — a capability, not a claim about identity). The
    /// root-only substrate dimensions (chroot, `ip netns exec`) still cannot be entered.
    NonRootNopasswd,
    /// A bare non-root connection: no shift can be mechanically effected. Degraded mode — every
    /// wrapped site degrades to guard/run (`27C:hole-static-identity`, best-effort tier by ruling).
    Degraded,
}

/// **Axis 2 — the escalation dial** (`27C` §1(2), the ternary admin surface). Gates whether the
/// admin consents to pointing (mechanically-available) escalation machinery at fallible oracle
/// code. Defaults to [`VouchedOnly`](EscalationDial::VouchedOnly) — shifts licensed ONLY for
/// functions carrying the `tolerates:` vouch (the double-ended ack: author's mark × admin's
/// default). Names STRAWMAN (human-uttered).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum EscalationDial {
    /// `--no-probe-escalation` — no oracle code EVER executes under a shifted context, in either
    /// lane (`27C` §1). Maximally defensive; wrapped modeled sites run (an out-of-context check is
    /// wrong-world and licenses nothing). The chosen-defensive reading also gates probe-time
    /// execution of lifted guard material whose recognized argv-shape sits under a wrapper.
    NoEscalation,
    /// `--probe-escalation` — THE DEFAULT: shifts licensed ONLY for `tolerates:`-vouched functions
    /// (`27C:vouch-tolerates`). Both-sides consent (author's mark × admin's default).
    #[default]
    VouchedOnly,
    /// `--escalate-any-probe` — shifts licensed for UNMARKED oracles too; the admin knowingly
    /// overrides absent author consent and owns the blast-radius alone (`27C:hole-unvouched-oracles`).
    AnyProbe,
}
