# 23M — the golden-hill round (elide-past-a-running-command): live working note

AI-authored, 2026-07-03, round #19, IN PROGRESS (chat-driven, human remote). NOT stamped —
raw working material, confidence-marked. Nothing here is welded; the round settles by dialogue
→ (eventual) adversarial crosscheck → re-welds/pins. Seeds: `notes/238`, `plans/23D` §5, `23J`
lane-privilege, the human's item-3 golden-hill statement (2026-07-03 chat).

## STATUS (read first): this round produced TERMINOLOGY + LANDMINES, not a design

Honest scope, 2026-07-03: round #19-so-far has been DISCUSSION, not design. What now exists is
(a) shared terminology for the problem-space (glossary below), (b) a map of where the naked
unsoundness sits and where it has been caged, (c) one load-bearing reframe (233 is permanent;
silence = wall; trust is opt-in). What does NOT exist: a software design — no spelled contract,
no wire protocol, no engine mechanism, no pins, no strawman that survived scrutiny. Nothing here
is welded or stamped. Do NOT build against this; it is the pre-design map, and the adversarial
crosscheck has not run.

## GLOSSARY (the round's main concrete product — all terms PROVISIONAL)

- **spell / spelled** — write sh that Dorc mechanically lifts/analyzes/ships (code-plane;
  typecheck/lint act here).
- **profess / professed** — state in human-facing prose a promise/boundary a user READS
  (trust-plane; NOT mechanical).
- **horizon** — the ONE global, Dorc-professed liability boundary ("past here you're on your
  own: [named residue]"). Not per-oracle; not a code concept.
- **footprint** — the set of ENTITIES a command touches; authored for fixed tools, DERIVED at
  probe time (a probe-time entity-set) for payload-bound tools (apt, hork). Positive, bounded.
- **backing** — where a fact's truth lives, operationally = its verdict-probe's read-set. The
  most ~SUSPECT surviving claim of the round.
- **disjointness** — engine intersects a running command's footprint against a downstream fact's
  backing; empty ⇒ elide-past-the-running-command; non-empty / ungrounded ⇒ guard or wall.
- **coordinate-kind** — a shared vocabulary with an OWNER (apt.Package, systemd.Service,
  dns.Zone, kernel.Sysctl…); anything with an owner, NOT filesystem-only.
- **entity-identity** — what counts as an entity in a kind, and when two references are the same
  one (aliasing); held by the ONE owner.
- **contribution** — adding a property/cell to a shared kind + depending on its entities; OPEN to
  all — the collaboration (scan_cve adds cve_clean onto apt.Package). Distinct from owning
  identity.
- **residue** — a DISCLOSED HOLE ("here is where we break, on purpose, and we tell you"); the
  contents of the horizon; NOT a rescue mechanism.
- **vetos** — [DEFERRED] optional owner-spelled protective veto of elisions it can't prove
  unsound; parked someday-maybe.
- **the three failure-directions** — *dangerous* (under-execute; must be caged in a
  bounded/attendable/opt-in home), *value* (over-verify; may roam unbounded), *disclosed*
  (horizon residue; professed, not defended).

## CHANGED vs. CHURNED (the honest ledger — did we fix anything, or just churn?)

CHANGED (concrete, load-bearing, vs the pre-crisis 233 "dangerous-middle"):
1. **Silence semantics: DEFAULT-TRUST → DEFAULT-WALL.** Pre-crisis, an oracle's silence licensed
   elision (unsound). Now silence licenses nothing; silence is a wall. This is the whole
   ballgame and it is a real change, not a rename.
2. **Burden assignment: nobody → the beneficiary, opt-in.** Pre-crisis nobody discharged the
   completeness claim and everyone got wrong elisions. Now the author who WANTS cross-oracle
   elision does the grounding work, opt-in, and slacking costs them VALUE, not correctness.
3. **Blame: unattributable-and-silent → concentrated + mostly attributable.** Wrong elisions
   went from silent-and-everywhere to {disclosed-horizon, owner-bounded-within-kind,
   mechanical-dangling-reference}.
4. **Understanding: entity-granular poisoning = the 233-impossibility = the collaboration
   engine.** Conceptual, not a mechanism change, but load-bearing (below).

CHURNED (relocated, not fixed — the human's own point, conceded):
- The completeness-claim still EXISTS — relocated to the owner's within-kind identity-coherence,
  never killed. 233 is caged, never beaten.
- footprint / backing / disjointness IS the pre-crisis "declare your dependencies" idea,
  rediscovered — now sound ONLY because of the silence=wall + opt-in fixes above. We churned
  back to dependency-declaration with the single fix that makes it honest.

## THE ROUND'S SPINE (human-landing 2026-07-03; PROVISIONAL): 233 is permanent — cage it, don't fix it

The week's churn taught the human (and it survives the conductor's check): **233 — grounding
soundness in a fallible completeness-claim is unsound — is a PERMANENT CONDITION of
eliding-past-a-running-command, not a fixable bug.** Sound past-a-wall elision requires SOMEONE
promising completeness over SOME vocabulary; every such promise is human and fallible; no design
removes the need for it. Accept-and-design-*around* is the only honest posture. "Design around"
has a precise, three-move shape (IDENTIFIED, not yet designed):
1. **CONCENTRATE** the naked completeness-claim into its smallest, most-attendable, most-attributable home: an
   owner's no-synonym promise over its OWN bounded, enumerable vocabulary. (The consumer-side
   ecosystem-survey completeness-claim de-fangs entirely to VALUE — miss the existing name → wall, not
   wrong-skip.)
