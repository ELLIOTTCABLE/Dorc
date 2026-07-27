# 26I — adversarial kernel review of the r26 rounds (clean-context, disowned-third-person)

AI-authored (Fable-class adversarial reviewer, clean context, 2026-07-27). Object under review:
`ai/r26-analyzer-findings` @ `9765d961` — the ~54-commit accumulation carrying W-A (loud
degrades), W-D (and-or gate forms), W-B (verdict-mark coordinate keying), and W-C (the validity
fixpoint / erasure ledger). Method: full re-read of the license/erasure/fold/keying code against
the registries and `26G`/`26H` (treated as claims), plus read-only experiments through the built
binary with hand-fed framed records (no artifact was ever executed). Suite state at review:
`mise run build` + `mise run test` green — 1562 nextest + 14 doctests, 0 failures, 1 pre-existing
`#[ignore]`d SPEC test (see obs-redirect-target-cmdsub-standing-debt below).

Verdict in one line: the r26 correctness machinery is substantially better-built than the brief
feared — the fence/license factoring (`query_substitutes`, one seat two readers) is real and I
could not break it — but ONE live wrong-yes-capable hole exists at HEAD (statement-level
state-mutating builtins in oracle bodies), and ONE load-bearing cross-mechanism coupling is
undocumented and unpinned (fixpoint monotonicity ⇐ merge ⊤-paranoia).

## Findings, most consequential first

### fnd-state-builtins-silently-mis-key (SEV: wrong-yes-capable — the `271:rul-sin-ordering`
### pope-sin class; CONFIDENCE: demonstrated at the probe surface, +SURE on the licensing chain)

The predict/verdict tracer parses `set`, `unset` (and any other state-mutating builtin that is
not a dialect keyword) as ORDINARY plain commands: it ships their bytes into the probe body and
walks on **without applying their effect to its own positional/variable state**. The static
coordinate key and the host-measured referent then diverge, silently — the same
statically-keyed-cell ≠ host-measured-referent disaster class as
`26G:§FINDING-andand-resolves-a-wrong-coordinate` (R-5), whose family `26H` §1 declared closed
("no known wrong-yes-capable defect remains open as of `6300dd78`"). The lexer instances were
closed; the statement-level instances were not.

Demonstrated (probe render, `dorc probe`, nothing executed; oracle body abridged):

```sh
foo__predict() {
   set -- alpha
   pkg : sm.dorc.X = "$1"
   dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.X:"$pkg"@present
}
```

Against book `foo install`, the emitted probe reads, verbatim:

```
# site 0: sm.dorc.X:install@present        <- static key: entity "install" (site argv)
foo__predict() {
   set -- alpha
   pkg="$1"                                 <- host binds "alpha"
   dpkg-query -W "$pkg" >/dev/null 2>&1     <- host measures ALPHA's state
}
```

The record for cell `X:install@present` carries alpha's measured rc. Zero diagnostics on any
surface; `dorc lint` is clean on the file (LF-verified). Same shape demonstrated for `unset pkg`
(key = bound entity, host probes the empty string) and `eval "pkg=other"` (key = bound entity,
host probes "other").

Reaching under-execution end-to-end (traced, not executed): the ESTABLISH lane is fenced — a
mis-keyed converged establish still cannot elide without a `ByVouch` (the elide-weld held every
time I poked it). The open door is the QUERY lane: `prove_query_replaceable` needs no vouch
(read-reproduction), so a `:?`-observe body carrying a `set --` between bind and probe keys the
book's entity while measuring another referent; a holds-rc folds the `||` fallback dead off the
wrong cell — priority-1 under-execution — and since W-C the wrong fold now CASCADES (each
erasure is one more wrong rung), while `dorc why` renders a confident chain pointing at the
authored line — mis-attribution on top.

Reachability honesty: requires the oracle author to mutate positionals/vars between bind and
probe. `set -- $list` is THE idiomatic POSIX list workaround in exactly the dialect this project
mandates, so this is not exotic authoring. It is author-side error-free — no contract clause
warns against it — so under `correctness-is-contract-bounded` this failure is currently
*nobody's fault*, the category `IMPLEMENTATION.md` names as necessarily ours.

