# 311b — Index identity: store, committee speech, and the missing authored promise

> AI-authored conversation ledger; design-duck sitting with the human. Continues the
> index-world discussion of `311a` §§2–5, especially
> `311a:open-bound-token-same-across-chains`. NOT a design of record or a build plan.
> **[TYPED]** records the human's substance; **[READING]** is the assistant's reading of
> prior decisions; **[PROPOSED]** is an unratified alternative or recommendation;
> **[CORRECTION]** retracts an assistant framing; **[EXPLORATORY]** is a question or
> conjecture, not a finding established by measurement. The human has not endorsed the
> alternatives as a whole; §7 records the subsequent selective acks and objections.
> Root documents and topic designs of record
> govern; older backing documents retain their own supersession qualifications.
>
> No code exploration, implementation, research dispatch, or new schedule accompanies
> this ledger. Section 6 records the first pause; section 7 banks the human's response.
> The user authorized banking this conversation directly on `ai/main`; no sibling work is active. Disposable shell
> examples and invented annotations were used in chat to explore meanings, not to settle
> language features; only short motivating examples and semantic descriptions are retained.

## §0 — Current position

**The question is not whether equal raw strings identify one mWorld.** Names already
retain their entry context. The unsettled proposal would let an owner-authored `resolve()`
bind separately keyed mPlaceholders to mTokens and use matching answers to establish
sameness across different mEntryChains. What authored speech grants that bridge, and how
broad is it?

Current direction [TYPED, provisional exploration premise; §7]: assume identity within
an authored store, with each link assigned to the human competent to describe it. Pursue
composition through existing mSort/index machinery; the human strongly disfavors a blanket
cross-context identity promise and expects useful stronger cases to fit that machinery.
Default store retains the full mEntryChain and its identity comparisons. This is NOT an
ack of a complete design; the immediate question is whether a store is an instance of an
existing mSort and how that relates to the overloaded term “store.”

## §1 — Human input and corrections to the discussion

- **`rul-disjoint-name-and-polarity` [TYPED]** — the human reports the narrow ruling:
  `__overlaps` became `__disjoint`, with rc polarity inverted for additional safety.
  Its current home is `30W:rul-disjoint-is-an-rc-predicate`; this sitting does not reopen
  its name, status contract, or speech form. Earlier references to `overlaps` are historical.
- **`rul-ground-discussion-in-strawmen` [TYPED]** — begin with a small set of glossed
  shell examples using ruled machinery. Only then invent spellings to explore alternatives.
  Jargon and project-local vocabulary must not obscure what an author actually writes or
  what a book actually does. The human explicitly invites invented syntax for exploration,
  not as an implicit ruling.
- **`cor-multiple-authors-is-not-the-problem` [CORRECTION]** — the assistant initially
  presented several authors contributing to one license as the central suspicion. The
  human points out that Dorc composes committee speech broadly and already has substantial
  prior reasoning in one particular sharp corner. Number of authors alone is not a new
  safety boundary. Whether this corner merits comparable machinery is itself in question.
- **`cor-entry-context-was-already-preserved` [CORRECTION]** — the assistant's example
  of two `id -u` `resolve()` calls returning `0` omitted the existing/proposed context qualification.
  The human recalls that names are keyed by entry path or adjacent machinery, and that
  escaping this qualification should require positive speech such as invariance. The
  corrected question is the authority for a NEW cross-chain bridge, not repair of an
  existing raw-mToken-equality default.
- **`cor-english-contract-was-not-language` [TYPED objection; accepted correction]** —
  the assistant's alternative, “the `resolve()` itself promises cross-context
  identity,” provided no way to EXPRESS that promise. Giving an operation an English
  gloss does not supply its authoring surface or delimit its store.
- **`gloss-referent-means-actual-thing`** — “K-referent” meant the actual thing a name of
  mSort K denotes; e.g. an actual account, not the string `0`. The term obscured rather
  than helped this conversation; prefer the concrete thing under discussion.

## §2 — Backing decisions and their limits

### `reading-custody-is-assumed-responsibility`

[READING] `28M` §§1–4, 7–8; `28K` §§1–5; `30J` §§3, 6, 12; and `30I` §§2.3, 3.3–3.4
supply the committee/custody precedent. Custody is responsibility taken through admitted
inclusion, not detection that two files were written by the same human. Book loads expose
code without merging speakers; dorc-lang inclusion can take custody of dependencies.
Containment is asymmetric, not equivalence: Alpha and Beta both including Common does
NOT merge Alpha and Beta. Exact guarded-source relationships have their own proof and
retain this distinction.

The earlier expensive corner involved the engine synthesizing judgments from separate
vocabularies: one author's claim mToken interpreted against another author's meanings could
spare a mCell neither meant to spare. `30J:rul-dialect-is-the-live-speakers-at-the-backing-frame`
consults one speaker closure's dialect; it does not pool words into a committee vocabulary.
Qualification is separately ruled by `30J:rul-predict-authorship-qualifies-family-vocabulary`.

`28M` also accepts cross-family probe composition, wrapper/guest contracts, and mSort-owner
answers. The reusable test is its composition-surface decomposition: enumerate the
actual granting inference, apply existing fences, then test whether the conclusion follows
from separately priced promises. Neither “several speakers” nor “one closure” substitutes
for that analysis.

### `reading-invariance-must-be-positive`

[READING] `271:rul-invariance-speech-act` moved authority out of the inferred absence of
context dependence and onto an explicit mSort-owner line. Its acknowledged benefit was
attribution and consent, not a proof that authors cease being wrong.
`271:rul-flag-is-razor-residue` distinguishes line-sayable false claims from residual
failures nobody's line can own. The later carry design in `27C` §4 still matters: mCell
identity alone does not close every measuring body's hidden dependence or access sensitivity.
Do not summarize this history as “all identity transport is unflagged.”