2. **OPT-IN** — the real anti-233 move. 233's specific sin was silence DEFAULTING to trust. The
   answer is not to make silence safe (impossible) but to make silence MEAN NOTHING: silence is
   a wall. The trusted-completeness-claim is never reached by default — only by an explicit grounding ACT, by a
   named owner, over its own vocab. You never *default into* the unsoundness; you can only ever
   *opt into* it, deliberately and attributably.
3. **PRICE the residue honestly.** What's left — the owner's within-namespace no-synonym promise
   — is genuinely NAKED, and (the hard truth) the guard-half is NOT a net under it, because
   elision BYPASSES guards. A within-namespace synonym error is a silent under-execute defended
   only by: attribution (after-the-fact), the conservative-fallback STANCE (pinkie-promise-tier,
   not a typesystem), and an eventual weak coherence-lint (pre-facto). This is the ONE spot the
   design ships naked 233 — it MUST be professed at the horizon in exactly those words.

HOPEFUL CRACK (ques4, tentative): synonyms are a NAMING problem — they exist only because we
reason over professed *names*. Disjointness computed over MEASURED referents (probe-time traced
inodes / resolved entities) would EVAPORATE synonyms (measurement sees through two names to one
referent). It does NOT kill 233 — it trades into "is your measurement complete," the
backing completeness-claim already flagged most-suspect — but it may dissolve the SYNONYM completeness-claim
specifically, for traceable coordinates (fs yes; abstract kinds no). The derived-footprint
thread; the one place with genuine leverage left.

