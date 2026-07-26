//! `plan::render` — the **artifact assembler**: the single audited home of every
//! place the engine emits sh *text* (`cm-3`, note `20A` §2; `20D` §3/§5; task-R).
//!
//! Before this module the sh-text construction was scattered inline across
//! `Plan::render_sh`, `Plan::render_apply`, and `ProbePlan::render_sh` — each a
//! hand-rolled string-assembly site, and each (per `20A` §1 fam-B) an *independent
//! dash-divergence surface*. `20D` §3 named the render-assembler as "the largest
//! remaining fam-B surface" and "the next cm-3 candidate". This module collapses the
//! emission to `O(1)` audited sites so future render work — guard-capable
//! substitution, member-elision list rewriting (`209` brk-1(b)), check-body shipping
//! with `rule-anno-render` — extends *here*, never by pasting a new `format!` into a
//! render method.
//!
//! # The split this module draws (assembly vs orchestration)
//!
//! This module owns **byte-level sh construction** only: given an already-decided
//! input (a resolved entity, a chosen [`StandIn`](crate::StandIn), a pre-sliced
//! line), produce the exact sh bytes. It does **not** own the *decision* of which
//! site/line/disposition gets which treatment — that stays in the render methods,
//! entangled with the `Plan`/`ProbePlan`/`Cfg`/`Ast` walks it needs (the same
//! boundary `20D` §2 rs-* drew for `sem`: the dep-free kernel hosts the rule, the
//! caller that holds the control-flow state applies it). The methods *call* these
//! emitters; the emitters never reach back into a plan.
//!
//! # Why an emitter, not a string, per construct
//!
//! Each emitter's doc records WHAT GUARANTEE its output carries — under which
//! preconditions it is `dash -n`-clean, and which gate proves it — because the
//! catastrophic render bug is a syntactically-broken or
//! observable-changing artifact (`ap-2` / `an-render-runnable`: spike-1 shipped
//! `if true; then # …; fi`, a `dash -n` error, green only because the harness
//! string-diffed). The `e2e` harness `dash -n` + exec gates (`spike/CLAUDE.md`
//! "Build / test / run") are the live proof; a guarantee that names its gate is one a
//! reviewer can check.
//!
//! # Quoting routes through `sem::single_quote` ONLY
//!
//! The F-QUOTE operand binding ([`probe::invocation`]) is the lone quoting decision
//! in any emitter, and it delegates to [`dorc_syntax::sem::single_quote`] (the `cm-3`
//! word-quoting home, `20D` §6). Verified: no emitter hand-rolls a quote. A bypass
//! would be a finding (task-R).

use dorc_core::{Interner, Symbol};
use dorc_syntax::sem;

use crate::{LeafId, StandIn};

// ===========================================================================
// Stand-in rendering — the value-preserving substitution bytes (`19A §5`)
// ===========================================================================

/// The sh that reproduces a [`StandIn`]'s exact observed exit status — `true` (rc 0),
/// `false` (rc 1), `(exit n)` (any other rc).
///
/// GUARANTEE: dash-n-clean and observable-faithful in **every** command position a
/// real command could occupy (the body of an `&&`/`||`, a `case` arm, a sequence,
/// a `for`/`while` body). `(exit n)` runs in a **subshell** so a non-zero stand-in
/// sets `$?` *without aborting* the surrounding script (a bare `exit n` would
/// terminate it — the bug the subshell prevents). This is the substitution *itself*,
/// not filler: it preserves the status a downstream `||`/`$?`/errexit consumer reads
/// (`inv-probe-sourced-values`; the round-19 `useradd[rc9] || mkdir` under-execute is
/// what forbids a blanket `true`). Proven by the `exec-*` e2e cases' ordered run-set +
/// `observable_matrix` unit tests (a `false` stand-in for an absent guard, etc.).
#[must_use]
pub fn standin_sh(stand_in: StandIn) -> String {
    match stand_in {
        StandIn::True => "true".to_string(),
        StandIn::False => "false".to_string(),
        StandIn::Exit(n) => format!("(exit {n})"),
    }
}

// ===========================================================================
// Probe-artifact emitters (`ProbePlan::render_sh`; `inv-site-keyed-results`)
// ===========================================================================

/// Probe-artifact emitters: the read-only, self-reporting shell-script the engine
/// ships to gather convergence facts (DESIGN "probing phase"; `inv-site-keyed-results`,
/// 202 §3). Assembly only — `compile_probe`/`ProbePlan::render_sh` decide *which* sites
/// are resolvable and walk them; these functions emit the bytes for one decided piece.
pub mod probe {
    use super::{Interner, LeafId, Symbol, sem};
    use crate::records::{self, Nonce};

    /// `rul-scratch-root-never-read-from-host` · `rul-probe-writes-only-what-it-owns`
    /// (`spike/CLAUDE.md`) — the report-scratch root is a controller-supplied LITERAL. It is never
    /// read from the host environment: not `TMPDIR`, not `HOME`, not `XDG_*`. Making it
    /// host-configurable is forbidden, not unimplemented — read those invariants first. (An
    /// admin-supplied override is out of scope for now, and would arrive as a controller value.)
    const SCRATCH_ROOT: &str = "/tmp";

