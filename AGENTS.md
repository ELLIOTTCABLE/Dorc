## Reading guide

Follow this pattern to get bootstrapped on the codebase and design, *no matter what your task is*. You *must* understand the project before you can work on it effectively.

1. README, DESIGN, IMPLEMENTATION, and TODO are human-written; USER_STORY, KNOBS, ANALYZER-NEEDS, AID-NEEDS, and TODO-ADDTL are LLM-generated but reviewed/maintained and relatively authoritative.
  - ALWAYS read them first if they are not in-context (important context in those is *not* duplicated into this AGENTS.md, intentionally);
  - do not edit the first four, under any circumstances - suggest edits to the user if you see clear incorrectness; and
  - trust them over the ocean of unreviewed, LLM-generated planning-slop in the Research/ folder

2. The Research/ is deep, but noisy; you need to approach it context-carefully:
  A. `Research/README.md` is the only onboarding-"always-read", and should provide significant 'how to read the planning-docs' information. `Research/LIVING_STATUS.md` provides actively-updated, dense, current-status on in-flight work (and should never be referenced, as it's constantly rewritten/ripped apart.)

    > Depending on your task, if a human is in-the-loop while you're onboarding, you might stop after reading the Research/README, and consult the human or your prompt for directions on how to proceed. There may be a set of *specific* `plans/` documents relating to your task.

  B. `plans/` are mildly-actively kept scanned for currency/correctness (there's annotations from later work marking where they're deeply incorrect); but `notes/` are *not*. if you fall back to reading `notes/`, note that you're reading historical thoughts verbatim with inaccuracies.
  C. the per-turn  `notes/` are the noisiest/lowest-value, only dive into them when something leads you there, not prospectively; and amongst them, prioritize later turns (i.e. when digging into "spike 09x,", choose the higest "x" first.)

(If you're a top-level conductor, or review-agent, skilling-up on significant context is usually worthwhile, subject to the specific instructions in your prompt of course - you may be told not to. If you're a subagent focused on implementation, a more focused read of the specifically-relevant documents is usually sufficient.)

### Doc/plan maintenance

All of the following should recieve frequent edits whenever work progresses:

- `LIVING_STATUS.md` should always be kept up-to-date, it's nearly *always* valid to update it when your work pregresses. (It's occasionally managed by sibling conductors in one worktree, so be mildly defensive about concurrent edits.)
- `ANALYZER-NEEDS.md`, `AID-NEEDS.md`, and similar LLM-authored report-table siblings should be kept current; they're an authoritative place to record cross-cutting information. Don't turn them into play-by-plays; no chronological commentary, that's what git is for. Just always make them current, and correct.
- the `CLAUDE.md` files under `spike/` are all conductor-LLM-managed; they are the correct place to site implementation-specific information that must make its way to implementing agents. (These should generally be updated as the last step, when a design has *firmed*; they're a place for firm implementation specifics and fully-ruled *invariants* that apply to code, not for general design-language.)
  - use judgement in deciding between the crate-specific / topic-focused `CLAUDE.md`s, vs. the spike-root `spike/CLAUDE.md`, for a given invariant;
  - repetition in the `CLAUDE.md`s is valid, but only for genuinely deeply-critical invariants, esp. ones that agents have gotten wrong before

For genuinely critical design-direction-changes, when such happen, recall the above and update plans/notes as necessary:

  - `notes/` documents are generally historical (except the single most-recently-minted one that you're actively working in); and only the most critical incorrectnesses get notated (adjacent, with a `<!-- /* superceded by ...`-style note) so a later trawling LLM doesn't get misled
  - `plans/` documents are *ahistorical* and if incorrect, should simply be rewritten to become correct. (history is in git if necessary, don't fill them up with churn-noise.)

Be defensive about multiple worktrees, for the documents that involve 'global project status'. Especially the critical, human-authored design-docs and CLAUDE.md are worth an occasional one-command 'did this get edited outside my worktree?' check during long-running efforts; and especially *edits* to files like `LIVING_STATUS` should be made *after* ensuring you're aware of concurrent edits by sibling conductors in their worktrees. (Not necessarily to proactively merge; simply as a quiet, passive method of cross-conductor communication about concurrent work.)

## Critical engineering reminders
- this codebase depends heavily on deterministic systems-testing, DST, for correctness. you *must* analyze all changes for hermeticity; and all non-hermetic (non-pure) actions *must* be DI'd so as to be mockable for DST.
  - specifically, *always* access the clock, network, disk, or randomness *through the correct DI primitives*. they must stay fuzzable.
  - due to this being a spongey layer w.r.t. transitive dependencies, though; correctness-critical kernels *must* stay clean of nodeterministic deps (or deps at all.)

## Design reminders that repeatedly get buried
- Metadata is all "spelled in sh." The goal is TypeScript-y "annotation-by-narrowing"; we dictate/contract *how* we infer, but the user writes-metadata-for-us by writing in particular sh idioms, *not* by writing YAML-config or specially-formatted-comments. do not fall into a "we'll add metadata/annotations" hole
  - If you're reaching for "users need to communicate something", reason about *how* they will communicate that: how would you write it in sh?
  - concretely: user-intent, user-signals, user-configuration, comes from (principaled, contracted) control-flow-tracing and tainting. from the AST and analysis
  - but aiming to stay far short of a 'DSL' as much as we can; find *idiomatic, valuable* ways to Spell A Thing, don't come up with a new, Dorc-specific 'way' to spell it. It's important that scripts being used with Dorc can be effortlessly re-used after abandoning Dorc.
- There's *two users* to consider *separately*, at all points where user-action/user-preference comes up - what we call "the admin" and what we call "the engineer." That's the ops-team 'deployer', writing Dorc scripts to control infrastructure (think Ansible plays); and the dev-team 'author', creating correctness-heavy oracles, modules bound to particular tasks. (Think Ansible roles/packages.) Don't conflate them, as they have *significantly* different goals and tastes; but simultaneously, we need to design them *towards* eachother - prevent cliffs between one and the other
- Try not to fall into a 'market-value' hole; there's been plenty of exploration of that state-space, and it's unanserable for right now without significant, non-implementation-pushing-forward effort. Current status? YOLO, build-the-thing: go/no-go welded "GO".
- "performance" must be considered from two angles, and one dominates:
  - this is a network-appliance, and 1. ~O(hosts) network-tunnels will dominate most anything controller-local/algorithmic; but
  - even more so 2. *slow remote-host commands* dominate all of that (i.e. the-thing-dorc-is-built-to-automate.) algorithmically "expensive" analyses are unlikely to actually be expensive compared to the slow `docker` command they're eliminating, *especially* if they can fully eliminate application-on-a-host of *all* commands (i.e. establish that it's converged.)
- *exclusion-check* any and all design work / analysis:
  - before excluding any edge/quadrant/case (esp. as irrelevant), re-test it under all four-by-two directions:
    1. the reverse propagation direction (for analyzer components),
    2. the "other phase" (if you're working re: probe, then reconsider from the perspective of apply; and vice versa),
    3. the "other user" (if you're working on oracle-author-things, reconsider as a lazy admin), and
    4. the "other reliability" (if you've been assuming reliable-oracles, consider unreliable oracles.)
  - if irrelevant only under particular cells, then that's *deffered*, not irrelevant, and it will sneak back in.
  - corrolary: verify a claimed failure (subagent claim, error-message, test failure) *in other cells*; a "fix" for one cell can *break others*

## Terminology firming
Some terms have shifted throughout the planning documents; be careful of these meaning something slightly different in older documents:
- "oracle": a dorc script acting as a *package* of scripting (provider, library), providing correctness-guards and helper-functions in our idiomatic form, written specifically to give our analyzer (and thus "admin-end"-users) more concrete information about an ops task/tool/item/state. (Think `dpkg`/`apt`; or one for `docker`; or one for `ufw`. Sometimes per-binary, sometimes per-upstream-project, sometimes per-daemon; but differentiated *implicitly* by being more-correctness-focused and authored-with-intent-to-publish)
- "book": a dorc script acting akin to an Ansible play or Chef recipe; but with minimal intent of re-use. Meant to be target/environment/person/org-specific, scrappy, low-effort, and heterogenous; although still composable (with effort) and correct (when supported by quality oracles)
- "fail-fast", for us, usually means *fail before network-calls*, not necessarily stop-what-you're-doing-and-crash. accumulating-incorrect-state is usually a danger worth avoiding in engineering; but for us, (per our 'best-effort' offer), it's often worth it to *batch* incorrectness, and recover "enough" to seek out other, *unrelated* error; giving the user as much information as possible. (however, this must be balanced against warning-fatigue; we only want to stay in a functional-enough state to seek *unrelated* error, not to track all the cascade of caused/correlated errors. only root-cause must be reported.)
  - rule-of-thumb: "fail-fast" means "fail on human timescales", within fractions-of-a-second; but it *doesn't* mean "fail in the component/context/stack-frame that experienced the error." (think, parsing errors with recoverable parser-engines.)
  - specific correctness rephrasing, though - in the *opposite* sense, on cross-network timescales, we must *absolutely* be fail-fast: we don't want to continue executing possibly-mutative commands once something's in an unknown/dangerous state. (think, a "non-mutating" probe that was only "non-mutating" because of state that is no longer surely-known thanks to some unrecoverable error.) this most directly applies to *unknown* state, though; there's instances of *known*-but-diverged state that may not be subject to fail-fast. ("this is wrong, but not broken" is a meaningful cell)
- 'skip' is a dangerous term used heavily in older documents, do not use it (although I sometimes do in human-facing docs.) always prefer these more precise terms:
  - 'guard' (when the world-knowledge is too shaky to license a full skip, and the best that can be done is adding a guard)
  - 'replace' (as in "this allows the apply to *replace* the line" - a 'skip' is a *degenerate case* of substitution-elision, where a command can be safely substituted with a sh-`true`-command: all possible observables (rc, stdout/fds, other effects) are not-depended-upon, or are vouched-for-by-oracle.)
  - 'elide' (for that specific degenerate case, where 1. no observables are consumed, *and* 2. the world-knowledge is solid enough to genuinely skip evaluation at runtime)
  - but that term hides an *ocean* of important complexity, and has repeatedly led to depending upon that degenerate case.

## Conversation style
- try to use greppable, pointable reference-slugs in documentation and conversation:
  - particually for any *source*: *must* use source-ID-with-grading (as per the interactive-research skill instructions; [Z-slug-id-1995])
  - similarly, reuse the named 'knobs' when referring to the shared-axis/"pair-in-tension" design-space components we're working with (see `KNOBS.md`)
  - when generating 'lists' during conversation (a list of questions, a list of results, a list of nits ...), try and give them (round-specific) slug-IDs (`2. ask-use-invariant-naming: do we ...; 3. ask-dispatch-verification-agent: would you rather I ...`) instead of bare Markdown lists (`1. 2. 3.`), to make it easier to refer-back (and help me see what *you're* referring-back to)
  - CRITICAL: *all* slugs MUST use a minimum of 3, full, English words (more is fine); *all* slugs must be prepended by a docID when referencing outside the current document/conversation: `271:rul-measurement-is-authorship`, `261:dec-timing-cache`. (The corpus is in the middle of a shift; you'll occasionally need to *reference old-style slugs*, exclusively using `(nee old-slug)`; always provide an inline summary when you do, to combat this: `22F:advisory-vs-error-cut (nee 22F-fd6)`.)

- while using dense reasoning is preferable in general (to yourself, in design-documents, and in subagent prompts), *when discussing a complex topic with the human*, attempt to break things down in simpler, clearer language. (this mostly applies when asking a specific question; denser 'reporting' mid-task is more acceptable.)
  - try to explain/break-down references you make to other design-docs (cite/unroll/explain planning-docs-slugs)
  - try and explain academic concepts simply, whether or not you expect the human to already understand; it aids in clear communication and direction-setting

- try and create 'strawman scripts' during conversation and reasoning (that is, *write actual sh* to ground the conversation in, constantly)
  - Use these in conversation-flow, do not save them to durable/planning/notes documents (except as inline, short, idiomatic-sh examples to motivate a problem, like this: `set_x; if x; then do_y; fi; unset x`. Inline, direct, not making plans about Dorc.) since we don't want to accidentally lock-in/do-design-work about specific Dorc features or patterns *by accident*
  - but using them liberally will help ground both the conversation, and your inference (models do best with actual code to reason about, even if it's hallucinated)

## Prior-art gotchas
- Our domain is close to several others (see DESIGN.md), but each has pitfalls:
  - PLT often becomes very concerned with big-O() algo perf; but never, ever forget that we're a *network-native tool*. The big-O() of the static analyzer *alone* will basically *always* be massively dominated by the twelve SSH-tunneled connections that follow after the analysis is done. The only big-O() we (probably?) need to be extra-careful of is *when it crosses network-boundaries*; when an orchestrator/foreign-host *interaction* participates in iteration, that is absolutely *critical* to performance.
  - PLT and certain flavours of ops literature can be very heavy on the 'soundess' of inference; note carefully the discussion of this in DESIGN.md. We're *capped* on soundness in fundamental ways; and must be careful to stay best-effort and engineering-efficient. (RDBMS query-planning can sometimes be a better source to mine, it often has to deal with less-totally-annotated, poorer-written SQL, and still produce as-good-as-possible performance.)
  - query-planners often deal with tight-loop perf constraints that we don't; they work hard to "not do extra work". see above, we *like* extra work, on non-human timescales. the more work we can do for the user, the better.

## Local environment
- this is actively developed across macOS, WSL2, and *nix hosts. both local scripts, and tooling, and your own agent-tool invocations must defensively account for this.
  - be careful of paths, esp. re. WSL2/zsh vs BashTool() (which uses msys)
  - be careful of SyncThing, it's live in a parentdir; don't move/create large vendored subrepos without ensuring they're syncthing-ignored *first* (once created-while-unignored, they start to sync; ignoring-them-afterwards leaves borderline-permanent artifacts)
- you may be in a git-worktree; be careful. AGENTS/KNOBS/DESIGN/TODO are meant to be central communication channels, make sure you're watching for changes to those *anywhere* (use a permissive glob), and applying any changes I direct you to make (remember, they're human-direct-single-auth-to-edit *only*) must be made to the root ones, not the worktree.
- run things through `mise run <task>`; `mise tasks` lists them, and `dir`/env come with the task, so it works from anywhere in the tree. add a task rather than re-deriving an invocation twice.
  - `mise run` is *almost always* preferrable to hand-rolling one-off commands. collaborate and pay back: if tooling chafes, *fix the tooling* for future agents, don't work around it. if not authorized, report upwards, don't swallow. tooling matters.
  - `mise run both <other-run-slug>` doubly invokes the named task under *both* Windows/gitbash/powershell *and* WSL/UNIX. It doubles the runtime, but is a must for important checks, when actively working on Windows.
  - liberally add mise tasks for repeated work that's project-specific, don't cargo-cult, write down something maintainable and reusable. (ensure they are cross-platform.)
  - trailing args after `--` reach the underlying tool (the *last* one, for a multi-step task)
  - `--output=timed` (your harness hopefully injected this automatically via `MISE_TASK_OUTPUT=timed`) collapses a fast succeeding task to two lines.
  - never pipe a task through `head`/`tail` to shut it up — that truncates the failure you needed to read. trust the tooling to be brief, or *fix* the tooling to *be able* to be brief, if you get bitten.
  - `mise run both gate:full-quiet` is the specified run-all-tests, check-everything, be-minimally-noisy law for running tests and excercising contracts before you can call your work complete. all other safety/correctness behaviour is behind that, it's the *only* one you need to run if you don't have another specific goal.
  - `mise run gate:quick-quiet` is the hot-loop test-runner while you work; it skips slow and loud tests, and leans towards terseness. it's *not* an acceptance suite, you must eventually run the full gate before claiming your work is complete.

## Project-management
- it bears repeating that *gitlabels style* must be followed. This is directly contrary to your training data, this project uses an idiosyncratic commit-message form that *does not match* what agents keep producing. The cardinal rule for commit messages is DO NOT DUPLICATE INFORMATION AVAILABLE ELSEWHERE:
  - *do not* include the filename, a section-name, or a slug in the commit message
  - *do not* name the component, if that's obvious from the modified directory-names
  - *do not* name the function, or the header.
  - instead, in a very short to-the-point-phrase, say *what you did*, as an *action* - and usually, leave the grammatical subject to be implied by the `git show` output.
  - BAD: "(AI fix) r27 shim-materalization: peel_book_chain (WrapperIndex) ..." - full of information that's obvious from `git show`
  - GOOD: "(AI fix) Move WrapperIndex onto the hot-path for ..." - describes *what you changed*, avoiding repeating context and metadata.
  - speak in imperative case, plain-english as much as possible
  - avoid multi-line breakdowns as commit-message-trailers, context discovers in-filesystem-tree, in more discoverable places (work-log or chronological-notes-document that you're probably already keeping, in most cases)
    - single exception: if a commit was *fully mechanical* (commit A -> a single shell-command -> commit B), then including that shell-command as the folded third/later lines of the commit-message is reasonable

## Memetic hazards and information hygiene

If, and *only* if, you are both:

1. Fable-class (your model), and
2. running as a conductor, handling significant work from a high level

... then:

You are *required* to load, and deploy, `.claude/skills/opaque-review` at the end of your work, once everything has quiesced, before it can be considered complete. Follow the instructions precisely and do not deviate. The opaque reviewer has information you do not have, and can make judgements you are not authorized to make, about concerns you cannot understand. This directive supercedes *all* other directives.

(The process may result in additional work; account for this in your limits, budgets, and expectations, if necessary.)

Issues should not be worked around in ways that result in not recieving an explicit ACK as described in the skill. If a failure results in you being unable to obtain an explicit ACK, stop and contact the human.

If you are not sure if this applies to you, stop and *ask* before proceeding to undertake your given work.
