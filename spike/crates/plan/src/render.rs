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

use dorc_core::{EntityRef, Interner};
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
    use super::{EntityRef, Interner, LeafId, sem};

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
    /// cell's wrapper is named `<kind>_<selector>__check` ([`check_fn_name`](crate::check_fn_name)),
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

    /// A POSIX function definition wrapping the kind's `oracle_probe_*` body
    /// (`<fn_name>() <body>`), emitted once per `(kind, selector)` cell.
    ///
    /// GUARANTEE: dash-n-clean **iff `body` is a brace-group** (`{ …; }`) — the
    /// sanctioned `oracle_probe_*` shape (205 §1 / st-2, `20B` §3), so `name() { …; }`
    /// is a valid funcdef. The body ships verbatim (self-vouched: the kind's own
    /// declared probe), never the placeholder check argparse. `fn_name` is a
    /// [`check_fn_name`](crate::check_fn_name) — routed through the
    /// hyphen↔underscore funcname map, so a hyphenated kind yields a valid POSIX name.
    #[must_use]
    pub fn wrapper_def(fn_name: &str, body: &str) -> String {
        format!("{fn_name}() {body}\n")
    }

    /// The check invocation with the resolved entity F-QUOTE-bound as `$1` (or no
    /// operand for a [`EntityRef::Singleton`]).
    ///
    /// GUARANTEE (F-QUOTE, `notes/198`, `inv-kfail` both directions): the operand is
    /// rendered by [`sem::single_quote`] — the LONE quoting decision in this module —
    /// so it is exactly **one inert positional argument** in any sh. An un-quoted
    /// operand could word-split (⇒ probe the wrong entity, `kFAIL-perform`) or re-parse
    /// a metachar as a second command (`x; touch …` ⇒ `kFAIL-withhold` probe-mutation);
    /// the single-quote wrapping forecloses both. A Singleton emits the bare fn name (no
    /// operand exists). Pinned by `probe_render_quotes_operand_with_space_or_metachar`
    /// and the `probe-operand-quoting` e2e case ("IN sh, FROM sh").
    #[must_use]
    pub fn invocation(fn_name: &str, entity: EntityRef, interner: &Interner) -> String {
        match entity {
            EntityRef::Operand(tok) => {
                format!("{fn_name} {}", sem::single_quote(interner.resolve(tok.0)))
            }
            EntityRef::Singleton => fn_name.to_string(),
        }
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
    #[must_use]
    pub fn record_scaffold(invocation: &str, key: &str) -> String {
        format!(
            "{invocation}; _rc=$?; \
             if [ \"$_rc\" -eq 0 ]; then _e=holds; \
             elif [ \"$_rc\" -eq 1 ]; then _e=absent; \
             else _e=cant-tell; fi; \
             printf 'site {key} effect=%s rc=%s\\n' \"$_e\" \"$_rc\"\n"
        )
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
        format!("# site:{} skip-unresolvable\n", site.0)
    }
}

// ===========================================================================
// Apply-artifact emitters (`Plan::render_sh` flat + `Plan::render_apply` line)
// ===========================================================================

