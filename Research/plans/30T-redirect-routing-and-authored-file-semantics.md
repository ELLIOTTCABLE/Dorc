# 30T — Authored file semantics: redirect routing, the filesystem binder, and the ask-the-world discipline

> Design of record for redirect routing, filesystem binding, and file-identity discipline.
> Subordinate to the root docs, `spike/CLAUDE.md`, `KNOBS.md`, `plans/30I`, and
> `plans/30P`; read with `plans/30U`, which defines the cross-kind finished-definition
> gate consumed here. Sections 0–9 state product semantics and deliberate limitations;
> §10 records implementation components and dependencies. Shell examples are illustrative;
> the language and role specifications govern exact dialect details and portable spellings.

## §0 the-design-in-one-screen

- **`rul-routing-is-the-third-edge`** — the shell's plumbing connects command channels to
  three kinds of target. Channel→channel (pipes) composes authored predict bodies along the
  edge (`rul-only-oracle-bytes-ship`). Channel→value (`$(…)`, `read`) follows the capture
  lane (`275`). Channel→world (`> f`, `>> f`, heredoc writes) follows this component. A file
  named in argv (`tee f`) needs nothing additional — that is tool semantics, already
  oracle territory. The redirect is the case where *the shell, not the tool*, places the bytes.
- **`rul-locators-are-parsed-coordinates-are-authored`** — the engine's parse yields a
  structural *locator*: (target word, cwd-state at that line, open mode). POSIX warrants it;
  no author does. Turning a locator into a *claim* (`disturbs sm.dorc.File:<entity>`) is a
  vocabulary act, and it is authored: a filesystem role member receives the locator and
  answers or declines, exactly as argv flows through a tool oracle's argparse
  (`rul-argv-flows-bytes-do-not`, generalized: structure flows through authored interpreters;
  the engine transports).
- **`rul-silence-walls-decline-walls`** — no binder loaded, binder declines, unknown cwd,
  unknown filesystem type, fd-state weirdness: the write is an unmodeled mutation and a total
  wall. Binding only ever *narrows* walls; an empty oracle world remains byte-identical
  (`empty-world-byte-identical`).
- **`rul-binder-claims-are-whole-entity`** — a binder claim names the whole entity
  (`sm.dorc.File:<path>`, selector-less, which the algebra reads as ⊤-selector: collides with
  every cell of that entity). Why §3.4 explains: a tool can mutate *through* its routed channel
  in ways neither speaker can name per-selector (an `fchmod` of stdout). Per-selector
  refinement therefore requires channel-relative tool speech (§9).
- **`rul-engine-holds-no-world-facts`** — no platform-conditional fact about the world is
  load-bearing in engine code, in this domain or any it touches. The engine's verbs are:
  *parse* (sh structure), *compile questions*, *fold answers* (the algebra), *act and verify
  its own mutations*, and *arrange geometry* so authored sh stays true (`30P` mirroring).
  Never: interpret the world. What the engine may know about paths is path *syntax*; every
  path *semantic* is authored speech or a phase-time measured answer (§5).
- **`rul-questions-route-to-latest-phase`** — every world-question has a latest sound phase:
  plan-time analysis may speculate only inside the one audited decidable set (falsified at
  probe standup, `mech-two-standups`); the probe measures; a guard re-measures at apply, in
  sequence, staleness-free by construction. {elide, guard, run} is this rule's canonical
  instance: elide = answered at probe; guard = deferred to apply; run = never answered.

## §1 the-problem — the mutation argv cannot see

The oracle contract is argv-keyed by law (`identity-declared-never-inferred`): the engine
parses no tool argv; the oracle's own argparse is the sole entity-resolver. A redirect target
never reaches argv. `cat >f <<EOF`, `printf … >f`, `sed … >f.new` — the single most common
way shell mutates a file is structurally invisible to the mechanism that describes mutations.
An fs-stdlib author cannot write a `cat__disturbs` naming the target; `cat`'s argv on that
line is empty.

Without a binder answer, such a write is an unmodeled mutation and a total wall, so
everything below it guards or runs. This makes the careful admin's write-if-changed idiom
self-wall: building the candidate file invalidates every downstream fact, including facts
about unrelated state. The filesystem binder gives that routing effect an authored,
bounded name while leaving the admin's own comparison live.

