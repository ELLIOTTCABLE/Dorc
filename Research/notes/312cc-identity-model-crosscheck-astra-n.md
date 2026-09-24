# Identity model review

HEAD at start: `6b7108b4ccabc3d961508d98a563ebc06616623b` (verified with `git rev-parse HEAD`).
HEAD at end: `6b7108b4ccabc3d961508d98a563ebc06616623b` (verified with `git rev-parse HEAD`; matches the required commit and the start).

Static design review of `Research/notes/311-identity-and-relation-model.md`. Only this report is written. No repository program, fixture, book, build, or task is executed; no git mutation or external access is used.

## Initial independent reasoning

This section preserves the initial record written after reading the eight requested core documents and all of 311, before opening corpus neighbours or history. These are candidates, not final adjudications. Their final dispositions appear under “Considered and dead.”

### Singleton siblings cannot separate

~SUSPECT, as-written / ergonomics-forced. Consider a root-run book:

```sh
chmod 0600 /etc/example.conf
printf '%s\n' ready > /etc/example.conf
```

World: the regular file already contains `ready\n`, has mode 0644, and the first command really runs. The second command has a reached author vouch that already-equal content licenses its elision; its stream and status requirements are separately satisfied. No hooks or concurrent writers participate. The filesystem oracle describes the chmod as a write to the file's mode cell; the binder describes the redirection's content write. The file identity is an inode mKey in a warranted filesystem mParent-Store. Its mode and contents are two singleton mSorts identified in that same file, with truthful closed dependencies and finished write definitions. Both parties describe their own operations, without needing each other.

At §3.2 the common ancestor is the file. Both singleton cells are their own tops, so the one-top rule does not apply. They have no mSchemes (§1.9), so the two-top rule does not apply. Different mSorts then read KNOWN_UNSPOKEN. §2.6 requires DISJOINT before consulting read/write independence. Thus complete knowledge cannot spare the content fact. This is lost elision, not under-execution. The admin retains a guard or unnecessary line; the engineer has no additional permitted declaration that distinguishes the sibling cells. Repair direction: give sibling cells an attributable separation route without treating arbitrary distinct vocabulary as distinct state. Need check whether this exact failure is already recorded and whether the predecessor's selector distinction supplied the missing authority.

### Container touching perishes siblings

~SUSPECT, as-written / ergonomics-forced. Consider:

```sh
printf '%s\n' new > /srv/a
printf '%s\n' ready > /srv/b
```

World: two distinct regular-file inodes in one ordinary filesystem; only `a` differs from desired content. The primary inode scheme has both uniqueness warrants, and the filesystem warrants aliases-nothing-else. The first writeset is truthfully bounded to `a`; the second line has a truthful convergence vouch over `b`. Initially the two inode mKeys compare DISJOINT. But §2.6 says a write to an mKey is also a write to every container on its fully qualified chain. §3.3 says a state mutation touching a mParent-Store perishes tokens scoped in it. Taken together, the write touches the filesystem and perishes `b`'s inode token, defeating the separation just earned. §2.6's common-ancestor exception explicitly suppresses a container's entailment in a pair test; it is unclear whether that exception also limits token perishing. Need distinguish a forced global loss from an ambiguity with an already intended narrow reading. Repair direction: distinguish changing state inside a store from changing the lookup/identity of every member; provide an attributable identity-changing effect for copy-up and similar cases.

### Scope choices do not prove exclusion

~SUSPECT, as-written / correctness. The one-top rule in §3.2 warrants separation without either uniqueness warrant: a primary scheme has one shape identified in a store and another shape directly under the common ancestor. It appears to treat the latter's different identity scope as a statement that its referent cannot be a member of that store. Yet §2.2 explicitly says identifying is not containing, and §3.2 admits multiple derivations of one referent (provider ID versus machine ID; filehandle versus inode). Need construct a fully grounded book where a provider identifier and a more locally scoped identifier are two true descriptions of the same object, their parent declarations are both true, and the store's outgoing-alias warrant is true. A counterexample depending on an invalid choice of primary scheme or an unmentioned transit will be rejected. Repair direction if it survives: require a positive exclusion declaration rather than interpreting a choice of scope as disjoint membership.

