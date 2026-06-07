# 172 — K1b: the cross-oracle kind-channel — adjacent-field prior-art gather (round 17, 2026-06-07)

> Firewalled K1, **REFOCUSED** (human steer, 2026-06-07): packaging dropped (cross-*manager* equivalence
> was the rabbit-hole). The crux is the **cross-oracle channel** — two oracles authored *independently*
> (A, B) converge on one **opaque kind-handle + its state sub-data** so a fact B *probes* discharges a
> precondition A *declares*, with Dorc routing it and understanding **none** of the semantics. Concretely:
> A = "non-mutative *provided your wombat is defrocked*"; B = "I can check defrocked / frocked / wet."
>
> **Steer:** stay type-systems/PLT **light**; hunt **ergonomic, NON-type-systems** prior-art in **adjacent
> fields** for *"agree on + communicate discovered-and-compared/checked identifiers carrying SOME
> dimensionality/sub-data — how much = TBD."* Ideally sh-spellings; cross-field findings welcome.
>
> Eventual synthesis → `plans/175` (integrates `notes/171` + this; folds in the prior packaging map). Sources →
> `../sources.json`. Raw graded findings; AI-generated; confidence-marked; trust repo-root docs over this.

## Findings (lifted, most-load-bearing first)

- **k1b-id-XOR-match (+SURE) — the central cross-cutting result.** Every durable, ergonomic scheme
  **separates the identifier from the match/compare rule, and lets the *consumer* pick the match-depth.**
  BCP-47 *tag* vs RFC-4647 language-*range* (graded fallback) [B-w3c-language-tags-2024]; InChI full-id vs
  layer-match [C-inchi-wikipedia-2026]; semver *version* vs *precedence* (which ignores `+build`)
  [B-semver-spec-2013]; Pact *contract-by-example* vs schema [B-pact-cdc-docs-2022]. → For Dorc: oracle B
  emits a kind-handle (carrying its sub-data); oracle A's precondition is a *pattern/range* that matches at
  the depth A needs; **Dorc routes by matching, consumer(A)-driven**, never needing a full kind-schema.

- **k1b-minimal-extensible (+SURE) — answers "how much sub-data: TBD".** The ergonomic answer is *not* a
  fixed schema; it is **carry the least that distinguishes, extend on demand.** BCP-47's golden rule
  verbatim: *"keep the tag as short as possible; avoid subtags except where they add useful distinguishing
  information"* (`ja`, not `ja-JP`) [B-w3c-language-tags-2024]; InChI: *"not all layers have to be
  provided… omit if not relevant to the application"* [C-inchi-wikipedia-2026]; Pact: only the parts the
  consumer actually uses are in the contract [B-pact-cdc-docs-2022]. So `dq-entity-algebra` should start
  **flat + optional layers**, never a mandatory structured kind.

- **k1b-revdns (+SURE) — the X3 clobber solved ergonomically, and the strongest sh-native candidate.**
  Reverse-DNS roots a globally-unique handle in the **existing DNS namespace** instead of inventing a
  registry: *"rather than creating a new central database of global names, the domain name registry is
  used… globally unique to its owner"* [C-reverse-dns-notation-wikipedia-2026]. **Universal cross-field
  convergence** — Java, Apple UTI, Android, dconf, D-Bus, freedesktop Desktop Entry, Flatpak, iSCSI IQN,
  AT-proto Lexicons. A reverse-DNS string is **decentralized, collision-free, and already a plain
  string** → an oracle can spell `kind=net.example.wombat` as a lifted sh datum (the `dq-kOOB`
  declarative-datum-to-index from `notes/171`), with **zero Dorc-internal registry**.

- **k1b-coherence-forced (+SURE) — the InChI smoking gun: cross-party matching forces *some* agreed
  shape.** InChI is *computed, not assigned* (decentralized), yet when independent groups needed their
  ids to **match**, optional-per-party layers failed — group A's id (with stereo) ≠ group B's (without) —
  so they defined a **"standard InChI" with a fixed layer-set** [C-inchi-wikipedia-2026]. This is `095
  f28` coherence made real by a working community: **per-party dimensionality does not compose; the
  *compared* dimensions need a thin agreed standard.** Bounds the freedom of k1b-minimal-extensible — A
  and B may carry private extra sub-data, but the dimensions they *match on* must be coherent.

