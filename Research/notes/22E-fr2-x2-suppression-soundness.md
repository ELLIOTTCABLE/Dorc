# 22E — fr-2 + x-2: the why-lens dedup's over-suppression soundness

> Round-22 conductor, 2026-06-14. Human-directed synthesis of (a) the fr-2
> source-grade and (b) the x-2 over-suppression crosscheck on the harvested
> why-lens dedup. The grader wrote no notes (correct — a bare source-grade); this
> note is the COMBINATION (the human's "stamp notes for the combination"). Written
> just before a planned context-clear, so it stands alone for the post-clear
> session. AI-authored; +SURE/~SUSPECT marks. The fix is RECORDED here, NOT yet
> built (follow-up task — a fresh-context session implements it).

## §1 fr-2: what it grounds, and the transfer caveat (dc-7)

fr-2 = "Sound Non-Statistical Clustering of Static Analysis Alarms", Lee/Lee/Yi,
VMCAI'12. Graded **B** (`Research/sources/B-lee-lee-yi-sound-nonstatistical-alarm-
clustering-vmcai-2012.{pdf,txt}`, gitignored on-disk; clean text-layer, no OCR).

The criterion: collapsing alarm A under alarm B is SOUND iff a genuine dependence
`B-false ⇒ A-false` holds (Def 1, Lemma 1). The trivially-sound special case is
**syntactic clustering** (§4.1, Example 4): when the transfer between the two
alarm points is identity-up-to-the-alarm-variable — A is ⊤ *solely because* B's ⊤
flowed into it — dependence is structural, needs no abstraction, always precise,
NO refutation. OVER-SUPPRESSION = collapsing two alarms that are merely
CORRELATED but INDEPENDENT (no `B-false⇒A-false`), hiding the second.

THE TRANSFER CAVEAT (dc-7, +SURE on the gap): the paper PROVES dependence for the
hard (non-syntactic) cases via **refinement-by-refutation** — assume an alarm
false (slice out its error-states), RE-RUN the analyzer to a new fixpoint, and if
another alarm dies too, dependence is proven. That re-run-under-a-counterfactual
is exactly the backward/phase-fusing machinery Dorc WELDED OUT (ru-13:
forward-built, backward-QUERIED only; no counterfactual re-runs). So the paper
grounds the POSTURE (sound clustering is achievable) but its MECHANISM is off the
table for Dorc. Dorc's dedup lives entirely in the paper's trivially-sound
SYNTACTIC corner (collapse a ⊤-origin's pure poison-descendants — the cause is
the SAME ⊤ propagated forward, read straight off the dataflow). dc-7 cap lifts
`~SUSPECT` → qualified `+SURE`: sound clustering transfers to Dorc ONLY in the
pure-propagation/syntactic regime; anything correlated-but-independent would need
the ruled-out refutation, so the design discipline is STAY IN PURE PROPAGATION.

## §2 x-2 verdict: the dedup is sound in straight-line, OVER-SUPPRESSES in two cells

Crosscheck pair (Opus, adversarial-crosscheck skill, read-only, grounded in fr-2).
Reconciled (conductor): CONVERGENT on the sound parts; DIVERGENT on the headline
defect (the adversarial pass found what the neutral pass's reasoning missed).

CONVERGENT-SOUND:
- The cross-consumer dedup (22D stage-4: "one ⊤-origin explained once across N
  poison-descendants") stays inside the paper's sound syntactic regime. Two
  disclosures share a cause `ProvId` iff they hash-cons to the same
  `(TopCause, span)` = the same ⊤-origin. In straight-line code every command AST
  node is lowered once ⇒ unique span ⇒ unique cause ⇒ the dedup never wrongly
  merges. (Neutral + adversarial agree. Also: the `Reach::Top` poison-DESCENDANTS
  emit NO `CmdsubOperandTop` — they classify MustRun with a clean argv — so the
  "N poisoned consumers" never even enter the dedup set; the code comment
  overstates. The dedup only ever sees origin-SITE disclosures.)
- Determinism: SOUND. `shown_causes` is a `Vec` walked in CFG-node order;
  `ProvId` is `!Ord` (no hash-set leak); ids append-order. First-seen stable.
- The member-family `site:None` suppression (f-3b) is SOUND (true re-disclosure
  dedup; the ⊤ surfaces once at the single-cell fallback).

THE TWO REAL OVER-SUPPRESSIONS (both DISCLOSURE-incompleteness only — `⊤ ⇒ Opaque
⇒ MustRun ⇒ the command RUNS`, kFAIL-perform; stderr/exempt-plane; NO mis-elision,
NO weld/artifact/gate touched — low-severity, but real vs the fr-2 criterion):

- **x2-fd1 (the headline; adversarial-found, neutral MISSED; the divergence).**
  Function INLINING breaks the neutral's "distinct ⊤s ⇒ distinct spans" premise.
  `apt_install() { apt-get install -y "$1"…; }` called twice with DISTINCT dynamic
  args `"$(curl …a…)"` / `"$(curl …b…)"`: both calls inline; each `$1` binds a ⊤;
  each spliced `install -y "$1"` emits `CmdsubOperandTop` — but the cause is keyed
  on the BODY command's AST span, and inlining gives both spliced copies the SAME
  body AstId (`inv-leaf-seam`), so both hash-cons to ONE cause `ProvId` ⇒ the
  dedup collapses two GENUINELY INDEPENDENT forced-runs, suppressing the second
  `why:`. No `B-false⇒A-false` (fixing `…a…` leaves `…b…` forcing its run) ⇒ NOT
  the sound syntactic case ⇒ the paper's forbidden correlated-collapse. Reachable:
  ONE literal-swap from the passing e2e case `inline21-wrapper-converged-elides`,
  and UNPINNED (the why: line has no e2e assertion — cf. #16). The exclusion-check
  miss: the dedup was validated only in the straight-line cell (span-identity ⟺
  cause-identity); never re-tested under inlining where that equivalence breaks
  (AGENTS "verify a claim in other cells").
- **x2-fd2 (both passes; upstream of the dedup; a documented scope-cut).**
  `command_effect` returns `Opaque` on the FIRST ⊤ operand and never inspects
  later operands. `apt-get install "$(a)" "$(b)"` (two independent ⊤s) discloses
  only operand 1; "fix operand 1" leaves the command running on `$(b)`. This is a
  disclosure-completeness gap in `command_effect`, present even if the dedup were
  deleted. 22D §1 stage-1 documented operand-level causes as "aspirational, not
  required" — so it is a known cut, but it lands as real over-suppression vs fr-2.

## §3 The fix (RECORDED, not built — follow-up)

- x2-fd1: key the dedup on `(cause_ProvId, site.leaf)` (or `(cause, position)`),
  NOT `cause` alone — collapses the genuinely-same-fix straight-line/literal-arg
  case, keeps two distinct dynamic operands / two inlined call sites SEPARATELY
  disclosed. Small change in the `cmdsub_cause`/`shown_causes` dedup
  (cli/src/main.rs). +SURE on the shape; confirm exact line-targeting at build.
- x2-fd2: either disclose ALL ⊤ operands of a command (not just the first), or
  key the cause on the OPERAND span rather than the whole-command span (so two
  operands get distinct causes). Larger (touches `command_effect`'s first-⊤
  early-return) — weigh against the documented "operand-level aspirational" cut.
- Pin both with the inlining + two-independent-operand e2e/unit cases (folds into
  #16, the why-lens e2e-pin gap).
- DISPOSITION owed to the human: fix now vs accept-as-documented-scope-cut. Both
  are disclosure-only (no correctness/safety risk), so deferrable; but x2-fd1 is a
  reachable, deterministic wrong-suppression that contradicts the "never hide an
  independent cause" claim — recommend fixing (the dedup-key change is cheap).

## §4 Method note

x-2 grounded in fr-2 did exactly what the formal grounding sharpens: it
distinguished Dorc's sound syntactic dedup from the unsound correlated-collapse,
and the paper's "you'd need a refutation-proof" is precisely why x2-fd1 can't be
hand-waved (Dorc has no such proof and ru-13 forbids building one). The pair's
DIVERGENCE (neutral cleared the dedup; adversarial found the inlining break) is
the value: a single same-model pass — or the build's own author — accepted
"distinct ⊤s ⇒ distinct spans" without testing the inlining cell. Post-Fable,
this is the third time this round adversarial coverage caught a real thing the
straight read missed.
