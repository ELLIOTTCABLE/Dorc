# 311b — Index identity: scope, committee speech, and the missing authored promise

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

**The question is not whether equal raw strings identify one world.** Names already
retain their entry context. The unsettled proposal would let an owner-authored identity
read bind separately keyed placeholders to tokens and use matching answers to establish
sameness across different entry chains. What authored speech grants that bridge, and how
broad is it?

Current direction [TYPED, provisional exploration premise; §7]: assume identity within
an authored scope, with each link assigned to the human competent to describe it. Pursue
composition through existing kind/index machinery; the human strongly disfavors a blanket
cross-context identity promise and expects useful stronger cases to fit that machinery.
Default scope retains the full entry chain and its identity comparisons. This is NOT an
ack of a complete design; the immediate question is whether a scope is an instance of an
existing kind and how that relates to the overloaded term “store.”

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
  of two `id -u` reads returning `0` omitted the existing/proposed context qualification.
  The human recalls that names are keyed by entry path or adjacent machinery, and that
  escaping this qualification should require positive speech such as invariance. The
  corrected question is the authority for a NEW cross-chain bridge, not repair of an
  existing raw-token-equality default.
- **`cor-english-contract-was-not-language` [TYPED objection; accepted correction]** —
  the assistant's alternative, “the identity reader itself promises cross-context
  identity,” provided no way to EXPRESS that promise. Giving an operation an English
  gloss does not supply its authoring surface or delimit its scope.
- **`gloss-referent-means-actual-thing`** — “K-referent” meant the actual thing a name of
  kind K denotes; e.g. an actual account, not the string `0`. The term obscured rather
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
vocabularies: one author's claim token interpreted against another author's meanings could
spare a cell neither meant to spare. `30J:rul-dialect-is-the-live-speakers-at-the-backing-frame`
consults one speaker closure's dialect; it does not pool words into a committee vocabulary.
Qualification is separately ruled by `30J:rul-predict-authorship-qualifies-family-vocabulary`.

`28M` also accepts cross-family probe composition, wrapper/guest contracts, and kind-owner
answers. The reusable test is its composition-surface decomposition: enumerate the
actual granting inference, apply existing fences, then test whether the conclusion follows
from separately priced promises. Neither “several speakers” nor “one closure” substitutes
for that analysis.

### `reading-invariance-must-be-positive`

[READING] `271:rul-invariance-speech-act` moved authority out of the inferred absence of
context dependence and onto an explicit kind-owner line. Its acknowledged benefit was
attribution and consent, not a proof that authors cease being wrong.
`271:rul-flag-is-razor-residue` distinguishes line-sayable false claims from residual
failures nobody's line can own. The later carry design in `27C` §4 still matters: cell
identity alone does not close every measuring body's hidden dependence or access sensitivity.
Do not summarize this history as “all identity transport is unflagged.”

