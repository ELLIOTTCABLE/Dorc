# 277 — The entity algebra: coordinate, selector dialect, comparison chokepoint, grammar

AI-authored (Fable conductor, 2026-07-12) — the task-12 design note owed by
`24C:rul24-selector-pre-stdlib` and gating `270:block-rebuild` dispatch
(`270:adj-entity-algebra`). This note ASSEMBLES the typed rulings of the block-settle
sittings into one buildable spec, plus the residual exact grammar — settled through
the 2026-07-12 delta pass (the ack record lives in `plans/271`'s task-12 entries;
this document states rules plainly and keeps the ceremony there). Per-item
disposition: §8. Authority:
root docs and `plans/271` outrank this; on conflict, the cited ruling wins. Companions:
`notes/272` (kind-side topology) · `notes/273` (wrapper surface) · `notes/274`
(eval'er/reentry) · `notes/275` (value-predictions) · `plans/24S` (proposal-tier
keystone; its §0 impossibility ledger stands) · `plans/17N` §4 + `notes/24F` (the
identity-spec lineage this supersedes as coordinate shape).

## §0 — What this note is for

Four consumers, in dispatch order:

1. **The corpus-respell brief** (`270:block-rebuild` step 1): §4 is the mark grammar
   the churn rewrites into; §7a lists the specimen amendments exhibiting it.
2. **The entity-algebra-rebuild brief** (step 4): §§1–3 are the spec; §5 the seams it
   must reserve; §6 the fences it must pin as tests.
3. **The value-recipe-reshape brief** (step 5): §5's backing/re-bind seams (carried
   jointly with `271:rider-value-recipe-reshape-capture-seams`).
4. **The human's delta pass**: §8.

The two seam reservations owed to `plans/270` §2 (né `24S:A7`) — the
**context-qualifier slot** in the coordinate representation, and the
**kind-topology room** in the kind-owner surface — are DISCHARGED by construction in
§1 and §5.

## §1 — The coordinate

