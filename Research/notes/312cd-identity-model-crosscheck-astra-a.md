# HEAD verified at review start: 6b7108b4ccabc3d961508d98a563ebc06616623b

Required/start HEAD: `6b7108b4ccabc3d961508d98a563ebc06616623b`. The start assertion passed. A late check after completing the substantive review still matched. The final check after the report's verification edit returned `b164565a9aa2620e18be47e55fbb310ddd697ea0`: **the required end assertion failed**. I made no git mutations and have not tried to restore HEAD. A read-only name diff between those commits showed only another report and `SLUGS.md`; 311 and the sources reviewed here are unchanged. The review targets the required commit; it is not a certification that HEAD remained pinned through delivery.

Static review of `Research/notes/311-identity-and-relation-model.md` at that commit. This report is the only file written. No repository scripts, fixtures, books, builds, or tasks were executed. Shell excerpts below are thought experiments, not executions. No scout was used. Claims about unverified real-tool details remain `-GUESS`.

## Findings

One as-written omission survives: the region algorithm lacks a guard for two whole-region operands. Its intended restriction is already in the history; the finding is the restriction's absence from the frozen model, not a new objection to the intended design. No other initial attack established a novel failure. The initial reasoning and its final dispositions remain below.

### Separate directory roots do not separate their trees

- kind: `as-written`
- consequence: `correctness`
- severity: major · confidence: `+SURE`
- where: `311:2.9-hierarchical-and-the-region-test`, `311:1.5-token-and-the-two-warrants`, `311:2.6-may-write-the-writeset`; the region rule entered in `63de48a281f5698ab38214a0b36508417dd2eb60`.

The book, run by the owning user, with no symlinks or mount changes:

```sh
chmod -R 0755 /srv/left
chmod -R 0700 /srv/right
```

The world: `/srv/left` and `/srv/right` are distinct ordinary directories on one Linux filesystem. `/srv/left/shared` and `/srv/right/shared` are hardlinks to one regular-file inode. There are no bind mounts, symlinks, special mode bits, concurrent changes, or failing operations. The owning user performs both commands. Initially the right tree, including that file, is mode 0700. The two directories themselves have no alternate directory entries or mount exposures in the sense of §1.5's directory closure. The first command must run; the second was converged when probed. Running both leaves the shared inode mode 0700; removing the second leaves it mode 0755. The recursive traversal and shared-permission behaviour were checked against primary manual text; nothing was executed. The tree's own hardlink identity example is `Research/plans/30T-redirect-routing-and-authored-file-semantics.md`, §6 `file-identity`.

The model's reading: `L` and `R` are mKeys of a hierarchical path mScheme of the file mSort. Their mParents-Catalog are the directory entries' catalogs. They yield distinct inode-primary mKeys `iL` and `iR` identified in the same filesystem mParent-Store `F`; `F` has a completely measured chain to the same local mRoute. The shared file is a third inode `iS`, also identified in `F`. Neither directory is an identity store for `iS`; it is a routing region containing a name for it.

The committee's speakers make these statements:

- The inode-scheme owner supplies the two uniqueness warrants for inode keys within `F`. They are true; hardlinks give two paths to one inode, not two inode numbers for it. The filesystem owner supplies the ordinary disk filesystem's non-aliasing store warrant and closed entailments.
- The path-scheme owner supplies true per-level closures for `R` and its directory ancestors. None has another mount exposure. No closure about `iS` is supplied or needed by this particular walk. A lookup of `right/shared` would withhold its closure, correctly, but the test never requests it.
- The `chmod` verb author bounds the first line's effects by `L` given whole and finishes the definition. The probe/verdict author observes the right tree and backs its converged answer by `R` given whole; its dependencies are closed. Coarse bounds are allowed by §2.9 and need not list the files they cover. Any relevant per-file mode/metadata effects are inside these bounds. The inode/file-sort author has no additional effect outside the region in this world. Filesystem-level disk entailments do not force this pair to collide: §2.6 excludes entailments at the shared ancestor `F`.
- The verdict's author vouches that the second line is unnecessary while that measured answer remains valid. It was valid at probe time. Both measurements use the same observer, so no observer-independence claim is needed.

These bounds are true even though they overlap: `iS` belongs to both. No speaker claims that the two trees are disjoint, that all descendants have unique paths, or that a directory's closure closes its descendants.

