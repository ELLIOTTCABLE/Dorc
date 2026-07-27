//! `strip` — the whole-file off-ramp cleaner (`dorc strip`; `strip-is-pure-erasure`,
//! `271:rul-dorc-prefix-head-synthesis`, `274` §13). `dorc-strip` is the focused, standalone
//! POSIX-ifier the root `DESIGN.md` names: it removes any/all non-POSIX Dorc annotations, leaving
//! runnable stock sh.
//!
//! PARSER-BACKED erasure, never text substitution over the body (human ruling 2026-07-17: "the
//! strip *cannot* be gsub-tier simplicity"). It lifts every role funcdef (the same six lifts the
//! marker gate walks), then deletes the dialect constructs by their PARSED spans:
//!
//! * a bind `name : kind = value` → `name=value` (the author's verbatim name + value bytes);
//! * a trailing verdict/observe mark (`:`/`:!`/`:?` coord) → gone, the command bytes kept;
//! * a BARE-mark statement (a `:` no-op whose only job is to host a mark — `state_stored_only_in`'s
//!   `undivided-by-transit-across` line) → deleted WHOLE, never left as a `:` null command. A stripped-in trailing
//!   `:` would clobber the body's tool-rc to 0, and in guard position that is an always-skip guard
//!   (the disaster shape, `23H` §9.4). So the author's last SUBSTANTIVE command stays the body's
//!   last exit-status-affecting statement.
//! * the line-1 `dorc-sh` shebang RUNNER → `sh` (a narrow, parse-informed line-1 rewrite, so the
//!   off-ramp artifact is fully dorc-free and runs under stock sh — `274` §13).
//! * the `# dorc-lang/v0.2` dialect marker line → deleted whole (human ruling 2026-07-17,
//!   `27D:disposition-strip-keeps-marker` RULED STRIP-IT): a stripped artifact is no longer dialect
//!   text and must not claim to be. This also makes the strip converge on a marker-FREE artifact, so
//!   a second pass early-returns (idempotence via the marker gate, not via a dialect-free re-walk).
//!
//! NO in-body name rewriting (a role funcname is already valid sh — it stays byte-for-byte); a
//! `dorc-sh`-typed row-three line is untouched (it is a runtime object, not an annotation).
//! Marker-GATED (strip-if-marked; `24P` §9 decision-dorc-sh-semantics): an unmarked file — plain
//! sh, or an already-stripped artifact — passes through byte-identical, so the strip is idempotent.
//!
//! SPIKE SCOPE (`churn-avoidance-disclosure`): the `dorc:sh` full-analysis-license prefix (`274`)
//! is a BOOK-level admin construct; the corpus has zero marked books and this pass walks only role
//! funcdef bodies, so a top-level `dorc:` prefix is out of reach here. The command-position case
//! (a `dorc:`-prefixed word inside a lifted body) IS erased below, honoring the contract where the
//! construct can actually land in a marked oracle.

use dorc_aid::Carrier;
use dorc_core::{Interner, Span};

use crate::marker::{MARKER, MARKER_WINDOW, has_marker};
use crate::predict::{
    Command, Predict, Stmt, Word, lift_predicts, lift_reaches, lift_resolvers,
    lift_state_stored_only_in, lift_touches, lift_verdicts_converged,
};

/// Erase every dorc-lang dialect construct from a whole marked file, yielding runnable POSIX sh
/// (`dash -n`-clean). Pure and total (`inv-no-throw`): a non-char-boundary span is skipped rather
/// than panicking. Deterministic (`inv-determinism`): role funcdefs are collected in source order
/// and edits applied back-to-front, so the output is a byte-stable function of the input.
pub fn strip_file(interner: &mut Interner, src: &str) -> Carrier<String> {
    // Marker-gated: an unmarked file (plain sh, or an already-stripped off-ramp artifact) is
    // returned byte-identical — idempotence + the identity-on-plain-sh half of dorc-sh's contract.
    if !has_marker(src) {
        return Carrier::pure(src.to_owned());
    }
    Carrier::pure(apply_edits(src, collect_strip_edits(interner, src)))
}

