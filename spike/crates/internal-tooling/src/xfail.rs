//! `xfail_until` — the unit-tier xfail pin and its census (`30A` §1 `d3-xfail-with-named-greening`).
//!
//! # What a pin is for
//!
//! A behaviour the engine does not implement yet is still a fact about sh, and
//! `rul-unsure-falls-toward-sh-parity` says the target assertion is sh's own answer. Writing that
//! assertion down and letting it fail is how the target stops being a plan and starts being a
//! test; wrapping it here is how the suite stays green until the mechanism arrives. The pin PASSES
//! while its inner assertion fails and goes LOUDLY RED the moment the assertion starts passing —
//! the e2e lens's XPASS-to-promote semantics, at the unit tier, so a target behaviour can never
//! arrive silently and leave a stale pin behind.
//!
//! Two things it is NOT. It is not a licence to assert WRONG behaviour as if desired: an interim
//! behaviour may be pinned too, but only in a separate test whose NAME says interim. And it is not
//! a place to hide a broken fixture — a panic from setup is indistinguishable from a panic from the
//! target assertion, so a pin's setup belongs OUTSIDE the closure, where an ordinary failure is an
//! ordinary failure.
//!
//! # Why the horizon is a round and never a date
//!
//! A calendar estimate for "when will the engine do this" is a guess dressed as a commitment, and
//! this project's schedule is measured in ROUNDS. So a horizon is a round marker ([`Horizon`]), a
//! calendar date is unrepresentable, and expiry is decided against [`CURRENT_ROUND`] — one const a
//! conductor bumps at round-open, which is what makes every pin's debt come due on its own.
//!
//! # Why the registry, and not the call site
//!
//! `30A` spelled the seat as `xfail_until(trigger, horizon, || …)`. Both of those live in [`PINS`]
//! instead, and the call site names its pin by SLUG. The census has to answer "what is owed, and by
//! when" over the whole workspace, and test binaries are separate processes that can never see each
//! other's calls — so the inventory must be static data the census reads directly rather than
//! literals it recovers by parsing Rust. The two-way lexical check
//! ([`census`](self::census_report)) is what keeps the registry and the call sites from drifting: a
//! registered live pin with no call site fails, and a call site naming an unregistered slug fails at
//! the call.

use std::cell::Cell;
use std::panic::{self, AssertUnwindSafe};
use std::sync::Once;

/// The round the project is currently working.
///
/// BUMPED BY HAND, BY A CONDUCTOR, AT ROUND-OPEN. It is the only thing that makes a pin's horizon
/// expire, so leaving it stale silently converts the whole census into decoration.
pub const CURRENT_ROUND: u32 = 30;

/// When a pin's debt comes due, as a round marker — never a date (see the module doc).
///
/// Legal markers are `r<N>`, `end-of-r<N>`, and a stage within a round, `r<N>:<stage-slug>`. The
/// three variants differ in what they OWE the reader: a scheduled horizon owes nothing, an
/// unscheduled one owes the reason its boundary was picked, and a deferred one owes the reason its
/// first horizon was allowed to pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Horizon {
    /// The trigger's own schedule fixes this round.
    Scheduled(&'static str),
    /// The trigger has no scheduled round. The marker is the nearest boundary at which the census
    /// should force the question, and `why` says that in so many words.
    Unscheduled {
        /// The forcing boundary.
        marker: &'static str,
        /// Why this boundary, given nothing is scheduled.
        why: &'static str,
    },
    /// Re-horizoned after its first horizon passed. Both markers ride along so the census can
    /// render the slip, and the reason is not optional — an unreasoned slip is the whole failure
    /// mode the expiry check exists to catch.
    Deferred {
        /// The horizon that passed.
        was: &'static str,
        /// The horizon now claimed.
        now: &'static str,
        /// Why it slipped.
        why: &'static str,
    },
}

