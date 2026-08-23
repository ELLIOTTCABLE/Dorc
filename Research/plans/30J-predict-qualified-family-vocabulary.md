# 30J - Predict-qualified family vocabulary

> Tier: LLM-authored plan from the 2026-08-19 human design dialogue. The core
> ruling is **[TYPED/ACKED]** by the human. Supporting mechanics are marked
> **[ACKED]** where the dialogue settled their substance and **[PROPOSED]** where
> implementation remains builder latitude. Root human documents and
> `spike/CLAUDE.md` outrank this plan.
>
> This is the authoritative home for the selector-vocabulary ("dialect")
> corner: which authored marks mint a family's survival vocabulary (sections
> 0–9) and whose vocabulary a comparison consults (section 12). It supersedes
> `28M`'s keep-for-now verdict-word exclusion and its committee fence
> (`28M` §4, never built). Design is RULED; implementation is deliberately
> DEFERRED under section 10.

## 0. The ruling in one screen

`rul-predict-authorship-qualifies-family-vocabulary` **[TYPED/ACKED]** - For
each family under one speaker closure, an admitted `family__predict` that
genuinely models at least one invocation shape qualifies that family's selector
vocabulary for survival. Once qualified, every valid assertion, refutation, and
observation selector attributed to the family's role members or custody-valid
helpers participates in that closure's dialect for the family. Qualification is
family-wide, never branch-relative; which closure's dialect a comparison reads
is decided per frame (section 12).

Without qualification, selector marks in `family__is_converged` still name
facts, key judgments, and widen read backings. They do not warrant
selector-distinct survival. When missing qualification alone changes a relevant
survival comparison from spare to collide, the aid plane records that exact
denied opportunity and its productive remedy: author one minimal, genuine
predict model for the family. No counterfactual effect means no push warning.

The author-facing capability curve is:

```text
is_converged only
   -> judge the family's own sites
   -> guard and elide where otherwise licensed
   -> record precise cells and read backings
   -> selector-distinct survival stays conservative

first genuine predict model
   -> all of the above
   -> the family graduates into shared survival vocabulary
```

One predict arm is enough. It need not model the same branch, execute the same
commands, or read the same state as a convergence arm. A vacuous declaration
that declines every shape is not enough.

## 1. Why this is an authorship gate, not a proof

Dorc cannot prove that an oracle author understood a tool, its manual, or the
semantic relationship among selector words. Every option in this corner is a
soft heuristic over fallible authored speech. The design question is therefore
not "which rule proves correctness?" It is:

```text
which ordinary sh act best distinguishes
an admin's small selfish convergence check
from an engineer undertaking shared descriptive responsibility,
without inventing ceremony or making good factoring ineffective?
```

Writing a useful `__predict` is the chosen signal. A genuine predict model asks
the author to engage with the command's argv shapes, observable channels,
read-only stand-ins, and decline boundaries. That does not establish expertise,
but it is useful evidence of attention paid. Unlike a promise flag or magic
comment, the act also produces real value and remains ordinary sh.

The gate protects the first cross-user step. A convergence-only judgment mostly
bites its own family and its own sites. Selector-granular survival can preserve
someone else's elision past a mutation on the strength of the family's state
decomposition. P2 asks for one additional engineering act before that shared
power activates.

This is deliberately not a per-mark proof and not a confidence score. The
engine never claims "this selector is correct because predict exists." It says
only "this family crossed the authored capability rung at which its selector
vocabulary participates in the already-risk-gated survival mechanism."

## 2. The alternatives and why P2 won

The dialogue compared four nested policies:

```text
P0  only selectors derived from predict participate
P1  plus convergence selectors for invocation shapes also modeled by predict
P2  plus every convergence/read selector in a family having one genuine predict
P3  every valid selector mark participates, predict or not
```

### 2.1 P0 is too conservative

P0 is the current implementation floor. It makes genuine convergence-specific
read state permanently imprecise unless predict happens to repeat the same
selector vocabulary.

That repetition is often unnatural. `__predict` describes what an invocation
does and how its observables can be reproduced. `__is_converged` asks what can be
read to judge whether running it again is worthwhile. A command's best
convergence query may inspect state the modeled operation never directly reads.

Requiring every such read to reappear in predict pressures authors toward fake
or ceremonial prediction arms. That is the wrong direction for an honesty-first
oracle contract.

### 2.2 P1 conflates the roles branch-by-branch

P1 looks attractive for command families whose subcommands are almost separate
tools, such as `git push` and `git gc`. Attention paid to one branch is weak
evidence of attention paid to another.