    /// Format a record's site key: `N` for an ordinary single-fact site, `N.M` for member
    /// `M` of an in-loop Members fact-family (task-L2 item-4). The `.M` sub-key is the one
    /// grammar extension the member-precision slice adds; it keys a record back to a
    /// specific member of a specific leaf (`site <leafid>.<member-idx>`). Centralised here
    /// so the emitted grammar and the cli's `parse_results` stay in lockstep.
    #[must_use]
    pub fn site_key(site: LeafId, member: Option<u32>) -> String {
        match member {
            Some(m) => format!("{}.{m}", site.0),
            None => site.0.to_string(),
        }
    }

    /// The probe artifact header — documents the results-record grammar (205 §2
    /// rule-probe-exec-gate consumers, and the human reading the artifact, depend on it).
    ///
    /// GUARANTEE: a valid `#!/bin/sh` prologue of pure comment lines — dash-n-clean
    /// standalone. The grammar it documents (`site <leafid> effect=<holds|absent|cant-tell>
    /// rc=<n>`) is the out-of-band return channel.
    ///
    /// `stdout=`/`stderr=` are RESERVED record keys (`19F` §3 one-Observable tuple): the cli
    /// parser accepts-and-stores them (`parse_results`), but PRODUCING them is FUTURE WORK —
    /// this probe emits only `effect=`/`rc=`, so the EMITTED header text stays unchanged (the
    /// reserved keys live in the cli parser's doc + the record type, not in the shipped
    /// artifact bytes — which keeps every golden byte-identical). A consumed `Stdout`/`Stderr`
    /// blocks elision unconditionally regardless (16F §3), so reserving the keys is a SHAPE
    /// completion, not a behavior change.
    ///
    /// Wrapper naming (task-P/find-1, kept OUT of the emitted bytes to honor
    /// zero-extra-golden-churn — same posture 20H took for the reserved keys): each probed
    /// cell's wrapper is named `<kind>_<selector>__predict` ([`predict_fn_name`](crate::predict_fn_name)),
    /// one definition per `(kind, selector)` ([`wrapper_def`]); the selector segment is what
    /// lets a multi-selector kind ship two distinct bodies without collision.
    #[must_use]
    pub const fn header() -> &'static str {
        "#!/bin/sh\n\
         # dorc probe (read-only): checks per-SITE convergence, mutates nothing.\n\
         # When run, emits one results-record per site on stdout (the return channel):\n\
         #   site <leafid> effect=<holds|absent|cant-tell> rc=<n>\n\
         # effect is derived from the probe command's rc (0=holds, 1=absent, else cant-tell);\n\
         # rc is the raw PROBE-command status (opaque to Dorc — the record is the out-of-band lane).\n\n"
    }

    /// A per-site provenance comment naming the cell the site (or member) checks (`# site
    /// <key>: label`, `<key>` being `N` or `N.M` — [`site_key`]).
    ///
    /// GUARANTEE: one `#`-prefixed comment line ⇒ dash-n-clean. `label` is a
    /// [`fact_label`](crate::fact_label) (display-only, `inv-referent-agnostic`); it
    /// rides in a comment, never re-parsed.
    #[must_use]
    pub fn site_comment(key: &str, label: &str) -> String {
        format!("# site {key}: {label}\n")
    }

    /// The oracle's stripped `<provider>__predict` funcdef, emitted verbatim (R3 / 23D §1 —
    /// the check IS the oracle, shipped strip-only). `funcdef` is the whole
    /// [`strip_predict`](dorc_oracle::predict::strip_predict) output (`name() { …; }`), already
    /// `dash -n`-clean and byte-stable; this only appends the trailing newline. The render
    /// re-emits it before an invocation whose provider's body differs (the multi-check
    /// provider — [`ProbePlan::render_sh`](crate::ProbePlan::render_sh)).
    #[must_use]
    pub fn wrapper_def(funcdef: &str) -> String {
        format!("{funcdef}\n")
    }

    /// The check invocation with the site's argv F-QUOTE-bound (`<fn_name> 'install' '-y'
    /// 'nginx'`) — R3: the check's own argparse resolves the entity from these positionals.
    ///
    /// GUARANTEE (F-QUOTE, `notes/198`, `inv-kfail` both directions): each argv word is
    /// rendered by [`sem::single_quote`] — the LONE quoting decision in this module — so it
    /// is exactly **one inert positional argument** in any sh. An un-quoted word could
    /// word-split (⇒ resolve the wrong entity, `kFAIL-perform`) or re-parse a metachar as a
    /// second command (`x; touch …` ⇒ `kFAIL-withhold` probe-mutation); the single-quote
    /// wrapping forecloses both. A verbless/nullary check emits the bare fn name (empty
    /// argv). Pinned by `probe_render_quotes_operand_with_space_or_metachar` and the
    /// `probe-operand-quoting` e2e case ("IN sh, FROM sh").
    #[must_use]
    pub fn invocation(fn_name: &str, argv: &[Symbol], interner: &Interner) -> String {
        let mut out = fn_name.to_owned();
        for word in argv {
            out.push(' ');
            out.push_str(&sem::single_quote(interner.resolve(*word)));
        }
        out
    }

    /// The self-report scaffold appended after an `invocation`: capture the check's rc,
    /// map it to the three-outcome word, and `printf` the site-keyed record.
    ///
    /// GUARANTEE: dash-n-clean — an `invocation; _rc=$?; if … fi; printf …` command
    /// sequence valid wherever a command-list is (here, at script top level). The rc is
    /// captured into `_rc` *immediately* (before any other command can clobber `$?`),
    /// mapped by the oracle's `an-probe-shape` convention (`0⇒holds`, `1⇒absent`, else
    /// `cant-tell`), and the record (`site <key> effect=%s rc=%s`) is the out-of-band
    /// lane (rc stays opaque to Dorc — a standing human ruling). `_e`/`_rc` are
    /// probe-local names chosen unlikely to clash with a check body. The `site <key>`
    /// keys the record back to the apply leaf — or to a member of it (`N.M`, [`site_key`],
    /// task-L2 item-4) — (`inv-site-keyed-results`). Pinned by the `printf 'site …
    /// effect=` assertions across the probe-render tests + the `exec-*` gate-1 parity.
    ///
    /// THE ≥2 SINK-LANDING SITE (`sigpipe-flap-class`, `279f` §5): the `else _e=cant-tell`
    /// arm is the flat ≥2 rc sink (`rul-rc-partition`) — every rc that is not 0/1 lands here,
    /// including **rc 141** (`128 + SIGPIPE`), which a `pipefail`-off pipeline whose early-exit
    /// consumer (`… | grep -q`) closed the pipe before an upstream stage finished writing can
    /// produce race-dependently. That landing is ALWAYS SAFE here (cant-tell ⇒ `Unknown` ⇒
    /// can't-elide ⇒ run), and it never flaps the VERDICT: whether the race fires or not, the
    /// site runs. The cli's results readout attaches a why-lane note on a 141 landing
    /// ("likely benign early-exit race; consider a full-read form"). CONTRACT (recorded here
    /// because no such surface exists yet): a `dorc plan --exit-code`-like surface must NEVER
    /// source from these raw sink-landings — it must compute from divergence-of-world facts
    /// (`276:rul-verdicts-never-stable` / plan-as-API), so a benign SIGPIPE race can never move
    /// a process exit code. Until that surface exists, this doc IS the contract.
    #[must_use]
    pub fn record_scaffold(invocation: &str, key: &str, nonce: &Nonce) -> String {
        // `262` §2 framing: the emitted record line is `{nonce} site {key} effect=… rc=…
        // {token}` — the bare-nonce prefix (drain-keying) + the terminal token (tear-detect),
        // both printf-format literals; the `%s` args are unchanged.
        let framed = records::frame(nonce, &format!("site {key} effect=%s rc=%s"));
        format!(
            "{invocation}; _rc=$?; \
             if [ \"$_rc\" -eq 0 ]; then _e=holds; \
             elif [ \"$_rc\" -eq 1 ]; then _e=absent; \
             else _e=cant-tell; fi; \
             printf '{framed}\\n' \"$_e\" \"$_rc\"\n"
        )
    }

    /// Open the per-attempt report-scratch DIRECTORY — emitted ONCE per artifact, and only when
    /// some check drains (`rul-probe-writes-only-what-it-owns`). `mkdir` IS the safety property:
    /// it creates exclusively and does not resolve a symlink at the final component, so anything
    /// pre-positioned makes it FAIL rather than clobber, and `-m 700` applies the mode at creation
    /// (umask unapplied — no group/other-readable window). Failure empties `$_dsc`, the
    /// degradation signal every later site reads: the lane falls to `/dev/null` and the plan
    /// proceeds. Never retry, never fall back to a second name, never remove what is there.
    #[must_use]
    pub fn report_scratch_prologue(nonce: &Nonce) -> String {
        format!(
            "_dsc=\"{SCRATCH_ROOT}/dorc-drep.{n}\"; mkdir -m 700 \"$_dsc\" 2>/dev/null || _dsc=\n",
            n = nonce.0,
        )
    }

    /// The TIER-3 report-DRAIN scaffold (`27W` §3 C4 · `decline-class-emission`): the same effect
    /// record as [`record_scaffold`], plus the check running with `DREP_V1` bound to a per-site
    /// file INSIDE the exclusively-created scratch directory, its emissions re-framed as `report
    /// site=<key> …` records. Emitted ONLY for an [`emits_report`](crate::ProbePredict::emits_report)
    /// check, so every other probe stays byte-identical (`empty-world-byte-identical`).
    ///
    /// Every pathname operation here is confined to a container Dorc owns: the `: >` truncate is
    /// safe because [`report_scratch_prologue`] created the parent at mode 700 this run, and
    /// `rm -f` unlinks only inside it. When the prologue degraded, `$_dsc` is empty and the sink is
    /// `/dev/null` — no create, no read-back, no unlink. Pre-creating keeps the drain simple (a
    /// body that emits nothing yields an empty file, not a missing one), and `DREP_V1` needs no
    /// export because the check runs in this same shell.
    ///
    /// GUARANTEE: dash-n-clean — assignments, `if`s, and a `while IFS= read` loop, all valid at
    /// script top level. Each drained line is ONE `printf` with the payload value-passed as `%s`,
    /// so a `%` or a space in an author's emission cannot corrupt the frame.
    #[must_use]
    pub fn record_scaffold_draining(invocation: &str, key: &str, nonce: &Nonce) -> String {
        let effect = records::frame(nonce, &format!("site {key} effect=%s rc=%s"));
        let report = records::frame(nonce, &format!("report site={key} %s"));
        format!(
            "if [ -n \"$_dsc\" ]; then DREP_V1=\"$_dsc/{key}\"; : >\"$DREP_V1\"; \
             else DREP_V1=/dev/null; fi\n\
             {invocation}; _rc=$?\n\
             if [ \"$_rc\" -eq 0 ]; then _e=holds; \
             elif [ \"$_rc\" -eq 1 ]; then _e=absent; \
             else _e=cant-tell; fi\n\
             printf '{effect}\\n' \"$_e\" \"$_rc\"\n\
             if [ -n \"$_dsc\" ]; then \
             while IFS= read -r _dl; do printf '{report}\\n' \"$_dl\"; done <\"$DREP_V1\"; \
             rm -f \"$DREP_V1\"; fi\n"
        )
    }

    /// Close the report scratch — emitted once, under the same condition as
    /// [`report_scratch_prologue`]. `rmdir` removes only an EMPTY directory, so cleanup cannot
    /// cascade beyond what the per-site `rm -f`s already unlinked; `|| :` keeps a failed cleanup
    /// off the artifact's exit status. Residue on a hostile or unusual host is acceptable and
    /// disclosed — a failed cleanup is never an error and is never retried by name. `rm -rf` is
    /// permanently forbidden in this lane (an empty `$_dsc` would make it catastrophic).
    #[must_use]
    pub const fn report_scratch_epilogue() -> &'static str {
        "if [ -n \"$_dsc\" ]; then rmdir \"$_dsc\" 2>/dev/null || :; fi\n"
    }

    /// The comment recording an **un-resolvable** site (never invoked): a kill, opaque,
    /// written establish, `MustRun`, or a resolvable class whose kind has no declared
    /// probe (`can't-probe ⇒ can't-elide`, `kFAIL-perform`).
    ///
    /// GUARANTEE: one `#`-comment line ⇒ dash-n-clean. Transparency for the human
    /// reading the artifact and the D3 argv-echo differential; it emits no invocation,
    /// so the apply runs the site for real.
    #[must_use]
    pub fn unresolvable_comment(site: LeafId) -> String {
        format!("# site:{} unresolvable-no-probe\n", site.0)
    }
}

