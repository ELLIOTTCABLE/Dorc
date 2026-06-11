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
}
