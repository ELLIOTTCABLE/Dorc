# 21N (fork f1) — render-refusal pierces the omit-safety gate: confirmed, fixed. Strain + worlds.

> Round-21 fork f1-omitsafety, builder note. Charter: verify-or-kill the hostile-crosscheck
> finding "a heredoc-bearing Query guard's render refusal leaves a live guard over a
> `:`-substituted Omit body". Append-only; confidence-marked. Engine edit: `plan/src/lib.rs`
> (`is_neutralised` ×2 arms + docs). Tests: `plan/tests/observable_matrix.rs` (+5),
> 3 e2e cases (`omitsafe21-*`). HEAD before task: `e512cc3`. (Slug 21N picked from the free
> letters at fork time — if a sibling fork collided, the `f1` in the filename disambiguates.)

## §0 Headline

+SURE (executed, not just traced): the finding is REAL and was a live defect at `e512cc3`.
`dpkg -s nginx <<EOF >/dev/null 2>&1 || apt-get install -y nginx` (heredoc parses onto the
guard leaf; valid known-rc pkgstate Query; probe holds rc 0) produced, pre-fix:
guard `Replace(QueryGuard)` (license-time never consults the refuse-set) → heredoc
REFUSES the guard's edit (`error[render-heredoc-refused]`, guard kept verbatim/LIVE) →
body `Omit{controller=guard}` → `is_neutralised` read the guard's DISPOSITION (Replace ⇒
neutralised) and never the refusal → body edited to `:`. Artifact:
`dpkg -s nginx <<EOF >/dev/null 2>&1 || :` — a physically-kept, live-re-deciding guard over
an omitted body, verbatim the configuration the `Disposition::Omit` doc forbids. Worse, the
`:`-edit is UNDISCLOSED on that line (`comment_safe` drops provenance comments on `<<`
lines). Constructible in BOTH AndOr forms, BOTH polarities' dead cells, AND the if-form
(compound controller via `subtree_leaves_all`): `||`+holds, `&&`+absent, `if G; then :;
else B; fi`+holds all yielded live-guard-over-`:`.

## §1 The world-trace (against TOCTOU-WONTFIX)

Renders compared: bare book `G ∘ B`; frozen-both (intended) `true || :` / `false && :`;
pre-fix live-guard `G ∘ :`; post-fix = bare book. Worlds = apply-time guard agrees /
flipped / errors. sh semantics pinned with builtin-only probes (and note 215's dash runs).

**`||` form (probed rc 0, body dead):** list rc is 0 in EVERY world for BOTH frozen-both
and live-guard (`G(rc1) || :` ⇒ `:` ⇒ 0; `true || :` ⇒ 0); `$?`, errexit (left-operand
exempt), script exit rc all identical; body runs in NEITHER. +SURE outcome-equivalent on
all rc-observables — the orchestrator's suspicion confirmed. Residual deltas: the guard
executes (time, heredoc stdin read, its own terminal bytes — the same delta-class every
sanctioned Replace already accepts, reversed), the artifact-readability lie, and the stated
gate-contract breach.

**`&&` form (probed rc 1, body dead): GENUINE rc divergence.** Flipped world (guard rc 0 at
apply): live-guard `G && :` ⇒ list rc 0 — "guard held AND body succeeded" with the body
never run. Frozen-both ⇒ rc 1 (honest-stale); bare book ⇒ runs B (rc = B's own; B failing
⇒ errexit ABORT — B is the list's LAST command, not exempt). No baseline world produces
the pre-fix composite: a fabricated rc-0 for a never-run body, reachable by live readers
(`$?`, script exit status — the e2e harness itself consumes exit rc; enclosing consumers).
That is `inv-probe-sourced-values` pierced via the render seam — the `:`'s rc-0 was
licensed as unreachable-behind-frozen-controller, and the refusal un-froze the controller.
Guard-errors world diverges too (live rc 2 vs frozen rc 1). BEYOND the sanctioned TOCTOU
class: WONTFIX sanctions stale *decisions*, not fabricated *observables*.

## §2 The fix (minimal, render-side)

`is_neutralised` now answers its own doc honestly ("rendered form reproduces the decision
without running it"): a `Replace` counts ONLY if `!leaf_has_heredoc` — both the leaf arm
and the compound-walk pred. Two functional lines; dispositions untouched (`--debug-argv`
still shows `omit`; the fix is the same render-side gate kept-`Run`-controller Omits
already take). An `Omit`-disposed heredoc node deliberately keeps NO check: its verbatim
text sits BEHIND its own controller's frozen short-circuit (transitive recursion is the
honest gate), so the heredoc-leaf-inside-dead-block cell keeps its sound elisions
(pinned). `leaf_has_heredoc`'s doc now names its three lockstep consumers (collect_edits /
refusal-diagnostics / is_neutralised); a future refusal class must extend all three.

Poles pinned: unit ×5 (`refused_heredoc_guard_keeps_dead_{oror,andand}_body_verbatim`,
`…in_if_cond…`, `clean_guard_still_elides_dead_body_heredoc_sibling_stays_unreachable_verbatim`,
`clean_query_guard_still_renders_dead_body_as_colon`) + e2e ×3 (`omitsafe21-heredoc-guard-
keeps-body` agree-world; `omitsafe21-heredoc-guard-flipped-runs` ||-flipped revives the
install; `omitsafe21-heredoc-and-flipped-runs` &&-flipped revives the reload — the
PROBE_RESULTS=authored marker simulating probe→apply TOCTOU, mocks = apply-time world).
All 3 e2e FAIL at pre-fix HEAD (verified by stash/rebuild: exec-gate wrong-run-set ×2,
content diff ×1). Zero churn to the 93 existing cases pre- AND post-fix. The elision lost
is value-free: the live guard runs regardless, so the `:` saved zero remote commands in
the agree world and under-executed in the flipped one.

## §3 Flags

- **tc-lost-elision-profile** (informational): the fix removes the heredoc-guard-controlled
  body elision. ~SUSPECT zero real cost (above); flagged because it IS a licensed-elision
  removal.
- **tc-heredoc-diag-dead-region** (cosmetic, pre-existing): a heredoc Omit leaf inside a
  genuinely-dead block still emits `render-heredoc-refused` saying "it runs verbatim" —
  misleading (it is kept verbatim but UNREACHABLE). Untouched (out of charter).
- **tc-provenance-undisclosed-on-heredoc-lines** (pre-existing, narrowed by the fix): any
  edit landing on a `<<`-bearing rendered line gets no provenance comment (`comment_safe`).
  Post-fix no edit can land on the hazard line, but the general comment-drop class stands.
- --WONDER: `leaf_has_heredoc` matches `Subshell`/`Group` redirs too; a heredoc-on-subshell
  GUARD (`( dpkg -s x ) <<EOF || B`) is NOT a hazard (the edit lands on the inner Simple,
  heredoc untouched, decision reproduced) — the compound walk correctly checks only Simple
  leaves. Traced, not exec-pinned.