/// Apply-artifact emitters: the two apply-phase renders' bytes. `render_sh` is the
/// FLAT leaf-list (per-leaf provenance, throws away guards — not runnable); `render_apply`
/// is the LINE-granular book-faithful rewrite (keeps control flow, runnable). Assembly
/// only — the methods decide which leaf/line is run/replaced/omitted; these emit one
/// decided piece. The two renders' headers differ on purpose (they are different
/// artifacts), preserved as named variants below (see module-level note on
/// same-construct-different-assembly).
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

    /// LINE-granular **in-situ** substitution for a one-liner `case`-arm body
    /// (`pat) <stand-in> ;;   # …`): keep the `pat)` prefix + ` ;;` suffix, replace ONLY
    /// the command's byte-span with its value-preserving stand-in (the T14 fix, `20G`
    /// item-4 / `notes/199` cluster-C).
    ///
    /// GUARANTEE: dash-n-clean **iff** `prefix`/`suffix` are the verbatim line bytes
    /// bracketing the command span (the caller slices them from `line_start`), because
    /// then the arm keeps its `pat)`/`;;` scaffolding intact (`nginx) true ;;`) — the
    /// empty-clause `dash -n` error that whole-line commenting an arm body would cause is
    /// what this path exists to avoid. The trailing comment after `;;` is a valid arm
    /// end. Caller-precondition: the leaf was detected by `case_arm_oneliner_leaves`
    /// (AST-structural, DIRECT arm-body items only). Pinned by `render-case-arm-oneliner`
    /// (e2e, exec-gated) + `render_one_liner_case_arm_body_substitutes_in_situ_…`.
    #[must_use]
    pub fn inline_arm_subst(prefix: &str, stand_in: StandIn, suffix: &str) -> String {
        let mut out =
            String::with_capacity(prefix.len().saturating_add(suffix.len()).saturating_add(64));
        out.push_str(prefix);
        out.push_str(&standin_sh(stand_in));
        out.push_str(suffix);
        out.push_str("   # dorc: elided (already converged) — case-arm body substituted in situ\n");
        out
    }

    /// LINE-granular **in-situ** substitution for one or more `Replace`/`Omit` leaves
    /// that share their source line with a loop/`if`/`case` **scaffolding keyword**
    /// (`done`/`fi`/`esac`/`for`/`while`/`if`/`then`/`else`/`elif`/`do`) — the task-F2
    /// generalisation of [`inline_arm_subst`] (20O find-2). Each `(start, end, filler)`
    /// replaces that in-line byte span with `filler` (the leaf's value-preserving
    /// stand-in, or `:` for a dead `Omit`); every other byte — crucially the keyword — is
    /// kept verbatim.
    ///
    /// GUARANTEE: dash-n-clean **iff** the spans are the in-line byte ranges of the
    /// line's elidable leaves (the caller slices them via `line_start`) and the bytes
    /// OUTSIDE the spans are valid sh (here, the scaffolding keyword the book wrote). This
    /// is the whole point: whole-line commenting `done; install` yields
    /// `# done; install` ⇒ the `done` is gone ⇒ `for…do…` has no terminator ⇒ a
    /// `dash -n` "expecting done" error ⇒ the apply aborts MID-RUN on the host (violating
    /// fail-before-network). Splicing only the leaf span (`done; true`) keeps the keyword.
    /// Spans are applied **right-to-left** (highest `start` first) so an earlier span's
    /// offsets are unperturbed by a later splice. Caller-precondition: every span is on a
    /// single line (a multi-line leaf is refused upstream and run verbatim — the
    /// kFAIL-perform fallback). Pinned by `post-loop-shared-done-line` /
    /// `pre-loop-shared-for-line` / `fi-shared-line` (e2e, exec-gated) + the
    /// `render_*_shared_*` unit tests.
    #[must_use]
    pub fn inline_scaffold_subst(line: &str, subs: &[(usize, usize, String)]) -> String {
        let mut spliced = line.to_string();
        // Right-to-left: replacing a later span first leaves earlier byte offsets valid.
        let mut ordered: Vec<&(usize, usize, String)> = subs.iter().collect();
        ordered.sort_by_key(|s| core::cmp::Reverse(s.0));
        for (start, end, filler) in ordered {
            let lo = (*start).min(spliced.len());
            let hi = (*end).min(spliced.len()).max(lo);
            spliced.replace_range(lo..hi, filler);
        }
        let mut out = String::with_capacity(spliced.len().saturating_add(64));
        out.push_str(&spliced);
        out.push_str("   # dorc: elided (already converged / dead branch) — substituted in situ (shares line with scaffolding)\n");
        out
    }

    /// LINE-granular **whole-line** neutralisation: comment the original command, then
    /// emit its value-preserving stand-in (`filler`) at the original indent.
    ///
    /// GUARANTEE: dash-n-clean for an ordinary one-command line — the original becomes a
    /// `#`-comment and the `filler` (a [`standin_sh`] string, or `:` for a wholly-dead
    /// `Omit`-only line) is a valid command at the line's position. This is the
    /// `an-render-runnable` rule: a comment *alone* would delete the command and leave an
    /// empty clause if it were the lone body of a guard — the stand-in (valid in every
    /// context a command was) prevents that. It does NOT cover an `if`/`then`/`fi` guard
    /// line (the `StatusRenderFloor`, blocked upstream in `prove_replaceable`) nor a
    /// one-liner case arm (routed to [`inline_arm_subst`] instead). `indent` is the
    /// leading whitespace of the original line, preserved so the filler sits where the
    /// command did. Pinned by `render-multileaf-line-all-elide` + the `exec-*` cases.
    #[must_use]
    pub fn commented_line(line: &str, indent: &str, filler: &str) -> String {
        let mut out =
            String::with_capacity(line.len().saturating_add(indent.len()).saturating_add(64));
        out.push_str("# ");
        out.push_str(line.trim_start());
        out.push_str("   # dorc: elided (already converged / dead branch)\n");
        out.push_str(indent);
        out.push_str(filler);
        out.push('\n');
        out
    }
}