/// A stripped file paired with a stripped-line → original-line map (`27R` §4 dir-strip-then-lint:
/// the external-tool remap needs to name the user's ORIGINAL line even though the tool saw the
/// stripped bytes). Kernel-pure (`strip-is-pure-erasure`): the map is DERIVED from the same edits
/// `strip_file` applies — it changes no output byte.
#[derive(Debug, Clone)]
pub struct StripMapped {
    /// The stripped text — byte-identical to [`strip_file`]'s output for the same input.
    pub text: String,
    /// `line_map[i]` = the 1-based ORIGINAL source line of the `(i+1)`-th stripped line. Only
    /// whole-deleted annotation-LINES shift the numbering (`27R` §4); within-line edits (a bind
    /// rewrite, a trailing-mark deletion, the shebang rewrite) keep their line, so those entries
    /// are the identity. An unmarked file yields the pure identity map.
    pub line_map: Vec<u32>,
}

/// Strip a file AND return the stripped-line → original-line map (`27R` §4). The stripped `text`
/// is byte-identical to [`strip_file`] (both apply [`collect_strip_edits`]); the map is computed
/// from the SAME edits, so it can never disagree with the bytes. An unmarked file is the identity
/// (no edits, no shifts) — the uniform path that lets external tools remap marked and unmarked
/// files alike (`27R` §4 "unmarked files pass through strip unchanged by construction").
pub fn strip_file_with_map(interner: &mut Interner, src: &str) -> Carrier<StripMapped> {
    if !has_marker(src) {
        return Carrier::pure(StripMapped {
            text: src.to_owned(),
            line_map: identity_line_map(src),
        });
    }
    let edits = collect_strip_edits(interner, src);
    let line_map = line_map_from_edits(src, &edits);
    let text = apply_edits(src, edits);
    Carrier::pure(StripMapped { text, line_map })
}

/// Collect the FILE-ABSOLUTE `(lo, hi, replacement)` erasure edits for a marked file (the spans are
/// file-absolute; funcdefs are disjoint, so the regions never overlap). Shared by [`strip_file`] and
/// [`strip_file_with_map`] so the two can never diverge (single source of truth for the edit set).
fn collect_strip_edits(interner: &mut Interner, src: &str) -> Vec<(usize, usize, String)> {
    // Every role funcdef in the file. Each `lift_role` scans only its own `__<role>` suffix, so a
    // funcdef is collected exactly once; the six sets partition the file's role funcdefs. These are
    // pure, cheap re-lifts (the marker gate runs the identical six).
    let mut predicts: Vec<Predict> = Vec::new();
    for set in [
        lift_predicts(interner, src).value,
        lift_touches(interner, src).value,
        lift_verdicts_converged(interner, src).value,
        lift_resolvers(interner, src).value,
        lift_reaches(interner, src).value,
        lift_state_stored_only_in(interner, src).value,
    ] {
        for sym in set.providers() {
            if let Some(p) = set.get(sym) {
                predicts.push(p.clone());
            }
        }
    }
    let mut edits: Vec<(usize, usize, String)> = Vec::new();
    for p in &predicts {
        collect_file_strip_edits(&p.body, src, &mut edits);
    }
    if let Some(edit) = shebang_runner_edit(src) {
        edits.push(edit);
    }
    if let Some(edit) = marker_line_edit(src) {
        edits.push(edit);
    }
    edits
}

/// Apply the erasure edits back-to-front (sorted by descending `lo` so earlier offsets stay valid).
/// `inv-no-throw`: a non-char-boundary or out-of-bounds span is skipped, never panicked on.
fn apply_edits(src: &str, mut edits: Vec<(usize, usize, String)>) -> String {
    edits.sort_by_key(|e| core::cmp::Reverse(e.0));
    let mut out = src.to_owned();
    for (lo, hi, repl) in edits {
        if lo <= hi && hi <= out.len() && out.is_char_boundary(lo) && out.is_char_boundary(hi) {
            out.replace_range(lo..hi, &repl);
        }
    }
    out
}

/// The identity line-map for an unmarked file: `[1, 2, …, n]` for `n` source lines. `src.lines()`
/// under-counts by one when the file lacks a trailing newline, so a non-empty file always has at
/// least one line; a trailing-newline file's final empty segment is not a stripped line.
fn identity_line_map(src: &str) -> Vec<u32> {
    let n = count_output_lines(src);
    (1..=n).collect()
}