// ===========================================================================
// Derivation-probe emitters (`DerivationPlan::render_sh`; 24E §2/§5 — payload-bound footprints)
// ===========================================================================

/// Derivation-probe emitters: the read-only, self-reporting sh that DERIVES a payload-bound
/// footprint the static `evaluate_touches` tracer could not resolve (24E §2 — the SECOND
/// probe-shipping path). It rides in the SAME phase-1 artifact as the convergence probe (no
/// second `#!/bin/sh` — the e2e shebang-split keeps it in phase-1): each escalated wall-candidate
/// ships its stripped `<provider>__disturbs` body, and when run pipes its stdout coord-lines into
/// per-site `deriv <leafid> coord=…` records (`inv-site-keyed-results`). Assembly only —
/// [`DerivationPlan::render_sh`](crate::DerivationPlan::render_sh) decides which sites escalated
/// and walks them; these emit the bytes for one decided piece.
pub mod deriv {
    use super::{Interner, LeafId, Symbol, sem};
    use crate::records::{self, Nonce};

    /// The derivation-probe banner — comment-only (no shebang), documents the `deriv` record
    /// grammar. GUARANTEE: pure `#`-comment lines ⇒ dash-n-clean; appended to the convergence
    /// probe, so it never opens a second phase.
    #[must_use]
    pub const fn header() -> &'static str {
        "# dorc derivation-probe (read-only, 24E §2): payload-bound footprints the static\n\
         # tracer could not resolve. Each escalated wall-candidate ships its touches() body\n\
         # strip-only; when run it prints its footprint coordinates, re-keyed per site:\n\
         #   deriv <leafid> coord=<kind:entity>\n\
         # (the SAME leaf-id the apply plan assigns — inv-site-keyed-results).\n\n"
    }

    /// A per-site derivation provenance comment (`# deriv <leafid>: <provider> (host-derivation
    /// via <call>)`). GUARANTEE: one `#`-comment line ⇒ dash-n-clean. `provider`/`call` are
    /// display-only (`inv-referent-agnostic`), riding in a comment, never re-parsed.
    #[must_use]
    pub fn deriv_comment(site: LeafId, provider: &str, call: &str) -> String {
        format!(
            "# deriv {}: {provider} (host-derivation via {call})\n",
            site.0
        )
    }

    /// The derivation invocation with the site's argv F-QUOTE-bound (`<fn_name> 'install'
    /// 'nginx'`) — the SAME F-QUOTE guarantee as [`super::probe::invocation`] (`inv-kfail` both
    /// directions: each word is exactly one inert positional via [`sem::single_quote`], so an
    /// operand cannot word-split or re-parse a metachar into a second command).
    #[must_use]
    pub fn invocation(fn_name: &str, argv: &[Symbol], interner: &Interner) -> String {
        let mut out = fn_name.to_owned();
        for word in argv {
            out.push(' ');
            out.push_str(&sem::single_quote(interner.resolve(*word)));
        }
        out
    }

    /// The self-report scaffold: pipe the touches invocation's stdout (the coordinate lines the
    /// body printed) through a per-line `printf`, re-keying each as a `deriv <leafid> coord=<line>`
    /// record. `_c` is a probe-local name (unlikely to clash with a touches body).
    ///
    /// GUARANTEE: dash-n-clean — a pipeline into a `{ … }` group, valid at script top level.
    /// A body that prints NOTHING ⇒ no coord records ⇒ an empty derived footprint ⇒ the site
    /// walls (silence = wall, 24E §4). An un-shimmed derivation command under `PATH=mocks-only`
    /// (the fork-4A layer-3 mocks net) prints nothing ⇒ empty ⇒ wall — the safe direction
    /// (`kFAIL-perform`), never a wrong-elision.
    ///
    /// `262` §2 / `26A` stop-1 — THE at-most family close: deriv is a variable-count family
    /// with no inherent completion marker, so a mid-family cut is otherwise undetectable and
    /// would SHRINK the at-most footprint (⇒ MORE survivals — the under-execution direction).
    /// Each family closes with `deriv-end {site} n=<K>` where K is the count emitted; the
    /// consumer refuses a family whose received count ≠ K (or that has no end-record) ⇒
    /// wall-total. The count `_n` is scoped INSIDE the pipe's `{ … }` subshell — a bare
    /// `while` on the RHS of a pipe runs in a subshell whose variable increments never reach
    /// the parent (POSIX), so the end-record is emitted from within the SAME group.
    #[must_use]
    pub fn record_scaffold(invocation: &str, site: LeafId, nonce: &Nonce) -> String {
        let coord = records::frame(nonce, &format!("deriv {} coord=%s", site.0));
        let end = records::frame(nonce, &format!("deriv-end {} n=%s", site.0));
        format!(
            "{invocation} | {{ _n=0; while IFS= read -r _c; do printf '{coord}\\n' \"$_c\"; \
             _n=$((_n+1)); done; printf '{end}\\n' \"$_n\"; }}\n"
        )
    }
}

