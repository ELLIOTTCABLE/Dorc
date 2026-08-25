# 30Ta - Review: redirect routing is real; file semantics are not one relation

> Tier: ADVERSARIAL REVIEW / ADJUDICATION INPUT. LLM-authored (GPT-5.6-Sol,
> 2026-08-25) at the human's request. This reviews the uncommitted
> `30T-redirect-routing-and-authored-file-semantics.md`; it rules nothing and changes
> no register. `30T` remains proposal-tier. Root documents, `spike/CLAUDE.md`, and
> human-typed rulings outrank this report.
>
> Scope: design-level coherence and integration with the current oracle, fact,
> survival, cwd/loading, and artifact models. Spike implementation details are out.
> Confidence words are load-bearing: `+SURE` is directly supported by the cited
> design or a closed shell counterexample; `~SUSPECT` is the reviewer's synthesis.

## 0. Verdict

`30Ta:verdict-routing-survives-file-architecture-does-not`

+SURE `30T` starts from a real hole. A shell output redirection is a mutation site,
and its target is absent from the command's argv. The engine must represent that
routing edge or common file-producing shell remains a total wall
(`30T` section 1; `ANALYZER-NEEDS:an-redirection-effect`).

+SURE the narrow repair is coherent: parse a structural redirect locator; let an
authored filesystem role bind the locator into a claimed effect coordinate; retain a
total wall when binding declines; and let any downstream sparing remain behind the
ordinary `kSURVIVAL-trusted` gates. This does not require Dorc to implement a symbolic
filesystem.

+SURE `30T` as a whole is not coherent enough to adopt. Its flagship walkthrough
licenses an incorrect elision in the exact write-candidate-then-compare idiom used to
justify the feature. Separately, its one pairwise "File identity" relation conflates
path entries, current referents, and opened file descriptions, then treats referent
inequality as non-interference. Those are different semantic objects and the
distinction is load-bearing for existing survival and fact-transport consumers.

~SUSPECT "pipe-dream" is too broad for the redirect-routing seam and fair for the
larger claim that live relational questions make filesystem semantics disappear. The
questions replace hard-coded platform tables. They do not replace an ontology of what
a fact names, what a mutation affects, or what invalidates an answer.

Recommended disposition: split the proposal. Keep redirect routing as a narrow effect
and wall-precision design. Decline the broader File identity/convergence architecture
until the findings below have explicit answers.

## 1. What is coherent

`30Ta:surviving-narrow-design-core`

+SURE these parts of `30T` survive review:

- +SURE `30T:prop-routing-graph-third-edge`: channel-to-world is a real structural
  edge alongside pipes and capture.
- +SURE `30T:prop-locators-not-coordinates`: sh syntax can warrant a locator while
  authored code owns the vocabulary act that turns it into a world-state claim.
- +SURE exact-or-havoc for cwd, fd routing, and dynamic target values has the right
  failure direction: uncertainty retains the mutation and its wall.
- +SURE live pairwise questions are useful generators for existing-referent aliases.
  Hardlinks, symlinks, and some bind-mount aliases are better asked of the target than
  reconstructed from a controller-side platform table.
- +SURE authored convergence adequacy is the right ownership boundary. "Equal bytes
  are enough" is a judgment; the engine should not freeze it for every filesystem and
  every operation.
- +SURE phase routing is useful: controller-static where proven, probe-measured where
  possible, guard-time remeasurement where staleness has an identified in-book cause,
  otherwise run.

`30Ta:locator-fence-is-not-automatic-refutation`

+SURE `272:addresses-are-not-coordinates` does not, by itself, refute all of `30T`.
That fence governs locators emitted by `kind__state_stored_only_in()`: those store
locators may key context but must not silently become File facts. A redirect locator
may share a low-level representation only if its origin and permitted consumers remain
type-distinct. A general registry that allowed every locator to flow through the File
binder would violate the fence; a narrow redirect-only binder need not.

## 2. Critical findings

### `30Ta:finding-candidate-write-invalidates-comparison`

+SURE this is a direct wrong-elision in `30T`'s central walkthrough, not an exotic
filesystem corner.

`30T` proposes:

```sh
sed 's/old/new/' "$CONF.dist" >"$CONF.new"       # A: always runs
cmp -s "$CONF.new" "$CONF" || cp "$CONF.new" "$CONF" # B
```

At plan time, before A runs, let both `$CONF.new` and `$CONF` contain `old`, while
`$CONF.dist` will make A produce `new`. B's probe-time comparison says converged. At
apply time A writes `new` to `$CONF.new`. If B is elided as `30T` section 4 claims,
`$CONF` remains `old`: the needed copy did not run.

