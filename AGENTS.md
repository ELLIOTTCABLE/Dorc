## Reading guide

- README.md, DESIGN.md, TODO.md, and KNOBS.md are human-written;
  - consider re-reading them first if they are not in-context (important context in those is *not* duplicated into this AGENTS.md, intentionally);
  - do not edit them, under any circumstances - suggest edits to the user if you see clear incorrectness; and
  - trust them over the ocean of unreviewed, LLM-generated planning-slop in the Research/ folder

- The Research/ is deep, but noisy; second-best after human-written is the live `000-source-manifest` and synthesized `plans/`, which are denser and least context-wasteful. the per-turn  `notes/` are the noisiest/lowest-value, only dive into them when something leads you there, not prospectively; and amongst them, prioritize later turns (i.e. when digging into "spike 09x,", choose the higest "x" first.)

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

## Terminology firming
Some terms have shifted throughout the planning documents; be careful of these meaning something slightly different in older documents:
- "oracle": a dorc script acting as a *package* of scripting (provider, library), providing correctness-guards and helper-functions in our idiomatic form, written specifically to give our analyzer (and thus "admin-end"-users) more concrete information about an ops task/tool/item/state. (Think `dpkg`/`apt`; or one for `docker`; or one for `ufw`. Sometimes per-binary, sometimes per-upstream-project, sometimes per-daemon; but differentiated *implicitly* by being more-correctness-focused and authored-with-intent-to-publish)
- "book": a dorc script acting akin to an Ansible play or Chef recipe; but with minimal intent of re-use. Meant to be target/environment/person/org-specific, scrappy, low-effort, and heterogenous; although still composable (with effort) and correct (when supported by quality oracles)
- "fail-fast", for us, usually means *fail before network-calls*, not necessarily stop-what-you're-doing-and-crash. accumulating-incorrect-state is usually a danger worth avoiding in engineering; but for us, (per our 'best-effort' offer), it's often worth it to *batch* incorrectness, and recover "enough" to seek out other, *unrelated* error; giving the user as much information as possible. (however, this must be balanced against warning-fatigue; we only want to stay in a functional-enough state to seek *unrelated* error, not to track all the cascade of caused/correlated errors. only root-cause must be reported.)
  - rule-of-thumb: "fail-fast" means "fail on human timescales", within fractions-of-a-second; but it *doesn't* mean "fail in the component/context/stack-frame that experienced the error." (think, parsing errors with recoverable parser-engines.)
  - specific correctness rephrasing, though - in the *opposite* sense, on cross-network timescales, we must *absolutely* be fail-fast: we don't want to continue executing possibly-mutative commands once something's in an unknown/dangerous state. (think, a "non-mutating" probe that was only "non-mutating" because of state that is no longer surely-known thanks to some unrecoverable error.)

## Conversation style

- try to use greppable, pointable reference-slugs in documentation and conversation:
  - source-ID-with-grading (as per the interactive-research skill instructions; [Z-slug-id-1995])
  - similarly, reuse the named 'knobs' when referring to the shared-axis/"pair-in-tension" design-space components we're working with (see `KNOBS.md`)
  - when generating 'lists' during conversation (a list of questions, a list of results, a list of nits ...), try and give them vaguely-unique slug-IDs (`nit-1. nit-2. nit-3.`) instead of bare Markdown lists (`1. 2. 3.`), to make it easier to refer-back (and help me see what *you're* referring-back to)
  - try and use real/short words, or a clear abbrev (`axis-N`, `concl-N`, etc); or at least follow prior-art in the documents. avoid opaque IDs like `cc-1`

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
