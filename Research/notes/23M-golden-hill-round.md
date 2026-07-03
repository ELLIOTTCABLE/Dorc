# 23M — the golden-hill round (elide-past-a-running-command): live working note

AI-authored, 2026-07-03, round #19, IN PROGRESS (chat-driven, human remote). NOT stamped —
raw working material, confidence-marked. Nothing here is welded; the round settles by dialogue
→ (eventual) adversarial crosscheck → re-welds/pins. Seeds: `notes/238`, `plans/23D` §5, `23J`
lane-privilege, the human's item-3 golden-hill statement (2026-07-03 chat).

## THE ROUND'S SPINE (human-landing 2026-07-03; PROVISIONAL, unconfirmed against final reply): 233 is permanent — cage it, don't fix it

The week's churn taught the human (and it survives the conductor's check): **233 — grounding
soundness in a fallible completeness-claim is unsound — is a PERMANENT CONDITION of
eliding-past-a-running-command, not a fixable bug.** Sound past-a-wall elision requires SOMEONE
promising completeness over SOME vocabulary; every such promise is human and fallible; no design
removes the need for it. Accept-and-design-*around* is the only honest posture. "Design around"
has a precise, three-move shape, and it IS what this round achieved:
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
file-coordinates) — coordinate translation, never kind-equivalence.

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

## THE DANGEROUS OPEN CELL — synonym/coherence (fails toward UNDER-execute; next up)

Every other gap in this design fails SAFE (missing/ungrounded/horizon-exceeded → wall → guard →
over-verify). The one that fails UNSAFE: two honest oracle-authors using DIFFERENT names for the
SAME referent (synonyms). Disjointness intersects coordinates; synonyms make the intersection
come up EMPTY when it should HIT ⇒ false-disjoint ⇒ elide-when-shouldn't ⇒ under-execute (the
cardinal sin). This is 233's "silence licenses nothing" one layer up (`23D` §5: "no shared name
is 233's silence one layer up"), the SYNONYM dual of round-17's homonym problem. Proposed answer
(mainline candidate, `23D` §5): the namespace-ownership convention — reverse-DNS kinds have
owners; an owner honestly guarantees no-synonyms WITHIN their namespace; disjointness concludes
only WITHIN one namespace, never across; Dorc owns only `sm.dorc.*` (bootstrap vocab, adopted by
gravity, no registry/arbiter). Plus the entity-aliasing fence (within-kind identity ≠ string
compare — symlinks/mounts/normalization; the kind-owner pins entity-identity semantics). NOT yet
worked in dialogue — this is the next thread.

## Open, remaining (agenda)
- synonym/coherence + namespace-ownership + entity-aliasing (THE dangerous cell — next).
- the derived-footprint PROTOCOL: how a payload-bound oracle emits its per-run footprint at
  probe time, and how it ships/is-consumed (touched, not worked).
- licensing tier + cross-site blast/attribution (a wrong footprint deletes SOMEONE ELSE's
  command — the permanently-sharp-knife tier; `23J` lane-privilege lives near here).
- spelling (strawman-tier, LAST).
- then: adversarial crosscheck of the whole package before any weld.