### Local shape uniqueness versus global uniqueness

--WONDER: §1.5 allows warrants per matched shape, but §3.2 compares distinct tokens from two shapes. If unique-name promises uniqueness only within each shape, different canonical encodings can both be locally unique and still alias. If it promises uniqueness against every other shape too, the counterexample is a false warrant and dies. Need read history before promoting this ambiguity.

### Composite dependencies omit direct parts

--WONDER: §2.11 takes the union of the parts' may-read sets, which appears to omit the parts themselves and their resolutions. A composite predicate over two files depends on those files even when they have no additional may-read entries. However, independent marked reads may supply the dependencies already, and KNOWN_UNSPOKEN may prevent a false sparing regardless. Do not promote without a complete path through §2.6.

### Correspondence dependencies need a seat

--WONDER: a transition owner's true guest-pid/host-pid correspondence can stop holding after an in-book lifecycle operation. §3.3 explicitly perishes resolutions and tokens; the corresponding lifecycle of an independent mCorrespondence derivation is less explicit. A finding must identify a surviving authoritative derivation after all dependent mResolutions have been withdrawn, not merely ask for implementation detail.

## Findings

### Whole regions escape descendant overlap

- kind: `as-written`
- consequence: `correctness`
- severity: major · confidence: `+SURE` for the false region comparison
- where: `311:2.9-hierarchical-and-the-region-test`, especially outcome 3 and “Only x's mTraversals are walked”; `311:1.5-token-and-the-two-warrants`; `311:2.6-may-write-the-writeset`.

The book:

```sh
chmod -R g-w /srv/a
chmod -R g+w /srv/b
```

The world: one ordinary local filesystem, one mount namespace, no symlinks, bind mounts, other mount exposures, concurrent writers, or lifecycle changes. The directories `/srv/a` and `/srv/b` are distinct. Each contains a regular file named `shared`; these two entries are hardlinks to one inode H. Before the book, B and everything beneath it are group-writable; H has mode 0664. The first line really runs, removing group-write from H. The second must restore that permission through B. The admin wants B's tree group-writable, independently of the first line's request about A.

Tool-behaviour qualification: `-GUESS` for the unchecked real-tool instantiation that this host's `chmod -R` performs those ordinary recursive permission changes. No installed command or external manual was consulted. The load-bearing abstract world is explicitly the one just stated: a mutation of the members reached below A, followed by a fact about members reached below B. The shared-inode case is independently in `GOTCHAS:containment-by-path-prefix-lies`, the hardlink that lies both inside and outside a subtree, and `30T` §6, which distinguishes shared-inode state from directory-entry existence. No prospective Dorc spelling is needed.

The model's reading:

1. **Objects and scopes.** Let the example mScheme `sm.Path` resolve hierarchically into primary mScheme `sm.Inode` of mSort `sm.File`. The filesystem F is the shared mParent-Store of the inode mKeys. Its own chain is known and shared. Directory roots A and B have distinct inode tokens, as do their proper path ancestors. H is another inode in F, reached by the two descendant entries. Secondary catalog parents describe the directory lookups; they do not replace the primary inode's identifying parent F.
2. **Who says what.** The path owner supplies the traversal and truthful `alias nothing-else` closures for B and its ancestors: each of those directory referents has exactly one entry in the whole lookup instance. The inode owner supplies both uniqueness warrants in F. The filesystem owner supplies its applicable store warrant and finished effects. The chmod author bounds the first line's effects by A given whole, and supplies a read-only convergence answer for the second line, backed by B given whole, with a reached vouch and completed read/write declarations. All other observable and permission conditions are satisfied. Nobody asserts that every descendant of B has a unique path. Such an assertion would be false for H, but §1.5's closure expressly concerns the one resolved level, not its descendants.
3. **Read and write regions.** Call the denotations of A-whole and B-whole R(A) and R(B). H belongs to both. The convergence cell can be `B@sm.TreeGroupWritable`: a singleton mSort under B, with no own mScheme, whose owner declares the dependency on B-whole. Its marked read and that may-read entry belong to the fact's readset. A whole-region may-read entry is explicitly admitted by §2.9; the convergence author need not name every inode separately. Thus a truthful coarse readset may contain B-whole rather than an enumerated entry for H. The corresponding coarse writeset contains A-whole. This supplies a concrete region pair to the meet, regardless of any additional conservative pair elsewhere in the full readset.
4. **The returned answer.** In §2.9 set D = A-whole and x = B-whole. B's leaf is not A. No level of B's lookup traversal is A. Every such level compares DISJOINT with A through the common filesystem and unequal warranted inode tokens. Each level has its true alias closure. Outcome 3 therefore returns DISJOINT. Walking the two legs in the opposite order does not repair this example: A's root and ancestors also have unique entries and omit B.