DISCIPLINE going forward (the human's frame, cleaner than the six-question agenda): *name the
quantifier; name its failure-direction; dangerous (under-execute) completeness-claims go in bounded/attendable
homes, opt-in, lint-where-possible; value (over-verify) completeness-claims may be unbounded; everything residual
is disclaimed at the professed horizon.*

## The two planes (PROPOSED terms — human said "mint specific terms"; not yet finally acked)

Repeated conflation of two acts forced this. Provisional vocabulary, in effect for round docs
pending the human's final word on the term choice:
- **spell / spelled** (already canonical): to write sh that Dorc *mechanically* lifts,
  analyzes, or ships. Code-plane. Typecheck/lint/analysis act here. Oracles SPELL footprints.
- **profess / professed** (new): to state, in human-facing prose (README, first-run output,
  docs), a promise or boundary a *user reads*. Trust-plane. Not mechanical. Dorc PROFESSES the
  horizon. (Alternatives offered: "promise"/"advertise"; "profess" leads for collision-freedom.)

## HORIZON = one global, Dorc-professed liability boundary (HUMAN-ORIGINATED, 2026-07-03)

The load-bearing correction of the round. The horizon is NOT a per-oracle code concept and
oracles do NOT declare their own horizons (the conductor's earlier `# horizon:` annotation on a
footprint function was WRONG — deleted). Reasoning (the human's): anything that breaks that
(a) can't be mechanically attributed to a specific broken line AND (b) wasn't already broken in
the user's bare pre-Dorc script becomes, in the user's mind, "because I ran Dorc." So the
horizon is *Dorc's liability boundary*, settable only where Dorc holds communication-leverage:
the frontloaded first-contact surface (README / first-run / first doc page), in Dorc's own
voice. 10,000 oracle docs = an ocean nobody reads = zero leverage. Therefore ONE horizon,
professed once, global; oracle-authors are *pushed* (docs, margin-linting, our own
design-assumptions and protections) to LIVE UP to it. Mostly NOT mechanical work — position and
trust — so most of the horizon lives OUTSIDE this mechanism-round, in the frontloaded-docs work.
The "whole game" of horizons is a balancing act between (1) un-analyzable things and (2)
unshared expectations between authors; very little is typecheckable.

## The mechanism (spelled/analyzed): footprints, backing, disjointness

The elide-past-a-running-command move. F = the downstream fact under test; the retained command
= X (runs between probe and where F's site would run).

- **footprint (spelled, often DERIVED)** — X's oracle spells "what X touches" as entity-
  coordinates in shared kinds. Payload-bound tools (apt) can't author it statically → it is
  DERIVED at probe time by asking the tool (`hork list-plugins`, `dpkg -L`). Positive, bounded.
- **backing (spelled)** — F's oracle spells where F's truth lives, operationally defined as
  *the read-set of F's own verdict-probe* (`is-enabled` reads unit-state + /etc/systemd/**;
  `dpkg-query` reads /var/lib/dpkg/**). ~SUSPECT this operational definition is the answer to
  leg-2's "isn't 'fully carried by S' a fresh completeness universal?" — a fact never promises
  more than its probe measures; backing only covers what the probe reads. (Not yet human-acked.)
- **disjointness (mechanical)** — the engine intersects X's footprint against F's backing, per
  shared coordinate-kind. EMPTY ⇒ F's plan-time proof survives X's run ⇒ F's site ELIDES even
  though X ran. NON-EMPTY ⇒ no proof survival ⇒ F's site GUARDS. Absence of a grounded
  coordinate (ungrounded/undeclared) ⇒ WALL ⇒ guard. Only expressed-and-intersected-empty
  licenses; everything else degrades to verification.

Why this escapes the killed completeness-vouch (the three legs; PROPOSED, not yet acked):
(1) bounded quantifier — a footprint ranges only over the author's OWN attended substrate, not
the other guy's unknowns; (2) named residue — everything past the horizon is a disclosed hole,
not silent trust; (3) derive-where-unwritable — testimony replaced by probe-time derivation for
payload-bound tools. Softest leg = leg 2's backing-completeness (see the operational-probe-read
reframe above). The horizon reframe RELOCATED residue from per-oracle to the one professed
boundary (above), which strengthens leg 2's "named residue" foundation.

## Contribution vs. identity — the collaboration model (conductor-corrected 2026-07-03; the earlier "lint against writing in a namespace you don't own" was WRONG, anti-collaborative — RETRACTED)

Two DIFFERENT operations on a shared kind were conflated. Separating them is the round's key
unlock:
- **CONTRIBUTING** a property/cell to a shared kind, and depending on its entities — `scan_cve`
  adding `cve_clean` onto `apt.Package`, reading `apt.Package:nginx` — is THE WHOLE GAME, open
  to everyone, exactly what Dorc drives authors toward. NO lint against it. This is the
  collaborative-non-communicative construction the reverse-DNS kind-system exists for.
- **ENTITY-IDENTITY** of a kind — what counts as an entity, when two references are the same
  one, the aliasing rules — is what the ONE owner holds. Authority over the *nouns*, never a
  monopoly on the *sentences*.

**The reconciliation (233-impossibility IS the collaboration engine):** apt CANNOT enumerate
properties it has never heard of — "install nginx leaves `cve_clean` alone" is a dead
completeness claim (apt's author never heard of scan_cve). So apt's only HONEST footprint is
entity-granular: "I touched `apt.Package:nginx` — assume every property of that entity moved,
including ones I don't know exist." That is PRECISELY what poisons scan_cve's `cve_clean` and
fires its re-scan. The impossibility 233 named is not the tax on collaboration — it is the
mechanism OF it: apt can't say "I don't touch cve_clean," so it must poison it, so scan_cve gets
its notification for free. (Consequence: footprint poisoning is ENTITY-granular for the touched
entity — known properties poison by declaration, unknown properties of the same entity poison by
silence=wall. Over-conservative on properties apt doesn't really move → over-verify, safe, a
value cost, the honest floor.)

**So the CONCENTRATE-move refines:** the dangerous completeness-claim was never "no third party
writes my names." It is "the owner keeps a coherent entity-identity for its own kind" — one
owner, one bounded question (what are my entities, when are two the same), attendable over its
own substrate. Everyone else contributes cells freely, CONSUMING that identity, never
redefining it.

**Bonus defense (found in re-check):** for an ENUMERABLE kind, a reference to a non-canonical
entity (`apt.Package:nginx-http`, no such package) is a DETECTABLE DANGLING REFERENCE at probe
time (apt knows its packages) → wall or loud diagnostic, NOT a silent under-execute. So the
third-party-MISTAKE case is better-defended than feared; the genuinely-naked residue shrinks to
apt itself declaring one real entity under two real keys — its own bounded substrate, one owner
to attribute. That is the smallest the dangerous claim gets.

## Grounding = owner-provided coordinate translation (types-to-types; the human asked for this)

NOT filesystem-only. A coordinate-kind is anything with an owner: `apt.Package:nginx`,
`systemd.Service:horkd`, `dns.Zone:example.com`, `kernel.Sysctl:net.ipv4.*`. Finer coordinates
RECOVER elisions coarse ones lose (apt + a vendor tool both write /var/lib/dpkg/status →
false-conflict by path; clean-disjoint by package-set). Grounding-bridges are owner-spelled
translation functions (apt's `manifest() { dpkg -L "$1" ;}` expands a package-coordinate into
file-coordinates) — coordinate translation, never kind-equivalence. (NB refined below — manifest()
is a footprint-EXPANSION bridge, not co-reference.)

## THE CROSS-KIND BOUNDARY — footprints do NOT cross kinds unaided (landmine + the NEXT thread, 2026-07-03)

Entity-granular poisoning poisons all properties of a touched entity WITHIN ITS KIND — it does
NOT propagate across kinds. `apt.Package:geo` poisoned does NOT poison `systemd.Service:geo`:
different kinds, different entities. So a package postinst's CHARACTERISTIC effect — enabling its
OWN service — crosses the package→service kind boundary and ESCAPES to residue unaided.
(CORRECTS an over-optimistic conductor claim that "the common maintainer-script effect is
self-caught by entity-granular poisoning": it is NOT — self-caught only WITHIN the touched
entity's kind; the common enable-my-service case is a kind-crossing and escapes. So the deferred-
veto hole is bigger than that reassurance implied, and it lives exactly here.) The BRIDGE is the
non-blunt alternative to vetos for this hole.

Two DISTINCT bridge senses — were sloppily conflated under "grounding-bridge"; separate before working:
- **footprint-expansion bridge** — "touching X REACHES Y" (part-whole, like package→its-files
  via manifest(); OR causal-effect, like package→its-service). DIRECTIONAL; broadens a footprint
  ACROSS kinds. Same 233 shape: can add KNOWN edges, can't claim completeness (a postinst may
  touch any service) → known edges broaden the footprint, the remainder stays residue. Improves
  coverage without claiming completeness. This is what the cross-kind escape needs.
- **co-reference / identity bridge** — "X and Y NAME THE SAME REFERENT" across two OWNED
  namespaces (a vendor's `vendor.Pkg:nginx` ≡ `apt.Package:nginx`; a deliberate cross-namespace
  synonym). SYMMETRIC. The OPT-IN that recovers disjointness across two owners (silence = wall; a
  co-reference assertion reaches for the trusted-claim). The consumer-side grounding act.

RECURRING THIRD OPTION — the measurement crack, AGAIN: traced referents (probe-time strace/eBPF
of what actually got touched) cross kinds NATURALLY (measurement observes the real unit-file
written, not a declared coordinate) AND dissolve name-synonyms (same referent → one inode). It
keeps reappearing as the answer to the hard cells (synonyms, now cross-kind). Cost: needs tracing
infra (the DX arc) + works only for locally-OBSERVABLE state (fs/process yes; dns.Zone / cloud
API no). Its own hole: backing-completeness ("did I trace everything").

## Residue (HUMAN-CORRECTED framing) + vetos (HUMAN-NAMED + DEFERRED)

Residue is a DISCLOSED HOLE — "here is where our system breaks, on purpose, and we tell you"
(inotify watchers; apt maintainer-scripts doing strange shit) — NOT a mechanism that rescues
anything. It is the contents of the one professed horizon. It is one of 239's two knowingly-
accepted trust edges. **Binary residue** (operative-by-default): intersect in-horizon
territories; the named hole is accepted everywhere; an in-horizon-disjoint site elides, and when
the hole bites it's an accepted/attributed/priced under-execution. **Vetos** (formerly
"reach-grading"; HUMAN-RENAMED + DEFERRED 2026-07-03, someday-maybe): let an oracle
proactively/protectively VETO elisions it can't prove unsound (a spelled, veto-only, judgment-
tier list attached to a named residue class; staleness degrades to the binary floor, never
below — veto-only can only fail to prevent, never newly cause, a wrong elision). Human's verdict:
extra machinery, moves neither correctness nor value needle on its own, just tunes the curve's
middle; tolerable only with aggressive attribution + an admin off-switch. Parked.

## THE DANGEROUS CELL — synonym/coherence (fails toward UNDER-execute; WORKED, residue located)

Every other gap fails SAFE (missing/ungrounded/horizon-exceeded → wall → guard → over-verify).
The one that fails UNSAFE: two honest authors using DIFFERENT names for the SAME referent
(synonyms) ⇒ disjointness intersection comes up EMPTY when it should HIT ⇒ false-disjoint ⇒
under-execute (the cardinal sin). 233's "silence licenses nothing" one layer up (`23D` §5); the
SYNONYM dual of round-17's homonym problem. WORKED in dialogue (see "Contribution vs. identity"
above); result:
- The synonym danger is NOT "third parties writing my names" (that's contribution, encouraged).
  It is the OWNER failing to keep coherent entity-identity WITHIN its own kind (declaring one
  real referent under two real keys). Bounded, own-substrate, one owner to attribute.
- The consumer's "did I find/use the right existing name" failure is VALUE-only (mint a private
  name → cross-namespace → wall). Not dangerous.
- A dangling reference (non-existent entity in an enumerable kind) is mechanically detectable at
  probe time → diagnostic, not silent under-execute.
- RESIDUAL NAKED SPOT (the one place we ship bare 233): owner within-kind identity incoherence,
  defended only by attribution + the conservative-fallback STANCE (pinkie-promise) + an eventual
  weak coherence-lint; NOT netted by the guard-half (elision bypasses guards). MUST be professed.
- Entity-aliasing fence still owed: within-kind identity ≠ string compare
  (symlinks/mounts/normalization); the kind-owner pins entity-identity semantics. Partly
  addressed by the measurement crack (traced referents dissolve name-synonyms).

## PRIOR-ART + THE 236b RELATIONSHIP (surfaced 2026-07-03 — MUST reconcile before #19 proceeds)

CORRECTION (2026-07-03, human — the conductor got reverse-sycophancy'd on first pass): 236b is
a SUBAGENT's adversarial-crosscheck output = SAME SLOP-TIER as 23M, NOT academic prior-art, and
was de-centered by the human FOR A REASON (point 3 below). Its existence is not authority. Two
things are true and were mis-framed on first pass:
- **The re-derivation (23M ≈ 236b's static half) is MAXIMUM-VALUE SIGNAL, not wasted effort.**
  Two INDEPENDENT passes through the slop converging on the same mechanism (footprints,
  invalidation-bases, entity-granular poisoning) is one of the only trustworthy signals when
  working through unreviewed AI output. Convergence ⇒ the static-footprint mechanism is probably
  RIGHT. (Re-value all such convergences this way.)
- **236b-alt1 (the barrier / dynamic re-observation) is DOA for #19, and the spine is VINDICATED
  — see the consent-wall kill below. #19 and #11 are DIFFERENT PRODUCTS, not one problem-space.**

Mapping of the re-derivation (convergence = signal the STATIC mechanism is sound):
- 23M "derived footprint = probe-time entity-set" = **236b-alt3** (probe-computed manifests, `dpkg -L`).
- 23M "backing = probe-read-set" = **236b-alt2** (generation-probes: a kind-owner's digest of the
  substrate its probes read — a per-kind change-detector / generation-token; MORE developed).
- 23M scan_cve "entity-granular poisoning" = **236b-alt6** (property invalidation-bases; refined
  grammar core-cell ∪ own-substrate ∪ volatile/TTL — the EXTENDER declares what kills its property).
- 23M "measurement crack" = **236b-alt5** (observed footprints: measure→ratify→verify).
- The admin-lever gap I never named = **236b-alt4** (admin's one-line book-local stub =
  `changed_when:`/`creates:` recast in sh — the missing cheap rung; 236b-fail9).

THE CONSENT-WALL KILL (human, 2026-07-03 — load-bearing; this is WHY the barrier was
de-centered, and why the spine STANDS): 236b-alt1's barrier re-observes crossing facts AFTER the
wall runs — but the wall runs during APPLY, and:
1. the wall runs during apply, when the user has ALREADY consented;
2. the user can only consent once they've SEEN the plan;
3. seeing the plan CONSUMES attention;
4. elision-as-a-concept exists to CONSERVE attention.
⇒ dynamic-proof-driving-elision is DOA: it spends the very currency it works to conserve. A
dynamically-decided line cannot be ABSENT from the consented plan (probe is non-mutating, so the
post-wall world is unobservable before the plan renders); at best it's a CONTINGENT line ("will
run if re-check fails") — PRESENT, and a conditional costs MORE attention than a plain line. The
barrier conserves PERFORMANCE (skip work at apply), categorically NOT attention.

**Consequence — the spine is VINDICATED, the naked residue is IRREDUCIBLE.** The attention
product REQUIRES trusting a static plan-time proof with NO runtime net — because any net either
costs attention (disclosed contingency) or hides risk (undisclosed → violates
rul-attention-honesty). So the naked-233 residue is not a bug a cleverer mechanism dissolves; it
is THE PRICE of the attention product, by construction. The golden hill is dangerous by
construction — which is exactly why it is the golden hill and not the default. (My first-pass
"236b dissolves the residue via unconditional generation-probe tripwire" was wrong twice:
reverse-sycophancy, AND the dissolution is apply-time, after attention is spent — the tripwire is
a PERFORMANCE-tier safety net, and to be honest it must be disclosed, which re-costs attention.)

**#19 ≠ #11.** #19 (the golden hill) = the ATTENTION product = elide lines from the plan BEFORE
consent = STATIC proof ONLY. #11/236b barrier = the PERFORMANCE product = skip work at apply =
dynamic, offers the attention goal NOTHING. Different products; do not collapse them.

**What survives from the 236b family for #19:** 236b-alt5 (measure → RATIFY → verify) is NOT
dynamic-proof — it's measured evidence (prior runs, eBPF) RATIFIED into a static claim BEFORE the
plan, so it pays attention ONCE (the human's "the consent-wall is a one-time thing") and
amortizes forever. That is just static proof with better evidence; it survives the consent-wall.
The ordinary authored footprint is the same shape: the oracle-AUTHOR pays the attention once, in
the library, amortized across all users forever (the community-library value-prop).

ACADEMIC VOCAB MAP (from `learning-path/gradual-success-typing.ai-pointers.md`, already curated —
the research agent should VALIDATE + deepen, not re-gather):
- Dorc's static layer = **gradual effect system**; ⊤/Opaque = `Dyn`/`?` + **consistency** (Siek & Taha).
- no-cliff = the **gradual guarantee** (Siek/Vitousek/Cimini/Boyland).
- warn-don't-reject = **success typings** (Lindahl & Sagonas / Dialyzer).
- oracle-as-no-runtime-effect + library social model = **pluggable types** (Bracha) + DefinitelyTyped/typeshed.
- static-derive + runtime-backstop = **soft typing** (Cartwright & Fagan).
- effect-map = **effect systems** (Lucassen & Gifford; Koka).
- guard-lifting (`[ -f X ] && …`) = **occurrence typing** (Tobin-Hochstadt & Felleisen; Typed Racket).
- **SYNONYM / entity-identity / co-reference = a SEPARATE literature** (abstract string domains;
  equivalence/**ontology alignment**; `owl:sameAs`; record linkage) — explicitly flagged as the
  deferred hard-corner, NOT the gradual-encoding question. THE one genuinely-unresearched cell,
  and where the research agent should dig hardest.
- barrier / re-observe-after-interference = **OCC** + **incremental recomputation** (build systems
  `075`; Mokhov verifying-trace rebuilder; salsa/Adapton) — relevant to the PERFORMANCE product
  (#11), NOT the attention product (#19), per the consent-wall kill. Research it for #11, not #19.
- measured-evidence-ratified-static (alt5) = **profile-guided optimization** shape + the
  record-linkage/entity-resolution literature (for turning traces into ratified entity-claims).

## Open, remaining (agenda)
- **derived-footprint = a probe-time ENTITY-SET** (unlocked by entity-granular poisoning — the
  derivation need only emit touched *entities*, not a property-map, and can't lie by omission):
  what a tool is asked at probe time to yield its entity-set; how it's spelled (footprint meets
  `predict()`); how it ships + is consumed in disjointness; its own horizon (residue still
  professed). NEXT.
- licensing tier + cross-site blast/attribution (a wrong footprint deletes SOMEONE ELSE's
  command — the permanently-sharp-knife tier; `23J` lane-privilege lives near here).
- entity-identity spelling + the measurement-over-names crack (ques4-adjacent).
- spelling (strawman-tier, LAST).
- then: adversarial crosscheck of the whole package before any weld.