+SURE the proposed pair answer `distinct($CONF.new, $CONF)` is irrelevant. B's
convergence judgment reads both source and destination contents. A writes the source
input. Distinct source and destination paths do not preserve a predicate whose left
operand just changed.

+SURE the example `cp__is_converged()` is under-backed as written. Its `cmp` reads
`$1` and `$2`, but its only mark names `$2` (`30T` lines 150-156). Under
`ORACLE_PROVIDES:provides-binding` and `277:backing-sets`, every state cell influencing
the verdict belongs in the backing. With the required source backing present, A's
`File:$CONF.new@contents` disturbance collides directly and B must guard or run.

+SURE the project had already reached the correct result. In the originating
boothook specimen, the generated candidate stays live and the compare rechecks it at
apply; the note says explicitly that it cannot survive because it reads the file the
redirect writes (`r26-glue-strawmen/userdata-boothook-web.note.md` lines 219-238), and
that its input does not exist until the producer runs (lines 251-278). `30T` reverses
that result without adding the missing value mechanism.

+SURE fixing this requires more than referent identity. Plan-time elision would need
an exact post-A content fact: reproduce the producer through admitted oracle bytes,
prove the relevant stdout complete under `30D`, stage it in controller-owned scratch,
bind that value to A's future write, and compare the staged value with `$CONF`. This is
the unbuilt content-establishment/capture work in
`FORFEITS:forfeit-content-establishment-by-known-write` plus
`30T:ask-payload-staging-sitting`. Without it, the correct product is the admin's
in-sequence guard, not a hidden B.

+SURE this invalidates the proposal's strongest teaching claim: careful admins have
indeed supplied a valuable guard, but Dorc cannot front-lift its result across the
candidate producer merely by learning that the two pathnames are distinct.

### `30Ta:finding-file-identity-has-multiple-subjects`

+SURE there is no single operation-independent "file referent" named by a pathname.
Common sh operations act on different objects:

| spelling | affected object |
|---|---|
| `printf x >p` | the referent opened after pathname resolution |
| `rm p` | the directory entry named `p`, not the referent retained by other hardlinks |
| `ln -sf target p` | the directory entry/symlink object at `p` |
| `chmod 600 p` | normally the final followed referent |
| `mv tmp p` | source and destination directory entries; usually a replacement referent |
| `exec 3>p; printf x >&3` | an opened file description whose later writes no longer resolve `p` |

+SURE two tiny books expose both directions:

```sh
ln -s /srv/a link
printf x >link    # writes /srv/a
rm link           # removes the link, not /srv/a
```

```sh
ln /srv/data a
ln /srv/data b
rm a              # a disappears; b and /srv/data remain
```

`test a -ef b` correctly reports one current inode in the second book. That answer is
useful for a contents cell and wrong as the identity of the two pathname-existence
cells. Conversely, atomic replacement intentionally gives `p` a new inode while
preserving the logical managed entity "configuration at path p".

+SURE asking the host does not choose the semantic subject. `stat`, `lstat`, `open`,
and parent-directory lookup answer different questions. Someone must first decide
whether a coordinate denotes a directory entry, a followed object, an opened
description, or a logical file-at-path slot. That is ontology and command semantics,
not platform variance.

+SURE `30T` currently proposes one kind-level pair member and one `sm.dorc.File`
coordinate family for these uses (`30T` sections 3, 5, 7, and ask 4). Feeding its
`-ef` answer into the existing ternary comparison would let a referent answer govern
selectors whose identity is path-entry-relative. This breaks the shared-kind premise:
two oracle authors would use the same coordinate while naming different cells.

~SUSPECT a viable design needs at least an explicit separation between path-slot,
followed-referent, and opened-description identities, plus relations among them. This
could be selector-dependent comparison inside one carefully defined kind or multiple
kinds with an explicit cross-kind topology. Either route is a filesystem ontology.
The narrow redirect-routing feature can avoid this commitment by naming only an
opaque redirect effect and refusing cross-operation identity claims.

### `30Ta:finding-referent-distinctness-is-not-noninterference`

+SURE two different inodes are not necessarily independent cells in a pathname-based
state model. A namespace mutation to a directory or symlink can change what every
descendant pathname denotes without writing those descendant inodes.

```sh
# At probe time tree/f exists and already matches payload.
mv tree old
mkdir tree
cp payload tree/f
```

At probe time, `tree` and `tree/f` are existing, distinct referents. If the modeled
`mv` footprint names `File:tree` and `File:old`, a pairwise `-ef` answer says those
coordinates are disjoint from `File:tree/f`. Sparing the downstream fact would elide
the final `cp`, although `tree/f` disappeared when the namespace edge moved.

+SURE the identity query itself therefore has a backing and a lifetime. Its truth
depends on every path component, symlink traversal, mount view, and relevant directory
entry used to resolve both operands. A planned mutation to any of those is an
identified in-book cause and must invalidate the relation under
`spike/CLAUDE.md:toctou-scope` and the effective-world model in `28Q`/`30K`.