Why wrong or forced:

`+SURE`: the rule establishes that B's root referent is outside A's region. It does not establish that B's descendants are outside A's region. Yet x may itself be given whole: neither the rule's domain nor its DISJOINT arm excludes that case. The test has substituted separation of the two roots for separation of their reachable members. The all-true statements above permit a false DISJOINT because H is in the intersection. This is the model's own hardlink gotcha recurring one level below the keys it examines.

The consumer is DISJOINT for sparing, not SAME for transport. A B-whole convergence answer initially says the second line is unnecessary. The first line falsifies it through H. §2.6's read/write meet, if it consumes this region answer, incorrectly preserves that license; the second chmod is then wrongly removed and B's file remains without group-write. No author omitted a write or read: the coarse regions include H on both sides. The admin gets the wrong permissions. The engineer cannot repair the inference by correcting a false warrant, because the root-level warrants are true; their current escape is withholding useful closure or enumerating descendants even though the published rule does not require that enumeration.

There is a material limit to this end-to-end walk. §3.3's already-recorded broad “touches a mParent-Store” reading could perish every inode token after the first line and therefore block the elision. Additional conservative readset pairs can also block a particular license. I am not claiming that every interpretation of those separate gates must remove the second line. The finding is the false DISJOINT that §2.9 itself returns. A separate refusal can mask its consumer effect; it cannot make the answer true or satisfy §0's all-statements-true test. The wrongly removed chmod describes the consequence when the other gates permit consumption, not an independently demonstrated inevitability under every reading of them. In the stipulated world the recursive chmod changes permissions, not the roots' names or their alias closures.

`spike/CLAUDE: set-lifting-universal-meet` does not repair this. The backing set here has a B-whole entry. Universally testing every entry is insufficient when that entry's own region comparison is false. The rule must quantify correctly over the denotation of the entry before a meet over entries helps.

A simpler directional check reaches the same omission without hardlinks: D is `/srv/tree/sub` given whole and x is `/srv/tree` given whole. Walking only x's ancestors never visits D and can return DISJOINT; reversing them finds the ancestor and returns UNKNOWN. This variant exposes the missing second-region condition. The sibling-hardlink world above is stronger because either argument order fails.

Repair: restrict this refinement to one whole region against a non-region key. If both entries are given whole, retain UNKNOWN unless adequate owner-supplied enumeration or another explicit universal coverage proof establishes separation. Merely walking both roots' ancestor chains is insufficient, as the sibling-hardlink world shows. This is a small domain restriction, not a request to restore an authored pairwise region predicate.

Novelty and history: `311t` §15, `311t:rul-the-outside-rule-with-the-alias-closure`, already states that region against region stays unknown unless an owner enumerates. I am not presenting that safety rule as new. The finding is that 311 does not carry it: the exception is absent from the region rule introduced in commit `63de48a281f5698ab38214a0b36508417dd2eb60`, and remains absent at the reviewed HEAD. I found no prior report identifying that omission. This is `as-written`, not a claim that the final prose rewrite dropped a guard previously present in 311.

## Considered and dead

These entries adjudicate the independent candidates above and the additional paths investigated afterward. “Dead” means not promoted as a novel supported finding, not necessarily that the underlying concern is resolved.

### Singleton siblings already recorded

`+SURE`: the initial mode/content example reaches KNOWN_UNSPOKEN under the current §3.2 rules. It is lost elision, not incorrect execution. Its product consequence is already recorded in `311p:thr-identifying-store-makes-siblings-collide`, concerning mode versus contents and enabled versus active. The changed mechanism is also acknowledged in `311t` §14's shared-keyspace discussion and §15's singleton discussion. The original placement-overlap explanation no longer applies, but re-reporting the surviving product limitation as newly discovered would mislead. Considered-and-dead as a novel finding.