The answer: the region algorithm takes `D=L`, `x=R`. The leaf `R` is not `L`. None of `R`'s ancestor routing keys is `L`. Each is a different inode, and every required directory-route closure is true. Step 3 returns DISJOINT. Reversing the regions gives the same erroneous result: there is no shared directory ancestor equal to the other root, and neither root has a directory alias. This is precisely a region/point proof applied to a region/region question. The result should be UNKNOWN, or a collision established from their shared member.

How it breaks: §2.6 explicitly applies the region test to whole keys in writesets and readsets. Its universal meet has one coarse region pair here; it does not enumerate the pair's members. Under §2.6, the bad DISJOINT permits survival of the second line's license. Consuming that license removes the necessary second `chmod` and leaves the shared inode in the first line's mode. The admin loses a permission repair; the engineer receives a false separation despite truthful bounds and closures. This is a DISJOINT-consumer defect, not a SAME-transport claim, not a lost elision, and not a demand to teach the engine filesystem syntax.

Self-refutation checks:

- `closure-covers-the-tree`: §1.5 explicitly closes the referent at one resolved level. Strengthening it to close every descendant would change the statement, not expose a false statement in this example.
- `universal-meet-enumerates-members`: the meet is over supplied backing entries. A whole-region entry is expressly permitted. Requiring a complete member list would supply the missing restriction and lose the coarse-region case.
- `store-expiry-masks-comparison`: §3.3 says touching a primary store expires its scoped tokens, while §2.6 describes writing a child as writing its containers. Reading that combination maximally broadly kills these identities independently and leaves a guard. This is the pre-existing ambiguity recorded in `311p`, §3; I do not claim an unconditional end-to-end under-execution result under that reading. It does not make the region algorithm's positive answer true. §0 explicitly forbids a false answer even with all underlying statements true. On the narrower reading that allows the sibling-file sparing §2.6 expressly intends, this independent mask disappears.
- `point-only-is-implicit`: that is the plausible intended contract, and the historical ledger supports it. It is absent from the algorithm being promoted: its input is an mKey `x`, with no exclusion for `x` given whole. §1.1 distinguishes a referent from the set a whole key denotes; the algorithm must preserve that distinction.

No claim is made about the predecessor Rust implementation. The demonstrated result is an invalid comparison rule; the command-removal consequence is conditional on its being consumed rather than independently invalidated.

Repair: apply the four-step walk only when the other operand denotes one referent. Two whole regions remain UNKNOWN unless one side is completely expanded and every member passes the appropriate region test, or another explicit sufficient rule proves their separation. Do not reinterpret the existing per-directory closure as a subtree-wide closure.

Novelty qualification: the *need* for a region/region floor is already on record in `Research/notes/311t-cross-root-identity-ledger.md` §15: the region-against-region case stays unknown unless an owner enumerates. The finding is that this restriction never entered the model's four-step algorithm, including its introduction in `63de48a2`; it is not a claim to have newly discovered hardlinks. This is an as-written omission, not a claim that the STE100 rewrite deleted a previously present sentence.

## Initial attacks from core documents alone

### Writing one file expires its neighbours

~SUSPECT, candidate `as-written / ergonomics-forced`. Book: `printf '%s\n' new > /srv/a; chmod 600 /srv/b`. The two existing, unrelated files are on one ordinary disk filesystem; the probe has measured `/srv/b` already mode 600 and its oracle vouches the last line. The path scheme yields inode-primary keys `a` and `b`, both identified in filesystem key `F`; both inode warrants hold and `F` is non-aliasing. All declarations can be complete. Section 2.6 calls a write to `a` a write to every container in its fully qualified key, including `F`. Section 3.3 says a state mutation whose writeset touches a primary store perishes tokens scoped in it. Read literally, `b` loses its identity even though the write and backing separated. The last line must remain guarded. The antecedent exemption in 2.6 explicitly suppresses container *entailments* at the common ancestor; it does not explicitly suppress token perishing. The question to resolve is whether touching a child means touching its store for this rule, and whether the token can survive by another independent derivation. A pathname warrant may rescue some cases; find a case where it cannot truthfully do so rather than pretending every pathname is unwarranted. If broad expiry is intentional, the claim is lost elision, never under-execution.

