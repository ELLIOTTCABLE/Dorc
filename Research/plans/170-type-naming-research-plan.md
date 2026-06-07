# Round 17 — "type naming": kind-identity spelling & the minimal type-discipline (research plan)

*Status: GATE artifact (the interactive-research skill's `plan.md`, sited in the `Research/plans/`
convention). **Pre-gather** — this charters the round; it leads with the *inherited* frame (what prior
rounds settle going in) and then maps the two threads + fronts. Per-front raw findings will land in
`notes/170`–`17x`; the round closes with **two maps** (not a synthesis — see §Sequencing). Sources graded
into `../sources.json` via `new-source.sh`. Prose is AI-generated; trust the human-written repo-root docs
(`DESIGN`/`KNOBS`/`README`/`TODO`) over it.*

Target: the **single unspelled artifact** four rounds reached and deferred (`notes/151` THE CONVERGENCE,
`16Q §3 dq-kOOB`) — the sh idiom by which an oracle (a) NAMES the kind its predicate serves, (b) anchors a
sound skip, (c) reports a verdict. Round 17 attacks the part that is genuinely open: **how the named kind is
spelled, and what type-discipline consumes it.**

---

## The framing — one firewall, two threads

`learning-path/gradual-success-typing.ai-pointers.md` drew a firewall and then fenced one side off as
contaminating ("the entity-identity nightmare — $PATH-derivation, cross-manager package identity, the
versioning-lattice — is a *different* literature: abstract domains for strings; equivalence/ontology. Keep it
separate"). This round takes on **both** sides, kept deliberately apart:

- **K1 — kind-identity spelling** (the fenced-off "entity-identity / equivalence / ontology" thread). "Say
  `cp`'s arg-1 is *the same thing as* `rm`'s arg-1." The **declaration / spelling** question. Feeds `dq-kOOB`
  + the *identity* half of `dq-entity-algebra`.
- **K2 — the minimal type-discipline** (the pointers-file's actual Tier-A/B/C body: success/gradual/soft/
  pluggable typing, effects, occurrence-typing). The **semantics** question — the least type-system that
  won't force worse work later. Feeds the *shape* half of `dq-entity-algebra` + `kBURDEN`/`kVERIFY`.

The firewall is a **method rule, not just a topic split**: the two threads are researched in **two separate
fresh contexts** (`_tmp-170-k1-brief.md` / `_tmp-170-k2-brief.md`), so K1 never sees the PLT/typing material
and cannot be poisoned by it (and is steered *away* from academic-PLT framing on purpose). They reunite only
*after* the round — see Sequencing. Reason: the ontology tar-pit must not swallow the encoding question, and
"it's just types" must not hand-wave past the genuinely-unsolved identity problem.

### K1's North Star (the human's q-1 steer, load-bearing)
The bar is **independent value**, and the floor to beat is named: *shell-comment type-annotations*
(`# dorc: kind=package`) are the **failure floor** — inert, Dorc-only, `kOOB`-forbidden. This round reaches
*up* from that floor for spelling that **pays for itself without Dorc**. The merely-inert off-ramp ("still
runs under `dash`, does nothing") is a necessity but is *trivial* (mechanical method-rename) and **not this
round's target.** K1 succeeds iff it finds a way to spell cross-referent kind-identity that a human would
write — or a system already emits — for reasons of its own.

---

## Part A — the inherited frame (true going in; do not relitigate — A3–A5 are subtle, read them slowly)

- **A0 · referent-agnostic by default (not an eternal prohibition).** The *general mechanism* keeps relational
  contracts over opaque symbols and does not infer what `nginx`/`package` *is* (`099 §0/W4`; the `095`
  adjudication over the Harnad lens). This is the round's frame and the default posture — it is **not** a
  forever-ban on deliberately special-casing a few critical features later, once a general mechanism exists.
  Don't read "Dorc never decides what X is" as an absolute; this round simply doesn't *need* to, so the feature
  stays fully general.
- **A1 · the shape is fact-centric + 3-place (settled `16Q §4`; carry-into-DESIGN pending).** Cross-oracle
  identity binds to a **named kind**, never a shared token; the relation is **3-place** `(kind, provider,
  verb) → effect`, not a 1-place naming convention (which clobbers — `X3`, `notes/151`); the oracle is plain sh
  the analyzer **lifts statically, never sources/runs**; the anchor probes a *fact* ("does `kind:entity` hold"),
  never dry-runs the mutator (`16P` T7 / DP-1).
- **A2 · the `kOOB` lean — a lean, not a survey-blinder.** Dorc *leans* in-band (`kOOB-directional`): sidecar
  *configuration* (YAML/frontmatter/pragma/comment-parsing) is generally distasteful and the design wants to
  avoid it; the preferred shape is an analyzer-internal index lifted from user-authored sh (the `X3` de-risk).
  **But this round still surveys the full space, sidecar-config included** — if it turned out everyone
  (Ansible/K8s/Puppet/…) shared one declaration form we could crib, that is a significant finding even if
  unwelcome. The redline informs the map's *lean*; it must not blind the survey.

> **A3–A5 are the soundness machinery. Agents repeatedly conflate the three things in A3; the whole map
> depends on keeping them apart. This is the one place to read slowly.**

- **A3 · three distinct dualities that all borrow the words "must / may / sound" — never conflate them.**
  - **(i) Engler MUST vs MAY — *trust of the contract* (where an anchor comes from)** (`096`). MUST = the
    relational contract is *directly implied by idiomatic structure* (a guard) **or** *oracle-declared* —
    sound; elision *may* rest on it. MAY = *mined / distributional / co-occurrence* — a ranked hint that
    bootstraps the oracle library **offline**, and **never** licenses a per-run skip. (Necessary for elision,
    *not* sufficient — see A4.)
  - **(ii) may- vs must-*analysis* — *approximation orientation* (order-dual lattices)** (`16P` T4). A
    **may**-analysis over-approximates (⊥-start, ⊔/union-merge: "holds on *some* path"); a **must**-analysis
    under-approximates (the order-dual, ⊓/intersection: "holds on *all* paths"). The coercion is **one-way**:
    `Must → May` only — a "might-hold" can never be re-promoted into a skip-license.
  - **(iii) `kFAIL` phase-keyed soundness — *the safe failure-direction, opposite per phase*** (`KNOBS kFAIL`,
    `16P` T5). **Probe phase ⇒ `kFAIL-withhold`** (unsure → do *not* act/probe; never mutate in a read-only
    pass). **Apply phase ⇒ `kFAIL-perform`** (unsure → *run*; never skip a needed mutation). Two **opposite**
    safe directions, **welded**, not a dial. An "unknown" therefore rounds the *opposite way in each phase*.
  The trap: the `∧ Must` in the elision predicate (A4) is **(ii)** the dataflow under-approximation — **not**
  **(i)** Engler-MUST. A spelling can carry an Engler-MUST contract and still be un-elidable because the
  *analysis* is only May-grade, or because the *phase* folds the other way. Three axes, not one.
- **A4 · a skip (replace) is a bidirectional CONJUNCTION, never "a MUST contract."** The apply-2 elision
  predicate (`16P` T13): **elide leaf L iff** `probe(L.fact)=Converged` **∧** `ambient` **∧** `Must` **∧**
  `no-consumed-unvouched-observable` **∧** `¬⊤-contained` — **and** `can't-probe ⇒ can't-elide` (a kind with
  an effect but no declared probe is un-checkable → it runs, even on a host that holds the fact). The
  directions in play:
  - **forward** — `ambient` = reaching-definitions over the effect-map (value-validity: did an upstream
    in-script write disturb the fact? if so the resting probe is stale → run) (`16P` T8).
  - **liveness / relevance** — `no-consumed-unvouched-observable` (the observable/replace model: a value-bearing
    consumed stdout/stderr blocks the stub) (`16P` T10). In apply-2 this is computed **structurally during CFG
    lowering, *not* as a backward solve** (the flattened CFG can't see pipe-consumption; a backward fixpoint was
    *considered and rejected*) (`16P` T11). A *genuine* backward dataflow appears only in **apply-3** (the
    targeted desired-set `dorc try`: apply-2 **plus** a backward relevance-reduction; **apply-3 ⊃ apply-2**)
    (`16P` T13).
  - **superposition** (`16P` T11): the engine emits **phase- and orientation-agnostic** facts; only the
    *phased caller* collapses them. Never bake a phase default into a fact — a baked posture is a wrong-skip
    under the opposite phase's `kFAIL`.
- **A5 · the floor, and why ⊤ is safe in *both* phases (a two-step, not magic).** Anything
  unanchored / undecidable / transient / unmodeled ⇒ ⊤. The safety is a *chain*, not a single move: ⊤ in the
  **probe** phase ⇒ withhold the probe (`kFAIL-withhold`) ⇒ no converged-fact ⇒ in the **apply** phase the leaf
  **runs** (`kFAIL-perform`) ⇒ **apply-1** (full unconditional run) is the floor — never worse than not using
  Dorc. So both maps may **kill aggressively**: a killed candidate degrades *to the floor*, never to
  incorrectness. **Live hazard to respect (SF-3, `notes/151`):** a single three-valued verdict
  (`Converged/Diverged/Unknown`) *crossing both phases* cannot carry the two opposite `kFAIL` fail-orientations
  at once — don't design one artifact to serve both phases' folds.
- **A6 · the welds K2 stays inside.** In-bounds: the *forgiving* lineage (success / gradual / soft / pluggable
  typing, typestate, small effect systems, occurrence typing). Welded **out** (`kVERIFY` = "TypeScript, not
  Coq"): HM / full-inference, dependent types, Cousot-Galois soundness machinery — they buy a soundness Dorc
  structurally cannot collect (`16P` T12). Also welded: `kLANG` (sh), `kVOLATILES` (hermetic).

---

## Part B — K1 fronts (entity-identity spelling; prior-art + idiom hunt, hindsight-forward)

*Orientation: how does the world **already** declare cross-referent kind-identity in a form that pays for
itself, and how did that age? Keep/kill/order is the agent's call with the human in-loop — **not** pre-judged
here. (Aim the prior-art search at ops / packaging / the-web-in-the-wild, not academic-PLT type theory — that's
K2's firewalled half.)*

K1 value-criteria (what makes a finding worth keeping — judge by these, not by this charter's opinion of a
lead): (a) **independent value** — the declaration pays for itself without Dorc (beats the comment-floor);
(b) **cross-referent-identity fit** — it actually says "these two operations touch the same named thing";
(c) **inference-leverage** — how little the user must type; (d) **hindsight** — old enough to carry a verdict
on how the approach aged.

Fronts (neutral territory — angles + leads already surfaced, no rankings):
- **f1-provides** — package-manager capability/virtual systems. Leads: Debian `Provides:` + virtual packages,
  RPM `Provides:`/`Requires:`, `update-alternatives`, `pkg-config` `.pc`.
- **f2-crossmanager** — cross-ecosystem package identity ("apt's nginx ≡ brew's nginx"). Leads: purl/package-url,
  repology project-identity, CPE, SWID / SPDX external-refs.
- **f3-ontology** — equivalence / ontology declarations. Leads: RDF `owl:sameAs`, OWL equivalence,
  ontology-alignment; microformats / microdata / RDFa (semantics in the artifact you'd write anyway).
- **f4-sh-idiom** — the sh-native spelling + the off-Dorc-value bar. Leads: oracle-as-real-helper-library;
  reading existing system metadata (systemd `Wants=`/`Requires=`, dpkg status, `.desktop`/mailcap); sh
  namespacing under the 1-place-clobber constraint (`X3`).
- **f5-recognition** — recognize-the-kind-without-config mechanisms. Leads: ShellCheck `# shellcheck`
  directives, tree-sitter / semgrep / CodeQL patterns, `file(1)` magic, Make suffix/pattern rules.

---

## Part C — K2 fronts (the minimal type-discipline; disprove / avoid / kill)

*Orientation: minimize. For each mechanism ask "do we need it, or does declare-don't-infer + ⊤-run already
cover it?" The kill-orientation is the round's *goal*; the *outcome* of each kill is for the agent to find, not
assumed here. Keep/kill/order is the agent's call with the human in-loop.*

K2 value-criteria: (a) **minimality** — does this earn its complexity, or does the floor already cover it;
(b) **forgiving fit** — does it belong to the never-reject lineage (vs the welded-out sound/total one);
(c) **kill-justification** — every exclusion needs a *cited* "why we can skip it," not an assertion.

Fronts (neutral territory — angles + leads):
- **f6-success** — success typings / Dialyzer (Lindahl & Sagonas): the "discrepancy, not error" posture; the
  success-set polarity; how it aged.
- **f7-gradual** — gradual typing + the gradual guarantee (Siek & Taha; Siek/Vitousek/Cimini/Boyland): `?`/`Dyn`
  vs `Opaque`/⊤; consistency-not-subtyping; the gradual guarantee vs no-cliff. Kill-question: the
  gradual-typing performance literature (Takikawa "Is Sound Gradual Typing Dead?") — does it bear on a
  cast-free analyzer?
- **f8-soft-pluggable** — soft typing (Cartwright/Fagan); pluggable / optional types (Bracha); the stub-library
  social model (DefinitelyTyped, typeshed) as an oracle-corpus governance analog.
- **f9-typestate** — typestate (Strom/Yemini; Aldrich); effect systems (Lucassen-Gifford; Koka). Questions: is
  typestate the right frame for install/purge transitions; how much effect-machinery is too much.
- **f10-minimality** — the kill front: nominal-vs-structural; subtyping-vs-flat+⊤ (the `dq-entity-algebra`
  flat-vs-structured call); unification / HM; Wadler-Blott **coherence** as a candidate un-dodgeable rule.
  Posture anchors: soundiness (Livshits et al.); TypeScript's own unsound-by-design non-goals.

---

## Knobs touched (KNOBS.md is human-authoritative; no new knob proposed pre-gather)
- **`kOOB`** (`directional`→ maybe `welded` via `dq-kOOB`) — K1's whole question is "what in-band sh form is
  legitimate." **`kBURDEN`** (`declare-anchor + infer-propagation`, `099 §5`) — K2 sizes *how little* must be
  declared. **`kVERIFY`/`kFAIL`/`kLANG`/`kVOLATILES`** (welded) — K2's guard-rails. `dq-entity-algebra` /
  `dq-kOOB` (16Q decision-slugs, not KNOBS) are the two retrofit-hostile calls both maps feed.

## Sequencing & deliverable shape (the human's q-2/q-3 steer)
1. **Gather** both threads in their two firewalled contexts → graded sources + `notes/17x`.
2. **Two maps** (not a synthesis, not a decision): each thread closes as a **map with a lean** — the
   prior-art ranked by closeness-to-constraint (K1) / the include-exclude boundary with per-mechanism
   justification (K2). A *map*, because the human takes the **lean into adversarial-crosscheck** to earn it.
3. **Adversarial** (human-run, end of round) → then **synthesis** (after adversarial, reuniting the threads —
   the named kind is one object: an identity-anchor for K1, a nominal type for K2).
   *This round stops at step 2.*

## Open questions for the human (slugged; gate before/during gather)
- **q-1 (K1 closeness bar)** — *answered:* independent-value is the goal; beat the comment-annotation floor;
  inert-off-ramp is trivial and out-of-focus. (Recorded here so the K1 agent inherits it.)
- **q-2 (deliverable)** — *answered:* map-with-a-lean; human applies adversarial for the lean.
- **q-3 (firewall)** — *answered:* hold through gather + maps; synthesis only after adversarial.
- **q-4 (run order)** — default: K1 and K2 in **parallel** fresh contexts (firewall makes them independent);
  within each, fronts in listed order. Confirm, or serialize?
- **q-5 (quarantine)** — round-16 spike code/notes stay `DO-NOT-READ`; both agents reach last-mile evidence
  only through `16P`/`16Q` citations. Confirm that's the intended boundary.