- **k1b-self-describing-floor (+SURE) — bake the kind in, but you can't escape ≥1 out-of-band anchor.**
  Multihash bakes a type-tag into the value so consumers stop hardcoding assumptions (*"how many scripts
  assume a git hash is sha1 / 160-bit?"* → read it from the value instead) [B-multihash-multiformats-2024];
  Apple's **dynamic UTIs** losslessly encode discovered sub-data (a `frob` extension) *inside* the handle
  so an unknown kind round-trips with **no registry** [B-houghton-utis-2012]. But the honest caveat is
  load-bearing: the multihash critique — *you still must know out-of-band that a blob IS a multihash* —
  and InChIKey's *known hash collisions* both confirm Dorc's `094 g4/g5` floor: **decentralize everything
  except the last bit (that this field IS a kind-handle).** Self-description reduces, never removes, the
  anchor.

- **k1b-carry-vs-compare (+SURE) — a handle may carry sub-data it does NOT match on.** semver's `+build`
  is *communicated but ignored for precedence* [B-semver-spec-2013]; UTI *tags* (extension, MIME, OSType)
  ride along and are queryable but are not the identity [B-houghton-utis-2012]. → the kind-handle can
  carry **provenance / version / extra sub-data** that flows A↔B without affecting the match; only the
  *state dimension* A's precondition names ("defrocked") participates in the discharge.

- **k1b-consumer-driven-channel (~SUSPECT the best direct analogue) — Pact is the channel shape.**
  Consumer-driven contract testing: *"the consumer defines what it needs from the provider, and the
  provider verifies it can fulfill those needs — without either service needing to [integrate]"*
  [B-pact-cdc-docs-2022]. Maps onto **oracle A's precondition (the consumer's need) ↔ oracle B's probe
  (the provider's verification)**, mediated by a shared contract artifact (the "broker"). *Contract by
  example*, not schema, means A declares the *concrete* state it needs ("wombat defrocked"), not a full
  wombat type — and B's other behaviours stay free to change. The ergonomic minimal-coupling channel.

- **k1b-no-magic (~SUSPECT, the round's honest negative — as the human predicted).** Across all five
  fields there is **no escape from a shared anchor**; the state of the art *decentralizes down to a
  minimal one* (reverse-DNS = zero-coordination root; canonical computation = InChI; shared codec table =
  multihash) and makes everything above it **consumer-driven + minimal-but-extensible**. The least-bad
  cross-oracle spelling is therefore: a **reverse-DNS-rooted kind-handle (a lifted sh string) + optional
  state sub-tags + consumer-driven matching + a thin coherence standard for compared dimensions** — not a
  magic inference. Pure-book idiom cannot reach cross-script/cross-author on its own (confirms `094 g4`).

## The dimensionality menu — how each field answered "how much sub-data, and how is it compared?"
(The `dq-entity-algebra` axis, grounded in real communities with verdicts.)

| field | identifier shape | "how much" choice | match / compare rule | hindsight |
|---|---|---|---|---|
| MIME media types | `type/[tree.]subtype[+suffix][;params]` | fixed 2 levels + optional tree/suffix/params | exact + param negotiation | registration friction (IETF) + `x-`/`vnd.` tree mess; UTI was a reaction |
| Apple UTI [B-houghton-utis-2012] | **flat** reverse-DNS id + **external conformance DAG** + tags | flat id; structure lives in declared *conformance*, not the string | `conformsTo` graph query (multi-parent) | praised; the dynamic-UTI sub-data-in-handle is the standout |
| multihash [B-multihash-multiformats-2024] | TLV `<type><len><value>` | self-describing **type prefix** from a shared table | equality; type read from the value | great for upgrades; "still need OOB that it's a multihash" |
| semver [B-semver-spec-2013] | `maj.min.patch[-pre][+build]` | fixed small + a *carried-but-ignored* slot | precedence (ordered); `+build` excluded | near-universal; the carry-vs-compare split is the lesson |
| BCP-47 [B-w3c-language-tags-2024] | ordered **extensible** subtags | minimal; "shortest that distinguishes", add on demand | RFC-4647 **language-range** w/ graded fallback | durable; identifier ⊥ matching is the win |
| InChI [C-inchi-wikipedia-2026] | **optional progressive layers** | per-application; omit irrelevant layers | layer-match; **"standard InChI"** fixes a layer-set for cross-party | the coherence-forced-a-standard lesson |
| geohash | prefix string; **length = precision** | truncate=coarser, extend=finer | prefix-match = containment | clean precision-as-length; (not separately graded) |
| reverse-DNS [C-reverse-dns-notation-wikipedia-2026] | hierarchical labels, own-a-domain root | extensible hierarchy | prefix/segment scoping | universal; the decentralized-root answer to X3 |

## Citations (verbatim; [slug]:loc, cite-certainty)

> [B-houghton-utis-2012]:body (relevance: +1:SURE)
> If you want a new MIME type, you either have to register it with IETF, which is time-consuming … or you
> can stick an x- prefix on it (but this might create clashes). If you want a UTI, however, all you need
> do is own an Internet domain … you could use com.example.MyDataType.
> … UTIs … have a conformance hierarchy … public.jpeg conforms to public.image, which in turn conforms to
> public.data … APIs that allow you to test any given UTI for conformance with any other UTI.
> … [dynamic UTI] dyn.age80q6xtqk … the information you gave the system is still there. If you ask … for
> the file extension … it will immediately tell you: frob. You could transmit this UTI across a network,
> and it would still tell you … the extension is frob.

> [B-pact-cdc-docs-2022]:body (relevance: +1:SURE)
> the consumer defines what it needs from the provider, and the provider verifies it can fulfill those
> needs — without either service needing to deploy the world first.
> … Unlike a schema or specification (eg. OAS), which is a static artefact that describes all possible
> states of a resource, a Pact contract is enforced by executing a collection of test cases … "contract
> by example". … only parts of the communication that are actually used by the consumer(s) get tested …
> any provider behaviour not used by current consumers is free to change without breaking tests.

> [C-reverse-dns-notation-wikipedia-2026]:body (relevance: +1:SURE)
> Reverse-DNS names are a simple way of eliminating namespace collisions, since any registered domain name
> is globally unique to its owner. … [adopters:] Java … Apple's Uniform Type Identifier (UTI) … dconf …
> freedesktop.org Desktop Entry Specification and D-Bus Specification … Flatpak … iSCSI Qualified Naming …
> AT Protocol [Lexicons].

> [B-multihash-multiformats-2024]:body (relevance: +1:SURE)
> A multihash follows the TLV (type-length-value) pattern. <hash-func-type><digest-length><digest-value>.
> … How many programs out there assume a git hash is a sha1 hash? … Tooling … can avoid making assumptions
> about the length, and read it from the multihash value instead.
> └ HN critique (kagi-surfaced, not separately graded): "the fact that it's a multihash has to be inferred
>   from context or out-of-band just like before" — self-description does not bootstrap the anchor.

> [C-inchi-wikipedia-2026]:body (relevance: +1:SURE)
> describe chemical substances in terms of layers of information … Not all layers have to be provided; for
> instance, the tautomer layer can be omitted if that type of information is not relevant …
> … computed from structural information and do not have to be assigned by some organization …
> … "standard InChI", a version of the InChI with a fixed level of detail and collection of layers … will
> simplify comparison of InChI strings and keys generated by different groups.
> … The InChIKey … is not unique: though collisions are expected to be extremely rare, there are known
> collisions.

> [B-w3c-language-tags-2024]:Overview (relevance: +1:SURE)
> The golden rule when creating language tags is to keep the tag as short as possible. Avoid region,
> script or other subtags except where they add useful distinguishing information. For instance, use ja
> for Japanese and not ja-JP, unless there is a particular reason …
> … [RFC 5646 tag] … [matching is RFC 4647 "language-range"]. … xml:lang="" … I do not want to associate
> any language with this information.

> [B-semver-spec-2013]:§ Precedence (relevance: -0:SUSPECT)
> Precedence MUST be calculated by separating the version into major, minor, patch and pre-release
> identifiers in that order (Build metadata does not figure into precedence).

## Open / next (carry to plans/175 synthesis)
- **The emerging sh-spelling candidate:** an oracle-emitted **reverse-DNS-rooted kind-handle** (lifted sh
  datum → analyzer index, per `dq-kOOB`) + **optional state sub-tags** (InChI/BCP-47 minimal-extensible) +
  **consumer-driven matching** (Pact/RFC-4647: A's precondition matches at its needed depth) + a **thin
  coherence standard** for the *compared* dimensions (the InChI-standard lesson). Carry vs compare split
  (semver `+build`) lets the handle also carry provenance.
- **The honest negative to state plainly in the synthesis:** no field escapes ≥1 shared anchor; the win is
  *which* anchor is least-burdensome (reverse-DNS root = most sh-native, zero-coordination) — not magic.
- **Untaken/lighter leads (declared, not chased):** COM/CORBA IID & GUID interface identity (independent
  binary components agreeing on an interface — af4-adjacent); ISBN/Luhn **check-digits** (the "checked"
  facet, lightly covered by multihash/InChI recompute + Pact verification); HTTP `Accept;q=` content
  negotiation (af3-adjacent graded matching); RFC 6838 + Apple UTI-Concepts as the *canonical* primaries
  behind the MIME/UTI rows; the peer-reviewed InChI primary (Heller et al. 2015) to upgrade
  [C-inchi-wikipedia-2026] → A.
- **Firewall:** held PLT-light per steer — no occurrence-typing/qualifier framing pulled in; ergonomic
  practitioner prior-art only.
