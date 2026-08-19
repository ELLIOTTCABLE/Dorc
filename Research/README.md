# Dorc Research/ — the planning-ocean's map

All stored *sources* herein are human-authored, graded, and reproduced locally; all
non-resource content is AI-generated planning material — living notes, not verified,
authoritative, nor necessarily coherent. Human-*authored* content lives at the repo root;
the most significant findings are reproduced by-hand there, in the human's words, and the
root docs always outrank anything in here.

**This file is the MAP** — what each round settled, and the few documents that carry it.
Project *status* (the live arc, queue, branches, fences) lives in **`LIVING_STATUS.md`**:
start there if you're conducting; start here if you're hunting for where a question was
answered. Keep this file a map: refresh the topic-index as topics move, append rounds to the
round-map, and keep chronology/news OUT (that's LIVING_STATUS's job).

How to trust what you find: stamped `plans/` are the durable syntheses and are lightly
kept-current (later corrections get annotated in place); `notes/` are historical thoughts
verbatim, with known inaccuracies. On any topic, the latest round wins — the soundness story
in particular was re-cut repeatedly (bias-inversion `051` → perf demotion `076` →
trace-don't-derive `077` → MUST/MAY contracts `099` → the ternary verdict `233`/`239`);
never cite an early cut of it without the late ones.

Conventions: `{notes,plans}/YYx-slug.md`, where YY is the round and "x" ascends within it —
read the highest "x" first when digging. `notes/000-source-manifest.md` grades every source
and carries the license-contamination map.

## Topic index — "where was X answered?"

- **What is it, and where's the hard part?** — `plans/021` (empty dir → CFG/effect engine) +
  `plans/041` (language / parser / orchestration decisions).
- **Can sh be analyzed soundly enough?** — `plans/055` (analysis architecture).
- **Who needs it, and what do we build?** — `plans/064` (per-feature integrate-vs-delineate
  matrix; userbase evidence in `notes/060`–`062`).
- **Fast enough?** — `plans/076` (performance architecture + the "decide-now,
  retrofit-hostile" list).
- **What must the core never optimize away?** — `plans/077` (the wrappable-leaf hook surface
  + seccomp network backstop — *a live constraint, not history*).
- **How do we build it to fail fast?** — `plans/088` (falsification-first — *advisory, not a
  phased plan*).
- **Tracking shared state across hosts** — `plans/099` (relational contracts, MUST-vs-MAY,
  the IFDS decidable floor) + `plans/09A` (real-world specimen grounding).
- **Security / threat model** — `plans/102` (STRIDE, the soundness-boundary doctrine) +
  `plans/101` (the map; `kAGENTLESS` weld).
- **How do errors / provenance / "why" flow?** — `plans/111` (one PROV-shaped
  derivation-DAG; `(result × diagnostics)` never-throw) + `plans/22A` (the round-22
  deepening) + `notes/22D` (the `why`-lens contract).
- **How do we test a network appliance without a network?** — `plans/128` (DST; the one
  all-nondeterminism seam is the controller↔host transport).
- **Does it run on Windows / odd targets?** — `plans/139` (`kLANG` weld; sh-precondition
  tier-A/B targets; CRLF policy).
- **How does the controller talk to hosts?** — `plans/142` (executorless-OOB transport;
  read its Resolution — it supersedes the doc's earlier "My read" lean); the adjudicated
  ssh-transport spec, consumable single-host, is `plans/260` §5 (+`26A`); the round-26
  live-execution seed (executor + acceptance loop + salvage map) is **`notes/26D`**.
- **Where do the plans break / what wastes effort?** — `notes/151` (the adversarial
  convergence: the named-kind oracle contract was the unspelled hinge — since answered by
  rounds 17/23/24).
- **How do oracles name & reconcile kinds (symbol-grounding / 'types')?** — `plans/17N`
  (K1+K2 reunion: spell · analyze · reconcile), crosschecked in `notes/17O`.
- **The verdict vocabulary {elide, guard, run} & the oracle contract** — `plans/233` (HUMAN
  crisis log — **honor its end-annotation**) → `plans/239` (signed closure; the two-halves
  doctrine) → `notes/23O` (settled law + history). The elide-ceiling: `notes/238`.
- **The language dialect: names, marker, kinds, the typeless floor** — **`notes/24M`
  (binding human rulings, 2026-07-07)** over `plans/24L` (mechanism spec) over `notes/24Kc`
  (the crosscheck that forced it); the executable floor + the pipefail lanes `notes/276`
  (the kWHICHSH two-binary weld; the `unsafe` hatch) · the mark grammar
  **`plans/281`** (THE spec — supersedes `notes/277` §4's worked minimum; takes over
  the grammar `278` §6 deferred) · the one-page reference `notes/278` (DRAFT —
  assembles, never rules).
- **What's actually BUILT, and its residue** — `LIVING_STATUS.md` + the arc ledgers
  (`notes/28S` · `28P` · `28L`/`28N` · `28F` · `289` · `287` · `28A` · `27U` · `27D`; the
  r24-era per-landing ledger is `notes/24C`); spike-1 history in `plans/16P`/`16Q`.
- **The context/kernel unification (frames · entry-closure · context-availability)** —
  **`plans/28Q`** (THE live kernel plan; §9 is its pin ledger) + `notes/28R` (the six-lane
  review round, folded into the plan 2026-08-13).
- **Oracle loading & name-resolution (custody, committee speech, one-name-two-definitions)** —
  `plans/28K` + `plans/28M` (specs) over the build/resume ledgers `notes/28O`/`28P`;
  same-role-name collisions fail-fast pre-network (the `28Q` §4 rider; priced background
  `notes/28U` (née 293) §5).
- **Static loading & bundle emission (cwd parity, transitive dependencies, plan forms,
  source maps)** — **`plans/30I`** (THE design; one load model feeding analysis,
  explicit bundles, multipart plans, and full flattening; its §13 is the xfail
  specimen matrix) over the closure build record `notes/30G`.
- **When convergence vocabulary may license survival** — **`plans/30J`** (THE
  ruled P2 model: one genuine predict qualifies the whole family's verdict/read
  selectors within its speaker closure; implementation waits for the real-oracle
  boundary).
- **Correctness-tooling (checker triad · Lean mini-model · Aeneas)** — **`notes/28T`** (the
  adoption plan); evidence base `.claude/research/refinement-types-industrial-cost/`; the two
  proof spikes live on `ai/research-{lean-sparing,aeneas}-spike` (disposition pending).
- **The first real-machine field trial** — `plans/252` (THE protocol — SUPERSEDED as
  ceremony 2026-07-27, see `notes/26D`; the tooling survives at `Research/trial/`) +
  `notes/254` (adjudication + retrospective ledger) + `notes/255-*` (book + dry-run +
  predictions — still the live-run instrument).
- **Multi-host & read-concurrency** — `plans/262` (build spine) + `plans/260`/`261`,
  adjudicated `notes/26A` (round 26, branch `ai/spike3-r26`); joined out-of-order by
  `notes/26B` (reactive plan-construction direction + the capture-fold deferral,
  minted mid-r27).
- **Wrapper contexts (sudo/su/env): context-entry probing, the escalation dial, the
  tolerance vouch** — **`plans/27C` (THE current spec: measurement in the site's denoted
  context; reuse-never-acquire; the fallback consumption lane)** over `notes/273` (the
  wrapper surface redesign — predict absorbs wrapper modeling; dissolved `24S` §2b/§6b)
  over `plans/24S` (proposal-tier; its §0 impossibility ledger stands).
- **Payload decomposition (`sh -c` strings, heredoc books, the render ladder)** —
  `plans/24T` (proposal-tier; composes with 24S).
- **Dotfiles / ops-siblinghood / why-run honesty** — `plans/24R` (secondary positions;
  the §0a why-run ledger governs every dry-run claim).
- **Where did round-24's unfinished work go?** — `notes/24U` (close-out + reshuffle map)
  → `plans/270` (the round-27 charter; block arc + adjudication ledger + the naming
  discipline binding all briefs).
- **The entity algebra (coordinate · selector dialect · comparison chokepoint)** —
  **`notes/277` (THE spec)** over `plans/271` (the block-settle rulings ledger + task
  map whose typed acks it assembles). The mark GRAMMAR moved: **`plans/281`**
  supersedes `277` §4 (verb vocabulary · `@` selector · `#:` carrier · the respell
  grep-map).
- **Kind-side topology & never-derive-separation** — `notes/272` (address-derived
  topology; `kind__state_stored_only_in()`; §12 is its ratification-status table).
- **Value-predictions & the capture lane** — `notes/275` (its §6 carries a NOT-RATIFIED
  banner — honor it); origin analysis `notes/219` (round-21 vintage; concepts carry,
  cites stale); the BUILD is deferred to the r26 revival — direction + design bank
  `notes/26B` (reactive plan-construction; the binding-site gate-question) +
  **`notes/26C`** (the fixpoint semantics made precise; the quiet-welding audit; the
  R0–R4 revival ladder; the captured-bytes-as-data law ask).
- **Eval'ers & the `dorc:sh` reentry trio** — `notes/274` (reentry token ·
  descend-don't-license · the probe-shipping split).
- **Did round-27's design survive adversarial review?** — `notes/279f` (the crosscheck
  adjudication: verdicts, spec amendments, brief riders, ask-list, dismissals).
- **User aid (errors / hints / lints / `dorc why` / whylog)** — root **`AID-NEEDS.md`**
  (THE registry + law) over `notes/27V` (the build phase: whylog, evidence plane,
  one-catalog, gap ledger) · `notes/27U` (the build phase's as-built ledger) ·
  `plans/282` (the transcript-case prose pipeline; the round-28 build) · `notes/27W`
  (authored decline-classes + the versioned report lane; show-the-code) ·
  USER_STORY's "Recovery" section · `notes/27R`/`27S`
  (the `dorc lint` lane) · the r22 spine `plans/22A` + `notes/22D`/`22E` · `plans/111`
  (the round-11 conclusion).
- **The `dorc explain` teaching surface (density registers · command-block
  transclusion · the docs-home strawman)** — **`plans/286`** (design-tier,
  build-punted; the AI-voice carve + the complexity-ceiling law live there);
  research base `.claude/research/explain-prose-reuse/`.

## Per-round map

- **r1 foundations** — the problem carved up; parser prior-art, engine shape, positioning.
  → `plans/021` · `plans/041`; survey notes `010`–`040`; `learning-path/` (human
  curriculum).
- **r2 analysis engine** — soundness/mutation/scale prior-art; the soundness-bias inversion.
  → `plans/055`; notes `050`–`054`.
- **r3 userbase & problem-space** — corpus plan + orchestration go/no-go + user-studies.
  → `plans/063` · `plans/064`; notes `060`–`062`.
- **r4 performance** — complexity cliffs, probe parallelism, build-systems prior-art.
  → `plans/076`; notes `070`–`075`.
- **r5 recovery (trace-don't-derive)** — → `plans/077` (*live constraint*) ·
  `plans/deferred/078` (privileged tracer — deferred).
- **r6 corpus go/no-go** — instrument + first tally. → `plans/086` (the de-biased validation
  protocol; `notes/081` = why the blind three-model variant must NOT be run); `notes/080`
  (interim tally; ~95%-test-code caveat).
- **r7–8 synthesis & kill-criteria** — `plans/083` (**historical**: the rounds-1–5 accord;
  its "last gate before first code" framing is dead) · `notes/087` → `plans/088`
  (build-to-kill, advisory).
- **r9 state-tracking + specimens** — relational/referent-agnostic frame; the impossibility
  walls (Rice · Ramalingam · frame problem). → `plans/099` · `plans/09A` (specimens are a
  design-quarry, not a measurement); notes `091`–`096`; `specimens/090`–`093`.
- **r10 security** — Chef why-run refutation; Salt-CVE blast-radius; `kAGENTLESS` welded.
  → `plans/101` + `plans/102`; sources `notes/100`.
- **r11 error/provenance spine** — → `plans/111`.
- **r12 cross-network TDD / DST** — → `plans/128` (`plans/121` is the frozen mid-round map);
  notes `120`–`127`.
- **r13 platform-compat** — `kLANG` weld; tier-A/B targets; CRLF. → `plans/139` ·
  `plans/deferred/13A` (Win32 bootstrap addendum).
- **r14 transport (`kCOMMS`)** — executorless-OOB. → `plans/142`; graded primaries
  `notes/141`.
- **r15 adversarial premise-review** — no plan, by design; convergence is the signal.
  → `notes/151` (the hinge) + `notes/150`; citation-audit `notes/20260604-*`.
- **r16 impl-spike 1 (`do-4`)** — the cheapest `055` tier built; apply-2 end-to-end under
  DST. → `plans/16P` (postmortem — read its §3 built-vs-designed ledger first) + `plans/16Q`
  (the precision/recency keystone owed). **Quarantine:** the raw round-16 notes + spike code
  are in `notes/quarantine-DO-NOT-READ/` — reach evidence only through the postmortems.
- **r17 kinds/types** — the symbol-grounding hinge, mapped. → `plans/17N` (on-ramp; built
  from `175` + `17H`) + `notes/17O` (adversarial crosscheck); strawmen `notes/17x-strawmen/`.
- **r18 substrate prior-art** (small; no plan) — `notes/180` (substrate wave-1) ·
  `notes/181` (the ANALYZER-NEEDS extraction method).
- **r19 spike-2: the keystone** — the precision/recency layer `16Q` owed; the Half-B
  (guard-subsumption) insight. → `plans/191` (charter) · `plans/19H` (value-plane needs +
  check-contract lifting) · `plans/19I` (corpus as acceptance measuring-stick); handoff
  `notes/196`.
- **r20 "take-3" build round** — input-side/check-dialect rebuild, per-task strain notes.
  → `plans/20K` (round report) · `plans/20V` (errexit doors) · `plans/20U` (overnight
  addendum).
- **r21 build continuation** — errexit/rc-deadness doors, render/splice repairs, the
  differential harness. → `plans/21W` (close report) · `plans/21Z` (spike-4
  error-provenance inventory).
- **r22 errors/provenance research** — the round-11 spine deepened toward tooling.
  → `plans/22A` (synthesis) · `plans/22H` (live-plan / concurrent / incremental seed —
  what forces `kCOMMS`) · `plans/22W` (close report).
- **r23 the oracle-contract crisis** — the poison-default dilemma → the ternary verdict
  {elide, guard, run}; converged-vouch; two-halves doctrine. → `plans/233` (HUMAN crisis
  log — **honor the end-annotation**) → crosschecks `notes/234`–`237` → ceiling `notes/238`
  → `plans/239` (SIGNED closure) · guard pin-set `notes/23A` · rulings ledger `plans/23D` ·
  `notes/23O` (settled law + the round's full history).
- **r24 the build round** (CLOSED 2026-07-10 by reshuffle → `notes/24U`; branch
  `ai/spike3-r23`) — the `plans/240` ladder Stages 1–5 LANDED (honest walls · survival
  machine · guard tier + claim algebra · derived footprints · resolve/reaches). →
  `notes/24C` (landing/residue ledger — accretes per landing) · stage specs
  `notes/24A`/`24D`/`24E`/`24F` · `notes/24G` (kind-owner family) · the language arc
  `notes/24Kc` → `plans/24L` → **`notes/24M` (binding rulings)** · `notes/24I` (e2e
  slimming) · `notes/24J` (pipe-guard lift) · `notes/24P`/`24Q` (respell specimens +
  shebang digest) · `notes/24O` (stage-6 dispositions) · the late design keystones
  `plans/24R`/`24S`/`24T` · **`notes/24U` (close-out + where every owed item moved)**.
- **r25 field-trial methodology** (branch `ai/spike3-r25`) — the human's first real contact,
  pre-registered so the day yields decisions. → `plans/250` (charter) · `plans/252` (THE
  protocol + fan-out) · `notes/254` (adjudication + retrospective ledger) · `notes/255-*`
  (validated book + dry-run + predictions) · `notes/256` (system recon).
- **r26 multi-host + read-concurrency** (branch `ai/spike3-r26`; TABLED — plans +
  adjudication only, zero build commits; resumes post-field-trial per `plans/270` §5)
  — → `plans/262` (build spine) · `plans/260` (multi-host) · `plans/261` (read-concurrency)
  · `notes/26A` (crosscheck adjudication) · `notes/26B` (reactive plan-construction
  direction + capture-fold deferral bank; minted out-of-order 2026-07-17, mid-r27)
  · `notes/26C` (same-day deep pass: fixpoint semantics + conflict carve, the
  quiet-welding audit vs the r27 tip, the R0–R4 revival implementation ladder).
- **r27 the consolidation round** (CLOSED 2026-07-18; branch `ai/spike3-r23`) — rest-of-round-24 +
  the wrapper/payload work, authored-once discipline. → **`plans/270` (charter: block
  arc · adjudication ledger · naming discipline)**; predecessor accounting `notes/24U`.
  Block-settle CLOSED 2026-07-12 → **`plans/271` (rulings ledger + task map)** + the arc
  durables `notes/272` (address-derived topology) · `273` (the wrapper surface:
  predict-merge + `cmd__lend_map()`) · `274` (eval'er + reentry token) · `275`
  (value-predictions; §6 carries a NOT-RATIFIED banner) · `276` (the language sitting;
  the kWHICHSH floor weld) · `277` (THE entity-algebra/mark-grammar spec) · `278`
  (dorc-lang v0.1 reference, DRAFT). The 270-era adversarial crosscheck, adjudicated →
  `notes/279f`. Wrapper/context transport resolved by context-entry probing →
  **`plans/27C`** (trail: `notes/27A` superseded-in-part · `notes/27B`
  superseded-as-design). Spike steering law (`spike/CLAUDE.md` + the seven crate
  `CLAUDE.md`s) rewritten current-truth 2026-07-16. The user-aid design sitting
  (2026-07-18) minted root `AID-NEEDS.md` + `notes/27V` (build phase: whylog ·
  evidence plane · one-catalog) + USER_STORY's "Recovery" section; the
  `dorc lint` lane landed as `notes/27R`/`27S`. Build ledgers: block-rebuild `notes/27D`
  (+`27E`–`27I`) · block-context `notes/27K`/`27L`/`27N`–`27P` · the aid as-built `notes/27U`.

- **r28 (BUILD COMPLETE 2026-07-20; branch `ai/r28-impl`)** — the syntax v0.2
  unification + the errorloom prose pipeline, both built. Specs: **`plans/280`**
  (charter) · **`plans/281`** (THE annotation mark-grammar, now v0.2 in-code) ·
  **`plans/282`** (errorloom: the transcript-case prose pipeline). Build record:
  **`notes/28A`** (THE conduct ledger — every ruling + the human deferred-queue) ·
  `notes/28B` (respell map) · `notes/28C` (janitor sweep) · `notes/283`
  (generation-flip map) · `notes/285a`–`285d` (the DeepSeek errorloom review +
  conductor adjudication + polish repair-plan). What landed: errorloom the
  standalone crate; the `281` mark-grammar cutover (`@`/word-verbs/`#:`/v0.2
  marker; grep-map in `281`'s tail is now history); the `282` generation flip
  (catalog case-derived, promote-v2, roster retired) + phase-5 case backport;
  docs/steering re-synthesis. De-passthrough KILLED (opaque sibling lane owns the
  taint work). Deferred (human-owned, `28A`): the `sm `-prose Fable pass · catalog
  canonicalization · glued-param seam · errorloom LICENSE/publish. The post-charter
  arcs, each with its own durable: errorloom phase-three `notes/287` · the aid/loom
  unification `plans/288` → `notes/289` (+ maps `28Va` (née 290)/`28Vb` (née 291)) · the why-surface
  sitting + W1→W4 build `notes/28E`/`plans/28G` → ledgers `notes/28F`/`28H`
  (red-lines `28I`, W5 worklist `28J`) · THE LOOM-FINAL ARC (stamped provenance;
  the ~176-case loom corpus) — conduct `notes/28L`, prose accounting `notes/28N` ·
  the name-resolution sitting `plans/28K`/`28M` → ledgers `notes/28O`/`28P` (+ the
  input-surface/role-collision note `28U`) · the error-message-authorship close
  `notes/28S` (71 codes prose-authored; the ProseTier ratchet) · the kernel arc
  **`plans/28Q`** (LIVE) + its review `notes/28R` · the correctness-tooling
  synthesis **`notes/28T`**. Human queues ride `28F`/`28H`/`28Q` §9.

- **r29** — a quarantined lane (`Research/quarantine-DO-NOT-READ/`); off-limits.

- **r26-revival — live execution (MINTED 2026-07-27; executed arcs landed 2026-07-31;
  the ROUND stays open)** — Dorc ran against a real machine and the numbers held: the
  ssh executor (pipe-completeness — `dorc apply host <plan.sh` does its own ssh'ing),
  the gate/bless-tier live-acceptance loop (real ssh / real apt, never hot-loop), the
  Vultr experimentation kit. THE seed: **`notes/26D`**; arc ledgers: conduct
  `notes/26F` · the kernel arc `notes/26G`+`26H` (26G only WITH its three appended
  corrections) · the adversarial kernel review `notes/26I` (maximum-skepticism law) ·
  builtin-deny `notes/26J` · the ops-glue-residue round: `KNOBS.md:kBOOT` + root
  `SIBLINGS.md` + **`plans/26K`** (§0 is the actionable head). NOT complete: the `26K`
  §0a fruit arc is unstarted, and the reactive/capture + multi-host revival waits
  behind the r28/`28Q` push; stdlib and the why/loom prose tails stay gently held —
  `LIVING_STATUS.md`. (Same round-id as the tabled multi-host r26 above — branch
  lineage, not chronology; this entry is its revival.)

## Vendor/ (full-history clones)

CoLiS ecosystem (morbig, morsmall, colis-language, colis-constraints, shstats, lintshell,
…), shellcheck, mvdan-sh, smoosh, oils, goblint-analyzer, tree-sitter-bash. See the
source-manifest for grades/licenses.