+SURE `30T`'s proposed screen - "no other line targets this path in between" - is
too weak (`30T` lines 119-128). Exact target equality misses mutations to ancestors,
renames, mount points, and symlink entries. The host can answer whether two names
co-refer now; it does not return the dependency graph that makes that answer stable.

+SURE this failure is Dorc's, not an accepted oracle lie. The pairwise oracle answered
truthfully at probe time. The engine used a point observation as a durable separation
license after a visible mutation without tracking what the observation depended on.

~SUSPECT the minimal safe choices are expensive in opposite directions: represent
identity answers as ordinary backed facts and model namespace reach; or treat every
namespace-changing mutation as a total File-identity wall. The latter preserves
correctness and gives up much of the resolver's promised value. `30T` currently does
neither, so its claim that relational questions collapse filesystem aliasing is false.

### `30Ta:finding-cross-kind-default-has-no-safe-complement`

+SURE `30T` itself identifies the cross-kind case correctly:

```sh
sysctl -w net.ipv4.ip_forward=1
printf 1 >/proc/sys/net/ipv4/ip_forward
```

The two spellings affect one state through `Sysctl` and `File` coordinates. Current
v1 has no cross-kind `same` generator, while the kind fence otherwise makes different
kinds survival-disjoint (`277` sections 1, 5, and 6). A File-bound redirect can
therefore spare a Sysctl-backed fact it changed.

+SURE an authored list of `/proc`, `/sys`, and `/dev` declines is not a conservative
closure over the path namespace. `30T:risk-cross-kind-referent-aliasing` admits the
problem: an allowlist of boring paths is not realistically enumerable, while a denylist
plus "claim the rest" treats unknown special namespaces as ordinary files.

+SURE two coherent policies exist, and `30T` chooses neither. Cross-kind comparisons
involving an automatically bound File can remain `unknown` until an explicit bridge
exists; or the filesystem author can make a positive, completeness-shaped domain claim
whose consequence is openly part of `kSURVIVAL-trusted`. Keeping today's automatic
cross-kind disjointness while saying unknown taxonomy merely declines is not coherent:
the binder cannot decline a category it failed to recognize.

~SUSPECT the first policy is the correct pre-stdlib floor. It preserves useful
within-File aliasing and refuses the systemic cross-kind knife until the already-parked
co-reference mechanism exists. It also makes the cost honest: File routing narrows File
walls without pretending every path-shaped interface belongs exclusively to File.

## 3. Artifact claim that does not hold

### `30Ta:finding-byte-manifest-does-not-prove-path-injectivity`

+SURE `30T` says a case-fold collision needs no target-filesystem model because one
manifest byte comparison will fail (`30T` lines 317-321). That is false when the
colliding files have identical bytes:

```text
oracles/Foo/entry.oracle.sh
oracles/foo/entry.oracle.sh
```

On a case-insensitive target, both path spellings may resolve to one object. If the two
expected files contain the same bytes, both per-path content checks pass. The intended
two-path artifact set has still collapsed to one target object.

+SURE byte equality proves content, not injective placement. `30P` already reserves an
exact `ArtifactSet` identity over paths and bytes (`30P` lines 691-699); the apply
standup must check the path-identity/multiplicity part as well as content. The pairwise
question machinery could supply that check, but then target-fs identity is part of
artifact verification after all.

~SUSPECT this is repairable and does not sink redirect routing. It does refute
`30T`'s stronger "act and verify means no target-fs model is needed" argument as
currently stated.

## 4. Named gaps that are not review discoveries

`30Ta:honest-open-gaps-are-not-contradictions`

+SURE the following are real but honestly disclosed by `30T`; they should not be
inflated into new findings:

- +SURE persistent `exec` redirects, fd duplication, group redirects, noclobber, and
  `>|` still need an exact-or-havoc routing boundary (`30T:risk-exec-redirect-routing-state`).
- +SURE dynamically discovered pairs may require a second exchange and correctly remain
  collide-until-reviewed (`30T:residue-second-exchange`).
- +SURE `test -ef` floor membership and portable fallback spellings need measurement
  (`30T:residue-floor-membership`).
- +SURE candidate-payload staging is unexamined (`30T:ask-payload-staging-sitting`).
- +SURE full per-line cwd flow remains a prerequisite; `30T` correctly sequences after
  it rather than inventing a redirect-local cwd model (`30T` section 9;
  `ANALYZER-NEEDS:an-cwd-state`).

+SURE these gaps make `30T` non-buildable as one plan, but they are not why it is
incoherent. The critical findings above are stronger: they break the claimed result even
if every named gap is implemented exactly as proposed.

## 5. Integration consequences