/// Count the lines a text presents to a downstream line-based tool: one per `\n`-delimited segment,
/// plus a final segment if the text does not end in `\n`. An empty string has zero lines.
fn count_output_lines(text: &str) -> u32 {
    if text.is_empty() {
        return 0;
    }
    let newlines = u32::try_from(text.bytes().filter(|&b| b == b'\n').count()).unwrap_or(u32::MAX);
    if text.ends_with('\n') {
        newlines
    } else {
        newlines.saturating_add(1)
    }
}

/// Build the stripped-line → original-line map from the erasure edits (`27R` §4). An original line
/// VANISHES iff a whole-deleted annotation-line covers it — encoded here as a deletion edit (empty
/// replacement) whose span covers the line's content start through its terminating newline. Only
/// those edits shift the numbering; within-line edits (a trailing-mark deletion covers only the
/// mark suffix, a bind rewrite carries a non-empty replacement, the shebang rewrite is non-empty)
/// never make a line vanish, so their original lines survive 1:1. Referent-agnostic and pure.
fn line_map_from_edits(src: &str, edits: &[(usize, usize, String)]) -> Vec<u32> {
    let deletions: Vec<(usize, usize)> = edits
        .iter()
        .filter(|(_, _, repl)| repl.is_empty())
        .map(|(lo, hi, _)| (*lo, *hi))
        .collect();
    let mut map: Vec<u32> = Vec::new();
    let mut line_no: u32 = 0;
    let mut line_start = 0usize;
    let bytes = src.as_bytes();
    for i in 0..=bytes.len() {
        let at_newline = bytes.get(i) == Some(&b'\n');
        let at_end = i == bytes.len();
        if at_newline || at_end {
            let content_start = line_start;
            let line_end_incl = if at_newline { i.saturating_add(1) } else { i };
            // A trailing `\n`+EOF leaves a final empty segment that is not a line.
            if !(at_end && content_start == i) {
                line_no = line_no.saturating_add(1);
                let vanished = deletions
                    .iter()
                    .any(|&(lo, hi)| lo <= content_start && hi >= line_end_incl);
                if !vanished {
                    map.push(line_no);
                }
            }
            line_start = i.saturating_add(1);
        }
    }
    map
}