Where the fence belongs, mechanically (`oracle/src/predict/eval.rs` + `parser.rs`): the parser's
keyword dispatch (`parse_stmt` :484) already shields statement-`shift` (modeled, `run_shift`
:496) and refuses assignment-led and-or lists, and `is_rc_forging_head` (:255) shows the
known-heads pattern — but nothing walls the plain-command fallthrough for {`set`, `unset`,
`export`, `readonly`, `eval`, `exec`, `.`, `trap`, quoted-`'shift'` (at_keyword :285 requires
unquoted, and sh still runs a quoted builtin)}. The cheap sound fix is ⊤-degrade on those heads
in command position (`TopReason::UnmodeledStatement` already exists post-W-A), modeling
`set --`-with-literal-words later only if the idiom proves load-bearing. `read VAR` and
`export`-style var-touching degrade safe already (an unbound var ⇒ `UnresolvedAnnotationValue`);
the dangerous subset is precisely mutation of state the tracer ALREADY resolved.

Sub-finding, same door: **the dialect's "authored `eval` never" law is unenforced at lift** —
`dorc lint` flags eval (`syntax-unsupported`, advisory), but the predict lift accepts it and
ships it inside the probe artifact. Two parsers, opposite verdicts, and the licensing one is the
permissive one.

### fnd-monotonicity-rests-on-merge-paranoia (SEV: no live unsoundness — a latent re-opening of a
### wrong-yes under a plausible future edit; CONFIDENCE: high — traced end-to-end + empirically
### corroborated)

`settle_validity_fixpoint` (`cli/src/main.rs:5995`) iterates `ledger := ledger ∪
proofs(view(ledger))` and stops when no NEW proof appears. It never re-derives old entries at
settle: the final artifacts are built from the settled classes + the settled fact-view, while
the ledger's entries were each justified only under the ROUND-k view that minted them. This
overshoot shape is sound iff `proofs()` is monotone in the view — and the doc-comment's argument
("erasure only removes invalidators ⇒ more queries valid ⇒ more folds") SKIPS the step where the
view is built: `facts_from_sites` re-merges per round with per-round validity, so revealing a
previously-withheld sibling rc changes the cross-site meet.

The hole this would open (traced): fact F measured at sites A and B; round 1 has A valid
(rc 0) and B invalid; F folds Value(0), an opaque wall behind `A-guard || hork` is proven dead
and ERASED; round 2 validates B, whose recorded rc disagrees; F meets to ⊤; the settle loop
finds no new proofs and returns — with hork still erased in the model (Run, renders LIVE,
verbatim guard) while every downstream elision minted off its erasure stands, and the why-chain
attributes them to a proof the final view itself no longer believes. Under-execution plus
mis-attribution.

Why it is NOT reachable at HEAD: `merge_observable` (`main.rs:6286`) treats `Value` vs `Top` as
disagreement, so a fact with ANY not-yet-revealed sibling is already status-⊤ in round 1 — a
clean merged Value requires every sibling already valid and agreeing, and validity only grows,
so a clean fact-view can never degrade across rounds. Empirically corroborated: both the
disagreeing-sibling AND the agreeing-sibling ladder books render fully verbatim (zero elisions,
zero erasures) at HEAD.

Why it still ranks second: the coupling is (a) undocumented at either end —
`merge_observable`'s doc says "conservative meet", not "the W-C monotonicity argument rests on
⊤-vs-Value counting as disagreement"; the fixpoint's doc claims monotonicity "by construction"
without the merge lemma — and (b) unpinned — no test fails if someone makes the merge smarter.
And the pressure to make it smarter is REAL and adjacent (see fnd-sibling-requery-kills-cascade:
two AGREEING measurements of one cell currently kill the cascade, the most natural precision
complaint imaginable). The edit "treat ⊤ as identity in the merge" looks like a pure precision
win, passes every existing test I know of, and silently converts the fixpoint from sound to
wrong-yes-capable. Recommend: one registry bullet on each side of the coupling, plus a tripwire
case — the agreeing-sibling ladder pinned to stay verbatim, with a comment saying that if a
merge change makes this case start eliding, the settle loop must first gain final-view proof
re-derivation (drop-and-recompute until the ledger is a true fixpoint of the final view).

### fnd-sibling-requery-kills-cascade (SEV: precision/UX, no unsoundness; CONFIDENCE: demonstrated)

