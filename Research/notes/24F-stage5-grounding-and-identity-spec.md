# 24F — Stage 5 spec: grounding + identity (the resid-aliasing closure) — conductor spec

AI-authored (Opus conductor), 2026-07-04, round 24. The type-contract + design SPEC for Stage 5,
authored per `rul24-overtype`. Companion to `24E` (Stage 4); the charter is `plans/240` Stage 5;
the mechanism's terminology + landmines are `23M` (contribution-vs-identity, the two bridge
senses, the dangerous cell). Per the 240 boundary, the SPELLINGS herein are strawman-tier
(build-to-learn, explicitly disposable); the TYPE-CONTRACTS and safety-postures are the spec.
Confidence-marked.

## §1 What Stage 5 is for — the one dangerous cell

Every other gap in the survival tier fails SAFE (missing/ungrounded/⊤ → wall → run/guard →
over-verify). The one that fails UNSAFE is **within-kind aliasing** (`resid-aliasing`, 24C —
PRIMARY): `disjoint` is token-equality on interned `(kind, entity)`, so two tokens naming the
SAME real referent (`nginx`/`nginx-full` via provides; `/etc/nginx` vs a symlinked path) come up
wrongly-disjoint ⇒ wrong-survival ⇒ **silent under-execute** — the cardinal sin, live today
behind `--trust-footprints`, disclosed-but-unmechanized. Stage 5 closes the mechanism. (The
horizon *wording* — the human-voice profession of what residue remains — is a parked human
thread that becomes writable once this lands; `aliasing-horizon-wording`.)

## §2 The scoping insight — the expansion bridge is (mostly) already built

23M names two bridge senses. Checking them against the Stage-4 machinery:

- **The footprint-expansion bridge** ("touching `package:X` REACHES `file:Y`/`service:X`" —
  directional, at-most-broadening) **collapses into Stage-4 derivation.** A `touches()` body
  already emits whatever cross-kind coords its own sh computes (the `dpkg -L | sed` idiom emits
  `file:` lines; §8-as-built: "body computes and emits"). And because a footprint is an
  **at-most claim, over-emission is always SAFE** — apt's touches() unconditionally emitting
  `service:%s` for install (the postinst-enables-service edge, 23M's cross-kind escape) is
  conservative: it costs elision-value (service sites won't survive apt walls), never
  correctness. The VALUE refinement — emit `service:` only when a unit file actually ships —
  is just a richer derivation body (`dpkg -L "$1" | grep -q '\.service$' && printf …`), again
  Stage-4 machinery. **Consequence: Stage 5 builds NO new bridge mechanism for the directional
  case** — it ships richer strawman derivation bodies exercising it, and measures the yardstick.
- **What does NOT collapse: IDENTITY.** Token-equality is wrong in two irreducible ways:
  within-kind synonyms (aliasing, §3) and cross-namespace sameness (co-reference, §5). Those
  are Stage 5's genuinely-new machinery.

+SURE on the safety direction of the collapse (at-most over-emission is monotone-safe);
~SUSPECT on its completeness (a bridge case that CANNOT be spelled as body-emission may
surface in the build — if so, that strain is the deliverable, record it).

## §3 The aliasing closure — owner-declared canonicalization (dynamic points-to)

The literature frame (23M/23N): must-not-alias-or-wall; test disjointness over **resolved
referents, not names** — the "measurement crack" = dynamic points-to. The kind-OWNER holds
entity-identity (23M contribution-vs-identity: authority over the *nouns*); so the mechanism:

- A kind-owner may ship a **canonicalizer** for its kind — the identity role-sibling. Strawman
  spelling (disposable): `<kind-owner>.resolve()`, invoked per-coordinate at probe time with the
  entity text, printing the canonical form on stdout. Examples: the fs oracle's resolver is
  ~`realpath -m -- "$1"`; the package kind's is
  ~`dpkg-query -W -f '${Package}\n' -- "$1" 2>/dev/null || printf '%s\n' "$1"` (resolves a
  provides/virtual name to the real installed package; falls back to the name itself).
- **The engine canonicalizes BOTH sides before intersection** — every footprint coord AND every
  backing coord in a resolver-bearing kind passes through the kind's resolver; `disjoint`
  compares canonical forms. Two names for one referent now HIT (the under-execute closes);
  distinct referents stay disjoint (the value survives).
- **Per-kind gradual enhancement, honest residue:** a kind with NO resolver keeps today's
  token-equality + the professed residue (the status quo is the floor, not an error). Resolver
  coverage buys aliasing-safety kind-by-kind — exactly the you-get-what-you-put-in curve.
- **Probe-lane surface: same inertness story as Stage 4, deliberately.** A resolver is a
  host-run read-only body — the SAME structural self-vouch (authoring it IS the vouch), the same
  rc-127 mocks net, the same one-flag-wide professed caveat, the same per-site readback pattern
  (a `resolv <leafid> canon=<kind:entity>=<canonical>`-shaped record lane riding the Stage-4
  demux transport). NO new trust machinery; the fourth role-sibling rides the third's rails.

### §3a Failure-direction analysis (the four-by-two check, spelled out)

- **Resolver ⊤ / can't-resolve / non-zero rc / malformed output** for a coordinate: the pair's
  comparison degrades to **MAY-ALIAS ⇒ hit ⇒ demote** (fail toward run), NOT to token-equality.
  Rationale: once an owner has declared "identity in this kind needs resolution," an unresolved
  coordinate is suspect by the owner's own testimony. (Contrast the NO-resolver kind, where
  token-equality stays the honest floor — the owner never claimed better.) -GUESS this is the
  right default; the alternative (degrade to token-equality) trades safety for value — builder
  should implement may-alias and the strawman family should measure how often the degrade fires;
  if it swamps the yardstick, that is a finding, not a license to flip the default silently.
- **A WRONG resolver** (canonicalizes two distinct referents together ⇒ over-hit ⇒ over-verify:
  safe; canonicalizes one referent apart — returns different canons for two names of one thing ⇒
  the under-execute REOPENS). The second is the sharp edge: the resolver sits on the SAME
  sharp-knife tier as the footprint (a wrong one silently under-executes someone else's line),
  and gets the same treatment — attributed by name in the why-lens, opt-in, and a **lying-resolver
  axis in the sweep** (§7).
- **Reverse direction:** kill-wall coherence comparands canonicalize identically (same
  intersection machinery). **Other phase:** resolution is probe-lane; apply consumes canonical
  coords via the license — no apply-time resolution. **Other user:** the admin never sees any of
  this; the engineer pays one resolver per kind (DX cost bounded, owner-scoped). **Unreliable
  oracle:** the lying-resolver sweep axis is the net.

## §4 Dangling-reference detection (23M's bonus defense — cheap, do it)

For an ENUMERABLE kind, a reference to a non-existent entity (`package:nginx-http`, no such
package) is mechanically detectable at probe time: the resolver's natural failure
(`dpkg-query -W` non-zero) IS the detection. Surface it as a loud per-coordinate diagnostic
(`dangling-reference`) + the §3a may-alias degrade. This turns the third-party-typo case from
silent-value-loss into a pointed hint. Rides the resolver lane for free.

## §5 Co-reference (cross-namespace sameness) — SEED ONLY, scope-gated

The symmetric bridge ("vendor.Pkg:nginx ≡ apt.Package:nginx", two owners, deliberate
cross-namespace synonym; 23M) is the OPT-IN grounding act reaching for the trusted claim. It is
the least trial-critical piece (the r25 homelab book is single-vendor) and the most
contract-shaped (whose declaration wins, staleness, the 233 shape one storey up). Stage 5 builds
AT MOST a strawman seed: one fixture where a second oracle's kind co-references the package kind
through an explicit declaration, consumed as canonicalize-into-the-target-kind — reusing §3's
machinery, adding no new trust type. If it strains, STOP and record; the full co-reference
contract is post-trial design. ~SUSPECT even the seed may be cuttable if time presses — it is
the first thing allowed to give (the aliasing closure is not).

## §6 Type-shapes (conductor domain; lighter than 24D/24E)

- **`CanonicalCoord`** — a newtype the intersection consumes; minted ONLY by the engine's
  resolution step (private ctor). `disjoint` re-signs to compare canonical coords, so a raw
  interned coord CANNOT reach the intersection in a resolver-bearing kind by construction
  (the compile-error family, TC-style). For resolver-less kinds the mint is the identity
  (token = canon, explicit, one arm).
- **Resolution outcome enum** — `{Canonical(coord), MayAlias(reason)}`; `MayAlias` flows to
  demote (§3a). NO boolean.
- **Attribution** — the `SurvivalWitness`/`Crossing` carries which resolver(s) canonicalized the
  compared coords (the why-lens: "…survives: disjoint AFTER apt-get.resolve() canonicalization").
- **Provenance tag parity** — resolver output is `Derived`-provenance by construction (host-run);
  reuse `FootprintOrigin`-style tagging, don't invent a parallel.

## §7 Testing (the nets, in priority order)

1. **The lying-resolver sweep axis (the soundness net — MUST land with the mechanism, not
   after).** Extend the Stage-4 declared-vs-true model: the generator invents a TRUE identity
   (two names → one referent in `CellDelta` terms) and a DECLARED resolver answer; honest ⇒
   resolver merges them; lying ⇒ resolver keeps them apart ⇒ wrong-survival ⇒ end-state RED,
   attributed. Assert the lying branch is non-vacuous (fc-5). This is the direct analogue of
   `derived_lying_divergences` and closes the same class for identity.
2. **e2e strawman family:** `strawman24-alias-*` — (a) the provides case: a book installs
   `nginx`, a downstream fact's backing names `nginx-full`, the resolver merges ⇒ correctly
   DEMOTES (no survival) where token-equality would wrongly survive; (b) the symlink case
   (fs kind, realpath); (c) a dangling-reference case (loud diagnostic, may-alias degrade);
   (d) the richer-derivation service-edge case (§2: apt emits `service:` only when a unit
   ships — the systemctl site survives a no-service install, walls on a with-service install).
   Differential-verified throughout.
3. **Unit:** the `CanonicalCoord` mint unrepresentability; the §3a degrade direction.

## §8 What Stage 5 must NOT do (scope fence)

No full co-reference contract (§5 seed at most; first-to-give). No cross-oracle contribution
story beyond what the fixtures exercise (the scan_cve narrative is Stage-6 measurement
material). No new trust tier or flag (everything rides `--trust-footprints`; a resolver is
probe-lane self-vouched like its siblings). No engine-side kind-crossing (§2: bridges stay
body-emitted; the engine interns, canonicalizes, intersects — never infers an edge). No
closure-check (still deferred). No TOCTOU/freshness machinery (standing WONTFIX). Do not
relitigate settled law.

## §9 Confidence + the open leans

+SURE: the dangerous cell + its direction (§1); at-most over-emission is safe (§2); the
canonicalize-both-sides-before-intersection shape; the lying-resolver net requirement; the
same-inertness-tier argument (§3, rides Stage 4's rails). ~SUSPECT: the §2 collapse is complete
(watch for an unspellable bridge case); the resolver-as-fourth-sibling spelling (strawman —
build-to-learn); the co-reference seed survives scope pressure. -GUESS: the §3a may-alias
default for resolver-⊤ (measure the fire-rate; a swamped yardstick is a finding to surface, not
silently flip). OPEN (human, eventually, non-blocking): the aliasing-horizon WORDING (theirs,
post-mechanism); the co-reference contract proper (post-trial).

## §10 — human-caught corrections (2026-07-04, post-dispatch; BINDING; supersede §2/§3 where they conflict)

**corr-§2-collapse-is-half-wrong (the human's catch).** §2's "Stage 5 builds NO new bridge
mechanism" holds ONLY for the own-oracle case (a tool's touches() emitting cross-kind coords for
its OWN sites — legitimately Stage-4 machinery, and over-emission stays monotone-safe). It is
WRONG for the cross-author case, which is what makes the bridge a bridge: `23M` designed the
expansion bridge as an **owner-spelled, types-to-types translation function** (`manifest() {
dpkg -L "$1" ;}`), declared ONCE per kind and **engine-applied to every footprint coordinate of
that kind, whoever emitted it**; `ORACLE_PROVIDES` reserves it as a *vocabulary-relation*. The
load-bearing case body-emission cannot cover: a third-party oracle honestly emits
`package:nginx` in its touches() — the package's file-manifest is APT'S knowledge, not theirs —
so without engine-applied expansion, that footprint under-covers and a downstream `file:` fact
wrongly survives (the 233 CONCENTRATE-move demands the reach-knowledge live once, with the
owner). No LIVE unsoundness at HEAD (no cross-author package-coordinate emitters in the corpus),
but the mechanism gap is real. **Disposition: Stage-5 Part B** (engine-applied expansion,
footprints-only — backings stay self-framed narrow; single-step expansion, no fixpoint, for the
spike; its own lying-manifest sweep axis — omission is the sharp edge, same tier as footprints).
Part A (aliasing, in flight) is unaffected.

**corr-kind-keying (conductor self-catch while walking the UI).** §3's strawman
`apt-get.resolve()` is MIS-KEYED. The identity/reach functions are not role-siblings of the
COMMAND-oracle trio — they are a second family keyed by the KIND (23M contribution-vs-identity:
the owner holds the nouns): `package.resolve()`, `package.manifest()`, invoked with an ENTITY,
not an argv. A command-keyed resolver would mint identity for a "kind" no coordinate uses and
the closure would silently never fire. Enforce: at-most-one-resolver-per-kind; a conflict, or a
resolver keyed to a known PROVIDER name, is a LOUD diagnostic (make the confusion a warning,
never a silent dud). The authored surface is thus two families: the per-tool trio
(predict/is_converged/touches — every oracle author) and the per-kind pair (resolve/manifest —
kind-owners only, stdlib-concentrated); all five optional, each silence degrading to the
existing floor (the monotonic contract).

**corr-alt6-disposition.** `236b`-alt6 (property invalidation-bases — the extender declares what
kills its contributed property, in the kind's core vocabulary; default = killed-by-any-entity-
touch) is VALUE-tier and stays deferred: entity-granular poisoning already carries its safety
case; alt6 buys selector-granular survival precision the current entity-granular machinery
cannot host yet. Ledgered, not built.