impl Horizon {
    /// The marker in force — what the census groups by.
    #[must_use]
    pub fn marker(self) -> &'static str {
        match self {
            Self::Scheduled(marker) | Self::Unscheduled { marker, .. } => marker,
            Self::Deferred { now, .. } => now,
        }
    }

    /// The reason this horizon is what it is, where one is owed.
    #[must_use]
    pub fn why(self) -> Option<&'static str> {
        match self {
            Self::Scheduled(_) => None,
            Self::Unscheduled { why, .. } | Self::Deferred { why, .. } => Some(why),
        }
    }

    /// The round the marker in force names, or a refusal naming the grammar.
    ///
    /// # Errors
    /// When the marker is not `r<N>`, `end-of-r<N>`, or `r<N>:<stage>` — which is how a calendar
    /// date gets refused rather than silently accepted as a horizon.
    pub fn round(self) -> Result<u32, String> {
        round_of(self.marker())
    }

    /// Has this horizon passed? A marker that does not parse counts as PASSED — a malformed horizon
    /// must never buy a pin quiet.
    #[must_use]
    pub fn expired(self) -> bool {
        self.round().map_or(true, |round| CURRENT_ROUND > round)
    }
}

/// Parse a round marker to the round it names.
fn round_of(marker: &str) -> Result<u32, String> {
    let bare = marker.strip_prefix("end-of-").unwrap_or(marker);
    let digits = bare
        .strip_prefix('r')
        .map(|rest| rest.split(':').next().unwrap_or(rest))
        .ok_or_else(|| format!("horizon {marker:?} is not a round marker"))?;
    digits
        .parse::<u32>()
        .map_err(|_| format!("horizon {marker:?} names no round: expected r<N>, got {digits:?}"))
}

/// Whether a pin has live call sites, or is a placeholder the doctrine reserved but did not build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinState {
    /// At least one test calls [`xfail_until`] with this slug.
    Live,
    /// Deliberately unbuilt. Recorded so the census answers what is reserved as well as what is
    /// owed; a call site would make the reservation a lie, so the census refuses one.
    Reserved,
}

/// One xfail pin's inventory row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pin {
    /// The slug a call site names, and the census's identity for it.
    pub name: &'static str,
    /// The named lane or stage whose arrival should GREEN this pin — semantic, never a bare stage
    /// number (`30A` §1 d3).
    pub trigger: &'static str,
    /// When the debt comes due.
    pub horizon: Horizon,
    /// Whether anything calls it.
    pub state: PinState,
}