### `30Ta:consequence-survival-is-the-main-consumer`

+SURE redirect coordinate binding grants little attention value in honest-wall mode.
The producer must already be modeled, the filesystem claim must bind, relevant identity
questions must answer, and the admin must enable `kSURVIVAL-trusted` before a narrow
redirect footprint can preserve unrelated downstream elisions. `30T` eventually admits
this in `risk-billing-correction`; its teachability section still bills the payoff too
generally.

+SURE the admin's `cmp || cp` remains valuable without any of that machinery: it is an
apply-time guard and an excellent off-ramp. What it does not generally become is a
plan-time elision when its candidate is produced earlier in the same apply.

### `30Ta:consequence-every-erased-effect-needs-its-author`

+SURE a redirected command contains at least two semantic contributors: the producer's
ordinary command effects/observables and the shell's open/truncate/append effect. Eliding
the whole site requires every erased establish to carry its own reached vouch under
`spike/CLAUDE.md:rul-every-erased-establish-is-vouched`; a filesystem kind verdict cannot
silently vouch for the producer, and a tool verdict cannot receive the redirect target
through argv.

+SURE a future structural-site verdict can be one contributor to an aggregate license.
It is not, by itself, the answer. Its judgment also cannot exist until the exact candidate
value it judges is available. This is why `30T:ask-kind-species-verdict-member` and
`30T:ask-payload-staging-sitting` are one design problem rather than independent asks.

### `30Ta:consequence-context-is-part-of-every-answer`

+SURE file identity answers are scoped by host, generation, wrapper context, user,
mount/fs view, cwd, and time. `27C` and `28Q` already provide the context-slot direction.
No pair answer may be reused across those keys, and an absent fs-view model must remain
unknown rather than default-host-equivalent.

+SURE network filesystems add a separate cross-host problem. A path that names shared
state is not made host-local merely because each host runs its own `stat`. Until
`ANALYZER-NEEDS:an-cross-host-kind` has authored semantics, the safe File floor is
host-scoped and may over-run across a fleet.

## 6. Recommended recut

`30Ta:recommend-split-routing-from-file-ontology`

+SURE a small, coherent first design can be stated now:

1. Parse output redirects into a typed `RedirectTargetLocator` carrying the exact shell
   routing mode and pre-redirection cwd state.
2. Route only that locator species to one authored filesystem binder. Store locators from
   `state_stored_only_in()` remain incapable of this conversion.
3. Let a successful bind mint an attributed at-most redirect effect used for wall
   narrowing. It grants no direct mutation-elision license; downstream survival still
   requires the existing vouch, measurement, complete backing, comparison, and admin flag.
4. On unknown cwd, target expansion, fd state, taxonomy, context, or binder decline, retain
   the total wall.
5. Keep candidate-producing write-if-changed guards live. Do not claim plan-time content
   convergence from path identity.
6. Make cross-kind File comparisons `unknown` until an explicit co-reference bridge is
   designed.

~SUSPECT this narrow slice is still a high-value analyzer increment. It recovers
unrelated downstream survival for modeled `printf`/`cat`/`sed` producers without claiming
to understand their payloads, and it gives precise wall attribution even when no elision
results.

+SURE the broader architecture needs a separate filesystem-ontology sitting before any
kind-level pair member or structural-site verdict is adopted. That sitting must answer:

1. Which cells denote pathname slots, followed referents, directory entries, metadata,
   and opened descriptions?
2. Which identity relation applies per cell/operation, and which relations may feed
   transport versus survival?
3. What backs an identity answer, and which in-book namespace mutations invalidate it?
4. How are directory containment, rename, symlink replacement, bind/overlay changes, and
   atomic publication represented without deriving false separation?
5. How does a producer's complete predicted payload become an owned staged value and then
   a post-write content fact?
6. How do every producer and redirect establish contribute distinct vouches to one atomic
   replacement?
7. How does artifact standup prove path placement is injective, not merely byte-equal?

## 7. Bottom line

`30Ta:bottom-line-broad-proposal-rejected-narrow-seam-retained`

+SURE the human's incoherence reaction is justified for `30T` as written. The central
worked proof performs the cardinal sin in an ordinary, deterministic book, and the
pairwise-identity architecture uses one relation for several non-equivalent filesystem
objects.

+SURE the underlying redirect-routing observation is not a pipe-dream. It should survive
as a much narrower feature: identify where shell routing writes, let authored code name the
claimed effect, and improve wall precision without claiming that live identity questions
solve file convergence.

~SUSPECT the proposal became incoherent by asking one mechanism to buy three different
things at once: routing, identity, and convergence. Routing is structural and tractable.
Identity is cell- and operation-relative. Convergence additionally needs candidate values
and authored adequacy. Keeping those three planes separate is the recut.
