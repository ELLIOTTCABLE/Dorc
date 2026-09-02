# 30W — Index-kinds, region overlap, and owner-answered referents: the pivot algebra over existing claims

> AI-authored design-of-record (Fable, the 2026-08-31/09-01 pivot-book sitting; human
> present and adjudicating; banking human-directed). Ahistorical: mechanics, effects,
> limitations, UX, and the safety story, as they hold IF BUILT. Ratification status is
> per-item in §10; the sitting's trail lives in `notes/26M`. Authority: root docs,
> `spike/CLAUDE.md`, and the standing welds outrank this. Extends — never re-opens —
> `notes/272` (kind topology), `notes/273` (the wrapper surface), `plans/30T` (authored
> file semantics and the ask-the-world discipline), `plans/30U` (finished definitions),
> `plans/30S` (witness envelopes), `plans/27C` (measurement in the denoted context).

## §0 — the design in one screen

Pivot-class books — controller-scoped books whose lines denote other machines and cross
lifecycle boundaries (create, reboot, re-login, reimage) — need three capabilities the
single-world engine lacked: facts about several worlds in one analysis; facts crossing a
lifecycle boundary with survival exactly where an owner has justified it; and referent
questions (which names co-refer; what lies inside what) across filesystems and machines.
All three reduce to the existing claim algebra plus four small additions:

1. **Index-kinds.** A context axis is an ordinary kind used to index other kinds' cells;
   the context slot is a product over such kinds. `Host`, `Boot`, `LoginSession` join
   `User` and `NetNamespace` on the same footing.
2. **The ternary relation, one generator richer.** `compare(cell@ctx, cell@ctx) ∈ {same,
   provably-disjoint, unknown}` gains region **overlap** as an owner-authored generator,
   alongside resolve, per-aspect identity, invariance, lends, and keying.
3. **Owner-answered referents, everywhere.** The engine knows syntax; every semantic
   question about a name is shipped to the owning kind's member, measured in the
   denoted context, three-valued, perishable, attributed. No engine table of world facts.
4. **Per-arm incremental claims with completion sentinels.** Every at-most family is
   arm-incremental (safety-monotone to extend) with one deliberate totality sentence.

No new claim polarity, no new grounding namespace, no engine ontology. The pivot
patterns — patch-day survival, transport re-keying, staged state across a reboot,
multi-host partition, subtree containment — each fall out of the composition (§7).

## §1 — the object model

**Kinds, entities, selectors, cells** are unchanged (`277`). What is made explicit:

- **Index-kinds.** Any kind may serve as an index for other kinds' cells; a cell's
  context is a tuple of entities drawn from index-kinds. `user=alice` is an entity of
  the User kind, `host=web1` of Host, `boot=a41f…` of Boot. Two flavours differ only in
  how the engine obtains the value:
  - *entered* — a wrapper's `lend_map` entry sets it for the remainder (`sudo -u` →
    User; `ip netns exec` → NetNamespace; `ssh dest` → Host, a mapped lend on the Host
    index whose value is the destination operand);
  - *measured* — no entry form exists (nothing lends time); the value is read from the
    world (`boot_id`, `machine-id`/instance-id, the session UUID) by an ordinary
    stdlib predict, and carried as a witness (§4).
- **Address-space tokens are index-kinds viewed as namespaces.** A locator interpreted
  "in the filesystem" is a name in the current MountNamespace entity's address space;
  "in the network kernel", the NetNamespace's. Short tokens (`fs`, `net-kernel`,
  `process`, `endpoint`) remain as sugar for the stdlib kinds they name.
- **Engine-blessed kinds: File and Host only.** sh semantics force the engine to know
  that redirects and `.` operands address files in a mount namespace; transport forces
  it to know destinations address hosts. Every other kind — including Boot and
  LoginSession — is stdlib content referenced by claims.
- **Referent-transparent kinds.** A kind's owner may declare that its names ARE
  referents (minted-unique identifiers: `boot_id`, `machine-id`, a pod UID). For such a
  kind, entity inequality is a disjointness generator. The default is the opposite:
  names are names, and inequality means nothing.

## §2 — the relation and its generators