// ===========================================================================
// Resolver-probe emitters (24F §3 — the identity CANONICALIZATION lane)
// ===========================================================================

/// Resolver-probe emitters: the read-only, self-reporting sh that CANONICALIZES a coordinate via its
/// kind's `<kind>.resolve()` (24F §3 — the resid-aliasing closure). Rides the SAME phase-1 artifact
/// as the convergence + derivation probes (no shebang): per resolver-bearing coordinate it invokes
/// the stripped `<kind>__resolve` body with the ENTITY and, when run, prints the canonical form
/// re-keyed by the coordinate (`resolv <kind:entity> canon=…`), or a `dangling` marker when the
/// resolver fails on an enumerable kind (§4). Assembly only — the cli decides which coordinates need
/// resolution and walks them; these emit the bytes for one decided piece.
pub mod resolv {
    use super::sem;
    use crate::records::{self, Nonce};

    /// The resolver-probe banner — comment-only (no shebang), documents the `resolv` record grammar.
    /// GUARANTEE: pure `#`-comment lines ⇒ dash-n-clean; appended to the earlier probes, so it never
    /// opens a second phase.
    #[must_use]
    pub const fn header() -> &'static str {
        "# dorc resolver-probe (read-only, 24F §3): owner-declared identity canonicalization. Each\n\
         # resolver-bearing coordinate runs its kind's <kind>.resolve() with the entity; when run it\n\
         # prints the canonical form (or a dangling marker), re-keyed by the coordinate:\n\
         #   resolv <kind:entity> canon=<canonical>   |   resolv <kind:entity> dangling\n\
         # The engine canonicalizes both footprint and backing coords through this before disjoint.\n\n"
    }

    /// A per-coordinate provenance comment (`# resolv <kind:entity> via <kind>.resolve()`).
    /// GUARANTEE: one `#`-comment line ⇒ dash-n-clean. `coord`/`kind` are display-only
    /// (`inv-referent-agnostic`), riding in a comment, never re-parsed.
    #[must_use]
    pub fn resolv_comment(coord: &str, kind: &str) -> String {
        format!("# resolv {coord} via {kind}.resolve()\n")
    }

    /// The stripped `<kind>__resolve` funcdef, emitted verbatim (strip-only; re-emitted per kind on a
    /// body change, sh last-writer-wins). GUARANTEE: `funcdef` is `dash -n`-clean + byte-stable; this
    /// only appends a trailing newline.
    #[must_use]
    pub fn kind_def(funcdef: &str) -> String {
        format!("{funcdef}\n")
    }

    /// The self-report scaffold: invoke `<kind_fn> '<entity>'` (the entity F-QUOTE-bound — the SAME
    /// guarantee as [`super::probe::invocation`]: exactly one inert positional, no word-split/re-parse),
    /// capture its rc + stdout, and print the `resolv <coord> canon=…` record — or `resolv <coord>
    /// dangling` when the resolver rc is non-zero or its stdout empty (24F §4 — the enumerable kind's
    /// natural failure IS the dangling detection).
    ///
    /// GUARANTEE: dash-n-clean — a `$(...)`-capture + `if` + `printf` command sequence valid at script
    /// top level. `_c`/`_rr` are resolver-local names. An UN-SHIMMED resolver under `PATH=mocks-only`
    /// (the mocks net) 127s ⇒ empty stdout ⇒ `dangling` ⇒ the coord degrades to may-alias (§3a) — the
    /// safe direction (`kFAIL-perform`: fail toward run), never a wrong canonical.
    #[must_use]
    pub fn record_scaffold(kind_fn: &str, entity: &str, coord: &str, nonce: &Nonce) -> String {
        let q = sem::single_quote(entity);
        // `262` §2 framing: nonce prefix + terminal token on both arms; `canon=` is the
        // free-content field (last-to-token) so a canonical form with spaces survives.
        let canon = records::frame(nonce, &format!("resolv {coord} canon=%s"));
        let dangling = records::frame(nonce, &format!("resolv {coord} dangling"));
        format!(
            "_c=$({kind_fn} {q} 2>/dev/null); _rr=$?; \
             if [ \"$_rr\" -eq 0 ] && [ -n \"$_c\" ]; then printf '{canon}\\n' \"$_c\"; \
             else printf '{dangling}\\n'; fi\n"
        )
    }
}