The cost is larger and more general. Many mutative invocations are easy to judge
and unreasonable or impossible to model faithfully. Convergence checks are
expected to exercise different commands, tools, and state from predict. Requiring
matching predict coverage makes well-authored backing ineffective precisely
where `__is_converged` provides the most value.

P1 also needs a user-visible notion of "the same invocation shape" across two
independent sh argparsers. That shape may depend on global flags, nested verbs,
values, helpers, and partial declines. A local warning can name one argv, but the
contract would still require authors to understand a non-local branch-alignment
rule.

### 2.3 P3 removes the author-side gate

P3 has the simplest mark semantics, but it allows a convergence-only author to
enter selector-granular survival without undertaking any descriptive-model work.
The admin's `--risk-faultless-skips` consent is only one side of the trust
transaction. P3 provides no author-side signal at the point where the author's
vocabulary first preserves other lines' elisions.

The gate's understanding cost is lower than P3's simplicity argument initially
suggested. Its denial is observable only when it changes a real survival
comparison, and the aid plane can explain that one local loss. Everywhere the
mark would have had no survival effect, the user has nothing new to learn.

### 2.4 P2 is the product-shaped middle

P2 uses the family, already Dorc's vocabulary and coherence unit, as the
responsibility unit. It preserves ordinary factoring, allows convergence to own
its genuine read set, and gives the gradual-enhancement curve one teachable
capability transition. It accepts that a carefully modeled branch can qualify a
quickly authored branch a year later. That is the residual price of choosing a
simple speaker heuristic over a branch-sensitive attention analysis.

The residual risk remains heavily bounded: survival still requires a reached
vouch, a measured fact, a running wall, an at-most footprint, custody, the
per-speaker dialect (section 12), the universal backing meet, and the admin's
explicit risk flag. P2 reduces none of those independent gates.

## 3. Precise semantics

### 3.1 Qualification key

Qualification is keyed by `(speaker closure, family)`.

- A family is the existing name-derived command or kind family.
- A speaker closure is the existing asymmetric custody closure minted by admitted
  marked-file sourcing.
- CLI co-loading and book sourcing mint no speaker and therefore no
  qualification relation.
- An ambient or cross-custody helper remains legal sh; missing custody only
  withholds its marks from this family's qualified vocabulary and narrates the
  lost capability. It never turns the load into refusal
  (`30I:rul-cross-custody-distinction-is-narrative`).
- Another family's predict never qualifies this family, even when both call the
  same helper or use the same kind and selector spelling.
- Two speakers never pool: a comparison consults exactly one closure's
  vocabulary, the one live at the backing's frame (section 12).

### 3.2 Genuine predict seed

The seed is not the presence of a function name. `family__predict() { return 2; }`
does not qualify anything.

A qualifying predict must be admitted under the ordinary oracle contract,
somewhere live under the function environment, and genuinely model at least one
invocation shape. "Genuinely model" means at least one non-declining modeled path
provides a real predict claim under the existing per-channel vocabulary. It need
not cover every channel or every family branch.

The exact code-level witness is implementation latitude, but these properties
are not:

```text
function name alone                 insufficient
dead/never-bound predict            insufficient
whole-shape decline on every path   insufficient
one honest partial model            sufficient
one honest delegation model         sufficient
```

### 3.3 What enrolls after qualification

After qualification, the family's valid selector marks enroll according to the
standing mark grammar:

- assertion selectors (`:` / `asserts`);
- refutation selectors (`:!` / `refutes`);
- observation/read selectors (`:?` / `reads`);
- the same marks inside custody-valid helpers attributed to the family's role
  members.

Claim and `disturbs` emissions still never mint vocabulary. They may reuse a
token already present in the backing family's dialect, which is how external
tools make precise at-most claims without inserting terminology into another
family's vocabulary.

Read marks enroll because they are not decoration. They define members of the
fact's backing set. Without backing-side vocabulary membership, a carefully
declared read remains entity-granular and every same-entity disturbance collides.
The read may still invalidate more before qualification; qualification allows
known-distinct sibling cells to spare it.

### 3.4 What marks do before qualification

Verdict and read marks remain useful before qualification:

- a verdict selector can key the convergence fact;
- an observation selector can widen that fact's backing;
- exact coordinates and source attribution remain available to analysis and aid;
- same-cell and whole-entity disturbances still collide;
- no selector-distinct survival rests on those dormant vocabulary words.

This is a soft denial of one capability, not a parse error and not a refusal of
the oracle. It follows the ordinary gradual-enhancement posture: preserve what
the author safely earned and explain the next available rung only when it matters.

