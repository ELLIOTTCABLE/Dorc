# Gap-check: does the survival why-chain distinguish resolver-claimed disjointness from name-floor disjointness?

Scoped question (`notes/300` §5, `rul-reference-entity-name-floor`'s discipline-close check item, ~line 178): when a survival elision's why-chain renders, does the machinery distinguish the two sources of entity-disjointness — (a) a kind-resolver's claim (`kind__resolve()` canonicalization) vs (b) the resolver-less name-comparison floor (plain string inequality) — with DIFFERENT typed `SpeechAct` provenance, or do both collapse into one undifferentiated verdict?

## Answer: NOT distinguished at the rendered why-chain. Partially tracked one layer down, but that tracking never reaches the `SpeechAct`-typed render, and is itself coarser than the question assumes.

+SURE on every claim below; each is a direct code read, not inference.

## The chain, chokepoint to render

**1. `core::coord::Relation` carries no provenance field at all** — `spike/crates/core/src/coord.rs:155-178`. `Relation::ProvablyDisjoint` is a bare unit variant. `compare()` (`coord.rs:306-347`) returns it from THREE independent branches with zero distinguishing data left behind:
   - cross-kind fence, `coord.rs:325-327`
   - distinct canonical entity within a kind, `coord.rs:335-338` — this is where (a) vs (b) lives conceptually, since `EntityResolution::Canonical(EntityRef)` (`coord.rs:141-148`) is documented to hold either "the resolver produced a clean canonical entity" OR "the kind is resolver-less ⇒ identity" — **the same variant, same shape, for both.** The chokepoint itself cannot tell a caller which one it got.
   - dialect-sparing (selector-level), `coord.rs:342-346`

   **This is the earliest point the (a)/(b) distinction dies** — it never existed past `compare()`'s return value in the first place.

**2. `plan::survival` partially RECONSTRUCTS an approximation, outside `compare()`.** `Crossing::via_resolver: Option<KindId>` (`spike/crates/plan/src/survival.rs:731-735`, accessor at `:789-796`) is computed once per `wall_verdict()` call:
   ```
   let via_resolver = resolutions.has_resolver(backing.coord.kind).then_some(backing.coord.kind);
   ```
   (`survival.rs:1002-1004`, then threaded into every `Crossing` built in that call, `survival.rs:1018`).

   This is a coarse proxy, not a faithful per-comparison attribution: it asks only "is the BACKING's kind resolver-bearing at all", never which of `compare()`'s three branches actually produced ProvablyDisjoint for *this* footprint×member pair. Concretely it can mislabel:
   - a cross-kind-fence disjointness (kind-fence fires *before* canonicalization ever runs, `coord.rs:322-324`'s own doc comment) as "via resolver", whenever the backing's kind happens to carry a resolver for unrelated reasons — the loom fixture below is the *resolver-less* instance of exactly this shape (Package vs Firewall, different kinds).
   - a dialect-selector-sparing disjointness (same canonical entity, different minted selector) as "via resolver", since it never checks which branch fired, only backing-kind resolver registration.

   So even where the distinction is tracked, it isn't the precise (a)-vs-(b) split the ledger's check item names — it's "was any resolver registered for this kind", asked once per witness, not "did a resolver's claim decide this particular verdict".

**3. `via_resolver` is consumed in exactly three places, none of them the pull-surface why-chain:**
   - `plan::survival`'s own tests (`survival.rs:1253,1462,1567`)
   - `dorc-sweep`, the internal DST/differential-fuzzing harness (`spike/crates/sweep/src/lib.rs:265` — feeds a `CrossedWall.resolver: Option<String>` record used for fingerprinting/differential comparison, never shown to a user; also folded into a determinism-fingerprint string at `sweep/src/lib.rs:316-318`)
   - `cli/src/main.rs::emit_survival_attribution` (`main.rs:2976-3029`) — the **stderr compact-attribution line**, explicitly the OTHER surface (per the doc comment at `main.rs:2966-2975`, this is "Stage 2 attribution" on the plan-time stderr lane, distinct from the `dorc why` pull surface). Here it IS textually surfaced: `via_resolver.map_or_else(String::new, |k| format!("; disjoint AFTER {k}.resolve() canonicalization"))` (`main.rs:3010-3015`) — present when Some, silently absent when None. But this is a hand-rolled `format!` string built directly in `main.rs`, **not routed through `dorc-aid`'s catalog/arrangement/`SpeechAct` machinery at all** — it wears no typed tier, it is bare uncatalogued prose on a secondary diagnostic surface.

**4. `cli/src/why.rs::survival_chain` — the actual pull-surface why-chain (`dorc why <addr>`) — never reads `via_resolver`.** Confirmed by grep: the string `via_resolver` does not appear anywhere in `spike/crates/cli/src/why.rs`. The single `derives` `ChainLink` for a survived crossing is built once, outside the per-crossing loop:
   ```rust
   let derivation = ChainLink {
       tier: SpeechAct::Derived,
       speaker: Some(ENGINE_SPEAKER.to_owned()),
       payload: Said::words("why-derives-payload-disjoint", &[&backing]),
       quoted: false,
       event: None, explanation: None, excerpt: None,
   };
   ```
   (`why.rs:705-713`). It is unconditionally `SpeechAct::Derived`, and its payload template (`spike/crates/aid/src/arrangement_lock.rs:400-406`) has exactly ONE interpolation hole (`backing`):
   > "that claim is proven disjoint from {backing} -- it does not overlap what was reported"

   No branch, no resolver name, no second `SpeechAct` variant. **Whatever produced the ProvablyDisjoint verdict — kind-fence, resolver-canonicalized distinct entity, resolver-less name-floor distinct entity, or dialect-sparing — the rendered why-chain emits byte-identical prose, tagged the same tier.** Confirmed against the live golden transcript `spike/crates/aid/tests/why-claims-payload.loom:127-129` (a cross-kind Package-vs-Firewall case, resolver-less): the `derives` row reads exactly the generic sentence above, with no resolver mention — consistent with, but not distinguishable from, what a resolver-bearing case would also render.

## Where the distinction dies

Two separate "deaths", worth keeping apart for the ledger entry:

- **Structural death** (irrecoverable without a `Relation`/`compare()` signature change): at `core::coord::Relation::ProvablyDisjoint`, `coord.rs:155-178` — the chokepoint itself never captures which of its three internal branches fired. Nothing downstream can recover MORE than survival.rs's coarse per-backing-kind reconstruction, because the fine-grained fact isn't there to recover.
- **Render death** (recoverable without touching the chokepoint — pure wiring gap): at `cli/src/why.rs:705-713` / `aid/src/arrangement_lock.rs:400-406` — even the coarse `Crossing::via_resolver` that DOES exist by the time a `SurvivalWitness` is built is simply never read by the `dorc why` builder. The data survives into the witness (survival.rs) and dies one hop later, at the render call site, not for lack of a carrier but for lack of a read.

## Confidence markers

- +SURE: `Relation::ProvablyDisjoint` carries no fields (coord.rs read in full, 866 lines).
- +SURE: `Crossing::via_resolver` exists, is `Option<KindId>`, and is computed via the coarse per-backing-kind proxy shown above (survival.rs read in full for the relevant sections, 1846-line file).
- +SURE: `cli/src/why.rs`'s `survival_chain` never references `via_resolver` (grep over the whole file for the literal string, zero hits; the `derivation` ChainLink construction read directly).
- +SURE: the `why-derives-payload-disjoint` arrangement entry has exactly one interpolation hole (arrangement_lock.rs read directly).
- +SURE: `via_resolver`'s only other consumers are `plan::survival` tests, `dorc-sweep` (internal DST harness, confirmed via its own module doc comment), and `main.rs`'s stderr compact-attribution line (hand-rolled, uncatalogued, not `SpeechAct`-typed).
- ~SUSPECT: I did not exhaustively search for every possible render surface (e.g. any JSON/machine-readable plan-output lane) beyond stderr/why/sweep; the three consumer sites found via full-repo grep for the literal `via_resolver` identifier make a fourth surface unlikely, but I did not verify there is no serialization path for `Survival`/`CrossedWall` reaching some other output format.
- ~SUSPECT: whether the stderr compact-attribution line (`main.rs`) is itself considered in-scope for `trust-tier-is-syntax` compliance, or is an intentionally-exempted lower-tier surface (its own doc comment implies it's deliberately a lossy shadow of the why-chain, "throws the attribution away"). I report the fact of its ad hoc, untyped construction without asserting it's a law violation — that adjudication belongs to whoever ledgers this.

No recommendations, no edits made. This finding is reported for the conductor to ledger as an aid-plane gap.