Any same-fact re-query downstream of a wall blocks the wall's own erasure even when every
measurement agrees: the invalid sibling's withheld-⊤ observable poisons the shared cell's meet
in round 1, the controlling guard never substitutes, the fixpoint never starts. Books that
defensively re-check a fact they already guarded on — idiomatic careful sh — lose the entire
W-C value below the wall. Priced coarseness at one site (`24L` §3) but nowhere stated as a
cascade-killer. Also a naming nit: the minted narrative + `SharedCellMeasurementsDisagree`
advisory spell this "disagreement" even when the two measured rcs agree and only
Value-vs-withheld differed (the diag prose's "or could not answer" covers it; the code/slug
does not).

### fnd-crlf-trips-book-parser (SEV: minor robustness/cross-platform; CONFIDENCE: demonstrated)

A CRLF-line-ended oracle file trips a spurious `error[syntax-malformed]: unterminated brace
group` from the book-side parser under `dorc lint`, while the byte-identical LF file lints
clean — and the predict LIFT accepts the CRLF file fine (the probe ships). Layer disagreement:
an author on Windows gets a phantom syntax error from lint on a file the licensing path happily
consumes. (Found as a fixture artifact; verified LF-vs-CRLF is the sole variable.)

### obs-gate-idiom-shift-led-lost (SEV: authoring UX; CONFIDENCE: verified by parse trace)

`shift || return 2` — a natural arity-gate spelling squarely inside the human's stated W-D
motivation ("can't be typing a three-line gate around every single command") — is out-of-dialect
by accident of parse order: keyword-`shift` consumes, the dangling `||` fails the next
statement, the whole body goes loud-⊤. Safe direction, but the diagnostic will not say "gate
with a shift head is unsupported"; W-A's cause-naming should cover it, and phase-2's gate table
could legitimately admit shift-led gates by modeling the shift on the fall-through.

### obs-redirect-target-cmdsub-standing-debt (pre-existing, disclosed; unchanged by r26)

The one `#[ignore]`d test (`observable_matrix.rs:645`, 16G HOLE#1) documents that a `$()` in a
redirect target / case pattern does not lower, so its Kill cannot poison — under-execution-
capable and known. Checked for W-C amplification: none — the unlowered Kill never existed in
the model, so downstream elisions already stood pre-fixpoint; the blast radius was maximal
before and after. Standing debt, correctly fenced as a named SPEC hole.

## Suspicions investigated and CLEARED (signal, not filler)

- **clr-ledger-outlives-justification** — the headline suspicion (grow-only ledger surviving a
  justification collapse via round-2 conflict reveal) is NOT reachable at HEAD; closed by the
  merge ⊤-paranoia. Downgraded to fnd-monotonicity-rests-on-merge-paranoia above.
- **clr-erase-fence-render-mismatch** — `controller_substitutes_away` (`plan/src/erase.rs:309`)
  vs the render truly agree: per-leaf it is literally the same predicate the Replace mint uses
  (`query_substitutes`, `plan/src/lib.rs:864` — verified `prove_query_replaceable` :586 adds
  nothing), same floors (in-loop, ⊤-successor), same heredoc-only refusal as `is_neutralised`'s
  Replace arm (:4690). The fold-dead-leaf-inside-a-controller case composes (a dead leaf's
  killer is provably inside the same controller subtree, and its leaves are fence-checked too).
  The one asymmetry — `leaf_has_blocking_output_redirect` absent from the fence — is compensated
  upstream: `output_redir_observables` (`analysis/src/cfg.rs:1941`) marks any non-devnull output
  redirect as consumed Stdout/Stderr, which blocks BOTH readers symmetrically. Experiment: a
  `>>/tmp/x`-redirected controller renders fully verbatim.
- **clr-redirect-suppression-replace-tier** — "Part B" (a Replace-tier redirect refusal) turns
  out unnecessary for the same reason: a non-devnull output redirect can't reach a Replace
  (consumption blocks); the devnull scalpel is the single seat and it is correct.
- **clr-forged-rc-gate-family** — W-D phase 2's gates are tight: literal N ≥ 2 only (0 forges a
  pass, 1 forges the complement — both refused, pinned by the N-ladder tests), command-left only
  under `||`, rc-forging heads and pipelines excluded, `&& return N` command-left refused. The
  or-true masking class degrades ⊤. Tracer-state mutation cannot enter gate-left through the
  parser (keyword dispatch + assignment-led refusal) — the surviving door is statement-level,
  which is fnd-state-builtins above, and quoted-`'shift'` (pathological, author-self-harm tier).
