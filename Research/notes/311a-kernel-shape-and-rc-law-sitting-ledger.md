# 311a — Kernel-shape sitting: index-kinds (W-2), the rc law, and the oracle authoring surface

> AI-authored LEDGER (Fable, design-rubber-duck sitting with the human, 2026-09-05; the
> r31-prep session). Notes-tier. Grades: **[TYPED]** the human typed the substance ·
> **[ACKED]** put and acked as read · **[LEAN]** the human's stated lean · **[PROPOSED]**
> conductor synthesis awaiting the human's word · **[FOUND]** a finding, not a ruling.
> Nothing is ruled unless TYPED or ACKED. Authority: root docs, `spike/CLAUDE.md`, the welds,
> `plans/30W`, `plans/30T`, `plans/30U`, `plans/27C`, `notes/26Ob` outrank this. Scope: the
> kernel-shape material of the pre-r31 sittings — the `26Ob:list-must-sits-before-kernel-work`
> W-2 dig, the exit-status finding it surfaced, and the oracle authoring-surface law that fell
> out. NOT transport: the `26N`/`26O`/`26Ob` lineage returns in its own round (probably `311`).
> Kept deliberately shorter than `26Ob`; each section is one sitting-turn; append new turns as
> new sections, keep §-last as the state at close. Where this ledger and `26Ob` overlap, `26Ob`
> §15 carries the earlier state and this file the later.
>
> **A cleanup pass is OWED and not started** (human-directed 2026-09-05: ledger now, apply to
> the existing docs later, in one pass): see §7 for the list. Nothing outside this file, the
> two `26N` superseded notes, and the README map entry was edited this sitting. `plans/310` and
> `ROADMAP.md` are the sibling conductor's and were left alone by direction.

## §0 — one screen

W-2 (the `30W` §10 sitting widened by 26Ob's q3/q8/q10/q11) reduces to three shape rulings,
most of whose substance the human had already typed: the context slot is a map over
index-kinds with no engine axis list (S1); index-values are value-plane objects carrying
provenance, with a per-entry-chain placeholder as the new object (S2); worlds relate only
through the compare chokepoint, never key equality (S3). The S1 exploration found that no
compose operator is needed anywhere — the key is the entry chain, and every step toward `same`
or `disjoint` is authored — and that two 27C rules (fs-view path concatenation; enumerate-every-
dimension) should retire. `kind__overlaps` was ruled IN as a kernel generator (§4). Under it the
sitting found a real hazard: `30W`'s `overlaps` put its knife on rc 1, the status errexit and
every false test produce; the human then typed the rc law (§6): oracles always return 0,
non-zero means erroneous and speaks nothing, knife-tier acts are records with a completion
sentinel, convergence and predict stay rc-bearing because they have off-ramp value. §7 works
`__predict`'s fail-safe; §8 records the human's oracle shell-state principle.

## §1 — register touch-ups done this sitting, and one nit

- `notes/26N` §7 and §10 carry superseded notes: the engine-side "simple words are identity"
  carve is WITHDRAWN (`26Ob:ack-charset-is-tool-knowledge`); `park-re-parse-carve-explainer`
  points at `26Ob` §1. `Research/README.md` maps `notes/26Ob` with a grep-first caveat.
- `nit-no-sugar-over-stdlib-kinds` [TYPED 2026-09-05] — if index-kinds are kinds, they get
  `sm.dorc.*` names in every spelling; no `user`/`fs`/`net-kernel` sugar for now. `30W` §1's
  "short tokens remain as sugar" and item 7's "sugar preserved" read under this.
- `note-transit-mechanics-are-open` [TYPED 2026-09-05] — the actual mechanics of transit are
  fairly open (whatever is most idiomatic first; likely responsive to configuration and target
  limitations; a which-comes-first question); the sh semantics and user attention matter, and
  errexit and `$0` have repeatedly been painful to honor without stomping intent. So `26N` §2's
  in-memory `eval` spelling versus plan.md's `sh -c "$(cat …)"` is not a question; leave both.
- Host blessing [LEAN 2026-09-05] — the human leans that Host need not be engine-blessed if
  transport routes through userspace (the stdlib ssh oracle); File stays blessed (sh semantics).

## §2 — W-2 restated: the three shapes, where each stands

- **S1 `shape-slot-is-a-map-over-kinds`** — the slot is a finite map from index-kinds to
  index-values; an absent kind inherits the caller's value; the empty map is the ambient world;
  there is no engine list of axes. Substance TYPED by `26Ob:ack-entered-kind-is-declared-never-host`
  and §1's Host lean. Absorbs `30W` ruling 1 (`271:rul-axis-vocabulary-v1`'s reserved `host` has
  nothing to be reserved in) and ruling 4 (dissolve the closed vocabularies). Vantage, if ever
  needed, is one more kind; nothing to rule. The ρ component (env, cwd, options) rides beside the
  map as an engine-owned builtin, not a kind (§8).
- **S2 `shape-index-values-carry-provenance`** — Entered (a value-plane object that flowed
  through a `lend_map`; never a string — `26Ob:nit-entered-arm-holds-a-value-object`) · Measured
  (a placeholder `sk(E, K)`, "the value of kind K under entry chain E", until the standup binds it
  to an opaque token) · Fresh (⊤). E = peeled spelling × vantage × the entry form's
  `DefinitionId` (`26Ob:nit-batching-key-carries-entry-definition-identity`). PROPOSED, acked
  in direction. The placeholder's binding state is a license input (an elision on an unbound
  placeholder is guard-ceiling unless the standup binds it — `26Ob:ack-guards-reach-elisions-witness`).
- **S3 `shape-worlds-compare-through-the-chokepoint`** — key inequality is `unknown`, never
  `disjoint`; equal maps are the cheap `same` pre-filter. Substance TYPED (`26Ob` §10b
  `hard-thing-two`; `ack-cross-world-wall-is-the-floor`). The filtered meet (per index-kind,
  consult the cell's owner's store trichotomy invariant/keyed/⊤; skip invariant, compare keyed,
  ⊤ ⇒ unknown; meet) is PROPOSED and additive. `30W` ruling 2 is the referent-transparent row of
  the relation table; ruling 6 its authoring contract.
- `30W` ruling 3 (`rule-incarnation-invariance-passes-the-razor`) — the sitting's reading
  [PROPOSED, confirms `30W` §8]: an invariance line against a tenure/Boot kind is vouch-tier for
  TRANSPORT (the cell is one cell across the index — a closed claim about the owner's own store,
  like user-invariance) and flag-tier for SPARING past a reboot that actually fires
  (`26Ob:fnd-below-a-fired-transit-three-rungs`). Two roles where the user line has one.
- `30W` ruling 5 (`_only` decomposes) — the human's own 26M nit, adopted, never typed as a
  ruling; shape-neutral; may wait for the stdlib arc.
- Tenure versus Boot [PROPOSED lean] — exclude folding the tenure token INTO the entered kind's
  value (structure inside the coordinate, the shape `271:rul-coordinate-shape-flat-three-place`
  declined); whether Boot is a separate stdlib kind the entered kind's owner entails
  (containment via `reaches`, `30W` §4) is stdlib content, not a kernel shape. The word itself
  (`tenure`/`incarnation`) is renamable pre-publication and blocks nothing.
- W-3 (`26Ob:lean-replace-wrapper-detection-with-argv-landing`) — one sentence, unchanged,
  foldable: wrapper-ness is where the site's argv lands through the predict body's modelled
  transformations, not a category detected by `"$@"` in command position.
- Builder rider, not a ruling: with Host entry punted, the empty map means "the CLI target"
  in r31 and flips to "the controller" when the synthetic head lands; nothing persists, one
  seat; the context-slot audit brief should say so.
- Minimal W-2, under the less-work goal: type S1–S3; let `30S` build on the two-variant slot
  (`310` §4b); everything else above waits for the transport round.