### Broad perishing already recorded

`~SUSPECT`: the initial `a`/`b` book still exposes tension between §2.6's ancestor touch and §3.3's store-wide token perishing. `311p` §3 already records the store-touch ambiguity as a safe-direction concern. There is no new all-true wrong-execution counterexample here. It remains a qualification on the region finding's eventual consumer effect, not a second finding.

### Scope distinction lacks counterexample

`+SURE`: the current one-top rule is narrower than the initial suspicion allowed. Both distinctions must be made within the one primary scheme's own definition; a separate provider identifier or arbitrary second author cannot manufacture its premise. `311t` §14 explicitly investigates this distinction and gives the omission inside one definition its exclusion meaning. My proposed account had not supplied that meaning truthfully. Considered-and-dead: no counterexample in which all the actual required statements are true. This does not certify every possible store model.

### Shape uniqueness is global

`+SURE`: §1.5 says one referent has one key within its parent. Declaring that warrant on a matched path does not weaken its meaning to “one key among this path's encodings.” Two encodings of the same referent in one parent therefore refute the proposed warrant, not the model. Considered-and-dead.

### Composite omission lacks execution

`~SUSPECT`: §2.11's union of parts' may-read sets deserves precision about direct part state, but the initial suspicion never completed a counterexample through the independent marked-read obligations and §2.6's initial identity comparison. In particular, supplying a convergence body that actually reads a part while omitting that read would violate the author's contract (`30T` §3.1), and a different-sort pair may already collide as KNOWN_UNSPOKEN. Considered-and-dead as a finding; I did not establish that the model forces an unsafe composite answer.

### Correspondence loses dependent authority

`+SURE`: §3.3 withdraws dependent SAME conclusions when endpoint resolutions perish; §3.2's OR over derivations does not make a dependent derivation timeless. `kill "$pid"; ...` or a namespace reconfiguration alone does not establish a correspondence surviving after its endpoint authority has been withdrawn. I did not find the additional independent surviving derivation needed for the initial attack. Considered-and-dead.

### Nested backing closure unproven

`~SUSPECT`: a file in a loop filesystem whose image is itself in another loop filesystem tests whether may-read closure follows the newly reached entry's own chain. An outer `dd` write could otherwise miss the innermost fact. However, §2.6 demands no joining write path, §3.5 says the engine chains, and the text calls the readset closed. I could not establish that a one-step-only expansion is required rather than an inadequate interpretation. Container effects and perishing provide further conservative exits. Considered-and-dead; no claimed forced false survival.

### Captured footprint already backed

`+SURE`: an in-book package update can stale a probe-emitted file list, but that is not intrinsically a hole in identity. `275` §4–§5 makes captured values' backing fresh through the apply-time binding point; §3.3 invalidates lookups that read changed state. A stale `dpkg -L` result cannot simply be assumed to stay authoritative. I did not establish a demanded exception to those rules. Considered-and-dead.

### Other namespace limitation recorded

`+SURE`: `311t:rul-the-outside-rule-with-the-alias-closure` explicitly leaves a daemon writing from another mount namespace uncovered. The region's root and the tested member can be exposed through different namespace trees, so a local closure must not casually become a global route claim. This is already recorded; I did not promote it or use it in the finding, whose entire world is one namespace.

## Strong properties and coverage

`+SURE` about the reading performed: README, DESIGN, IMPLEMENTATION, USER_STORY, AGENTS, KNOBS, spike/CLAUDE, Research/GOTCHAS, and all of 311 were read before the initial report was written. Research/README, neighbours, prior adjudication, and history were opened afterward. No scout was used. Repository code was not executed or reviewed as an implementation of 311. The configured review-pass skill file was inaccessible, and no repository copy was found; the final report hygiene check was performed directly.