## §2 what-the-user-sees

This is the user-facing capability boundary.

**Dorc knows shell; it does not know filesystems.** It can see *that* your book writes a
file. What it cannot see, and deliberately never guesses, is what that path *means*: whether
two spellings name the same file, whether "already has these bytes" counts as done, or
whether a path is an ordinary file at all (on Linux, writing to some paths under `/proc`
reboots the machine). Filesystems differ too much — case-insensitive disks, symlinks and bind
mounts, SELinux labels, network mounts — for built-in opinions to be safe anywhere but the
machine in question, and a wrong opinion commits the one sin Dorc refuses: skipping a command
that needed to run.

So Dorc holds no opinions about files. Every file question is answered one of three ways:

1. **An oracle answers, on the machine in question.** The filesystem oracle in the standard
   library is ordinary sh, run where the answer lives: on the target host during the
   read-only probe, or as a check directly in front of your own command at apply time.
   Answers are evaluated by the actual filesystem rather than translated through a controller
   model, so platform behavior comes from that machine. Their semantic correctness remains
   the named oracle author's contract, like every other authored answer.
2. **Dorc verifies its own work instead of predicting it.** When Dorc places files on a host
   (your plan, and the oracle files it travels with), it copies, then checks — every file
   present, byte-exact, and *as many distinct files as intended* — before your book's first
   line runs. A host that disagrees stops the run; it never bends it.
3. **Nobody answers → the line stays.** A write nobody can speak for is treated as touching
   something unknown: it runs, and everything below it stays in your plan, running or
   guarded. Silence never removes a line; only an answer with a name attached can.

**What this buys you, stated exactly.** Writing a file stops costing you *the rest of* your
plan. Your own `cmp -s conf.new conf || cp conf.new conf` stays in the plan — it reads the
file the write produces, so no honest tool can remove it, and it is already the correct
guard — but the unrelated lines *below* it (the service enable, the firewall rule, the
package check) recover their elisions where `30U`'s finished definitions license the
cross-kind comparisons, instead of drowning behind the write. On a converged host the
honest render is "a few to run, most skipped", not "everything vanished."

**Caveats.** Everything fails toward running — missing knowledge costs attention, never
correctness. Files that don't exist yet often answer can't-say. And any elision kept past a
command that really runs rests on named authors' at-most claims and sits behind
`--risk-faultless-skips`, exactly like every other trust of that shape.

## §3 the-two-speakers — authorship territories

A redirected line has two describers, and they are different people: whoever wrote what the
*tool* means, and whoever wrote what the *shell construct* means. Input separation keeps them
from becoming a committee.

### §3.1 the tool oracle's side

The tool author follows the ordinary role contract: `predict` models the command's own
behaviour per channel (stdout default-declined, claimed by authored DREP speech — `30D`);
`is_converged` is the vouch; `disturbs` is the argv-keyed at-most write-set. Nothing about
redirects appears in any of it, and nothing *can*: the tool oracle's functions receive the
site's argv, and the routing information is not in it. A tool author cannot overreach into
routing territory even by error — the claim is unspellable from their inputs.

The canonical idiom makes one contract discipline especially visible:
**`req-verdict-marks-every-read-cell`** — a verdict body must mark every world cell whose
state its exit status reads. The `cmp`-shaped verdict reads *both* operands and must mark
both; marking only the destination leaves the source out of the fact's backing, and a fact
whose backing omits a cell an upstream line writes can be wrongly spared past that line.
The backing carries no completeness burden with respect to the *world*
(`an-backing-selfframing` — adequacy is priced at the vouch); this requirement is about the
body's own *visible reads*, and it is mechanically detectable: a falsification-first
detector (never a gate — `rul-unprovable-rides-the-vouch`) warns when a body's path-bearing
operand reads exceed its marks. Oracle authorship includes this read-to-mark contract.

### §3.2 the filesystem binder's side

One kind-species role member receives structural locators. Its contract:

