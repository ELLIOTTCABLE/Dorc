# 279d — review working notes (deepseek-n)

Scratch notes for the critical review of the 270–278 design package. Transient; the final
report replaces this. Do not cite.

## Documents read (all)

- Root: README, DESIGN, IMPLEMENTATION, USER_STORY, KNOBS, TODO-ADDTL, AGENTS, spike/CLAUDE
- Package: 270, 271, 272, 273, 274, 275, 276, 277, 278
- Cited for context: LIVING_STATUS, 24J (header only / debt confirmation)

## Finding candidates (severity-sorted draft)

### F1 — CRITICAL: Block-context builds on known-debt probe composition (task-14 gap)

- 273 §6 probe-form composition is explicitly DRAFTED and gated on task 14 ("re-derive the
  structural-vouch hard law in a fresh session")
- 24J header confirms the current mechanism (raw book bytes, engine-side shape-matching)
  is "CONFIRMED STANDING-LAW DEBT" against round-20 structural-vouch ruling
- 270 §2 block-context stages (wrapper-sudo, payload-v1, read-value-slice) all depend
  on correct probe composition — predict bodies shipping as stand-ins
- 274 §5: "Probe-form composition remains task-14-gated: nothing here ships oracle bodies
  as stand-ins yet; the synthesis only settles what the reentry point IS."
- 271 task 14 DEFERRED past Fable window → Opus-conductable
- Risk: building block-context against the debt-confirmed mechanism bakes structural
  assumptions that task 14's resolution may invalidate. The wrapper probe-outside
  licensing, carrier reentry shipping, and capture-lane delegation all depend on correct
  composition semantics.
- Mitigation in place: 271 says "Gates block-context lanes only; block-rebuild never
  waits on it." But block-context IS the consumer. If block-context proceeds before
  task 14, it builds on sand.
- Counter-argument: W1 (wrapper-peel) regression is "wrapper-free corpus goldens
  byte-stable" — it may not need composition at all. And the bare lane (274 §5)
  ships real bytes, not composed bodies. So the dependency may be narrower than
  "all of block-context."
- Confidence: MEDIUM — the dependency exists but its blast radius depends on how
  much of block-context actually exercises composition vs bare-lane shipping.
- Cites: 270:130-164, 273:217-244, 274:160-170, 24J:12-13, 271:87-92

### F2 — MAJOR: Formal spine (generator registry + comparison relation) is conductor-proposed, not ratified

- 277 §2 defines the formal spine: one comparison relation, generator registry mapping
  authored surfaces to verdict classes, consumer map
- 277 §9 status: "the generator registry stays conductor-proposed, on the table for the
  adversarial pass"
- This is the load-bearing abstraction tying disturbs, invariance, lends, keying, and
  dialect comparison into ONE chokepoint
- The entity-algebra-rebuild brief (§7b) is supposed to implement against this spec
- If the adversarial pass finds issues with the registry, the rebuild's chokepoint
  implementation may need to change — and the chokepoint is what everything feeds
- Mitigation: the individual components (coordinate shape, selector dialect, invariance
  line) ARE typed/ratified. The registry is composition of ratified parts. Changes
  would be in how they compose, not what they are.
- Confidence: MEDIUM — the risk is real but the individual pieces are solid.

### F3 — MAJOR: The `only` contract on `kind__state_stored_only_in()` has wide blast-radius

- 272 §2: "this member is complete-by-contract — the author must survey the kind's
  stores totalistically before authoring at all"
- 272 §8 "The knife": an omitted axis-dependent store causes wrong invariance → wrong
  transport → under-execution across context boundaries
- Unlike other at-most claims (which affect only the author's own tool), a wrong
  store-member claim affects every tool that interacts with the kind across context
  boundaries. The blast radius is wider.
- The design prices this the same as other at-most claims ("Attributed to the member's
  line; same tier as the rest of the at-most family" — 272 §8)
- The razor-conversion (271:rul-invariance-speech-act) moved transport from derived
  (silence-based, unattributable) to typed (explicit line, attributed) — this is correct
  but doesn't reduce the fire frequency (priced honestly in 271)
- The differential harness can falsify wrong claims after the fact but can't verify
  the survey was done upfront
- Mitigation: the contract is explicit, the survey is a human judgment (consistent with
  vouch-tier philosophy), and wrongness is attributed. The design doesn't hide this.
- This is more a "sharp edge" than a flaw — the design acknowledges it. But the gap
  between "same tier as other at-most claims" and "wider blast radius" is worth noting.
- Confidence: HIGH (the anatomy is accurate; the severity judgment is the question)

### F4 — MINOR: "invariant" terminology collision between authored and derived meanings

- "invariant" in 277 §4e (the `invariant:<axis>` token) is a speech-act on a typed line
- "invariant" in 272 §3 (derivation outcome for substrate-borne axes) is engine-derived
  from the carried-by table
- Both produce transport/probe-outside licenses, but with different provenance
  (vouch-tier vs structural/engine-warranted)
- The status table (272 §12) distinguishes them, and 271:rul-invariance-speech-act is
  explicit about the difference. But the naming collision could confuse implementors.
- Confidence: LOW (the design documents distinguish them; it's a naming nit)

### F5 — MINOR: 274 §1 "incorrectness-inexpressible" claim for invited-rooms typing

- 274 §1: "Enforcement tier = TYPESYSTEM, not test-pin — incorrectness-inexpressible
  type-differentiation between invited-room analysis (may mint licenses) and hint-only
  rooms (may not)"
- This is achievable with Rust newtypes + module boundaries, but "incorrectness-
  inexpressible" is a strong claim. The type system must prevent hint-derived facts
  from reaching license-consuming code paths.
- If the typing gap exists, hint-lane data could feed survival-by-accident.
- Mitigation: Rust can express this with wrapper types and visibility control. The
  claim is strong but plausible.
- Confidence: LOW — this is an implementation concern, not a design-logic flaw

### F6 — MINOR: 272 §3 r2 (emission-set non-interference) was re-roled but the old
description still dominates the document

- 271:rul-invariance-speech-act re-roled r2 from license-generator to contradiction-
  checker. 272 §3 was AMENDED (note at line 139-143) but the main text describing r2
  still reads as if it generates transport licenses.
- An implementor reading top-to-bottom might implement the pre-amendment semantics
  before reaching the amendment note.
- Confidence: LOW — the amendment is prominent and the status table records it.

## Investigated and dropped

### I1 — "kappa architecture" / two-phase design
- The block-settle → block-rebuild split is clean. The charter (270) makes clear what
  gates what. No problem here.

### I2 — Whether any ruling contradicts root DESIGN.md
- Checked: DESIGN's "never generates probes, lifts them" aligns with authored-body
  shipping. DESIGN's "best effort" aligns with vouch-tier attribution. IMPLEMENTATION's
  correctness band is respected by the survival flag's explicit trust buy-in.
  No contradiction found.

### I3 — Whether the double opt-in (admin flag + author claim) for survival correctly
isolates risk
- 271:rul-flag-is-razor-residue is clear: the flag gates the open-world at-most residue
  only. Typed lines (invariance, lend entries, disturb claims) ride the vouch economy
  un-flagged. The flag gates the survival *consumer* of the comparison relation, not
  the claim types. This is coherent and well-specified.
- Dropped: no problem.

### I4 — Whether 278 (the reference) contradicts or misrepresents any ruling
- Spot-checked: `cmd__disturbs()` rename, `only`-rule, rc partition, env-claim ladder,
  stability ledger all match their sources. The reference correctly marks itself DRAFT.
- Dropped: no contradiction found.

### I5 — Whether the charter's "adj-" items that were struck/deferred during sittings
indicate planning failure
- Tasks 11, 13, 14, 15 were struck/deferred during the Fable window. The charter (270)
  predates the sittings and listed the full agenda — the sitting's job was to triage.
  The triage is recorded in 271's task map with reasons. This is normal design-process
  refinement, not a flaw.
- Dropped: no problem.

### I6 — Whether the never-settled-backed regime (275 §3) creates a classification gap
- The human said "don't even mention" for clock-backed values. The regime is
  HARD-DEFERRED. The classification boundary is clear enough for v1 (register vs
  world-cell vs never-settled). Any ambiguity would only surface when clock-backed
  values are unparked, which is explicitly not happening.
- Dropped: not a v1 problem.

### I7 — Whether 276's book-tolerance openness contradicts the off-ramp promise
- 276 explicitly leaves book-acceptance as an open question while welding oracle
  dialect. DESIGN.md's off-ramp promise applies to the stripped artifact, which is
  always floor-legal sh. The open question is about analysis quality for non-POSIX
  books, not about whether they can run. This is honest about limitations.
- Dropped: consistent with DESIGN, not a contradiction.

### I8 — Whether the "faultless" in `--risk-faultless-skips` is misleading
- The human acknowledged the potential confusion and demanded a help-text disclaimer.
  This is a UX naming concern, not a design-logic flaw. Per the weighting rules, this
  is very low value.
- Dropped: naming nit, already addressed in the ruling.