/// Collect the erasure edits for a statement list, recursing through `case`/`if`/`while` bodies
/// (binds and marks nest there). File-absolute offsets. Mirrors the marker gate's `first_dialect_
/// span` walk and the per-funcdef `predict::collect_strip_edits` — but NON-mangling (the funcname is
/// never rewritten) and with the bare-mark whole-statement delete the probe-lane strip never needs
/// (`state_stored_only_in` is inert there, so it only reaches THIS whole-file pass).
fn collect_file_strip_edits(body: &[Stmt], src: &str, edits: &mut Vec<(usize, usize, String)>) {
    for stmt in body {
        match stmt {
            // A bind `name : kind [= value]` → `name=value` (verbatim name + value bytes; the
            // nullary Singleton form has no value ⇒ `name=`).
            Stmt::Annotation(a) => {
                let lo = a.span.lo.0 as usize;
                let hi = a.span.hi.0 as usize;
                let name = span_text(src, a.name_span);
                let value = a.value_span.map_or("", |vs| span_text(src, vs));
                edits.push((lo, hi, format!("{name}={value}")));
            }
            Stmt::Command(c) => {
                if let Some(m) = &c.mark {
                    if is_bare_colon_host(c) {
                        // A bare-mark statement: the `:` is only the mark's host. Delete the WHOLE
                        // line (leading indentation through the trailing newline) — never leave a
                        // bare `:` behind (the always-skip disaster).
                        let lo = line_lead_start(src, c.span.lo.0 as usize);
                        let hi = consume_trailing_newline(src, m.span.hi.0 as usize);
                        edits.push((lo, hi, String::new()));
                    } else {
                        // An ordinary command carrying a trailing mark: delete `[cmd-end .. mark-end]`
                        // (the inter-token whitespace + the mark), keep the command bytes verbatim.
                        let lo = c.span.hi.0 as usize;
                        let hi = m.span.hi.0 as usize;
                        edits.push((lo, hi, String::new()));
                    }
                }
                // The `dorc:` full-analysis-license prefix on a command word (`dorc:sh …`) → erase
                // the 5-byte `dorc:` prefix, keeping the runtime command. (Corpus-absent; see the
                // module scope note — honored where a marked body could carry it.)
                if let Some(dorc_prefix) = dorc_command_prefix_edit(c, src) {
                    edits.push(dorc_prefix);
                }
            }
            Stmt::Case { arms, .. } => {
                for arm in arms {
                    collect_file_strip_edits(&arm.body, src, edits);
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_file_strip_edits(then_body, src, edits);
                collect_file_strip_edits(else_body, src, edits);
            }
            Stmt::While { body, .. } => collect_file_strip_edits(body, src, edits),
            // Region-erase only: a whole-line delete would take the rest of the list with it.
            Stmt::AndOr(list) => {
                for r in &list.refused_marks {
                    edits.push((
                        r.host.hi.0 as usize,
                        r.mark.span.hi.0 as usize,
                        String::new(),
                    ));
                }
                for c in list.commands() {
                    if let Some(dorc_prefix) = dorc_command_prefix_edit(c, src) {
                        edits.push(dorc_prefix);
                    }
                }
            }
            Stmt::Assign { .. } | Stmt::Shift { .. } => {}
        }
    }
}

/// Is this command a bare `:` no-op whose sole purpose is to host a trailing mark (the
/// `state_stored_only_in` `undivided-by-transit-across` line)? Only then is the whole statement deleted.
fn is_bare_colon_host(c: &Command) -> bool {
    matches!(c.words.as_slice(), [Word::Literal(w)] if w == ":")
}

/// The `dorc:` prefix erasure edit for a command whose first word is a `dorc:`-prefixed literal
/// (`dorc:sh -c '…'` → `sh -c '…'`), or `None`. The word carries no span, so the prefix is located
/// at the command's start ([`Command::span`] `.lo`) — where the first word begins.
fn dorc_command_prefix_edit(c: &Command, src: &str) -> Option<(usize, usize, String)> {
    let Word::Literal(w) = c.words.first()? else {
        return None;
    };
    if !w.starts_with("dorc:") {
        return None;
    }
    let lo = c.span.lo.0 as usize;
    let hi = lo.saturating_add("dorc:".len());
    // Only if the source at that position really is the prefix (defensive — the span must align).
    (src.get(lo..hi) == Some("dorc:")).then(|| (lo, hi, String::new()))
}

/// Verbatim source text of a span (`inv-no-throw`: an out-of-bounds span yields `""`).
fn span_text(src: &str, span: Span) -> &str {
    src.get(span.lo.0 as usize..span.hi.0 as usize)
        .unwrap_or_default()
}

/// Walk back from `pos` over the line's leading `[ \t]` indentation, returning the index where the
/// indentation run begins (so a deleted whole-statement takes its indentation with it).
fn line_lead_start(src: &str, pos: usize) -> usize {
    let bytes = src.as_bytes();
    let mut i = pos;
    while let Some(prev) = i.checked_sub(1) {
        if bytes[prev] == b' ' || bytes[prev] == b'\t' {
            i = prev;
        } else {
            break;
        }
    }
    i
}

/// Extend `pos` over one trailing newline (`\n`, or `\r\n`) so a deleted whole-statement takes its
/// line terminator with it (no residual blank line).
fn consume_trailing_newline(src: &str, pos: usize) -> usize {
    let bytes = src.as_bytes();
    let mut i = pos;
    if bytes.get(i) == Some(&b'\r') {
        i = i.saturating_add(1);
    }
    if bytes.get(i) == Some(&b'\n') {
        i = i.saturating_add(1);
    }
    i
}

/// The line-1 shebang-runner rewrite (`274` §13 / `271:rul-dorc-prefix-head-synthesis`): a `#!`
/// shebang whose effective RUNNER is `dorc-sh` is rewritten to run under stock `sh`, so the stripped
/// artifact carries no dorc dependency. Returns the `(0, line1_end, replacement)` edit, or `None`
/// when line 1 is not a dorc-sh shebang (any other shebang — or none — is left untouched; the engine
/// never parses shebang CONTENT, `24P` §9 decision-strip-leaves-shebang). Parse-informed: it splits
/// the interpreter + args and identifies the runner, never a blind text substitution.
fn shebang_runner_edit(src: &str) -> Option<(usize, usize, String)> {
    let line1_end = src.find('\n').unwrap_or(src.len());
    let line1 = src[..line1_end].trim_end(); // tolerate a trailing \r
    let rest = line1.strip_prefix("#!")?.trim_start();
    let fields: Vec<&str> = rest.split_whitespace().collect();
    let first = *fields.first()?;
    let rewritten = if basename(first) == "env" {
        // `#!/usr/bin/env dorc-sh [args]` — the runner is env's first non-flag argument.
        let runner_pos = fields
            .iter()
            .position(|f| !f.starts_with('-') && *f != first)?;
        if basename(fields[runner_pos]) != "dorc-sh" {
            return None;
        }
        let mut new_fields = fields.clone();
        new_fields[runner_pos] = "sh";
        format!("#!{}", new_fields.join(" "))
    } else if basename(first) == "dorc-sh" {
        // A direct `#!/abs/path/dorc-sh` — no stock path corresponds, so rewrite to canonical sh.
        "#!/bin/sh".to_owned()
    } else {
        return None;
    };
    Some((0, line1_end, rewritten))
}

/// The final `/`-delimited segment of a path token (`/usr/bin/env` → `env`, `dorc-sh` → `dorc-sh`).
fn basename(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

/// The `# dorc-lang/v0.2` dialect marker line edit: the FIRST marker line within the same
/// `MARKER_WINDOW` [`has_marker`] honors is deleted whole (through its trailing newline). Human
/// ruling 2026-07-17 (`27D:disposition-strip-keeps-marker` → STRIP IT): a stripped off-ramp artifact
/// is no longer dialect text. `None` when unmarked (`strip_file` already gated, so a marked file
/// always yields `Some`). The marker sits near the top (line 1–2), disjoint from every funcdef-body
/// edit and from the shebang edit `(0, line1_end)` — the marker line starts past that newline.
fn marker_line_edit(src: &str) -> Option<(usize, usize, String)> {
    let mut offset = 0usize;
    for line in src.split_inclusive('\n').take(MARKER_WINDOW) {
        if line.trim_end() == MARKER {
            return Some((offset, offset.saturating_add(line.len()), String::new()));
        }
        offset = offset.saturating_add(line.len());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A marked oracle exercising every strip case: the dorc-sh shebang, a bind, a trailing verdict
    /// mark, a trailing emission mark, and — the load-bearing one — a `state_stored_only_in`
    /// `undivided-by-transit-across` bare-mark statement (a `:` no-op hosting a mark).
    const MARKED: &str = "#!/usr/bin/env dorc-sh\n\
# dorc-lang/v0.2\n\
apt_get__predict() {\n\
   pkg : sm.dorc.Package = \"$1\"\n\
   dpkg-query -W \"$pkg\" >/dev/null 2>&1 : sm.dorc.Package:\"$pkg\"@installed\n\
}\n\
sm_dorc_Package__state_stored_only_in() {\n\
   printf '/var/lib/dpkg\\n' : stored-in fs\n\
   : undivided-by-transit-across user\n\
}\n";

    fn strip(src: &str) -> String {
        strip_file(&mut Interner::default(), src).value
    }

    #[test]
    fn plain_sh_passes_through_byte_identical() {
        // Marker-gated: an unmarked file (plain sh, or an already-stripped artifact) is untouched.
        let plain = "#!/bin/sh\napt_get__predict() { dpkg-query -W \"$1\"; }\nls -la\n";
        assert_eq!(
            strip(plain),
            plain,
            "an unmarked file is returned byte-identical"
        );
    }

    #[test]
    fn marked_file_erases_binds_marks_and_shebang() {
        let out = strip(MARKED);
        assert!(
            !out.contains("dorc-sh"),
            "the dorc-sh shebang runner is gone:\n{out}"
        );
        assert!(
            out.contains("#!/usr/bin/env sh"),
            "rewritten to stock sh:\n{out}"
        );
        assert!(
            out.contains("pkg=\"$1\""),
            "the bind became a plain assignment:\n{out}"
        );
        assert!(
            !out.contains(": sm.dorc.Package") && !out.contains(": fs"),
            "every trailing mark is erased:\n{out}"
        );
        assert!(
            out.contains("dpkg-query -W \"$pkg\" >/dev/null 2>&1\n"),
            "the marked command's bytes survive verbatim, mark gone:\n{out}"
        );
        assert!(
            out.contains("printf '/var/lib/dpkg\\n'\n"),
            "the emission command survives, its `: fs` mark gone:\n{out}"
        );
        // The `# dorc-lang/v0.2` marker line IS erased (human ruling 2026-07-17,
        // `27D:disposition-strip-keeps-marker` → STRIP IT): a stripped artifact is not dialect text.
        assert!(
            !out.contains("# dorc-lang/v0.2"),
            "the marker line is erased:\n{out}"
        );
    }

    #[test]
    fn bare_mark_statement_deleted_whole_never_left_as_colon() {
        // THE load-bearing pin (`strip-is-pure-erasure`, `23H` §9.4): a `:` no-op hosting a mark is
        // an annotation-LINE, deleted WHOLE. If it were left as a bare `:` it would clobber the
        // body's tool-rc to 0 — an always-skip guard, the disaster shape. The author's last
        // SUBSTANTIVE command (`printf …`) must stay the body's last status-affecting statement.
        let out = strip(MARKED);
        assert!(
            !out.contains("invariant"),
            "the invariant bare-mark line is gone:\n{out}"
        );
        assert!(
            !out.lines().any(|l| l.trim() == ":"),
            "no bare `:` null-command remains (would be an always-skip guard):\n{out}"
        );
        // Structurally: the last statement of the state body is the `printf`, immediately followed
        // by the closing brace — no `:` between them.
        assert!(
            out.contains("printf '/var/lib/dpkg\\n'\n}\n"),
            "printf is the state body's last status-affecting statement:\n{out}"
        );
    }

    #[test]
    fn dorc_sh_shebang_runner_rewritten_to_sh() {
        let out = strip(MARKED);
        assert_eq!(
            out.lines().next(),
            Some("#!/usr/bin/env sh"),
            "line 1 is the stock-sh shebang"
        );
    }

    /// An and-or list ships BYTE-EXACT: the operator, both items, and their redirects survive the
    /// strip untouched. This is the corpus's own resolver line, which is also the `dorc_sh_smoke`
    /// strip anchor — a list that failed to lift would take a working resolver with it.
    #[test]
    fn an_and_or_list_strips_byte_exact() {
        let src = "#!/usr/bin/env dorc-sh\n# dorc-lang/v0.2\nsm_dorc_Package__resolve() {\n   dpkg-query -W -f '${Package}\\n' -- \"$1\" 2>/dev/null || printf '%s\\n' \"$1\"\n}\n";
        let out = strip(src);
        assert!(
            out.contains(
                "   dpkg-query -W -f '${Package}\\n' -- \"$1\" 2>/dev/null || printf '%s\\n' \"$1\"\n"
            ),
            "the list survives verbatim:\n{out}"
        );
    }

    /// A REFUSED mark still erases. The list is not a mark host any more, but its bytes are still
    /// dialect text, and an off-ramp artifact that shipped one would not be dialect-free.
    #[test]
    fn a_refused_mark_on_a_list_still_erases() {
        let src = "# dorc-lang/v0.2\nw__predict() {\n   thing : sm.dorc.Thing = \"$1\"\n   w q \"$thing\" : sm.dorc.Thing:\"$thing\"@present || return 2\n}\n";
        let out = strip(src);
        assert!(
            !out.contains("sm.dorc.Thing:"),
            "no mark survives the strip:\n{out}"
        );
        assert!(
            out.contains("   w q \"$thing\" || return 2\n"),
            "the list's own bytes are kept:\n{out}"
        );
    }

    #[test]
    fn strip_is_idempotent() {
        // strip(strip(x)) == strip(x): the once-stripped artifact is now marker-FREE (the marker line
        // is erased), so the second pass hits the marker gate's early return and is byte-identity.
        let once = strip(MARKED);
        assert!(
            !once.contains("# dorc-lang/v0.2"),
            "once-stripped is marker-free"
        );
        assert_eq!(strip(&once), once, "strip is idempotent");
    }

    #[test]
    fn dorc_prefix_erased_from_command_word() {
        // `dorc:sh …` → `sh …` (the `274` full-analysis-license prefix). Placed inside a lifted role
        // body so the whole-file walk reaches it (the module scope note covers book-level `dorc:`).
        let src = "# dorc-lang/v0.2\nfoo__predict() {\n   dorc:sh -c 'echo hi'\n}\n";
        let out = strip(src);
        if out.contains("sh -c 'echo hi'") {
            assert!(
                !out.contains("dorc:sh"),
                "the `dorc:` prefix is erased, the runtime command kept:\n{out}"
            );
        }
        // If the dialect lexer declines `dorc:sh` (out-of-dialect ⇒ the funcdef does not lift), the
        // construct simply is not reached — the corpus-absent limitation the module documents; the
        // test does not force a lift that may not exist.
    }

    fn strip_mapped(src: &str) -> StripMapped {
        strip_file_with_map(&mut Interner::default(), src).value
    }

    #[test]
    fn line_map_is_identity_for_unmarked_file() {
        let plain = "#!/bin/sh\nfoo() { :; }\nls -la\necho done\n";
        let m = strip_mapped(plain);
        assert_eq!(m.text, plain, "unmarked file is byte-identical");
        assert_eq!(
            m.line_map,
            vec![1, 2, 3, 4],
            "identity map for an unmarked file"
        );
    }

    #[test]
    fn line_map_shifts_only_on_whole_deleted_annotation_lines() {
        // `27R` §4: only the whole-deleted marker (orig 2) + `undivided-by-transit-across` bare-mark (orig 9) shift.
        let m = strip_mapped(MARKED);
        assert_eq!(
            m.line_map,
            vec![1, 3, 4, 5, 6, 7, 8, 10],
            "only the marker + bare-mark lines shift the numbering:\n{}",
            m.text
        );
        let stripped_lines: Vec<&str> = m.text.lines().collect();
        let orig_lines: Vec<&str> = MARKED.lines().collect();
        assert_eq!(
            stripped_lines.len(),
            m.line_map.len(),
            "one map entry per stripped line"
        );
        assert_eq!(m.line_map.get(3).copied(), Some(5));
        assert!(stripped_lines[3].contains("dpkg-query -W"));
        assert!(
            orig_lines[4].contains("dpkg-query -W"),
            "original line 5 is the dpkg-query"
        );
    }

    #[test]
    fn shellcheck_directive_not_detached_by_deleted_annotation_line() {
        // `27R` §8 pin: a deleted bare-mark line between a `# shellcheck disable=` directive and its
        // target must leave them ADJACENT (no residual blank line, or the directive suppresses nothing).
        let src = "# dorc-lang/v0.2\n\
foo__state_stored_only_in() {\n\
# shellcheck disable=SC2086\n\
: undivided-by-transit-across fs-view\n\
printf 'x\\n'   : stored-in kernel\n\
}\n";
        let m = strip_mapped(src);
        assert!(
            m.text
                .contains("# shellcheck disable=SC2086\nprintf 'x\\n'\n"),
            "the directive is immediately followed by its target (annotation line deleted, no blank line):\n{}",
            m.text
        );
        assert!(
            !m.text.contains("invariant"),
            "the bare-mark invariant line is gone:\n{}",
            m.text
        );
    }

    #[test]
    fn dorc_sh_runtime_object_command_word_is_untouched() {
        // Row-3 documented-dangle (`274` §1 / `dorc-sh-trio`): `dorc-sh` typed DIRECTLY as a command
        // word is the runtime object, NOT an annotation — strip must leave it byte-for-byte (only the
        // `dorc:` colon-prefix is erasable; `dorc-sh` starts with `dorc-`, never matched). Half-strip
        // is worse than no-strip: the author's documented buy-in is that a post-uninstall `dorc-sh`
        // dangles loud-127 identically under bash/sh/perl.
        let src = "# dorc-lang/v0.2\nfoo__predict() {\n   dorc-sh -c 'echo hi'\n}\n";
        let out = strip(src);
        assert!(
            out.contains("dorc-sh -c 'echo hi'"),
            "the typed `dorc-sh` runtime object is left untouched (documented-dangle):\n{out}"
        );
    }
}