### 3.5 Qualification is family-wide

Once the seed exists, qualification covers the family's selectors across every
role branch. Predict does not need to model the convergence branch, the internal
query used by convergence, or the selector itself.

This is intentional. The heuristic classifies the speaker's level of engagement
with the family, not the semantic relationship between two role paths. It avoids
requiring predict to restate read backing or to lie about shapes it cannot model.

## 4. Grounding strawmen

### 4.1 The low rung stays useful

```sh
foo__is_converged() {
   foo status "$@"
}
```

This markless floor can vouch, guard, and elide its own sites. It has no selector
vocabulary to misuse and P2 adds no concept the author needs to learn.

### 4.2 Dormant typed convergence vocabulary

```sh
# dorc-lang/v0.2
foo__is_converged() {
   case "$1" in
   start)
      unit : org.example.Service = "$2"
      foo is-running -- "$unit" \
         : org.example.Service:"$unit"@running
      ;;
   *) return 2 ;;
   esac
}
```

`@running` keys the convergence fact. With no genuine `foo__predict`, it does not
yet distinguish sibling selectors for survival.

### 4.3 One real predict graduates the family

```sh
foo__predict() {
   case "$1" in
   status)
      foo "$@"
      ;;
   *) return 2 ;;
   esac
}
```

The status branch is enough. Foo's existing selectors, including `@running`, now
form Foo's survival vocabulary. The predict branch need not mention `@running`.

### 4.4 Convergence-specific read backing

```sh
foo__predict() {
   case "$1" in
   output-generator)
      model_output_generation "$2"
      ;;
   *) return 2 ;;
   esac
}

foo__is_converged() {
   case "$1" in
   repair)
      item : org.example.Output = "$2"
      foo quick-query "$item" \
         : org.example.Output:"$item"@healthy \
         :? org.example.Output:"$item"@happiness
      ;;
   *) return 2 ;;
   esac
}
```

Predict models neither `repair` nor `quick-query`. P2 still enrolls `@healthy`
and `@happiness`: the speaker demonstrated predict-tier authorship for Foo, and
convergence owns the genuine read set of its judgment.

### 4.5 The accepted broad-family residue

```sh
git__predict() {
   case "$1" in
   push) model_git_push "$@" ;;
   *) return 2 ;;
   esac
}

git__is_converged() {
   case "$1" in
   gc)
      repo : org.example.Repository = "$2"
      git count-objects -v \
         : org.example.Repository:"$repo"@packed
      ;;
   *) return 2 ;;
   esac
}
```

The carefully authored push model qualifies the later GC selector. P2 does not
claim the push work proved GC correct. It accepts this family-level false positive
to avoid branch-sensitive capability semantics and dishonest mirrored models.

### 4.6 External users may reuse but not insert vocabulary

A footprint from another family may spell Foo's `@happiness` token. The claim is
interpreted against the backing Foo family's dialect. It can collide precisely
with `@happiness` or spare known-distinct Foo siblings after qualification. The
external family cannot mint a new Foo token by using a novel word.

Likewise, another family's predict does not qualify Foo. Producing bytes consumed
inside Foo's convergence body and authoring what those bytes mean are separate
acts; Foo remains the backing's semantic speaker.

## 5. Aid contract

### 5.1 Counterfactual denial narrative

When a survival comparison collides, the engine can compute whether the only
missing conjunct was family vocabulary qualification. Only that exact case mints
the qualification-denied narrative.

The structured payload must identify:

- the affected site and backing family;
- the selector pair that remained conservative;
- that the family has no genuine qualifying predict;
- the survival that would otherwise have been available;
- the productive remedy category: author one genuine predict model for any
  supported family shape.

The render wording remains loom-authored and `KNOBS:kFLOW`-unwelded. The payload
must not prescribe a fake predict arm for the current convergence branch.

### 5.2 First-predict activation account

Adding the first qualifying predict can activate existing verdict/read selectors
non-locally. The authoring surface therefore owes an inspectable account of which
family selectors changed from dormant to active. Exact push policy is selection-
layer work; the underlying before/after set must be computable and available to
lint and pull surfaces.

### 5.3 No-warning cells

No push item is warranted when:

- survival is disabled;
- no running wall reaches the fact;
- the comparison would collide even after qualification;
- the dialect has fewer than two relevant distinct tokens;
- another independent gate already withholds the survival;
- the family is already qualified.

These exclusions keep the teaching cost local to denied value rather than turning
every verdict-only oracle into a standing nag.

## 6. Safety and exclusion checks