| generator | authored surface | verdict it may yield | consumer / tier |
|---|---|---|---|
| canonicalization | `kind__resolve()` | same (within kind, within context) | transport; vouch-tier |
| per-aspect identity | the owner's per-selector relation mapping (`30T` §6) | same / disjoint per aspect | both; measured, perishable |
| **overlap** | `kind__overlaps()` — region × region or region × entity | overlaps (⇒ collide) / disjoint / unknown | sparing; the disjoint arm under the flag |
| invariance line | `undivided-by-transit-across <index-kind>` in the store member | same across that index | transport; vouch-tier |
| lend entries | `cmd__lend_map()` | boundary identity (full) / re-keying (mapped) | transport; vouch-tier |
| keying | derived from stores × index dependence | blocks transport only; never disjoint | license-free |
| referent-transparent inequality | the kind's declared property + measured values | disjoint | sparing; structural |
| contradiction | typed claims vs derivation (`272` §3 as checker) | refuse-both | fail-fast |

**The two-consumer law.** *Same* licenses transport and endangers it when wrong; *disjoint*
licenses sparing and endangers it when wrong; *unknown* is safe for both. Therefore every
authored member's lazy answer must land on unknown (decline, `return 2`); each positive
arm is a deliberate, pointable line; and each is gated by the consumer it endangers —
attribution and vouch-tier for transport (`271:rul-invariance-speech-act`,
`rul-flag-is-razor-residue`), the admin's `--risk-faultless-skips` plus the footprint
kind's finished definition (`30U`) for sparing. Member spellings follow: `overlaps`
names the safe arm (rc 0 = collide); the disjoint arm is written on purpose.

## §3 — the claim family, per-arm and incremental

All at-most families share one shape: arm-incremental emissions that only ever *add*
(collisions, backing, reach — the safe direction), plus one deliberate completion
sentinel that asserts totality and is the sole licensor of disjointness in that family.

- `cmd__disturbs()` — at-most footprint per invocation shape; completion by the per-arm
  `disturbs nothing-else` record (`30U`).
- `kind__disturbance_reaches()` — widening entailment toward other kinds; same sentinel.
- `kind__state_stored_in()` — per-arm `(locator, address-space)` emissions naming where
  a selector's truth lives; every emission adds backing (more collisions); the
  invariance colon-lines ride here (`:  : undivided-by-transit-across sm.dorc.Boot`);
  the completion record `stored nothing-else` (printf-sentinel shape, per selector or
  whole-kind) is the totality act that licenses "a footprint outside my stores spares
  me". Without it, stores are open-world: an unlisted store is unknown and collides.

  ```sh
  sm_dorc_Service__state_stored_in() {                 # STRAWMAN
     case "${2-}" in
     enabled) printf '/etc/systemd/system\n'   : fs
              :                                : undivided-by-transit-across sm.dorc.Boot ;;
     active)  printf 'systemd\n'               : process ;;
     esac
     printf 'stored nothing-else\n' >>"${DREP_V1:-/dev/null}"
  }
  ```

- `kind__overlaps()` — measured in the denoted context; rc 0 overlaps, 1 provably
  disjoint, ≥2 unknown; mount-crossing and other view-aliasing shapes decline:

  ```sh
  sm_dorc_File__overlaps() {                            # STRAWMAN
     a=$(realpath -m -- "$1") && b=$(realpath -m -- "$2") || return 2
     case "$a" in "$b"|"$b"/*) return 0 ;; esac
     case "$b" in "$a"/*)      return 0 ;; esac
     [ "$(findmnt -no TARGET -T "$a")" = "$(findmnt -no TARGET -T "$b")" ] || return 2
     return 1
  }
  ```

Every emission member ships on the derived-footprint rails (strip-only, read-only,
stdout-emitting, all-or-nothing readback, body-death refuses the whole claim), and every
claim carries its author's name into the render and `dorc why`.

## §4 — keying, invariance, and lifecycle boundaries

Per (kind × selector × index-kind), the store member yields one of three outcomes:

- **invariant** — the owner's explicit `undivided-by-transit-across` line: the cell is
  one cell across values of that index; a fact transports across the boundary.
- **keyed** — the store depends on the index (a `: process` store depends on Boot; a
  who-am-I ingredient depends on User): cells are named per index value. Post-reboot,
  `Service@active` is a *different cell* from the one measured — unmeasured, hence
  guard or run. Keying is derived and license-free.
- **⊤** — silence or an untraceable body: no license, may-alias, walls.

Lifecycle events need no havoc machinery beyond this:

