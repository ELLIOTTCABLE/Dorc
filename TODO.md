## Architecture-planning / footgun-uncovering-research-phase

- [.] (PARTIAL) deep security-dive on both the 1. language/analysis and 2. orchestrator architectures,
  - specifically flag 'including seccomp in the core'; LLM claims it's secure-by-design, but I very much do not trust that claim without a specific threat-model and analysis
- [.] dig into control-flow hazards that will easily pass parsing (`set -e`, `trap`.) may be mined for contract-sh-spellings?
- [ ] prior-art on linking oracles' binaries-to-be-invoked to *hash*, spelled-in-sh (`if [ "$(shasum thebin)" = "abcdef..." ]; then ...`)
- [x] (DEFERRED) skim shell-script corpii to validate design
- [ ] came up in R23, but i've seen hints before: difference between two potential things an oracle-author *could* be intending to claim: "I'm converged in live state" (this is in the state I wanted it to be in, modulo unknown/irrelevant-to-me noisy changes; and I wish to elide the mutator) vs. "this is a no-op in live state" (this command is *fully* state-precise as of probing; it will literally do nothing when run)
- [x] 24K lang-fallout: dialect-version-marker *now*, before the stdlib. I think probably how mise does it? or potentially rust? I want something hyper-defensive against cargo-culting, I've seen languages get locked into their ineffective "but but but I said this was TEMPORARY!!!" claims before ...
- [x] 24K lang-fallout: I need to rubber-duck two-formats. I *really*, really, deeply hate EOL-comment-annotations like `shellcheck-ignore` or whatever; that's not a programming language, that's a fucking fungal growth on your code, a nightmare to maintain, a nightmare to beautify. but also, a whole host of agents came back with basically the same advice, and it's fairly convincing - "unforced abandonment of the offramp." idk. torn.
  - considering an absolutely-absurd two-dialects stance (work as comments *or* inline), but that's probably a ton of work for very little benefit, esp. if I don't set the EOL-comments as the default ... whichever would be the default, should probably just be the only one
  - alternatively, can a bare subset of the oracle-language be blessed? *can* you currently write a meaningful oracle with zero type annotations? I'm not even sure anymore
- [.] 24K lang-fallout: not the top item from the adjudicator, but probably *my* biggest jump-scare: one pointed out that everybody-except-typescript went the *comments*-adjacent route for their post-hoc gradually-typed language implementation, and for one good reason: **they often didn't have Microsoft-scale resources**, and writing a good LSP/shellformat/linter are very much Microsoft-scale issues. guess who definitely also doesn't have Microsoft-scale resources ...
  - but I hate EOL comments so unbelievably much that I might actually swallow this risk, idk.
  - arguably it's cheaper for us, largely because the ecosystem there isn't already rich-and-we're-abandoning it. shellcheck is a wonderful project, but it's fighting uphill, and definitely not as foundational/indispensible as other linters; ditto shformat. and I don't think there even *is* an sh LSP ...
- [ ] 24K lang-fallout: `dorc strip` *now*, don't make the promise and then fail to keep it. the offramp needs continuous code-maintenance and attedance-to, not just lip-service, I buy that, okay.
- [ ] dotfiles research: bash/zsh rule the space; I don't really want to swallow parsing-support for bashisms/zshisms. Maybe make it an *explicit split* - *only* support dorc-lang in dorc-lang-POSIX-sh files; *only* try to support bash/zsh in dorc-lang-less runbooks?
  - this narrows kWHICHSH: kWHICHSH, by my thoughts, is primarily a drive twoards *sharing*: besides making parsing simpler and staying broadly compatible no matter what shell somebody uses (offramp-maintaining for *both* zsh and bash users), it's a good-practice-encouraging and helps drive oracle *sharing* with other users. this doesn't really apply for runbooks, though; so I'm tempted to carveout runbooks from kWHICHSH entirely. (repurp-finding16/38.)
- [ ] repurp-finding5/18 resurface privilege; we *really* need to figure out the plan for `sudo`/`sh -c`/`source` flavoured content, because "unsupported" just isn't good enough.

## Spike-3

- [ ] phased CLI (not TUI); actual plan/apply as-driven-by-a-user
- [ ] *just* the streaming-updates part of the TUI;

## Preparation-for-agentic-implementation 🤢 phase

- [ ] how to TDD: how do we *architect* a network-scale appliance? even miniaturely? is there any better option than "I keep a proxmox host up at all times to dynamically Become A Network Of Fake Hosts, then execute tests against that?" I need something *fast* and I just don't know how to structure. (also, sandboxed, for agents.)
- [ ] hunt for *implementation* details that *can* be decided/implemented rather upfront (like TDD, but besides TDD) - data-structures and algorithms that are *guaranteed* to be relevant to the project, and can be *seeded* as initial, black-box modules/code (or picked as upstream libraries.)
- [ ] major refactor of the planning: re-split, first, along *what-needs-to-be-known-per-component* lines, not *what did I want to research* lines (i.e. there's findings relevant to "the analyzer" all over the research-base; they need to be collated and extracted.)
  - I do not trust a single agent on this; I think probably a pass to decide *which* components; then a pass per-component to extratct-and-pointer-back; then finally a pass to break into implementation-phases (spike/soon/need/defer?)
- [ ] agentic meta-pass on "how best to structure this codebase so agents can work on it", some nightmare-slop bullshit.
  - most importantly I'm guessing deterministic triggers per-component, if possible. point agents entering certain dirs/components to authoritative design-documents for that component, so when they 'cross boundaries' to chase a fix/feature, they hit Correct Context to make it intelligently?

## Small implementation details

- [ ] probe-bodies *also* need to be proved non-mutable w/ effect analysis

*(Machine-dropped items live in `TODO-ADDTL.md`, which is agent-maintained and unstable; this file stays human-voice only.)*
