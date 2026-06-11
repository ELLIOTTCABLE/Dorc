# 21E — arch-1 P1 fix: adjacent multi-line span-edit orphan / comment-in-quote corruption

Fixes the P1 a hostile crosscheck of commit `140c303` found and note 214 §9 hunt-7 had
flagged ("a multi-line edit whose region contains ANOTHER edit"). Worktree base `92162f1`
(span-edit render). Scope was surgical: `crates/plan/src/lib.rs` (the `emit_span_edits` region
+ `comment_safe`), plan tests, one new e2e case, this note. No `render.rs` change was needed
(its `provenance_comment` GUARANTEE doc already says the CALLER owns the comment-safety
precondition — which is exactly what f-2 hardens).

## §1 The P1, reproduced against the real binary + dash

Book (`;`-separated so the SECOND install STARTS on the FIRST's closing line):

```
apt-get install -y "a
b"; apt-get install -y "c
d"
```

Both operands carry a literal newline; both probe `effect=holds` ⇒ the engine licenses BOTH
(`--debug-argv`: `argv 0 replace`, `argv 1 replace`). PRE-FIX rendered apply:

```
true; apt-get install -y "c   # dorc: elided [apt-get install -y "a b"] (already converged / dead branch)
d"
```

— `dash -n` rc=0 (CLEAN BY QUOTE COINCIDENCE), but under argv-echo mocks the SECOND install
RUNS for real (a silent over-execute — `kFAIL-perform` violated in the wrong direction: we
elided nothing, we ran a converged mutator AND corrupted its operand). Variant with an odd
embedded quote in operand 1 (`"a'b<LF>x"`): the spliced comment's `'` flipped quote-state ⇒
hard `dash -n` "Unterminated quoted string" (rc=2). Both verified at HEAD with
`target/debug/dorc.exe` piped to `/bin/dash`.

### Mechanism (three interacting defects, all in `emit_span_edits`)

The source has 3 lines: `0: …"a` / `1: b"; … "c` / `2: d"`. Edit A (install-1) spans lines
0–1; edit B (install-2) spans lines 1–2.

- **(1) orphan** — `emit_span_edits` keyed `by_line` by each edit's START line and tracked
  `consumed_through[start]`. Edit A landed in `by_line[0]` with `consumed_through[0]=1`. The
  line-walk visited `i=0`, spliced A, then jumped `i = last_consumed + 1 = 2` — **skipping line
  1 entirely**, where `by_line[1] = [B]` lived. Edit B was never visited ⇒ never applied.
- **(2) half-splice** — A's spliced region was `[line0_start, line1_end)`, which includes B's
  leading bytes (`b"; apt-get install -y "c`). A's replacement covered only A's own span
  `[A.lo, A.hi)`, leaving `; apt-get install -y "c` live after the `true`, and `d"` rendered raw
  on line 2.
- **(3) comment-in-quote** — `comment_safe` only rejected a trailing `\` or a `<<`. The
  half-spliced region ended `… install -y "c` — inside an open double-quote — so the provenance
  comment was appended INSIDE B's string literal.

## §2 The fix (f-1 + f-2, defense-in-depth)

**f-1 — region-grouping** (`group_edits` + rewritten `emit_span_edits`). Process edits in
line-overlap GROUPS, not by single start line. A group is the transitive closure of edits whose
covered line-ranges intersect/abut: sweeping the (already `lo`-sorted) edits, the next edit
joins the running group iff its start line ≤ the group's current last line, else it opens a new
group. The group's region spans first-member-start-line-start → last-member-end-line-end; ALL
members splice right-to-left over that one region (span-disjoint per `normalise_edits`, so byte
arithmetic stays absolute and total-ordered), emitted as ONE rendered line; the provenance
comment carries EVERY member's original (the emitter already took a Vec). The walk jumps past
the group's whole line span — but now no edit is stranded, because a stranded edit would have
been IN the group. Invariant: `spliced_count == edits.len()` (`debug_assert_eq!` tripwire;
structurally guaranteed because each edit lands in exactly one group and the walk visits every
group — consecutive groups never share a line, so the post-group jump lands at or before the
next group's first line).

**f-2 — harden `comment_safe`** (`region_ends_in_quote`). A minimal POSIX quote-state machine
over the RENDERED region: single-quote (everything literal until the next `'`), double-quote
(`\` escapes the next byte, `"` closes), unquoted backslash (escapes the next byte, so `\'`/`\"`
are literals not toggles). Refuse the comment if the scan ends inside any quote (or on a
dangling unquoted `\`). Kept the `<<` rejection; no heredoc/expansion parsing (out of f-2
scope). When refused: DROP the COMMENT, never the EDIT (the existing artifact-over-prose rule;
the OOB verdict lane still carries the disclosure). f-2 is defense-in-depth: f-1 removes the
orphan that PRODUCED the open-quote line, but a genuinely quote-crossing rendered line can also
arise when a VERBATIM `Run` leaf opens a quote on a group's first line (e.g. `true; systemctl
reload "c` where the systemctl spans two lines) — f-2 catches that too.

### Fixed renders (verified, both reproducers + cousins)

```
# plain P1:
true; true   # dorc: elided [apt-get install -y "a b"; apt-get install -y "c d"] (already converged / dead branch)

# odd-quote variant (was a hard dash -n break):
true; true   # dorc: elided [apt-get install -y "a'b x"; apt-get install -y "c d"] (already converged / dead branch)
```

Both: `dash -n` rc=0, exec rc=0 under argv-echo mocks, run-set EMPTY, both leaves substituted.

## §3 What I did NOT change, and why

- **`normalise_edits`** — untouched. Its sort + partial-overlap-`debug_assert` +
  containment-drop are still correct and are now a *precondition* f-1 relies on (members are
  span-disjoint within a group, so right-to-left splice is sound). The P1 was NEVER a
  normalise-overlap bug (the two leaf spans ARE disjoint — `"a\nb"` ends before `"c\nd"`
  begins); it was purely the EMIT-side line-keying. ~SURE this is the right boundary.
- **`collect_edits` / `render_apply` / `render_refusal_diagnostics`** — untouched. The edit SET
  was always correct; only its emission was broken.
- **`render.rs` `provenance_comment` / its GUARANTEE doc** — untouched. The doc already states
  the caller owns the comment-safety precondition; f-2 strengthens the caller's check to match
  what the doc already promised. +SURE no render.rs edit was warranted.
- **The multi-line *original*-flatten** (`provenance_comment`'s `split_whitespace().join(" ")`)
  — unchanged. It already correctly collapses a disclosed original's interior newline to a space
  (tc-multiline-original-flatten, 214 §10). f-1 just feeds it MORE originals per line; the flatten
  still applies per-original.
- **No new ⊤-trigger, no channel change, no disposition-layer touch.** This is a pure
  render-emission fix; the engine's elision decisions are byte-identical (the `--debug-argv`
  dispositions are unchanged — both installs were always `replace`; we only stopped corrupting
  the artifact that expresses that).

## §4 Tests (f-3)

Unit (`crates/plan/src/lib.rs` test mod): `region_ends_in_quote_tracks_posix_quote_state`
(balanced/closed/nested/escaped vs open single/double, dangling `\`, the odd-quote shape);
`comment_safe_refuses_open_quote_line` (the f-2 wiring on top of `\`/`<<`);
`group_edits_merges_abutting_multiline_edits` (the two-edit abut → one group, hand-built spans);
`group_edits_keeps_disjoint_edits_separate` (no over-merge across a blank line).

Integration (`crates/plan/tests/observable_matrix.rs`):
`render_adjacent_multiline_elides_both_no_orphan` (the plain P1 book, byte-for-byte assertion on
`true; true   # dorc: elided […; …]`); `render_adjacent_multiline_odd_embedded_quote_no_dashn_break`
(the odd-quote variant); `render_multiline_then_single_line_orphan_cousin_both_elide` (the P3
single-line-orphan cousin — multi-line install then `; install curl` on its closing line);
`render_verbatim_run_leaf_opening_quote_drops_comment_f2` (f-2 comment-drop when a verbatim
`Run` leaf opens a quote on the shared line). The `debug_assert_eq!` count invariant fires under
the debug test build if any of these ever re-orphans.

E2e: `render21-adjacent-multiline-elides` (new — the plain P1 book, both converged ⇒ both
substitute, run-set EMPTY, hand-derived golden; passes the dash-n gate, exec gate, gate-1 probe
parity+vouch with no `PROBE_RESULTS=authored` marker, gate-5 argv-echo differential — both sites
`replace` so the run-only filter skips them, and the stderr floor). Corpus 75 → 76, zero churn
to existing goldens.

## §5 Acceptance — full gate set green ×2

From `spike/`: `cargo fmt --check` clean; `cargo clippy --workspace --all-targets -- -D warnings`
clean (debug AND `--release`); `cargo test --workspace` all green (plan: 45 unit + 33 integ, +8
mine; workspace ~380 tests, 1 pre-existing `#[ignore]`); `sh e2e/run.sh` ×2 all 76 green; `typos
spike` (from worktree root) clean. Zero existing-golden churn beyond the one new case.

## §6 Residual hunt-list (verified-safe where checked; flagged where deferred)

- **res-1 three-deep adjacent multi-line chains — VERIFIED SAFE.** `apt-get install -y "a\nb";
  apt-get install -y "c\nd"; apt-get install -y "e\nf"` (all converged) renders `true; true; true
  # dorc: elided [a b; c d; e f]`, dash-clean, run-set empty. The transitive-closure sweep grows
  one group across all three (each edit's start line ≤ the running last line). +SURE no
  arbitrary-depth chain orphans (the sweep is O(n) and exhaustive).
- **res-2 a group spanning a SCAFFOLDING-KEYWORD line — VERIFIED SAFE for the leading-keyword
  case.** `for x in a b; do echo "$x"\ndone; apt-get install -y "c\nd"; apt-get install -y "e\nf"`
  renders the loop verbatim then `done; true; true   # dorc: elided […]`, dash-clean, only the
  loop body runs. The group's region is `[done-line-start, last-install-end)`; `done` is
  REGION-PREFIX bytes (not an edit span), so it survives verbatim — the same mechanism the
  existing `post_loop_install_sharing_done_line` pin relies on, now composed with grouping.
  ~SUSPECT the UN-tested sub-case is a keyword *between two edits' interior consumed lines* (a
  keyword on a line strictly inside a multi-line operand) — but a keyword can't be inside a
  quoted operand's continuation by construction (it'd be quoted text, not a token), so I
  --WONDER if that shape is even reachable; flag for a future crosscheck rather than claim it
  impossible.
- **res-3 comment-drop disclosure-loss accounting.** When f-2 drops the comment (region ends in
  an open quote), the elision's disclosure is LOST FROM THE ARTIFACT TEXT for that line. Today
  this only happens when a VERBATIM `Run` leaf opens the quote (an elided Replace sharing the
  line would itself be inside the group and its closing quote spliced) — so the dropped
  disclosure is for a Replace whose neighbour runs; the Replace's elision is still in the OOB
  `$DORC_VERDICT` lane (the standing rule). ~SUSPECT acceptable (artifact-correctness dominates),
  but it IS a real disclosure-fidelity gap: a human reading ONLY the artifact text sees `true;
  systemctl reload "c` with no note that a converged install was elided into that `true`. Flagged
  as the honest cost of the conservative drop (mirrors tc-multiline-original-flatten's
  disclosure-fidelity flag). If disclosure-completeness is ever weighted, the fix is a SEPARATE
  comment line BEFORE the group (a `# dorc: …` on its own line, always quote-safe) rather than a
  trailing one — deferred, not done (it'd churn every existing golden's comment placement).

## §7 Confidence summary

- +SURE: both P1 reproducers + the P3 cousin + the three-deep chain render dash-clean with
  empty run-sets, verified against the real binary and `/bin/dash`; full gate set green ×2; the
  fix is render-emission-only (zero disposition change, `--debug-argv` byte-identical).
- +SURE: f-1's grouping is exhaustive (every edit in exactly one group, every group visited) and
  the count invariant pins it; `normalise_edits`'s disjointness precondition is unchanged and
  still holds.
- ~SUSPECT: the f-2 quote machine is correct for the modeled POSIX subset (single/double/
  backslash); it does NOT model `$(...)`/backtick/parameter-expansion nesting, but those don't
  change whether a TRAILING `#` is inside a string literal (an unclosed `$(` is its own ⊤-reject
  upstream, never a rendered leaf), so I believe the subset is sufficient for the comment-safety
  question. A hostile crosscheck should probe a rendered line ending inside `"$( …` (I expect
  `region_ends_in_quote` returns true via the open `"`, conservatively dropping — safe).
- --WONDER (res-2 sub-case): whether a scaffolding keyword can ever land on a line strictly
  interior to a multi-line edit's consumed span in a way that the region-prefix-verbatim
  treatment mishandles. I could not construct one (interior lines of a quoted operand are quoted
  text), but I did not exhaustively prove it unreachable.