### 6.1 Probe versus apply

Qualification is controller-static authored-source analysis. It ships no new
probe bytes and grants no probe-execution license. It changes only the selector
dialect consumed by flag-gated apply-plan survival.

Predict never licenses elision. The reached `__is_converged` vouch, concrete host
measurement, and all ordinary replacement gates remain independently necessary.

### 6.2 Admin versus engineer

The engineer supplies the qualification signal by writing predict and the
vocabulary by writing marks. The admin separately enables the survival tier for
one invocation. Neither act substitutes for the other.

A convergence-only admin receives all lower-rung value and is asked for more work
only when their dormant vocabulary would recover a real survival.

### 6.3 Reliable versus unreliable oracle

For a careful oracle, P2 makes normal factoring and genuine read backing useful.
For an unreliable oracle, P2 can activate poorly considered selectors outside the
modeled predict branch. This is the accepted residual. The family/speaker key,
the per-speaker dialect, risk flag, attribution chain, and next-run remeasurement
bound it; they do not remove it.

### 6.4 Reverse consumer direction

Qualification feeds survival sparing only. It does not feed coordinate identity,
transport, context carry, fact generation, resolver equivalence, or any other
consumer. The ternary compare consumer map remains unchanged: dialect-derived
disjointness is inadmissible for transport.

### 6.5 Cross-family use

Claims may reuse backing vocabulary because the backing family remains the
interpreter. Facts and backings from another family never import the claimant's
dialect. Cross-family registration remains the separate, unresolved kind-owner
question from `28M` section 11.

## 7. Compatibility posture

The project is unreleased, so current strawman behavior carries no compatibility
claim. P2 replaces P0 in place before publication.

After publication, family qualification and selector enrollment are authority
semantics, not mere plan formatting. A later narrowing would remove earned value;
a later widening could grant old text new survival power. Either direction requires
an explicit language-version or contract event rather than shelter under
`plan-as-API` instability.

The role names remain permanent; this ruling changes only how their authored marks
participate in the current dialect version before that version is public.

## 8. Implementation shape, without committing architecture

The dialect gains one key and loses one fold:

```text
Dialect: (speaker closure, family, kind) -> set(selector)
```

No branch key or invocation-region identity is needed; the closure is the
speaker the positional function environment already resolves per frame. The
change is at the producer and at the lookup's key:

1. Derive one qualification result per `(speaker closure, family)`.
2. Collect valid selector marks attributable to that family's role members and
   custody-valid helpers, per closure.
3. Mint those selectors only when the closure is qualified for the family.
4. Preserve predict-derived selectors as part of the same qualified set.
5. Retire `oracle::build_dialect`'s whole-unit last-wins minting source; the
   comparison looks the dialect up by the closure live at the backing's frame
   (section 12).
6. Expose dormant-versus-active sets to the aid plane.
7. Add the counterfactual denial at the survival comparison.

The ternary comparison, universal backing meet, resolver, reference model, and
survival witness need no semantic redesign; the comparison's dialect argument
changes key, nothing else. Any implementation that changes those consumers
further must justify why that was insufficient rather than treating this ruling
as license for adjacent algebra work.

No qualification state is durable. It is re-derived from the authored source
snapshot on every run. It neither touches `kSTATE` nor expands whylog contents.

## 9. Acceptance obligations

The build must pin at least these worlds:

1. A verdict-only family mints facts and widened backings but no selector-distinct
   survival.
2. A vacuous or never-live predict does not qualify the family.
3. One honest partial predict arm qualifies every valid family selector, including
   selectors appearing only in unrelated verdict branches.
4. Verdict-side `:?` reads enroll after qualification and participate in the
   universal backing meet.
5. A custody-valid helper's marks qualify with the family; a merely co-loaded or
   cross-custody helper does not.
6. Another family's predict does not qualify this family.
7. External `disturbs` claims may reuse active backing-family tokens but cannot
   insert novel tokens.
8. An admin-blessed replacement (`unset -f family__predict; . ./local/…`) with
   its own genuine predict: a claim spelled in the replaced closure's word
   collides against the replacement's fact below the bless, and vice versa — two
   closures' words never pool at one frame.
9. Flag-off plans are byte-identical.
10. Qualification changes no probe artifact, vouch, fact identity, transport, or
    context-carry answer.
11. The counterfactual narrative fires only when qualification alone would change
    the survival result.
12. The first-predict activation account lists the exact dormant selectors becoming
    active.

Tests must include the broad-family residue (`git push` qualifying `git gc`) as an
intentional positive case, not merely a tolerated side effect. Otherwise a future
"precision" repair will silently turn P2 back into P1.