// ===========================================================================
// Reach-probe emitters (24G §4 — the reaches() EXPANSION lane, DYNAMIC arms)
// ===========================================================================

/// Reach-probe emitters: the read-only, self-reporting sh that runs a DYNAMIC `reaches()` arm for a
/// footprint coordinate (24G §4 — the cross-author footprint-EXPANSION mechanism). Rides the SAME
/// phase-1 artifact as the convergence + derivation + resolver probes (no shebang): per
/// (reach-bearing coordinate, dynamic arm) it invokes the arm's per-arm wrapper with the ENTITY and,
/// when run, prints each stdout line (a RAW ENTITY in the arm's annotated kind — typed emission)
/// re-keyed by the coordinate AND the arm index (`reach <coord> arm=<index> entity=<line>`), so the
/// controller joins arm→kind STATICALLY (the vocabulary fence — a host never mints a kind). Assembly
/// only — the cli decides which coords/arms escalate and walks them; these emit one decided piece.
pub mod reach {
    use super::sem;
    use crate::records::{self, Nonce};

    /// The reach-probe banner — comment-only (no shebang), documents the `reach` record grammar.
    /// GUARANTEE: pure `#`-comment lines ⇒ dash-n-clean; appended to the earlier probes, never a
    /// second phase.
    #[must_use]
    pub const fn header() -> &'static str {
        "# dorc reach-probe (read-only, 24G §4): owner-declared reaches() expansion. Each dynamic\n\
         # reaches() arm of a reach-bearing footprint coord runs with the ENTITY; when run it prints\n\
         # the RAW ENTITIES it drags (its stdout lines), re-keyed by the coord AND the arm index:\n\
         #   reach <kind:entity> arm=<n> entity=<reached>\n\
         # The controller joins arm->kind statically (the kind is fixed at lift — never host-minted).\n\n"
    }

    /// A per-(coord, arm) provenance comment (`# reach <kind:entity> via <kind>.reaches() arm N`).
    /// GUARANTEE: one `#`-comment line ⇒ dash-n-clean. `coord`/`kind` are display-only
    /// (`inv-referent-agnostic`), riding in a comment, never re-parsed.
    #[must_use]
    pub fn reach_comment(coord: &str, kind: &str, arm_index: usize) -> String {
        format!("# reach {coord} via {kind}.reaches() arm {arm_index}\n")
    }

    /// The per-arm wrapper funcdef, emitted verbatim
    /// (`<kind>__disturbance_reaches_only_<n>() { <arm bytes> ; }` —
    /// the arm command's byte-exact span-slice, mark-free by construction; re-emitted per arm-fn on a
    /// body change, sh last-writer-wins). GUARANTEE: `funcdef` is `dash -n`-clean (author sh wrapped
    /// in a function body) + byte-stable; this only appends a trailing newline.
    #[must_use]
    pub fn arm_def(funcdef: &str) -> String {
        format!("{funcdef}\n")
    }

    /// The self-report scaffold: invoke `<arm_fn> '<entity>'` (the entity F-QUOTE-bound — the SAME
    /// guarantee as [`super::probe::invocation`]: exactly one inert positional, no word-split/re-parse)
    /// and pipe its stdout lines through a per-line `printf`, re-keying each as a `reach <coord>
    /// arm=<index> entity=<line>` record. `_re` is a reach-local name (unlikely to clash).
    ///
    /// GUARANTEE: dash-n-clean — a pipeline into a `while read` loop, valid at script top level. An
    /// arm that prints NOTHING ⇒ no records ⇒ no expansion for this coord (the footprint stays narrow
    /// — the safe direction: a narrower footprint elides MORE, but that is the un-expanded floor, not
    /// a wrong-elision beyond it; the wall still walls on its own coords). An UN-SHIMMED reach command
    /// under `PATH=mocks-only` (the fork-4A layer-3 mocks net) 127s ⇒ prints nothing ⇒ no expansion —
    /// never a wrong-reach (`kFAIL-perform`: an omitted reach only fails to WIDEN, the honest floor).
    #[must_use]
    pub fn record_scaffold(
        arm_fn: &str,
        entity: &str,
        coord: &str,
        arm_index: usize,
        nonce: &Nonce,
    ) -> String {
        let q = sem::single_quote(entity);
        // `262` §2 framing: nonce prefix + terminal token; `entity=` is the free-content field
        // (last-to-token) so a reached entity with spaces survives (the old single-token
        // truncation is fixed — `279f` rider generalization).
        let rec = records::frame(nonce, &format!("reach {coord} arm={arm_index} entity=%s"));
        format!("{arm_fn} {q} | while IFS= read -r _re; do printf '{rec}\\n' \"$_re\"; done\n")
    }
}

