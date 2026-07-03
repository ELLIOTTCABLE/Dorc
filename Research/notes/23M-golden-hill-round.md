# 23M — the golden-hill round (elide-past-a-running-command): live working note

AI-authored, 2026-07-03, round #19, IN PROGRESS (chat-driven, human remote). NOT stamped —
raw working material, confidence-marked. Nothing here is welded; the round settles by dialogue
→ (eventual) adversarial crosscheck → re-welds/pins. Seeds: `notes/238`, `plans/23D` §5, `23J`
lane-privilege, the human's item-3 golden-hill statement (2026-07-03 chat).

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