- A **transit verb** (`reboot`, `doctl compute droplet create`, `kubeadm reset`) is an
  ordinary mutator whose footprint includes an index-value cell. One engine rule: a
  disturbance of an index-value cell re-keys that index for everything downstream.
  Invariant cells are untouched; keyed cells are new, unmeasured cells.
- **Containment among index-kinds** (a machine ⊃ its boots ⊃ their sessions) is
  ordinary `reaches` between those kinds, declared by their owners.
- **Expected sever.** A dispatch whose footprint includes the index-value cell backing
  its own entry path is self-severing; sentinel absence at that site is the success
  shape, not a transport loss.
- **The witness.** Measured index values are captured at probe and asserted at apply
  standup and after every fired index disturbance (the `30S` envelope shape). A
  mismatch is an integrity outcome — withhold, per the unplanned-churn carve — never
  a verdict and never a license input. This is deliberately not a general
  assert-any-fact-at-apply surface; the TOCTOU horizon stands.

The derivation over stores (address-space × index dependence; the mount tier of an `fs`
locator, read from the host's mount table) is a **contradiction-checker and hint source
only**: a `: process` store, or an `fs` locator on a tmpfs mount, claiming
boot-invariance is a declarations-genuinely-contradict fail-fast. The engine never
licenses from the derivation.

## §5 — worlds

- **Host is an entered index.** The ssh entry form's `lend_map` maps the Host index to
  the destination operand; the Host kind's binder turns that locator into a measured
  identity (host key, machine-id) or declines. Every fact carries its world in the
  context slot from the moment it enters the controller.
- **Two destinations are one world only by measured identity** — the Host owner's
  `resolve` compares what the machines *report*, never the spellings. Name inequality
  separates nothing; identity inequality (Host is referent-transparent by its owner's
  declaration) separates cleanly, so multi-host books partition by derivation rather
  than by architectural assumption, and a mutation on one world walls only that world.
- **Edge facts** (reachability, endpoint state) are keyed by vantage and far end; an
  endpoint address change re-keys endpoint cells and touches no interior cell.
- **Shared state across worlds** is not a special declaration: a volume mounted on two
  hosts is one referent because its owner's `resolve` measures the same identifier
  through two entries.

## §6 — the world-knowledge law, general form

The engine knows **syntax**: path structure, argv structure, redirect geometry, entry
chains, dot-source operands. Every **semantic** question about a name — co-reference,
containment, what a write means here, which machine this is — is compiled into a
question and shipped to the owning kind's member, executed in the denoted context at the
latest sound phase (`30T:rul-questions-route-to-latest-phase`). Answers are three-valued,
attributed, and **perishable**: bounded by the effective-world reach of namespace-mutating
acts (`mv`, `rm`, `ln`; a rebuild; a re-login), never consumed as timeless. The engine's
verbs are *measure*, *act-and-verify its own mutations*, and *arrange geometry*; never
*interpret*. Locators from any parsed structure become coordinates only through a binder
(`30T:rul-locators-are-parsed-coordinates-are-authored`, applied to every locus); the
engine compares no names of its own accord.

## §7 — how it functions in practice

- **Patch day** (a mid-book `reboot` above a large configure tail): the reboot's
  footprint is the Boot index-value cell. `Package@installed` and `File@content` carry
  boot-invariance lines ⇒ elide; `Service@active` is boot-keyed ⇒ its half of an
  `enable --now` site guards, re-measured live post-boot; volatile-only sites run. The
  why-chain's naked link is a *storage* line — "active lives in the process" — a
  documented, auditable claim.
- **A transport change** (sshd moved to another port): the modeled restart's footprint
  is the endpoint cells; interior facts never referenced them; the tail survives under
  the existing cell-scoped footprint machinery. No lifecycle machinery is involved.
- **Staged state across a reboot** (grub/cmdline, a first-boot oneshot): the Boot
  witness measured before and after; the render and `dorc why` relate pre-transit
  staging, the witnessed transit, and post-transit measurement — and localize a
  failure to the gap between them.
- **A multi-host conductor book**: each entry measures its machine identity; worlds
  partition by identity; a mutation dispatched to one world walls only that world.
- **Subtree containment**: an owner's `overlaps` answering *disjoint* for a footprint
  region and a fact's store region spares the fact under the flag, with cross-mount and
  aliasing shapes declining to unknown.

## §8 — the safety story and the enhancement curve

The governing law: **no authoring step reduces safety; disjointness is licensed only by
a deliberate arm, consumed only under the admin's flag.** The curve for a kind-owner:

1. silence — total walls;
2. `state_stored_in` arms — keying/invariance transport across contexts; same-kind
   footprints still collide;
3. `overlaps` with only the overlaps/unknown arms — safety-neutral (adds walls at most);
4. the disjoint arm, and the `stored nothing-else` sentinel — the deliberate acts; their
   sparing consequences consumed only with `--risk-faultless-skips`, cross-kind only
   with the footprint kind's finished definition.

The knives, each attributed to one line: a wrong disjoint arm (spares past a real
write); a wrong *same* from a Host `resolve` (transports across two real machines); a
wrong invariance line (the pipx-in-`~/.local` shape: an omitted index-dependent store);
a premature `nothing-else`. Every survival names its licensor; every transport names
its vouch. The v0 floors of `30T` §6 stand until the identity tier exists:
entry-mutating verbs are total walls; same-kind path-distinct comparisons are unknown.

## §9 — deliberate limitations

- **No metric time.** The model carries event-time as identity plus order. Durations,
  freshness windows, expiry, and decay are either *authored* (clock-reading guards —
  `-nt`, `-newermt`, expiry judgments — spelled in sh, judged by their authors) or
  *parked* behind the standing hermeticity fences (`an-freshness`, `kSTATE`, the TOCTOU
  horizon). Wait/deadline modeling and planning-duration staleness are the named
  re-entry points; any freshness work rides §4's witness mechanism.
- **Exits are never inferred.** Every dispatch truthfully ends a session; only a
  *claimed* index disturbance re-keys. Silence about a boundary is not a wall.
- **Endpoint opacity** (`272` §8): beyond a daemon's socket nothing is visible.
- **Below a blind act** nothing is claimed (`30P:law-no-unsoundness-below-a-blind-act`).
- **Same-entity precision** and namespace-tolerant precision follow `30T` §9's
  non-capture list unchanged.

## §10 — owed changes and the build sketch

Rulings owed (none of §1–§9 is welded; items marked TYPED are ruled, the rest await):

- `rule-unreserve-host-as-entered-index` — `271:rul-axis-vocabulary-v1` reserves `host`
  as never; §5 needs it as an entered index. Explicit ruling required.
- `rule-separation-from-identity-not-name` — §1's referent-transparent kinds as a
  disjointness generator; `272` §4's carve re-grounded on names.
- `rule-incarnation-invariance-passes-the-razor` — whether an invariance line against
  Boot/Machine is an own-domain closed claim (vouch-tier, unflagged, like User) or
  survival-tier. §8 assumes the former for transport and the flag for sparing.
- `rule-dissolve-closed-axis-and-substrate-vocabularies` — index-kinds as kinds; short
  tokens as sugar; the carried-by table retired to checker knowledge.
- `rule-only-decomposes-everywhere` — `state_stored_only_in` → `state_stored_in` +
  `stored nothing-else`, mirroring the reach family; the `only` quantifier leaves the
  lexicon.
- `rule-two-consumer-name-bias` — §2's law refines `30T`'s name-bias law: lazy ⇒
  unknown; both positive arms deliberate.

Build items, by dependency:

1. **Context-slot generalization** — from a fixed dimension tuple to a product over
   referenced index-kinds. Retrofit-hostile; audit the as-built `FactKey.context`
   normal form first.
2. **Index-kind stdlib** — Boot / Machine / LoginSession with identity reads; Host's
   binder and `resolve`; the referent-transparent declaration.
3. **The re-keying rule** and expected-sever derivation at the settle/wall seat.
4. **The witness** on the `30S` envelope rails; integrity-plane consumption only.
5. **`overlaps`** as the identity tier's second relation on `30T`'s
   `comp-identity-tier` (shared plumbing with the loader standups).
6. **Store-member decomposition** — per-arm emissions, the completion sentinel, the
   invariance lines' new index targets; the contradiction-checker's mount-tier map.
7. **Verb surfaces accepting kind coordinates** (`lends`, `stored-in`,
   `undivided-by-transit-across`), with sugar preserved.

Sequencing: items 1–4 serve the pivot arc directly and precede any multi-host revival;
5–6 ride the identity tier and the stdlib arc; nothing here re-opens the survival gate,
and the license-consuming half remains additionally deferred on economics.
