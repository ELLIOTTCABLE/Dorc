# 272 — Address-derived context topology: the `kind__state_stored_only_in()` design

AI-authored (Fable, the `270:block-settle` rubber-duck sittings, 2026-07-10/11), minted
at the human's direction as the comprehensive durable of the task-3
(adj-trichotomy-spelling, né `24S:A3`) arc — the design was judged "large enough to
deserve a separate document, not a sub-heading of 271." Note-tier, but kept-current
through block-settle (the stage-spec precedent, `24A`/`24D`-class). Authority:
root docs and the `271` rulings ledger outrank this; ratification status is MIXED and
marked per-section (§12 is the status table — read it before citing anything here as
settled). Companions: `plans/271` (the rulings; its task-3 entries now summarize and
point here) · `notes/27x-strawmen-topology/` (the five worked real-world examples) ·
`plans/24S` (the wrapper-context keystone whose kind-side this design answers).
Naming per `271:rul-at-most-family-names` and `271:rul-class-prefixed-role-names`:
`cmd__disturbs()` (né touches) · `kind__disturbance_reaches_only()` (né reaches) ·
`kind__state_stored_only_in()` (né the `lives_at` strawman). All sh is STRAWMAN except
where a name is marked ratified.

## §0 — The problem (two lines of sh)

```sh
dpkg -s nginx >/dev/null 2>&1 || sudo apt-get install -y nginx
sudo crontab -l 2>/dev/null | grep -q renew || printf '0 3 * * * renew\n' | sudo crontab -
```

Probes run read-only as the invoking user (imp-1: probes never escalate). May an
alice-measured fact license skipping a root-context command? Line 1: yes — one dpkg
database, whoever asks. Line 2: never — alice's crontab and root's crontab are two
objects sharing a spelling. The wrapper behaved identically; the difference is
knowledge about the NOUN. So the declaration attaches to the kind, not to sudo. This
document is how that knowledge is authored, derived, consumed, and bounded.