Final disposition: already on record, not a new finding. `311p`, §3, records the earlier review's ambiguity about perishing on store touch. The later common-ancestor exclusion in §2.6 makes the interaction worth specifying, but I did not establish a new unavoidable loss beyond that recorded ambiguity. It is also the strongest independent mask for the retained region finding.

### Composite read sets lose their parts

~SUSPECT, candidate `as-written / correctness`. Book shape: `cp base.new /etc/app/base.conf; docker compose -f /etc/app/base.conf -f /etc/app/override.conf up -d`. The last command is initially converged; changing the base makes it unconverged. Real Compose behaviour is not yet checked, so this tool-specific instance is `-GUESS`. Section 2.11 gives the composite the union of its parts' *may-read sets*, which is different from the union of the parts themselves and their effective readsets. Two direct-state parts can have empty additional may-read sets. A composite read then loses both dependencies unless its owner's function or the verdict body supplies them by another rule. The composite identity still names the roles and identities correctly: the file's content changes without changing its identity. Need a fully specified separation path before calling this a wrong DISJOINT: differing leaf sorts alone give KNOWN_UNSPOKEN and safely kill. Also test the strongest refutation: 2.5's general declaration contract may require these dependencies independently, making the purported closed set false rather than exposing an all-true failure. Smallest repair if it survives: union effective part backings, including the part keys and inherited dependencies, not merely declared auxiliary may-read entries.

Final disposition: considered-and-dead as an all-true counterexample. §2.5 includes the body's marked reads, not only auxiliary may-read entries; a composite owner's closure cannot truthfully omit state on which the composite depends. I found neither a forced omission from an otherwise complete readset nor an actual DISJOINT overcoming the cross-sort floor. The union sentence deserves care during implementation, but that is not this review's finding. The illustrative Compose details remain unverified and carry no conclusion.

### Shared ancestors suppress real effects

~SUSPECT, candidate `as-written / correctness`. Book shape: `cp large.img /srv/a; df -P /srv` followed by a branch driven by the measured space. Filesystem `F` owns both file keys and a free-space cell. A write to `a` really changes the cell; the filesystem owner's truthful may-write entailment names it. Section 2.6 omits an ancestor's entailment when that ancestor is at or above the deepest common level. This may suppress the very cross-cell effect the owner supplied. Need to distinguish a real hole from the fact's may-read closure: if the free-space cell must declare the whole filesystem as a may-read entry, that independently collides and the attack dies. Also need a separating top pair: a cell directly under `F` versus a file under `F` may simply be KNOWN_UNSPOKEN. An abstract omission without an actual licensed line is not a finding.

Final disposition: considered-and-dead. A free-space dependency on filesystem contents gives the fact a colliding whole-store read entry. Without that entry the proposed closure is false. With separate file and free-space-cell sorts, I also failed to obtain DISJOINT in the first question of §2.6. History confirms that the discarded mirror rule incorrectly treated may-read as may-write; the present write entailment was introduced specifically to avoid that conflation (`311t`, §15). No evidence here justifies restoring it.

### Shape local uniqueness becomes cross shape separation

--WONDER, candidate `as-written / correctness`. Book shape: two file names, one relative and one absolute, or two accepted spellings of a service identifier, reaching the same referent. Each lookup path gives a per-shape unique-name warrant. Section 3.2's two-tops test requires one scheme but does not explicitly require one matched shape. If 1.5 means uniqueness only *among keys of that shape*, two true warrants could separate aliases across shapes. If it instead means each accepted key is the referent's only key in the whole scheme, one warrant is false and the attack is dead. The wording and revision history must decide; an ambiguity alone is not a proved fault.

Final disposition: considered-and-dead. The shape selects where the warrant is supplied, not a smaller equality universe. §1.5's assertion is that one referent has one key within the parent. Giving it for a referent with another key in the same scheme is a false statement. The example cannot satisfy the review's all-true premise.

### Upward placement forgets another route

--WONDER, candidate `as-written / correctness` or `ergonomics-forced`. A file is reached through two paths; a group lookup gives its containing tree from one path and a second containing tree from the other. Section 2.10 invokes the lookup with every held key but refuses two answers that disagree, while 1.6 and 2.9 allow multiple routes and closures enumerate aliases. Need to determine whether disagreement means different compatible members of one relation or genuinely contradictory closed answers. Only the latter should refuse. A true closed answer must cover every route in the stated sort, so incomplete path-specific answers may already violate the closure. Check real directory alias cases without alleging a false closure is true.