## 10. Sequencing: ruled now, build later

`rul-family-vocabulary-build-is-not-an-r30-blocker` **[ACKED direction]** - The
implementation does not block the current r30 static-loading and bundle-emission
work.

Grounds:

- P0, the current behavior, fails conservatively toward collision and guard/run.
- `30I` explicitly excludes blessing reach and verdict-word enrollment from its
  lane.
- P2 changes the dialect's producer and lookup key, not the load, bundle,
  artifact, or locator models.
- speaker/custody closures already exist; no queued r30 mechanic needs a
  qualification witness.
- `plans/30L` edits book-custody regions and consumes the existing dialect/survival
  answer; its universal route meet neither needs nor mints a P2 qualification witness.
- no wire, durable, cache, or public format must reserve a field now.
- the project has no real stdlib or published third-party oracle corpus whose
  behavior currently depends on the ruling.

Building it inside the current 30I push would increase concurrency and review
surface without unblocking that push. Do not absorb it opportunistically.

The implementation deadline is the earliest of:

1. immediately before or with the starter-oracle stdlib revival;
2. before a field trial intended to evaluate real selector-granular survival
   authoring;
3. before third-party oracle publication or stabilization of the public oracle
   contract.

If a hand-authored real oracle trial occurs before the stdlib revival, this work
moves ahead of that trial. A trial run under P0 would measure the rejected policy
and train authors toward duplicate or fake predict marks.

Until the trigger fires, build no qualification key, helper-grade cache, dormant
selector registry, or warning scaffold. `30G:b6-blessing-keying-family-rooted` was
correct to refuse a key without its consumer. The ruled design and acceptance
matrix here are the reservation.

## 11. Closed questions and remaining implementation latitude

Closed by this sitting:

- P2 over P0, P1, and P3;
- family-wide rather than branch-relative qualification;
- one genuine partial predict is sufficient;
- verdict assertion/refutation and read selectors participate after qualification;
- helpers follow speaker custody and family attribution;
- cross-family predicts never qualify;
- consequential denial receives counterfactual aid;
- implementation may wait until the real-oracle boundary.

Implementation latitude:

- the exact private qualification witness type and crate home;
- how an admitted predict proves one genuinely modeled path without duplicating
  the predict evaluator;
- how helper-contained marks are collected from the existing closure machinery;
- which push surface, if any, shows first-predict activation by default;
- exact diagnostics and arrangement wording through the loom;
- test allocation among unit, property, DST, and whole-product specimens.

None of those may weaken the family-wide semantics into branch qualification or
widen the family/speaker key into closure-global or cross-family permission.

## 12. Whose dialect a comparison reads

`rul-dialect-is-the-live-speakers-at-the-backing-frame` **[ACKED 2026-08-23]** -
A dialect is a property of one speaker closure, per family. A survival comparison
consults exactly one: the dialect of the closure whose `family__` definition is
live at the protected fact's frame, as the positional function environment
resolves it. The wall's claim contributes only a token; it is interpreted in that
one vocabulary or it collides. Two closures' words never pool into one set, at
any frame; swapping the live definition (a bless, a subshell re-source, an
`unset -f`) swaps the dialect with it.

This dissolves two earlier questions rather than answering them:

- A committee fence (`28M` §4: "role members spanning speakers ⇒ the family spares
  nothing") is unnecessary: one definition is live per frame, so one speaker's
  vocabulary is consulted, and the composite "A's word ≠ B's word" cannot be
  minted. Co-speakerhood — a stranger's `disturbs` spelling the backing family's
  token — stays what it was: the token is read in the backing speaker's
  vocabulary (section 6.5).
- Which position governs `claim@p × backing@q` (`28Q:pin-two-position-sparing`):
  the backing's frame. A token minted by any closure other than the one live
  there is not in the dialect, so it collides by construction; no separate
  agreement rule is needed.

What remains open elsewhere: cross-family registration (section 6.5), the
kind-owner question of whether two families may deliberately co-refer.

Build: with the rest of this plan (section 10); the `28T` sparing mini-model is
the proof home, and it gates promoting plural-idiom books beyond the census's
enumerated blessed idioms. As-built until then, `oracle::build_dialect` mints one
`(family, kind)` set per unit from a last-wins file, which can pool two
closures' words below an admin bless — a too-large dialect spares more. The
shadow refusal (`core::ContestedFamilies`) keeps two live minting members out of
one frame except by that explicit bless, which bounds the exposure to
admin-consented plurality.