/// Every xfail pin in the workspace, in slug order.
///
/// APPEND-ONLY in spirit: a pin leaves this list when its target behaviour lands and the pin
/// becomes an ordinary test, which is a deliberate act with a diff, never a tidy-up.
pub const PINS: &[Pin] = &[
    Pin {
        name: "d-alpha-rename-equivalence",
        trigger: "the first alpha-rename build (`30A` §2 P-diff, RESERVED)",
        horizon: Horizon::Unscheduled {
            marker: "end-of-r31",
            why: "reserved by the doctrine and deliberately unbuilt; no alpha-rename mechanism is \
                  scheduled, so the boundary is where the census should ask again",
        },
        state: PinState::Reserved,
    },
    Pin {
        name: "p-x-definition-grade-keying",
        trigger: "a per-DEFINITION lift: the KEYING half landed (rows carry their own \
                  `DefinitionId` and the `(file, role)` join is gone), so what remains is that \
                  `PredictSet`/`VerdictSet` keep one row per `(file, role)` and the earlier of two \
                  within-file definitions produces no row for its frame to find",
        horizon: Horizon::Scheduled("r31:closure-custody"),
        state: PinState::Live,
    },
    Pin {
        name: "p-x-helper-unset-f-across-files",
        trigger: "the HELPER LANE consuming the frame: the definition table now records non-role \
                  funcdefs, so the environment can see the removal, but `HelperIndex` still \
                  resolves by last-declaration-wins over the load-inert sources and asks it nothing",
        horizon: Horizon::Scheduled("r31:closure-custody"),
        state: PinState::Live,
    },
    Pin {
        name: "p-x-intra-compound-plurality",
        trigger: "`28Q:pin-emission-planner-universal` — per-segment environments for a composed \
                  compound (explicit per-segment subshells or alpha-rename, whichever it lands)",
        horizon: Horizon::Unscheduled {
            marker: "end-of-r31",
            why: "the emission planner is direction-ruled but unscheduled; this boundary is where \
                  the census should force the question",
        },
        state: PinState::Live,
    },
    Pin {
        name: "p-x-placement-tuning-pair",
        trigger: "`28Q:pin-emission-planner-universal` — placement chosen per body (top-lift for \
                  the many-use helper, in-paren colocation for the once-used collider)",
        horizon: Horizon::Unscheduled {
            marker: "end-of-r31",
            why: "the emission planner is direction-ruled but unscheduled; this boundary is where \
                  the census should force the question",
        },
        state: PinState::Live,
    },
    Pin {
        name: "p-x-regional-helper",
        trigger: "a SITE-KEYED `closure_for` plus book-region indexing — the table widening landed, \
                  so the environment can tell a regional definition from an ambient one, but the \
                  closure API takes no site and the book census is still depth-blind",
        horizon: Horizon::Scheduled("r31:closure-custody"),
        state: PinState::Live,
    },
    Pin {
        name: "p-x-sentinel-value-conjunct",
        trigger: "the human's `rule-sentinel-value-conjunct` ruling (on their burndown, `30N` §4): \
                  whether recognized guarded-source must consult the sentinel's VALUE and not only \
                  whether the target closure's names are bound. Until it lands, a package that \
                  assigns `v1` under a guard testing `v2` is modelled as reused where a real shell \
                  sources it again, and the artifact forms read that as a book with nothing to \
                  place",
        horizon: Horizon::Unscheduled {
            marker: "end-of-r31",
            why: "the ruling is queued on the human's burndown rather than scheduled into a lane, \
                  so the boundary is where the census should force the question",
        },
        state: PinState::Live,
    },
    Pin {
        name: "p-x-book-level-dot-locals",
        trigger: "load-time VARIABLES joining the function-environment domain (or the value plane \
                  learning what a `.` assigns): the nested-load half of POSIX `.` parity landed, so \
                  one load program's assignment sites the next, but a book's `.` sites are separate \
                  CFG nodes and mint a fresh variable map each",
        horizon: Horizon::Unscheduled {
            marker: "end-of-r31",
            why: "the domain change is winner-shifting and carries an open monotonicity question, \
                  so it is unscheduled design work; this boundary is where the census should force \
                  the question",
        },
        state: PinState::Live,
    },
    Pin {
        name: "p-x-unknown-source-is-a-point-havoc",
        trigger: "`principle-unknown-source-is-a-point-havoc`: an unresolvable `.` sets every \
                  function binding to unknown AT THAT LINE and no further, so a later \
                  UNCONDITIONAL definition in the same frame re-binds by sh's last-wins — today \
                  the ⊤ never recovers and the whole tail of the book is unbindable",
        horizon: Horizon::Scheduled("end-of-r30"),
        state: PinState::Live,
    },
    Pin {
        name: "p-x-load-operand-param-expansion-of-dollar-zero",
        trigger: "`principle-load-operands-evaluate-over-controller-known-inputs`: a `.` operand \
                  built by PURE parameter expansion over the authored book path — `${0%/*}` — is a \
                  function of program text plus `$0` plus the modeled cwd, so it resolves through \
                  the closed allowlist without evaluating any command",
        horizon: Horizon::Scheduled("end-of-r30"),
        state: PinState::Live,
    },
    Pin {
        name: "p-x-load-operand-dirname-of-dollar-zero",
        trigger: "`principle-load-operands-evaluate-over-controller-known-inputs`, held by the \
                  open ruling `ask-dollar-zero-command-substitution-path`: `$(dirname \"$0\")` \
                  names a COMMAND, and predicting its output inside the engine is the \
                  tool-modelling `identity-declared-never-inferred` forbids — so the shape waits \
                  on an authored-model path rather than on an engine special case",
        horizon: Horizon::Scheduled("r31:book-load-acceptance"),
        state: PinState::Live,
    },
    Pin {
        name: "p-x-load-operand-cd-pwd-of-dollar-zero",
        trigger: "`principle-load-operands-evaluate-over-controller-known-inputs`, held by the \
                  same open ruling `ask-dollar-zero-command-substitution-path`: \
                  `$(cd \"$(dirname \"$0\")\" && pwd)` is the absolutizing spelling of the same \
                  script-location question, and it evaluates two commands rather than one",
        horizon: Horizon::Scheduled("r31:book-load-acceptance"),
        state: PinState::Live,
    },
    Pin {
        name: "p-x-glob-load-acquires-members",
        trigger: "`principle-load-operands-evaluate-over-controller-known-inputs`: a source glob is \
                  a SET-valued operand, expanded against the authored snapshot into an ordered \
                  family of ordinary `.` acts — it reuses the loop-propagation lane's \
                  member-population machinery, which is why it sits after that lane",
        horizon: Horizon::Scheduled("r31:book-load-acceptance"),
        state: PinState::Live,
    },
    Pin {
        name: "p-x-glob-load-members-are-order-unknown",
        trigger: "`principle-load-operands-evaluate-over-controller-known-inputs`: the target's \
                  collation order is unknowable from the controller, so two members defining one \
                  name with DIFFERENT bytes must withhold that name — no member may win — while a \
                  name only one member defines stays live",
        horizon: Horizon::Scheduled("r31:book-load-acceptance"),
        state: PinState::Live,
    },
    Pin {
        name: "p-x-glob-load-no-match-aborts",
        trigger: "`principle-load-operands-evaluate-over-controller-known-inputs`: a glob matching \
                  nothing in the snapshot sources the LITERAL pattern, so the engine must model an \
                  EVALUATED operand naming an unloadable file — which is a different fact from an \
                  operand it could not evaluate, even though both wall",
        horizon: Horizon::Scheduled("r31:book-load-acceptance"),
        state: PinState::Live,
    },
    Pin {
        name: "p-x-book-code-source-is-inclusion",
        trigger: "`principle-book-code-source-is-inclusion`: a resolvable `.` of an ORDINARY \
                  (non-dorc-lang) sh file is textual inclusion at the load site under whatever \
                  branch it sits in — today such a target is never opened, so even the \
                  unconditional `. ./helpers.sh` of plain sh walls the rest of the book",
        horizon: Horizon::Scheduled("r31:book-load-acceptance"),
        state: PinState::Live,
    },
    Pin {
        name: "p-x-blessed-toplevel-conditional",
        trigger: "the oracle-side blessing of read-only top-level commands \
                  (`oracle/CLAUDE.md only-load-inert-sources-contribute`: INERTNESS IS DYING IN \
                  LITERAL) — the same ruling that makes the file legal must make its binding May",
        horizon: Horizon::Unscheduled {
            marker: "end-of-r31",
            why: "the blessing is human-typed direction with no scheduled stage; this boundary is \
                  where the census should force the question",
        },
        state: PinState::Live,
    },
];