`281` §5 distinguishes `safe-across` (safe execution of a body in a shifted context) from
`undivided-by-transit-across` (the kind's state is not divided by that transit). The
latter is NOT merely equality of output strings. `30W` generalizes the index vocabulary;
its unsettled clauses must be read under `311a` §§2–5, not as six untouched old rulings.

### `reading-keys-are-not-partitions`

[READING] `26Ob` §§10a–10c and §14, then `311a` §§2–5: entry-chain placeholders preserve
spelling, vantage, and entry-definition identity; entered values retain value-plane
provenance. Different keys do not establish disjointness. World relationships must pass
through the comparison chokepoint, and unknown worlds may still collide under mutation.
The precise placeholder and filtered-comparison shape remains proposal-tier where those
ledgers say so.

An apply-standup identity check is a budgeted integrity backstop, not proof that a
cross-author identity contract means what the engine says it means. `26Ob` §14 corrected
the earlier claim that bare remote lines could never elide: a standup may witness their
worlds. Do not resurrect that guard ceiling or the proposed perpetual re-witnessing scheme.

## §3 — The shared grounding examples

- **`example-package-state-can-be-shared`** — `dpkg -s nginx || sudo apt-get install -y nginx`:
  two execution users can discuss one package database. Wrapper speech supplies the change
  of context; kind-owner speech warrants relevant invariance. Actual fact transport still
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
sites, and sparing facts past a running mutation. They require distinct permissions;
none independently removes the vouch, observables, chronology, or other replacement gates.

## §4 — Alternatives explored, not adopted

### `option-retain-contextual-names`

[PROPOSED alternative] A reader's answer stays scoped by its entry context. Equal answers
from different contexts do not bridge them. Existing authored relationships must first
establish an appropriate common naming context.

Strict variant: retain entry-chain separation wherever existing machinery supplies no
bridge. No new syntax; useful as a conservative floor, but permanently tying identity to
route syntax would deny value for equivalent routes.

Richer variant: the identity answer names the authority within which it is meaningful —
for example, an account number within one account directory. That directory has its own
identity question. If both routes reach the same directory, matching account identifiers
can support sameness. The chat's invented `identity-within` annotation explored that
meaning only; it is not a new member or grammar commitment.

Benefits: local names retain familiar meaning; different owners supply their own expertise;
ordinary authors need not serialize globally unique compound identifiers; repairs can be
directed to the mistaken account scope or directory identity rather than the whole stack.

Costs and traps: scope identity can recurse or cycle; scopes are graphs, not necessarily
Host/Container/User containment trees; a different scope does not prove a different object;
a scope declaration sufficient to license sameness is stronger than a partial list of
influences. Its completeness must not be silently inferred. The implementation needs
bounded dependency evaluation, cycle-to-unknown handling, provenance, and invalidation.
Cross-route value may require several owners' descriptions, making the stdlib bootstrap
steeper even though the low in-context rung remains cheap.

### `option-author-cross-context-identity`

[PROPOSED alternative] A dedicated positive claim authorizes an identity answer to compare
across entry chains for the reached cases. The chat's invented
`identity-equality-across-entry-paths` mark meant: “equal successful answers identify the
same account even when obtained through different entry routes.” Merely authoring a
reader or returning a token does NOT carry this promise by default.

Benefits: a provider may already expose the identity directly, avoiding reconstruction of
its internal stores. One kind-owner supplies a reusable relationship for independent
wrappers and tool oracles. A wrong equal-answer conclusion can contradict a particular
positive claim, making ordinary vouch-tier treatment plausible, subject to remaining carry
gates; this is not a ruling about flag consumption.

Costs and traps: “across paths” can hide differences between aliases, users, containers,
machines, installations, snapshots, and incarnations. An identifier may be unique within an
installation and duplicated in every clone. A reader may have no way to detect the unsafe
case. Its contract must narrow scope, explicitly disclose a horizon, or decline; “UUIDs
are normally unique” is not a language contract. A one-line mark can impose little typing
but enormous reasoning burden and be dangerously easy to copy from a tutorial.

Implementation is potentially simpler than recursive scope derivation, but answers still
need compatible authority, intact acquisition, chronology, and a precise subject. Equal
bytes from unrelated readers are not compatible by default. The existing single-owner
kind-member rule and dependency custody are relevant protections, not new mechanism gaps.

### `option-explicitly-scoped-strong-identifiers`

[PROPOSED synthesis] The useful middle may be the same semantic operation for both a
small local integer and a provider UUID: an authored name within an authored identity
domain. A broader domain buys cross-route value when its owner can actually identify it.
A universal cross-context identity promise is then the strongest case, never the default.

The hard questions are whether scope identity has a non-circular anchor and whether the
existing kind/store/invariance language already expresses the needed relationship.
Do not introduce a parallel identity-dependency system merely because the new terminology
makes it sound smaller. Conversely, do not overload store invariance if it does not make
the intended claim.

A reader-specific “comparable across this dimension” mark was another chat sketch.
Important distinction: output-byte invariance is not identity authority. A reader that
always prints `0` is output-invariant but identifies nothing across contexts. Kind-state
invariance and the identity relation of a reader's output may coincide in a particular
case; the engine must not declare them synonymous for syntactic convenience.

## §5 — Comparative reasoning and failure accounting

### `assessment-semantic-scope-before-speaker-detection`

[PROPOSED assessment] Unlike pooled selector vocabularies, the intended identity
composition can have one owner defining a relation, with other authors supplying contexts
and facts to which it applies. That resembles existing shared-kind collaboration. Reuse
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
| reader returns a wrong identifier | identity reader |
| identifier is locally accurate but declared comparable too broadly | identity-scope claim |
| a modeled change invalidated a reused answer | engine or the specific incorrect disturbance claim |
| matching one context component is promoted to whole-context equivalence | engine |
| protected fact/check has another unaccounted context dependence | applicable backing/carry contract |
| explicitly excluded outside churn occurs | stated horizon, not invented individual blame |

A false identity result does not prove the reader was the culprit. Why must retain the
entry definitions, reader and scope speech, handling of remaining context differences,
and validity chain with epistemic labels. A longer scoped derivation can be as attributable
as a shorter broad promise; shortest-chain is not the objective.

### `assessment-safety-and-product-crosschecks`

- **Same object, different answers:** two users may address one file while only one can
  read it. Identity never substitutes for existing access-sensitive carry obligations.
- **Unequal identifiers, same object:** equality authority does not authorize disjointness.
  Aliases should lose reuse, not silently spare mutations. `__disjoint` stays a separate
  deliberate answer with its own contract.
- **Recreation and time:** logical identity and incarnation identity are distinct subjects.
  A correct identity answer is perishable; a freshened placeholder must not be reconnected
  with stale tokens after a modeled mutation. Lifecycle naming is not settled here.
- **Partial authorship:** a declined identity capability must not remove independent
  in-context convergence value. Hint the precise missing relationship, not “add this mark.”
- **New dimensions:** an axis-by-axis scheme cannot silently ignore future user-minted
  kinds. A sufficient naming-domain claim can be broader, but that breadth is authorial
  responsibility, not a free default.
- **Worlds still collide:** not merging their facts does not establish independence of
  their mutations; key inequality never proves separation.
- **Reverse consumers:** future backward relevance slicing must not treat bookkeeping
  separation as absence of dependencies. Preserve the comparison's authorized consumers.
- **Batching:** fact transport is not automatically permission to replace several entry
  executions with one; entry effects and fidelity obligations remain independently live.
- **Off-ramp:** the answer-acquisition body should be useful ordinary sh, not mainly
  certificate-formatting ceremony. Stronger annotations can erase without making a useful
  reader meaningful only inside Dorc.
- **Implementation/testing:** missing scopes, incompatible readers, cycles, stale answers,
  unknown components, and cross-world kill traffic admit mechanical checks. Semantic truth
  of the provider's identity promise remains contracted; clone/recreation differentials
  calibrate that promise, never establish it universally. No test or code was run here.

## §6 — First pause, before the human response

The human has asked to bank the discussion before sending their substantive response.
The assistant's alternatives and recommendation remain unratified. No new identity role,
annotation, scope kind, flag policy, or custody requirement has been selected.

The present discussion hinge: can existing language express “this account identifier is
meaningful within this directory, and these entries reach the same directory” without
circularity or an implicit sufficiency claim? If it can, complete or clarify that machinery;
if it cannot, identify the missing authored sentence before choosing a mechanism.

## §7 — Human response: linked namespaces, repairable responsibility

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
  Option B assigns the kind-owner responsibility for the whole conceptual hole, including
  stores and axes they may never have heard of. Merely naming a responsible author does
  not make the failure repairably attributable. Responsibility belongs at the nearest
  competent edge: the person equipped to investigate and repair that link. Conservative
  unknown/guard can be more principled than asking one owner to cover every context.
- **`lean-shortcuts-belong-inside-the-machinery` [LEAN]** — a stdlib-owned UUID kind,
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
- **`ack-concrete-namespaces-change-the-problem` [ACKED]** — this differs from the old
  vocabulary/frame-problem committee corner: here a concrete external shared namespace
  exists, and different humans describe how namespaces ground in other namespaces.
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
  namespace identity cycling belongs in the existing incarnation model: the responsible
  mutator top-pushes the affected sub-namespace without disturbing unrelated state.
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
  embedding every scope into a string is not an obligation.
- **`ack-default-scope-retains-the-entry-chain` [qualified ACK]** — silence licenses
  nothing; the full chain of entry steps and their identity comparisons supplies the
  default scope. More authorship is required to gain authority. The human flags that
  even this may need worst-case scrutiny later; the immediate exploration is the happy
  case under referential agnosticism, not a final proof of the default.
- **`direction-assign-each-link-to-its-expert` [TYPED exploration direction]** — the
  identifier's meaning-within belongs to its describer; what that enclosing namespace
  itself means-within may belong to another person. Seek direct composition rules that
  assign each link structurally to its competent author. Uneven knowledge and gradual
  enhancement are primary: correct additional work buys value or nothing, never harm.
- **`direction-filesystems-and-networks-are-the-tests` [TYPED]** — exercise nesting,
  mounts, aliases, multiple targets selected by routes, and `/proc` early. These domains
  are the principal stress cases, not engine ontologies. The path through which a boot
  ID is read is a useful example of several different descriptions and kinds of
  relationship; harder non-filesystem/network counterexamples can follow later.
- **`premise-assume-identity-within-scope` [TYPED, explicitly provisional]** — proceed
  with identity-within-some-scope, not the entire proposed design. The human's first
  question: should that scope be an existing kind, and is it the relationship previously
  called a “store,” where one kind contains entities of another? The human dislikes the
  store name. Recommendation items 1/2/3/~5/6 were effectively endorsed; ~4 was resisted
  because stronger cases should fit the existing machinery, not a separate broad promise.
- **`rul-explore-deeply-explain-in-turns` [TYPED conversation preference]** — the prior
  answer was too long. Think broadly, summarize unexpanded directions, and explain one
  angle per turn when the topic opens into substantial depth. Favor interactive design
  over exhaustive single-message presentation.

## §8 — Immediate synthesis: kinds versus scope instances versus storage

[PROPOSED assistant answer, not yet human-reviewed] Reuse kinds as the vocabulary and
owner boundary, but an actual scope is an entity/instance of a kind, qualified by its own
context. “AccountDirectory” is the category; this directory versus that directory is the
scope distinction. An account number is meaningful inside a particular directory, not
merely because both declarations mention the AccountDirectory kind.

Keep two relationships distinguishable until shown equivalent: **naming scope** (where an
identifier has meaning) and **state storage** (where the facts about that thing live).
A file path used to read a boot ID does not, by itself, make the filesystem its identity
scope. A network route used to reach a directory is likewise not automatically the
account-naming authority. Existing store machinery has carried several jobs; the design
can reuse its kinds/relations without treating access location, naming scope, and physical
storage as one unqualified containment relation.

This points toward an existing kind/entity/index shape, not another ontology or inferred
partition. No new syntax or recursive comparison algorithm is selected here. The next
human response can resolve whether the conceptual distinction matches their meaning of
“store” before scope-composition laws are developed.