- **Answer or decline, per locator.** An emission is an at-most claim ("this routing act
  disturbs at most this entity — whatever else, I answer for"); no emission is no claim, and
  the wall stands total. Declining is ordinary control flow and always safe.
- **Whole-entity claims only** (`rul-binder-claims-are-whole-entity`). The claim names
  `sm.dorc.File:<entity>`; never a selector.
- **Taxonomy is measured, not enumerated.** An existing target binds only when its resolved
  object is an ordinary regular file on a filesystem from a small authored allowlist of
  persistent filesystems (ext4/xfs/btrfs/apfs/…). An absent target binds only when its create
  parent and redirect mode establish the corresponding regular-file creation; dangling or
  ambiguous routes decline. FIFOs, devices, sockets, directories, procfs, sysfs, devtmpfs,
  FUSE, network filesystems, and every unknown object or filesystem type decline and retain
  the wall. Path-prefix denylists are not a conservative closure; object type and filesystem
  type are measured inside the shipped authored body. The allowlist is platform-oracle
  content, never an engine table (`30S`).
- **The standing emission discipline applies whole**: the completion sentinel on every
  completing path (`an-atmost-completion-signal`), body-death refuses the whole footprint,
  the report-lane idiom for declines. The binder ships on the same rails `disturbs` bodies
  ship on (`an-derived-footprint`): strip-only, read-only, stdout-emitting, all-or-nothing
  readback.
- **Attribution**: every claim carries the binder author's name; every survival it licenses
  cites it; `dorc why` shows the arm.

Entity spelling: the locator's target word resolved against the modeled cwd at that line.
An unresolvable word or ⊤ cwd (including everything below a blind act —
`law-no-unsoundness-below-a-blind-act`) never reaches the binder: the locator is ⊤ and the
wall is total before any authored code is consulted.

### §3.3 why the speakers cannot collide

The mutation surface of a simple command partitions along sh syntax itself: argv-side
effects belong to the tool's author; routing-side effects belong to the shell, whose meaning
the filesystem author holds; shell-*state* effects (assignments, `cd`, options) belong to
the engine's sh-parity model. The partition is enforced by **input separation**, not by
discipline: tool functions receive argv (no routing info); the binder receives the locator
(no argv). There is nothing shared to disagree about, so no coherence check is needed —
compare the wrapper surface, where two speakers parse one shared argv and the dual-peel
check must fail-fast on disagreement (`wrapper-law`). And at the vocabulary level the binder
has no committee power by construction: claims and disturbs emissions never mint selector
dialect (`an-selector-dialect` — claims contribute tokens only, interpreted at the backing's
closure). Two authors, one line, zero pooled speech.

The composed line's footprint is the union of the two territories' claims, and the
universal-meet consumers already treat unions correctly: sparing requires every member
provably disjoint from every backing member (`set-lifting-universal-meet`), and a wrong
survival cites the specific member that licensed it — singly authored, singly named.

### §3.4 residue, and why whole-entity is the floor

At-most claims carry residue ("whatever else, I answer for"), and a two-speaker line has one
structural residue class worth naming: a tool can act on the *open file description* the
shell handed it in ways beyond writing bytes — `fchmod` of stdout being the sharp case,
which mutates the target's mode. The tool author cannot name the cell (no vocabulary for
"whatever my stdout resolves to"); the binder cannot know the tool does it. A per-selector
binder claim (`@contents`) would therefore under-claim in a way *neither diligent author*
could repair. Whole-entity claims close the class: the claim collides with every cell of the
written entity, so a fact about that entity's mode, label, or anything else correctly
refuses to be spared past its write. The cost is same-entity precision — a fact about the
written file itself can never survive its write — which is almost always the correct answer
anyway. Per-selector refinement uses channel-relative tool speech ("I mutate at most my
stdout's target"), composed by the engine through the routing graph the same way pipes
compose; §9 records its absence from the base tier as deliberate non-capture.

What residue remains is ordinary authored incompleteness on each side of a clean boundary,
with one honest note: an unexplained under-execution on a composed line names *two* suspects
where a plain line names one. Both are named (`271:rul-sin-ordering`: attributed class), the
territories tell the investigator which side to read first, and the known structural
instance is closed by the whole-entity rule.

## §4 how-a-line-decides — phases and the canonical shape

The canonical book shape has this behavior:

```sh
cat > /run/web1.conf.new <<EOF
…rendered config…
EOF
cmp -s /run/web1.conf.new "$CONF" || cp /run/web1.conf.new "$CONF"
systemctl enable --now nginx
ufw allow 443/tcp
```

- The `cat` line **always runs**: no vouch exists for a bare write (whether re-writing
  identical bytes is acceptable churn is a judgment — converged≠no-op — and nobody has
  authored it at this tier). Its wall, with the binder loaded and answering: at most
  `File:/run/web1.conf.new`.
- The `cmp || cp` line **stays and guards** — correctly, permanently, at this tier. Its
  verdict reads the file the wall writes, so its backing contains the wall's claimed entity,
  the universal meet collides, and no honest machinery can spare it. It is also already the
  right runtime behaviour: the admin's own in-sequence check, re-measured live after the
  write, at zero added attention.
- The `systemctl` and `ufw` lines — backed in other kinds — **survive** under the admin's
  flag when the File kind's finished definition licenses those cross-kind comparisons
  (`30U`; §7), which is where the attention payoff lives.
- With an unmodeled producer in the `cat` seat, its own ⊤ keeps the wall total regardless of
  the binder: binder value is conditional on modeled producers, by construction.

The phase behavior is:

| phase | where | what happens |
|---|---|---|
| analysis | controller (pure text) | parse mints the locator (word + cwd-state + mode); routing state is EXACT-or-havoc — `exec >f`, fd-dups, `>|`, unresolvable words all decline-to-bind ⇒ total wall; the binder's reached arm is traced only where it lies wholly in THE decidable set, else deferred to probe; the tool oracle's verdict is traced structurally (reached arm, marks, argv flow — opaque commands never evaluated); the read-vs-marks detector runs |
| probe standup | target host sh | engine-owned expectation checks: capability handshake; falsification of anything the tracer pre-evaluated (host runs the same authored arm; disagreement ⇒ Refused pre-consent — checking the tracer's reading of the author's sh, never the world) |
| probe | target host sh | shipped stripped bodies run with site values: the binder body (its own fstype measurement inside), verdict bodies; rc's and emissions return through bounded intake; the host answers multiple-choice and mints no vocabulary |
| plan mint | controller | pure folding: vouches + measurements + claim unions through the universal meet; the flag gates survival; render reasons name every licensor |
| apply standup | target host sh | artifact integrity: every shipped file present, byte-exact, and *injective* — N manifest paths measure as N distinct filesystem objects, so a case-folding target that collapses two authored paths into one object stops the run before line one, byte-identical contents included |
| apply | target host sh | unanswered questions get their latest spelling: inserted guards (`( check ) || original`) for guarded sites; the admin's own hand guard ships untouched (no-double-guard) |
| `dorc why` | controller | nothing runs; the chain renders measured/vouched/claimed/consented with names; reached arms shown as code |

## §5 the-world-knowledge-law

**Syntax vs semantics.** Path *syntax* — component structure, prefix decomposition, word
expansion, dot-resolution — is language, POSIX-specified, platform-invariant, and lives once,
in the engine, falsified by the floor differential and the standups. Path *semantics* — what
a path denotes, which names co-refer, what a write means on this platform, what convergence
requires here — is world, and lives in authored bodies evaluated on hosts. The one deliberate
overlap is the decidable-set tracer: a Rust evaluation of a closed sh subset, mandatory only
where a phase-zero answer is structurally required (the load plane: "what program am I
analyzing" must be answered before any phase exists), optional speculation everywhere else,
and in all cases falsified per-host at standup and grown by-name at license-review tier
(`30P:the-load-plane-stays-correct`). **One decidable set, one fence** — this component
consumes that list and never mints a second, laxer one.

**Measure, act, arrange.** The engine's three ways of touching filesystems, none
interpretive: *measure* — compile a question, ship it to the phase where somebody knows
(authored bodies on hosts; the standup checks); *act* — for its own resources, do the thing
and verify the outcome (open the authored spelling and let the kernel resolve it, identify
files by content digest and open-time identity, never by lexically-normalized path strings;
create owned scratch and degrade on failure); *arrange* — construct geometry so authored
expressions stay true (the mirrored tree that makes `$(dirname "$0")/x` land, `30I`/`30P`).

**Expectation-setting corners.** Where the engine's own operation depends on file reality —
loading, artifact placement, scratch — it sets expectations controller-side from what it
holds, checks them host-side, and treats the host's influence as strictly boolean: stop,
never steer (`30P`'s controller-expectation/host-check pattern; `rul-admission-is-a-closed-outcome`).
The injectivity check of §4 belongs to this family.

## §6 file-identity — semantics and the v0 floor

File identity is *measured, per-aspect, and perishable* — three properties that together
dictate a very conservative floor.

**Per-aspect.** "Same file" is not one relation. Same-for-contents is referent identity (two
hardlinked names, one inode: `[ a -ef b ]` answers yes, and a write through either changes
both). Same-for-existence is directory-entry identity (`rm a` removes *a* and leaves *b*
standing — the referent answer is exactly wrong for the existence cell). Opened descriptions
are a third subject (`exec 3>p` holds a description that later `p` mutations do not touch).
The identity tier therefore carries an authored per-aspect relation mapping —
which relation each selector's comparisons consult — declared by the kind's owner like every
other vocabulary act, with the name-bias law applied (`an-name-as-contract`: spell members so
the lazy answer errs safe — a `same()` whose 0 means *same* over-collides when incomplete;
"provably distinct", the dangerous claim, must be the deliberate arm).

**Measured, including resolvable unborn referents.** Living questions subsume the platform
taxonomy: `-ef`-class checks answer hardlinks, symlinks, bind mounts, and case-folding
without knowing which mechanism is in play, because the kernel answers. A path that does not
exist yet is sited by *anchoring*: living-canonicalize its parent, look up the exact future
name (the lookup inherits the directory's folding semantics for free — the kernel folds),
and let the engine's CFG answer the program half ("does any line between here and there
target this path"). A future referent can only come to alias an existing one if some agent
creates the alias, and agents are either in-book (engine-visible, wallable) or out-of-book
drift (outside scope by `toctou-scope`, as everywhere). When two absent spellings cannot be
distinguished by parent anchoring and lookup, the answer is unknown rather than distinct.

**Perishable.** An identity answer is a point observation whose truth depends on every
directory entry, symlink, and mount used to resolve both operands. A later in-book `mv`,
`rm`, or `ln` can silently invalidate it — renaming a directory changes what every
descendant path denotes without touching their inodes. So no identity answer is ever
timeless: any consumer must either carry the answer as a backed fact invalidated by the
effective-world reach of namespace mutations, or refuse to consume answers across them.

**The v0 floor:** entry-mutating verbs (`mv`, `rm`,
`ln`, `rmdir`, `mkdir`-over) make **no at-most claims** — their oracles decline `disturbs`
entirely, so any such line is a total wall and every identity question below it is moot.
Same-kind path-distinct comparisons answer unknown ⇒ collide (no pairwise machinery is
consulted, because none exists). The floor forfeits same-kind sparing and all
namespace-tolerant precision, recorded in §9 with reds, and remains conservative.

## §7 rul-binder-claims-are-ordinary — cross-kind handling

Binder-minted claims are ordinary authored at-most claims, the same species a hand-written
`disturbs()` body mints. They enter the existing survival machinery with no binder-specific
trust class or comparison rule.

Cross-kind sparing follows `30U`. Without the footprint kind's reached finished definition,
the comparison answers *unrelated* and collides. `kind__disturbance_reaches` widens the
claim-side footprint; `kind__state_stored_only_in` adds backing-side collisions; the reached
`disturbs nothing-else` record is the completeness act that licenses cross-kind disjointness.
A redirect-derived File claim therefore spares a Service, Firewall, Package, or other
backing only when the File kind's finished definition is present and neither the reach nor
store relations add a collision. The admin's `--risk-faultless-skips` flag remains the
separate consent gate over that authored completeness claim.

## §8 invariants

1. **`inv-no-world-facts-in-engine`** — platform-conditional world-facts are never
   load-bearing in engine code; authored speech or phase-answers supply them.
2. **`inv-routing-territories-stay-partitioned`** — a simple command's mutation surface partitions by
   sh syntax: argv-side to the tool's author, routing-side to the filesystem author,
   shell-state to the engine. Enforced by input separation (tool functions see argv only;
   the binder sees locators only); no function ever receives both.
3. **`inv-binding-is-authored`** — the engine never mints world coordinates from parse.
   The locator→claim act is authored, declining, and attributed.
4. **`inv-binder-claims-stay-whole-entity`** — binder claims are selector-less without
   channel-relative tool speech; per-selector refinement never comes from engine assumption.
5. **`inv-identity-answers-are-perishable`** — no identity answer is consumed as timeless;
   consumers bound answers by the effective-world reach of namespace mutations or refuse
   across them.
6. **`inv-book-sites-feed-facts`** — only book sites feed the fact plane. Oracle-body
   redirects route to the reflexive-inertness falsifier (falsification-first, never a
   completeness gate); the report sink is exempt by construction, its value being
   engine-supplied (`rul-probe-writes-only-what-it-owns`).
7. **`inv-routing-is-exact-or-havoc`** — routing state (fd table, `exec` redirects,
   clobber modes) is modeled exactly or the site declines to bind; no middle
   (`30P:rul-load-head-is-exact-or-havoc`'s sibling, one plane over).
8. **`inv-taxonomy-is-measured-and-authored`** — ordinary-file classification is a measured
   object-type and filesystem-type allowlist inside authored bodies; unknown types fail safe.
   An engine path table or prefix denylist never forms the safety boundary.
9. **`inv-placement-is-injective`** — artifact integrity includes distinctness of
   placed files, not only per-path byte equality.
10. **`inv-one-decidable-set`** — every static speculation over authored bodies in
     this domain rides THE decidable set and its standup falsification; no second list.

## §9 deliberate-non-capture

These are deliberate limitations. Their extension paths belong in FORFEITS with reds
(`30P:rul-forfeits-carry-reds`); none changes the base semantics above.

- **Same-kind sparing** — a fact on `File:X` below a wall on `File:Y` collides even when
  the two are genuinely distinct files. Capture: the identity tier (§6's per-aspect
  relations + perishable answers). The corresponding red is a boothook-shaped XFAIL
  asserting the cross-path survival.
- **Namespace-tolerant precision** — everything below an entry-mutating verb walls totally.
  Capture: at-most claims for `mv`/`rm`-class verbs *plus* prefix-aware collision or
  answer-lifetime machinery, together (either alone is unsound).
- **Same-entity selector precision** — a fact about the written file's other aspects cannot
  survive its write (whole-entity claims). Capture: channel-relative tool speech composed
  through the routing graph, extending `30D`'s channel vocabulary.
- **The idiom's own elision** — `cat >f <<EOF` never elides on content-match at this tier;
  the semantically identical `cp f.new f` can, via its argv-keyed oracle. Capture: the
  content-establishment work already registered
  (`FORFEITS:forfeit-content-establishment-by-known-write`,
  `forfeit-file-content-facts-from-exact-checks`) plus owned-scratch payload staging and a
  structural-site convergence judgment-holder — one design problem, and every erased
  contributor (producer effect and routing effect) requires its own vouch
  (`rul-every-erased-establish-is-vouched`).
- **Dynamically-derived pair questions** — identity questions whose operands emerge from
  first-round results require a second host exchange and therefore rest at collide. The
  repeated-probing review gate governs enabling that exchange
  (`rul-repeated-probing-reviewed-before-design`).

## §10 components-and-interdependencies

Implementation work is decomposed below by dependency. This section records no schedule.
"Rides built rails" means no new transport or substrate.

- **`comp-routing-locator`** — the locator parse (word + cwd-state + mode) and the
  routing EXACT-or-havoc carve (`inv-routing-is-exact-or-havoc`). Pure kernel
  (syntax/analysis); redirects already parse as mutation sites (`an-redirection-effect`),
  this re-exposes them bindable. Self-contained. Everything else here consumes it.
- **`comp-fs-binder-member`** — role recognition, arm trace, claim mint, ship-and-readback
  on the derived-footprint rails (built: `an-derived-footprint`). Requires
  `comp-routing-locator`; consumes §7. Touches oracle + analysis + core claim
  surfaces. The authored fs stdlib file itself is stdlib-arc work, not engine work — but the
  *contract* it is written against is this unit.
- **`comp-claim-consumption`** — the settle/wall seat accepts binder claims into footprint
  unions; render reasons name the binder. The union/render half is a small delta on existing
  seats (`an-wall-topology`); the cross-kind gating of each footprint×backing pair on the
  footprint kind's finished definition is `plans/30U`'s machinery, consumed here, not
  re-derived. Requires `comp-fs-binder-member`. Until it exists, binder claims are inert and the
  authored surface cannot be meaningfully tested: any oracle-authorship exercised against
  the binder contract before this unit lands is authorship without feedback.
- **`comp-backing-detector`** — the read-vs-marks falsification detector
  (`req-verdict-marks-every-read-cell`). Analysis-plane, independent of every other unit,
  and general: it detects under-backed verdict bodies in every domain, not only fs.
  Oracle authorship performed before it exists will systematically under-mark (the
  `cmp`-shaped two-operand verdict is the natural mistake) and churn when it lands.
- **`comp-artifact-injectivity`** — the apply-standup distinctness check
  (`inv-placement-is-injective`). Engine scaffolding, independent of every unit here;
  naturally part of the multipart/artifact lane, whose integrity story is incomplete until
  it exists (identical-bytes case-fold collapse passes today's per-path byte checks).
- **`comp-identity-tier`** — the pairwise per-aspect members, the relation mapping, the
  chokepoint's relational consultation, and perishability (answers as backed facts under
  effective-world reach). Requires `comp-fs-binder-member` (claims worth comparing exist)
  and the engine-question transport that the load plane's standup verification also
  requires (`mech-two-standups`) — the two consumers share plumbing and seats, and building
  either without awareness of the other churns the same code. Until this unit exists, §6's
  v0 floor stands with zero identity machinery consulted.
- **`comp-channel-relative-speech`** — tool vocabulary for "at most my stdout's target",
  composed through the routing graph. Extends `30D`'s channel algebra; requires nothing
  above; permits relaxing `inv-binder-claims-stay-whole-entity`. Without it the whole-entity rule
  stands indefinitely, correct and blunt.
- **`comp-content-establishment`** — the registered FORFEITS capture (known-write contents
  cells, payload staging in owned scratch, the structural-site convergence judgment-holder).
  Interacts with `comp-fs-binder-member` (the establish side of the same locator species)
  and with `rul-every-erased-establish-is-vouched` (two contributors per erased site).
  Separable from everything above; the only route to §9's "idiom's own elision."

External couplings, both directions:

- **Producer predicts (the stdlib arc, behind the `30D`/`30J` predict-contract work)** —
  binder value is inert against unmodeled producers (their ⊤ keeps walls total). In the
  reverse direction: fs-stdlib authorship performed before `comp-fs-binder-member` +
  `comp-claim-consumption` + `comp-backing-detector` exist is authored against a laxer
  contract than the one that will bind it, and will churn when they land — the engine
  surface is part of what makes stdlib authorship a real test.
- **The cwd/load-plane precision residue (`an-cwd-state` full book flow, `30I` §3.2)** —
  the binder consumes cwd-state as locator input; it functions before that work lands
  (⊤-cwd locators decline, safely) and gains precision automatically after; no hard
  dependency, pure value coupling.
- **The loader standup work (`30P:mech-two-standups`)** — shares the engine-question
  transport with `comp-identity-tier`, per above; also the falsification pattern every
  traced binder arm relies on.
- **hostsim / DST** — each unit that ships host-answered questions adds an injectable
  answer species to the host fault model (forged, dropped, flapping answers); the sim never
  needs a filesystem model, only answer streams. This is a standing benefit of the
  measure-don't-model posture, and a test-surface obligation on every unit above.