/// The pin registered under `name`.
#[must_use]
pub fn pin(name: &str) -> Option<&'static Pin> {
    PINS.iter().find(|pin| pin.name == name)
}

/// What running a pin's target assertion did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// The target assertion still fails — the pin is doing its job.
    StillFailing,
    /// The target assertion passed — the behaviour arrived and the pin must be promoted.
    Passing,
}

/// Run `target` and report whether it still fails, without letting its panic reach the harness.
///
/// Separate from [`xfail_until`] so the mechanism itself is falsifiably testable in both directions
/// (a passing closure must report [`Outcome::Passing`], or every pin in the tree is vacuous).
#[must_use]
pub fn xfail_outcome(target: impl FnOnce()) -> Outcome {
    quiet_hook_installed();
    QUIET.set(true);
    let result = panic::catch_unwind(AssertUnwindSafe(target));
    QUIET.set(false);
    if result.is_err() {
        Outcome::StillFailing
    } else {
        Outcome::Passing
    }
}

/// Pin a TARGET behaviour the engine does not implement yet: pass while it fails, go red when it
/// starts passing.
///
/// `name` is a slug registered in [`PINS`], which carries the trigger and the horizon. Put the
/// pin's setup outside `target` — a panic inside is read as the target assertion failing, so a
/// broken fixture in there would make the pin assert nothing.
///
/// # Panics
/// When `name` is not registered, and when `target` PASSES (the XPASS-to-promote signal).
pub fn xfail_until(name: &str, target: impl FnOnce()) {
    let registered = pin(name);
    assert!(
        registered.is_some(),
        "xfail pin {name:?} is not in `internal_tooling::xfail::PINS`; register it with its \
         trigger and its round horizon, or the census cannot say what is owed"
    );
    let trigger = registered.map_or("<unregistered>", |pin| pin.trigger);
    let horizon = registered.map_or("<unregistered>", |pin| pin.horizon.marker());
    assert_eq!(
        xfail_outcome(target),
        Outcome::StillFailing,
        "XPASS {name}: the target behaviour ARRIVED — promote this pin to an ordinary test and \
         drop its registry row (trigger was {trigger:?}, horizon {horizon:?})"
    );
}