`281` §5 distinguishes `safe-across` (safe execution of a body in a shifted context) from
`undivided-by-transit-across` (the mSort's state is not divided by that transit). The
latter is NOT merely equality of output strings. `30W` generalizes the index vocabulary;
its unsettled clauses must be read under `311a` §§2–5, not as six untouched old rulings.

### `reading-keys-are-not-partitions`

[READING] `26Ob` §§10a–10c and §14, then `311a` §§2–5: mEntryChain mPlaceholders preserve
spelling, mVantage, and entry-definition identity; entered values retain value-plane
provenance. Different mKeys do not establish disjointness. mWorld relationships must pass
through the `compare()` chokepoint, and unknown mWorlds may still collide under mutation.
The precise mPlaceholder and filtered-comparison shape remains proposal-tier where those
ledgers say so.

An apply-standup identity check is a budgeted integrity backstop, not proof that a
cross-author identity contract means what the engine says it means. `26Ob` §14 corrected
the earlier claim that bare remote lines could never elide: a standup may `witness()` their
mWorlds. Do not resurrect that guard ceiling or the proposed perpetual re-witnessing scheme.

## §3 — The shared grounding examples

- **`example-package-state-can-be-shared`** — `dpkg -s nginx || sudo apt-get install -y nginx`:
  two execution users can discuss one package database. Wrapper speech supplies the change
  of context; mSort-owner speech warrants relevant invariance. Actual fact transport still
  requires the applicable carry and observation gates.
- **`example-crontabs-can-be-separate`** — `crontab -l; sudo crontab -l`: identical command
  and arguments can denote different crontabs. This is the ordinary multi-author
  collaboration the language already intends, not a reason to require common custody
  between sudo and crontab authors.
- **`example-equal-numbers-are-insufficient`** — `chroot /srv/red id -u; chroot /srv/blue id -u`:
  two observations of `0` do not make the execution contexts interchangeable. A particular
  account identity might agree while filesystem or application state does not. The
  entry-context qualification must not be silently dropped.
- **`example-two-routes-need-relationship`** — `docker exec web appctl configure; podman exec web appctl configure`:
  neither equal `web` spellings nor different wrapper names establish sameness or
  separation. With appropriate oracles, there may be an authored relationship to discover;
  there is no claim here that these invocations ordinarily reach the same application.

Three benefits were separated: avoiding duplicate measurement, transporting facts between
mSites, and sparing facts past a running mutation. They require distinct permissions;
none independently removes the vouch, observables, chronology, or other replacement gates.

## §4 — Alternatives explored, not adopted

### `option-retain-contextual-names`

[PROPOSED alternative] A reader's answer stays scoped by its entry context. Equal answers
from different contexts do not bridge them. Existing authored relationships must first
establish an appropriate common naming context.

Strict variant: retain mEntryChain separation wherever existing machinery supplies no
bridge. No new syntax; useful as a conservative floor, but permanently tying identity to
route syntax would deny value for equivalent routes.

Richer variant: the identity answer names the authority within which it is meaningful —
for example, an account number within one account directory. That directory has its own
identity question. If both routes reach the same directory, matching account identifiers
can support sameness. The chat's invented `identity-within` annotation explored that
meaning only; it is not a new member or grammar commitment.

Benefits: local names retain familiar meaning; different owners supply their own expertise;
ordinary authors need not serialize globally unique compound identifiers; repairs can be
directed to the mistaken account store or directory identity rather than the whole stack.

Costs and traps: store identity can recurse or cycle; stores are graphs, not necessarily
Host/Container/User containment trees; a different store does not prove a different object;
a store declaration sufficient to license sameness is stronger than a partial list of
influences. Its completeness must not be silently inferred. The implementation needs
bounded dependency evaluation, cycle-to-unknown handling, provenance, and invalidation.
Cross-route value may require several owners' descriptions, making the stdlib bootstrap
steeper even though the low in-context rung remains cheap.

### `option-author-cross-context-identity`

[PROPOSED alternative] A dedicated positive claim authorizes an identity answer to `compare()`
across mEntryChains for the reached cases. The chat's invented
`identity-equality-across-entry-paths` mark meant: “equal successful answers identify the
same account even when obtained through different entry routes.” Merely authoring a
reader or returning a mToken does NOT carry this promise by default.

Benefits: a provider may already expose the identity directly, avoiding reconstruction of
its internal stores. One mSort-owner supplies a reusable relationship for independent
wrappers and tool oracles. A wrong equal-answer conclusion can contradict a particular
positive claim, making ordinary vouch-tier treatment plausible, subject to remaining carry
gates; this is not a ruling about flag consumption.

Costs and traps: “across paths” can hide differences between aliases, users, containers,
machines, installations, snapshots, and incarnations. An identifier may be unique within an
installation and duplicated in every clone. A reader may have no way to detect the unsafe
case. Its contract must narrow store, explicitly disclose a horizon, or decline; “UUIDs
are normally unique” is not a language contract. A one-line mark can impose little typing
but enormous reasoning burden and be dangerously easy to copy from a tutorial.

Implementation is potentially simpler than recursive store mDerivation, but answers still
need compatible authority, intact acquisition, chronology, and a precise subject. Equal
bytes from unrelated readers are not compatible by default. The existing single-owner
mSort-member rule and dependency custody are relevant protections, not new mechanism gaps.

### `option-explicitly-scoped-strong-identifiers`

[PROPOSED synthesis] The useful middle may be the same semantic operation for both a
small local integer and a provider UUID: an authored name within an authored identity
domain. A broader domain buys cross-route value when its owner can actually identify it.
A universal cross-context identity promise is then the strongest case, never the default.

The hard questions are whether store identity has a non-circular anchor and whether the
existing mSort/store/invariance language already expresses the needed relationship.
Do not introduce a parallel identity-dependency system merely because the new terminology
makes it sound smaller. Conversely, do not overload store invariance if it does not make
the intended claim.

A reader-specific “comparable across this dimension” mark was another chat sketch.
Important distinction: output-byte invariance is not identity authority. A reader that
always prints `0` is output-invariant but identifies nothing across contexts. mSort-state
invariance and the identity relation of a reader's output may coincide in a particular
case; the engine must not declare them synonymous for syntactic convenience.

## §5 — Comparative reasoning and failure accounting

### `assessment-semantic-scope-before-speaker-detection`

[PROPOSED assessment] Unlike pooled selector vocabularies, the intended identity
composition can have one owner defining a relation, with other authors supplying contexts
and facts to which it applies. That resembles existing shared-mSort collaboration. Reuse
custody to select the reader and bind responsibility for its dependencies; do not initially
require common custody across reader, wrappers, and fact author.

A granting committee hazard would remain if Dorc combined several weaker promises into
an identity promise nobody stated. Test the actual inference, not the count of speakers:
can a wrong conclusion occur while every participant fulfilled the contract the language
let them express? Shared custody alone would not repair an underspecified relation.

### `assessment-repairs-follow-specific-promises`

The candidate explanation chain distinguishes these causes rather than guessing a culprit:

| failure | repair target |
|---|---|
| entry runs the reader outside the book invocation's denoted context | entry author |
| `resolve()` returns a wrong identifier | `resolve()` author |
| identifier is locally accurate but declared comparable too broadly | identity-store claim |
| a modeled change invalidated a reused answer | engine or the specific incorrect disturbance claim |
| matching one context component is promoted to whole-context equivalence | engine |
| protected fact/check has another unaccounted context dependence | applicable backing/carry contract |
| explicitly excluded outside churn occurs | stated horizon, not invented individual blame |

A false identity result does not prove the reader was the culprit. Why must retain the
entry definitions, reader and store speech, handling of remaining context differences,
and validity chain with epistemic labels. A longer scoped mDerivation can be as attributable
as a shorter broad promise; shortest-chain is not the objective.

### `assessment-safety-and-product-crosschecks`

- **Same object, different answers:** two users may address one file while only one can
  read it. Identity never substitutes for existing access-sensitive carry obligations.
- **Unequal identifiers, same object:** equality authority does not authorize disjointness.
  Aliases should lose reuse, not silently spare mutations. `__disjoint` stays a separate
  deliberate answer with its own contract.
- **Recreation and time:** logical identity and incarnation identity are distinct subjects.
  A correct identity answer is perishable; a freshened mPlaceholder must not be reconnected
  with stale mTokens after a modeled mutation. Lifecycle naming is not settled here.
- **Partial authorship:** a declined identity capability must not remove independent
  in-context convergence value. Hint the precise missing relationship, not “add this mark.”
- **New dimensions:** an axis-by-axis scheme cannot silently ignore future user-minted
  mSorts. A sufficient naming-domain claim can be broader, but that breadth is authorial
  responsibility, not a free default.
- **mWorlds still collide:** not merging their facts does not establish independence of
  their mutations; mKey inequality never proves separation.
- **Reverse consumers:** future backward relevance slicing must not treat bookkeeping
  separation as absence of dependencies. Preserve the comparison's authorized consumers.
- **Batching:** fact transport is not automatically permission to replace several entry
  executions with one; entry effects and fidelity obligations remain independently live.
- **Off-ramp:** the answer-acquisition body should be useful ordinary sh, not mainly
  certificate-formatting ceremony. Stronger annotations can erase without making a useful
  reader meaningful only inside Dorc.
- **Implementation/testing:** missing stores, incompatible readers, cycles, stale answers,
  unknown components, and cross-mWorld kill traffic admit mechanical checks. Semantic truth
  of the provider's identity promise remains contracted; clone/recreation differentials
  calibrate that promise, never establish it universally. No test or code was run here.

## §6 — First pause, before the human response

The human has asked to bank the discussion before sending their substantive response.
The assistant's alternatives and recommendation remain unratified. No new identity role,
annotation, store sort, flag policy, or custody requirement has been selected.

The present discussion hinge: can existing language express “this account identifier is
meaningful within this directory, and these entries reach the same directory” without
circularity or an implicit sufficiency claim? If it can, complete or clarify that machinery;
if it cannot, identify the missing authored sentence before choosing a mechanism.

## §7 — Human response: linked stores, repairable responsibility

The human supplied a written response through `_tmp-response.md`. Substance is banked
here so the temporary file is not a durable dependency. These are graded separately;
no blanket endorsement of the preceding assistant design was given.

- **`rul-justify-each-safety-direction` [TYPED]** — sameness is not a correctness freebie.
  Some consumers are endangered by wrong sameness, others by wrong distinctness. Before
  claiming a fail-safe direction, establish why it is safer: consumer semantics,
  blast-radius, author competence, or the enhancement rung may supply the asymmetry.
  Users and recognizable execution failures must be able to express unknown rather than
  being forced into a binary answer.
- **`lean-reject-blanket-cross-context-promises` [strong LEAN, not formal blanket nack]** —
  Option B assigns the mSort-owner responsibility for the whole conceptual hole, including
  stores and axes they may never have heard of. Merely naming a responsible author does
  not make the failure repairably attributable. Responsibility belongs at the nearest
  competent edge: the person equipped to investigate and repair that link. Conservative
  unknown/guard can be more principled than asking one owner to cover every context.
- **`lean-shortcuts-belong-inside-the-machinery` [LEAN]** — a stdlib-owned UUID mSort,
  invariant across context changes, could supply a simpler distinguishing property
  without an engine special case. This is a candidate, not a ruled UUID guarantee.
  General target: complicated nested/shared contexts may admit a simpler shared property
  establishing disjointness. The design should support such shortcuts ordinarily;
  counterexamples remain welcome. Equal versus unequal IDs and the subject of the
  comparison still need separate contracts.
- **`caution-tool-answers-need-careful-ops-authorship` [human observation, neither ack nor nack]** —
  “the provider knows” cannot justify trusting its raw interface by itself. Much of Dorc's
  purpose is knowledgeable ops authors compensating for tools' poor idempotence,
  namespace awareness, privilege handling, and configuration behavior. Delegation can
  be mechanically convenient without being semantically sufficient.
- **`ack-concrete-stores-change-the-problem` [ACKED]** — this differs from the old
  vocabulary/frame-problem committee corner: here a concrete external shared store
  exists, and different humans describe how stores ground in other stores.
  The problem is expressible composition of those descriptions, not inventing common
  meaning for unrelated words. Where no shared grounding is described, names gain no
  bridge beyond the appropriately qualified identical-entry-path case.
- **`rul-identity-is-only-an-input` [hard ACK]** — identity means exclusively identity.
  It supplies an input to other actions; those consumers determine what it permits.
  No additional behavior or fact-transport authority attaches to identity implicitly.
- **`rul-links-must-admit-unknown` [hard ACK on answer spectrum; strong suspicion on every-link scope]** —
  same/disjoint/unknown must be available. The human strongly expects the whole spectrum
  at each link so partial authorship can remain fully correct; no half-implemented link
  should be forced into a dangerous definitive answer.
- **`rul-reuse-incarnation-invalidation` [TYPED; nack of separate refresh-specific identity machinery]** —
  store identity cycling belongs in the existing incarnation model: the responsible
  mutator top-pushes the affected sub-store without disturbing unrelated state.
  Push that model into the new composition rather than add boot-specific IDs or a
  parallel refresh mechanism. Dorc need not compute the successor identity/state to
  remain safe; withholding reuse of the old state suffices. This does not settle the
  precise composition or implementation of the invalidation.
- **`rul-no-unknown-axis-totality-burden` [hard ACK]** — avoid new “and nothing else,
  including things I have never heard of” enumerations. Future user-minted dimensions
  cannot be silently excluded by a partial author's knowledge.
- **`lean-offramp-is-secondary-for-type-relations` [gentle/potential nack]** — the
  scoped identity chain may inherently be Dorc type-level information, not useful
  standalone output. Preserve useful ordinary output where natural, but do not contort
  this type-level design to manufacture off-Dorc value. Returning a useful UUID is fine;
  embedding every store into a string is not an obligation.
- **`ack-default-store-retains-the-entry-chain` [qualified ACK]** — silence licenses
  nothing; the full chain of entry steps and their identity comparisons supplies the
  default store. More authorship is required to gain authority. The human flags that
  even this may need worst-case scrutiny later; the immediate exploration is the happy
  case under referential agnosticism, not a final proof of the default.
- **`direction-assign-each-link-to-its-expert` [TYPED exploration direction]** — the
  identifier's meaning-within belongs to its describer; what that enclosing store
  itself means-within may belong to another person. Seek direct composition rules that
  assign each link structurally to its competent author. Uneven knowledge and gradual
  enhancement are primary: correct additional work buys value or nothing, never harm.
- **`direction-filesystems-and-networks-are-the-tests` [TYPED]** — exercise nesting,
  mounts, aliases, multiple targets selected by routes, and `/proc` early. These domains
  are the principal stress cases, not engine ontologies. The path through which a boot
  ID is read is a useful example of several different descriptions and kinds of
  relationship; harder non-filesystem/network counterexamples can follow later.
- **`premise-assume-identity-within-store` [TYPED, explicitly provisional]** — proceed
  with identity-within-some-store, not the entire proposed design. The human's first
  question: should that store be an existing mSort, and is it the relationship previously
  called a “store,” where one mSort contains mKeys of another? The human dislikes the
  store name. Recommendation items 1/2/3/~5/6 were effectively endorsed; ~4 was resisted
  because stronger cases should fit the existing machinery, not a separate broad promise.
- **`rul-explore-deeply-explain-in-turns` [TYPED conversation preference]** — the prior
  answer was too long. Think broadly, summarize unexpanded directions, and explain one
  angle per turn when the topic opens into substantial depth. Favor interactive design
  over exhaustive single-message presentation.

## §8 — Immediate synthesis: sorts versus store instances versus storage

[PROPOSED assistant answer, not yet human-reviewed] Reuse mSorts as the vocabulary and
owner boundary, but an actual store is an mKey/instance of an mSort, qualified by its own
context. “AccountDirectory” is the category; this directory versus that directory is the
store distinction. An account number is meaningful inside a particular directory, not
merely because both declarations mention the AccountDirectory mSort.

Keep two relationships distinguishable until shown equivalent: **naming store** (where an
identifier has meaning) and **state storage** (where the facts about that thing live).
A file path used to read a boot ID does not, by itself, make the filesystem its identity
store. A network route used to reach a directory is likewise not automatically the
account-naming authority. Existing store machinery has carried several jobs; the design
can reuse its mSorts/relations without treating access location, naming store, and physical
storage as one unqualified containment relation.

This points toward an existing mSort/mKey/index shape, not another ontology or inferred
partition. No new syntax or recursive comparison algorithm is selected here. The next
human response can resolve whether the conceptual distinction matches their meaning of
“store” before store-composition laws are developed.

## §9 — Testing whether storage can supply the store

- **`ack-store-is-a-key` [TYPED]** — the human confirms the mKey-not-mSort
  distinction; their earlier phrasing was shorthand, not a different model.
- **`question-storage-versus-naming-needs-a-counterexample` [TYPED]** — do not adopt a
  separate naming relationship merely because it sounds different. Exhibit a specific
  description that is correct and useful for an existing consumer but fails if used
  for naming. Reconcile the store member's older keying role, explicit invariance, and
  later collide-only relationships. The human recalls filesystem/proc cross-mSort
  relationships chiefly as collision propagation and asks for scrutiny in the opposite
  direction, not an assumption that dangerous cross-mSort identity already exists.
- **`reading-syntactic-siting-is-not-physical-repeatability` [assistant assessment]** —
  the proposed same-chain baseline includes resolved arguments, shell/context state,
  entry-definition identity, and unperturbed step-address mResolution. Immediate repetition
  alone proves none of a tool's world semantics: a pool-selecting entry can choose a
  different target on each invocation without a book mutation. The qualified baseline
  remains subject to the existing authored mResolution/identity contract and horizon;
  this caveat does not mandate per-mSite re-witnessing or new refresh machinery.
- **`example-shared-database-separate-tenant-stores` [assistant counterexample]** —
  consider a quiescent single-file account database whose complete model is rows keyed
  by `(tenant, local_id)`. Red's account 7 is enabled; Blue's account 7 is disabled.
  Both account stores truthfully declare the same database File as their store.
  A write to that File correctly collides with both stores' account facts under
  `30U` §7. Even a complete storage declaration does not say Red and Blue are the same
  store: reusing Red's positive account-7 result for Blue would wrongly elide Blue's
  enable operation. Every authored storage statement can be true while the proposed
  promotion from shared storage to shared naming is false. The identical filesystem
  object and local number are insufficient; tenant selection is the missing distinction.
- **`reading-counterexample-is-consumer-specific` [assistant limit]** — this is NOT an
  as-built bug or proof that new syntax is needed. Retaining the full qualified mKey
  names/context remains safe. Existing mSorts could potentially describe a finer chain
  through the database's logical tenant stores; what cannot happen silently is
  treating an existing broad File store as that finer store. `272` §5 already
  identified shared package storage as coarser than mKey identity; `30U` §7 adds
  collisions, not equality; `271`'s positive invariance line separately supplies a
  transport-related claim. The new counterexample tests the cross-chain identity
  promotion, not those guarded consumers.

## §10 — One sort namespace; coarse speech versus naming authority

- **`rul-no-automatic-sameness-from-repetition` [TYPED clarification]** — “immediate
  repetition” was the human's gloss, not a license. Under dangerous-by-default disturbance
  assumptions it usually establishes no sameness. mResolution uses the full function
  environment and value-flow-sensitive equality, never source-text equality. Expect most
  identity descriptions to do nothing without substantial contributions from several
  humans; no cheap implicit baseline is owed.
- **`rul-explanations-are-part-of-the-proof-product` [TYPED]** — the engine must construct
  a self-consistent chain from authored claims and mint Narration. This is user-exposed
  product behavior, not merely an internal invariant: each nontrivial semantic inference
  must explain its causative line or narrow collaborating set of speakers/lines.
  An unexplained semantic leap lacks both authority and the typed explanation `dorc why`
  exists to deliver. Engine mDerivations compose the claims; they do not invent tool facts.
- **`hypothesis-one-index-with-intermediate-keys` [human, tentative]** — perhaps
  the tenant counterexample's store declaration is wrong for the intended contract:
  there are two logical data/name stores, both clobbered by the database File. The
  example may support one common mSort/mKey semantics rather than a separate naming
  system. The human requests an attack on that hypothesis, not a new ruling.
- **`assessment-common-objects-distinct-relations` [assistant, unratified]** — the
  intermediate stores can indeed be ordinary mKeys of ordinary mSorts; no second
  universe of names has been justified. The engine may not infer tenants from SQL or
  distinctness from their different coordinate spellings. Authors must supply those
  relationships. A shared File can invalidate both without identifying either with it
  or with each other.
- **`assessment-coarse-storage-is-not-yet-a-contract-breach` [assistant, unratified]** —
  current backing text permits coarse storage locations: `272` §5 explicitly allows
  approximate addresses for its dependence consumer, while `30U` §7 adds collisions.
  Therefore saying both tenants' persistent state lives in this File is not, by itself,
  a false claim under that existing meaning. Requiring the intermediate store for
  naming value is coherent; retroactively calling an otherwise true coarse storage
  declaration invalid would change the contract and pressure authors beyond gradual
  enhancement. Coarse speech can retain collision value while leaving identity unknown.
  What remains to establish is whether one existing edge contract can carry both
  descriptions without its naming consumer promoting coarse containment into equality.

## §11 — Shared-volume accounts through two transport entries

[Assistant analysis, not human-ratified.] The human requests the top-of-curve case:
careful authors contributing maximally within their own knowledge, with filesystem and
transport wrinkles exposing composition seams. “Maximum effort” does not mean any
participant knows every other tool or may close an unknown dimension by silence.

### `example-shared-volume-two-account-directories`

Alpha's `/srv/people/accounts.db` and Beta's `/mnt/team/people.db` designate the SAME
shared filesystem object. A third installation may hold a byte-identical private copy.
The small file-backed tool explicitly selects `(database, directory, account-number)`;
Red's account 7 is enabled, Blue's account 7 disabled. No hidden replica protocol or
background writer is needed for the example. These are stipulated world facts for
reasoning, not facts the engine may recognize unauthored.

Three book lines motivate the distinct consumers (the `acct` interface is illustrative):
`ssh alpha acct --db /srv/people/accounts.db --directory red enable 7`;
`ssh beta acct --db /mnt/team/people.db --directory red enable 7`;
`ssh beta acct --db /mnt/team/people.db --directory blue enable 7`.

### `assessment-relations-compose-but-do-not-change-meaning`

The candidate authored account rule is scoped and concrete: same authoritative database
object, same directory key, same account number identifies the same account for this
file-backed tool. A filesystem owner supplies the database-object relationship; transport
and entry owners supply correct siting. The account author need not know NFS or ssh;
filesystem and transport authors need not know account schemas. Conversely a coarse
“state stored in this File” edge supplies collision dependence, not this addressing rule.
All objects may remain ordinary mSorts/mKeys; the unresolved question is whether the
existing authored edge semantics express the rule or need enriching.

The composed narration must retain each contributing source line. If all these local
contracts hold, the candidate equality follows without a shared custody closure across
all authors. If the account author's binding claim is absent, broad storage alone cannot
complete that mDerivation. This is an application of the committee-speech test, not a
claim that the current language already expresses every step.

### `assessment-three-consumers-different-obligations`

1. Fact reuse between Alpha/Red and Beta/Red needs account identity PLUS compatible
   measurement/judgment, complete relevant backing handling, and freshness. Same object
   is insufficient to reuse a permission-sensitive rc or an observation from another
   cache/view. The applicable `27C` carry, `30S` environment, and custody laws remain.
2. A running mutation through Beta/Red must invalidate relevant Alpha/Red facts on the
   shared object. Known-distinct hosts do not spare them. Scoped at-most claims,
   :reaches/store collision, and the risk-gated sparing consumer retain their own roles.
3. Blue must not borrow Red's answer merely because both directories live in the same
   database. Coarse writes can collide with both; fine identity needs the directory mKey.
   A private copy with identical bytes is another reason content equality is not identity.

### `assessment-entry-changes-and-index-lifetimes`

Distinct questions have distinct competent authors: ssh's name mResolution and entry
siting; the account tool's path/directory selection; filesystem object identity under the
entered mount view; the lifetime of each relationship. Host-local `st_dev`/inode pairs,
mount labels, or equal path text are not automatically cross-host object identifiers.
A remount can change a later path's mReferent without changing the old database. A
store-recreating command can invalidate old handles without requiring the engine to
compute the new handles. Reuse existing scoped incarnation/re-keying and effective reach;
add no parallel TTL or per-mSite re-`witness()` protocol. Book versus oracle execution geometry,
control-lane integrity, and source-definition identity remain independent entry obligations.

### `assessment-world-comparison-cannot-short-circuit-shared-state`

Candidate design seam, not an as-built bug: `30W` §5 promises shared-volume mReferents
across distinct hosts, but also summarizes host identity as a mWorld partition. Its latter
wording cannot be applied literally to every mCell: different hosts may reach this very
same database. Likewise a `compare()` procedure that requires ambient mWorlds to match
BEFORE permitting any owner relation to establish shared-store identity would block this
example circularly. The context-slot/comparison design must admit an authored,
mCell-relevant cross-context bridge without requiring a globally false “File is invariant
across Host” claim. It must not infer that bridge from shared storage alone either.
This is the concrete form of the original index-world question, not yet its resolution.

## §12 — Human direction for synthesis

The next `_tmp-response.md` response is banked here without retaining the temporary file
as a dependency.

- **`rul-ground-identity-in-final-outcomes` [TYPED]** — use small named end-of-chain
  cases distinguishing catastrophic wrong sameness from catastrophic wrong disjointness.
  Explain carrying measured convergence as sufficient to remove an apply, or feeding a
  predicted value into the book's observing consumer; abstract “value reuse” is not the
  product. Identity is the mKey-plane subject, not a hidden extra user value.
- **`rul-study-useful-directional-exceptions` [TYPED]** — take availability of unknown
  as read. Seek specific links or acts with a justified universally safer direction;
  these are opportunities for simpler authoring. Do not repeat unknown as a substitute
  for finding which consumer makes a direction safe.
- **`rul-stores-may-be-ordinary-sort-keys` [TYPED]** — assume generalized storage
  in arbitrary abstract mSorts, transitively reaching File or network/Host-shaped mSorts
  where appropriate. File-only is insufficient. Prior basis confirmed: `30W` §1 makes
  indexes ordinary mSorts; `311a` §4 explicitly reads `stored-in KIND` as a coordinate
  of that mSort, with its own index dependence transitively described. The precise
  naming-edge extension below remains unratified despite this generalization.
- **`question-authority-is-another-context-dimension` [potential nack]** — User/sudo
  should not become a separate abstract epistemic mechanism from Mount/File/chroot.
  Their differences are described through the same transit machinery. The assistant's
  permission-sensitive example must not imply a new parallel authority subsystem.
- **`rul-do-not-read-host-partition-as-an-override` [TYPED interpretive direction]** —
  `30W`'s host-partition summary should not override prior cross-dimension work or the
  project's known NFS/netns cases. Host identity is one contributing dimension, not a
  universal proof of state independence. The human's detailed interpretation remains
  tentative; the compatibility requirement stands.
- **`request-one-coherent-user-facing-strawman` [TYPED]** — synthesize an internally
  coherent architecture/contract centered on knowability and engine decisions. Derive
  what can be proved; ask authors only for irreducible tool knowledge. Nearby ruled
  mechanisms may be questioned explicitly if necessary. Present the final answer as
  the narrow user-facing type, benefit, timing, and dangers, not a machinery dump.

## §13 — Proposed synthesis: qualified resolution in one key graph

EVERY design choice in this section is **[PROPOSED]**, not a consequence already ruled by
§12. This is a coherent candidate for discussion, not a build brief or final spelling.

### `proposal-addresses-are-qualified-not-globally-interned`

Keep one mSort/mKey/mCell vocabulary and the existing context qualification. Add no
second identity ontology and no automatic host partition. An unresolved address retains
its full origin context, current value-flow and definition identities, and validity
conditions. Merely repeating it mints no new mWorld assertion.

The proposed minimal authored extension is to the existing mSort-owner `__resolve`:
instead of only a bare canonical mToken, it may return a canonical local name qualified
by another ordinary mKey. The parent is a full context-qualified mKey, not a raw
string or merely an mSort name. One result asserts an equality for this reached case:
“the supplied mKey is this local name within this particular store.” The name's
interpretation belongs to the child mSort's owner. There is no new blanket “all contexts
are irrelevant” promise and no need for a new `__identity` family merely to carry it.

Example meaning, NOT surface syntax: account 7 within Red within the selected database
File. The account author describes the account/directory/database address structure;
the filesystem author may resolve that File through a shared-volume mKey; the volume
owner describes its own names. Every introduced object stays qualified until another
explicit relation justifies comparing it across contexts. The engine does not inspect
SQL, mount paths, SSH options, UUID formats, or directory mKeys for mWorld semantics.

### `proposal-storage-dependence-does-not-become-equality`

`__state_stored_in` continues emitting ordinary mSort/mKey coordinates, with its
existing collision and explicit invariance consumers. A broad File store can remain a
useful, correct coarse description. The qualified-resolve result supplies what that
coarse statement cannot: the owner's rule that a particular local name in a particular
parent identifies the child. Two children sharing a parent or storage ancestor are not
therefore identical. This reuses objects and role families without conflating different
relations over them.

It may prove possible to express the same qualified-resolution statement using existing
bind/index syntax instead of extending the member's return form. That would be a surface
simplification of this proposal, not permission to infer the statement from storage.
No exact marker token, return calling convention, or wire representation is selected.

### `proposal-comparison-uses-only-licensed-implications`

A qualified mResolution is an identity equality, scoped to the reached author and live
question. The engine may compose valid equality edges: equivalent parent stores and
matching canonical local names under the same applicable mSort contract establish the
same child. Equality closure does not traverse storage or disturbance edges as if they
were identity edges. Cross-mSort identification requires explicit typed permission; a
storage link is never that permission.

Unequal canonical strings, unequal parent mKeys, or different host contexts do not
automatically establish child disjointness. The mSort-owner's `__disjoint` and other
already-authorized generators retain their exact consumer meanings. A distinctness
witness is not transitively composable like equality. In particular, `__disjoint`'s
non-disjoint/overlap answer is NOT an equality answer: overlapping regions need not be
one region. Store expansions remain universal for sparing; a single convenient route
cannot erase another relevant collision.

Consequential conclusions need well-founded mDerivations: an unresolved recursion is no
proof, and an equality assumption cannot justify its own premises. Questions are bounded
by the existing analysis/transport budgets; a query whose inputs only emerge after a
probe cannot trigger an unsanctioned extra exchange. Missing answers withhold the
corresponding optimization without changing the authored book's commands.

### `proposal-context-dimensions-share-one-calculus`

Host, User, mount view, and future authored indexes all participate in the same
context/relationship machinery. There is no separate permission-exception algebra.
The final convergence or prediction claim retains whatever contextual conditions its
existing body/carry contract requires. Identifying an account or File does not erase
those conditions, just as identifying a Host does not erase its mount distinctions.
This does not silently lift `27C` §4's existing user-axis restriction on the pure-predicate
carry route; altering that consumer would require an explicit ruling, not merely adding
an identity generator.

The kernel `compare()`s mCell-relevant authored chains, never globally settles mWorld equality
first. Thus different machines may resolve to one shared File while different User
contexts still block an unsupported inference about a check's status. An author is not
asked to enumerate all unknown future dimensions: unexplained incoming transitions
remain in the qualification; a concrete qualified-resolution claim only bridges what
its reached statement and premises actually identify.

### `proposal-incarnation-and-narration-ride-existing-rails`

mResolution observations and authored-address mDerivations retain their dependency and
validity information. A path retarget, store reset, or modelled identity-cycling act
invalidates the affected links through existing effective-mWorld reach/incarnation
machinery. No successor identity needs to be invented, and no new TTL, special boot-ID
scheme, or per-mSite witness protocol is introduced.

Each authored generator and consequential composition produces its associated Narration.
The correctness mDerivation and the decision-inert narration remain separate typed
objects: user-demanded explainability must not create a route from aid back into license
inputs. A source-to-conclusion explanation preserves the narrow collaborating set of
statements, not only the last resolver or a guessed culprit.

### `proposal-two-named-catastrophic-outcomes`

- **`goal-blue-account-must-be-enabled`** — Red/7 is converged, Blue/7 is not. A false
  same conclusion carries Red's convergence into Blue's decision and removes the
  necessary Blue enable. Naming/account store is the sharp link; same database alone
  must never supply it.
- **`goal-shared-account-must-be-reenabled`** — Alpha/Red/7 probes enabled; an earlier
  apply command through Beta disables that same shared account; a later Alpha enable
  must not remain elided. A false disjoint conclusion lets the old convergence survive
  the mutation. Host separation must never supply the spare by itself.

A direct unchanged, adequately described cross-route case can omit redundant checks or
carry convergence to a supported later mSite; that is an attention/performance benefit,
not permission to rewrite live book arguments or replay predicted bytes during apply.
The analogous prediction consumer remains under `30D` and the existing value-plane
contract, not a second identity-specific substitution mechanism.

### `proposal-useful-safe-directions-and-shortcuts`

Positive storage/reach information that ONLY adds collisions has a justified safe
direction for the sparing consumer: a false extra overlap costs an elision, never grants
one. It must not gain a second, equality-generating interpretation. The identity bridge
and disjointness grant do not enjoy that free direction.

A context-independent value mSort, such as a stdlib-described UUID value, can terminate
a recursive naming question through ordinary authored comparison/invariance rules. It
identifies UUID VALUES, not physical objects by magic. Mapping a volume/account/other
object onto that value is the particular owner's claim; whether unequal such values
prove object disjointness is separately warranted, never inferred solely because the
strings are unequal. This keeps the common shortcut inside the machinery without making
the UUID owner responsible for another tool's cloning, aliasing, or identity-reuse policy.

The main unresolved cost is authoring the positive qualified-address statement without
unnecessary ceremony. The example shows why SOME statement about naming is needed;
it does not prove a fresh syntax token or changed resolver signature is necessary.

## §14 — One authored description and alternative identity routes

- **`rul-context-dimensions-are-ordinary-types` [TYPED]** — treat the generalization as
  ruled: User, NetNS, and the other context dimensions are ordinary stdlib types, not
  engine-special types or built-in mWorld semantics. No continued qualification of that
  direction is needed.
- **`lean-enrich-one-description-retire-duplicate-authorship` [human LEAN]** — the
  human doubts a separate state-stored member is necessary; prefer one authored item
  capable of serving both identity and collision through the mKey graph. Argue for
  any retained semantic distinction, not for preserving the old member by inertia.
- **`question-optional-identifiers-add-alternative-routes` [TYPED exploration request]** —
  describe how normal mSort-based comparison and a platform/provider-specific UUID
  route coexist. The anticipated benefit is a shorter proof where the optional answer
  exists; the human requests holes in that assumption and complete exploratory functions.

### `proposal-retire-the-member-retain-weaker-speech`

[Assistant proposal] Retire separate `__state_stored_in` authorship in favor of an
expanded single `__resolve` description. Preserve collision-only dependency as a weak
statement within that description: an author may know an mKey's state is affected by
several locations without possessing a canonical address or a separating comparison.
Exact constituent addressing can project BOTH a qualified identity and collision
containment from one line, avoiding duplication for the common case. Naming through a
store is weaker than saying all of the mKey's state is a constituent of that
store; pointer-like store bindings and split storage require the weaker forms.
No old weak statement silently gains stronger authority in the migration.

The chat spelling deliberately distinguishes provisional forms rather than claiming
this many final keywords are required: `part-in` (canonical local name plus constituent
state dependence), `name-in` (canonical scoped name and mResolution dependence only),
`backed-by` (collision-only), `key-by` (an additional typed comparison mKey, under the
subject mSort owner's warrant), and `value-name` (the mSort's canonical value representation,
context-independent by the mSort owner's speech). These are prototype vocabulary only.
A UUID value is one ordinary value-mSort example; engine code recognizes no UUID format.

### `proposal-alternative-proofs-do-not-replace-dependencies`

Every returned identity route is an additional claim about the SAME subject; choosing a
cheaper successful proof never drops storage dependencies, origin-siting obligations,
measurement backings, or another required input. Distinguish alternative witnesses from
conjunctive dependencies: identity may have several proofs, while sparing must discharge
all applicable disturbance/backing obligations. A UUID shortcut is not an exception to
that universal meet.

A local scoped route being unknown is not a refutation of another complete equality or
separation proof. Mere shared coarse storage is not equality of children. Conversely,
actual same and actual disjoint claims about the same subject pair, aspect, and live
world conflict: never choose the shorter or higher-priority provider. Withhold the
contradicted authority and retain the named competing claims for why. No arbitrary
provider precedence, new function-shadow fallback, or automatic cross-author merging
is introduced. One selected mSort member composes provider helpers explicitly under
ordinary source custody and control flow.

An additional `key-by` clause conveys identity/disjointness only where its declared
contract applies to the SUBJECT mSort; it does not equate that subject with a UUID VALUE
or merge different subject mSorts carrying the same bytes. Unsupported optional keys
leave the ordinary route intact. A provider identifier that denotes only a logical
object across independently diverging replicas cannot by itself warrant equal physical
state. The final convergence/prediction comparison retains its real state backings and
contextual conditions; an identity-key clause must not bypass them.

### `proposal-local-key-comparisons-remain-owner-answered`

In the prototype, a same-parent canonical local-name match can establish sameness under
the resolved address contract. Unequal names consult that child mSort's `__disjoint`,
under the proved shared parent; they are not engine-inferred separation. Different
parents alone do not separate children: two distinct mountpoints can name the same
filesystem. An explicit typed comparison-key claim can supply an independent proof.
The normal `__disjoint` rc contract remains 0 disjoint / 1 overlap / other unknown;
an overlap answer never becomes equality.

### `proposal-prototype-scope-and-honest-limits`

Complete illustrative functions are presented in chat, not saved as a durable oracle
library. New annotated binds construct a store-qualified value; resolver arguments carry
its local name and ordered parent names with their type/provenance attachments (the
proposed ABI, not today's). Read-only `acct_query`, `fs_facts`, and `os_facts` in those
functions stand for ordinary author-owned tool adapters, never engine facilities.
They expose paths, local keys, or explicitly supported identifier values, not an engine
world-fact table. Their cross-platform implementation and validation are not supplied by
this sitting.

The filesystem example describes actual filesystem objects, not the identity of path
entries or subtree regions; those are separate subjects under `30T`'s per-aspect law.
A mountpoint is a route to its filesystem, not a globally disjoint container; mount
changes invalidate that route through existing scoped incarnation/reach. The same
calculus describes store-local kernel Users, deliberately distinguished from the
application's Account records. UUID-backed shortcut examples require an adapter that
only supplies mKeys suitable for the declared subject identity contract; ordinary cloned
filesystem UUIDs are not automatically such mKeys.