// ===========================================================================
// Apply-artifact emitters (`Plan::render_sh` flat + `Plan::render_apply` line)
// ===========================================================================

/// Apply-artifact emitters: the two apply-phase renders' bytes. `render_sh` is the FLAT
/// leaf-list (per-leaf provenance, throws away guards — not runnable); `render_apply` is the
/// LEAF-EXACT (span-based) book-faithful rewrite (arch-1, note 214: keeps control flow,
/// runnable, substitutes each elided leaf's exact byte-span in-situ). Assembly only — the
/// methods decide which leaf is run/replaced/omitted and compute the span edits; these emit
/// one decided piece. The two headers differ on purpose (different artifacts), preserved as
/// named variants below. The span render's byte-splicing lives in the method (`render_apply`)
/// because it needs the source bytes + edit set; this module owns the lone provenance comment
/// ([`provenance_comment`]) the span edit appends.
pub mod apply {
    use super::{StandIn, standin_sh};

    /// Header for the FLAT plan render ([`Plan::render_sh`](crate::Plan::render_sh)).
    ///
    /// GUARANTEE: a `#!/bin/sh` + comment prologue, dash-n-clean. NB the flat render is
    /// a per-leaf *disposition listing*, NOT a runnable rewrite (it drops enclosing
    /// guards — a known first-cut limitation); the shebang is provenance-shape, the
    /// `render_apply` artifact is the runnable one.
    #[must_use]
    pub const fn plan_header() -> &'static str {
        "#!/bin/sh\n# dorc plan (apply phase). Replaced leaves are already converged.\n\n"
    }

    /// Header for the LINE-granular book-faithful render
    /// ([`Plan::render_apply`](crate::Plan::render_apply)) — the CLI's final artifact.
    ///
    /// GUARANTEE: a `#!/bin/sh` + comment prologue, dash-n-clean. Distinct text from
    /// [`plan_header`] by design — this artifact IS runnable (it preserves the book's
    /// control flow), so its banner names the value-preserving-stand-in contract the
    /// body upholds. Kept as a separate emitter so the two banners never silently
    /// converge (a golden-churn tripwire).
    #[must_use]
    pub const fn apply_header() -> &'static str {
        "#!/bin/sh\n# dorc apply: the book, with already-converged/dead lines elided (value-preserving stand-in).\n\n"
    }

    /// The banner comment preceding the GUARD PREAMBLE defs (24D §2 / rul-ternary-verdict) —
    /// emitted ONCE by [`Plan::render_apply`](crate::Plan::render_apply) when ≥1 site guards,
    /// above the verdict-function defs the guarded lines invoke. Documents the strip-only sourcing
    /// (the two never-clauses); an artifact comment (rec-1: comments are part of the byte floor).
    ///
    /// GUARANTEE: pure `#`-comment lines, dash-n-clean; empty-preamble ⇒ this is never emitted, so
    /// a guard-free book stays byte-identical to HEAD.
    #[must_use]
    pub const fn guard_preamble_banner() -> &'static str {
        "# dorc guard preamble: the vouching oracle's own verdict body, shipped strip-only\n\
         # (dialect annotations removed, nothing else changed -- rul-ternary-verdict: the\n\
         # authored bytes verbatim, no engine-synthesized sh).\n"
    }

    /// The FLAT-render provenance block for a `Replace`d leaf (`# replace[id]: <sh>
    /// (→ <stand-in>)` + a why-line naming the fact).
    ///
    /// GUARANTEE: comment-only (`#`-prefixed both lines) ⇒ dash-n-clean. The flat render
    /// never emits the stand-in as live code (it is a listing, not a rewrite), so the
    /// stand-in text appears only inside the comment. `→`/`↳` are display glyphs.
    #[must_use]
    pub fn flat_replace_block(leaf: u32, sh: &str, stand_in: StandIn, fact_label: &str) -> String {
        format!(
            "# replace[{leaf}]: {sh}  (\u{2192} {})\n#   \u{21b3} {fact_label} already holds (probe: converged \u{b7} must \u{b7} ambient)\n",
            standin_sh(stand_in),
        )
    }

    /// The FLAT-render provenance block for an `Omit`ted (fold-dead) leaf.
    ///
    /// GUARANTEE: comment-only ⇒ dash-n-clean. A dead leaf has no status to reproduce,
    /// so (unlike a replace) there is no stand-in — just the provenance.
    #[must_use]
    pub fn flat_omit_block(leaf: u32, sh: &str) -> String {
        format!(
            "# omit[{leaf}]: {sh}\n#   \u{21b3} dead branch: a guard's known status proves it never runs\n",
        )
    }

    /// The trailing provenance comment for a rendered line that carries ≥1 leaf-exact span
    /// edit (arch-1, note 214 — the ONE provenance emitter the span render appends). It
    /// discloses each replaced command's ORIGINAL text (the whole-line-comment form the
    /// span render retired carried the original; the new form must not lose that — `20V` §4
    /// d-3), so the human still sees what was elided.
    ///
    /// GUARANTEE: dash-n-clean ONLY when appended at a comment-safe line end — a `#` begins
    /// a comment to end-of-line, valid after any complete command. The CALLER is responsible
    /// for the safety precondition (`20V` §4 d-3 SAFETY RULE): it must NOT append this on a
    /// line whose post-edit content involves a heredoc operator, a backslash-continuation,
    /// or any shape where a trailing `#` is not a comment boundary — there it DROPS the
    /// comment (artifact correctness over provenance prose; the OOB verdict lane still
    /// carries the disclosure). `originals` is the list of replaced commands' source text in
    /// left-to-right line order. Returns the bare ` # dorc: …` suffix (no newline) to splice
    /// onto the already-built edited line.
    #[must_use]
    pub fn provenance_comment(originals: &[String]) -> String {
        // The disclosure: each original command's text, `;`-joined inside `[…]` (a single
        // line, so multi-command lines read as one bracketed list). An empty `originals`
        // (an Omit-only line whose dead command we substituted with `:`) still discloses the
        // dead command, so this is never called with an empty slice in practice; guard it
        // anyway (a bare marker, no brackets).
        if originals.is_empty() {
            return "   # dorc: elided (already converged / dead branch)".to_string();
        }
        // A `#` comment runs to the next NEWLINE, so a multi-line original's embedded `\n`
        // would split the comment — the second line becoming a stray (possibly
        // unterminated-quote) command. Flatten interior newlines to a single space so the
        // disclosure stays ONE comment line and dash-n-clean (the comment is provenance
        // prose; collapsing its whitespace loses nothing load-bearing). The CALLER's
        // comment-safety check (`comment_safe`) guards the rendered-line shape; this guards
        // the injected original.
        let flat: Vec<String> = originals
            .iter()
            .map(|o| o.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        format!(
            "   # dorc: elided [{}] (already converged / dead branch)",
            flat.join("; "),
        )
    }

    /// The NO-BRACKET elided-provenance for the commented-original render form (the human's
    /// round-24 lean): a top-level standalone converged/dead line renders as `# <original bytes>`,
    /// so the disclosure needs no `[…]` echo of the original — it is already verbatim on the line.
    /// Returns the bare `   # dorc: elided (…)` suffix (no newline) spliced onto the `# <original>`.
    ///
    /// GUARANTEE: dash-n-clean — the whole rendered line is `# <original>   # dorc: elided (…)`, one
    /// comment running to end-of-line, so a `#`/quote/metachar in the original is inert (a comment
    /// is never tokenised). The caller ([`Plan::collect_edits`]) restricts this to SINGLE-LINE
    /// top-level Simple leaves, so the comment never strands a multi-line tail as live code.
    #[must_use]
    pub const fn commented_original_provenance() -> &'static str {
        "   # dorc: elided (already converged / dead branch)"
    }
}