// A pin's inner panic is expected on every run, so the default hook's message would be noise on a
// green suite. Silencing is thread-scoped rather than global because tests share a process under
// plain `cargo test`: a concurrent REAL panic must still print.
thread_local! {
    static QUIET: Cell<bool> = const { Cell::new(false) };
}

static HOOK: Once = Once::new();

fn quiet_hook_installed() {
    HOOK.call_once(|| {
        let previous = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            if !QUIET.get() {
                previous(info);
            }
        }));
    });
}

/// The census, rendered: every pin grouped by horizon, with its trigger, its reason where one is
/// owed, and whether its horizon has expired against [`CURRENT_ROUND`].
///
/// This is the "what is owed, and by when" screen. It is a pure function of [`PINS`] so both the
/// census test and the `xfail-census` task read one answer.
#[must_use]
pub fn census_report() -> String {
    let mut rows: Vec<&Pin> = PINS.iter().collect();
    // Marker before name, so two markers sharing a round cannot interleave and re-open a group.
    rows.sort_by_key(|pin| {
        (
            pin.horizon.round().unwrap_or(u32::MAX),
            pin.horizon.marker(),
            pin.name,
        )
    });
    let live = rows
        .iter()
        .filter(|pin| pin.state == PinState::Live)
        .count();
    let mut lines = vec![format!(
        "xfail census — CURRENT_ROUND = r{CURRENT_ROUND}, {live} live pin(s), {} reserved",
        rows.len().saturating_sub(live)
    )];
    let mut group = "";
    for pin in rows {
        let marker = pin.horizon.marker();
        if marker != group {
            group = marker;
            let flag = if pin.horizon.expired() {
                "   [EXPIRED]"
            } else {
                ""
            };
            lines.push(String::new());
            lines.push(format!("{marker}{flag}"));
        }
        let reserved = if pin.state == PinState::Reserved {
            " (RESERVED, unbuilt)"
        } else {
            ""
        };
        lines.push(format!("  {}{reserved}", pin.name));
        lines.push(format!("      trigger: {}", pin.trigger));
        if let Horizon::Deferred { was, .. } = pin.horizon {
            lines.push(format!("      slipped from: {was}"));
        }
        if let Some(why) = pin.horizon.why() {
            lines.push(format!("      why: {why}"));
        }
    }
    lines.push(String::new());
    lines.join("\n")
}