*(status: typed — `271`'s "entity-algebra spelling direction" + "entity-algebra
seams"; representation obligations conductor-derived from them)*

Vocabulary, used precisely throughout:

- **coordinate** — the NAME: the flat triple `(kind, entity, selector)`, plus the
  context slot. Syntax; a way of writing an address.
- **selector** — the third position: names a sub-entity aspect; the same token may
  recur across coordinates (`#enabled` on many services).
- **cell** — the state location a coordinate DENOTES: the referent. Two coordinates
  may name ONE cell — that is aliasing, and `kind__resolve()`/canonicalization exist
  precisely because names ≠ referents. Where this note says compare/spare/collide,
  the objects are cells, reached through their coordinates.
- **context** — the world-qualifier in the slot: the same computed name in two
  contexts may denote two cells (keying) or one (invariance-bridged).

The coordinate, everywhere it appears (facts, backings, footprints, disjointness,
probe keying), is the **flat three-place `(kind, entity, selector)`**, carried in a
representation that also holds a **context slot**. The slot's default value is the
host-default world — its NAME is deliberately unminted here (~SUSPECT it ends up
host-qualified, `<hostname>`-root-ish, when the multi-host round lands; nothing in
this note pre-designs that naming).

- Kind: a reverse-DNS dotted name (`24M:rul24M-reverse-dns-kinds` — mandatory, ≥2
  dots; stdlib `sm.dorc.TypeName`; munges into NAMEs for kind-keyed members).
- Entity: a name, bound by binds or resolved from argv/value-flow. Binds name
  ENTITIES, never cells (`271:rul-binds-entity-only-provisional` — typed SOFT, door
  open on counterexample).
- Selector: an opaque interned token naming a cell of the entity. The bare
  selector-less form permanently means "true / occupied / whole-entity". At the
  claim/consumer side a selector-less coordinate is ⊤-selector (collides with every
  cell of the entity).
- Context slot: at representation tier a space-tag and a qualifier field are the SAME
  datum (`271:rul-seam-context-slot-and-relational-chokepoint`) — the fork dissolves;
  default = the host-default world (§1 top); populated by the wrapper machinery
  (`273`).

`plans/17N` §4's recursive-struct lean was DECLINED as coordinate shape
(`271:rul-coordinate-shape-flat-three-place`); its motivators are honored flat:

- The canonical case — a systemd unit's `#enabled` and `#active` are independently
  mutation-gating (`enable --now` writes both; `is-active`→true must not discharge an
  unmet `#enabled`) — is two CELLS: `sm.dorc.Service:"$svc"#enabled` and
  `…#active`. Distinct facts, distinct backings, distinct kill-traffic.
- Present-key-is-true / absent ≠ asserted-false (the carry-vs-compare split) is the
  fact plane's native semantics: a cell nobody has predicted-against is not a fact
  that is false;
  a selector-less claim collides conservatively rather than asserting depth.
- Match-only-the-depth-you-need is the dialect comparison (§3): claims narrower than
  the backing spare; claims at unknown depth collide.
- Deeper structure (17N's nested values, stringy handles) lives in the owner-declared
  function mechanisms BETWEEN flat coordinates — `kind__resolve()`,
  `kind__disturbance_reaches_only()`, `kind__state_stored_only_in()` — never in the
  coordinate. Rider (typed): later structural expansion must remain
  language-design-easy.

**Representation obligations on the rebuild** (each an engine hedge from the typed
riders, test-pinnable):

- `SelectorId` stays opaque/interned; every selector comparison lives behind ONE
  choke-point function (`selector_covers`-shaped). No caller compares tokens inline.
- ALL whole-coordinate comparison sits behind one choke-point that MAY answer
  relationally — per-axis pointwise decomposition is never baked into the API
  (`271:rul-seam-context-slot-and-relational-chokepoint`).
- `CanonicalCoord` continuity (`24C` Stage-5A, LANDED): the private mint survives the
  re-key; `kind__resolve()` canonicalizes the ENTITY component within its kind,
  exactly as landed; selectors do NOT canonicalize at v1 (interned tokens compared
  only through the chokepoint); `Resolution{Canonical, MayAlias}` and MayAlias⇒demote
  stand. A raw coordinate still cannot reach the intersection in a resolver-bearing
  kind.
- The kind-fence (cross-kind pairs short-circuit disjoint BEFORE canonicalization)
  stands at v1 — but the rebuild must not bake it in harder than today:
  `24C:strain-coreference-crosskind` records that the parked co-reference mechanism
  needs the fence movable and a kind-carrying canonical someday. Reserve the
  door-swing, build nothing (§5).

## §2 — The formal spine (the one-page relation spec `272` §1 owes)

*(status: conductor-proposed as spec; the relation and its ternary shape are
demonstrated-necessary by `273` §4's safety inversion; wants the §8 delta-6 ack)*

One comparison, everywhere:

    compare(cellA@ctxA, cellB@ctxB) ∈ { same(coord) | provably-disjoint | unknown }

**Consumer map** (which verdict feeds what — and only this):

| verdict | consumers | notes |
|---|---|---|
| same | transport (a fact established in one context licenses action about the other); the probe-outside license | never survival |
<!-- /* under-qualified per 27Xf cr-277-stale-transport-rows (2026-07-17): true as
relation algebra (same cells ⇒ a fact does transport), but CROSS-CONTEXT consumption
is not licensed by this row alone — it routes through plans/27C (measure-in-context
first; the fallback lane's flag gates; the structural carried-by row's scope is
27Xf Tier-1, human-owed). Annotate-don't-rewrite; the relation stands. */ -->
| provably-disjoint | survival sparing (kill-traffic misses the backing) | consumed ONLY inside the flag-gated survive tier (`271:rul-flag-is-razor-residue`; the flag permits acting on separation claims, never manufactures them) |
| unknown | the safe bottom: no transport, collide, walls, run | safe for BOTH consumers — why the relation is ternary (`273` §4: believed-no-overlap and believed-overlap are each dangerous to one consumer; only unknown is safe for both) |

**Generator registry** (every authored surface is a generator of verdicts; the engine
mints licenses at the chokepoint, humans only mint claims):

| generator | authored surface | verdicts it may generate | tier |
|---|---|---|---|
| token-equality + entity canonicalization | `kind__resolve()` | same (within kind, within context) | vouch (kind-owner) |
| selector-dialect comparison (§3) | verdict/observe marks mint; claim marks consume | provably-disjoint (same entity, cross-cell) | vouch; consumed under the survival flag |
| axis-invariance line | `kind__state_stored_only_in()` + the explicit invariance mark (`271:rul-invariance-speech-act`) | same across a context boundary (per axis) | vouch (kind-owner's typed line) |
| carried-by rows | engine-owned table (`272` §3 r1) | invariance for substrate-borne axes | structural (engine-warranted) |
| lend entries | `cmd__lend_map()` | boundary identity (full lend) / re-keying (mapped lend) | vouch (wrapper author's line) |
| keying | derived (the emission-set non-interference ingredients, né `272` r2; mapped lends; ρ-resolution) | blocks transport by re-indexing; NEVER provably-disjoint | license-free (`272` §4) |
| contradiction detection | non-interference-vs-typed-invariance; dual-peel; duplicate owners | refuse-both fail-fast | dictate |

**Properties** (each load-bearing; from `272` §1):

- Generator-INCOMPLETENESS is value-loss only — the default is the safe bottom.
- The bite classes are generator-INCORRECTNESS (the attributed knife: a wrong typed
  line) and CONTRADICTION (the standing declarations-genuinely-contradict refusal,
  `271:rul-proven-mutation-fails-fast` family).
- No generator anywhere produces cross-kind *same* (parked co-reference; §6).
- Every license computed at the chokepoint renders its full attribution chain
  (`24S` §4a four links; line-level first link per `273`'s merge-riders).

## §3 — The selector dialect (the survival-license algebra)

*(status: TYPED 2026-07-10, spike-provisional — "as ruled for now, and see where the
spike goes with it"; the family amendment typed same date)*

Restated from `271:rul-selector-disjointness-dialect-scoped` as build-spec:

- **Minting.** A selector token enters existence only as an annotation on a runnable
  marked line — verdict marks (`:`/`:!`) and observe marks (`:?`) both mint;
  claim/disturbs emissions never mint. No dislocated declarations: a kind's
  cell-structure is the projection of the marks loaded oracles carry. (Wording note,
  per `271:rul-measurement-is-authorship`: this note avoids "measure" — Dorc holds
  predictions and claims from authored bodies, never raw measurements.)
- **dialect(family, kind)** = the selector tokens that FAMILY's verdict/observe
  marks carry for that kind. Family per `271:rul-family` — name-derived, never
  file-derived, never author-derived. No global per-kind vocabulary exists.
- **Comparison** (inside `selector_covers`): same-entity, a claim SPARES a backing
  iff BOTH sides carry minted selectors AND claim-token ∈ dialect(the backing's
  minting family, kind) AND claim-token ≠ backing-token. Everything else COLLIDES —
  a selector-less coordinate on EITHER side (a ⊤ claim spares nothing; a
  whole-entity backing is spared by nothing), unminted tokens, cross-dialect
  tokens are all ⊤-selector. Cross-entity/kind disjointness unchanged.
  *(AMENDED 2026-07-13 per `279f:fix-spare-top-backing` — the prior wording
  special-cased only claim-side ⊤, leaving a whole-entity backing sparable
  by any dialect-member claim: an under-execution path, 279a-A5. ACKED, typed,
  2026-07-16; the first of the `279f:ask-amendment-acks` five to close.)*
- **Engine cost:** per-(kind × family) dialect sets + backing provenance (minting
  family) carried into the comparison.
- Properties (pin as tests; REWORDED 2026-07-13 per `279f:fix-dialect-properties` —
  the prior absolutes were falsifiable as written, 279a-F6/279b-fd2):
  empty world ⇒ byte-identical to entity-granular HEAD · cross-family monotone (a
  newly loaded family never alters comparisons against OTHER families' backings) ·
  within-family dialect growth MAY flip collide→spare against that family's OWN
  backings — that is the family's declared kill-surface control, consumed only under
  the survival flag · noise-safety scoped: an unmatched token spares nothing, but a
  carelessly-minted real-word token is live vocabulary (see §6 divergent-meaning) ·
  no self-licensing · subscription semantics native (a cell hung on a shared kind
  auto-subscribes to the entity's kill-traffic) · attributable at the sparing
  verdict (claim line + backing line + the dialect member's minting line; the
  disjointness proposition itself is asserted by no single line — which is why
  sparing lives behind the flag) · DST-clean.
  *(ACKED SOFT, typed, 2026-07-16 — 4/5 of `279f:ask-amendment-acks`. The human's
  grading, verbatim: "ack all 4, relatively softly. some of this is drifting close
  to spike territory - I need to *play* with some of this to be sure." The four
  accepted deltas, named for the record: delta-monotonicity-split (absolute
  monotonicity un-promised; cross-family monotone is what the rebuild pins) ·
  delta-kill-surface-blessing (within-family collide→spare flips are declared
  semantics — a family's vocabulary sets its own facts' survival granularity,
  flag-gated; the un-proposed observe-marks-don't-mint dial was surfaced eyes-open,
  not taken) · delta-attribution-downgrade (three-line-attributed sparing verdicts;
  the disjointness proposition has no asserter; recorded as the
  `271:rul-flag-is-razor-residue`-consistent reason the consumer is flagged) ·
  delta-subscription-trim (the kill-surface-control phrase moved from the
  subscription clause to the flip clause, where its price is attached). The
  softness rides §3's standing spike-provisional grading — these harden or churn
  on spike contact, not by further prose.)*
- Fences riding the ruling: ghost-ack-mark stays UNBUILT · same-token-divergent-
  meaning is adjudicability-list tier *(superseded 2026-07-16 → README-class; the
  §6 fence per `279f:fence-divergent-meaning`, acked)* · physically-overlapping
  cells within one
  dialect are inherent to narrowing, differential-testable · this precedent does NOT
  auto-generalize to context topology (context went DERIVED instead — `272`).

## §4 — The grammar (the authored-spelling half; the ack target)

*(settled at the 2026-07-12 delta pass; the ack record is `plans/271`'s task-12
entries)*

### §4a — The coordinate literal

    KIND:ENTITY#SELECTOR            sm.dorc.Service:"$svc"#enabled
    KIND:ENTITY                     sm.dorc.Package:"$pkg"        (whole entity)
    KIND                            sm.dorc.PkgIndex              (singleton/whole kind)

- `#` introduces the selector — typed PERMANENT (`271:rul-selector-introducer-hash`).
- The `#` must be ATTACHED: whitespace is not mucked with in sh, and the dialect
  never fights shell-comments — a `#` becomes selector-introducer only when a valid
  coordinate character immediately PRECEDES it. Mid-word `#` is not a comment-opener
  in any care-set shell — the spelling rides safely through stock parsers.
- The old property-suffix `.prop` (`…"$pat".matched`, USER_STORY's `.synced`) is DEAD
  in the same churn: `.` no longer introduces anything in coordinate position — dots
  belong to kind names (reverse-DNS) and to entity content only. Rendered fact-labels
  re-key identically: `sm.dorc.Package:nginx.installed` →
  `sm.dorc.Package:nginx#installed` (a new predicted-delta class for the re-bless,
  §7b).
- **Polarity lives on the mark sigil, never on the coordinate.** The sigil family:
  `:` (verdict, named sense) · `:!` (verdict, complement sense — the flagship's
  remove arm: `:! sm.dorc.Package:"$pkg"#installed`) · `:?` (observe). This
  supersedes the older trailing-`!` coordinate suffix (respelled in the specimens);
  coordinates stay pure names.
- **The empty-entity transitional form**: an entity-less coordinate with a selector
  spells `KIND:#sel` (`io.opentelemetry.Collector:#v0155`) — the faithful conversion
  of the improvised empty-entity singleton bind, which the typeless floor already
  retires on schedule (`24P` d8; `24L` §2). Transitional, not durable grammar.
- Consequence, relaxing the unquoted-dotted-entity ⊤-reject micro-decision (né
  `24P` d4): the ambiguity DISSOLVES —
  with no `.prop` production, `sm.dorc.File:/etc/nginx.conf` parses unambiguously
  (everything between `:` and `#`/end is entity). The ⊤-reject posture retires;
  quoting remains supported and remains the style the corpus teaches for
  variable-bearing entities.

### §4b — Position charsets (POSIX-in-spirit: `271:rul-posix-in-spirit-defaults`;
takes up the KNOBS `kTYANNOT` deferral)

Charsets and lexical minutiae re-use POSIX rules, simplified for our purposes and
matched in spirit; conservative for the spike. Applied:

- Kind: DNS labels joined by `.` (≥2 dots) — the one deliberately non-POSIX
  identifier space (reverse-DNS IS the identity, `24M:rul24M-reverse-dns-kinds`);
  the munge into NAMEs is the landed ASCII path (`24M:ca-munge-charclass` risks
  stand).
- Selector token: a POSIX *name*, in spirit — letter or underscore first, then
  letters/digits/underscores. Covers every minted exemplar (`enabled`, `active`,
  `installed`, `matched`, `cve_clean`, `v0155`). No quoted selectors at v1.
  Coordinates carry no polarity; the sigil does (§4a). A mark failing the charset
  is a loud parse diagnostic, never a silent ⊤.
- Entity, unquoted: the POSIX portable-filename character set plus the path
  separator — letters, digits, `.`, `_`, `-`, `/` (covers paths, package names,
  unit names). Anything else takes the quoted form. Deliberately narrow-start:
  expanding later is cheap; characters once granted can never be clawed back for
  future grammar.
- Quoting, here and in every future grammar-minted quoted position: POSIX quoting
  simplified as far as possible while staying true in spirit — double quotes with
  `"$var"` interpolation, single quotes literal; the exact simplification is
  spelled at parser-build time under the same standing ruling.

### §4c — Multi-cell marks: brace alternation

    printf '%s\n' "$1"   : sm.dorc.Service:"$1"#{enabled,active}

- Grammar: `#{tok,tok[,tok…]}` — no internal whitespace; ≥2 tokens; each obeying the
  selector charset; expands to one claim per token.
- **Scope: claim-emission marks only** (disturbs/reaches/store lines). Verdict and
  observe marks stay SINGLE-cell: a marked runnable line asserts exactly one thing
  (`275` §2; the orthogonality doctrine) — a two-cell verdict is two probe lines.
  This also keeps minting per-line attributable: every dialect token cites one
  marked line.

### §4d — Mark positions, and the role-scoped vocabularies

The dialect has exactly three mark positions:

1. **Trailing verdict marks** `   : <coordinate>` and `   :! <coordinate>` (named /
   complement sense) — on a runnable line in a verdict-bearing member; bind the
   statement's rc to the fact; mint.
2. **Trailing observe mark** `   :? <coordinate>` — read-disclosure; mints; widens
   the enclosing fact's backing (§5 backing-SETS).
3. **Trailing token mark** `   : <token>` — a NON-coordinate mark whose closed
   vocabulary is selected by the ENCLOSING MEMBER'S ROLE:
   - in `cmd__lend_map()`: dimension tokens (`user`, `fs-view`, …) — `271:rul-lend-map`;
   - in `kind__state_stored_only_in()`: substrate tokens (`fs` default · `kernel` ·
     `net-kernel` · `process` · `endpoint`) on emission lines (`272` §2,
     reserve-the-slot at v1), plus the axis-invariance token (§4e) on a colon-line;
   - in `cmd__disturbs()` / `kind__disturbance_reaches_only()`: kind (and
     kind#selector) typing of emission lines (`271:rul-emission-selector-on-mark`).

   Role-scoping is the collision answer: `fs-view` (a dimension) and `fs` (a
   substrate) can never meet in one room; each vocabulary is engine-owned, closed,
   dialect-versioned; users never mint tokens.

Plus one PREFIX position, joining from `274` §10: **`dorc:` on a command word**
(`dorc:sh -c '…'`) — full-analysis license on an eval'er head; probe-ship rewrites to
`dorc-sh`; strip = prefix-erasure; no nesting (`271:rul-dorc-prefix-head-synthesis`).
It is the only prefix-position dorcism; everything else is trailing or inline
(binds).

Together these four surfaces — dimension tokens, substrate tokens, the invariance
token, the `dorc:` prefix — are the queued **ONE deliberate kOOB reading**
(`274` §12; `273` §8; task-8's routing): all are strip-erased annotation on runnable
sh, i.e. metadata riding the sanctioned trailing-mark lane, not sidecar
configuration; the runnable bodies remain the configuration. That reading of the
`kOOB` redline is settled (the 2026-07-12 delta pass); it is not a new mechanism.

### §4e — The axis-invariance line (`271:rul-invariance-speech-act`)

    sm_dorc_Package__state_stored_only_in() {
       printf '/var/lib/dpkg\n'                  : fs
       :                                         : invariant:user
    }

- Token grammar: `invariant:<axis>` — one colon-line per invariance-STATE, leaving
  room to brace-alternate later (`invariant:{user,netns}`, composing with §4c's
  precedent). Axis drawn from the ingredient-borne subset of the ratified axis
  vocabulary (v1: exactly `invariant:user`).
- Carried on a colon-line (`:` — sh's nothing-command; strips to a harmless no-op;
  the lend-map precedent).
- Scope: WHOLE-member. Invariance is a claim about the total emission's negative
  space (`272` §2's `only`-contract) — per-line invariance buys nothing (one keyed
  store keys the kind) and invites incoherent mixes.
- The emission-set non-interference derivation (né `272` r2) runs as
  CONTRADICTION-CHECKER: a typed `user-invariant` line in a body whose emission
  carries who-am-I ingredients (`$HOME`, `$(id -un)`, …) is
  declarations-genuinely-contradict ⇒ plan-time loud fail-fast. Absent the typed
  line, the derivation yields keying/conjecture/hints only — transport never
  licenses from negative space (task-8's razor-conversion, closing `272` §3's
  superseded derivation-as-license).

### §4f — Binds (unchanged; restated for completeness)

    pkg : sm.dorc.Package = "$1"

Binds name entities, never cells (`271:rul-binds-entity-only-provisional`, SOFT). No
selector in bind position at v1 — facts about cells attach via marks on
probing/emitting commands. Strip erases the bind to plain assignment semantics as
landed.

### §4g — The command-word thread (RESOLVED 2026-07-12; né delta-command-word-disposition)

`271:rul-dorc-prefix-head-synthesis` banked the human's lean toward revisiting old
syntax toward command-word spellings (127-loud beats tail-position corruption) and
parked it at this sitting. RESOLVED by `271:rul-trailing-marks-stand` (typed):
trailing marks stand for the spike; the command-word idea retires as a passing
thought, not a roadmap candidate. The analysis that settled it, kept on record:
trailing marks on real commands corrupt silently under `sh file` (marks become
argv — the printf-emission case actually mis-emits), but the corruption surface is
narrowed by the `#!/usr/bin/env dorc-sh` shebang (loud-fails direct execution on
dorc-less boxes), the marker gate, and `dorc strip`; the colon-line marks are
already command-shaped and inert; `#`-selector and the mark grammar are typed; the
spike is the kTYANNOT livability experiment. Syntax remains marker-gated
(`276:rul-verdicts-never-stable`), so nothing is foreclosed.

## §5 — Seams reserved (representation room; build nothing)

- **seam-context-qualifier-slot** (né `24S:A7`(i)) — DISCHARGED into §1's context
  slot + relational chokepoint.
- **seam-kind-topology-room** (né `24S:A7`(ii)) — DISCHARGED into the store member + the kind-owner registry
  (`271:rul-seam-kind-owner-registry-room`: one engine-internal extensible per-target
  registry, at-most-one-per-(family × role) generalized to clause families not yet
  minted, loud on conflict, zero file semantics) + the substrate-mark slot
  (`24S` §7 A7(ii)).
- **uniqueness-bit** (`271:rider-entity-algebra-uniqueness-seam`): room in the
  coordinate/comparison representation for a future uniqueness bit. No strong update
  exists at v1 — `Kill` accumulates; the standing `231` fence rules "probably unique"
  may only DEMOTE, never license. `17N` §4's warning is the design driver:
  over-coarsening manufactures false uniqueness ⇒ unlicensed strong-update =
  over-correlation; uniqueness is SF-1-undecidable, so the bit will someday be
  fed by owner declaration or engine conservatism, never inferred hot.
- **backing-SETS** — a fact's backing is a coordinate SET, derived per-channel
  through recipe dataflow (`275` §2); an observe mark inside a verdict body WIDENS
  the enclosing fact's backing (`271` observe-backing-widening — safe direction,
  kill-surface only grows). First consumer of the fragment-preserving recipes; rides
  the value-recipe-reshape. Set-lifting law (added 2026-07-13 per
  `279f:fix-set-lifting`): consumers quantify UNIVERSALLY — sparing over a
  backing-set requires EVERY footprint×backing pair provably-disjoint (the `272`
  §1 ternary verdict: generator-licensed, never derived; any unknown member ⇒
  collide); transport over a backing-set requires every member to transport. An
  accidental existential in either consumer is an under-execution path (279b-fd5).
  Two side-invariants (added 2026-07-17 per 27Xf cr-set-lifting-vacuous-at-empty —
  universal-over-∅ is vacuously true, so the law without them licenses
  sparing-every-wall on an empty set): (inv-backing-set-non-empty) a fact's
  backing-set is non-empty BY CONSTRUCTION — the minting line's own coordinate is
  always a member; (inv-top-never-empty-set) ⊤/unknown is never ENCODED as ∅ — an
  unknown backing is a ⊤ member, which collides. DST pins: ∅-unrepresentable;
  ⊤-never-∅. (The in-flight build independently minted the same precondition —
  `27D` stage-4b's minting-line threading; these invariants ratify that shape.)
  *(ACKED, typed, 2026-07-16 — 2/5 of `279f:ask-amendment-acks` — with a
  fixpoint-soundness clause. Banked human concern: the generator/consumer split
  reads pipeline-dependent; under any future re-run-to-fixpoint, a spare/collide
  OUTCOME circling back as input could launder disjointness-information from
  sources fenced out of generating it. Clause of record: the universal meet is
  the fixpoint-robust form — at every intermediate iteration state a
  not-yet-licensed member reads unknown ⇒ the set collides, so the set-verdict is
  conservative and evaluation-order-independent, where an existential would spare
  on whichever member resolved first. Two pins join the §5 rebuild-brief
  quantifier pins: (pin-no-outcome-as-generator) compare-verdicts feed only their
  licensed consumer — same→transport, provably-disjoint→sparing — and never
  re-enter the relation as evidence for a later verdict; (pin-set-meet-order-
  independence) a set with any unknown member collides at every iteration,
  whatever the member-resolution order. NB the spike at HEAD has no such
  back-edge — the value plane runs strictly before the probe, and plans are
  minted once, never probe-re-entrant within a run; the post-probe re-bind
  (seam-pipeline-order, directly below) re-reads this clause the day it is
  designed.)*
- **The re-bind seams** (`219` q-3.e/q-4.b/q-4.c, carried by
  `271:rider-value-recipe-reshape-capture-seams`; restated because the fact-domain
  re-key is the cheapest moment they will ever have): (seam-pipeline-order) the value
  plane runs strictly BEFORE the probe — folding a captured literal back requires a
  second value-flow pass or a fold-time substitution channel; (seam-literal-
  provenance) a value-plane-visible distinction between source-literal and
  probe-captured bytes (a provenance slot on `ValueOf`, or the site-keyed-channel
  route — `219` leans the latter; leave the slot OPEN either way).
- **co-reference door-swing**: the kind-fence stays pre-canonicalization at v1, but
  the rebuild keeps it MOVABLE and keeps `CanonicalCoord` extensible toward a
  kind-carrying canonical (`24C:strain-coreference-crosskind`) — the parked ONE
  mechanism (`24M:rul24M-kind-unify-owed` · `24S` A5 · `272` §5's fourth pointer)
  lands against this door later.
- **OutClaim rename** (`275` care-outclaim-rename): the newtype is channel CONTENT,
  not a claim; rename rides the reshape, not urgent.

## §6 — Fences and carves (pin as tests in the rebuild brief)

- **never-derive-separation** (`272` §4): derivation yields keying, never separation;
  address-inequality is not referent-inequality; separation across context-values
  arrives only as a declared, owned act — and at v1 does not exist at all.
  Terminology (delta-5): **keyed** (derived, safe, license-free re-indexing) vs
  **partitioned** (declared-only, knife-tier); the conflating token "sensitive"
  retires.
- **addresses-are-not-coordinates** (`272` §5): store-member locators feed exactly
  the per-axis dependence bit and the per-value keying recipe — never
  file-coordinates, never address-granular disjointness, never intersected against
  `sm.dorc.File` facts.
- **silence never identifies; ⊤ identifies with nothing** (including itself) — `24S`
  §8 / `273` §9, unchanged.
- **same-token-divergent-meaning is a frontloaded limitation** (added 2026-07-13 per
  `279f:fence-divergent-meaning`): a claim-token is interpreted in the
  BACKING family's dialect, so two families spelling the same token for different
  cells can spare each other's backings under the flag. README-class constraint per
  `271:rul-net-quality-u-curve` (documented and differential-tested, never
  lint-rescued); previously parked as adjudicability-tier only (279b-fd2/279a-A6).
  *(ACKED, typed, 2026-07-16 — 3/5 of `279f:ask-amendment-acks` — with the
  attribution-class clause banked. Rationale of record, human-typed: the ecosystem
  is deliberately a coheres-only-by-chance-and-effort, dynamically-discovered,
  multi-author space — kind-coherence is explicitly best-effort, and
  failures-of-kind-coherence are a KNOWN hole. Reverse-DNS kinds
  (`24M:rul24M-reverse-dns-kinds`) are the ATTRIBUTION-CLASS: don't mint from
  scratch inside `com.widget` unless you own widget.com at some registrar; search
  first, read the namespace owner's docs on their types, respect those. Dorc is
  FORCED into registry-less comingling — no login, no owner-language, no
  mechanical way to distinguish the real `com.elliott` author from an interloper
  minting `com.elliott.Widget` — but no-mechanical-owner ≠ names-are-wild-west:
  "use your own names, and re-use others as they dictate you use them; doing
  otherwise is violating the oracle contract → Dorc breaks" (the standing
  you-break-contract ⇒ promises-void class). Hedge preserved: this records the
  human's understanding of the defense, not a certification of its adequacy —
  adequacy evidence stays at the differential/field-trial tier. Engine-side bound
  restated for the record: dialects grow ONLY by authored marks parsed at
  oracle-read (§4b–§4d; emission lines carry raw entity bytes, kind#selector
  rides the mark) — a host can never mint a selector at runtime, so fd2's
  n(n−1)/2 pairwise surface is authored, reviewable text. fd2's stronger remedy
  (default unequal-to-unknown pending a partition speech-act; one accountable
  owner per family) stays DECLINED; any future positive partition spelling
  arrives additive, on the withholding side, never as a re-read of minted
  marks.)*
  *(Refined, human-typed 2026-07-16 during the 4/5 sitting: the ownership clause
  above is slightly too strong — "minting into a namespace = claiming ownership"
  overstates. Collaboration inside someone's namespace WILL happen; the tooling
  is there, and nothing can prevent it. The correct contract is
  OWNER-ADJUDICATION: once an oracle has been out in the world a while, its
  namespace is a backwards-compatibility concern that only its owner can manage.
  Dorc — a binary, in a loaded universe of oracles — can never see which marks
  are authoritative, nor (modulo later versioning features; MH2) which are
  ROTTED. We owe the owner a ROUTE to adjudicate; the rest is tools-later plus
  human interpersonal communication and documentation — no other way around it.
  No mechanism change; the adequacy hedge above stands.)*
- **keying never feeds survival · hint-lane values never feed survival · the flag
  permits acting on separation claims, never manufactures them · per-invocation,
  never a default** — task-8's restated fence bundle, acked at its close.
- **Marked runnable lines mint (verdict/observe); claims never mint** (§3) — and a
  mark asserts exactly one thing; per-channel/multi-consequence readings are
  DERIVATIONS (`275` §2).
- **The bootstrap file kind's grounding is engine-native**: no kind-owner authors an
  sh identity/grounding function for files — that function would be pure ceremony
  (at v1, file identity is interned token-equality; any future symlink-aware
  canonicalization would also be engine-native, gated by the fs-view work). A
  deliberate bootstrap exception, not a precedent: capability stays
  language-side-first everywhere else (the wrapper/eval'er machinery is the model).
- **Cross-kind same does not exist** at v1 (no generator; the fence + the parked
  mechanism).
- **Empty world ⇒ byte-identical to HEAD** — the whole algebra is invisible until
  oracles mint it (rung-0 regression, every brief).

## §7 — Consequences routed

### §7a — Specimen amendments (`24P` §10's conductor obligation; reviewed in-code as delta-specimen-amendments)

1. All five mark-bearing specimens: `.prop` → `#prop` in marks and expected
   fact-labels (`sm.dorc.GrepMatch:"$pat".matched` → `…"$pat"#matched`; goldens'
   rendered labels likewise).
2. `strawman24-survive-simple`: `apt_get__touches` → `apt_get__disturbs`
   (`271:rul-touches-becomes-disturbs`).
3. `strawman24-reach-static-service`: `sm_dorc_Package__reaches` →
   `sm_dorc_Package__disturbance_reaches_only` (`271:rul-at-most-family-names`).
4. `strawman24-alias-provides` gains the NEW-member exhibit (the respell rider's
   third name): `sm_dorc_Package__state_stored_only_in()` with one `: fs` emission
   and the `: user-invariant` colon-line — §4e's grammar, exhibited for the in-code
   ack.
5. `strawman24-pipe-guard-oracle-converged` re-gains an XFAIL: the `#matched`
   respell un-parses at HEAD, so the case goes back to declared-failing-spec status
   (it was un-XFAIL'd only because its old delta was golden-text-only, `24P` §9b).
6. `24P` gains a §11 addendum recording 1–5 + the predicted-delta update (§7b);
   §4's class-1 now includes the selector-introducer re-key corpus-wide.

### §7b — Block-rebuild brief obligations

The re-key: transfer functions × footprint intersection × resolver coordinates ×
probe keying move to the §1 coordinate + chokepoints; §3's dialect sets + backing
provenance enter the comparison; §5's seams reserved in the representation; §6's
fences pinned as tests; `24P` §3's residual flags (unquoted-dotted-entity corner now
DISSOLVED per §4a — drop it from the brief; forward-munge keying; marker-gating
scope; stale comments) re-checked against this grammar.

## §8 — Disposition (post-delta-pass, 2026-07-12; the ack record lives in
`plans/271`'s task-12 entries)

Settled and applied in place: the coordinate grammar (§4a/§4b, incl. the
POSIX-in-spirit charsets and the `:` / `:!` / `:?` sigil family) · brace
alternation (§4c) · the `invariant:<axis>` line (§4e) · the role-scoped mark
vocabularies and the kOOB reading (§4d) · keyed/partitioned (§6 — "partitioned"
hard-acked, "keyed" tolerated-unloved; no churn) · the command-word disposition
(§4g) · the topology bundle (§§2/4d/6 — mixed hard/soft/spike-tier acks, graded
per-row in `271`; the formal spine's generator registry and the verification
posture remain conductor-proposed, deliberately left on the table for the human's
adversarial pass) · the specimens (in-tree, committed).

Riding the human's adversarial pass, by design: the `275` ratifications (three
regimes · backing-inheritance/per-channel backing sets · the cross-context
transport chain) and everything conductor-proposed above.
<!-- /* superceded 2026-07-17 (27Xf): the `275` transport ratifications were REFUSED
(279f §3), not still-pending — the adversarial pass happened. Cross-context answering is
re-answered by plans/27C (measure-in-context; transport demoted to the flag-tier fallback
lane). The unflagged structural carried-by row (§2 generator table) is scoped to
substrate-borne axes only and is the open design question — 27Xf Tier-1. */ -->

## §9 — Status table

| component | status |
|---|---|
| flat three-place coordinate + riders | TYPED (`271` task 1) |
| `#` selector introducer | TYPED, PERMANENT |
| emission-selector-on-mark; binds-entity-only (SOFT); kind-vs-selector menu | TYPED |
| selector-dialect algebra + family amendment | TYPED, spike-provisional |
| context slot ≡ qualifier; relational chokepoint; registry room | TYPED (`271` task 2) |
| store member name/existence; invariance speech-act; carried-by re-role | TYPED (tasks 3/8) |
| formal spine (§2): the ternary relation + safety inversion | acked; the generator registry stays conductor-proposed, on the table for the adversarial pass |
| exact grammar: attached-`#`, sigil family `:`/`:!`/`:?`, charsets, brace alternation, invariance token | settled (POSIX-in-spirit per `271:rul-posix-in-spirit-defaults`; `#{a,b}`; `invariant:<axis>`) |
| the deliberate kOOB reading | settled |
| keyed/partitioned vocabulary | settled ("partitioned" hard; "keyed" tolerated) |
| topology bundle rows | acked, graded per-row (`271` task-12 entries; substrate marks soft/narrow-as-built; spine-registry + verification posture conductor-proposed) |
| `275` ratifications (regimes · backing-inheritance · transport) | riding the human's adversarial pass <!-- /* superceded: transport ratification REFUSED (279f §3); re-answered by plans/27C. */ --> |
| command-word thread | closed (`271:rul-trailing-marks-stand`) |
| specimen amendments | in-tree, committed |
| seams (§5) | reserve-only; carried into rebuild + reshape briefs |