Distinguish throughout: **ρ** (value-plane — what strings compute under an
environment; sudo's HOME-retarget making `$HOME/.gitconfig` compute differently) vs
**context** (identity-plane — whether the same computed name denotes the same object).
ρ is substitution; context is aliasing. Much apparent per-user-ness is just ρ (§7);
the surviving identity problem is confined to LOGICAL entities whose stores hide
behind binaries.

## §1 — The formal object: one relation, many generators
*(status: conductor-proposed as the entity-algebra design note's formal spine; unruled)*

Everything funnels through one comparison (the seam
`271:rul-seam-context-slot-and-relational-chokepoint` reserved):

    compare(cellA@ctxA, cellB@ctxB) ∈ { same(coord) | provably-disjoint | unknown }

Consumers: **same** → transport (a fact measured in one context licenses elision in
the other) and the probe-outside license; **provably-disjoint** → survival
(kill-traffic sparing); **unknown** → the safe bottom (no transport, collide, walls,
run). Every authored surface — `kind__resolve()`, the dialect-scoped selectors,
this design's member — is a GENERATOR of verdicts feeding that relation.
Load-bearing property (the answer to the microtypesystem challenge):
generator-INCOMPLETENESS is value-loss only, because the default is the safe bottom;
the bite classes are generator-INCORRECTNESS (the attributed knife) and
CONTRADICTION (the standing refuse-both category). The design note owes a one-page
spec (kVERIFY-calibrate tier: spec, not proof) of the relation, the consumer map, and
the generator registry (surface × verdicts-it-may-generate × license tier).

## §2 — The authored member
*(status: NAME ratified/typed 2026-07-11; the member's existence rides that
ratification and the bake-into-respell directive; substrate marks are
conductor-proposed with the v1 reserve-the-slot move)*

One sh function per LOGICAL-entity kind, written by the kind-owner (`rul-family`'s
KIND species; registry per `rul-seam-kind-owner-registry-room`; extend-by-new-name —
this is a NEW member, deliberately not a re-read of `kind__disturbance_reaches_only()`,
see §10). Calling convention (strawman): `$1` = entity, `$2` = selector (absent ⇒
whole-entity). The body emits **(locator, substrate) pairs**: the locator is ordinary
sh that LOCATES the store; the substrate is a trailing mark from a small,
engine-owned, dialect-versioned vocabulary naming the address-space the locator is
interpreted in (strawman: `: fs` default · `: kernel` · `: net-kernel` · `: process`
· `: endpoint`; governance identical to the axis vocabulary — users never mint).

```sh
# dorc-lang/v0.1

sm_dorc_Package__state_stored_only_in() {
   printf '/var/lib/dpkg\n'                                    : fs
}
sm_dorc_Cron__state_stored_only_in() {
   printf '/var/spool/cron/crontabs/%s\n' "$(id -un)"          : fs
}
sm_dorc_Firewall__state_stored_only_in() {
   printf 'nft-ruleset\n'                                      : net-kernel
}
```

Locator forms, all ordinary sh: constants; who-am-I-parameterized paths; QUERIES
(`find /var/spool/cron/atjobs -user "$(id -un)"` — the honest spelling of an
ownership-filtered subset); dynamic host-question arms (`$(brew --prefix)`-rooted —
run read-only at probe time, the stage-5 dynamic-frame precedent), which are
classifiable only via a capture-claim (§11, the task-7 coupling). Static arms are
traced; per-platform `case` arms and `return 2` declines are ordinary control-flow.

**The `only` contract.** The name carries the quantifier deliberately
(`271:rul-at-most-family-names`): this member is complete-by-contract — the author
must survey the kind's stores totalistically before authoring at all. The epistemic
reason it earns `only` most strongly in the family: its key consumer reads the
emission's NEGATIVE space. Invariance is a universally-quantified absence ("no
who-am-I ingredient appears anywhere in the total emission"), so a partial store-list
is not partially trustworthy — it is meaningless. Contrast `cmd__disturbs()`, whose
at-most claim is scoped per matched invocation-shape and whose coverage may grow
verb-by-verb.

## §3 — The derivation
*(status: conductor-proposed; the D-direction it implements is
ratification-implied by the naming bake; wants one cheap explicit ack)*

The engine traces the body (ordinary oracle-body tracing, under the site's ρ) and
derives per-axis topology by two rules:

- **r1, the carried-by table** (engine-owned, closed, versioned): substrate × axis →
  reinterpreted-or-invariant. `fs` is carried by fs-view (chroot/bind/overlay swap
  the whole address space under you — nothing textual to mine, which is why the
  substrate mark exists); `kernel` is not (the sysctl inversion: `/proc/sys/...`
  looks like a path, denotes host-global kernel state); `net-kernel` is carried by
  netns and NOT by user or fs-view; `process` by pidns; `endpoint` RECURSES — the
  socket's own locator derives through these same rules, and everything beyond the
  endpoint is declared opaque (§8). Future-proofing, the design's strongest
  robustness property: **new axes arrive as new table rows consuming declarations
  that already exist** — when netns lands, every `: net-kernel` store on earth starts
  deriving correctly with zero author rework.
- **r2, emission-set non-interference** (for axes with visible ingredients — user, at
  v1): does the SET of emitted lines vary with the axis's labeled inputs? The blessed
  who-am-I labeling: `$HOME`, `$USER`, `$LOGNAME`, and the captures
  `$(id -un)`/`$(id -u)`/`$(whoami)` (mapped to the context's user-axis value, never
  executed for it). Function-level, not string-level — a `find … -user "$(id -un)"`
  query varies the set through its SELECTION and is caught identically to a
  parameterized printf. This is the DCC/taint lineage; AGENTS' own "principled,
  contracted control-flow-tracing and tainting."

Outcomes, per (kind[, selector] × axis):

- **invariant** (no dependence, over the `only`-total emission): buys BOTH the
  identity bridge (transport: alice's fact speaks for the root site — the `24S` §2
  headline; the admin's own `dpkg -s … || sudo apt-get install` guard lifts through
  the boundary) AND the probe-outside license (same referent ⇒ the outside read is
  legitimate). One declaration, both uses (`24S` §6a preserved).
- **keyed** (dependence present): cells are NAMED per axis-value — `cron@alice` and
  `cron@root` are different cells, the naming falling out of evaluating the authored
  locator under each context's ρ. Keying is a re-indexing, license-free; the
  sudo-crontab footgun becomes uncommittable by construction (the cells never shared
  a name). Keying licenses NOTHING else — see §4.
- **⊤ / silence / untraceable**: the floor. No license of any sort; may-alias; walls.
  Note the degradation direction: candidate B (an sh-bodied classification function)
  degraded to a fuzzy LICENSE; this degrades to NO license. Cleverness costs the
  author value, never costs anyone else correctness.

## §4 — The carve: never-derive-separation
*(status: conductor-proposed, upgraded to demonstrated-necessary by a live
counterexample; wants explicit ack — it is the design's sharpest safety line)*

"Per-user" smuggles two claims: *different names per user* (keying, safe) and
*guaranteed non-overlap* (separation — what survival consumes, knife-tier). The
derivation yields keying and **never separation**: address-inequality is not
referent-inequality (paths alias; addresses can be host-conditional). The validating
counterexample (`27x-strawmen-topology/05`): on a DEFAULT-INSTALL docker host, alice
and root reach one daemon through one socket; deriving separation from the
per-user-looking rootless branch would let a probed fact "survive" root's
`docker system prune` while the prune destroys its backing — silent under-execution
in the most ordinary configuration docker has. Separation across context-values
arrives only ever as a DECLARED, owned act with the same minting care the
selector-dialect ruling demanded within entities — and at v1 it does not exist at
all. Cost of the carve: only the within-kind cross-context disjointness dividend
(cross-kind disjointness stays free by construction; credential-capped sites stay
guarded via imp-1 regardless). Terminology proposal for the design note (unruled):
retire the conflating token `sensitive` in favor of **keyed** (derived, safe) vs
**partitioned** (declared-only, knife).

## §5 — The fence: addresses are not coordinates
*(status: conductor-proposed; wants explicit ack — an unfenced build constructs a
parked mechanism by accident)*

Emitted locators are consumed EXCLUSIVELY for the per-axis dependence bit and the
per-value keying recipe — never turned into file-coordinates, never intersected
against `sm.dorc.File` facts, never used for address-granular disjointness. Two
reasons: shared stores (every Package cell lives in `/var/lib/dpkg`; address-level
comparison would merge them all and over-wall catastrophically — the coordinate
abstraction exists because it is FINER than the physical store); and cross-kind
identification (`cron:alice ↔ file:/var/spool/…/alice`) is the parked co-reference
mechanism, which the corpus insists is designed once (`24M:rul-kind-unify-owed` ·
`24C:strain-coreference-crosskind` · `24S:A5`) — this member is registered as the
FOURTH pointer at it and becomes its authored INPUT when it unparks (one surface,
staged consumers). Pleasant corollary (the narrowed knife, §8): since invariant
locators are read only for dependence-shape, they may be APPROXIMATE (`/run/systemd`
standing in for pid-1 memory is fine); the exhaustiveness obligation narrows to
"never forget an axis-DEPENDENT store."

## §6 — The axis roadmap
*(status: axis vocabulary TYPED (`271:rul-axis-vocabulary-v1`); the netns re-scope
TYPED (`271:rul-networking-unpunt`); the rest is that ruling's conductor riders)*

- **user** — v1, the built axis; r2 carries it.
- **netns** — un-punted 2026-07-11 (containers make network-naming core 2026
  single-host ops), sequenced AHEAD of full fs-view: `ip netns exec NAME` yields
  argv-named axis values exactly like `sudo -u USER`; the `net-kernel` substrate has
  no aliasing ladder (no symlink/bind/copy-up analog); cross-namespace state is
  disjoint-by-construction. Serves the ip/nft/sysctl tool class; docker-MANAGED
  networking stays endpoint-opaque. Axis is naming/scoping only — no network access;
  no-cross-host (`24S:imp-5`) untouched.
- **fs-view** — soft-deferred (its own round someday; spike stub `fs-straw`).
  Covered meanwhile BY REFUSAL: the contract locates a store within a space and never
  claims path-identity across views — chroot re-roots, bind-mounts re-route, overlayfs
  merges, and all three sit at the may-alias floor until an authored map bridges them.
  Copy-up (a write MOVES the referent to the upper layer) is why bridge-maps stay
  hard and deferred; the `24S` §3b honest ladder stands.
- **endpoint** (not an axis — a substrate): state behind a daemon socket. The
  locator's own address derives (docker's socket-selection logic is spellable and
  derivable); beyond the socket is opaque, frontloaded (§8).
- **pidns / utsns / ipc / cgroup / time** — representable in the substrate frame;
  rare-tail; rows land if/when their axes ever do.

## §7 — The burden map (who writes what)

Admins: nothing, ever — wrapper-heavy books lose value (walls with named hints),
never execution fidelity, and their own hand-written guards are what invariance lifts
through boundaries. Everyday tool-oracle authors: nothing — no wrapper-awareness, no
topology-awareness (the `24S` §2c referendum extends). Tools whose entities are
RESOLVED ADDRESSES need no kind topology at all: bind the ρ-resolved path as the
entity and the value plane keys cells for free (the git-config dissolution,
`27x-strawmen-topology/02` — `sudo git config --global` keys to `/root/.gitconfig`
with zero declarations). The bootstrap file-kind's grounding is ENGINE-SUPPLIED
(an authored identity function is cargo-cult). Only LOGICAL-entity kind-owners — the
`resolve()`/`reaches()` cohort, <10% of authors — write this member, once per kind.
Behaviour-menu addition (extends `271:rul-kind-or-selector-is-a-behaviour-choice`):
logical entities buy cross-tool collaboration at the price of owned topology;
address entities are collaboration-poor but topology-free. Mixed-topology tools
(systemd system vs `--user` units) resolve by argv-driven KIND-binding in the tool
oracle — kinds are the topology unit; minting a kind is also choosing a topology.

## §8 — Failure modes and the frontloaded limitations
*(the README-class list, per `271:rul-lint-never-drives-design` — these are the
product's stated constraints, not lint fodder)*

- **The knife (the one way this surface under-executes):** an omitted AXIS-DEPENDENT
  store — a kind declared with no who-am-I anywhere whose tool also keeps per-user
  state (the pipx-in-`~/.local` shape). Wrong invariance → wrong transport → a
  sudo'd line elides that needed to run. Attributed to the member's line; same tier
  as the rest of the at-most family. The narrowed form (§5) is the author's whole
  obligation.
- **Ownership-multiplexed shared stores** (the at-spool class: one spool,
  per-user ownership, per-invoker views): the per-user-ness may not textualize as a
  path. The honest native spelling is a QUERY locator (`find … -user "$(id -un)"`),
  which r2 catches; where even the author cannot spell the filter, the contract says
  DECLINE. Frontloaded sentence: *this member speaks to where facts are recorded,
  never to who a tool chooses to show them to.*
- **Endpoint opacity:** multiplexing beyond a daemon socket (peer-auth'd rows,
  per-caller daemon views) is invisible to this surface, permanently. The socket's
  own address participates; the daemon's interior does not.
- **Untraceable/clever bodies, unclaimed captures:** floor. Value lost, author's own.
- **A lying wrapper upstream** (wrong ρ/axes declaration): wrong context feeds the
  derivation; attribution lands on the wrapper's line via the `24S` §4a chain, not on
  the kind-owner.
- **What this surface cannot break, by construction:** separation (never generated),
  other kinds (cross-kind disjointness is independent), the apply artifact (analysis-
  side only; user bytes verbatim).
- **Referent vs access** (postgres, `27x-strawmen-topology/03`): invariance speaks to
  the referent; ACCESS is per-probe, enforced by rc-reality (license ≠ ability —
  a licensed probe that gets peer-auth-refused answers ≥2 ⇒ can't-say ⇒ run). The
  imp-1 cap composes gracefully; no interaction with the declaration.

## §9 — Verification posture
*(per `271:rul-net-quality-u-curve`: documentation is the design-tier artifact;
mechanical nets below are OUR verification, never the product's rescue)*

The derivation is officially **conjecture-generator + differential-checker**: the
mutate-as-A/probe-as-B differential (`24S:A4`) is the derivation's load-bearing other
half — it mechanically falsifies wrong invariance (create a root at-job, probe as
alice, watch the fact not flip), which declared tokens never allowed. Runs as stdlib
CI over every shipped kind (kVERIFY-calibrate). DST grows a lying-address sweep axis
beside lying-topology/lying-peel. Invariants test-pinnable from day one: silence
never identifies; ⊤ identifies with nothing; every cross-context elision renders its
four-link chain; keying never feeds survival; rung-0 books byte-identical to HEAD.

## §10 — Naming (ratified) and the collapse record

Ratified 2026-07-11 (`271:rul-at-most-family-names`, `271:rul-class-prefixed-role-names`):
**`cmd__disturbs()`** · **`kind__disturbance_reaches_only()`** ·
**`kind__state_stored_only_in()`**; the `only`-rule (`only` = complete-by-contract,
totalistic-survey-before-authoring; absent = arm-incremental); class-prefixed role
names in all documentation; the chain sentence as the teaching frame: *a command
disturbs cells; that disturbance reaches only what the kind-owner enumerated; a
kind's state is stored only in the substrates its owner declared.* Rides the
corpus-respell (`270:block-rebuild` brief rider). Unruled remainder: `predict`/
`resolve` menus (delivered in-chat 2026-07-11); wrapper-member names (task 5).

Collapse/identity record (why this is a new member and where it lands): NOT
`kind__disturbance_reaches_only()` — the rhyme was a single-file-store coincidence
(cron's store = its own reach; dpkg splits payload/causal-edges from the fact-store);
one-member-with-arm-marks declined (two contract texts in one body). YES downward —
the implementation is a pure consumer of scheduled block-rebuild machinery
(value-recipe-reshape for un-collapsed locator recipes; the relational chokepoint;
the backing-SETS seam): new authored surface, zero new engine lanes. NOT the
measurement-line lane — per-probe read-set disclosure as an invariance carrier is
open-world enumeration (the opaques7-finding20 objection-class); observe-disclosures
stay the safe widener (`271` observe-backing-widening thread).

## §11 — Open couplings and deferred value

- **task-7 (adj-capture-claim):** dynamic locator arms need an AXIS-INDEPENDENCE
  value-bound in the read-blessing vocabulary (`brew --prefix` output is
  user-invariant) — task 7 has a second customer; couple the sittings.
- **task-8 (adj-survival-flag-outcome):** its adjudicability condition must be
  re-read against a DERIVED (not declared) clause; note this member's transport
  license is the one whose tier is unruled — if vouch-tier wins, it is the only
  at-most member whose wrongness bites un-flagged.
- **task-12 (entity-algebra design note):** imports §1's one-page formal spec, the
  §5 fence, the substrate-mark slot reservation, and the keyed/partitioned
  vocabulary decision.
- **Per-host topology refinement** (probe-time evaluation of host-conditional
  branches — rootless-absent ⇒ invariant on that host): real future value,
  chronology-priced (a measured branch condition must itself survive walls);
  deferred alongside task-7's planes-meet-at-chronology cell.
- **Candidate A (the minted static clause):** demoted to named fallback, narrow but
  inhabited (capture-rooted kinds wanting a cheap assertion before task-7 machinery
  exists — homebrew); resist reopening except on field evidence
  (two-mechanisms-for-one-act).
- **The adversarial crosscheck** (`270` §6, exclusions-not-inclusions, conductor's
  self-flagged weak points stripped from the packet): runs against the entity-algebra
  note with this design in it; the containers lens rides it (conductor task 13).
- **The substrate-vocabulary mini-adjudication:** the mark set is new annotation
  vocabulary (engine-owned, closed, kOOB-adjacent) — v1 reserves the slot; the
  vocabulary itself gets its deliberate reading when fs-view or netns consumption
  arrives.

## §12 — Status table (what is settled vs riding vs proposed)

| component | status |
|---|---|
| the three names + `only`-rule + class-prefix + respell rider | TYPED 2026-07-11 |
| the member's existence / D as the direction | ratification-implied by the naming bake; blanket ack welcome |
| axis vocabulary v1 {user, fs-straw}; netns un-punt ahead of fs-view | TYPED (`271:rul-axis-vocabulary-v1` + `271:rul-networking-unpunt`) |
| lint-never-drives-design; net-quality-u-curve (govern §8/§9 framing) | TYPED, standing |
| substrate-typed emissions + carried-by table (reserve-slot v1) | conductor-proposed |
| emission-set non-interference + who-am-I labeling | conductor-proposed |
| never-derive-separation carve | conductor-proposed; docker-validated; ack wanted |
| addresses-are-not-coordinates fence + fourth-pointer registration | conductor-proposed; ack wanted |
| relation+generators as the design-note formal spine | conductor-proposed; unruled |
| keyed/partitioned terminology | proposed; unruled |
| engine-supplied file-kind grounding; behaviour-menu addition | conductor-proposed |
| differential-as-other-half; DST lying-address axis | conductor-proposed (verification posture) |