## §3 — the S1 exploration: no compose operator; the key is the chain

The question put: is innermost-wins a sufficient compose op for the context slot, or is
composition userspace-defined? Answer [FOUND; conductor-argued, human-read]: innermost-wins is
right for what a link SETS and unsound as the KEY; no compose op exists; what userspace defines
is stores and identity. Four rules:

- `rule-set-innermost-wins` — a link's `lend_map` names the kinds it sets; innermost wins.
- `rule-inherit-or-top-with-sentinel` [PROPOSED] — kinds a link does not name are ⊤ (27C's
  posture, kept) until the wrapper author finishes the claim with a `lends nothing-else`
  sentinel, after which unnamed kinds inherit the caller's value. A second, `env -i`-shaped
  sentinel says "reset unnamed kinds to the new world's ambient" (the ssh shape). Both are the
  author's claims. This REPLACES `27C` §3's enumerate-every-dimension law, which cannot survive
  user-minted kinds (a `nice` oracle cannot enumerate `org.docker.Container`); it mirrors
  `271:rul-env-claim-inversion`'s ladder and 30U's at-most pattern.
- `rule-derive-only-toward-unknown` [FOUND; already law in two halves] — derivation may make
  a cell unknown or fresh (30W §4 transit re-keying: a disturbed index-value cell freshens
  keyed cells downstream), never `same` or `disjoint` (`272:never-derive-separation`;
  `271:rul-invariance-speech-act`). Corollary, correcting the sitting's own first cut: "entering
  a new Host resets Container to the new ambient" is a derived `same` with that ambient and is
  NOT license-free; it is the ssh author's reset sentinel, or nothing.