Final disposition: considered-and-dead. Two different partial enumerations do not justify two contradictory closed answers. A truthful closure must account for the additional routes. The bytes-only input to `:places` also raises an apparent ambiguity between same-shaped values of different schemes, but §2.10 allows a lookup to decline and supplies downward enumeration as another lane. I have not shown either an all-true wrong answer or an unavoidable authoring/network catastrophe. This is not a second finding.

### One top rule assigns separation from missing structure

--WONDER, candidate `as-written / correctness`. One primary scheme admits direct filesystem handles and handles identified within a presented filesystem. The one-top rule infers separation because another shape in the same body is identified in the opposite top's sort. Try a direct underlying handle and a handle for that same referent in the presented store. The obvious defence is 2.3: a presenting store cannot truthfully claim aliases-nothing-else. If that defence covers every concrete case, reject the attack. Need an example where every store warrant remains true; mere unfamiliarity with the rule is not a charge.

Final disposition: considered-and-dead. The proposed presenting store assigns its own keys to another store's referents. Its non-aliasing warrant is therefore false. The two-leg check in §3.2 blocks the concrete alias cases I constructed; I found no counterexample retaining both true store warrants.

## Opinions

The hostile framing is a search strategy, not evidence against the authors. The vocabulary's density is not a finding. Neither is a missing sh spelling: these tests use real commands to describe the world and the abstract contracts, and do not depend on inventing a Dorc syntax.

The retained defect is small to repair but belongs before promotion. Its seriousness comes from the false positive, not the size of the missing condition. Conversely, the report does not establish that the architecture is unbuildable, that honest engineers cannot express ordinary operations, or that the model forces additional network exchanges.

### Previously recorded issues are not findings

- `region-pair-floor-already-intended`: `311t`, §15, already states the correct region/region UNKNOWN floor. Only its absence from 311's actual rule is retained above; no novelty is claimed for the intended restriction.
- `store-token-expiry-already-recorded`: `311p`, §3, already records broad store-touch perishing as a safe-direction ambiguity.
- `sibling-cell-separation-already-missing`: `311p`, thread 3, and `311t`, §15's retraction of its proposed dissolution, already record the inability to separate some sibling singleton sorts. This is a lost-elision issue, not a newly discovered correctness failure.
- `cross-sort-completion-already-rejected`: the former claim that a finished definition itself proves separation is deliberately superseded in 311 §4.2. The current rule still needs a positive comparison. I found no reason to restore the old claim.
- `mirror-entailment-already-replaced`: `311t`, §15, records why may-read cannot be mirrored onto the write side and why explicit container may-write replaced it. The historical loop-filesystem counterexample is not a new discovery here.

## Coverage

Initial full reads completed: root README, DESIGN, IMPLEMENTATION, USER_STORY, AGENTS, KNOBS; spike/CLAUDE; Research/GOTCHAS; and 311. This initial attack ledger was written before reading Research/README, neighbours, reviews, or history. Static content was read with sanctioned `git show HEAD:<path>` because no native file reader was exposed. The target is the pinned committed text.

Subsequent reading included Research/README; the full 30U, 30W, 30S and 311p documents; relevant portions of 30T, 27C, 272, 275, 277, ANALYZER-NEEDS and ORACLE_PROVIDES; and the attack/firming and cross-root ledgers 311q and 311t. The target's `git log -p --follow` history was retrieved and searched, and the relevant rule introductions examined. This was targeted history review, not a claim to have line-reviewed every historical revision or every prior reviewer report. No contemporaneous sibling-review framing was used.