- **clr-rc1-controller-divergence** — the `&&`-direction cascade is faithful: rc-1 substitutes
  `false`, never a forged success (experiment: `false && :` + downstream rung fully elided via
  a purge erasure). Query Replace does not demand convergence, matching the fence.
- **clr-splice-ambiguity-proofs** — inlining's non-injective AstId map cannot mint a proof off
  the wrong call-site: `leaf_facts`' last-wins collision and the fold's AST-keyed deadness are
  both fenced by condition 4's `node_of_ast` ambiguity check (`Some(Some(_))` or refuse).
- **clr-fold-semantic-holes** — pipeline rc models POSIX-no-pipefail, but a book-side
  `set -o pipefail` is an unmodeled command ⇒ Opaque ⇒ poison wall ⇒ nothing downstream elides,
  so the modeling gap cannot license; `case` never rc-folds; errexit rides the CFG's
  failure-edge materialization and the fold's kill directions match dash's actual and-or
  gotchas. Statically-known controllers (assignments, funcdefs) cannot erase (no command leaf ⇒
  fence false) — records-grounded-only holds as written.
- **clr-verdict-keying-narrowness** — W-B's selection rule is implemented as specified:
  entity from the reached BIND (never mark text), one verdict mark per path or keys nothing
  (rc-arity), declines and `return 1` fall to the auto-cell, brace-alternation refused
  (`oracle/src/verdict.rs:348`). Family threaded exactly (`effect.rs:record_backing`), and
  verdict-minted selectors sit outside `build_dialect` (`oracle/src/lib.rs:393` mints from
  `KindIndex.effects` only) ⇒ they COLLIDE in sparing, the disclosed conservative fallback.
  `fence-no-disjoint` re-registration survived W-B (`main.rs:1278`, still per-provider). The
  ship discriminator reads the per-site lane set, never the kind (`main.rs:908-916`).
- **clr-driver-loop-wiring** — cap = site count (unreachable bound), cap-hit discards the WHOLE
  ledger and returns the origin classification with a narrative (no half-settled state); later
  rounds' throwaway `degrades`/`verdict_lane` args are round-invariant quantities; the final
  plan consumes the SETTLED round's classes/kills (`main.rs:1177-1181`) — the origin-kills clone
  at :866 feeds only the pre-fixpoint probe/derivation compile, and probe emission is frozen by
  design (v1 scope cut, `brg-emission-exclusion-is-v1-scoping`).
- **clr-vouchless-elide-weld** — every route I traced into a mutation-elision demands the
  `ByVouch<VerdictVouch>` by value (`prove_replaceable` establish arm, `AllEstablishesVouched`
  aggregates); the guard tier refuses without a vouch; a mis-keyed ESTABLISH record therefore
  cannot elide on its own (which is why fnd-state-builtins grades through the query lane).

## Composition questions I could not settle

- **opn-overshoot-shape-vs-reactive** — the settle loop's overshoot iteration (union of
  per-round proofs, no final re-derivation) is sound only under view-monotonicity. The `26H`
  §4¾ bridge law guards record-SET changes, but view-nonmonotonicity can arrive WITHOUT a
  record-set change (any future merge/firewall precision edit — fnd-monotonicity above — or any
  reactive-era partial view). Before the reactive round wires streaming waves in, the loop
  should either become a genuine re-derived fixpoint of the final view or carry a proof
  obligation that every view source is monotone. I could not settle whether the planned `26B`
  confluence machinery already implies this; the seam text does not say it.
- **opn-verdict-dialect-future-registration** — if verdict-minted selector tokens are ever
  registered into the sparing dialect (the registry names it "its own future dispatch"), the
  observe-widening and family-collision paths for verdict-lane facts need a fresh pass; today's
  safety is conservative-by-exclusion, and I did not attempt to verify what registration would
  require.
- **opn-builtin-head-enumeration** — I demonstrated three members of the state-builtin family
  (`set --`, `unset`, `eval`) and reasoned two more safe-by-degrade (`read`, `export`); I did
  not systematically enumerate every builtin the closed dialect's plain-command fallthrough
  admits (`exec`, `trap`, `.`, `cd`, `umask`, `ulimit`, …) against tracer-state and
  probe-environment effects. The fix shape (deny-list at the fallthrough) makes the
  enumeration moot; auditing each individually does not scale.
