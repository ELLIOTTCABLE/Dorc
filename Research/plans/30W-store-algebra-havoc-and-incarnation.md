# 30W — the store algebra: havoc, persistence, and incarnation over existing claims

> AI-authored design synthesis (Fable, the 2026-08-31 pivot-book sitting; human present
> and adjudicating throughout; banking human-directed same day). Ahistorical
> design-of-record for the MECHANISM; the sitting's exploration trail, ack-ledger, and
> superseded strawmen live in `notes/26M`. Authority: root docs, `spike/CLAUDE.md`, and
> the standing welds outrank this; NOTHING here is welded or ruled unless it cites a
> `26M` ack. The license-consuming half (§4) rides the existing `kSURVIVAL` gate
> unchanged and is additionally deferred on economics (human, 2026-08-31: no new
> machinery buildable now or soon); the witness/integrity half (§3) is independently
> buildable. Companions: `notes/272` (`state_stored_only_in`'s origin) · `plans/30U`
> (disturbance-reaches + finished definitions — the entailment substrate this composes
> onto) · `plans/30S` (the witness-envelope pattern) · `plans/27C` (measurement in the
> denoted context) · `plans/233`/`239` (the survival machinery this extends, never
> re-opens).

## §0 — the result in one screen

Pivot-class books (controller-scoped books whose lines denote other machines) forced a
cluster of apparently-new concepts — epochs, incarnations, transits, havoc-axes,
partial-havoc, reboot-survival. All of them collapse onto the EXISTING claim algebra
with **no new claim polarity and no new grounding namespace**:

> **Havoc is disturbance of a store. Persistence is storage in a disjoint store.
> Time enters the model only as identity and order.**

A *store* is not a new sort: it is a ROLE an ordinary kind takes on when its owner
declares the right relations (§1). `File` has been playing this role implicitly since
stage 5; the synthesis lifts the role out and generalizes it. Two engine extensions are
owed (§2 items 2 and 5); everything else is inherited: the verbs, the reverse-DNS
grounding economics, the knife-holders, the `kSURVIVAL` gate, the attribution lanes.

## §1 — the Store role

A kind is store-capable when its owner's declarations give it these four relations —
all pre-existing surfaces:

1. **Codomain of `state_stored_only_in`** — other kinds declare, per selector, which
   store holds their cells' truth. This is the `272` role member with its codomain
   generalized past its file-flavored origin:

   ```sh
   sm_dorc_Service__state_stored_only_in() {        # STRAWMAN throughout
      printf '%s\n' enabled : sm.dorc.File          # unit symlinks — the disk
      printf '%s\n' active  : sm.dorc.Boot          # the running incarnation
   }
   ```

2. **Ordinary object of `disturbs`** — what the sitting called a "transit" is nothing
   but an at-most footprint naming a store's cell:

   ```sh
   reboot__disturbs() { : disturbs sm.dorc.Boot ;}
   ```

3. **Containment via `disturbance_reaches`** (+ the `30U` finished-definition gate) —
   the store nesting order (Machine ⊃ Disk ⊃ Boot ⊃ Session) is entailment between
   store-kinds, in the emission form that family already has.

4. **Incarnation-identified entities, measurable as cells** — a store's entity IS its
   incarnation, and the world already mints these identifiers per class: `boot_id` per
   boot, `machine-id` per disk-life (known cloud-image-duplication wart), the
   provider's instance-id per machine, logind's session UUID per session, the pod UID
   per pod. The stdlib ships trivial reads for them (`sm.dorc.Boot@current` etc.).

Strawman stdlib store-kind roster: `sm.dorc.Machine` · `sm.dorc.Disk` · `sm.dorc.Boot`
· `sm.dorc.LoginSession`; `sm.dorc.File` re-read as the universal fine store it always
was. `File` and `Machine`/host are the two *universal* stores — which is the algebraic
reading of the standing expectation that File and Host end up the two engine-blessed
types (`26M:ack-hosts-blessed-someday-fenced-now`): the blessing is the engine knowing
those two store-kinds specially (sh semantics; transport/security), while this algebra
stays fully referentially agnostic.

## §2 — mechanisms owed (the build inventory)

1. **`state_stored_only_in` codomain generalization** — per-(kind × selector) → a
   store-kind coordinate. Kind-owner speech; silence keeps today's behavior.
2. **Backing CHAINS + transitive disturbance closure** — ENGINE EXTENSION. A fact's
   backing becomes a rooted chain (`Service@active → Boot:"a41f" → Machine:"i-123"`);
   a fact is reached by a disturbance iff the claimed footprint touches ANY node of
   its chain. Chain-disjointness under finished definitions is the survival license —
   the *existing* stage-5 intersection, closed transitively; flag, attribution, and
   double-opt-in unchanged.
3. **Incarnation cells + the continuity witness** — the stdlib store-kinds' identity
   reads, captured at probe, asserted at apply standup and after any fired
   store-disturbance, in the `plans/30S` witness-envelope shape. INTEGRITY-PLANE
   FENCED: mismatch ⇒ withhold (the unplanned-churn carve), never a verdict, never a
   license input. This is deliberately not a general assert-any-fact-at-apply surface
   — that would be the TOCTOU-freshness engine through the back door; the standing
   fiat stands, with the recorded tie-in that any future freshness work rides THIS
   mechanism rather than a parallel one (`26M`).
4. **Dead-chain unconsultability** — a fact whose chain names a superseded incarnation
   entity is UNREACHABLE by keying (the `26C` key-don't-filter pattern), never
   found-and-rejected. Expected-sever also derives here, not as a special executor
   flag: a dispatch whose footprint includes a store backing its own entry path is
   self-severing — sentinel-absence at that site is the success shape.
5. **Chain- and view-qualified comparison** — ENGINE EXTENSION, from the namespacing
   check: entity strings are LOCAL coordinates; absolute identity is the rooted chain;
   and resolvers inherit measurement-in-the-denoted-context (`readlink -f` inside a
   chroot answers differently than outside — view-aliasing: chroots, bind mounts,
   symlinks are same-cell-different-name, distinct from different-chain-same-string).
6. **Quality/lint riders** (kWARN-era): a `state_stored_only_in` referencing a kind
   with no store role ⇒ fail-fast contradiction-class (typo'd kind; loud beats silent
   value-evaporation); a predict body invoking its own family's store-disturbing verb
   ⇒ probe-fires-the-havoc hazard; the coverage nudge ("N kinds lack storage
   declarations — a fired Boot disturbance walls their cells").

## §3 — how it functions in practice (the patch-day compression)

The purpose-ordered pivot book (create → secure → update → conditional reboot →
configure → apps; full strawman in the `26M`-era chat, oracles as §1). On patch day
the reboot fires:

- `apt-get`/`dist-upgrade` run (diverged — the work).
- The reboot site runs; its footprint is one cell: `sm.dorc.Boot`. Expected-sever
  applies; post-wait, the continuity witness re-reads `Boot@current` and sees the new
  incarnation — the planned disturbance occurred (unplanned mismatch would withhold).
- Downstream, per chain: `Package@installed` and `File@content` chains root through
  `File`/`Disk` — disjoint from the fired cell — **elide, surviving** (under
  `kSURVIVAL`, attributed to the storage declarations by name). `Service@active`
  chains through `Boot:"a41f"` — dead chain — its half of an `enable --now` site
  re-measures live: **guard**. Volatile-only sites **run**.

All three grades appear, each for the right reason, and the why-chain's naked link is
now a *storage* claim ("active lives in the boot") — documented, concrete, and
auditable in a way behavioral survival claims never were. Without the §2.2 license
half built, the same book renders today's honest total wall below the reboot; §3's
witness and narration halves function regardless.

## §4 — what consumes the license plane, and its standing status

Chain-survival is the `kSURVIVAL` tier with chain-shaped footprints — not a new trust
cell. It stays behind the existing flag and full attribution; it is additionally
DEFERRED on economics (human-typed lean, 2026-08-31), with the named revival trigger:
the patch-day everything-book class (`26M`'s Class A — a routinely-firing mid-book
store-disturbance above a large persistent-kind tail), where the attention product
otherwise dies every maintenance window. The duller-knife note carries to the eventual
gate design: storage declarations restate the described tools' own documented
persistence contracts, the best-grounded claims in the survival family.

## §5 — laws and candidate law-forms (named for ratification, NOT ruled)

- **law-names-are-view-relative** — names are view-relative; identity is chain-rooted;
  minted-unique entities (boot_id, instance-id) are the bridge. Corollary: addresses
  (hostnames, paths, ports) are names-in-a-view, never identities — the algebraic root
  of `forfeit-no-host-merging`'s inverted knife.
- **law-event-time-only** — the model contains event-time as identity-plus-order and
  NO metric time. Metric time is either AUTHORED (clock-reading guards — `[ -nt ]`,
  `-newermt`, expiry judgments — spelled in sh, judged by their authors) or PARKED
  (`an-freshness`/`kSTATE` fences; TOCTOU fiat). Named reentry points, so the
  reduction never over-claims: wait/deadline modeling, planning-duration-as-staleness,
  any freshness unpark (which rides §2.3's witness mechanism).
- **law-exits-are-never-inferred** — store-disturbances fire only where CLAIMED. Every
  ssh dispatch truthfully ends a login session; an inferred session-havoc would wall
  the world at every dispatch boundary. Silence about an exit is not a wall; walls
  come from claimed disturbances and unmodeled mutators, as ever.
- **law-dual-speakers** — actor-side narrowing (`disturbs`, by the command's author)
  and patient-side narrowing (`state_stored_only_in`, by the cell's kind-owner) are
  duals; each party speaks only their own knowledge. The frame-problem division of
  labor that makes the composition safe.
- Knife notes: a wrong storage declaration under-executes (same knife as a wrong
  footprint; more auditable); a wrong incarnation-entity choice (hostname-as-entity)
  reopens ABA — minted identifiers are the discipline.

## §6 — place in the planning-whole

- DISSOLVES `26M`'s `hole-axis-identity-grounding` (no second grounding namespace
  exists to design) and the sitting's interim spellings (`: transits <axis>`,
  `outlives`, region-declarations) — superseded, trail in `26M`.
- The pivot-book arc consumes it: world-keying = the chain root; the `26M` payload/
  custody/entry drills continue on top of it unchanged.
- Session-store subtleties recorded: its main value is cell-scoping and narration
  (`@effective-groups` stored in the session ⇒ the `usermod`-then-relogin dance
  renders honestly), not havoc. File-as-store-interior (held-fd / loaded-config
  outliving file replacement — the reload phenomenon) is REAL and deliberately
  unblessed; the someday-home for reload semantics.
- Open asks for the human at ratification: the stored-in coverage burden (every kind
  eventually owes one line per famous store — consistent with silence-licenses-nothing
  economics, but it is the one place the model taxes every kind forever); the §2
  extension pricing; the stdlib store-kind roster and its incarnation-cell reads.