| 311 sections attacked | Question and result |
| --- | --- |
| §0; §§1.1–1.4 | Can every statement be true and a positive comparison false? Retained the distinction between a directory referent and the set its whole key denotes. |
| §§1.5–1.8; §§2.1–2.4 | Aliases across shapes, per-parent uniqueness, primary versus routing parent, presenting stores, alternative derivations. No new false SAME survived the warrant checks. |
| §1.9; §2.11 | Cell identity versus dependencies, singleton siblings, composite part backings. Known sibling-cell limitation; composite attack not established. |
| §§1.10–1.11; §2.7 | Local placeholders, route/root endings, correspondence and attribution. Unknown links and absent correspondence decline; no new positive fault established. |
| §§2.5–2.6 | Readset completeness, ancestor write entailments, shared filesystem state, two-question sparing, universal meet. The false region result can feed the sparing question; other attacks lacked a separating pair or a true closure. |
| §§2.8; 3.4 | Same object under changed observer, lent catalogs, inherited placeholders, sentinel completeness. An unchanged identifier is not a licence to ignore changed observer-state dependencies; no new all-true counterexample. |
| §§2.9–2.10 | Region/point and region/region, hardlinks below distinct directory roots, missing routes, alias closure scope, upward and downward membership, both operand directions. Retained the missing region/region precondition. |
| §§3.1–3.2 | Two-top and one-top separation, both store legs, cross-sort UNKNOWN/KNOWN_UNSPOKEN, coherent alternative identities, comparison consumers. No new false positive from the identity walk itself. |
| §3.3 | Routing, state, lifecycle invalidation; resolution versus referent state; shared-ancestor token expiry. The broad expiry ambiguity limits the end-to-end claim above. |
| §3.5 | Which author can know each statement. The retained failure needs no dishonest closure or cross-author omniscience. Other committee objections reduced to false completeness assertions or optional precision. |
| §§4.1–4.2 | Checked intentional supersessions against the named neighbours and retained welds. No separately established squares-badly finding; no dropped historical safeguard claimed. |

Exclusion check: the retained comparison error occurs at probe-derived planning time and concerns survival past an apply-time writer. Reversing the two whole operands produces the same false answer. Withholding any necessary closure restores UNKNOWN, so the charge specifically uses all true positive speech. An unaware engineer's ordinary hardlink does not invalidate the directory owner's statement. The admin pays with a missed mode repair if the licence is consumed; the engineer cannot fix that by making their already-true directory closure more truthful. On the aid side, a reported positive separation is also wrong, but no additional diagnostic behaviour is assumed. The SAME consumer is not implicated by this example. No source command, book, fixture, task, build, test, or repository script was executed; only read-only git and document tools were used. No agent was spawned and no git state was mutated.

## Considered and dead

- `different-worlds-prevent-comparison`: unlike roots/routes deliberately return UNKNOWN; this is safe, and an independent correspondence may supply another derivation, so the global-world wording alone proves no wrong answer.
- `observer-identity-misses-state`: changing group membership while retaining a uid is not enough; a truthful readset can include the relevant observer state, and 2.8 does not excuse a false closure.
- `file-link-count-suffices`: link count alone is inadequate evidence for a mount-namespace-wide alias closure, but an incorrect closure is a false authored statement; that observation alone does not refute the model's all-true law.
- `writing-expires-all-neighbours`: the potential precision loss is already recorded as store-touch ambiguity; it is not a novel finding.
- `composite-union-loses-parts`: the proposed omitted dependency defeats the truthful-closure premise, and no positive separating pair was established.
- `shared-ancestor-hides-space`: the filesystem-wide dependency collides, or the cross-sort pair lacks a separation generator.
- `shape-local-uniqueness-separates-aliases`: a per-shape warrant still claims uniqueness within the parent; the proposed alias makes that claim false.
- `upward-placement-refuses-valid-aliases`: incompatible complete answers are not both true; partial answers and alternate enumeration do not prove a forced loss.
- `one-top-separates-presented-handles`: the presenting store cannot honestly supply the required non-aliasing warrant.
- `unknown-route-vacuously-separates`: the explicit flat/absent-route floors and §2.10 default prevent a demonstrated positive comparison; an empty-quantifier suspicion alone is not enough.
- `observer-same-uid-transports`: retaining the uid does not eliminate mode/group/policy dependencies from a truthful readset or remove 27C's transport gates.
- `lookup-dependencies-force-roundtrips`: dependent lookups alone do not require another host exchange; the probe can perform dependent work on the host. No catastrophic network lower bound was established.

## Overall verdict

Do not promote the region algorithm verbatim. It admits a region/point proof where both operands denote sets, and two ordinary directory trees can then be declared disjoint despite a shared file. Restore the region/region restriction already stated in 311t and make the operand distinction explicit. That is the one defensible as-written finding from this review. The stronger claim of inevitable command removal remains `~SUSPECT` because store-token expiry may independently force a guard. I did not establish a novel failure of the intended identity architecture, a forced authoring impossibility, or a forced network catastrophe; the hostile brief does not justify pretending otherwise.
