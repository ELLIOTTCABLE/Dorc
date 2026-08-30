# 30U — Finished definitions: `disturbance_reaches` and the `disturbs nothing-else` record

> Tier: LLM-authored design-of-record candidate (Fable; the `r30-design-duck-file-paths-and-redirects`
> sittings, 2026-08-29; human-steered throughout). Grades: **[TYPED]** the human typed it ·
> **[ACKED]** substance confirmed in dialogue · **[PROPOSED]** conductor synthesis. Ahistorical:
> this document describes the product as it should exist — §10 (implementation sketch) is the
> one exception and is explicitly pending a successor's rewrite. Subordinate to the root docs,
> `spike/CLAUDE.md`, `KNOBS.md`. Sibling of `plans/30T` (authored file semantics). Remit: the
> `disturbance_reaches` role member and its `disturbs nothing-else` record — the survival
> tier's wider law stays where it lives (`KNOBS:kSURVIVAL` · USER_STORY's bought-unsoundness
> receipt); §7 names the components this design constrains.

## §1 the-design-in-one-screen

- **`rul-cross-kind-sparing-needs-a-finished-definition`** [ACKED 2026-08-29] — an elision
  survives a running wall across *vocabularies* only when the wall's claimed kind carries a
  finished definition for the claim's shape. A footprint cell of kind A may be found disjoint
  from a backing cell of any other kind only through A's finished definition; absent one, the
  cross-kind comparison answers *unrelated* and the fact guards. Within-kind comparison, the
  guard tier, the honest-wall default mode, and the admin's flag are all untouched. The
  survival tier's double consent — the admin's typed flag, a named author's claim — now holds
  in the cross-kind cell too, where previously sparing rested on the comparison machinery's
  structural inability to find overlap (nobody's speech at all).
- **`rul-reaches-is-arm-incremental`** — the kind-owner member is `disturbance_reaches`
  (née `disturbance_reaches_only`): invoked once per footprint cell of its kind with that
  cell's entity as `$1`, it emits the cells the entity's disturbance *entails*. Emissions
  WIDEN footprints — they add collisions, the safe direction — and license nothing by
  themselves. Any author may write one from partial knowledge; incomplete is legal and
  useful (rung 1).
- **`rul-the-nothing-else-record`** [spelling TYPED 2026-08-29] — the finished-definition act
  is one report-lane record in tail position on a reached path:
  `printf 'disturbs nothing-else\n' >>"${DREP_V1:-/dev/null}"`. Its *content* is the
  kind-owner's completeness claim ("for shapes reaching this path, disturbing this kind
  entails at most the emitted cells — and nothing else, in any vocabulary"); its *arrival* is
  the execution witness (everything above it ran). One line, both jobs (rung 2).
- **`rul-witness-where-licensure-rides-runtime`** [TYPED 2026-08-29] — the general law the
  record instantiates: a runtime emission carries license-authority only if witnessed
  complete, and the witness is this same record; bodies whose emissions license nothing need
  no witness and get no second sentinel. There is no `others-unknown` form.
- **`rul-definitions-not-surveys`** [TYPED 2026-08-29, the framing] — a kind is minted
  speech: a shared word, and an abstraction over what a family of tools does. The reaches
  body is part of the word's *definition* — what disturbing this kind *entails, by meaning* —
  never a measurement of the world. The engine consumes only the lattice of words and
  entailments; no referent is interpreted anywhere.

## §2 why — the epistemics, briefly

An at-most claim ("I touch at most `Package:nginx`") carries an *and-nothing-else* tail that
is honest at the author's granularity — in the author's mind, touching a package includes
its files — and false at cell granularity, where the machinery compares. Before this design,
the machinery consumed the strong cell-level reading nobody meant: an unsurveyed kind's
claims spared *more* than a surveyed one's, since a definition could only ever add
collisions. Ignorance outperformed diligence, and cross-kind sparing was the survival tier's
one cell resting on no author's speech (its "condition 5" had a free rider).

The finished definition unrolls the entailment once, kind-side, so the claim's
nothing-else means the same thing to its author and to the comparison. The exclusion of
kinds the definer has never heard of costs nothing and requires no knowledge of them: a
finished positive list plus "nothing else" excludes the open world by its logical shape —
one signature about your own suitcase, never a compatibility matrix. Cost is one act per
kind, from self-knowledge; value is linear in finished kinds and front-loaded in practice,
because the walls that dominate drifted mornings belong to a handful of famous kinds whose
definitions the stdlib finishes once.

The gate is unary and attaches to the *claimed* (footprint) side only. The asymmetry is
epistemic, not arbitrary: a footprint describes the future writes of a binary nobody holds —
an open-world completeness claim, unverifiable in principle, for which deliberate speech is
the only sound instrument. A backing *constitutes* its fact (the belief is, definitionally,
"this check read these cells and answered so") and cannot under-cover it; its residual
dangers — transcription infidelity between marks and a body's actual reads, narrow-sense
vouches, address aliasing — are verifiable held-text properties, the vouch's priced
judgment, and the identity tier's business respectively, each with its own instrument.
Speech where proof is impossible; proof where the text is in hand.

What this does NOT change: the size of the knife. A wrong finished definition under-executes
exactly as a wrong at-most claim always could (the receipt's condition-6 family, permanent,
frame-problem-shaped). What changes is that every sparing now rests on a specific sentence
with a specific author, and the incentive gradient rewards the diligent definer instead of
the silent one.

## §3 the-member — `disturbance_reaches`

- **Invocation contract** — KIND-species, name-derived membership as ever. The engine
  invokes it once per footprint cell of its kind, whoever emitted the cell (a tool's
  `disturbs`, the filesystem binder of `30T`, a store-declaration derivation); `$1` is the
  cell's entity. Emitted lines name entailed cells, kind riding the trailing mark
  (`: disturbs sm.dorc.File`), raw entities on stdout — the stage-7 shape unchanged.
- **Static and host arms, per the standing split** — an arm whose emission is pure
  expansion over `$1` is traced at plan time on the controller and ships nothing; an arm
  that must ask the host (`dpkg -L -- "$1"`) ships strip-only into the probe lane, runs
  read-only, and its stdout becomes the entailed cells, under the standing execution
  framing (the engine's end-records; a dying body refuses the whole footprint — `28P`).
- **Rung 1 — informative** — a body (or a path) with no `nothing-else` record: its
  emissions add collisions and close holes; they never license. Truncation of a rung-1
  body loses defense-in-depth, never correctness, so no witness is required — the
  no-licensure case is self-checking [TYPED 2026-08-29]. Detected body death still reports
  loudly; retained partial emissions keep colliding (integrity failure withholds trust,
  never caution).
- **Rung 2 — finished** — the record on a reached path finishes the definition for the
  shapes that reach it. Per-path scope means coverage spreads shape-by-shape along the
  ordinary argparse gradient: finish the entity-shapes you have genuinely thought through,
  leave the rest at rung 1, one `;;` at a time.
- **The name** — `only` is gone from the member because the member is no longer
  complete-by-contract (`271:rul-at-most-family-names` harmonizes: absence of `only` =
  arm-incremental, exactly what the member now is). The totality moved into the record,
  where it is per-path, granular, and attributable to a line.

The canonical body:

```sh
sm_dorc_Package__disturbance_reaches() {           # invoked per Package footprint cell;
   case "$1" in                                    #   $1 = that cell's entity, e.g. 'nginx'
   *:*) ;;                                         # multi-arch spellings: rung 1 at most
   *)   printf '%s\n' "$1" : disturbs sm.dorc.Service   # static arm — traced, ships nothing
        dpkg -L -- "$1"    : disturbs sm.dorc.File      # host arm — ships, read-only, probe
        printf 'disturbs nothing-else\n' >>"${DREP_V1:-/dev/null}" ;;
   esac                                            # ↑ rung 2: the finished-definition act
}                                                  #   AND the completion witness, one record
```

## §4 the-record — `disturbs nothing-else`

- **Lex** [TYPED 2026-08-29] — the record lane's fixed `verb arg` shape, reusing the
  `disturbs` verb; `nothing-else` is the standalone special-case argument, unambiguous by
  the keystone disambiguator (kinds carry ≥2 periods; `nothing-else` carries none). The
  enumeration reads as a closed sentence: *disturbs File, disturbs Service, disturbs
  nothing-else.*
- **Record lane only, never a trailing mark** [TYPED 2026-08-29] — a completeness claim
  that can reach runtime must be spelled as a runtime emission, because only arrival can
  witness execution; and one spelling serves both worlds, since the tracer already reads
  authored report-lane speech at completion points statically (`30D`'s own
  expected-confirmation mechanics). A trailing-mark `: disturbs nothing-else` is
  unrecognized — there is no second spelling.
- **Tail position, exactly-one** — the record sits after everything it vouches for; sh's
  sequential execution makes its arrival imply every emission above it ran. A path with
  zero records is rung 1; an execution producing more than one (a loop-placed record, two
  reached branches) refuses the whole footprint, per the standing at-most rules. Pushing
  the record into branches is the author's burden, as everywhere in the family.
- **Fail-safe by spelling** — errexit trips, a dying host arm, even the `printf` failing on
  a full disk all yield the same outcome: no record, no finished definition for that
  invocation, footprint walls total. No engine special case exists or is needed.
- **Off-Dorc** — `${DREP_V1:-/dev/null}` makes the line inert everywhere else; the off-ramp
  reads the body as an ordinary instrumented script.
- **As a generator** — a finished definition is a generator of cross-kind
  *provably-disjoint* verdicts (the claim's widened set excludes the backing), joining the
  generator registry like every authored surface; the record never re-enters the relation
  as evidence for anything else.

## §5 one-record-across-the-at-most-family

The same record is the *mandatory* completion witness in dynamic `cmd__disturbs` bodies —
there, emissions ARE at-most claims (license-bearing), so exit-0 truncation would
under-claim and wrongly spare; the record in tail position is the ruled at-most completion
sentinel (`ANALYZER-NEEDS:an-atmost-completion-signal`), which this design supplies with
its concrete verb. Static disturbs arms are trace-complete and need none. The whole family
under one law: **witness exactly where licensure rides runtime emission** — mandatory for
dynamic `disturbs`, the opt-in rung-2 act for `reaches`, absent by design for collide-only
surfaces (rung-1 reaches; store declarations).

## §6 the-authorship-contract, summarized

- **Tool authors** — unchanged: `disturbs` names what your tool touches, in vocabularies
  you chose to adopt; dynamic bodies end in the record. Adopting a kind is your attributable
  act: if the word's definition under-describes your tool, decline the kind or emit further
  claims — word-misapplied is your line, word-wrong-for-everyone is the definer's.
- **Kind owners** — you may write `disturbance_reaches` from partial knowledge, any time,
  and only ever add safety. You write `disturbs nothing-else` on a path when — and only
  when — you have genuinely finished that shape's definition; it is the one dangerous
  sentence in your file, the why-chain will cite it by line, and every book on every host
  inherits both its value and its repair.
- **Admins** — nothing new to write. The flag's meaning sharpens: past it you are trusting
  named authors' at-most claims *and finished definitions*. Your line-granular remedy
  remains your own hand-written guard, which no claim anywhere can override.

## §7 constraints-on-other-components (brief; each is its own design's business)

- **The comparison** — the answer set gains *unrelated* (no generator ever spoke),
  behaviorally the safe bottom exactly like unknown, distinctly labeled; cross-kind pairs
  answer unrelated unless the claimed kind's finished definition licenses disjointness.
- **The trace/settle seats** — the reaches trace records finished-status per
  (kind, reached shape); settlement gates each cross-kind footprint×backing pair on the
  footprint cell's origin kind. Universal-meet semantics unchanged: one unfinished-origin
  pair collides the whole spare. Widened cells are part of the finished statement, not
  separately gated.
- **Store declarations** (`kind__state_stored_only_in`) — the dependent-kind complement: a
  declared store adds collisions against footprint cells landing inside it. Collide-only,
  monotone-safe, incomplete-legal; it is what makes a substrate-like kind's own finished
  definition honest (the File kind can finish "a write entails the file's cells" because
  other kinds' residency in files is those kinds' speech). Wiring is that design's item.
- **`30T`'s filesystem binder** — consumes this gate uniformly: binder claims are ordinary
  at-most claims (30T §7), so cross-kind sparing below a redirect wall requires the File
  kind's finished definition, like every kind below its own. No special case.
- **Aid surfaces** — *unrelated* renders distinctly from *provably-disjoint* everywhere
  (the weakest link never wears the strongest label); plan-time hints count recoverable
  sites ("N verifications would lift with a finished definition for K"); the missing-
  definition epilogue names the kind, the member, and its owner; the wrong-definition chain
  cites the record's file:line; the hand-guard remedy is taught in the same breath.
- **Consent language** — the flag's description carries the author-side condition; the
  survival tier can finally state "silence licenses nothing" without an asterisk.
  (USER_STORY's receipt and stage-5/7 renders are human-authored; suggested edits ride the
  ordinary queue.)

## §8 invariants-adopted

1. **`inv-30U-no-sparing-on-nobodys-speech`** — no cross-vocabulary sparing ever rests on
   the comparison's structural inability to find overlap; only a finished definition
   licenses it.
2. **`inv-30U-unary-never-pairwise`** — the gate is per-kind, self-knowledge-only; no
   mechanism may ever require kind-pair enumeration or cross-kind acknowledgment.
3. **`inv-30U-witness-iff-licensure`** — runtime emissions carry license-authority only
   when witnessed by the record; witness-free bodies are legal exactly where emissions
   license nothing; no second sentinel form exists.
4. **`inv-30U-one-spelling`** — the record is report-lane speech in both static and shipped
   worlds; no trailing-mark twin.
5. **`inv-30U-collide-on-integrity-failure-keeps-collisions`** — detected body death voids
   authority, retains emitted collisions, and reports; integrity failure never reduces
   caution.
6. **`inv-30U-definitions-are-speech`** — reaches bodies define words, never measure
   worlds; the engine consumes entailment structure only.

## §9 deliberate-non-capture

- A wrong finished definition under-executes (condition-6, permanent, flag-priced,
  attributed to the record's line). Not closable; now fully consented.
- Un-finished kinds' walls guard cross-kind on drifted days — value withheld, not lost;
  the hint machinery prices the upgrade continuously.
- Whether dynamic rung-1 bodies should someday be *required* to end in the record (making
  truncation always-detectable, at one mandatory line's cost) is left to authored reality
  and lint pressure; opt-in is the ruled floor [ACKED 2026-08-29].

## §10 implementation-sketch — PENDING SUCCESSOR REWRITE

[Non-ahistorical; no schedule exists; a successor re-derives this against the live tree
before any dispatch. Unit shapes only:]

- rename respell (`disturbance_reaches_only` → `disturbance_reaches`), corpus-wide, one
  commit, no-compat.
- comparison answer `unrelated` + the settle gate + finished-status tracing (one lane;
  amends the ternary consumer-map steering entry).
- record recognition: `nothing-else` argument in the report-lane grammar + the tracer's
  static-recognition arm + tail-position/exactly-one enforcement + the disturbs-body
  mandatory-witness check.
- store-declaration collide consumer (independent, small).
- aid rows: unrelated label, epilogue arms, recoverable-count hints; prose `[unwritten:]`.
- reds/pins: unfinished-guards, finished-spares, rung-1-collides, the incentive-inversion
  regression pin (a definition must never reduce sparing relative to its own absence).
- steering/register edits per the staleness scout's report (`30U`-adjacent; see the scout's
  findings when they land).