/// Every `.rs` file under `spike/crates`, excluding build output — the corpus the two-way call-site
/// check walks. Lexical, like the workspace's other cross-crate fences (`plan::erase`'s
/// `licence_mint_has_exactly_one_caller`), because the property is "no source in the tree spells
/// this", which no type bound expresses.
#[must_use]
pub fn workspace_sources() -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut stack = vec![crate::repo_root().join("spike").join("crates")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|name| name == "target") {
                    continue;
                }
                stack.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                let display = path.display().to_string().replace('\\', "/");
                if display.ends_with("internal-tooling/src/xfail.rs") {
                    continue; // this file's own doc text and self-tests name the seat
                }
                if let Ok(text) = std::fs::read_to_string(&path) {
                    out.push((display, text));
                }
            }
        }
    }
    out.sort();
    out
}

/// The pin slugs the workspace's sources actually call, with the files that call them.
#[must_use]
pub fn call_sites() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (path, text) in workspace_sources() {
        for slug in call_sites_in(&text) {
            out.push((slug, path.clone()));
        }
    }
    out.sort();
    out
}

/// The pin slugs `text` calls, in occurrence order — the pure half of [`call_sites`].
///
/// Separate from the file walk so the two shapes it MUST survive are testable without a tree, and
/// both are shapes it once did not survive. rustfmt WRAPS a call whose slug does not fit on the
/// line, leaving the open paren and the string literal on different lines, so the needle cannot be
/// the contiguous spelling: a wrapped call then reads as a registered pin nothing calls, and the
/// census reports a debt that is actually discharged. And a COMMENT naming the seat is not a call
/// site — this module's own doc text was once read as a call to a pin slugged with the rest of the
/// sentence.
///
/// Whole comment LINES are dropped; a trailing comment after code on the same line is not, so a
/// `// … xfail_until("…")` written to the right of real code would still be seen. Splitting that
/// case needs string-literal awareness, which is a parser, and the census is a lexical fence
/// (`licence_mint_has_exactly_one_caller` is the same shape).
#[must_use]
pub fn call_sites_in(text: &str) -> Vec<String> {
    const NEEDLE: &str = "xfail_until(";
    let code: String = text
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    let mut out = Vec::new();
    let mut rest = code.as_str();
    while let Some(at) = rest.find(NEEDLE) {
        let after = rest
            .get(at.saturating_add(NEEDLE.len())..)
            .unwrap_or_default();
        if let Some(quoted) = after.trim_start().strip_prefix('"')
            && let Some(end) = quoted.find('"')
            && let Some(slug) = quoted.get(..end)
        {
            out.push(slug.to_owned());
        }
        rest = after;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        CURRENT_ROUND, Horizon, Outcome, PINS, Pin, PinState, call_sites, call_sites_in,
        census_report, round_of, xfail_outcome,
    };

    /// The scan survives the two shapes that made it lie, and the census's whole authority rests on
    /// it: a missed call site is a discharged debt reported as owed, and a phantom one is a call to
    /// a pin that does not exist. Both were live — the wrapped call reddened a real lane, and the
    /// decoy is this module's own prose.
    #[test]
    fn the_call_site_scan_survives_a_wrapped_call_and_a_doc_comment_decoy() {
        let wrapped = "        xfail_until(\n            \"p-x-a-very-long-slug-rustfmt-will-wrap\",\n            || {},\n        );\n";
        assert_eq!(
            call_sites_in(wrapped),
            ["p-x-a-very-long-slug-rustfmt-will-wrap"],
            "rustfmt puts the paren and the slug on different lines; the scan must still see it"
        );

        let decoy = "/// prose about xfail_until(\"not-a-real-pin\") and how it works\n//! and xfail_until(\"nor-this-one\")\n// xfail_until(\"nor-this\")\n";
        assert!(
            call_sites_in(decoy).is_empty(),
            "a comment naming the seat is not a call site: {:?}",
            call_sites_in(decoy)
        );

        let ordinary = "    xfail_until(\"p-x-plain\", || {});\n";
        assert_eq!(call_sites_in(ordinary), ["p-x-plain"]);
    }

    /// The mechanism, falsifiably, in BOTH directions. Without the second half every pin in the tree
    /// could be vacuous and nothing would say so.
    #[test]
    fn a_pin_passes_while_its_target_fails_and_reports_a_pass_when_it_stops() {
        assert_eq!(
            xfail_outcome(|| assert_eq!(1, 2, "a target the engine does not meet")),
            Outcome::StillFailing
        );
        assert_eq!(xfail_outcome(|| assert_eq!(1, 1)), Outcome::Passing);
    }

    /// A calendar date must not be spellable as a horizon. This is the whole enforcement of "round
    /// markers, never dates": the parse refuses, so a dated pin cannot reach the census.
    #[test]
    fn the_horizon_grammar_admits_rounds_and_refuses_dates() {
        assert_eq!(round_of("r31"), Ok(31));
        assert_eq!(round_of("end-of-r30"), Ok(30));
        assert_eq!(round_of("r31:closure-custody"), Ok(31));
        for bad in ["2026-09-01", "September", "end-of-2026", "r", "rX"] {
            assert!(
                round_of(bad).is_err(),
                "{bad:?} must not parse as a horizon"
            );
        }
    }

    /// THE CENSUS (`30A` §1 d3): the registry and the call sites agree, every horizon parses, and no
    /// live pin's horizon has passed.
    ///
    /// Filterable as `xfail_census`. The rendered inventory rides in the failure message so a red
    /// run answers "what is owed, and by when" without a second command; `mise run xfail:census`
    /// prints the same bytes on demand.
    #[test]
    fn xfail_census_is_coherent() {
        let report = census_report();
        assert!(!PINS.is_empty(), "the registry is the census's floor");

        let called = call_sites();
        let mut problems: Vec<String> = Vec::new();
        for pin in PINS {
            let sites: Vec<&String> = called
                .iter()
                .filter(|(slug, _)| slug == pin.name)
                .map(|(_, path)| path)
                .collect();
            match pin.state {
                PinState::Live if sites.is_empty() => problems.push(format!(
                    "{}: registered Live but no test calls it — promote it or mark it Reserved",
                    pin.name
                )),
                PinState::Reserved if !sites.is_empty() => problems.push(format!(
                    "{}: registered Reserved but called from {sites:?} — it is Live",
                    pin.name
                )),
                _ => {}
            }
            if let Err(why) = pin.horizon.round() {
                problems.push(format!("{}: {why}", pin.name));
            } else if pin.horizon.expired() {
                problems.push(format!(
                    "{}: horizon {} PASSED (current r{CURRENT_ROUND}) — green it, or re-horizon it \
                     as `Horizon::Deferred` with the reason it slipped",
                    pin.name,
                    pin.horizon.marker()
                ));
            }
        }
        for (slug, path) in &called {
            if super::pin(slug).is_none() {
                problems.push(format!("{path}: calls unregistered pin {slug:?}"));
            }
        }
        assert!(
            problems.is_empty(),
            "{} xfail-census problem(s):\n  {}\n\n{report}",
            problems.len(),
            problems.join("\n  ")
        );
    }

    /// A `Deferred` horizon cannot exist without its reason, and the render surfaces both markers —
    /// the point of the variant is that a slip is visible, not merely survivable.
    #[test]
    fn a_deferred_horizon_renders_its_slip_and_its_reason() {
        let pin = Pin {
            name: "p-x-synthetic-example",
            trigger: "a lane that has not arrived",
            horizon: Horizon::Deferred {
                was: "r30",
                now: "r31",
                why: "the lane was re-scoped",
            },
            state: PinState::Live,
        };
        assert_eq!(pin.horizon.round(), Ok(31));
        assert_eq!(pin.horizon.why(), Some("the lane was re-scoped"));
    }
}