| 311 sections | Review focus and outcome |
| --- | --- |
| §0 | All-true-statements criterion, reachable true answers, single-party knowledge. The region finding fails the first; losses of elision were kept separate. |
| §1.1–§1.4 | Referent/state/value distinction, sort and scheme ownership, primary uniqueness, plan-time keys and late measurements. No additional supported finding. |
| §1.5–§1.7 | Direction of both warrants, per-shape meaning, alias-closure scope, catalog/store separation, traversal dependencies. These supply the region counterexample's true premises. |
| §1.8–§1.11 | Fully qualified chains, alternative derivations, singleton cells, vantage/route distinction, ambient placeholders, claim species. Singleton limitation and broad perishing cross-checked against prior records. |
| §2.1–§2.4 | Yield composition, identity scopes, store self-knowledge, looked-up-in relations. One-top and shape-warrant attacks did not survive their actual premises. |
| §2.5–§2.6 | Marked reads, may-read closure, completed writesets, container entailments and common-level exception, both sparing questions. Checked whether another closure repairs or masks the region answer. |
| §2.7–§2.8 | Correspondence authority and observer dependence; checked lifecycle invalidation and measured-in-context fallback. No independent false SAME established. |
| §2.9–§2.10 | Whole-entry meaning, all four region outcomes, descendant aliases, both argument directions, missing routes and placing lookups. Main finding is here. |
| §2.11 | Composite identities and dependencies. Candidate retained above as considered-and-dead. |
| §3.1–§3.5 | Recursive identity, four answers, both separation cases, coherence, SAME composition, three kinds of perishing, lends and sentinels, attribution. No second supported finding. |
| §4.1–§4.2 | Imported boundaries and deliberate supersessions. Did not count retained predecessor language as a new inconsistency. |

Secondary comparison covered `30U` and `30S`, relevant sections of `30W`, `30T`, `27C`, `272`, `277`, and `275`, and the relevant contract entries in ANALYZER-NEEDS and ORACLE_PROVIDES. `311p` and pertinent sections of `311t` supplied prior-finding and settlement checks. History inspection used read-only `git log`/`git show`, including the introduction of the region refinement and subsequent repairs. This was targeted history inspection, not an assertion that every line of every revision was audited. Searches for overlap with prior findings were also targeted; novelty is bounded by that coverage.

The strong properties that held under these attacks:

- `+SURE`: separate equality and inequality warrants prevent “this lookup is functional” from becoming a uniqueness-of-naming claim. The hardlink attack cannot pass through the inequality warrant on path spellings themselves.
- `+SURE`: KNOWN_UNSPOKEN never sparing, even after a finished definition, closes the predecessor's route from unrelated vocabulary to unjustified separation. This is a deliberate §4.2 change, not a regression finding.
- `+SURE`: stores describe their own aliasing construction, while routing closures describe the resolved referent's routes. Those are different promises; the main finding does not require either author to lie.
- `+SURE`: resolving names and invalidating the resulting authority remain separate from computing successor identities. In-book name replacement does not silently carry an old SAME forward.
- `+SURE`: observer dependence defaults to refusal of cross-observer carry; wrapper silence and unknown links remain conservative. The primary measure-in-context lane in `27C` remains available.
- `~SUSPECT`: the committee law's local ownership is substantially stronger than the predecessor's finished-definition-only separation. The present defect is in engine composition of truthful local claims, precisely the remaining place that law requires scrutiny.

The exclusion check did not remove the main finding: it is an apply-time sparing error backed by probe-time true statements; reversing the two region operands leaves the hardlink example unsafe; an admin and independent path/filesystem/tool engineers can each be unaware of the others' statements; no unreliable measurement or outside writer is required. Aid-only use cannot itself under-execute, but it would still receive the false separation answer. No forced extra network round trip, impossible authoring obligation, or additional performance finding was established.

## Overall assessment

`~SUSPECT`: promotion should wait for the whole-region domain restriction to appear in 311 itself. The repair is small and already has a recorded design precedent. `+SURE`: as written, §2.9 can return a false DISJOINT from truthful root identities and truthful per-level closures. Most initial suspicions either met existing conservative gates or reproduced recorded limitations; they do not justify a broader rejection of the model. This review establishes one localized correctness defect, not whole-model soundness or an exhaustive clearance of every GOTCHAS case.