- `rule-same-is-generated` — across differently-keyed contexts, `same` needs a measured
  identity token (the owner's read) or a declared invariance of the name's resolution across
  the differing kinds; `disjoint` needs referent-transparent tokens that differ, under the flag;
  else unknown. Names are never arithmetic.

Corrections to `27C` this implies (cleanup pass): retire `rul-dimension-owned-compose-ops`'
fs-view caller-relative path concatenation (engine-held path semantics, forbidden by
`30T:rul-engine-holds-no-world-facts`, and wrong under a symlink inside the chroot); identity
across `chroot /mnt chroot /t` and `chroot /mnt/t` is measured by the fs-view owner (device and
inode of `/` inside), never concatenated. Retire enumerate-every-dimension per the sentinel
rule. `rul-top-absorbs-absolute-maps` survives as "⊤ at any link for kind K is ⊤ for the chain".

Examples that decided it (compressed; `26Ob`-style, all STRAWMAN): absolute kinds (`sudo -u`)
work under map-overwrite by luck · relative kinds break it (`chroot /mnt chroot /t` versus
`chroot /other chroot /t` both key to `/t` under overwrite — a wrong-world elision) · a
world-replacing link (`docker exec c ssh web1`) is the author's reset sentinel · a tunnelling
link (`ssh -A`) is an explicit inherit lend outranking any derivation · partial passthrough
(`docker run -v host:ctr`) is `overlaps`/measured identity, never a lend · a compose-file
container name is a MEASURED lend (why S2's Measured arm exists for lends) · fake identity
(`unshare -r` versus `sudo -u root`) is safe by the relation table's "one map has a kind the other
lacks ⇒ unknown" row even when the User owner declared nothing · `nsenter` sets two kinds from one
operand, nothing special · `ssh -J bastion` needs no vantage kind if Entered names key with the
calling context · env and cwd are ρ, engine-owned by sh parity, not kinds.

- `lean-no-absoluteness-declaration` [PROPOSED] — an owner-declared "my names are absolute"
  would recover pre-network transport for absolute kinds; the chain key costs one extra standup
  per distinct chain before tokens bind; not worth an authored surface now.

## §4 — stores, `overlaps`, and the ordering hazard

- The recast reading [FOUND, agrees with the human's recollection]: `state_stored_in` emits
  locators "stored in a kind" — under `30W` §1/item 7 and §1's no-sugar nit,
  `printf '/var/lib/dpkg\n' : stored-in sm.dorc.File`. No document had that spelling written;
  `30W` §3's and `26Ob` §10a's strawmen still use `: fs`/`: process`. The old token did two
  jobs: which index-kind KEYS the store (now by transitivity — File's own store declaration says
  File names live in `sm.dorc.MountNamespace`'s address space) and which cell kind the region IS
  (the File coordinate itself, for collisions).
- The subtree relation is `kind__overlaps` (`30W` §2/§3); the consumer is `30U` §7's
  store-collide rule. Absent `overlaps`, "inside" can only be exact-name equality.
- `hazard-file-finished-definition-leans-on-overlaps` [FOUND] — `30U` §7 lets File finish its
  definition BECAUSE other kinds' residency in files is those kinds' store speech; that honesty
  needs the store-collide consumer to find subtree containment. File's `disturbs nothing-else`
  is therefore unsafe to author until `overlaps` and the collide consumer exist. Not recorded
  anywhere the conductor read; the human did not recall it ruled.
- **`rul-overlaps-is-a-kernel-generator`** [TYPED 2026-09-05, conditional on the conductor
  finding no mechanism hole; none found] — `kind__overlaps` comes forward into the kernel
  organization: a generator of the compare chokepoint for same-kind region pairs and for the
  store-collide consumer's containment predicate; absent or declining reads unknown; consulted
  only within one context after the world relation; dynamic pairs collide at v0 (30T §9). Body-
  quality items for the stdlib bar, not blockers: `realpath` prefix checks answer disjoint for a
  hardlinked file whose other name sits inside the store (`-ef` or a link-count check belongs in
  the body); perishability is moot under the v0 entry-mutating-verb wall. The sibling re-homes
  it out of the identity tier.
- `rul-store-containment-is-may-inside` [PROPOSED, the carriage rule] — `30U` §7's "adds
  collisions against footprint cells landing inside it" must read as a MAY-inside predicate:
  unknown adds the collision; only the owner's deliberate disjoint answer removes it. Without
  `stored nothing-else` an unlisted store is open-world and collides with every footprint of that
  kind (`30W` §3), so truncation of a store list lands safe — which supersedes `30U` §5's
  "absent by design for store declarations" (it predates the sentinel).

## §5 — the universal safe default, and its two deliberate exceptions

- Every generator's absent or lazy answer lands on unknown, and unknown (and `unrelated`) is
  safe for both consumers: `resolve` absent ⇒ may-alias · `overlaps` absent ⇒ collide ·
  referent-transparency undeclared ⇒ names are names · identity read absent ⇒ placeholder never
  binds · invariance absent ⇒ keyed or ⊤ ⇒ no transport · store declaration absent ⇒ ⊤ ⇒ the
  filtered meet answers unknown · "one map has a kind the other lacks" ⇒ unknown · a wrapper
  silent on a kind ⇒ ⊤.
- Deliberate non-unknown defaults, both authored or ordinary law: (1) two sites under one
  syntactic entry chain share a placeholder pre-measurement — the ordinary reach rule applied to
  the name's resolution cell (⊤ backing unless the owner declares stores ⇒ any running mutator
  between them re-keys the lower; `26Ob:fnd-batching-key-is-syntactic-wall-is-rekeying`); the
  residue is outside churn, a declared horizon with the standup witness as backstop; (2) the
  wrapper's inherit/reset sentinel (§3).
- `note-transport-single-consented-sparing-double` [FOUND] — transport's `same` is vouch-tier,
  unflagged (an invariance line, a `resolve`, a bound token; bites the author's own consumers);
  sparing's `disjoint` is author plus flag. The razor by design; the 24S headline feature.
- `open-bound-token-same-across-chains` [OPEN; the human: committee-speech-adjacent, owed an
  entire turn with a very critical eye] — when `same` comes from two placeholders binding to
  equal tokens across DIFFERENT entry chains, is it a closed claim (vouch-tier, unflagged) or
  survival-grade? `26M`'s un-acked rider argued survival-grade for hosts;
  `FORFEITS:forfeit-no-host-merging` makes it moot at v0 for Host only; live for Container or
  User the moment a read exists. The turn must read `plans/28M`, `28K`, `30J` (not in this
  sitting's window; the law as steering states it — one closure's dialect, custody as one
  newtype, `rul-vouch-reaches-own-custody-only`, 30T §3.3 input separation — was). First read
  only: one speaker's read compared with itself, but the transport it licenses crosses custody
  (author X's fact in world A reaching X's site in world B on owner Y's read and entries Z1/Z2).
- `ack-names-are-literal-narratives` [TYPED 2026-09-05] — "every step toward same or disjoint
  has an author's name on it" is literal: each generator mints a Narrative at build time
  (`AID-NEEDS:law-collapse-mints-narrative`, `trust-tier-is-syntax`) and the name reaches the
  angry user at `why`-time.
- `note-enhancement-curve-survey-wanted` [human, 2026-09-05] — with `overlaps`, finished
  definitions, store sentinels, identity reads, and lends sentinels all ahead of cross-kind
  elision, USER_STORY has gained several oracle-author stages before anything elides; not
  necessarily bad (correctness is never traded for value without extreme motivation), but a
  full survey of the gradual-enhancement curve is wanted at some point. Not scheduled.

## §6 — the exit-status finding and the rc law

- `fnd-errexit-returns-the-failing-status` [FOUND] — errexit exits "as if by `exit` with no
  arguments", i.e. with the failing command's status: rc 1 for a false `[ … ]`, `grep` no-match,
  `cmp` differ, `findmnt` nothing; 2 for `test`/tool usage errors; 127 not-found; nounset trips
  are shell-dependent (dash 2). A body that falls off its end after a false test returns 1 with no
  errexit at all. Under a `set -eu` book, a guard's `( check )` subshell inherits errexit.
- `hole-overlaps-rc-one-is-the-knife` [FOUND] — `30W` §2's spelling (rc 0 overlaps, rc 1
  provably disjoint) puts the dangerous arm on the status accidents produce; the naive
  `[ "$1" = "$2" ]` body claims disjoint for a subtree pair.
- `fnd-zero-is-the-only-errexit-unreachable-status` [FOUND; human-acked with the caveat that
  0 is also what humans reach for as "true, do it"] — a dangerous claim carried in an exit
  status is errexit-safe only at rc 0 (the verdict family's convention; every accident is
  non-zero and runs); any non-zero placement is unsafe by construction and no value is exempt.
- `fnd-no-small-integer-is-clean` [FOUND, training-data] — rc 3: `systemctl is-active` and
  the LSB status convention (not running), Nagios/Icinga UNKNOWN, `pg_isready` no-attempt,
  `curl` malformed URL, `wget` I/O, `rsync` selection errors, `restic` partial read. rc 2:
  `test`/`grep`/`diff`/`cmp` trouble, `terraform plan -detailed-exitcode` changes-pending. 4:
  `systemctl` no-such-unit. 124 `timeout`; 125–127 exec failures; 128+ signals (141 = the
  sigpipe-flap class); 255 ssh. The human's option 1 (a rarely-used knife rc) has no candidate.
- **`rul-oracles-always-return-zero`** [TYPED 2026-09-05, the human's phrasing] — in most
  entry-points ALL non-zero rcs are reserved for ERRONEOUS classes: not explicit declines, never
  authorization or license, implying no authorial speech; reserved for errexit-internal-step-
  failure. Every path an author wants to turn into a meaningful decline must be caught, handled,
  and PRINTED (the report lane). Convergence (`__is_converged`) and `__predict` stay the
  exceptions for now. The principled line is OFF-RAMP VALUE: functions one might copy off GitHub
  without Dorc should use meaningful, rich rc because that is easier to consume outside Dorc;
  everything else speaks in records.
- `rul-three-rc-regimes` [ACKED 2026-09-05, "holds afaict"] — (1) `__predict`: rc is the
  tool's predicted rc, every value, no Dorc meaning (`30D`); `__enter` passes the guest's rc
  through, ≥2 before entry is siting decline. (2) `__is_converged`: 0 holds / 1 complement /
  ≥2 can't-say; rc-borne because its danger is self-scoped (own tool's line) and 0 is
  errexit-unreachable; the admin's lifted hand-guard has no emission to give. (3) every other
  member (`disturbs`, `disturbance_reaches`, `state_stored_in`, `lend_map`, `resolve`, the
  identity read, `overlaps`, the fs binder): rc is COMPLETION only — 0 ran to the end and its
  lines are read; non-zero died, everything withheld, collisions retained
  (`inv-30U-collide-on-integrity-failure-keeps-collisions`). No license rides a non-zero rc
  anywhere; knife-tier acts are tail-position records. rc 1 and rc 3 have no Dorc meaning
  outside regime 2; the rich decline taxonomy is `27W`'s `decline <class>` records.
- `fix-overlaps-disjoint-is-a-record` [PROPOSED; lean] — `overlaps` keeps rc as aid-grade
  (0 says overlaps; anything else can't-say; both collide) and the dangerous claim becomes a
  tail-position report-lane record (`printf 'overlaps nothing\n' >>"${DREP_V1:-/dev/null}"`-
  shaped, STRAWMAN verb): arrival witnesses completion, errexit death yields no record, the
  naive body emits nothing and is safe. The rename alternative (`kind__disjoint`, 0 = disjoint)
  is errexit-safe but the naive `[ "$1" != "$2" ]` is the knife; rejected.
- `srv-where-else-the-hole-lives` [FOUND] — `check-refutes-sense-flip`: if any seat reads rc 1
  as "holds" under `:!` (refutes) or the declared-dual glue's sense-flip, same hole; as-built
  grep. `check-predict-body-death-reads-as-tool-rc`: §7. `hole-resolve-fallthrough-idiom`:
  USER_STORY stage 6's `dpkg-query … || printf '%s\n' "$1"` masks can't-answer as "canonical is
  itself", two aliases stay distinct, the split knife; safe spelling `|| return 2` (decline ⇒
  may-alias); human-authored doc, suggested edit only. `exception-enter-is-structural`: the
  entry form's danger is the act of entering, priced by 27C's siting vouch, nothing to sentinel.
- `fnd-one-line-plus-rc-zero-is-not-enough` [FOUND; the human asked for pedantry] — a
  single-line answer (`resolve`, the identity read) under rc 0 is NOT a completion witness: an
  answer's content is OPEN (any prefix of a name is a valid name — `nginx-full` truncated to
  `nginx`), so a partial write followed by an explicit `return 0` under errexit-off yields a
  valid-looking wrong answer; a shared batch stdout interleaves writers above PIPE_BUF; `$( )`
  capture strips newlines; an orphaned `&` child can write late. What suffices: a closed-
  grammar tail sentinel (a truncated sentinel is not-a-sentinel), per-body capture never a
  shared stream, byte-level "exactly one `\n`-terminated line" counting, and either the
  sentinel on the SAME stream as what it witnesses or scaffold-declared errexit (a failed write
  makes `printf` return non-zero and abort before the sentinel). So `resolve` and the identity
  read are owed the sentinel like every emission member; and the existing cross-stream
  `disturbs nothing-else` (entities on stdout, record on the sink) is sound only under
  scaffold-declared errexit or a same-stream layout — a dependency to write down (§8).
  [REFINED §9: under scaffold-declared errexit, regime-3 bodies need NO compiled sentinel —
  rc 0 is itself the completion witness; the `only`-class totality records stay authored.]
- `srv-knife-consents-without-a-sentinel` [FOUND] — static marks (`undivided-by-transit-across`,
  referent-transparency, static `disturbs` arms) need none; emission members have or gain one
  (`disturbs nothing-else`, `stored nothing-else`, `lends nothing-else`, the `overlaps` record);
  `resolve` and the identity read gain one per the previous item. The fs binder inherits the
  record rule by riding the `disturbs` rails.

## §7 — the cleanup pass OWED (do not start piecemeal; one pass, human-directed)

Docs the rulings above touch, none edited yet: `plans/30W` §1 (Host blessing; sugar), §2/§3
(`overlaps` spelling; `stored-in` kind coordinates), §8/§10 (rulings 1–6 status) · `plans/27C`
§3 (compose ops; enumerate-every-dimension; the `"$@"`-verbatim entry rule already superseded
by `26Ob:ack-entry-verbatim-cannot-hold`, held by the human's "no 27C edit") · `plans/30U` §5
(store sentinel supersedes "absent by design"), §7 (may-inside) · `plans/30T` §6 (the name-bias
law gains the errexit clause) · `spike/CLAUDE.md` (rul-rc-partition, rc-naming-discipline,
rho-claim-ladder, the compare-consumer-map bullet; `27C:rul-dimension-owned-compose-ops`
pointer) · `ORACLE_PROVIDES` (trust/degrade lines for `resolve`/`overlaps`/identity; the entry
and fidelity shapes 26Ob owed) · `ANALYZER-NEEDS` (`an-kind-resolver`, `an-compare-chokepoint`,
`an-store-topology`, a row for `overlaps`, the rc-regime note on `an-verdict-3val`) ·
`KNOBS:kCONTRACT-RUNGS` (pointer) · `USER_STORY` (human-acked refresh OWED, never edited in
passing: the stage-6 resolver strawman's `|| printf '%s\n' "$1"` fallthrough is the split knife
— the human's lean is to make the correction a STEP in the story, narrowing it, rather than a
silent fix; also the enhancement-curve stages §5 notes) · `notes/26Ob` §10a/§10c strawmen
(superseded notes for the `: fs` marks and the identity read's sentinel) · `notes/272` §2/§3
(superseded notes: substrate tokens; the carried-by table) · `notes/30D` (the predict compile
and the signal-status rule, §9) · `plans/27C` §5 (the probe artifact's private-mechanics
license now carries the predict compile).

## §8 — the `__predict` turn and the oracle shell-state principle

- `hazard-predict-body-death-reads-as-tool-rc` [FOUND] — a probe runs `dpkg__predict -s nginx`
  and records its rc as the site's rc; `|| apt-get` and `&& systemctl restart` fold on it. A
  multi-statement predict body that dies via errexit on an unguarded step (a failed `cd`)
  returns a plausible tool rc, and the fold omits SOMEONE ELSE'S line. The trailing-succeeding-
  command shape (`dpkg "$@"; printf x >&2` ⇒ 0 always) is the same hazard at rc 0.
- `nack-predict-off-rc` [TYPED 2026-09-05] — moving `__predict` off rc as a control channel
  is unnatural and loses a ton of off-ramp value; not taken, despite the danger.
- **`prop-status-attribution-by-static-shape`** [PROPOSED; the primary out] — the predict
  tracer, which already walks completing paths for `predicts` records, attributes each path's
  exit status: it is a MEASUREMENT of the tool only where the status-producing statement is the
  family's tool invocation (or a `case $? in` remap over it, or an explicit `return N`, both
  authored speech) AND every preceding statement on the path is errexit-exempt-guarded or on an
  infallible safe-list (default-disqualify, the `an-read-set-closure` posture). Otherwise the
  path's rc is ⊤-provenance: no fold, both branches live, the site runs. Zero ceremony for
  delegation bodies (the whole low curve); engineers guard; off-ramp untouched; fails safe in
  both phases INDEPENDENT of runtime errexit; also kills the fall-off-the-end hazard for
  predicts. Self-scoped versus travels is the principled split: for `__is_converged` the same
  check stays a lint (`aid-lint-verdict-body-mechanicals`); for `__predict` it gates trust of the
  measurement, because a wrong predicted rc omits another line.
- `prop-predict-end-record-as-escape` [PROPOSED] — a closed tail record (`predict-end`-
  shaped, STRAWMAN) for bodies too clever for the tracer: opt-in ceremony, inert off-Dorc,
  restores trust of non-zero statuses on paths the static check declined.
- `nack-path-shim-rc-capture` [conductor] — capturing the tool's own rc through the per-run
  PATH shim is refag-leaky (the engine names "the tool") and bypassable (absolute paths,
  builtins, `sudo`, `command`); set aside.
- `hazard-errexit-ignored-in-exempt-contexts` [FOUND; ~SUSPECT; floor measurement owed] — a
  guard's check sits in an AND-OR list (`( check ) || original`) and any conditional consumption
  position is an errexit-IGNORED context by POSIX; bash documents that an explicit `set -e`
  inside such a compound command is still ignored; dash's behaviour is unmeasured. So "inject
  errexit at apply so bodies run identically" may not be a `set -e` inside the paren; a fresh
  shell process (`sh -e`-shaped, or `dorc-sh`) or a discipline that does not lean on runtime
  errexit may be needed. This is why the static attribution above is the robust primary and
  scaffold-declared errexit is defense in depth where achievable. Another day's question by
  the human's word; recorded so the floor lane knows what to measure.
- `30D` interaction: `return 2` predicts 2 (unchanged); a declining path emits `predicts none`
  and its status is unread; under the attribution rule a `|| return N` pre-step guard in a
  predict CLAIMS the tool would return N — the taught shape for the low curve is pure delegation.
- **`rul-oracle-shell-state-is-dorc-constructed`** [TYPED substance 2026-09-05; conductor
  wording] — oracle bodies receive their shell state (errexit and every other `set` option,
  IFS, PATH resolution, cwd, umask, locale class, the fd table) declared and constructed by
  Dorc's scaffolding plus the chain of world-declaring oracles above them in the CFG; never by
  the admin's book. Bodies must not behave differently under admin `set` values — all of them.
  The state is fully constructed at probe time and again at apply time the same way but without
  the `__predict`s; identical in construction, not in value (catching value divergence is the
  point). Oracle authors understand the tool and the measurable world; ensuring their probing
  state is correct is Dorc's job, and therefore their apply state. Generalizes `30S`'s
  pin-or-sever from env to all shell state; sits with `an-shell-options` (pipefail pinned on,
  per-host handshake) and `probe-composition-walls` (no book traps).
- `fnd-no-observable-world-to-shell-influence` [FOUND; the human's question, answered
  provisionally] — no case found where world state influencing shell state must be observable
  to a body for it to model a tool: who-am-I values are supplied from the modelled context
  (`272` r2 mapping; `27C:idiom-honest-read` is keyed by Dorc), PATH/umask/locale at the site
  are the book's own state Dorc models and replays (`30S`), `[ -t 0 ]` is a controller fact
  (`26O:rul-tty-test-is-a-controller-fact`), and genuine world facts (files, processes, ulimits)
  are measured AS world, not as shell state. Residue: shell state Dorc cannot model reads ⊤ and a
  body may not depend on it or must decline — the hermeticity precondition, one plane over.

## §9 — the predict compile turn (2026-09-05, later)

- `nack-attribute-to-the-tool` [TYPED 2026-09-05] — §8's static attribution "to the family's
  tool" is NACKED: the whole point of `__predict` is authorial license to build a good-enough
  mock to the author's own ops taste; a predict may reasonably never invoke the parent tool, and
  an explicit-return burden on every non-tool arm is complex and hard to keep in mind. The hope:
  predicts sit low on the enhancement curve, often one or two lines plus glob-declines. The
  human does NOT accept incorrectness as a price; correctness must be recovered another way.
- `lean-compile-in-completion-sentinels` [LEAN 2026-09-05, generalised by the human] — since
  Dorc controls oracle bodies tightly and there is no speech act for completion, compile the
  sentinel in: anything whose completion is not `only`-class (a totality claim, which IS
  authorial speech and stays authored) may have its completion witness inserted by Dorc; the
  author's explicit way out is declining on an arm. Conditional: only where errexit alone does
  not handle the realistic failure classes. Conductor ack with one refinement below.
- `fnd-predict-is-two-sided` [FOUND] — unlike the verdict, whose only dangerous status is 0,
  a predicted status of EITHER polarity can omit another line: a spurious non-zero kills the
  `&&`-right and the `if` body; a spurious zero kills the `||`-right. So no single rc is safe
  for predicts, and errexit alone cannot help: an errexit abort mid-body returns the failing
  step's status, indistinguishable from an intended non-zero answer.
- `fnd-regime-three-needs-no-compiled-sentinel` [FOUND; the refinement] — for emission and
  single-answer members (`disturbs`, `reaches`, `state_stored_in`, `lend_map`, `resolve`, the
  identity read, `overlaps`, the fs binder) the answer is in the LINES, so rc is free to mean
  completion: scaffold-declared errexit ON plus "0 = ran to the end" IS the witness (an abort
  or a failed `printf` on a short write lands non-zero; `rul-oracles-always-return-zero`). No
  inserted sentinel; the `nothing-else` records remain the authored totality acts. Per-body
  capture (never a shared stream) and byte-level "exactly one line" remain scaffold duties.
  Retracts §6's "owed the sentinel" for `resolve`/identity.
- **`prop-predict-compile`** [PROPOSED; answers the human's "treat each expected-last-command
  as the expected-return and track it at runtime"] — static and runtime halves, coherent:
  1. STATIC (the tracer, which already walks completing paths for `predicts` records): find
     every COMPLETION POINT and its ANSWER STATEMENT — the last statement of a path, or the
     statement before a bare `return`/`return $?`; an explicit `return N` is itself the answer.
     Compounds (`case`, `if`) recurse into arms; a helper call as the answer recurses into the
     helper's body (custody permits); a loop as the answer predicts ⊤ unless followed by an
     explicit `return`. The answer statement must be STATUS-MEANINGFUL: world-dependent (an
     external command, a `test` on world state or on a variable, a command substitution) or an
     explicit `return`; a status-flat tail (`printf … >&2`, `:`, `true`, an assignment) makes
     that arm predict ⊤ with a loud pre-network hint naming the line — never a refusal, the
     book still runs. This is attribution to "the author's intended answer", NOT to the tool.
  2. COMPILE (probe artifact only; predicts never run at apply, `28M:rul-predict-feeds-plan-
     never-apply`): scaffold-declared `set -e` for the body; each simple-command answer wrapped
     `set +e; <answer>; __dorc_rc=$?; set -e` (the toggle is what lets a non-zero answer be
     captured under errexit without aborting); then the completion record
     `printf '<nonce> predict-end <arm-id> %s\n' "$__dorc_rc" >>"${DORC_SINK}"; return "$__dorc_rc"`.
     A helper answer writes its own record at its completion points before returning, so the
     caller aborting on the helper's non-zero return afterward is harmless (the record is
     already written). Everything else in the body runs under errexit: an unguarded failing
     step aborts before any record.
  3. RUNTIME (the scaffold): a predicted status is TRUSTED only if the arm's completion record
     arrived, its rc equals the shell's exit status for that body (a compile-bug tripwire), and
     rc < 128 (signal deaths, 141 included, are the flap class, never predictions). Otherwise
     the site's status is ⊤: no fold, both branches live, the site runs, hint attached.
  Failure classes covered: errexit abort mid-body (no record ⇒ ⊤) · nounset trips (same) ·
  status-flat trailing statement (static ⇒ ⊤) · pipeline tails (pipefail pinned; per-host
  handshake) · signals/timeouts (⊤) · command-not-found from the answer itself (a faithful
  prediction under the shell-state principle's identical PATH; aid note). Residue, attributed
  not silent: a world-dependent trailing statement the author did not mean as the answer, and
  every wrong mock — both are authorial judgments the record's arm-id names by line.
- `acct-gradual-enhancement` — `dpkg__predict() { dpkg "$@"; }` compiles and folds with zero
  ceremony; a `case` with glob-declines likewise (a decline arm's record still witnesses
  completion; its status is unread); a trailing debug print costs that arm its fold and earns
  a hint; helper steps that fail cost the fold at probe with a hint. Nobody writes sentinels or
  `return 3`; the only authored records are the existing `predicts …` ones.
- `acct-kbackflips` — the compile lives inside the probe artifact, the surface `27C` §5
  already licenses for private mechanics; the author's file and `dorc strip` are untouched, so
  off-ramp value is whole; the transform is status-preserving on completing paths and only
  ADDS aborts where the author's sh would have continued after a failure; the record is
  Dorc-owned scaffolding, not speech (nonce-minted, unforgeable by authored text, closed
  grammar distinct from `predicts`); the rc cross-check bounds compile bugs; hostsim can inject
  an abort at every statement. Not free (the human: backflips never are), but the danger it can
  introduce is bounded to "a measurement refused", never a wrong measurement trusted.
- `nack-xtrace-as-witness` [conductor] — `set -x` with a nonce `PS4` would witness the last
  executed statement without editing bodies, but xtrace is pinned to fd 2 and inseparable from
  the tool's own stderr in POSIX sh (violates `law-control-never-shares-a-lane-with-freeform`
  as a hard rule), its format varies by shell, and it prints expanded argv (secret taint).
  Set aside.
- `prop-verdicts-run-errexit-off-in-both-lanes` [PROPOSED; the coherent v0] — the guard at
  apply is `( check ) || original`, an errexit-ignored context; an inner `set -e` is ignored
  there by bash's documented behaviour and ~SUSPECT by dash; the compiled form cannot enter the
  reviewed plan (`rul-ternary-verdict`'s strip-only; the attention floor). So for identical
  construction verdict bodies run with errexit declared OFF in probe and apply alike, and the
  A1 class (a failed `cd`, then the check in the wrong world) stays the quality bar's
  (`aid-lint-verdict-body-mechanicals`). Door to ON: the floor measurement — does `dash`
  honour an explicit `set -e` inside a subshell in an AND-OR list? If yes and bash-as-sh hosts
  are accepted as outside the floor, revisit.
- `rul-authored-set-in-bodies-is-out-of-dialect` [PROPOSED, follows §8's principle] — an
  author's `set`/`trap`/`exit`/`&` inside a role body is a dialect refusal: Dorc declares the
  options; a body that wants fail-fast on a step spells `|| return`/`|| { decline; }`.

## §9b — xtrace as the predict witness, evaluated; the static-only alternative

- `ctx-xtrace-is-already-per-host` [human, 2026-09-05] — apply reporting already depends on
  xtrace (`26O:mode-apply-cursor-via-xtrace`), so xtrace shell-compatibility is measured per
  host regardless; proposal put: both witness mechanisms long-run, the simpler xtrace form on
  targets probed as compatible, trace lines filtered out of stderr on the way through.
- `eval-xtrace-as-license-witness` [FOUND] — mechanics: `set -x` with a nonce `PS4` carrying
  `$LINENO` and `$?`; the last traced statement before the body exits is the completion
  witness (an abort's last line is the failing step, never the answer statement); the rc rides
  the scaffold's next traced line; no body edit, no errexit toggle needed. Axes: (a) LANE LAW —
  POSIX pins xtrace to fd 2, inseparable from the tool's own stderr; nonce-filtering a
  Dorc-captured per-body stderr file is mechanically sound (accidental collision impossible;
  continuation lines of a multi-line argv lack the prefix and fall to raw-err) but is control
  multiplexed INTO a stream whose content Dorc does not control, exactly
  `26O:law-no-multiplex-into-a-stream-we-do-not-control` [TYPED]; the cursor was admitted as
  AID-grade, never license (`26O` §6), and a witness that gates trust of a measurement is
  license-plane. Clean only where a dedicated trace fd exists (bash `BASH_XTRACEFD`); dash,
  posh, busybox ash, mksh have none, so the FLOOR cannot use it. (b) SHELL VARIANCE — bash
  repeats `PS4`'s first character per depth and quotes words `$'…'`; dash does neither;
  `$?`/`LINENO` inside `PS4` under the floor shells is the measurement the cursor already owes,
  so that cost is shared. (c) SECRET TAINT — expanded argv of every body command; mitigable by
  a host-side head-only framer keeping `<nonce> <lineno> <rc>` and dropping the rest before the
  link. (d) HOST REACH — a compiled record reaches fd-only hosts through an fd framer; the
  trace cannot leave fd 2 there. (e) TWO MECHANISMS — the compile would stay mandatory on the
  floor, so xtrace saves body-editing only on superset hosts, adds a differential obligation,
  and lets one predict body be trusted on host A and refused on host B. Verdict [conductor,
  for the human's word]: keep xtrace for the aid-grade cursor; not a license witness; do not
  re-open the lane law for it.
- **`prop-predict-static-only-errexit-off`** [PROPOSED; the exploration's yield, now
  preferred over `prop-predict-compile`] — run predict bodies with errexit OFF (nounset off,
  pipefail on per the host handshake), the same declared option state verdicts must have at
  apply (§9), so a body ALWAYS completes and its exit status is exactly its answer statement's
  (or an explicit `return`) on every path. The whole burden is then static and pre-network:
  (1) the answer statement is status-meaningful (§9 rule 1, unchanged); (2) every statement
  before it on the path is either provably infallible (a literal assignment, `local`, a `shift`
  the site's arity covers, a decidable-set builtin) or GUARDED (`|| return N`, an authored
  predicted status; or `|| { predicts none …; return 0; }`, a decline). An unguarded fallible
  step (an external command, `cd`, `$(…)`, `read`, a helper not proven infallible) makes that
  arm predict ⊤ with a precise pre-network hint ("line N can fail and the answer would then be
  measured in the wrong world; guard it or the arm predicts nothing"). Default fallible,
  safe-list infallible, the `an-read-set-closure` posture. `exit`/`set`/`trap`/`&` stay out of
  dialect in bodies; statuses ≥128 read ⊤. No runtime witness, no body edit, no compile, no
  trace. The plan's shape is a function of the text, never of which step happened to fail this
  run. Onramp: zero cost for one- and two-line predicts; the guard is the defensive-sh habit
  the dialect already preaches (`30S:rul-idiomatic-plus-offramp`, `27C:idiom-dependency-guard`).
  Off-ramp: untouched. kBACKFLIPS: nothing touched. Trade against the compile: a sloppy body
  with an unguarded step loses its fold ALWAYS instead of only when the step fails at probe —
  less value for sloppy bodies, deterministic plan shape, the hint at authoring time rather
  than after a round-trip. `prop-predict-compile` (§9) is demoted to a LATER, layered upgrade
  (private-probe machinery, no author-facing change) if field evidence shows sloppy-but-
  valuable predicts are common.
- `csq-one-authoring-surface` — under this, predict and verdict bodies share one declared
  option state (errexit off, pipefail on, nounset off) and one discipline (guard fallible steps;
  end on a status-meaningful statement or an explicit return): the consistent surface §8
  demands. The question "does errexit handle all realistic failure classes" resolves as:
  errexit is not the instrument for role bodies at all — static fallibility is — and errexit
  stays the BOOK's, the admin's intent surface (`26Ob:rul-cfg-error-handling-governs`).
  Regime-3 bodies (§9's refinement) keep errexit ON, where rc 0 is their completion witness
  and no answer rides the status; that asymmetry is deliberate: emission bodies carry no
  answer in rc, so abort-on-failure costs nothing and buys the witness for free.

## §9c — the backward walk from the off-ramp: a staged mix

- `eval-within-probe-fd-redirect` [FOUND; the human's last devil's advocate] — a temporary
  redirect inside the probe cannot give xtrace and the body's stderr separate channels in
  POSIX sh: the shell writes the trace to ITS fd 2 and hands that same fd 2 to every child it
  forks, and a redirect on the function call (`f "$@" 2>err`) moves BOTH. Only a per-command
  `2>&3` (the compile again) or a dedicated trace fd (bash only) separates them. Noted beside
  it: a nonce framer over the mixed capture is fail-SAFE for the witness (a glued or missed
  last line demotes to ⊤; forging needs the nonce, and a forger already owns the status), so
  the standing objection is the typed lane law's posture, shell variance, and two mechanisms —
  the human acked those. xtrace's legitimate remaining use here is AID: naming the line an
  aborted body died on, for the hint.
- `nack-static-only-as-the-whole-answer` [TYPED 2026-09-05, "not a hard nack, close"] —
  static is nearly always better mechanically; the problem is authorship burden. An
  experienced admin writing for non-Dorc reaches for EITHER guard-each-statement OR errexit,
  depending on which is noisier (many failures to stop on, or many to proceed through); both
  are meaningful speech. A rule that forces guards makes bodies unnatural and puts the pain on
  the user.
- `fnd-static-rule-is-option-independent` [FOUND] — "an unguarded fallible step before the
  answer ⇒ the arm predicts ⊤" is correct whatever the option state: under errexit-off the
  step's failure makes the answer wrong-world; under errexit-on it makes the status an abort
  misread as an answer. And "unguarded" is exactly POSIX's errexit-FIRING position (a fallible
  simple command not consumed by `if`/`while`/`&&`/`||`/`case $?`/a test's substitution) — so
  the static rule reads as "your body must be errexit-clean", the very discipline both
  philosophies already encode, one by guards and one by `set -e`.
- `fnd-subshell-body-is-the-native-trigger` [FOUND] — `f() ( set -e; … )` is legal POSIX
  (a function body may be any compound command, a subshell included), contains the option to
  the function, and is the natural standalone spelling of "this function is a fail-fast unit";
  `f() { set -e; … }` leaks errexit into the caller's shell off-Dorc and earns a lint. The
  subshell shape is therefore a zero-cost, already-meaningful user-control trigger between
  "I handle failures" (brace body: guards are my speech) and "failures abort me" (subshell
  `set -e`: the abort is my speech). No new syllable; Dorc reads the shape.
- **`prop-predict-staged-mix`** [PROPOSED; the lean, replacing §9b's static-only] —
  v0: the option-independent static rule for every predict body; brace bodies run with Dorc-
  declared errexit off, subshell `set -e` bodies run as written (contained); both get the same
  verdict: guarded ⇒ trusted, unguarded fallible step ⇒ ⊤ with a hint; `exit`/`trap`/`&`/`set
  -x` out of dialect in bodies; a `case` with no `*)` arm predicts ⊤ on the fall-through path
  (never the shell's 0) with a hint. v1, layered, engine-cost only, no author-facing change:
  the `prop-predict-compile` machinery (§9) applied ONLY to subshell `set -e` bodies — the
  toggle around the answer and the completion record — so their unguarded steps become
  runtime-witnessed (abort ⇒ no record ⇒ ⊤ only when it fires) and the guard requirement is
  refunded for exactly the authors who chose errexit. Brace bodies never need it: their guards
  are already speech. Onramp: each author writes what they would write standalone. Off-ramp:
  the authored file is untouched by either stage. kBACKFLIPS: v0 touches nothing; v1 edits
  only the private probe artifact, status-preserving, bounded by the rc cross-check.
- `acct-backward-walk` [the human's exercise; STRAWMAN body, a certs tool with a
  `sync-certs` mock, a `status` delegation, and a catch-all] — ABSOLUTE costs (Dorc-imposed
  changes to a standalone body): exactly one, at the lowest rung — the catch-all decline.
  `30D` reads `*) return 2 ;;` as "the tool returns 2 for unmodelled verbs" (every value
  predicted; no reserved decline), so a newcomer's natural catch-all is a WRONG prediction that
  omits other people's `&&`-rights; the taught catch-all is the one inert record line
  `*) printf 'predicts none\n' >>"${DREP_V1:-/dev/null}" ;;`, and a lint flags `*) return N`.
  Plus the rare bans (`exit`, `trap`, `&`, `set -x`). Nothing else: no sentinels, no return on
  every line, no toggles in the file. GRADUAL-ENHANCEMENT costs (a defensive author would do
  it; a risk-tolerant one may not): guard fallible steps or write the `set -e` subshell body;
  add a `*)` arm. Under v0 the errexit philosophy pays the guard cost too; v1 refunds it.
  Safety in every version: guarded bodies trusted; abort-capable bodies ⊤ (statically at v0,
  at runtime under v1); sloppy bodies ⊤ with hints and a book that still runs. No wrong trust
  anywhere; downstream elisions never rest on an unwitnessed predict status.
- `note-missing-catch-all-predicts-zero` [FOUND] — a `case` with no `*)` completes with the
  shell's status 0 on an unmatched verb; read as a prediction that would omit every `||`
  fallback below an unmodelled verb. The static rule's "no answer statement on this path ⇒ ⊤"
  covers it; recorded because it is the most natural newcomer shape of all.
- `assess-staged-mix` [conductor, 2026-09-06; the human asked "best we've discussed?"] — yes,
  because the ordering is forced, not chosen: the static rule is the ONLY thing that catches
  the debug-tail and missing-`*)` shapes, so it is required under every alternative, and the
  compile is additive refinement over it; static-first is therefore the minimum, and xtrace
  never reaches license-grade. Downsides, named: (1) the fallibility safe-list is a new
  decidable-set-adjacent component whose every widening is winner-shifting (license-review
  tier forever, like funcenv precision); (2) the errexit philosophy pays guards until v1 — v0
  alone IS the static-only posture the human called painful; (3) at v0 the two body shapes
  behave identically under Dorc, so `set -e` "does nothing" until v1 changes its VALUE (never
  safety) — a small story wrinkle; (4) v1 is the first Dorc edit inside authored bytes beyond
  strip, in the probe lane; fence it predict-only, probe-only, or every lane will want it;
  (5) hint precision is load-bearing ("arm X predicts nothing because line N can fail"), and
  "which line aborted" at v1 wants xtrace-as-aid or per-statement records; (6) how often real
  predicts carry unguarded steps is unmeasured, which is what decides whether v1 is soon.
- `door-reserve-status-two-in-predicts` [the human's nit, 2026-09-06: `return 2` never
  shipped, no compat or lints owed if it changes] — `30D`'s "every value predicted" was chosen
  so tools that genuinely return 2 (`grep` error, `diff`, `terraform plan -detailed-exitcode`)
  stay predictable. Reserving rc 2 as the predict DECLINE instead would remove the one
  absolute cost at the lowest rung (the catch-all becomes `*) return 2 ;;` again, meaning what
  every newcomer thinks it means) at the price of a `predicts status 2` record for the rare
  tool that really returns 2. Trades the record from the common case to the rare case; the
  human's call; `30D` §7 territory.

## §9e — the decline ladder across the two functions (2026-09-06)

- **`rul-decline-ladder-across-verdict-and-predict`** [TYPED 2026-09-06, the human's tune on
  the conductor's hint; closes `door-reserve-status-two-in-predicts` — 30D stands] — the
  one pattern accepted silently in BOTH functions is the named decline,
  `printf 'decline <class> …' >>"${DREP_V1:-/dev/null}"; return 2`: an author may use exactly
  the same shape in `__is_converged` and `__predict` as long as they declare their declines
  with names. A bare `return 2` decline (no record) is `__is_converged`-ONLY — an
  early-onramp accessibility gloss, always graduable to the printed form. In a predict a
  bare `return N` (N ≥ 2) keeps its 30D meaning (predicts N) and earns a hint that pushes up
  the ladder ("omitting the printf stops working here; a bare return has a different
  meaning"). The ENFORCING capacity flips between the functions: in `__is_converged` the
  `return 2` is load-bearing and the record is aid; in `__predict` the record is load-bearing
  (it declines the status channel, as `predicts none` does) and the status is unread. Both
  together is maximally helpful outside Dorc. Consequence for the cleanup pass (§7): the
  `27W`/`ORACLE_PROVIDES` wording "decline is aid-only" is RETIRED as stupid wording — the
  record's capacity is per-function, not per-record. A "stop bugging me" tag for this
  hint family is a later consideration, not now.
- `expl-decline-outside-dorc` [the human's "forget Dorc, unweld everything" exploration;
  conductor's findings] — is there any `__predict` that is helpful outside Dorc, maximally
  shell-idiomatic, and can decline to model a call-site WITHOUT stomping exit codes?
  Strictly no: Unix has no "I don't know" channel distinct from "I failed"; every real
  dry-run tool (`apt-get -s`, `rsync -n`, `make -n`, `git --dry-run`) conflates the two,
  because a dry run IS a run modulo mutation and its decline is its own error. The only
  idiom that separates a negative answer from an inability to answer is the predicate
  convention (`grep`/`cmp`/`test`: 0 yes, 1 no, 2 error) — which is exactly why
  `__is_converged`, a predicate by construction, gets the 0/1/≥2 partition and `__predict`,
  a mimic of an arbitrary tool whose status space is all of 0–255, cannot. Best available
  approximations, each priced: (1) a human-facing message on stderr, the universal "couldn't"
  idiom — free of status, helpful to any caller; (2) a decline status chosen by the author
  from THEIR tool's unused range (per-tool knowledge, refag-clean, never interpreted by Dorc);
  (3) the shell's own out-parameter idiom, a global set by the function (`getopts`/`OPTARG`,
  `read`/`REPLY` shape) — carries a decline with no I/O and no sink, so it would work on
  sinkless hosts, but it is a second mechanism, needs a reset at function entry (boilerplate),
  and dies in subshell/pipeline calls; noted, not pushed; (4) passthrough of unmodelled verbs
  to the tool's own dry-run (`*) foobar --dry-run "$@" ;;`) — the most helpful off-Dorc
  behaviour, not a decline but modelling-by-delegation, and under Dorc exactly the
  self-vouch question DESIGN.md names. The human's ladder is (1)+(2) already, with one
  refinement worth a line: the record sink's OFF-Dorc default is `/dev/null`, which makes a
  declined function silent to its human caller; `${DREP_V1:-/dev/stderr}` (a device, allowed
  by the pinned write-only/may-be-a-device contract; `/dev/stderr` present on Linux, BSD via
  fd 2, macOS) would make the same line print its named decline to stderr outside Dorc and
  reach the sink inside it — the "helpful outside Dorc" half at zero cost. -GUESS worth it;
  the human's call; a `30D`/`27W` cleanup-pass item if taken.

## §9f — two more decline carriers, compared against the ladder (2026-09-06)

- `alt-env-var-out` [the human's; conductor evaluation] — the body sets a variable
  immediately before returning (`DORC_DECLINE='unmodeled verb'`; on a completion path a
  reached-marker); the scaffold reads it after the call. FOUND: (1) `export` is the wrong
  verb — it leaks into every child the body runs and pollutes the constructed environment
  (§8); a bare assignment suffices. (2) It does NOT require in-process calls: the scaffold's
  per-body subshell can read the variable after the call and forward it on the scaffold's own
  fd by NUMBER (`( f "$@"; printf '%s %s\n' "$?" "$DORC_DECLINE" >&3 )`), so no sink file, no
  `/dev/fd` path, no fs-write — it reaches the sinkless cell (hardened containers, failed
  hosts, FreeBSD without `fdescfs`) that the record cannot, and a variable cannot be
  truncated, which retires the partial-write pedantry for the witness. (3) Statically it is
  identical to the record: an assignment on the arm, per-arm inventory, the reason in the
  value. (4) Off-Dorc it is the `getopts`/`OPTARG`, `read`/`REPLY` out-parameter idiom:
  programmatic, silent to a human caller, dies under `$(…)` and pipelines, needs a reset
  against staleness (the scaffold's fresh subshell per call gives that for free; an off-Dorc
  caller does not get it). (5) It is a second CARRIER for the same speech the sink already
  carries (`decline`, `predicts`, `nothing-else` — all flag-shaped), so either it replaces the
  sink for records (a re-cut of 27W/30D/30U's transport) or it is two spellings for one act
  (`inv-30U-one-spelling` forbids). Verdict: the authored surface stays the ladder; env-out is
  a door for the TRANSPORT round — records carried by scaffold-read variables instead of an
  author-written sink — whose one real prize is the sinkless cell; not taken now.
- `alt-env-var-in` [the human's; conductor evaluation] — Dorc invokes the body twice under
  different environments: once asking "handled?" (a predicate, 0/1/≥2, errexit-safe at 0
  since the dangerous claim is "handled"), once for the real status. FOUND: the caching
  variant ("first call caches the real rc, second returns it") needs a same-shell cache and
  is `alt-env-var-out` with an extra call. The recompute variant needs the author to switch
  on the mode in EVERY arm (ceremony on the lowest rung) or to duplicate the argparse in a
  handled-branch — and two `case`s over one argv drift, the dual-peel chimera class
  (`wrapper-law`; `28Q` §1's pope-sin), so "handled: yes" can pair with a body that falls
  through to the catch-all and predicts the fallthrough status: the footgun returns by the
  back door. Off-Dorc a mode switch inside the function is a Dorc protocol a normal caller
  cannot use (`kOOB`-adjacent). Statically it adds nothing: reach of a modelled arm is already
  the tracer's answer without a second call. Verdict: dominated by the ladder on onramp and
  off-ramp, equal on safety, and its one advantage (sink-independence) is `alt-env-var-out`'s
  for less.
- `note-what-the-winner-is` — the ladder of §9e remains the AUTHORED surface (the record ON
  the arm it declines: locality is what keeps handled-ness and the body from drifting apart);
  the open question these two surfaced is only the carrier on sinkless hosts, and that is
  `26O:rul-sink-form-is-a-context-capability`'s to answer with the scaffold-read variable as a
  third sink form beside file and fd.

## §9g — env-var-in, refined: a second call only for the statically-ambiguous class

- `refine-env-in-as-escalation` [the human, 2026-09-06] — not always two calls: only as a
  supplement where static analysis finds the errexit/error-flow ambiguous — idiomatic
  forms Dorc wants to accept (not `eval`-class) whose abort-versus-answer status cannot be
  decided from natural errexit or non-errexit code without an author's printf.
- `fnd-second-call-reveals-nothing-without-cooperation` [FOUND] — a second invocation can
  only add information if the environment change alters the body's behaviour along the
  ambiguity, or the author branches on it. The only environment knob tied to the ambiguity
  is errexit itself, so the cooperation-free form is the −e/+e DIFFERENTIAL: run once
  continuing, once aborting; agreement means no step failed OR the abort status coincided
  with the continued answer. Coincidence is common (1 is everyone's failure: bash `cd`, false
  tests, `grep` no-match), so agreement is evidence, never proof — not a trust instrument.
  It IS a good AID for exactly the ambiguous class: disagreement says "an internal step is
  failing" with evidence, which is the hint precision §9c named as load-bearing. One sound
  corner exists — when the answer statement's status set and every unguarded step's abort
  set are provably disjoint, one −e run disambiguates alone — but abort sets are shell- and
  tool-variant (dash `cd` 2, bash 1), so it is fragile and not taken.
- `fnd-cooperating-env-in-is-the-record-with-worse-locality` [FOUND] — with cooperation
  (`[ "${DORC_ASK-}" = handled ]` branches) it is authorial speech: per-arm it is the
  record's locality with more ceremony and no off-Dorc meaning; top-level it duplicates the
  argparse and drifts (the dual-peel chimera). Dominated by the record on the arm.
- `fnd-the-instinct-is-v1s-trigger` [FOUND] — the sound content of the refinement is its
  SHAPE: escalate to a runtime instrument only for the ambiguous class. That is exactly the
  staged mix's v1 trigger (§9c), and the only runtime instrument that tells abort from answer
  when the author has not spoken is the inserted witness (the compile), because no
  environment can make an unmodified body report where it exited. Env-in tries to obtain the
  compile's effect without the compile; sh does not permit it. The differential survives as
  the hint's evidence source for that class, and possibly as a cheap pre-v1 aid.

## §10 — state at close (2026-09-06, seventh fixpoint; the human rewinds after this)

- TYPED this sitting: `nit-no-sugar-over-stdlib-kinds` · `note-transit-mechanics-are-open` ·
  `rul-overlaps-is-a-kernel-generator` (conditional; condition met) · `rul-oracles-always-
  return-zero` · `rul-three-rc-regimes` (acked as holding) · `nack-predict-off-rc` ·
  `nack-attribute-to-the-tool` · `ack-names-are-literal-narratives` · the oracle shell-state
  principle (substance) · the cleanup-pass-later directive · the USER_STORY-as-a-story-step
  lean · the Host-unblessing lean · `lean-compile-in-completion-sentinels` (conditional) ·
  `rul-decline-ladder-across-verdict-and-predict` (2026-09-06; 30D stands, the
  reserved-status-two door closed, "decline is aid-only" wording retired).
- PROPOSED, each awaiting one typed line: the three shapes as written in §2 (S1 near-typed;
  S2 the placeholder species; S3 the filtered meet) · `rule-inherit-or-top-with-sentinel` (and
  the reset sentinel) · retiring 27C's fs-view concatenation and enumerate-every-dimension ·
  `rul-store-containment-is-may-inside` · `fix-overlaps-disjoint-is-a-record` ·
  `fnd-regime-three-needs-no-compiled-sentinel` (retracting the resolve/identity sentinel) ·
  `prop-predict-staged-mix` (§9c; the lean — v0 the option-independent static rule for all
  predict bodies, v1 the §9 compile for subshell `set -e` bodies only; supersedes §9b's
  static-only and §8's tool-attribution) · `prop-verdicts-run-errexit-off-in-both-lanes`
  (brace bodies; a subshell `set -e` body is contained and allowed) · `rul-authored-set-in-
  bodies-is-out-of-dialect` (narrowed by §9c: `set -e` as the first statement of a SUBSHELL
  body is the sanctioned fail-fast spelling; brace-body `set -e` lints; `set -x`/`exit`/`trap`/
  `&` stay out) · the taught catch-all decline record and its lint (§9c) · xtrace stays
  aid-grade, never a license witness (§9b) · ruling 3's transport/sparing split · the
  tenure-not-in-the-value exclusion.
- OPEN: `open-bound-token-same-across-chains` (committee-speech turn; read 28M/28K/30J first)
  · whether r31 builds the context-slot product or reserves it (the sibling's, with the human)
  · `check-refutes-sense-flip` (as-built grep) · the `$?`/`LINENO`-in-`PS4` and dash
  inner-`set -e` floor measurements (now the cursor's, not a witness's) · the enhancement-curve
  survey · the USER_STORY refresh (human-acked, as a story step).
- Successor: resume from this section; the transport lineage resumes from `26Ob` §15 and
  becomes `311`; `310`/`ROADMAP` are the sibling's; the cleanup pass is §7.
