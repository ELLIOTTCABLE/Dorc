# Ansible alternatives with UNIX philosophy

_Claude conversation · created 2026-05-27T16:44:04.179932Z · updated 2026-05-29T01:34:56.204672Z_
_Clean transcript: final human/assistant prose only; thinking + tool-call/search blocks stripped (see ansible-conversation.json for the complete log)._


---

## HUMAN

Are there any at-least-*slightly*-mature alternatives to Ansible that are a little less “idiosyncratic YAML” and a little more “classic UNIX do-one-thing-well”, more direct shell-script, focused down sharply onto just adding host-reconciliation/idempotency/status-checking-before-mutating behaviour in a *general* way that doesn’t try to wholesale supplant existing tools as Ansible does?

(Ideally still push, not pull-like-Puppet; but I’m not married to that.)

Find also discussions on this topic and general opinions, as well as surfacing obscure projects if no mature ones exist. I’m considering rolling my own and I want a state-of-the-industry overview if there’s no existing option that will cause me to stop-wanting-to-NIH.

---

## ASSISTANT



Found the lineup you're looking for. There's a small, identifiable subculture of "anti-Ansible minimalist" tools, and one of them (`cdist`) is essentially a direct articulation of your spec.

## Closest fits to your brief

**`cdist`** — SURE this is the closest match. POSIX shell is the configuration language; "types" are folders of shell scripts with an `explorer`/`gencode-remote`/`gencode-local` convention that gives you status-check-then-mutate idempotency without imposing a DSL. Push over SSH, target dependencies are literally just `sh` and `sshd`. Maintained since 2010 at ETH Zurich by Nico Schottelius — mature, slow-moving, production-used in Swiss hosting (`ungleich.ch`) and academia. Repo currently lives at `code.ungleich.ch/ungleich-public/cdist`. Last stable was `6.9.8` in 2021 with a `7.0.0` line in docs; SUSPECT the cadence is "done, not abandoned" rather than "dead" but worth verifying activity before committing.

**`pyinfra`** — SURE this is the most actively maintained option (~4.4k stars, multiple releases per year, recent 2026 activity). Operations are Python that compile down to shell commands; the model is explicitly "execute remote script, parse output, execute another shell command, like a human would." Two-stage with `--dry`. SUSPECT this is your pragmatic default if you want something with an actual ecosystem. The Python-as-config will rankle slightly given your OCaml/TS taste, but it's much closer to "code" than YAML and the abstraction is thin and inspectable.

**`rset`** (Eric Radman) + companion utilities `rinstall(1)` and `rsub(1)` — SURE this is the most ideologically aligned with what you wrote. Sister projects `drist` (Solène Rapenne, `tar | ssh` based) and `judo` (Kamil Cholewiński, scp-based) form a small "remote-execution model" cluster. Radman explicitly framed the design in a 2021 NYCBUG talk as "ship files associated with a host, run scripts on the remote, mix scripting languages freely, no fact-collection stage" — see `eradman.com/talks/minimalist_scripted_configuration/` and the umbrella site `scriptedconfiguration.org`. GUESS user count is in the dozens-to-low-hundreds based on signal volume, but the design is the most "do-one-thing-well" of anything in this space — `rinstall` is a single utility that installs a file and exits non-zero if nothing changed, which is *exactly* the idempotency primitive you'd compose by hand.

## Second tier / worth knowing exists

- **`mgmt`** (James Shubin / purpleidea) — different beast entirely. Event-driven reactive engine with its own `mcl` language. ~3.8k stars, decade-old, still pre-1.0. WONDER if it ever crosses the chasm. Not push-shell; mentioned only because it dominates "next-gen config mgmt" discussion threads.
- **`sup`** (Pressly) — "make for a network of servers." Uses a YAML `Supfile` but does *not* add idempotency — it's a remote command runner. SURE this is not what you want as-described, but worth knowing about.
- **`bashible`**, **`marionette`** (skx), **`deployr`** (skx), **`posixcube`** — solo-maintainer experiments in this space. GUESS none of them are production-mature; they appear in HN comment threads as "I built this when I got tired of Ansible." Useful as design references, not as dependencies.
- **`itamae`** — Ruby-based Chef-lite. Push, idempotent. Reasonably mature in Japanese ops circles. GUESS not what you want given the Ruby surface area and Chef-style resource DSL, but it's the most "shell-flavoured-but-typed" option if you can stomach Ruby.

## Sentiment / discussion landscape

The recurring HN/Lobsters thesis is something like: "Idempotency and reusability are what I like about Ansible. YAML is a dependency I could do without. Maybe what we need is a set of CLI commands that mirror Ansible modules in performing idempotent operations through SSH and APIs." That comment from the 2020 "alternatives that avoid the whole YAML mess" thread is followed by exactly one recommendation: `cdist`. The 2022 "Why are there no decent Go or Rust alternatives to Ansible?" thread surfaces `mgmt`, `marionette`, `deployr`, and an Ansible developer pointing out that Ansible's slowness for localhost ops is real and that modules can be written in any scripting language, not just Python.

The Lobsters "why you should try pyinfra" thread has a comment that captures your instinct precisely: "there would need to be a compelling reason to not just use idempotent shell scripts. I've felt the same way about Ansible." And one specifically arguing pyinfra should let you write `*.sh` files directly so that "If pyinfra might not exist anymore some day, the user could still execute the shell scripts manually." — this matches your preference for tools that compose with the existing UNIX environment rather than replacing it.

## On rolling your own

SUSPECT the "minimum viable framework" is smaller than you'd guess and `cdist`/`rset` have already each derived it independently. The shared primitives are roughly:

1. A way to dispatch a per-host bundle (files + scripts) via `ssh + tar` or `rsync`.
2. A status-check-then-mutate utility (`rinstall(1)` is the canonical reference implementation — ~150 lines of C).
3. A naming/routing convention so the same scripts can target different host classes.
4. Re-use the SSH `ControlMaster` so per-host turnaround is sub-second.

GUESS your homelab + Ubiquiti + macOS-dev-workstation context is small enough (probably <20 hosts) that `rset` plus its `rinstall`/`rsub` utilities would feel native to your stack and need ~zero customization. WONDER if reading the `rset` source for an afternoon would either (a) satisfy the NIH itch by being so small that you can mentally fork it whenever you need, or (b) trigger you to write a typed-OCaml version in a weekend, which would itself be a perfectly reasonable outcome and probably more enjoyable than adopting `cdist`.

My honest recommendation order for your context: **try `rset` first** (smallest, most UNIX-philosophy, lowest commitment), **fall back to `cdist`** if you want a real type library you didn't write, **fall back to `pyinfra`** if you want a community and a real ecosystem and can swallow Python-as-config.

---

## HUMAN

Are there any at-least-*slightly*-mature alternatives to Ansible that are a little less “idiosyncratic YAML” and a little more “classic UNIX do-one-thing-well”, more direct shell-script, focused down sharply onto just adding host-reconciliation/idempotency/status-checking-before-mutating behaviour in a *general* way that doesn’t try to wholesale supplant existing tools as Ansible does?

(Ideally still push, not pull-like-Puppet; but I’m not married to that.)

Find also discussions on this topic and general opinions, as well as surfacing obscure projects if no mature ones exist. I’m considering rolling my own and I want a state-of-the-industry overview if there’s no existing option that will cause me to stop-wanting-to-NIH.

Generally, I’m not hard-against some of Ansible’s design-choices — I think, what *I’d* build, is probably a new strict-superset-of-POSIX-sh that explicitly encourages (from a language-design standpoint) the correlation of each command/line as a thruple of (standard mutating command, check-command, undo command.) I’m lazy enough that I’d probably even attempt it at first as a transpiler-to-Ansible or similar, because I do think that a lot of the “negotiate host, manage connections and inventory, sort tasks by tags and figure out what tasks we care about right now … so on and so forth” are probably hard problems that I just care less about. The real issues to me are 1. verbosity, 2. dependency expression and resolution (both at the individual-mutation level / if-then-else kinda thing, *and* at the larger system-state/role/machine level); and 3. copy-pasteability (it shouldn’t be an hour’s work to ‘write’ an Ansible version of “install this obscure game-server that nobody’s written automation for”, it should be “copy-paste the upstream docs that know nothing about automation, then gradually enhance with idiomatic idempotency, safety-checks, dependency-information, and the like *as it becomes necessary*.)

I firmly believe one should be able to gain all the strict-end benefits of Ansible-et-al in a way that doesn’t preclude ‘found new obscure app i want to install, run the upstream instructions, and it’s installed in my infra’ being a five minute process instead of a five hour one.

(One last aside that’s completely unrelated, but in investigating these tools … I hate that Ansible buffers output lol. I understand why, but I’d swallow all the downsides I can see in a heartbeat to just *see what’s going on in realtime* when an ssh’d-remote-command is dumping a bunch of nightmare-state output as if actively proceeds to screw up … instead of seeing a frozen empty Ansible buffer while it waits for command-exit, followed by a red error.)

---

## ASSISTANT



Short answer: there are three categories worth knowing, none of which fully satisfy your criteria, and the conceptual primitive you described (`check`/`mutate`/`undo` thruple) has been built — just not in a form that survived.

## Mature-ish, push-based, shell-forward (use today)

**`cdist`** — SUSPECT this is the closest existing match to your stated criteria. Push over SSH, zero remote deps beyond `/bin/sh`, configuration is literally POSIX shell scripts organized into types (analogous to Ansible roles) and explorers (small scripts that glean information about the target system before manifests run). Initial manifests are `/bin/sh -e` files calling type-objects like `__package apache2 --state absent`. SURE this matches your "general, doesn't supplant existing tools" instinct because each type *is* shell, written by you, not a module DSL. Caveats: last tagged release was Aug 2021, repo is still touched but pace is glacial; the directory-layout convention is mildly idiosyncratic; the `type` abstraction is its own thing to learn.

**`pyinfra`** — SURE this is the most actively maintained option in the space, and it directly addresses your output-buffering complaint: it returns realtime stdin/stdout/stderr output with `-vvv` and was explicitly designed for instant debugging with no abstraction layer. Python instead of YAML, ~10x Ansible speed, idempotent operations with dry-run, escape hatch is `server.shell(...)`. SUSPECT the trade-off versus your "don't supplant existing tools" criterion: pyinfra *does* ship an Ansible-style module library (apt, files, systemd, etc.) — but unlike Ansible, dropping to raw shell is a first-class one-liner rather than a "you're holding it wrong" anti-pattern. One pyinfra user on HN summarized it as "I still feel like I'm dropping down to just hitting `server.shell` a bunch, but at least as a programmer first, it feels more comfortable vs the mountains of yaml" — which I GUESS matches the experience you'd have.

**`mitamae`** — SUSPECT relevant if you want single-binary deploy: Chef-style Ruby DSL compiled into mruby, distribute one binary, no Ruby on target. Itamae's `itamae ssh` is omitted by design in mitamae because it's slow; instead you transfer the binary plus recipes and execute over ssh. Idempotent by default. Ruby DSL probably violates your "less idiosyncratic" criterion.

**`Propellor`** — Haskell, Joey Hess's project. WONDER if your functional background makes this more interesting than it sounds — Hess's pitch is that "Since a propellor configuration is a haskell program, it gets compiled and most configuration errors are caught at compile time. Finding ways to use haskell's type system to eliminate entire classes of misconfigurations is what I'm really excited about with propellor going forward". Push or pull. Still actively maintained on Hackage.

## Prior art for what you described building

This is the important section.

**`Babushka`** (Ruby, archived 2019) — the design *is* the thruple you described. A "dep" is exactly `met?` + `meet`:

```ruby
dep "precompile assets" do
  met?  { File.directory? "public/assets" }
  meet  { `rake assets:precompile` }
end
```

Plus a `requires [...]` dependency graph between deps. SURE this is the canonical "check + mutate as a single primitive" design in the configuration-management space — see the original slide deck for confirmation that the concept was literally `dep "name" do; met? { return true or false, no side effects }; meet { do the actual work here }; end`, with a `requires [...]` block for dependency resolution. The author archived it, noting "requiring good old babushka for serious infrastructure work today could be considered a kind of warning sign" — but that's about ecosystem rot, not the design. The design holds up.

**`Doit`** (Perl, Slaven Rezic) — SUSPECT this is the closest published implementation to the "transpile to Ansible / reuse the inventory + connection plumbing" path you mused about. "A failing command throws an exception and thus stops the script… Before executing a command a check is done whether the command has to run at all. This makes it possible to write 'converging' scripts: a first run would do the changes, subsequent runs would not do at all. Command execution is logged". Components are Perl modules and "are also synced to remote systems for subsequent connections". Niche but the semantics map to your description almost exactly.

You won't find your `undo` primitive in mature OSS — that's the genuinely novel bit. SUSPECT the reason: properly reversible state transitions on real Unix hosts are very hard (deleting a package isn't the inverse of installing it, since you've also pulled in deps, accumulated cache, etc.), so the field gave up on rollback and reaches for snapshots/declarative-rebuilds (Nix, immutable images) instead.

## Smaller / less-mature options if you want to see the design space

- **`drist`** — rsync + shell scripts over SSH, that's the entire tool. "Unlike Ansible it uses simple shell scripts and rsync". Too minimal probably — no built-in idempotency, you write it inline in the script.
- **`pinspot.sh`** — single shell script, "avoid dependencies like ruby, python, etc; avoid bloated DSL or XML configuration files; avoid opening ports for master/agent communication; only SSH; only shell scripts; push-based configuration". Author admits "mainly for my private needs".
- **`bashible`** — Bash DSL that gives you `@` task / `-` step structure with `when` guards. Last meaningful activity ~2020. Worth reading the README for design ideas, not for production use.
- **`sup`** (pressly) — Make-for-SSH; no idempotency layer, just orchestrates command execution. Doesn't fit your need but worth knowing exists.
- **`Patchwork`** (joshyujump/Fabric-based) — "a mid-level library of Unix system administration primitives… Primary API calls strive to be idempotent: they may be called multiple times in a row without unwanted changes piling up… Patchwork is just an API: it has no concept of 'recipes', 'manifests', 'classes', 'roles' or other high level organizational units. This is left up to the user or wrapping libraries" — this is the *most direct* hit on your "general, doesn't supplant" criterion, but it's basically a stalled Fabric extension.

## Community sentiment, briefly

GUESS this matches your prior: the dominant HN/Lobsters take is that Ansible is the least-bad mature option, the alternatives have all-or-nothing failure modes, and the people who escape go to one of (1) pyinfra, (2) NixOS, (3) plain shell scripts plus discipline. Multiple HN threads ask exactly your question (e.g. "Are there any other Ansible alternatives that avoid the whole YAML mess?") and the recurring suggestion is "Maybe what we need is a set of CLI commands that mirror Ansible modules in performing idempotent operations through SSH and APIs" — which is itself an admission that nobody's built it.

## On rolling your own

WONDER whether your transpile-to-Ansible strategy is the right shape. The thing Ansible is genuinely good at (inventory, connection multiplexing, host facts, the bulk of "boring plumbing") is largely also what pyinfra is good at, and pyinfra exposes a Python API ("Pyinfra can be used both as a command-line tool for quick deployments and as a library within Python applications") — so SUSPECT a more pragmatic NIH path is "write your strict-superset-of-POSIX-sh as a frontend that emits pyinfra operations", since pyinfra already gives you real-time output, dry-run, idempotency primitives, and ~mature inventory handling. You'd inherit the bits you said you don't care about, and only have to design the language and the (check/mutate/undo) decomposition.

The copy-paste-from-upstream-docs problem you described is, GUESS, actually a *separate* hard problem from the idempotency-DSL one — it's mostly about a permissive parser that can accept literal `apt install foo && systemctl enable foo` and progressively annotate it with `# check: dpkg -s foo | grep -q '^Status: install ok installed'` style sidecar metadata. I'm not aware of any published tool that does this, which WONDER whether it's the most novel and worth-publishing piece of your design.

---

## HUMAN

Let's say I'm going to roll my own; look into these for me:

1. first off, I swear there was a third 'fundamental component' to an Ansible task besides check&execute; but, in general, research Ansible fundamentals. What can I 'steal', for my strict-sh-superset language, besides simply check/idempotence?
2. dig the same out of similar tools — look for prior art at a fundamental design level, i.e. idempotence, that is relevant to the task/state/mutation specification (moreso than host/transport)
3. finally, given the above description (and my old notes on this in Notion - [https://www.notion.so/troe/Simpler-ansible-sh-w-check-diff-rollback-mode-annotations-18df99b3151d80fbba90fdf61a566f70](https://www.notion.so/troe/Simpler-ansible-sh-w-check-diff-rollback-mode-annotations-18df99b3151d80fbba90fdf61a566f70?source=copy_link)), dig deeper into the tools I could steal as the *underlying engine*. You mentioned a Python thing; I have to admit I'm not sold by Python as a core engine, but ehhh whatever works I suppose ... evaluate by maturity/stability, yes, but also ecosystem (if I transpile to ansible, then I "get" a ton of ansible modules for free, for instance.)

We're in a high-level design-phase; stay at a high level, don't get into the weeds with specific implementations or anything. I'm checking for prior art and considering design-constraints; more than anything else, I want to catch any major design components that may affect the core design as early as possible.

As far as I can tell, assuming I'm de-prioritizing worrying about inventory-management … the primary stuff that matters is idempotence and version-management?

---

## ASSISTANT

I'll factor in your Notion notes if I can reach them, then dig into the fundamentals. Let me try the Notion link first.

Got your notes — that changes the framing substantially. Key things I'm now anchoring on: the TypeScript-for-shell gradual-enhancement vision, the `cmd.check`/`cmd.undo` contract-binary model, the *transactional rollback of a command group* requirement, the check-frontloading/parallelization performance insight, and today's addition about coupling task-spec versions to underlying-software versions (which is what you mean by "version-management"). Let me verify the Ansible/CFEngine/Chef fundamentals before I answer rather than asserting from memory.

Read your Notion notes. They sharpen this a lot — especially the contract-binary model, the *transactional rollback of a command group* (not just per-command undo), the check-frontloading/parallelization perf thesis, and today's note coupling task-spec versions to software versions. I'll answer your three, then push back on your closing question.

## 1. The "third component," and what to steal from Ansible

The literal answer: the third execution capability besides execute and check is **diff mode**. Ansible's own docs are explicit that there are exactly two *validation* modes layered on execute — check mode (runs without making changes; modules report what they would have made) and diff mode (before-and-after comparisons) — and they're *independent opt-in capabilities*, not the same thing. Check answers "would this change?" (a dirty-bit); diff answers "what, specifically, would change?" (content delta). You already have check+diff+rollback in your page title, so you independently rederived this.

Design consequence worth deciding now: `cmd.check` and `cmd.diff` are potentially *two* contract verbs, but a diff subsumes a check (the bit is `diff != ∅`). SUSPECT making diff primary and deriving check from it halves your contract surface — but it raises the authoring bar for tool providers, so it's a real tradeoff, not a free win.

But the thing you're *actually* half-remembering as "fundamental" is more likely the **status signal and its consumer, handlers/`notify`** — and this is the higher-value steal. Ansible fundamentally operates on task statuses: ok, changed, failed, or skipped; these determine whether further tasks execute and whether handlers are notified due to changes. That's *change-driven reactivity*: "reload nginx **only if** the config actually changed." It's a separate pillar from check/execute/diff because it's about *propagating the change-signal between commands*. If "did this line actually change something" isn't a first-class value other lines can subscribe to, you can't express the single most common real-world idempotence pattern (mutate config → reload iff changed) without redundant re-checking. This is easy to under-design and expensive to retrofit, because every contract verb's return type depends on it.

Stealable fundamentals, ranked by relevance to *your* tool:

- **`changed_when` / `failed_when`** — THE most relevant. This is already your annotation model, in YAML: for a raw command Ansible can't reason about, you annotate "treat as changed when stdout matches X / failed when rc≠0 and stderr≁Y." Your `cmd.check` is the generalization — a *reusable per-tool* predicate instead of a *per-invocation* one. Study where this YAML model breaks down; that's your design boundary.
- **`creates` / `removes`** — the cheap idempotence guard (skip if sentinel file exists). Proof that a large fraction of idempotence needs nothing more than a file-existence check. Give yourself this escape valve before reaching for full `.check`.
- **graceful degradation as contract** — modules that don't support check mode "report nothing and do nothing". Absence-of-check is *legal and surfaced*, not an error. That stance is exactly your TypeScript-gradual model; steal the principle directly.
- **`register`** — capture a line's output as a value for later lines. You'll need some minimal data-flow-between-steps.

The caveat to internalize, straight from Ansible's docs, because it threatens your perf thesis: check mode breaks on data dependencies — if task B depends on a file task A creates, A is skipped in check mode and B fails. Dry-run in an imperative pipeline is fundamentally lossy. That directly bites your "frontload all checks, then apply minimal mutation diff" idea (more below).

## 2. Prior art at the task/state/mutation level

Three bodies you didn't cite, in descending order of how much they should affect your core:

**(a) CFEngine / Promise Theory — convergence vs. idempotence.** Absorb this before locking the contract. Burgess's distinction: "Convergence is like fixing the outcome and computing the route, like a GPS finder; congruence is repeating a recipe in a sequence of known steps", and he's emphatic that convergence to desired state is sometimes incorrectly called idempotence. The bite for you: a *convergent* operator reduces distance-to-goal on every application from *any* broken starting state; an *idempotent* one only guarantees re-running from the same state is a no-op. Your `.check` is literally "are we at the fixed point yet?", and your whole value prop — fixing drifted/half-broken hosts — demands convergence, not mere idempotence. He built CFEngine around "fixed point" thinking: every desired state should be an achievable fixed point. Decide whether your contract *demands* convergence (strong, harder for tool authors) or merely idempotence (weak, breaks on partial state). It changes the contract wording.

**(b) Puppet — type/provider split + the ordering fork.** Two steals:
- **Resource-type vs. provider (the RAL).** Puppet separates the declarative interface ("package X installed") from the per-platform implementation (apt/yum/brew). Your `mv.check`/`mv.undo` binaries *are* a provider contract. The mature pattern: a *named capability* that multiple providers satisfy, selected by host facts. This is also exactly where your version-management idea lands (below).
- **The ordering decision, which is your single most important early call.** Puppet is a declarative DAG (state dependencies; engine orders and parallelizes). Ansible is imperative-linear (order = file order). Your "annotate existing shell, copy-paste from StackOverflow" vision *strongly implies linear* — that's the entire appeal. But your *performance* thesis (frontload independent checks across hosts, then apply a minimal mutation diff) *requires* a dependency graph to know which checks are independent. These fight. SUSPECT the resolution is TypeScript-shaped: imperative-linear by default, *statically analyzable where annotated* — but only if you design the annotation syntax so dependencies *can* be declared when present. If you don't reserve syntax for that on day one, you forfeit frontloading forever.

**(c) The (mutate, undo) pair is mature — just outside config-mgmt.** Two battle-tested sources:
- **DB migrations (Flyway/Alembic/Rails up/down)** — your thruple minus check. The hard-won lesson: down-migrations are routinely wrong, untested, or abandoned, because a correct inverse is often harder than the forward op and rarely exercised. Direct warning for `.undo`: coverage will lag mutate coverage badly, and *untested undo is worse than no undo* (false confidence). Your "fail-fast if it can't cleanly roll back" instinct is the correct mitigation — make undo-absence loud and undo-presence trustworthy-or-refused.
- **Snapshot/generation rollback (`dnf history undo`, NixOS generations, ZFS/btrfs `rollback`)** — and here's the load-bearing observation: *the tools that actually deliver reliable rollback do it via snapshots, not composed per-operation inverses.* NixOS rolls back by atomic symlink swap of an immutable generation; you, a ZFS user, already trust `zfs rollback` over any hand-written undo. This argues for a hybrid you should consider now: per-command `.undo` for fast fine-grained reversal where a tool can *honestly* provide it, with **snapshot-bracketing as the transactional backstop** for "command 3 of 5 failed, revert the group." Composing inverses in reverse order is the ambitious/fragile path; snapshot-bracketing is boring/robust. Let a block's annotation declare which it supports.

The model unifying all of these — and the cleanest articulation of your thruple — is Chef's: **load current state → compare to desired (check/diff) → reconcile (mutate) → optionally invert (undo)**. Chef makes "read current state" a *mandatory separate lifecycle phase* (`load_current_resource`), distinct from "would this change," with guards (`only_if`/`not_if`) as inline checks, `why-run` as check mode, and `notifies` as reactivity. So a sub-decision falls out: is "read current state" a *separate contract verb* from "would this change"? Chef says yes; Ansible folds them together. Your call.

## 3. Underlying engine

Frame first: you buy two things from any engine — maturity, and inherited ecosystem. The trap to see clearly: every existing ecosystem encodes check+execute(+diff) but **none encodes undo**. So "transpile to X for free modules" buys you idempotence-for-free and *zero* rollback-for-free — your headline feature is built at your layer regardless of engine. The engine choice is therefore really about the *less novel* half of your tool.

- **Transpile to Ansible.** Largest module ecosystem, plus check/diff/handlers/inventory/transport. Your notes pick this for v1; defensible — but two costs. First, you inherit the output-buffering and per-action-SSH slowness you hate, and your frontloading/parallelization thesis is *unimplementable* on Ansible's execution model, so you'd lock out your own perf goal. Second, Ansible has no undo, so your transactional rollback lives entirely above Ansible and composes with *no* Ansible primitive — your core feature and your engine don't interlock. GUESS: fine as a *disposable* v1 to develop the language/parser against a real module library; explicitly not the long-term engine.
- **pyinfra as a library.** Python core (noted reservation). But it's usable as a library, gives realtime stdin/stdout/stderr output (kills your buffering complaint), is ~10x faster, and has idempotent operations with dry-run. SUSPECT it's the better *real* engine than Ansible — coherent operation library plus an execution model that doesn't fight your perf goals, at the cost of a smaller ecosystem and a runtime you dislike. The Python is an implementation detail behind your sh-superset; users never see it. Don't over-weight your aesthetic objection to a hidden runtime.
- **mitamae / Chef provider model.** Single static binary shipped to host, Ruby/mruby DSL. Its resource/provider architecture is the *closest conceptual match* to your contract-binary idea — read it as design prior art even if you don't build on it. As an engine: smaller ecosystem, but the no-runtime-on-target property aligns with your "ship binaries from the control node" note.
- **Nix.** Rule out, but understand why: it's the only engine that gives real transactional rollback for free — by owning the entire world (immutable store, functional rebuild), the exact opposite of gradual-enhance-existing-shell. You can't annotate a StackOverflow snippet into Nix. Steal the architecture (atomic generation swap), not the engine.
- **Thin/own engine.** Since you're deprioritizing inventory/transport and the value is the language + contract + snapshot-bracketing, a parser-plus-shell-out-plus-snapshots engine has *no impedance mismatch*. You lose the module library — but that library gives you *none* of your novel half. WONDER if the library is worth less to you than it appears.

Least-regret GUESS: build the front-end against Ansible-transpilation as a throwaway v1 (free modules to test ergonomics), but keep the IR engine-agnostic and target pyinfra-as-library or a thin own-engine as the real back-end. The failure mode to avoid: letting Ansible's execution model leak into your IR, inheriting exactly the constraints that kill frontloading and rollback.

## On "is it just idempotence and version-management?"

No — those are two of ~six core-determining concerns, and not the two hardest. Ranked by retrofit-difficulty:

1. **Rollback/transactionality model** (per-command vs group vs snapshot-bracket) — your headline feature, least prior art, given by no engine. Decide first; it dictates your contract verbs.
2. **Ordering: linear vs DAG** — the copy-paste-vs-frontloading tension; it's the spine of the language and nearly impossible to change later.
3. **Change-signal as a first-class propagating value** (reactivity) — under-designed by default, expensive to add late.
4. **Check semantics** — dirty-bit vs structured-diff, and check-vs-load-current-state as one verb or two; halves or doubles your contract surface.
5. **Idempotence/convergence** — your requirement is probably the stronger one (convergence).
6. **Version-management** — your dep-spec coupling.

On (6), your today's-note instinct maps onto prior art better than you flagged: "which check/undo/mutate applies to foobar 2.x vs 1.x" is *precisely* Puppet's provider-selection problem, and version-coupled task-specs are what Chef cookbook / Puppet module versioning both wrestle with. Your genuinely novel twist is *transitive-dependency-driven* selection — a dependency's resolved version forcing a particular orchestration of its dependent. SUSPECT that's beyond existing tools: they version the module and select providers by *host facts*, not by *transitive version resolution*. If you pursue it, you're building a **constraint solver** (version constraints → selected provider set) — closer to a package manager's SAT-solver than to anything in the config-mgmt lineage. That's a second hard, novel subsystem stacked on the first (rollback). WONDER whether one of those two is the actual thesis and the other is honestly a v2 — two research-grade subsystems in one tool is how NIH projects die before community uptake, which your own notes correctly identify as the thing you can't afford to waste runway on.

---

## HUMAN

For dependency-selection, I have deep personal prior-art; I don't think it's documented in Notion, feel free to go digging, but I've long been planning a language that pushes version-matching down to the *API-node* level (every public value/function-typespec has an inherent semver; calls typed into code can/should include a hardcoded version-specifier; version-resolution is done at the full-set-of-consumed-API-calls level instead of the single-module-version level across all ecosystem participants.) In this model, since we're strict-superset, that would mean something like versioned shell-function/PATH-binaries, `foobar@2` being a sh func() for instance that only functions when the underlying package-manager providing `foobar` is providing `foobar@2.x.y` or similar. For now, not a primary concern; I have more active headspace on this one.

For check/change-signal/DAG, apologies if this wasn't clear — I'm planning a *full* sh superset; this is absolutely non-optional.

As a random example, this is valid shell-script:

if cmd_a_that_mutates --foo --bar; do
   cmd_b_that_mutates;
done

My static-analysis *must* be fully, deeply aware of the control-flow graph; and it *must* have best-effort (built-in-knowledge of shell-builtins; a wide array of best-effort guesses for things like `xargs` and `find -exec {}`; yadda, yadda) approaches for any possible arbitrarily-complex shell-script.

Given the turing-complete nature and myself as a single (admittedly experienced-with-PLT/shell/ops) author, there will have to be some sort of escape-hatch; equivalent of Rust's `unsafe`; something that explicitly loses you control-flow-analysis-totality, but also precludes the enclosing script and any tainted control-flow-subgraphs from ever being performantly skipped on a particular host ...

All that said, rollback/undo/transactionality is very much a random secondary feature I wasn't personally that attached to. For me, the two absolute-must-haves that nothing else in the market seems to provide, are

1. versioning

---

## HUMAN

For dependency-selection, I have deep personal prior-art; I don't think it's documented in Notion, feel free to go digging, but I've long been planning a language that pushes version-matching down to the *API-node* level (every public value/function-typespec has an inherent semver; calls typed into code can/should include a hardcoded version-specifier; version-resolution is done at the full-set-of-consumed-API-calls level instead of the single-module-version level across all ecosystem participants.) In this model, since we're strict-superset, that would mean something like versioned shell-function/PATH-binaries, `foobar@2` being a sh func() for instance that only functions when the underlying package-manager providing `foobar` is providing `foobar@2.x.y` or similar. For now, not a primary concern; I have more active headspace on this one.

For check/change-signal/DAG, apologies if this wasn't clear — I'm planning a *full* sh superset; this is absolutely non-optional.

As a random example, this is valid shell-script:

if cmd_a_that_mutates --foo --bar; do
   cmd_b_that_mutates;
done

My static-analysis *must* be fully, deeply aware of the control-flow graph; and it *must* have best-effort (built-in-knowledge of shell-builtins; a wide array of best-effort guesses for things like `xargs` and `find -exec {}`; yadda, yadda) approaches for any possible arbitrarily-complex shell-script.

Given the turing-complete nature and myself as a single (admittedly experienced-with-PLT/shell/ops) author, there will have to be some sort of escape-hatch; equivalent of Rust's `unsafe`; something that explicitly loses you control-flow-analysis-totality, but also precludes the enclosing script and any tainted control-flow-subgraphs from ever being performantly skipped on a particular host ...

All that said, rollback/undo/transactionality is very much a random secondary feature I wasn't personally that attached to. For me, the two absolute-must-haves that nothing else in the market seems to provide, are

First, the deep-control-flow static-analysis: providing *meaningful* coverage of real-world shell-scripts (i.e. mildly complex control-flow, but not necessarily absurdly pathological edge-cases) such that, given a set of 'idempotence/check modules' built by *somebody*/community for the involved binaries/commands, my language can have a high probability of extracting a no-op parallel-deployment state-evaluation program to ship and execute on all hosts at once. 90% of the value here is "completely skipping hosts that have *no* changes to make, with zero chance of mutative danger on those"; 10% further value is potentially extracting 'slightly-less-comprehensive mutations' at a more granular level than skip-entire-host.
2. deep handling of versioning. You shouldn't be version-managing the third-party-idempotence-scripts; this should be an invisible annotation-layer on top of the version-managers for the tools you already use: if you `npm install foobar@3.5.5` or the like, we should be able to derive and apply the relevant `foobar.check` from our registry-or-whatever. (Whether we push these into the relevant registries, i.e. `npm install foobar.check@3.0.0`, or maintain our own, shouldn't be a concern right now — but we definitely need the ability to hardcode versions deep into the idempotence- and management-scripts.)

Synthesize all of this; and let me know if I'm still missing anything and/or if we're in accordance.

---

## ASSISTANT

You've reframed this significantly — rollback demoted, and the real thesis is now sharp: **sound effect-analysis of arbitrary shell to extract a skip-safe check-program, plus a version layer riding on existing version managers.** Let me dig into your prior art as invited, and confirm the most relevant external work (which I think you'll want) before synthesizing.

No prior record of the version-language design in our chats — I'll work from your description here. Now the external prior art I most want to confirm, because it's almost exactly your must-have #1:

Read the Notion notes and dug for the version-language in our history (no record — working from your description). Confirmed the external prior art I most wanted, and it's close enough to your must-have #1 that it reorders the whole picture. Synthesis below.

## Accordance: revisiting the six pillars from last turn

Your reframe collapses or moves most of them, and I think correctly:

1. **Rollback** — you've dropped it. Affirm: this is the right call and a bigger simplification than "secondary feature" implies. Dropping `.undo` removes the only contract verb that *can't be made sound* (a correct inverse is undecidable in general, and untested inverses are worse than none — the DB-down-migration lesson). You delete transactionality, snapshot-bracketing, and reverse-order composition in one stroke. Don't relitigate it.
2. **Ordering (linear vs DAG)** — resolved, and this is now the *spine*: the script is authored as real linear-with-control-flow shell; the DAG/parallelism is *extracted by static analysis*, never authored. That inversion (analyze-to-parallelize rather than author-a-graph) is the actual architecture.
3. **Change-signal / reactivity / handlers** — I over-weighted this last turn under a flat-YAML-list assumption. In a *real language with real control flow*, `if config_changed; then reload; fi` **is** the handler pattern natively. Ansible needed `notify`/handlers precisely *because* its "language" was a flat list with no control flow. You get this pillar for free. Scratch it.
4. **Check semantics** — still central, now sharper (below).
5. **Convergence vs idempotence** — still load-bearing, now for *precision* specifically (below).
6. **Versioning** — your must-have #2, and the key reframe: **it's a precondition for #1's safety, not an independent feature.** A check-module is only a valid oracle for a *specific tool version*; bind them wrong and you get unsound skips.

So: we're in accordance, with two non-obvious couplings — (#6 gates #2's safety of #3… rather, #6 gates #1's safety) and (your versioned-function idea turns out to feed #1, below).

## The prior art you're almost certainly missing — and it's your exact problem

**CoLiS** (INRIA: Treinen, Marché, Régis-Gianas, Jeannerod, Becker, Sighireanu). They built a platform that does symbolic execution of arbitrary POSIX shell to analyze **Debian maintainer scripts** — i.e. `postinst`/`postrm`/`prerm`, which *are* install/idempotence/removal scripts — for exactly the class of properties you care about. The methodology is based on symbolic execution and feature tree constraints; the toolchain analysed a large set of Debian maintainer scripts and detected over 150 policy violations leading to bug reports, two-thirds since fixed. The scale: over twenty-seven thousand maintainer scripts, almost all written in POSIX shell, executed with root privileges at install/update/removal. Critically, the toolchain symbolically executes the script to compute a symbolic relation between the input and output states of the file system for each outcome — and a prior paper notes they target **relational properties like idempotency** and whether `postrm` undoes `postinst`.

Why this reorders everything:

- SURE: it's an existence proof that sound static effect-analysis of real-world POSIX shell is *feasible*, not just theoretically. Your must-have #1 is not greenfield research; it's been done for a harder version (full symbolic FS-state relations) of your problem.
- SUSPECT: their architecture differs from yours in the load-bearing way. CoLiS encodes **built-in formal specifications of UNIX utilities** (they have a "Specification of UNIX utilities" tech report). You offload utility-semantics to **pluggable community contract-binaries**. Yours scales coverage better (anyone can add `foobar.check`) but is *less sound* (you trust the module; they proved against a spec). That tradeoff is your central design tension, stated precisely.
- **Morbig** (Régis-Gianas & Jeannerod) — their static POSIX shell *parser*, written in **OCaml**. SUSPECT this is a directly stealable front-end for you specifically: it's a no-eval static parser in your primary language. The caveat that matters (below): it's *POSIX*, and real-world scripts are bashisms-heavy.
- **Smoosh** (Greenberg & Blatt) — an executable formal semantics of the POSIX shell, intended as a reference against which to prove static analyses sound. If you want your may-mutate analysis to be *provably* sound (necessary for the "zero chance of mutative danger on skipped hosts" guarantee you stated), this is the semantics you'd prove against.
- **ABASH** — older; abstract interpretation of bash accounting for shell variable expansion, for finding bugs in the arguments passed to commands. Less complete than CoLiS but it's the abstract-interpretation-of-shell precedent, and argument-expansion analysis is exactly a subproblem you'll hit.

Action: before designing the analyzer, read the CoLiS STTT paper and decide *spec-based vs oracle-based*. That single choice determines whether your safety guarantee is "proven" or "trusted."

## The formal core of must-have #1

Stripped down, #1 is: **a sound, forward, may-mutate abstract interpretation over the shell CFG, whose output is a host-local check-program.**

**The soundness asymmetry is your friend.** Lattice with ⊤ = "may mutate / unknown," ⊥ = "provably no mutation." Skip a host iff the join over all reachable paths is ⊥. Unknown commands, `eval`, dynamic command names, missing check-modules all evaluate to ⊤ → may-mutate → can't-skip. So your `unsafe` escape hatch isn't a bolted-on feature; it's *what ⊤ means*, and it degrades safely by construction. False positives (run when you didn't need to) are safe-slow; false negatives (skip when you shouldn't) are the only dangerous direction, and the lattice orientation forbids them. This is the correct shape and you already intuited it.

**The check-program should be a homomorphic transform, not a central dry-run.** Replace each mutating leaf with its `.check` variant, preserve control flow verbatim, accumulate a dirty-bit; ship that to every host; run in parallel; a host reporting "clean" is skippable. SUSPECT this is strictly better than centrally simulating, and it dodges the exact failure I cited last turn — Ansible check mode breaks on data dependencies: if task B needs a file task A creates, A is skipped and B fails. By evaluating the check-program *on the host against real read-only state*, branches resolve correctly instead of being simulated away.

**But guard-position commands force a richer contract.** Your own example (modulo the `if … do … done` typo — I read the intent as "a mutation occurs only if a prior command's exit status says so"):

```
if cmd_a_that_mutates --foo --bar; then cmd_b_that_mutates; fi
```

Here `cmd_a.check` must predict **the exit status the mutate would have returned**, not merely "would it change." Consider `if mkdir /foo; then …`: on a host without `/foo`, mutate returns 0 *and creates it*; on a host with `/foo`, mutate returns 1 *and changes nothing*. The branch taken differs by host state — which is *why* the check runs on the host — but it only works if `mkdir.check` faithfully reports the would-be `rc`. So the contract **bifurcates by usage context**: commands used purely for effect need only the dirty-bit; commands whose exit status (or captured stdout) feeds control flow must *predict that control-flow-relevant output*. Decide this contract split now; it changes the verb surface and the authoring burden.

**Shell-specific soundness hazards you must model (or be unsound):**

- **Redirections are mutation sites independent of commands.** `> file` truncates; `: > /etc/x` mutates with no command at all; here-docs write files. Mutation detection is *not* command-granular. Miss this and you'll skip hosts that a bare redirect would have changed.
- **`set -e` / `set -o pipefail` / `trap` alter the CFG itself.** `set -e` adds implicit exit edges from every un-guarded command; `trap … ERR/EXIT` adds handler edges. These can be set *dynamically/conditionally*. If you don't model them, your reachability set is wrong — and reachability being wrong is an unsoundness, not an imprecision. GUESS this is the subtlest correctness trap in the whole project.
- **`eval`, `$dynamic_cmd`, `. "$dynamic_path"`** — this is shell's actual `unsafe`. ⊤ unless the argument is a static string literal. Morbig refuses these statically, which is a feature.
- **Interprocedural reality**: functions, `source`/`.`, aliases ⇒ whole-program CFG construction, not per-file.
- **Concurrency**: `&`, subshells `(…)`, pipeline stages run concurrently; subshell *env* mutations don't escape but *FS* mutations do. Minor, but a subshell that mutates the FS is still a mutation.

**Slicing is the named mechanism for your "10% granular value."** Compute the backward program-slice from the tainted/dirty set; any subgraph provably clean *and* control-flow-independent of the tainted region remains skippable even when the host as a whole can't be. That's how you get sub-host granularity without hand-waving.

## Must-have #2, and how it feeds #1

Your model — `foobar@2` as a versioned PATH-function that only resolves when the package manager is actually providing `foobar@2.x.y` — does more than ergonomics. SUSPECT the real payoff: **it turns version-dispatch into statically-analyzable control flow.** Version selection becomes a guard your analyzer can *read*, instead of a runtime surprise that collapses to ⊤. So your versioned-function idea isn't orthogonal to #1; it's what keeps version-conditional logic inside the analyzable fragment.

The closest existing prior art to "identity/version at the API-node level" is **Unison** (the language) — content-hash-addressed definitions where identity *is* the hash, name+version is derived. Your whole-set-resolution-over-all-consumed-calls is the constraint-solver generalization of that; it rhymes with package-manager SAT resolution more than with anything in config-mgmt. WONDER if you've looked at Unison's model specifically; it's the limit case of your idea and worth a contrast even if you reject it.

Two version axes you'll need to keep distinct, because they resolve differently:
- (a) **tool-version → which `.check`/`.undo` module to fetch** (a registry-resolution problem, queried against the host's *actually-installed* version).
- (b) **tool-version → which branch of *your* script-logic runs** (your `foobar@2` versioned-function — the Notion "1.x orchestrated differently from 2.x" note).

And a subproblem nobody's solved cleanly: **cross-ecosystem version semantics.** `npm` semver, Debian version strings, Python PEP 440, and brew all compare differently. A resolver spanning them needs a unified version-ordering — closer to the repology universal-version problem than to anything in the lineage. Flag it early; it's a quiet rabbit hole.

## What you're still missing (genuinely new gaps)

1. **The check-oracle trust problem is your single biggest unsoundness risk, and it's a *contract* problem, not an analysis one.** `foobar.check` and the real `foobar` are different binaries, possibly different authors. If they disagree about "changed," you skip a host that needed work — silently and dangerously. This is precisely Ansible's "a module might report 'ok' in check mode when it would actually change something", except your modules are community-authored, so the surface is larger. Mitigations to design in from day one: version-bind the oracle (→ #2), and run periodic **audit runs** that execute the mutate anyway and assert no-change — differential testing of the oracle against ground truth. Treat an untested `.check` like an untested down-migration: untrusted.

2. **The closed-world / determinism assumption.** "Skip iff check-clean" presumes the script's effect is a pure function of (host state, script). Real scripts read the clock, `$RANDOM`, network, env, varying command output. `if [ "$(date +%H)" -gt 12 ]; then mutate; fi` makes check-today and mutate-tomorrow disagree. Soundly, every non-deterministic/external read is ⊤ in the *value* lattice → forces both-branch joins → kills precision fast. SUSPECT the empirical risk: on messy real-world scripts, "skip the whole host" may fire far less often than hoped, because external reads are everywhere. Measure this on a real corpus *before* committing — it's the assumption most likely to deflate the headline win.

3. **Convergence, not idempotence, is what your `.check` must encode** (the CFEngine point, now load-bearing for *precision*). A `chmod` to already-correct perms must report *clean*, or you never skip. The oracle must encode "distance to desired state = 0," not "I would touch the file." Burgess is explicit that convergence-to-desired-state is sometimes incorrectly called idempotence — and the distinction is exactly the difference between a precise check-module and a useless one.

4. **Parser choice gates your real-world coverage, and it fights Morbig.** Morbig is POSIX. Real "copy-paste from StackOverflow" scripts are bash-soaked: `[[ ]]`, arrays, process substitution `<()`, `${x//y/z}`. A POSIX-only front-end *rejects the corpus you're explicitly targeting*. SURE this is a concrete early constraint: either extend a POSIX parser to bash (large) or accept that bashisms fall into ⊤/unsafe (shrinks your coverage win precisely where real scripts live). This directly trades against "meaningful coverage of real-world shell."

5. **Bootstrapping: zero coverage = zero skip = plain shell.** Day one, no community modules exist, everything is ⊤, you skip nothing, and the tool is just a shell runner — your Notion notes correctly name community uptake as the existential risk. Mitigation: ship a built-in corpus of `.check` modules for the ~40–50 highest-frequency operations (coreutils, `apt`/`dnf`/`brew`, `systemctl`, `npm`/`pip`, file/redirect builtins) so there's immediate non-zero value. CoLiS's "Specification of UNIX utilities" is a literal starting corpus to mine.

6. **Partial-state resumption.** If a run dies at command 3, the host is in a partial state; the next check-program must detect *that* and resume correctly. This is convergence-from-arbitrary-state again, and it's where naive sentinel-file checks (`creates:`) fail. Not fatal, but it's where precision goes to die on hosts that have ever failed mid-run.

The throughline: your two must-haves are real and under-served, #1 is feasible (CoLiS proves it), and the hardest problems are *not* the static analysis itself — they're (a) the oracle trust/contract model, (b) the determinism assumption's real-world hit rate, and (c) parser coverage of actual bash. WONDER whether the right next step is a one-week spike: run Morbig (or ShellCheck's parser) over a corpus of your actual homelab scripts and *measure* what fraction parses cleanly and what fraction is ⊤-bound by external reads — that empirically de-risks gaps #2 and #4 before you write a line of the analyzer.

---

## HUMAN

1. (Note: you've used the word 'soundness' several places here. I'd argue that's probably an explicit non-goal; for any given real-world systems-orchestration problem-space, the set of "fully, provably-soundly-described binaries/components/moving-parts" is probably rather small. Treating this project as a functional-programming, Coq-space mathematical issue is … probably the wrong approach for this problem-space; as much as I generally adore correctness and provability, I think the *actual set of served users* for covering that last 5% gap between 'software-engineer correct behaviour' and 'mathematician correct behaviour' is probably fairly small.)
2. I don't see how an audit-run helps. The only 'solutions' I can see (mitigations, really) are general software-quality things; subject to the above logic/constraint, we're targeting "best effort", not "sound." In my head, that includes things like 'a development toolkit for building test-containers that can validate filesystem state against fixed versions of your oracle.' Feel free to push back, but the pareto frontier here doesn't feel like it stretches far into correctness. Real-world usage of this tool will always be against chaotic systems, chaotic tools, chaotic upstream package-environments … … … just look at any Ansible package, they're all always janky, mildly out-of-date, untrustworthy. We're going to be less-popular-than-ansible, so ours will be strictly *worse* than that.
3. I'm less worried about randomness or timestamps than you seem to be guessing. Remember, the purpose of this in the real-world is to sit somewhere in the stack roundabouts Ansible: "make things I need to be true on a bunch of computers, true, in a way that's described fully in some code my team maintains together." Nobody would ever try and *make the promise* (in the CFengine sense) 'the output of the last random() call on this machine was exactly 1111111.' That's an unrealistic, and unuseful, goal. Making oracles *accurate* is likely to include, rather than attempting-to-account-for-randomness, instead *removing* or *excluding* randomness. (Think "copy the logfile with timestamps removed then hash it" or "run the program in a mode that removes all timestamps"), because the entire goal of this is to establish and maintain long-term-truths about the system, such as 'is package installed' or 'is bound to this network' or whatever. Random or time-dependant things aren't truths worth establishing and maintaining.
4. I read a bit of the cfengine literature, I have to admit I'm mildly unsold. Part of the whole reason I want an Ansible-alike / POSIX-sh-based approach to this is that, at the core, I think 'static, provable machine-state' is borderline not meaningfully achievable in the current world — see the adoption-level of NixOS, for example. (Or, to put it another way: *the problem-space being addressed* is the set-of-machines with imperfect state; the set-of-tools/targets with imperfect state-to-be-assigned. Janky containers with instructions from newbie devs that boil down to 'install this with `npm install`'. Attempting to *somewhat* unify a few machines that can't be completely replaced with NixOS Perfection Boxes.) We're not building Coq - we're building TypeScript. In that mindset, I don't think the 'convergence towards set-of-immutable-truths' is the correct model; that's more like Prolog. NixOS already exists and is great for that; containers already exist and don't need to be improved upon. We're *explicitly* building for the classic, unkillable "linux machine somebody's run some commands on, accessible with sshd" use-case, we're *explicitly* building for the "deploying some shitty app that's written by a not-very-opsy programmer, that has no `check` mode, that has no perfectly-clean container, no clean API, etc etc."
5. Parser partially addressed above, but also, probably won't consume upstream parser. I explicitly want to plan on extending the language - not in comments like old FlowJS syntax, but *explicitly*, non-backwards-compatible syntax-extensions. Writing "idempotent, multi-machine State Establishing scripts" is an important/unique enough use-case (and non-compatible with an unrelated Ideal Shell Script for the other shell-scripting purposes) that I don't expect them to be cross-compatible. The point of using shell here is to give a gradual-enhancement path; not slurp up any possible complex bashism as-is. And using an existing parser limits me in all sorts of static-analysis (must-have), fluid-to-write-quickly (must-have), and graceful-failure/high-quality-error-messages (nice-to-have) ways. Parsers also just, aren't that hard. lol.
6. You mention 'ship-to-host' with a homomorphic oracle-transformed script; but I suspect that gains relatively little — the entire *point* of this is to do more substantial compilation/inlining than that. (For instance, `if cmd_a; then if cmd_b; then cmd_c; fi; fi` at runtime, if each of those is annotated to produce `cmd_X.success` as a semaphore, it makes no sense to *serially* execute cmd_a.check, then cmd_b.check, then cmd_c.check. They're all no-op, and even if cmd_b.check is *irrelevant* because cmd_a.check returns a 'success-but-false-exit-code, then we can simply throw away the output of cmd_b.check. Maybe it fails, who knows, who cares. Instead of a homomorphic script, we want to run the equivalent of `if [ -f cmd_a.success cmd_b.success cmd_c.success ]` all at once. (Hell, my suspicion is that *literally all relevant checks* can be run completely in one massively-concurrent block, per-host, without bothering with control-flow at all.) The only value of control-flow analysis is to then determine a relatively minimal set of *mutations* to run, yes?

Okay; now, elaborate on this:
> Action: before designing the analyzer, read the CoLiS STTT paper and decide spec-based vs oracle-based. That single choice determines whether your safety guarantee is "proven" or "trusted."

---

## ASSISTANT



The user is asking me to elaborate on the spec-based vs oracle-based distinction. This is a focused conceptual question, not a research request — they want me to expand on a point I made earlier, and they've spent this whole conversation establishing a very specific design philosophy that directly bears on the answer. They've just spent five numbered points telling me soundness is an explicit non-goal, convergence/CFEngine is the wrong model, they're building "TypeScript not Coq." So the elaboration needs to be filtered through *their* stated frame, not delivered as a neutral academic comparison.

This is not a question that needs the research tool. It's a "elaborate on this thing you said" request in an active high-context design conversation. Launching a research task would be wrong here — it's a conceptual/synthesis question where the user wants my direct reasoning engaging with the specific tradeoff, informed by the conversation we've been having. Let me just answer it well.

Let me also register their corrections, because several are sharp and change my framing — I shouldn't just bulldoze into the elaboration without acknowledging that points 1–4 substantially reframe what "the choice" even means.

---

First, registering your corrections — several change the answer to the question you're actually asking:

**On 1–4 (soundness/convergence are out):** Agreed, and this isn't a minor concession — it dissolves half of what I called "the central tension" last turn. If you're not chasing soundness, then "prove the analysis correct against Smoosh" stops being a goal and becomes, at most, a *spot-check oracle for your own analyzer's development*. The CFEngine convergence model similarly goes from "thing to adopt" to "thing to consciously reject." Your Prolog-vs-TypeScript framing is the right axis: convergence-to-fixed-point is a *declarative* promise about end-state, and you're explicitly building an *imperative gradual-enhancement* tool whose domain is precisely the machines that can't be expressed declaratively. I'll stop using "sound"; the correct word for your target is **"calibrated"** — the analysis should know *what it doesn't know* (⊤) and be honestly conservative there, but it owes you nothing about the 5% mathematical tail.

One narrow caveat I'll still defend: dropping *soundness as a goal* is right, but the ⊤/conservative-skip *direction* still matters, because the cost asymmetry is real (false-skip can break a host; false-run just wastes time). That's not mathematical soundness — it's just "be conservative in the expensive direction." You clearly already hold this; I'm flagging only that "best-effort" shouldn't quietly become "symmetric best-effort," because the two error directions have wildly different blast radii.

**On 2 (audit runs):** You're right that I undersold what it is and oversold it as a *correctness* mechanism. Reframed in your terms, it's not "verify the oracle is sound" — it's exactly your test-container toolkit: a way to pin `foobar.check@3.5` against known filesystem fixtures and catch *regressions* when someone bumps the module. That's a software-quality CI concern, not a correctness proof. I concede the framing; your version is better and I'll fold it in below.

**On 3 (randomness):** Concede entirely, and it's a genuinely good reframe. "The fix for non-determinism is *exclusion*, not *modeling*" is the correct move and it's actually a *design principle for oracle authors*, not just a dismissal: a good `.check` oracle is one that compares **canonicalized** state (timestamps stripped, ordering normalized, volatile fields masked). That's a guideline you'd put in the oracle-authoring contract. It also retroactively answers my "determinism assumption" worry — you're not assuming determinism, you're *requiring oracles to project state onto its stable subspace*. Fine. Withdrawn.

**On 5 (parser):** Fine, and "parsers aren't that hard, lol" is fair for someone whose background is compilers — the cost is real but you're the demographic that can eat it, and the three reasons (static-analysis freedom, fast-to-write, error quality) are each individually sufficient to justify owning it. The one thing I'd hold you to: owning the parser means owning a *semantics* too, and the semantics is where the actual difficulty you waved at lives (see the very last point below). Parsing `[[ ]]` is easy; deciding what `[[ ]]` *means for effect-analysis* is the part that's load-bearing.

**On 6 — this one I think is partly right and partly a trap, and it directly reframes the spec-vs-oracle question, so let me fold it into the main answer rather than answering separately.**

---

Now, the elaboration. And your point 6 is the perfect entry into it, because it exposes that "spec-based vs oracle-based" is actually *two* orthogonal choices I sloppily merged.

## Disentangling the choice into its two real axes

When I said "spec-based vs oracle-based," I conflated:

- **Axis A — where does effect-knowledge live?** In a *static description* the analyzer reads at compile time (a spec: "`mkdir` writes one dir entry, reads parent perms, rc=1 iff target exists"), **or** in an *executable artifact* run at deploy time on the host (an oracle: the `mkdir.check` binary)?
- **Axis B — is that knowledge used to *reason* (predict outcomes without touching the host) or to *probe* (ask the host directly)?**

CoLiS sits at (A=static-spec, B=reason): it has formal specs of utilities and *symbolically executes* to predict the input→output filesystem relation **without running anything**. That's the "proven" pole — and you've correctly thrown it out, because symbolic prediction of arbitrary janky upstream binaries is exactly the 5% mathematical tail you don't serve.

Your instinct all along — `cmd.check` binaries shipped to hosts — is (A=executable-oracle, B=probe): the knowledge is a binary, and you find out the truth by *running it against real host state*. That's the "trusted" pole.

Here's the thing your point 6 forces into the open: **you've actually been assuming you can probe (run all checks concurrently, ignore control flow) AND reason (statically extract the minimal mutation set) at the same time — and those two want the knowledge in different places.** Let me take your "run all checks in one concurrent block" claim seriously, because it's *mostly* right and the place it's wrong is exactly the spec-vs-oracle hinge.

## Why "throw away control flow, run all checks at once" is ~80% right

Your example: `if cmd_a; then if cmd_b; then cmd_c; fi; fi`, each producing a `.success` semaphore. You claim: run `a.check`, `b.check`, `c.check` all concurrently, discard irrelevant results, and the host is skippable iff all relevant checks say "clean."

For the **skip decision specifically**, I think you're right, and it's a real insight I under-weighted. The skip question is monotone: *"is there any reachable mutation whose check says dirty?"* Checks are (by your contract) side-effect-free, so **speculatively running checks down branches that won't be taken is harmless** — you just discard them. So you can indeed evaluate the whole check-lattice concurrently and ask "all clean?" without honoring control flow. The homomorphic-script idea I floated *was* needlessly serial for this purpose. Concede.

Where it breaks — and this is the crux — is the **two ways a check can be "irrelevant," which are not symmetric**:

1. **Control-irrelevant**: `c.check` doesn't matter because `a` failed, so the `cmd_c` branch is dead. You can discard `c.check`'s result. Harmless. ✅
2. **Data-irrelevant... except it isn't.** Consider `pkg_install foo; systemctl enable foo`. Run both checks concurrently against the *current* host: `enable.check` runs against a host where `foo` **isn't installed yet**, because the install hasn't happened — you're probing real state, and the real state is pre-install. `enable.check` errors or reports "no such unit." Is that "dirty" (must act) or "broken" (can't even check)? You *cannot* answer from the host, because the host isn't in the state that the second command would actually see.

This is the data-dependency problem from two turns ago, and notice it **survives your concurrency optimization completely** — running checks concurrently doesn't help, because the issue isn't ordering of checks, it's that *the later check's correct answer depends on the earlier command's mutation having happened*. The host can't tell you about a hypothetical post-install world by inspection.

And **this is precisely where spec-vs-oracle bites**, which is the answer to your question:

- A pure **oracle/probe** model *cannot* resolve case 2, because oracles only know how to interrogate *actual* state, and the actual state is wrong. You'd have to either (a) give up and mark the whole dependent chain ⊤/must-run, or (b) actually perform `pkg_install` first and then probe — but then you're mutating during the "check" phase, which destroys the entire "skip with zero mutation risk" guarantee that you said is 90% of the value.
- A **spec/reason** model resolves it trivially: the spec for `pkg_install foo` says "post-state: foo installed," so when reasoning about `enable.check` you evaluate it against the *predicted* state, not the real one. No mutation needed. This is exactly what CoLiS's symbolic input→output *relation* buys — a symbolic relation between input and output states *composes*, so the analyzer threads predicted-state through the sequence.

So the real content of "the single choice that determines proven-vs-trusted" is: **how do you handle a check whose correct answer depends on an upstream mutation that hasn't run?** And there are exactly three postures, which is the actually-useful version of the dichotomy:

## The three postures (this is the decision)

**Posture 1 — Pure oracle, no inter-command reasoning ("dumb probe").** Every check runs against real current host state, concurrently, à la your point 6. When a check's precondition isn't met (the upstream mutation hasn't happened), you **conservatively mark that check and everything downstream of it as ⊤/must-run**. Skip the host only if the *entire* script is checkable against current state with no unsatisfied data-dependencies.

- Cost: any script with a real install→configure→start chain (i.e. *most* of them) fails to skip past the first mutation, because everything after the first mutation is downstream of an unsatisfied precondition. Your skip-rate craters on exactly the multi-step scripts that matter.
- Benefit: dead simple, zero modeling, no spec language, oracles are trivial to write. Maximally janky-friendly.
- This is the *purest* expression of your stated philosophy, and I think it's **too pure** — it throws away your headline win on any nontrivial script.

**Posture 2 — Oracle + minimal effect-summary ("probe with hints").** This is the one I'd actually push you toward, and it's the pragmatic middle CoLiS's academic framing obscures. Oracles still *probe* real state (no symbolic execution of janky binaries — you keep your philosophy). But each command also carries a **tiny, declared post-condition summary** — not a formal spec, just "this asserts: package `foo` present." Then the analyzer threads *declared* post-conditions forward, so a downstream check can be evaluated against (real current state ⊕ upstream declared post-conditions). The oracle says "given that foo is asserted-installed, would I, `enable`, need to act?" — and crucially, a good `enable.check` can often answer that *without* foo actually being present, because "is foo enabled" is a question about systemd's wants-symlinks, which it can read even if the unit file isn't there yet, returning "would-change: yes, would create enable symlink."

- Cost: you need a *small* declarative annotation layer — "what truth does this command establish" — which is a second contract verb beyond check. But it's a *human-written one-liner*, not a formal semantics. It's the `creates:`/`changed_when` of your system, generalized to "establishes:".
- Benefit: skip-rate survives multi-step chains, you still never symbolically-execute upstream binaries, oracles stay janky-friendly, and the annotation is exactly the kind of gradual-enhancement metadata your TypeScript model is built around. The post-condition *is* the type annotation.
- This is "trusted, but composable." The trust is in the human-declared post-condition, not in a proof.

**Posture 3 — Full spec/symbolic (CoLiS).** Throw out. Named only so the frontier is complete.

So my actual recommendation, in your frame: **you are not choosing between proven and trusted — you correctly rejected proven two turns ago. The live choice is between Posture 1 (probe-only, simple, low skip-rate) and Posture 2 (probe + declared post-conditions, one more annotation verb, high skip-rate).** And I think point 6 accidentally argued you into Posture 1, while your *actual goal* (high skip-rate on real multi-step scripts) requires Posture 2.

## Why this is *the* early decision, concretely

Because it determines **your contract surface — i.e. what a community oracle author has to write**, which is the thing your Notion notes correctly identify as the make-or-break adoption variable:

- Posture 1: an oracle author writes **one** thing — `foo.check` (side-effect-free, reports dirty-bit + control-relevant exit code). Minimal. Maximal adoption odds.
- Posture 2: an author writes **two** things — `foo.check` *and* a post-condition annotation ("establishes: foo installed"). Roughly double the authoring burden per command, and you now need a *vocabulary* for post-conditions (is "foo installed" a free-text string? a typed predicate? namespaced?). That vocabulary-design problem is non-trivial and it's *new* (it's a baby version of a state-assertion language — adjacent to your version-language work, actually, since "foo@3 installed" is both a post-condition *and* a version fact).

And note the **coupling to your must-have #2 you'll want to see now**: in Posture 2, the post-condition "establishes foo@3.5" and the version-resolution "which `foo.check` applies" are *the same fact viewed twice*. The post-condition a command asserts and the version a downstream oracle must be selected for are the same string. So Posture 2 makes your two must-haves *share machinery*, where Posture 1 keeps them separate. That's an argument for 2 — it means the annotation layer you build for versioning is largely the same annotation layer you build for data-dependency-aware skipping.

## The one concrete action, restated for your frame

Don't read CoLiS to decide proven-vs-trusted (decided: trusted). Read **§3.5 (scenario composition) and the "Specification of UNIX utilities" tech report** for one narrow purpose: they had to invent a vocabulary for "what filesystem truth does this utility establish," and *that vocabulary is exactly your Posture-2 post-condition language*. You're not stealing their symbolic engine — you're stealing **their answer to "what's the minimal predicate vocabulary that covers coreutils,"** because that's the design problem that gates your oracle-authoring contract, and they already enumerated it for a couple hundred utilities. Steal the ontology, drop the proofs.

The thing I'd genuinely flag as still-open and possibly the hardest design call left: **whether the post-condition vocabulary is closed (you define a fixed set of predicate types: file-exists, package-present, service-enabled, port-bound…) or open (authors mint arbitrary string predicates).** Closed is analyzable and composable but can't express the long tail of janky-app state. Open is janky-friendly but degrades to ⊤ the moment two authors describe the same truth differently ("foo-installed" vs "foo_present"). That closed-vs-open call on the predicate language is, I'd now argue, a bigger lever on the project's success than the parser or the analyzer — and it's downstream of nothing except picking Posture 2. So that's the next thing to think about, *if* I've correctly guessed you'll land on Posture 2 once you see that Posture 1 kills your skip-rate.

Have I read 6 right — that you'd backed into probe-only, and that the multi-step-skip-rate cost changes the calculus? Or do you think the install→configure→start chain is rarer in your real target corpus than I'm assuming, such that Posture 1's crater doesn't actually hurt much?

---

## HUMAN

I haven't finished reading your message, but a quick response before I proceed reading posture 2&3 - am I missing something this just feels incorrect? *Obviously* there's state-dependencies; that's the entire reason we're using imperative shell-script as the fundamental language. "Step 2. Configure foo" is *obviously* a must-run/unknown, if "Step 1. Install foo" results in falsehood? The inherent nature of imperative-instructions && mutable-state *means* data-dependencies are serial and implicit. Rephrase Posture 1 if you can, to help me, if I simply misunderstand; then I shall read the rest.

---

## ASSISTANT

This is a conceptual disagreement in an active design conversation, not a research request — no tool needed. Let me just answer directly.

You're not misunderstanding — you're *correct*, and the thing you're missing is that I was answering a question you don't care about. Let me separate two phases that I sloppily merged, because the confusion is entirely mine.

There are two distinct moments where "does this need to run" gets asked:

**The skip-the-whole-host decision (cheap, read-only, upfront).** Here, you're 100% right and Posture 1's "crater" is *not a crater* — it's the correct and desired behavior. If a host already has foo installed-and-configured-and-started, then `install.check` says clean, and *because it says clean*, the configure/start steps' preconditions **are** satisfied against real current state — foo really is installed, so `configure.check` runs fine and also says clean, and you skip the host. The data-dependency only "fails to resolve" when an upstream step is **dirty** (foo genuinely absent). And if foo is genuinely absent, then **you are not skipping this host anyway** — there's real work to do. So marking the downstream "⊤/must-run" costs you exactly nothing: you'd already lost the skip the moment `install.check` came back dirty. The whole-host skip is monotone and your concurrent-checks idea works perfectly for it.

So for the 90%-of-the-value case ("skip hosts with nothing to do"), Posture 1 is *fine*, the data-dependency is self-resolving, and I was wrong to present it as a cost. Concede fully.

**The extract-a-minimal-mutation-set phase (your stated 10% goal).** *This* is the only place the data-dependency bites, and it bites only because of what "minimal" means. On a host where foo is absent, the host is already not-skippable. The question is no longer "skip?" — it's "can I run *fewer than all* the mutations?" And the answer requires reasoning about a hypothetical future state: "after I run `install`, would `configure` still need to run?" That's a question about a host-state that **doesn't exist yet** (post-install), so no read-only probe can answer it — you'd have to actually install first, then check configure. Which is fine! You're mutating anyway on this host. But it means the "minimal mutation set" can't be computed *purely upfront by read-only checks*; it partially interleaves with execution.

So the corrected statement of Posture 1 is:

> Posture 1 computes the **whole-host skip** perfectly and cheaply (your concurrent-check insight holds). What it *cannot* do is compute a **minimal sub-host mutation set** ahead of time, because pruning individual mutations requires predicting post-mutation states, and a probe-only model can only ever interrogate *current* state. So under Posture 1, once a host is dirty at all, you fall back to "run the script's mutations from the first dirty point onward, serially, honoring real control flow" — which is exactly normal imperative shell execution.

And restated plainly: **Posture 1 gives you the 90% (skip clean hosts) for free and correctly. It just doesn't give you the 10% (granular sub-host minimization), because that 10% is the part that needs hypothetical-future-state reasoning.** Postures 2/3 exist *only* to buy back some of that 10%. If you don't care much about the 10% — and your "90/10" framing suggests you might not — then **Posture 1 is the right answer and you can stop reading there.** I genuinely over-dramatized its downside; your instinct that "data-dependencies are serial and implicit, that's the whole point of using shell" is exactly right, and it's *why* the skip case self-resolves.

Read on only if the 10% (or the version-machinery-sharing argument at the very end) turns out to matter to you — otherwise I'd argue you've already arrived.

---

## HUMAN

(By the way, "Posture 2: An author writes two things" - we already established earlier that we'll definitely have more than just two things; we postulate a difference between `.diff` and `.check` for instance; with `.diff` being strictly stronger than `.check`, when it can meaningfully be provided. We probably also want something like `.version` and potentially `.version-compare`, so community-handling of versions can be rolled up gradually. While we *want* inline, first-class Quick Idempotency, we also want deep-integration as an API surface *available* to oracle-implementers — any given ops-person can slowly graduate the weird little tool their using into full-support a bit at a time.)

Okay, caught up. Yes, I do want to *design* around the 10%, just not *at the cost* of the 90%. It's not a non-goal, it's just a calibrated-importance subgoal.

Generally speaking, I'm pretty pro-annotation, what you're calling 'posture 2', to some extent. I'm not entirely sold by the *level* to which you're describing it applying; but awareness that we need to gradually introduce features that depend on metadata like that is, largely, the entire reason for starting out from day 1 with a strict "we're a *superset* of POSIX, and our needs come first, not bashism-compatibility" goal. (We're a new language, not an Everything Analyzer.)

"What filesystem truth does this establish" is, to me, too small of a goal. That's importing their project-goals by proxy; they *defined their research-problem* around filesystems, but filesystems are insufficient for 'general ops deployment tool' needs. We need port availability; usernames & passwords; so on and so forth. You mentioned a sort of 'datatype-definition library' approach from, uh, Chef? Puppet? earlier; I suspect that's a better direction; something a little more general and pluggable than 'just filesystem artifacts.'

However, that said, I'm tentatively against frontloading this. If we're *already* undertaking parsing *and* deep CFG analysis, I slightly suspet the 'correct' approach, even if it adds a bit of difficulty, is to *use POSIX sh statements' as the metadata-layer. In the same way we define `yourcmd.check` pattern, where it's just "a command" from the sh viewpoint; I suspect we can establish truth-establishing metadata in the TypeScript 'type narrowing' sense: oracle-authors write `brew install aria2@1; if [ -f /opt/homebrew/Cellar/aria2/1.37.0/bin/aria2c ]; then assert success; fi` to 'annotate' that that filesystem path will exist afterwards. We depend on our CFG-analysis to derive 'facts' and 'constraints-on-when-those-facts-are-true', then apply that to do remote, static analysis that the Ansible 'creates:'/'changed_when:' enables. Feel free to argue that this is unreliastic; but I suspect it's 1. a realistic enough *hope*, and 2. that the fallback-case ("write language-design into our very-under-our-control language to provide a non-sh approach to adding metadata") is Strong Enough that we don't need to worry upfront, we can uncover what our algorithm Needs To Know as we begin annotating real-world tools/binaries.

(Try and stay grounded in the problem-space moving forward; reason about things that come up in this thread *around your knowledge-base of ops tasks.* Remember that NixOS exists, don't try and be it; remember what parts of Ansible people hate, and try to be *good at what it does, and better at what it does poorly.* You're too-often designing in a vaccuum.)

In that vein: Note that, again, in-the-real-world, skipping an entire host for any mutation is a pretty big win; similarly, the *entire machine state* is *likely* to be relatively 'flat', not 'deep': sure, maybe you need to have `create_user theadmin` and `give_sudo_perms theadmin` before basically any other task needs to be completed … but a *large* majority of real-world tasks tend to be relatively shallow dependency-trees: "all the various configure-sshd tasks require installing sshd; but none of the configure-firewall, or configure-docker, or configure-Home-Assistant tasks do." So, even if we're *relatively* mildly 'dumb' (e.g. only handle, as a starting point, very basic linear shell-functions where each subsequent line is a dependant), we could still meaningfull-oracle-check, and meaningfully-SKIP, the user's `configure_home_assistant()` and `configure_docker()` and `configure_firewall()`, even if an oracle-data-dependency-failure means the Planning Host can't reason about the second half of the `configure_sshd()` function because the `is_sshd_installed()` came back false.

Yes? So, 'posture 2', but gently: my final argument is 'this doesn't matter as much upfront as you think', but I'm open to being convinced.

---

## ASSISTANT

Conceding the verb count first: you're right, and "two things" was my error — the real object is a *ladder* (`.check`, `.diff`-strictly-stronger-than-check, `.version`, `.version-compare`, …), and the design question isn't "how many verbs" but "what's the floor and is each rung independently worth its cost." I'll argue below that the floor should be *zero* authoring, and that each verb maps cleanly to a specific tier of skip-granularity — which is a better justification for the ladder than counting it.

On the sh-statements-as-metadata idea: SURE this is the right instinct, and I want to push it harder than you did, because you framed it as *authors adding asserts* (`...; if [ -f PATH ]; then assert_success; fi`) and the stronger version is that **the common case needs no annotation at all — the guard already is the check.** A huge fraction of real-world idempotent shell is already shaped `if ! dpkg -s foo >/dev/null 2>&1; then apt-get install -y foo; fi`. Your CFG analysis can lift the `dpkg -s` guard directly into a frontloadable probe with zero author effort, because the author *already wrote the check, in the place they'd naturally write it.* You've already connected this to `creates:`/`changed_when:`; the framing I'd add is *why* those exist in Ansible at all: `when:`, `creates:`, `changed_when:`, `register`+`rc!=0` are every one of them a YAML re-invention of a shell primitive (`if`, `[ ! -e ]`, `$?`-inspection, `if cmd; then`) — bolted on as *data* precisely because YAML has no *code*. In a real shell superset they collapse back into native control flow. That is the cleanest possible instance of "good at what Ansible does, better at what it does poorly": you delete an entire category of bolted-on metadata by having the control flow Ansible structurally lacks.

And it degrades exactly like the rest of the system: an unrecognized guard command → opaque fact → can't-prove-converged → must-run. Same ⊤-is-safe direction; the analyzer's fact-recognition vocabulary grows on the same gradual curve as oracle coverage. Coherent.

Now the part where I think I *overstated* a risk last turn and should walk it back, because it directly settles your "this matters less upfront than you think." The fact-identity problem (does `dpkg -s foo` match `[ -f /usr/bin/foo ]` as "the same fact") is real but it is **not on the critical path for per-function skip**, and here's the decomposition that shows why:

- **Skip a whole function** (`configure_docker`): needs *only that function's own internal guards*, lifted and run on the host. If `configure_docker`'s body guards its mutations behind `grep -q storage-driver /etc/docker/daemon.json`, the analyzer lifts that grep, runs it, and if it passes the function is a no-op — skip it. This requires *no cross-function fact-matching at all*. The guards are self-contained and idiom-consistent because one author wrote one function. SURE this is the big win and it's fully intra-idiom.
- **Order the mutations** among functions you've decided are dirty: needs cross-function dependency knowledge ("configure_docker after install_docker"). But the fallback is *source order* — what the author already wrote. So you can punt this entirely and still be correct.
- **Minimize within a dirty function** using another function's effects: *this* is the only tier that needs cross-idiom fact-identity, and it's the deep tail of your 10%. Defer indefinitely.

So your flatness argument is *stronger* than you pitched it: per-function skip on a shallow forest needs only intra-function guard-lifting, which is the easy case, and the hard cross-idiom matching only buys you the deepest tier. GUESS that's why "even a relatively dumb linear analyzer" gets you most of the value — the dependency structure of real config (mostly-independent leaf roles over a thin shared base) means function-granularity is both the natural unit *and* the self-contained one.

Here's my actual pushback, and it's on calibration, not mechanism. You filed per-function skip under "the 10%." I'd argue it's closer to *the* differentiator, because of *which Ansible workflow it fixes*. The dominant steady-state pattern is "I changed one role, re-run the whole playbook across the fleet, suffer 20 minutes." In that run, **whole-host skip saves nothing** — every host is dirty by the one changed role. The thing people actually hate (and manually work around with `--tags` and `--limit`, which are *hand-maintained annotations of exactly the subset your analyzer would derive*) is re-running the nine *unchanged* roles on every host. That's per-function skip. So per-function skip isn't the 10% afterthought to whole-host's 90% — in the most-hated workflow it's most of the relief, and whole-host skip is the one that pays off in the rarer pure-drift-check run. The reason I'm *not* concluding "therefore frontload it": per-function skip is *also* the cheap tier (intra-function guards, flat forest), so you get your most-differentiated value from your least-additional-work. The conclusion is architectural, not schedule: don't build the deep minimizer day one, but make sure the IR represents functions/roles as skip-units with liftable guards from the start, because retrofitting that boundary later is painful and it's where the Ansible-killing value concentrates. So: agree on "don't frontload the hard version," disagree that the *value* is 10%.

Two residual risks that are real and grounded:

**Guard-lifting needs effect-classification of the guard command itself.** `if ! systemctl is-active foo` is read-only and liftable; `if ! mkdir /lock 2>/dev/null` is a guard *whose check is itself a mutation* (lock acquisition). The analyzer can't lift the second without knowing `mkdir` is effectful and `is-active` isn't — which is the very effect-knowledge the oracle layer provides. Chicken-and-egg but resolvable: ship effect-classification for the ~couple-dozen common query commands in the built-in corpus (`dpkg -s`, `systemctl is-active`/`is-enabled`, `id`, `getent`, `ss`, `test`/`[`), and an unrecognized guard command → don't lift → run the whole function on-host. Safe fallback, same direction.

**Version-bearing literals are where your two must-haves actually touch, concretely.** Your `aria2c` example bakes `1.37.0` into the path. As a raw literal, that fact is opaque and breaks the moment the installed patch-version differs. This is *precisely* what `.version` buys: `aria2.version` resolves the installed version so the analyzer can fill the hole in the path template and match the fact across patch releases. So `.version` isn't a separate versioning feature bolted beside the metadata layer — it's the normalizer that keeps version-bearing facts from collapsing to opaque. SUSPECT that's the cleanest motivating example you have for why the version verbs exist, and it's worth holding onto. (Fact gen/kill — install establishes "foo present," a later `apt-get remove` kills it — is just standard CFG dataflow; you've got that.)

The one place I'd resist full deferral: the ~dozen *high-fanout base facts* ("package installed," "user exists," "apt cache fresh," "path exists," "service enabled," "port bound," "mount mounted"). These are the shared/deep nodes you already acknowledged, they're depended on by many functions, and they're the ones most likely to be written with *different idioms* across roles authored at different times — so they're exactly where pure-syntactic fact-matching fails worst. But because (per the decomposition above) cross-function facts only matter for the *ordering/minimization* tiers you're deferring, even this is deferrable — it just means: when you *do* reach for cross-function reasoning, give those dozen base facts *canonical predicate forms* and leave the long tail as opaque sh-extraction. That hybrid lets you defer essentially all of the vocabulary-design work now while reserving the small, enumerable, high-value core for when the deep tier earns attention.

The genuinely open question this surfaces, which I don't think either of us has answered: guard-lifting assumes the lifted guard, run on the host in the cheap upfront probe phase, returns *the same verdict it would have returned in-place during execution*. That holds when the guard reads only stable state — but a guard positioned *after* an earlier mutation in the same function reads post-mutation state that doesn't exist yet at probe time (your own serial-data-dependency point, now recursed one level down: inside a single dirty function). WONDER whether the right rule is simply "only the *first* guard in a function is liftable for the skip decision; guards behind an intra-function mutation are not" — which would mean per-function skip keys on the function's *leading* guard only. That's probably sufficient (most idempotent functions lead with their convergence check), but it's an assumption about how people actually write these, and it's the kind of thing I'd want to check against your real homelab scripts rather than assert.

---

## HUMAN

Elaborate on both:

1. "so they're exactly where pure-syntactic fact-matching fails worst"; and
2. "Minimize within a dirty function using another function's effects: this is the only tier that needs cross-idiom fact-identity."

As a presumptive response if I understood half of that — again, the point of something like `mycmd.version` is to 98% of the time be `mycmd.version() { mycmd --version "$@" }` or the like; dumb-as-rocks, bog-standard. Similar, as often as possible, for --check or --is-installed or whatever. We'll need to *support* and *provide leverage* for the long-tail of "I need to write a complicated separate program that interrogates nominally-internal state of this thing to manage a `.diff`"; but they're relatively rare.

Break down one (or a couple) very common real-world cases that are relevant to this static-analysis/facts/'cross-function'(???) discussion, and let's keep ahold of reality here. Present user-stories centered around them, if you think it's useful.

As for lifting out guards from idiomatic shell-script — that's, uhhh, I'm not *totally* sure, but that's I think the *opposite* of what I'm talking about? `if ! dpkg -s foo >/dev/null 2>&1` is the *test* in a *dependent*; but wasn't your point that we'd need metadata to establish, for instance, "the `dpkg -s xyz` command results in `xyz` being a binary in PATH"? I was picturing the flow-analytic/TypeScript-type-narrowing-esque "fact narrowing" approach to be: have the author of the *`dpkg`* oracles, "describe" the "metadata" about what "effects" it has, how to check-whether-its-necessary (`dpkg.check` for example, but not necessarily *only* that, for something as important as dpkg), by writing no-op assertions in a slightly-DSL-y-but-still-executable-shell-script-way in the `dpkg.xxx` functions/binaries they provide. Does that track? *That's* where I was saying the `if [ -f /the/path/to/the/binary ]` would live; the entire *point* of which is to then be statically mapped/checked against a later "consumer" doing `apt-get install -y foo` or whatever.

---

## ASSISTANT

Your architecture is the right one to center, and I conflated two different *inputs* last turn. Let me separate them cleanly, because the distinction is the whole game:

**Provider-supplied (your model):** consumer writes bare `apt-get install -y foo` — copy-pasted from upstream docs that know nothing about idempotency. The *oracle author* for `apt` supplies, in slightly-DSL-y executable shell, both "what this establishes" and "how to check whether it's needed." The analyzer attaches that check to the consumer's bare call.

**Consumer-supplied (what I described):** consumer writes `if ! dpkg -s foo >/dev/null 2>&1; then apt-get install -y foo; fi` — copy-pasted from a *good* StackOverflow answer that already wrote the guard. The check is already inline; the analyzer just lifts it.

Both emit the same frontloaded probe (`dpkg -s foo`). The difference is provenance. And here's why yours is load-bearing and mine is a sideshow: **guard-lifting only helps scripts that are already idempotent.** If every consumer had already wrapped its commands in guards, you wouldn't need the tool — you'd run the script and it'd self-skip. The entire reason-to-exist is turning *bare, non-idempotent, naive-upstream-docs* scripts into skippable ones, and that requires the knowledge to live in the *provider* oracle. So yes, your `[ -f /the/path ]` lives in the `apt`/`dpkg` oracle, as an executable post-condition assertion, statically matched against bare consumers. That tracks, and it's the right center. (Consumer-guard-lifting isn't worthless — even an already-idempotent guard benefits from being *extracted and parallelized* across the fleet rather than run serially per-host — but it's a small win next to making bare scripts skippable.)

The unification worth keeping from the TypeScript-narrowing framing: in your model the post-condition assertion *and* the skip-check are the same executable test. The author writes one boolean shell expression; it documents the effect statically and runs on the host to decide skip. SURE this is why your "dumb-as-rocks 98%" point holds for the verbs too — `foo.version() { foo --version "$@"; }`, and `foo.check` is usually `foo --dry-run`, a query subcommand, or a one-line artifact test. The only verb that routinely climbs into "write a real interrogator program" is `.diff` for genuinely stateful tools (diffing a live nginx `-T` dump, a DB schema, an installed-vs-declared package set). So the ladder's *cost curve* is: floor near-zero, and only `.diff`-on-stateful-tools is expensive. That matches your gradual story exactly.

Now the two elaborations, and they turn out to be the *same hard facts* viewed from two angles.

**(1) Why high-fanout base facts break syntactic matching.**

Consider the most canonical one: *"user `deploy` exists."* User story: you're standing up a new app host. Across your role library, "deploy exists" is established once and depended on by the chown of `/srv/app`, the SSH `AuthorizedKeysFile` under `~deploy`, the sudoers drop-in, the systemd unit's `User=deploy`, and three cron entries — six functions, written across two years.

The problem: *"user exists" has no canonical shell spelling.* Every one of these is legitimate and correct:

```sh
id deploy >/dev/null 2>&1
getent passwd deploy >/dev/null
grep -q '^deploy:' /etc/passwd
```

The `useradd` oracle declares the establish-fact in *one* of those spellings; a defensive consumer guards with *another*; a third function doesn't test at all and just `chown deploy:`s (its precondition is *implicit*, expressed nowhere). A purely syntactic matcher sees three different strings plus a silence, and unifies none of them — even though they're one real-world fact.

Contrast a *low*-fanout fact: "`/etc/foo/foo.conf` exists." Everyone writes `[ -f /etc/foo/foo.conf ]`. The path *is* the canonical identity; there's no other reasonable spelling, so syntactic matching just works. SUSPECT the crisp rule is: **syntactic matching fails exactly on facts that are (high-fanout) AND (have no natural normal form).** "User exists," "package installed," "service enabled," "apt cache fresh," "port 8123 bound" are all multi-spelling. "File at path P exists" is single-spelling. The first set is small and enumerable — which is *why* the mitigation is "give those ~dozen a canonical predicate token (`user-exists:deploy`) that `id`/`getent`/grep all map onto," and leave the path-shaped long tail raw. There's a fidelity wrinkle that *reinforces* this: the path-proxy check (`[ -f /usr/bin/foo ]`) is a *worse* check than the tool's own query (`dpkg -s foo`) — the file can exist while the package is half-configured — so the high-value base facts want a *curated* canonical check anyway, not whatever proxy an oracle author dashed off.

**(2) Why cross-function minimization is the only tier that needs that cross-idiom identity.**

User story: a host running both your base role and your Docker role.

```sh
setup_base()   { apt-get update; apt-get install -y curl ca-certificates; }
setup_docker() { apt-get update; apt-get install -y docker-ce; }
```

*Per-function skip* (the cheap, high-value tier) needs none of this. "Can I skip `setup_docker` wholesale?" is answered by `setup_docker`'s own checks — is `docker-ce` installed (`dpkg -s docker-ce`). Self-contained, intra-idiom, syntactic-matching-fine. SURE this is where 90% of the value is and it needs zero canonical-predicate work.

Cross-function shows up only when one function's *necessity* depends on another's *effect*. Here: both run `apt-get update`. If `setup_base` already freshened the cache *in this same run*, `setup_docker`'s update is redundant and skippable. To know that, the analyzer must recognize both calls establish the *same* fact ("apt lists fresh as of this run") and that base's already holds when docker runs — i.e. match docker's *needs/establishes* fact against base's *already-established* fact, **across the function boundary.** That's the cross-idiom identity. And notice the fact in question — "apt cache fresh" — is precisely a high-fanout, no-canonical-spelling base fact (its real state is a timestamp on `/var/lib/apt/lists/`, barely testable). So **(1) and (2) are the same hard facts**: the facts where syntactic matching fails are exactly the facts you'd need to identify to do cross-function minimization. The two problems collapse into one, which is convenient — it means the canonical-predicate work, *if* you ever do it, buys both at once.

The honest landing, which confirms your instinct: defer it. Per-function skip — the part that kills Ansible's most-hated "I changed one role and re-ran the whole fleet for 20 minutes" — needs only intra-function reasoning on naturally-canonical facts, so syntactic matching carries it and you write *no* canonical predicates day one. The cross-function tier buys you small things (dropping a redundant `apt-get update`, skipping a reload an upstream function already did) at the cost of the entire canonical-predicate ontology. WONDER whether you ever build it at all — but the architectural reservation is cheap and worth making now: represent a fact as an *opaque token plus its source expression*, so that "raw syntactic" today is just "every token is its own identity," and adding canonical predicates later is "some tokens alias together" rather than a representation change. That one decision keeps the door open without spending anything on the room behind it.

---

## HUMAN

For high-fanout facts, I think you're almost reaching for a … *full* interdependancy analysis of system-state, here? Which is probably "absolute-non-starter"-levels of unapproachable?

i.e. deriving that `chown deploy` is dependent on "fact: user `deploy` exists" is *maybe* achievable, across 𝑚 (all sorts of 'facts') × 𝑛 (all sorts of 'commands whose core purpose is to modify-that-particular-fact'); but it's *certainly* not going to catch, say, the command `docker spin-up coolapp.dockeryaml` wherein the coolapp author *didn't* mention in the readme that `useradd deploy` was one of the deploy commands necessary before it would work.

This is, canonically, why *dependencies* exist. In a perfect world, of course, you wouldn't have npm install — you'd just call functions, and the language would Magically Static Analyze Everything and *find out* that the correctly-named-function, at the correct version, for the kind of thing you're trying to do. That is, of course, though, an absurd pipe-dream. Similarly here, I think we *have* to include the ability to establish dependencies between tasks/functions/scripts/state … and while I do see that *one* way to do that is a deep-decomposition of system state into 'families of facts', and then a mapping of those across tools/binaries, that sounds … ambitious, and it's unclear to me the exact value that provides (in the real world, when only partially/imperfectly-applied, unevenly and spikily, across a variety of facts/tools.)

Can you refute and provide value-add examples; as well as justify the reasonableness/lowness of the effort required to have such decomposed/specific state-depedencies, besides just "you have to run install_docker() first", a la Ansible — a dumb, explicit dependency-tree?

---

## ASSISTANT

The correct half of your argument is fully correct, so let me concede it cleanly first: dependency *derivation* is dead on arrival. The `docker compose up coolapp.yaml` → `useradd deploy` case is underivable *in principle* — the dependency lives in coolapp's runtime (a `getpwnam("deploy")` buried in its code), which no static analysis of *your deploy script* can ever see. SURE this is the same wall package managers hit, and it's exactly why declared dependencies exist. Explicit declaration is non-optional. "Decompose state into families of facts and map them across tools" *sounded* like I was reaching for auto-derivation; that misread is on me.

But you've fused two separable things, and prying them apart is the whole answer:

- **Derivation** — does the analyzer figure out *that* a dependency exists? → non-starter, dead, agreed.
- **Targeting** — when a *human hand-declares* a dependency, does the right-hand side name a *producer function* (`requires: install_docker`, Ansible's model) or a *state predicate* (`requires: pkg-installed:docker-ce`)?

Orthogonal. Fact-targeting is *still fully hand-declared* — zero magic, the author writes it. The only question is what the declaration points at. And on that question fact-targeting dominates function-targeting on the two axes this entire project exists to win.

**Checkability — the fact-dependency *is* a frontloadable skip-check.** A function-dependency is not checkable against host state: "configure_docker requires install_docker" means "install_docker ran *this session*," and you cannot inspect a host and learn that a *function* ran. So on a host where docker came preinstalled in the base image, install_docker didn't run and shouldn't — yet the dependency *is* satisfied, and Ansible's model literally cannot say so. It either re-runs install_docker (the idempotency check no-ops but you eat the SSH round-trip, the exact slowness people hate) or you hand-maintain a `when:` guard. A *fact*-dependency (`requires: command-exists:docker`) collapses "already true" and "made true" into one host-checkable predicate; the producer runs *only* where it's actually false. That's not a coincidence — it's the *same check the consumer already needs for its own convergence*, reused.

User story, concrete: 40 hosts, 25 built from an image that ships docker, 15 bare. Fact-dep — `command -v docker` passes on 25, install runs on 15, done. Function-dep — install "runs" on all 40 (25 no-op but pay the round-trip), or you hand-write `when: not preinstalled` and maintain it forever.

**Producer-coupling — fact-deps decouple from implementation.** A function-dep hard-wires *which* producer satisfies the need. Migrate from the apt repo to the `get.docker.com` convenience script and every `requires: install_docker_apt` breaks; you edit them all. `requires: command-exists:docker` names no producer — any function (or base image, or hand-install) that establishes the fact satisfies it. SUSPECT this is the bigger long-run win: function-deps couple your dependency graph to your *implementation*; fact-deps couple it to the *state you actually care about*.

Now the honest limits, because both matter. First, your `coolapp.yaml` latent case: no fact is naturally extracted (the compose command tests nothing, it just fails at runtime), so the author must hand-declare regardless, and fact-form only helps *if* a `user-exists:deploy` predicate exists in the vocabulary and matches the producer's token. Second, a fidelity wrinkle you'll hit fast: `dpkg -s docker-ce` won't match a convenience-script install (different package identity), so "any producer satisfies it" holds only at the *altitude where the check is producer-agnostic* — `command -v docker` (binary on PATH) is robust, `dpkg -s docker-ce` (apt bookkeeping) is not. Picking the check's altitude is real judgment. But note: even an *imperfect* fact-check is more producer-agnostic than a function-dep, which names exactly one producer. So fact-deps still win this axis, wrinkle included.

Here's the synthesis that answers your effort question, and it inverts your intuition — fact-targeting is *less* machinery than a dumb tree, not more: **a function-dependency is just a fact-dependency whose fact is opaque.** `requires: install_docker` = "I depend on the unspecified aggregate postcondition of running install_docker" = the fact `ran:install_docker`, which is merely session-scoped and not host-checkable. That's the *same opaque-token-plus-source structure* we landed on last turn for skip-facts. So you don't build two systems. You build one — fact-targeted dependencies with a topological scheduler — and **Ansible's dumb explicit tree falls out for free as the all-facts-opaque special case.** The scheduler is one algorithm; opaque-vs-specific changes *only the satisfaction check* (opaque: "did it run this session," always false at start, so always runs = Ansible behavior; specific: "does the fact hold on this host," skippable). Supporting specific facts on top is "let some facts be host-checkable" — the identical machinery you already need for skipping.

So the effort answer, precisely:

- **Floor:** `requires: setup_docker` — opaque, = Ansible, no more work than Ansible, works day one.
- **Gradual upside:** *if* a given dependency earns it, refine to `requires: pkg-installed:docker-ce` and gain host-checkability + producer-independence. Per-dependency, opt-in, a one-token edit.
- **Bounded canonical set:** the ~dozen high-fanout facts (`pkg-installed`, `service-active`, `user-exists`, `port-bound`, `file-exists`) over the ~couple-dozen canonical tools that establish them (apt/dpkg/dnf, systemctl, useradd/adduser…), shipped as the built-in corpus so `requires: pkg-installed:docker-ce` "just works" with zero producer-side authoring by the user. *Not* m×n. Everything outside the core stays opaque — free, = Ansible.

Your "uneven, spiky, partial application" worry is therefore the *intended* shape, not a failure mode: opaque is a permanent valid resting point on the gradient, and a host-checkable specific fact is upside you purchase only where it pays. The one thing I'd actually fix now: make `requires:` accept *either* a function name *or* a fact token from turn one, with the function name being literal sugar for an opaque fact — so the gradient lives in the syntax and refining a dependency is a token swap, never a model migration. WONDER if that single syntactic decision is the entire "support fact-deps" cost you were trying to estimate.

---

## HUMAN

Hm. Reason about why this isn’t achievable with “narrowing.” if pkg-installed foo; then assert_failure; fi — this is analyzable as some particular type of fact, yes? Then we ship the noop-test version of pkg-installed to the remote host?

Outside of that, I don’t have a particular shape in mind for dependencies. Some part of me wants to be totalistic and encode them as shell-function-calls, but that’s probably taking things too far.

---

## HUMAN

Hm. Reason about why this isn’t achievable with “narrowing.” if pkg-installed foo; then assert_failure; fi — this is analyzable as some particular type of fact, yes? Then we ship the noop-test version of pkg-installed to the remote host?
Outside of that, I don’t have a particular shape in mind for dependencies. Some part of me wants to be totalistic and encode them as shell-function-calls, but that’s probably taking things too far.

---

## ASSISTANT

You're more right than my last message implied, and the mechanical model you sketched is correct: the `pkg-installed` in that assertion is the read-only oracle, you ship it to the host, it runs side-effect-free, you get a boolean back. The `assert_failure`/`assert_success` isn't shipped — it's *analyzer metadata* marking the intended result, interpreted at plan time. So the assertion cleanly splits: the **test** runs remotely (read-only, frontloadable), the **consequence** ("this is a required precondition") is the planner's to act on. That separation is exactly right and it's load-bearing — hold onto it.

And it generalizes to both sides, which dissolves the separate-`requires:`-syntax question entirely:

- Consumer asserts a **required** fact: `if ! pkg-installed docker; then assert_failure; fi` → "I require `pkg-installed:docker`."
- Producer asserts an **established** fact: `apt-get install -y docker; if pkg-installed docker; then assert_success; fi` → "I establish `pkg-installed:docker`."
- The **edge** is *derived* by fact-matching: consumer-requires-F meets producer-establishes-F → schedule producer before consumer, *or* skip the producer on hosts where F already holds.

So dependencies don't need a parallel declaration system — they're an emergent reading of narrowing on both ends. This is the same "function-dep = opaque fact-dep" unification from last turn, but cleaner: there's no `requires:` keyword at all, just facts asserted in shell. SURE this is achievable, and I was wrong to imply the latent case needed *different* machinery.

Here's the precise residual — the two things narrowing genuinely *can't* do, which is the real answer to your question:

**1. It can't *discover* a latent dependency.** Narrowing is an *expression* mechanism, not a *discovery* one. The coolapp author who forgot to document the `deploy` user requirement still has to *write* `if ! user-exists deploy; then assert_failure; fi` somewhere — the knowledge that the dependency exists lives in coolapp's runtime (a `getpwnam("deploy")` buried in its code), and no analysis of *your* script surfaces it. So when I said "explicit declaration is required," I was right about *knowledge* and wrong about *syntax* — and you caught the conflation. Narrowing changes the *form* of the declaration (inline assertion instead of metadata) but not the *necessity* of a human supplying the fact. That's the whole gap, and it's irreducible.

**2. It can't *select* among multiple producers.** If three functions establish `pkg-installed:docker` (apt, the convenience script, snap) and a consumer requires it on a host where it's *false*, fact-matching yields a *candidate set*, not a choice. On hosts where the fact's already true this is moot (you schedule none — the win). But when you must actually produce it, narrowing alone is ambiguous; you'd need a selection policy or a disambiguating annotation. GUESS this is rare enough to punt, but it's the one place an explicit producer-name still earns its keep.

(A third, minor: *pure ordering with no underlying fact* — "restart A before B because B wants a fresh A," where nothing checkable distinguishes the states. Narrowing has nothing to hang that on; you'd need an explicit ordering edge. Same place Ansible reaches for `meta: flush_handlers`. Rare.)

One refinement your example surfaces: **polarity is a fact-*value*, not just a fact-identity.** `if pkg-installed foo; then assert_failure; fi` as literally written reads as "if foo *is* installed, fail" = *"foo must be absent"* — a negative precondition (an installer that refuses to run over an existing install). I suspect you meant `if ! pkg-installed foo; then assert_failure` ("foo must be *present*"). Either is valid and useful, but it means the analyzer tracks `(fact-identity, required-value)` — `pkg-installed:foo = true` vs `= false` — because the test's boolean sense × the assert direction *is* what distinguishes "requires present" from "requires absent" from "establishes present." Get that wrong and a conflict-dependency reads as a requirement.

On the totalistic "encode deps as shell-function-calls" — your instinct that it's too far is right, and here's the *precise* reason, because it's not obvious: a function-call dependency calls the **producer** (`install_docker`) and leans on its idempotency to no-op. A narrowing precondition calls the **checker** (`pkg-installed docker`) and lets the *planner* decide whether and which producer runs. The function-call version **fuses check-and-fix**; the narrowing version **separates them** — and that separation is the thing your entire frontloading architecture rests on. Fuse them and you lose all three of: (a) hoisting just the read-only checks out for parallel evaluation (a mid-body producer call isn't cleanly hoistable; a marked assertion at the function boundary is), (b) plan-time "this host needs nothing, skip it entirely" (with function-calls you still *enter* `install_docker` and pay its check-cost at runtime rather than pruning it at plan time), and (c) producer-agnostic satisfaction. So function-calls take it too far *not* because they're conceptually wrong but because they collapse the check/mutate split you've spent this whole design preserving.

That said — function-call/opaque dependency is the correct **floor**, exactly as before: when a dependency has no checkable fact (the weird un-interrogable prereq), the honest declaration *is* "just run setup_prereqs first," which degenerates to an opaque call. So it's the ⊤ of the same gradient, not the primary mechanism.

Which gives you the shape you said you didn't have, and it's minimal: **dependencies emerge from narrowing — a required-fact assertion in the consumer, an established-fact assertion in the producer, edge derived by matching; opaque/function-call form as the floor for un-factable prereqs; an explicit ordering edge reserved only for the rare pure-ordering case.** No separate dependency subsystem — it's the fact machinery you already need, read in one more direction. WONDER if the only new syntax this actually requires is a way to mark an assertion as *requires* vs *establishes* vs *conflicts* (the three fact-value intents), and nothing else.

---

## HUMAN

Are you being compacted in this harness? Can you still see the raw content of the first message in this conversation?

---

## ASSISTANT

Two different things, so let me separate them, because the answer differs:

**Messages: yes, intact.** SURE I can see the first message in full — the `userPreferences` block, then the Ansible-alternatives ask, down to the closing parenthetical about hating that Ansible buffers output and wanting to watch a remote ssh command spew nightmare-state in realtime instead of staring at a frozen buffer. The thruple framing — *(standard mutating command, check-command, undo command)* — and the "five-minute process instead of a five-hour one" obscure-game-server example are both still verbatim in view, not reconstructed from memory. The user/assistant turn prose appears present as written; I don't see a summary block standing in for earlier turns.

**Tool results: partially cleared.** The *oldest* tool calls in this thread — the initial batch of `web_search`/`web_fetch` for Ansible alternatives — now show `[Older tool result cleared to save context]` stubs where the raw payloads were. The later ones (CFEngine, CoLiS, the Ansible check-mode docs, the `conversation_search`) still have full content. So it's a rolling/size-based clear of early tool output, not the messages. I retain my *synthesis* of the cleared results (the pyinfra/cdist/babushka/drist notes) because I wrote it into my own prose, but the underlying search payloads are gone.

**The honest limit:** I'm auditing my context from *inside* it. I can report what's observable — message prose intact, oldest tool results stubbed — but I can't fully rule out a form of compaction transparent to me. SUSPECT none of the turns themselves have been lossily summarized (I'd expect a visible summary artifact and don't see one), but that's an inference about my own machinery, not something I can verify with certainty.

Practical consequence worth stating: if you ask me to recover an *exact* figure, quote, or URL from one of those early searches, I likely don't have it and shouldn't fabricate one — I'd re-search. The design reasoning we've built since is unaffected; it's all in the message prose, which is whole.

---

## HUMAN

Let's move to something concrete.

Synthesize everything we've talked about; and be especially careful about frontloading the things I've had to correct you on multiple times into the front of the output; it will probably become prompting-input / agent-direction for planning. Leave out dead-ends or keep them to a brief one-liner, near the end of the output, about something we considered and rejected. Synthesize my latest additions to the Notion page, as I've been taking notes on our research, focused to the relevant bits.

---

## ASSISTANT

I'll synthesize this into a planning brief, append it to your Notion page, and surface it here. Let me load the Notion update tool and check the page's current state first.



---

## HUMAN

Don't edit Notion right now.

---

## ASSISTANT

Stopped — no Notion edits. (For when you do want it: their page already has most of this transcribed, ending mid-sentence at "for static"; the clean move would be appending a consolidated brief at the end rather than touching the existing notes. But not now.)

Here's the synthesis, written as planning-input — corrections-turned-principles first, since those are the load-bearing constraints an agent is most likely to violate:

---

## Project: a strict-superset-of-POSIX-sh that compiles to a skip-safe, parallel host-reconciliation program

A new language, not an orchestrator. You write shell; the tool statically extracts a side-effect-free check-program to decide what (if anything) needs doing per host, and applies the minimal mutations. TypeScript-for-shell: gradual enhancement over existing scripts, not a clean-world rewrite.

### Settled principles (do not re-litigate — these are the things I kept getting wrong)

1. **Best-effort, not sound.** This is not a verification project; the 5% gap between "engineer-correct" and "mathematician-correct" serves ~nobody. *But* the error direction is asymmetric and must hold: `⊤` ("unknown / may-mutate") is the default, and a host/function is skipped only when analysis reaches `⊥` ("provably no change"). False-run (wasted no-op) is fine; false-skip (skipped something that needed work) is the only dangerous direction. "Conservative in the expensive direction," not soundness.

2. **Target = imperfect-state machines, not convergence to immutable truth.** The use-case is the unkillable "Linux box someone SSH'd into and ran commands on," and "deploy a janky app from a non-ops dev — no check-mode, no clean container, no clean API." NOT NixOS, and must not drift toward it; Nix/containers already own the perfect-state case. Promise-theory / CFEngine convergence is the wrong model (it's the Prolog/declarative framing for a problem we're not solving).

3. **Gradual enhancement is the entire point.** Pasting naive upstream-doc commands (`npm install foo`; the three-line game-server README) must Just Run as a working v1. Idempotence, checks, effects, deps layer on incrementally, from *either* end (oracle-author or script-author). Day-1 floor: no worse than a plain shell runner.

4. **Non-determinism is excluded, not modeled.** Random/time-dependent state is not a truth worth establishing. Oracles attain accuracy by *canonicalizing* what they inspect (strip timestamps, mask volatile fields, normalize ordering) — compare the stable subspace; never try to account for noise.

5. **Keystone invariant: keep check and mutate SEPARATE.** Frontloading, plan-time host-skip, and producer-agnostic deps all rest on this. Anything that *fuses* them — lazy in-place idempotency baked into a mutating command, or expressing a dependency as a call to the producer — is rejected.

### The two must-haves (the thesis; everything else is replaceable)

**MH1 — deep control-flow analysis → skip-safe parallel check-program.** Given author/community check-oracles for the commands involved, derive a read-only state-evaluation program, ship it to all hosts at once, run concurrently. **90% of the value = completely skipping hosts/functions with no work to do, zero mutation risk on them.** SUSPECT the per-*function* slice of this is undervalued at "10%" — it's precisely what relieves Ansible's most-hated trait (re-run the whole fleet to change one role; today hand-hacked with `--tags`/`--limit`).

**MH2 — versioning as an invisible layer over existing version managers.** You do not hand-version the idempotence scripts; from `npm install foo@3.5.5` (apt/brew/…) derive and apply the version-correct `foo.check`/effects. Version-bearing facts (paths with embedded versions, `/opt/.../aria2/1.37.0/...`) normalized via a `.version` verb. Grounded in the API-node-level resolution model: versioned shell-functions/PATH-binaries (`foo@2` resolves only when the manager provides `foo@2.x.y`), resolution over the whole consumed-call set.

### Architecture

**Oracle model is PROVIDER-supplied.** Center case = *bare consumer + rich provider oracle*; the oracle author describes a command's effect/check via executable shell assertions. (Consumer-side guard-lifting — script already wrote `if ! installed; then install; fi` — is a minor bonus, *not* the model: it only helps scripts that are already idempotent, while the whole point is making *bare* scripts skippable.)

**Verb ladder, dumb floor.** `.check`, `.diff` (strictly stronger than `.check` where meaningfully providable), `.version`, `.version-compare`, … — graduated and opt-in. 98% are one-liners (`foo.version() { foo --version "$@"; }`; `foo.check` ≈ a query subcommand or `--dry-run`). Only `.diff` on genuinely stateful tools (live `nginx -T`, DB schema) climbs into "write a real interrogator." Design metric: the *floor* is ≈zero authoring → still get value, and each rung independently earns its cost.

**Narrowing = the check IS the post-condition (TypeScript type-narrowing).** Author writes one executable boolean test; it serves as both the static fact and the host-side skip-check. The test runs remotely read-only; the `assert_success`/`assert_failure` is analyzer metadata interpreted at plan time, *not* shipped. Analyzer tracks `(fact-identity, required-value)` — present / absent / established — because test-sense × assert-direction is what distinguishes "requires present" from "requires absent (conflict)" from "establishes."

**Skip tiers (calibrated 90/10, not all-or-nothing):**
- *Tier A — build first (90%):* per-host and per-function skip. Needs only INTRA-function reasoning over naturally-canonical facts (a function's own leading guard decides if it's a no-op). Plain syntactic fact-matching suffices; defer ALL canonical-predicate work.
- *Tier B — defer (deep 10%):* cross-function mutation-minimization. The *only* tier needing cross-idiom fact-identity, hence canonical predicates for the ~dozen high-fanout facts (`pkg-installed`, `service-active/-enabled`, `user-exists`, `port-bound`, `file-exists`) over the ~couple-dozen tools that establish them. Bounded, NOT m×n; everything outside the curated core stays opaque (= free). It's hard *and* deferrable for the same reason: high-fanout facts have no single canonical shell spelling (`id` vs `getent` vs `/etc/passwd` grep), so syntactic matching fails exactly there — but you only need cross-idiom identity for this low-value tier.

**Dependencies are EMERGENT from narrowing — not a separate subsystem.** Required-fact assertion (consumer) + established-fact assertion (producer); the edge is *derived* by fact-matching. Three intents to mark: requires-present / requires-absent / establishes. Opaque/function-call dependency ("just run setup_X first") is the *floor* — the `⊤` of the same fact-gradient (a function-dep = an opaque fact-dep), correct only as fallback for un-factable prereqs, never primary. The only two things narrowing genuinely *cannot* do: (a) **discover** a latent dependency (`docker compose up app.yaml` silently needing `useradd deploy` — knowledge lives in the app's runtime, must be human-supplied; narrowing is expression, not discovery); (b) **select** among multiple producers when a required fact is false (matching yields a set, not a choice → needs policy or a disambiguating name). Minor third: pure ordering with no underlying fact → rare, explicit ordering edge.

**Escape hatch (`unsafe`).** `eval`, dynamic command names, `. "$dyn"`, unrecognized guards → `⊤` by construction. Marking a region `unsafe` forfeits CFG-totality there and precludes the tainted subgraph (and its enclosing function) from being performantly skipped. Degrades safely — `⊤` is the honest conservative value.

**Analysis hazards the parser/analyzer must handle (correctness-affecting, not pedantry):** redirections are mutation sites independent of commands (`: > /etc/x`); `set -e`/`pipefail`/`trap` mutate the CFG itself (implicit exit/handler edges) and can be set conditionally; functions/`source`/aliases ⇒ whole-program CFG, not per-file; `&`/subshell/pipeline concurrency (subshell *env* doesn't escape, *FS* mutation does); fact establish/invalidate is standard gen/kill dataflow (a later `apt remove` kills `pkg-installed`). Own the parser (extensible via explicit non-backwards-compatible syntax, not comment-based) — required for analysis freedom, fast authoring, and good errors; demand pure-POSIX, push complex bashisms back onto the author to simplify first.

### Deliberately out of scope / open

- **Inventory, transport, connection-multiplexing, auth, registry:** deprioritized — borrow or defer. Not the value.
- **Realtime output is a hard requirement** (Ansible's buffered-then-red-error is a hated misfeature — stream remote stdout/stderr live).
- **Engine: OPEN; keep the IR engine-agnostic.** Throwaway-v1 option: transpile to Ansible to borrow its module library for ergonomics testing — but its per-action-SSH + buffering model *cannot* frontload, so it is explicitly NOT the long-term engine and must not leak into the IR. SUSPECT pyinfra-as-a-library is the stronger real engine (realtime output, ~10× faster, idempotent ops, library API) despite the Python runtime (hidden behind the sh-superset). Thin own-engine viable since none of the module library's non-novel half is wanted.

### First de-risking step (genuinely open)

- **Spike before building the analyzer:** run a real shell parser (ShellCheck's, or Morbig) over your actual homelab corpus and *measure* (a) what fraction parses cleanly and (b) what fraction is `⊤`-bound by external/non-deterministic reads. That empirically de-risks MH1's real-world skip rate before a line of analyzer exists.
- Other open calls: whether `.diff` subsumes/replaces `.check` (halves contract surface, raises authoring bar — SUSPECT worth it); the leading-guard-only hoisting rule (GUESS only a function's *first* guard is safely liftable, since later guards read post-mutation state that doesn't exist at probe time — validate against real scripts); represent every fact as `(opaque-token, source-expr)` from day one so "add a canonical predicate later" is "tokens alias," not a migration.

### Considered and rejected (one-liners)

- **Rollback / `.undo` / transactionality:** dropped from the core — correct inverses are undecidable, untested undo is worse than none, reliable rollback is done via snapshots not composed inverses. (Your note still flags zfs/nixos snapshot-bracketing as a "?" — that's the right shape *if* ever revived; the page title's "rollback" is otherwise out.)
- **Symbolic-execution / spec-based proven analysis (CoLiS):** rejected — proves a soundness you don't want; but mine its "Specification of UNIX utilities" for the bounded canonical-fact ontology.
- **Convergence-to-fixed-point (CFEngine / promise theory):** rejected — wrong problem space (NixOS's).
- **Nix as engine / anything own-the-world:** rejected — incompatible with copy-paste-from-StackOverflow as the unit of user activity.
- **Consumer-side guard-lifting as primary:** demoted — only helps already-idempotent scripts.
- **Totalistic function-call dependencies:** rejected as primary (fuses check/fix, kills frontloading); retained only as the opaque-dependency floor.

---

## HUMAN

Okay:

What *is* this? Don't depend on my own description earlier; we've significantly tweaked the design, several original goals have either 1. turned out to be non-starters, 2. been deferred to outside tools, or 3. turned out to fall out for free from the design.

How would you describe the tool that we've arrived at? It clearly fills one specific niche in the problem-space, but it's not really an orchestrator. Run one final research-pass across the web for similar tools now that we've narrowed down the design-space.

---

## ASSISTANT

Before searching, let me state plainly what we actually built, because it sharpens what to look for.

Stripped of your original framing, the thing has *one* defining mechanism and everything else is downstream of it:

> **It statically derives the "check / dry-run" version of an ordinary imperative shell script.** By tracking which commands mutate and replacing each with a read-only check-oracle while preserving control flow, it computes a side-effect-free probe that tells a host *whether anything needs doing* — then applies only the minimal mutations that do.

That single capability — *idempotency-by-analysis instead of idempotency-by-hand* — is the whole product. Every config-management tool in existence makes idempotency the *author's* job (each Ansible module / Chef resource is hand-built to check-before-mutate). Your thesis inverts that: the user writes naive imperative shell, and skip-safety is *compiled* from it plus a library of per-command check contracts. The parallel frontloading, host-skip, emergent dependencies, and version-pinning all fall out of "the check-program is a derivable read-only projection of the mutation-program."

So it is **not an orchestrator** (you ceded inventory/transport/connections), **not a convergence engine** (rejected CFEngine), **not a verifier** (rejected soundness/CoLiS), **not own-the-world** (rejected Nix). The tightest one-liner: ***`terraform plan` / `--check` mode for arbitrary imperative shell — derived rather than declared.*** Let me now check whether anyone occupies that exact niche.

## What it actually is

After the subtractions, the identity of the thing is *what's left*: a **language (POSIX-sh superset) + a static effect-analyzer + a per-command check-oracle contract.** Nothing else survived as core. Rollback, soundness, and full state-decomposition turned out to be non-starters; inventory/transport/connections/auth/registry/the-engine got deferred to other tools; dependencies, handler-style reactivity, and check-parallelism all fell out *for free* as properties of the read-only projection rather than features you build. So the honest one-sentence identity:

> **It's a `plan`/`apply` compiler for imperative shell** — a static effect-analyzer that *derives* the dry-run from an ordinary script, instead of a config-management framework that makes you *declare* desired state or *hand-write* idempotent resources.

The categorical line between it and everything adjacent is single and sharp: **every other tool puts idempotency on the author; this infers it.** Ansible/Chef/Salt/Puppet/Comtrya/mitamae/mgmt all require you to either declare end-state or use hand-built check-before-mutate resources; yours keeps the user in naive imperative shell and *computes* the check-before-mutate structure. It is the idempotency/check *layer* extracted out of config-management and turned into a language feature, deliberately shedding the orchestration framework that's normally bundled with it.

The niche is real and currently hand-hacked. The clearest statement of the gap in the wild: imperative scripts lack diffing and idempotency, and bolting it on by hand could easily double or triple the line count, and notably no one has thought to apply Terraform to bare metal or plain operating-system configuration — its domains are containers and cloud. The plan/apply UX everyone actually wants is the Terraform one: not just telling you what it would do but producing a plan it can later execute, aborting if the assumptions made during planning have changed. That's your exact value proposition, aimed at the imperative-shell world Terraform can't ingest.

## Landscape, by axis

**Language axis — Oils/YSH is your prior art, not a competitor.** Andy Chu's project is the serious attempt to make shell statically analyzable, and it answers your single biggest feasibility question affirmatively: statically parsing shell is achievable — it was the subject of a research paper — and the sources of dynamism (aliases, backticks, etc.) have been identified. He also built Hay ("Hay Ain't YAML") to add the missing declarative part to shell — but that's the *opposite* move (embedding declarative blocks), not inferring convergence from imperative code. SUSPECT you build on Oils' parsing/dynamism work rather than redo it.

**Technique axis — Dozer is the closest anyone has come, and it's instructive precisely because it chose the opposite execution.** NCSU's Dozer (ICSE-SEIP 2022) migrates shell commands into Ansible modules by tracing and comparing system calls to find Ansible modules with similar behaviors, then generating and validating migrations, on the insight that shell commands and Ansible modules can only change system state by communicating with the kernel via the syscall interface. This validates two of your premises (the right altitude for "what does this change" is the kernel boundary; shell commands have mechanical idempotent-equivalents) — but it's *dynamic* (strace, runtime) and a *one-time migration away from shell*, where you are *static* (analysis) and a *runtime convergence mechanism that stays in shell*. SURE this is the single most relevant paper to read, partly as a road-not-taken: dynamic syscall-profiling is the alternative to your static-analysis-plus-oracle approach, and its limitations (you must *run* the command to learn its effects) are exactly why you went static. Infer is the other technique reference — bi-abduction for compositional inter-procedural analysis, written in OCaml, with an incremental/reactive mode for CI — relevant to how you'd structure the analyzer, not a competitor.

**Engine/rethink axis — mgmt is the most ambitious modern competitor, and it's still declarative.** James Shubin's mgmt does reactive automation: modeling time, executing in parallel, reacting to instant infrastructure changes, and explicitly aims to be a better replacement for Terraform via lock-free DAG concurrency, with a deliberately non-Turing-complete language (mcl). It's the closest thing to "next-gen config-management," but it's a *declarative reactive resource graph in its own language* — the inverse of your imperative-shell-with-inferred-checks. Comtrya, similarly, is a lighter Rust provisioner born of the same frustration (the mismatch between configuration management and personal provisioning makes the big tools too cumbersome for the trivial use-case) but it's still YAML manifests with hand-authored providers and a petgraph dependency DAG.

## Verdict

The niche is essentially unoccupied. The *pieces* exist scattered — Oils proves shell is statically analyzable, Dozer proves shell-effects map mechanically to idempotent equivalents, Terraform proves plan/apply is the wanted UX, mgmt proves there's appetite to rethink the engine — but **no one has assembled them into "imperative shell in → derived parallel check-program out."** Everyone either declares state (Terraform, mgmt, Puppet), hand-authors idempotent resources (Ansible, Chef, Comtrya, pyinfra, mitamae), or profiles dynamically to migrate *away* from shell (Dozer). GUESS the reason the static-derivation path is empty is that it's hard and the soundness-tempting framing scares people toward verification (CoLiS) or toward declaring-state-instead (everyone else) — and your explicit "best-effort, ⊤-on-unknown" stance is precisely the move that makes it tractable where others balked.

So the NIH instinct is justified: there's no tool to stop wanting to build. The realistic threats aren't competitors but *load-bearing dependencies you'd lean on* — Oils for parsing, and Dozer's syscall-altitude insight for what a check-oracle should actually observe. WONDER if the sharpest framing for your own writeup is simply "Terraform plan/apply for the `sshd`-and-a-shell-prompt machines Terraform forgot" — it names the niche, the UX, and the deliberately-unglamorous target in one line.

---

## HUMAN

Produce for me, synthesizing all of the above:

1. an AGENTS.md specific to *this project*; for now focused on high-level design-goals, design-pitfalls. See my @elliottcable/System AGENTS.md for things that don't need duplication; and
2. a root-prompt to start an agent exploring this.

---

## ASSISTANT

I don't have access to your `@elliottcable/System AGENTS.md` — it wasn't shared in this conversation and I can't reach your filesystem or GitHub from here. Let me check whether it's in our past chats before I write around it.

I checked our history — your System `AGENTS.md` isn't accessible from here (it's not in this conversation, and I can't reach your filesystem or GitHub). So I've written the project file assuming the System file already carries: communication style + confidence-marker convention, your macOS/VSCode/POSIX-sh environment, and general engineering values (FP, strict typing, TDD, self-hosted > SaaS, etc.). Verify nothing below duplicates it.

```markdown
# AGENTS.md

<!-- codename: TBD -->
> **Scope.** Design charter for the early exploration phase: high-level goals and pitfalls
> only. No build/test/layout conventions yet (greenfield). For communication style,
> environment, confidence-marker convention, and general engineering values, see the
> **System AGENTS.md** — deliberately not duplicated here.

## What this is — internalize before proposing anything

A strict-superset-of-POSIX-sh **language** plus a **static effect-analyzer**. The core
mechanism: take an ordinary imperative shell script (which establishes desired state),
statically derive a **side-effect-free "check" projection** — every mutating command
replaced by a read-only check-oracle, control flow preserved — ship that probe to all hosts
at once, run it in parallel, and apply only the minimal mutations each host actually needs.
<!-- /* superseded 2026-05-30: NOT "control flow preserved." The probe is a cost-based,
behaviour-DROPPING optimizer — hoist cheap independent checks into a flat parallel batch,
keep expensive ones under cheap guards, drop un-provably-inert leaves to apply. How much CFG
is retained is an optimizer decision. See AGENTS.md "What Dorc is" + Research/notes/70. */ -->

One line: **`terraform plan`/`apply` for imperative shell — idempotency *derived by
analysis*, not *authored by hand*.**

It is **not** an orchestrator, **not** a convergence/desired-state engine, **not** a
verifier, **not** own-the-world. The categorical difference from every neighbor (Ansible,
Chef, Salt, Puppet, Comtrya, mitamae, mgmt, pyinfra): they put idempotency on the *author*;
this *infers* it from imperative shell plus a library of per-command check contracts.

## Settled principles — do NOT relitigate or drift from these

The constraints below are the ones most likely to be violated by pattern-matching to
"normal" config-management or PLT instincts. They are settled. If you think one is wrong,
say so explicitly and stop — do not quietly design around it.

1. **Best-effort, not sound.** This is not verification; the gap between engineer-correct
   and mathematician-correct serves nobody here. `⊤` ("unknown / may-mutate") is the
   default; skip a host/function only on `⊥` ("provably no change"). The error direction is
   asymmetric: false-run (a wasted no-op) is fine; **false-skip (skipping something that
   needed work) is the one forbidden outcome.** "Conservative in the expensive direction" —
   nothing more.
<!-- /* superseded 2026-05-30: refined two ways. (a) "best-effort, not sound" → "sound
over-approximation, BOUNDED" (verification is bounded, not a non-goal). (b) There are TWO
soundnesses with OPPOSITE fail-directions, never conflated: *probe-soundness* (read-only pass
must not itself mutate; uncertain ⇒ withhold/defer execution) and *elision-soundness* (this
paragraph — never skip needed work; uncertain ⇒ perform). One ⊤ cannot serve both. See
AGENTS.md §1 "Two soundnesses". */ -->
2. **Target = imperfect-state machines, not convergence to immutable truth.** The user is
   the unkillable "Linux box someone SSH'd into and ran commands on," and "deploy a janky
   app from a non-ops dev — no check-mode, no clean container, no clean API." Do **not**
   drift toward NixOS / declarative desired-state; Nix and containers already own the
   perfect-state case. Promise-theory / CFEngine convergence is the wrong (declarative)
   model.
3. **Gradual enhancement is the entire point.** Pasting naive upstream-doc commands
   (`npm install foo`; a three-line game-server README) must Just Run as a working v1.
   Idempotency / checks / effects layer on incrementally, from *either* end (oracle-author
   or script-author). **Day-1 floor: no worse than a plain shell runner.**
4. **Non-determinism is excluded, not modeled.** Random/time-dependent state is not a truth
   worth establishing. Oracle accuracy comes from *canonicalizing* inspected state (strip
   timestamps, mask volatile fields, normalize ordering) and comparing the stable subspace.
   Never try to account for noise.
5. **Keystone: keep check and mutate SEPARATE.** Frontloading, plan-time host-skip, and
   producer-agnostic deps all rest on this. Reject anything that fuses them — lazy in-place
   idempotency baked into a mutating command, or a dependency expressed as a call to its
   producer.
6. **Oracles are provider-supplied; center case = bare-consumer + rich-oracle.** The oracle
   author describes a command's effect/check via executable shell assertions — *the check
   IS the post-condition* (TypeScript type-narrowing). Consumer-side guard-lifting (a script
   that already wrote `if ! installed; then install; fi`) is a minor bonus, **not** the
   model: it only helps *already-idempotent* scripts, whereas the whole point is making
   *bare* scripts skippable.
7. **Dependencies are emergent from narrowing, not a subsystem.** Required-fact assertion
   (consumer) + established-fact assertion (producer); the edge is *derived* by
   fact-matching. Three intents to mark: `requires-present` / `requires-absent` /
   `establishes`. Opaque/function-call deps ("just run `setup_X` first") are the **floor
   only** — the `⊤` of the same fact-gradient. Narrowing genuinely *cannot* (a) **discover**
   a latent dependency (a human must supply it — expression ≠ discovery), nor (b) **select**
   among multiple producers when a required fact is false. Everything else is matching.
8. **Strict POSIX; our needs come first, not bashism-compatibility.** A new language, not an
   Everything Analyzer. Copy-paste targets *simple* upstream commands; complex bashisms get
   pushed back on the author to simplify, not absorbed. **Own the parser** (extensible via
   explicit, non-backwards-compatible syntax — not comments): required for analysis freedom,
   fast authoring, and good error messages.
9. **Engine-agnostic IR.** Inventory/transport/execution are deferred (see Out of scope).
   The IR must not bind to any one engine, and an engine's execution model — especially
   Ansible's per-action-SSH + output buffering — must never leak into it.

## The two must-haves (the thesis; everything else is replaceable)

- **MH1 — deep control-flow analysis → skip-safe parallel check-program.** 90% of the value
  = completely skipping hosts/functions with no work to do, *zero* mutation risk on them.
  The per-*function* slice of this is the part that relieves Ansible's most-hated trait
  (re-run the whole fleet to change one role; today hand-hacked with `--tags`/`--limit`).
- **MH2 — versioning as an invisible layer over existing version managers.** From
  `npm install foo@3.5.5` (apt/brew/…), derive and apply the version-correct
  `foo.check`/effects. Version-bearing facts (paths with embedded versions) normalize via a
  `.version` verb. Grounded in the API-node-level resolution model (versioned
  shell-functions/PATH-binaries; `foo@2` resolves only when the manager provides
  `foo@2.x.y`; resolution over the whole consumed-call set).

## Design pitfalls — the predecessor design process hit every one of these

Phrased as "don't X; instead Y," because these are recurring failure modes, not edge cases.

- **Don't reach for soundness/verification.** If a choice is justified by "to be *provably*
  correct," stop — that's rejected Coq-mode. (Principle 1.)
- **Don't reframe toward declarative/desired-state.** Wanting the user to declare an
  end-state or a resource graph means you've drifted to the rejected convergence model.
  (Principle 2.)
- **Don't build the canonical-fact vocabulary upfront.** The 90% (per-host / per-function
  skip) needs only *intra-function syntactic* fact-matching. Canonical predicates are a
  *bounded, deferred* concern for the deep cross-function tier only. Represent every fact as
  `(opaque-token, source-expr)` from day one so canonicalization is later a "tokens alias,"
  never a migration.
- **Don't make dependencies a separate declaration system.** (Principle 7.)
- **Don't fuse check and mutate.** (Principle 5.)
- **Don't model non-determinism.** (Principle 4.)
- **Don't let scope creep into "analyze any bash."** (Principle 8.)
- **Analysis hazards — correctness-affecting (unsound in the *bad* direction if missed),
  not pedantry:**
  - **Redirections are mutation sites independent of the command** (`: > /etc/x` mutates
    with no command; here-docs write files).
  - **`set -e` / `set -o pipefail` / `trap` mutate the CFG itself** (implicit exit/handler
    edges) and can be set conditionally. Getting reachability wrong here is unsoundness.
  - **`eval`, dynamic command names, `. "$dyn"`, unrecognized guards → `⊤` by
    construction.** This is the `unsafe` boundary: marking a region `unsafe` forfeits
    CFG-totality there and bars that subgraph *and its enclosing function* from performant
    skip. `⊤` is the honest, conservative value.
  - **Whole-program CFG** (functions / `source` / aliases), not per-file.
  - **Concurrency:** `&`, subshells, pipeline stages — subshell *env* mutations don't
    escape, *FS* mutations do.
  - **Fact establish/invalidate is standard gen/kill dataflow** (a later `apt remove` kills
    `pkg-installed`).

## Skip granularity (calibrated 90/10 — design *around* the 10%, never *at the cost of* the 90%)

- **Tier A — build first (the 90%): per-host and per-function skip.** Needs only
  intra-function reasoning over naturally-canonical facts (a function's *leading* guard
  decides if it's a no-op). Plain syntactic fact-matching suffices.
- **Tier B — defer (the deep 10%): cross-function mutation-minimization.** The *only* tier
  needing cross-idiom fact-identity, hence canonical predicates for the ~dozen high-fanout
  facts (`pkg-installed`, `service-active/-enabled`, `user-exists`, `port-bound`,
  `file-exists`) over the ~couple-dozen tools that establish them. Bounded, **not** m×n;
  everything outside the curated core stays opaque (= free).

## Out of scope / deferred to other tools

Inventory, transport, connection-multiplexing, auth, sharing/registry, and the execution
engine itself — borrow or defer; not the value. **Realtime output is a hard requirement**
(Ansible's buffered-then-red-error is a hated misfeature; stream remote stdout/stderr live).

## Reuse, don't rebuild

- **Oils / YSH** — shell parsing + the already-done identification of shell's dynamism
  sources (aliases, backticks, …). Lean on this for the parser; do not redo the
  "can-shell-be-statically-parsed" work.
- **Dozer (NCSU, ICSE-SEIP 2022)** — its insight that command effects live at the
  syscall/kernel boundary directly informs *what a `.check` oracle must actually observe*.
  (We are static where it is dynamic — read it as premise-validation, not as an approach.)
- **Infer** — bi-abduction / compositional inter-procedural analysis, OCaml,
  reactive/differential mode: architecture reference for the analyzer.
- **CoLiS "Specification of UNIX utilities"** — seed ontology for the bounded canonical-fact
  core (Tier B), *not* its symbolic-execution engine.

## Rejected — do NOT propose

- Rollback / `.undo` / transactionality (undecidable inverses; untested undo worse than
  none; reliable rollback is snapshots, not composed inverses).
- Symbolic-execution / proven-sound analysis *as a goal*.
- Convergence / promise-theory / CFEngine model.
- Nix / anything own-the-world.
- Consumer-side guard-lifting as the *primary* mechanism.
- Totalistic function-call dependencies as the *primary* dependency mechanism.
- Dynamic syscall-tracing as the primary effect-detection mechanism (that's Dozer's path;
  we are static).

## Open questions — flag, do NOT silently resolve

- **Implementation language.** OCaml-leaning (reuses the Morbig/CoLiS lineage; matches
  author strength) — but unsettled.
- **`.check` vs `.diff` collapse.** A diff subsumes a check (dirty-bit = `diff ≠ ∅`);
  making `.diff` primary halves the contract surface but raises the authoring bar.
- **Leading-guard-only hoisting.** Likely only a function's *first* guard is safely liftable
  for the skip decision (later guards read post-mutation state absent at probe time) —
  validate against real scripts.
- **Predicate vocabulary: closed vs open** for the Tier-B canonical core.
- **Producer-selection policy** when a required fact is false and multiple producers exist.
```

And the root-prompt to kick off an agent:

```
You're helping me explore and design a greenfield language/tool. Read AGENTS.md (this
project) and my System AGENTS.md in full before responding. The project file's "Settled
principles," "Design pitfalls," and "Rejected" sections are load-bearing — treat them as
hard constraints, and if you think any is wrong, say so directly and stop rather than
quietly designing around it.

Phase: high-level design + feasibility. The niche is already validated — no existing tool
derives a static check-projection from imperative shell; the closest neighbors and the
pieces worth reusing are in AGENTS.md. I am NOT asking you to start building the compiler. I
want to push the design forward and de-risk the riskiest assumption before committing
effort.

Do, in order:

1. De-risk MH1 empirically. Propose (and run, if you have the access) a minimal spike: take
   a corpus of real-world install/provision shell — my homelab scripts, distro maintainer
   scripts (postinst/postrm), README one-liners; suggest sources if you can't reach mine —
   parse it, and MEASURE two numbers: (a) what fraction parses cleanly under a strict-POSIX
   front-end, and (b) what fraction of mutating leaves would be ⊤-bound by
   external/non-deterministic reads (unskippable regardless of oracle coverage). These two
   numbers gate whether the thesis is worth pursuing. Report with caveats; do not oversell a
   small or biased corpus.

2. Produce a first-cut design sketch for: the narrowing annotation surface (how an author
   marks requires-present / requires-absent / establishes, and how the inline
   cmd.check/.diff/.version contracts attach); the effect lattice (⊤/⊥ and what sits
   between); and the check-projection transform (imperative script → read-only parallel
   probe). Keep it a sketch. Mark every choice the design underdetermines as an explicit
   open question — do not silently resolve the ones already listed in AGENTS.md.

3. Make a reuse-vs-build call for each item under "Reuse, don't rebuild": concretely, what
   would you take from Oils' parser, and what does Dozer's syscall-boundary insight imply
   about what a .check oracle must actually observe?

Working style: I would much rather answer a series of clarifying questions than have you
guess and produce work in the wrong direction — ask freely where the design is
underdetermined, especially on the open questions. Use explicit confidence markers
throughout. Push back hard where my framing is wrong: I am a fallible human and you are a
fallible model, so signal uncertainty rather than projecting false confidence.
```

One caveat on dedup: since I couldn't read the System file, the project file may double up on the confidence-marker convention or the "ask clarifying questions / push back" working style if those live at the system level — both the AGENTS.md and the root-prompt currently restate the working-style expectation. If the System file already covers it, cut the closing paragraph of the root-prompt and the parenthetical in pitfall headers.

---

## HUMAN

Several of those references look fairly old/outdated, although admittedly some of these OCaml-space projects are giving ‘done, not dead.’

I don’t understand the FB Infer docs, frankly. Smth smth process calculus? A bit beyond me.

I don’t use Terraform, explain the vibes there and why I care.

Finally, apply all of this logic to a real-world Ansible galaxy collection — review the features that ended up needed and required in real-world Ansible modules; review each of the attributes of an Ansible task, and figure out how they fit into our plan, at a high level.

---

## ASSISTANT

Three things, in order. The first two are quick; the third is the real work.

## Infer / bi-abduction, demystified

Skip the process-calculus framing — it's incidental. The one idea worth extracting: Infer figures out, *per function, in isolation*, "what must be true on entry for this not to crash, and what's true on exit" — a (precondition, postcondition) pair — and then *composes* those pairs across call sites to reason about a whole program without re-analyzing callees. "Bi-abduction" is just the inference rule that *guesses* the missing precondition. The reason I flagged it: that (pre, post)-per-procedure-then-compose structure is exactly your dependency model — a function's *required facts* are its precondition, its *established facts* are its postcondition, and matching-across-call-sites is composition. So Infer is prior art for "derive and thread pre/post-conditions through a call graph without whole-program re-execution," which is the mechanically hard part of your Tier-B analysis. You don't need its math; you need the knowledge that compositional pre/post inference is a solved, shipped technique. That's the whole takeaway.

## Terraform, the vibes, and why you'd care

The workflow that matters is two commands. You write a description of what you want; you run `terraform plan` and it prints a **diff against reality** — "will create these 3 things, change these 2, destroy this 1" — *without touching anything*; you read it, and if it looks right you run `terraform apply` to enact exactly that plan. The plan is a real artifact, not just advisory text: it's a plan it can later execute programmatically, and it aborts if the assumptions made during planning have changed.

Why you care, concretely: that plan/apply split **is your project's entire UX, already validated and beloved by a huge user base.** Your "ship a read-only check-program, see what each host would change, then apply the minimal mutation set" *is* plan/apply — derived from imperative shell instead of from Terraform's declarative HCL. So Terraform tells you (a) the interaction model you're targeting is proven and wanted, and (b) the precise gap you fill: Terraform's plan/apply only works for things with clean declarative providers (cloud APIs, mostly), and — the line worth internalizing — no one has applied the Terraform model to bare metal or plain OS configuration; its two domains are containers and cloud. You're plan/apply for the SSH-and-a-shell-prompt machines it ignored. The borrowable vocabulary: "plan," "apply," "drift" (reality diverging from description), and "refresh" (re-probing current state) — all four map directly onto your design and are worth stealing as terms of art.

Now the real exercise.

## Applying the design to real Ansible

I'll do this in two passes: first the **per-task keywords** (the anatomy of a single task — this is where your design either absorbs or rejects each Ansible concept), then **what a real Galaxy collection actually ships** (the module/collection level — this tells you what the *ecosystem* demands beyond single tasks). Let me ground the second pass in a real collection rather than assert from memory.

### Pass 1 — anatomy of an Ansible task, mapped

Ansible's task keywords sort cleanly into four buckets relative to your design. The interesting finding: a large fraction are *workarounds for YAML having no control flow*, and those **collapse to nothing** in a shell superset — they're not features you implement, they're features you delete.

**Bucket A — collapses into native shell (you implement zero of these):**
- `when:` → `if`. A conditional task is just a guarded command.
- `loop:`/`with_*` → `for`.
- `register:` → variable assignment (`x=$(...)`).
- `block:`/`rescue:`/`always:` → `{ ...; }`, `trap`, and `||`. Ansible invented a try/catch because YAML couldn't express one.
- `notify:`/handlers → `if changed; then reload; fi`. This was the big one two conversations ago: handlers exist *only* because a flat YAML task-list can't say "do X later iff something changed." You get it free.

SURE this bucket is the core of your "better at what Ansible does poorly" claim — every item is a data-language reinvention of a code primitive you already have.

**Bucket B — becomes your check/effect contract (the heart):**
- `changed_when:` → this *is* your `.check` / the dirty-bit half of narrowing. Ansible already admits per-task that "did this change something" is author-supplied metadata; you're generalizing it from per-invocation to per-command-reusable.
- `failed_when:` → your success/failure predicate; the exit-status half of the contract.
- `creates:`/`removes:` → the cheap-idempotency floor (skip if sentinel exists). Your "first-class `don't re-run if file exists`."
- `check_mode:`/`--check` and `diff:` → literally your derived plan/apply. Ansible bolts these on per-module and they're unreliable (modules opt in, many lie); you *derive* them, which is the whole thesis.

**Bucket C — deferred to other tools (you cede these, per the design):**
- `become:`/`become_user:` (privilege escalation), `delegate_to:`, `connection:`, `remote_user:` → transport/identity. Not yours.
- `tags:` → this one's poignant: tags are the *manual* version of your automatic per-function skip. People hand-annotate `--tags` to avoid re-running everything; you derive that set. So you don't implement tags — you **obsolete** them.

**Bucket D — genuinely needed, currently underweighted in your plan (the useful surprises):**
- `vars:`/templating (Jinja2) → you have shell variables, but Ansible's *fact* system and templated config files are heavily used; question is whether config-file *templating* (render-then-converge) is in scope or ceded. Flag, don't resolve.
- `async:`/`poll:` → long-running operations. Your realtime-output requirement partly covers the motivation, but "fire-and-check-later" is a real pattern (a 40-minute `apt dist-upgrade`).
- `retries:`/`until:`/`delay:` → **convergence-by-retry.** This is the one I'd genuinely add to the plan: real ops needs "try, check, retry until the fact holds or N attempts" because reality is flaky (a service takes time to come up). It's `until: result is fact` — and it interacts with your check contract directly (the retry condition *is* a check). SUSPECT this deserves first-class syntax; it's not derivable from a bare script and it's ubiquitous.
- `run_once:`, `throttle:`, `serial:` → fleet-level execution policy. Mostly cedeable, but `serial:` (rolling-batch deploys: "10 hosts at a time so the service stays up") is a real production need that your parallelism model should at least *not preclude*.

Let me ground Pass 2 in a real, heavily-used collection rather than guess at what modules end up needing.

Good — `community.general` gives a concrete cross-section. Let me ground Pass 2 in what these modules actually have to *do* internally, because the gap between "what the keyword surface looks like" and "what the module author actually wrestled with" is where your design gets tested.

### Pass 2 — what real modules actually need

Looking at representatives across the collection — `nmcli` (NetworkManager), `ufw` (firewall), `locale_gen`, `modprobe`, `filesystem`/`lvg`/`lvol` (storage), `apache2_module`, `timezone` — a few patterns recur that single-task keywords don't reveal. These are the demands your `.check`/`.diff`/effect contract has to satisfy in the wild.

**1. `state: present|absent` is universal, and it's two oracles, not one.** Essentially every module is parameterized by desired-state, and the check for "present" is not the inverse of the check for "absent" in any cheap way. `nmcli ... state: present`, `locale_gen state: present`, `ufw` rules — the module internally branches. For you: a command's `.check` must be *relative to intended end-state*, which means the oracle can't be a property of the binary alone — `aria2.check` is meaningless; `aria2.check --for-state=installed` is the unit. SUSPECT this nudges your contract toward "the check is derived from the *mutating invocation including its args/flags*," not from the bare command name. Your versioned-function model already points here (`foo@2` carries intent), but state-direction is a second axis beyond version.

**2. Modules are overwhelmingly "converge one resource to a described state," and the check is a *field-by-field diff*, not a boolean.** `nmcli` is one of the most feature-rich modules in the collection precisely because a NetworkManager connection has ~40 properties and "changed?" means "does any of these 40 differ from desired?" This is your `.diff` verb, and it tells you `.diff` is not a rare luxury for stateful tools — it's what *most* real modules fundamentally are. The boolean `.check` is the degenerate case (`diff ≠ ∅`); the field-level diff is the common case for any module touching config. GUESS this raises the priority of `.diff` in your ladder: the moment someone wants to manage `nmcli`/`sshd_config`/`ufw`-grade resources, they need structured diff, not a dirty-bit.

**3. The dominant internal shape is read-current-state → render-desired-state → diff → mutate-the-delta.** This is the Chef `load_current_resource` lifecycle, and nearly every config-touching module reimplements it by hand. For your design this is the single most important structural finding: **your derived check-program should compute the same (current, desired) pair the module would, and the "mutate" phase applies only the delta.** It also surfaces a scope question you've been deferring — config-file *templating* (render desired file, diff against on-disk, write iff different) is so common (`template:`/`copy:` are among the most-used modules of all) that ceding it entirely may be untenable. Flag hard: is "render-and-converge a file" a first-class thing you provide, or do you lean on existing tools (`m4`, `envsubst`, a `.diff` oracle around a templating binary)? It's the highest-frequency operation in real playbooks and your plan is currently silent on it.

**4. A lot of the collection is thin idempotent wrappers over a CLI — which is *exactly* your oracle model, validating it.** `modprobe`, `timezone`, `locale_gen`, `apache2_module` are mostly "run the tool, but check first whether it's needed." `apache2_module` enables/disables a module; `locale_gen` generates a locale if not present. These are 20-line wrappers whose entire value is the check-before-mutate. SURE this is the strongest existence-proof for your thesis in the whole exercise: a large fraction of the most-used config-management modules in the world are doing *manually, per-module* the exact check/mutate split you propose to *derive* — meaning the community has already demonstrated both the need and that the per-command effort is small (your "dumb-as-rocks 98%"). The flip side: the *count* is enormous (`community.general` alone has hundreds), so your bootstrap-coverage problem is real, and the migration path from "existing Ansible module" to "your oracle" is worth a thought — Dozer's syscall-tracing is one way to *bulk-generate* oracle checks from existing module behavior.

**5. The hard, non-cedeable demands that aren't in any single keyword:**
- **Partial-application / failure mid-resource.** `lvol` resizing a volume that's half-done, `nmcli` applying 38 of 40 properties then failing — the module must be re-runnable from a dirty intermediate state. This is your convergence-from-arbitrary-state requirement, and it's why `.check` must be *current-state-relative*, not *script-progress-relative*.
- **Ordering that's genuinely cross-resource and physical.** `lvg` (volume group) must exist before `lvol` (logical volume); a filesystem before a mount. These are real establish/require fact-edges (`vg-exists:foo` → `lv-creatable`), and they're the *high-fanout base facts* from two conversations ago, now concrete. Confirms Tier-B's canonical predicates would eventually be needed for the storage/network stack specifically.
- **`check_mode` support is per-module and incomplete in reality.** Many modules don't implement it or implement it wrong (the Magento `--dry-run` footgun I saw earlier is the same disease). Your derive-don't-declare approach is the structural fix, but it means your analyzer is on the hook for the cases module authors punt on — and they punt precisely on the hard ones (anything that reads state produced by an earlier mutation). That's your leading-guard-only / data-dependency limit, now confirmed as the same place the incumbent gives up.

### The synthesis worth keeping

Mapping the whole surface, the design holds up with three adjustments to the plan, ranked by how much they change things:

1. **`.diff` is not a tier-2 luxury — it's the median module.** Real config management is field-level convergence of multi-property resources. Promote structured diff toward the core; the boolean check is the degenerate case of it. (SUSPECT this also resolves the earlier open question — `.diff` subsumes `.check`, and the authoring-burden worry is offset by the fact that most real modules *already* compute the diff.)
2. **The check/effect oracle is keyed on the *invocation* (command + args + intended state), not the command.** `state: present|absent` being universal means intent is part of the contract's identity. Folds into your versioned-function idea as a second axis.
3. **Config-file templating is the highest-frequency operation and your plan is silent on it.** Decide whether render-and-converge is first-class or delegated. This is the one genuine gap the Ansible cross-section exposes that wasn't already in the design.

Everything else either collapses into native shell (the control-flow keywords — confirming the "better than Ansible at what it does poorly" thesis), maps cleanly onto the check/effect contract (`changed_when`/`failed_when`/`creates`/`check_mode`/`diff`), or is correctly ceded (transport/identity/fleet-policy) — with the lone addition of `retries/until` (convergence-by-retry) as a small first-class construct, since reality is flaky and it's not derivable from a bare script.

---

## HUMAN

I was planning/assuming that the “contract” for our oracles is “attempt to mirror the exact CLI of the binary.” Partially because many modules offer smart flags (—dry-run or whatever that explicitly validates the *exact thing you requested*.)

(We should be requesting our oracle-authors to fail-fast, though; be very strict about the accepted/understood/checkable/diffable flags; fail fast on any unparsable flag/arg. Check should be high-confidence when it exists at all.)

Diff, I’m unsold on your points. Just because it’s useful doesn’t mean it’s the default — ie. we don’t want an oracle-author collapsing “check-if-package-present” to a “create a full listing of every package version on the system, then filter it down by grep to the single package we care about.” Usually there’s performant/precise/correct privatives for individual checks, and I’d prefer authors defer/drill down, not just implement a “full diff” and say “good enough.”

File-editing seems like one of the best *possible* defers? Very few things in this world are as thoroughly executed and battle-tested as “plaintext file editing/diffing on *nix”, no? Sell me; what can we provide here that’s genuinely unique to our altitude?

---

## ASSISTANT

Three separate things; you're right on two and I'll concede cleanly, but the CLI-mirroring contract has a sharp edge worth surfacing before you commit.

## Diff — you're right, I overstated it

I conflated "most modules *conceptually* converge a multi-field resource" with "the oracle should *implement* a full diff." Those don't follow. Your `grep`-the-whole-package-list example is exactly the failure mode: a full-listing-then-filter is `O(system)` to answer an `O(1)` question, and shipping that to run concurrently across a fleet is precisely the cost you're trying to avoid. The performant primitive (`dpkg -s foo`, `systemctl is-active foo`) is both cheaper *and* higher-confidence, and an author reaching for diff-as-default is usually papering over not knowing the precise primitive.

So I withdraw "diff is the median." The correct framing is the one you're implying: **`.check` is the default and should use the narrowest correct primitive; `.diff` is an opt-in refinement for the genuine multi-field-convergence cases** (the `nmcli`-with-40-properties resource where "what specifically differs" is the actual question, and where there's no single cheap boolean). Authors should drill down to precise checks, not collapse up to a diff. The one residual I'd still defend: when a resource *is* irreducibly multi-field, the honest `.check` is itself a conjunction of primitives, and at that point a structured `.diff` is just that conjunction with the per-field results retained instead of `&&`-ed away — so diff isn't a *different* mechanism, it's "don't throw away which field failed." But that's a narrow case, not the default, and your instinct to make authors earn it is correct.

## File-editing — you're mostly right, and the defer is good, but the altitude is real

Concede the core: plaintext editing/diffing on Unix is maybe the most battle-tested tooling that exists, and reimplementing it is insane. `diff`/`patch`/`sed`/`m4`/`envsubst`/`jq`/`augtool` exist and are excellent. So "render the file with whatever tool, write it" is correctly *not* your job.

But notice what those tools are: they're all **content transformers**, not **convergence-checkers**. That's the altitude gap, and it's the same check/mutate split as everywhere else in your design. `envsubst` renders a file. It does not tell you *whether the on-disk file already matches the render* — and that boolean is the thing your frontloaded check-program needs. So the defer is real but narrower than "files are a solved problem": you defer the *transform* (render desired content) and the *diff/patch* (compute and apply delta), and what remains at your altitude is the **convergence predicate around them** — "would writing this change anything," answered read-only, frontloadably, as part of the host-skip decision.

Concretely, the idiomatic render-and-converge is already a clean check/mutate pair built from deferred tools:

```sh
envsubst < tmpl > /tmp/rendered          # transform — deferred
cmp -s /tmp/rendered /etc/foo.conf       # check — read-only, your altitude
# mutate: cp /tmp/rendered /etc/foo.conf  (only if cmp says differ)
```

`cmp -s` *is* the file `.check`. You provide nothing novel in the comparison itself — `cmp` is forty years old. What's unique to your altitude is the same three things that are unique everywhere in this design, and no file tool offers them because they're not file-tool concerns:

1. **The render is itself a mutation site that must be lifted out of the skip-path.** `envsubst < tmpl > /tmp/rendered` writes a file. Your analyzer has to know that's a scratch write (safe, ⊥) versus `> /etc/foo.conf` (real, must-skip-gate). Distinguishing those is CFG/redirection analysis — your job, not `cmp`'s.

2. **Establishes/requires fact propagation through the rendered file.** "After this converges, `/etc/foo.conf` contains the desired content" is a fact a *downstream* command (`nginx -t`, `systemctl reload`) can depend on. The file tools have no notion of this; it's your narrowing layer. This is what lets "reload iff the config changed" fall out instead of being hand-wired.

3. **A diff is something a human approves at plan-time.** This is the one genuinely user-facing thing, and it's the Terraform-plan vibe applied to files: when your `apply` is about to write `/etc/foo.conf`, showing the `diff` of current-vs-rendered *in the plan, before touching anything* is exactly `terraform plan`'s changed-resource output. The underlying `diff` is deferred; the *surfacing of it as part of a reviewable, abortable plan across N hosts* is your altitude. No file tool does fleet-wide pre-application diff review because that's a plan/apply-engine concern.

So the honest sell is narrow and I won't inflate it: **you provide zero unique file-editing. You provide the convergence-check, the effect-fact, and the plan-time diff-surfacing *around* deferred file tools** — the same wrapper you provide around `apt` or `systemctl`, with the bonus that for files the underlying check (`cmp`) and diff (`diff`) primitives are so good that the oracle is nearly free to write. Arguably files are your *best-case* oracle, not a special case: the deferred primitives are higher-quality than for almost any other domain.

## The CLI-mirroring contract — agree, with one edge that bites

"Mirror the exact CLI; lean on `--dry-run`/`--check` where the tool offers it; fail-fast on any unparsable flag" is a good contract and the fail-fast strictness is exactly right — a check that silently mishandles a flag it didn't understand is worse than no check, because it reports clean-when-dirty (your only dangerous direction). High-confidence-or-refuse is the correct stance. Strong agreement there.

The edge, and it's worth deciding now rather than discovering later: **the tool's own `--dry-run` answers a subtly different question than your check-program needs, and conflating them reintroduces the data-dependency bug.** Two specific failures:

- **Polarity / intent.** `apt-get install --dry-run foo` reports *what apt would do* (would unpack, would configure…). Your check needs *would this change the system from desired-state* — which is the *negation* plus a convergence judgment. On an already-installed host, `apt --dry-run install foo` still prints a plan ("0 upgraded, 0 newly installed" — but you have to parse that it's a no-op). So the oracle author isn't just forwarding `--dry-run`; they're *interpreting its output into a dirty-bit*. The contract should say so explicitly, or you'll get authors who forward `--dry-run`, get exit-0, and report "clean" when it actually would act.

- **`--dry-run` runs against *current* host state, which is wrong for any command behind a prior mutation.** This is the data-dependency limit, resurfacing through the CLI-mirror contract specifically. `systemctl enable foo --dry-run` on a host where `foo.service` isn't installed *yet* (because the install is an earlier line) reports failure/nonexistence — not "would enable." The tool's dry-run can't answer the counterfactual "what if the earlier line had run," because it's interrogating reality, not your plan. So CLI-mirroring is sound for the *leading* check of an independent operation, and breaks for checks downstream of an in-plan mutation — the exact leading-guard-only boundary from before. The contract is fine; just don't let "mirror `--dry-run`" imply "dry-run is a valid check in all positions." It's valid in the position where current-state = pre-command-state, i.e. the frontloadable host-skip check, which is the 90% you care about anyway.

Net: the contract's right, the strictness is right, and the one thing to write into the spec explicitly is that an oracle's job is **"emit a high-confidence dirty-bit by interpreting the tool, refusing on anything it can't interpret"** — not "forward the tool's dry-run flag and trust its exit code." The dry-run flag is an *input* the author may use to compute the bit, not the bit itself.

---

## HUMAN

1. "tmp isn't depended-upon by anything" seems rather straightforward and dosen't seem like it needs awareness/special-casing;
2. "file contains <x>" as a prereq is exactly the sort of thing we want to encourage authors to handle 1. granularly and 2. with first-class tools with rich oracles (i.e. "use `jq` to extract the desired key, check it's precisely the right value, before invoking the conf-consuming binary" — don't assert state-was-changed-to-all-of-my-favs-overall-earlier-with-zero-diff … assert *individual-narrow-state-fact-i-currently-depend-on*, using natural/minimal/granular/precise checking-tooling.)
3. similarly, cmp/diff/patdiff/`patch` are not our concern, just that the contract-point ("when parsed as yaml, key X has value Y, which is my requirement to run") matches between producer and consumer.

I'm still confused by the --dry-run claim you're making; you've made a similarly-shaped complaint earlier in the convo, and I didn't follow it then either. Didn't we determine that this didn't matter, at least specifically for the `check` case?

---

## ASSISTANT

You're right to push, and you're right on all three numbered points — I'll concede those in one line each and then isolate the `--dry-run` thing properly, because there are *two* different claims I've been making under one banner and you've correctly killed one of them already. The other is narrower than I've made it sound.

The three: (1) yes — scratch-temp-not-depended-on falls out of ordinary dataflow, no special-casing, withdrawn. (2) yes, and this is a better statement of the design than mine — narrow precise fact via the natural tool (`jq -e '.key == "val"'`), not "I rewrote the whole file earlier"; that's the *whole* fact-granularity principle and I was muddying it. (3) yes — the contract is the predicate match, the byte-mechanics are deferred.

Now `--dry-run`. Let me separate the two claims, because they really are different and you already disposed of one.

**Claim A — "checks behind a prior mutation can't run against current state."** This is the data-dependency thing. You correctly determined it *doesn't matter for the host-skip case*, and you were right: if an earlier line is dirty, the host isn't skippable regardless, so the fact that a later check can't evaluate is moot — you've already lost the skip. I keep re-raising it and it keeps being irrelevant to the 90%. **Drop it. Settled. My fault for relitigating.**

**Claim B — a different, smaller thing about what a `--dry-run` *returns*, that has nothing to do with ordering.** This is purely about a single command, evaluated against a host where its precondition *is* satisfied — no data-dependency, no "earlier line," none of Claim A. The point is just: **the tool's `--dry-run` reports "what I would do," and that is not always the same boolean as "is the system already in the desired end-state."** They coincide for most tools and diverge for a specific class.

The cleanest example. `apt-get install --dry-run nginx`, on a host where nginx is *already installed and current*:

```
0 upgraded, 0 newly installed, 0 to remove and 0 not upgraded.
```

Exit code: 0. If an oracle author follows "mirror the CLI, forward `--dry-run`, trust the exit code," they get exit-0 and report **clean** — which is correct here. But now the *same* command on a host where nginx is installed but a held-back version exists, or where a dependency would be pulled:

```
The following additional packages will be installed: libfoo
0 upgraded, 1 newly installed, ...
```

Exit code: still 0. `--dry-run` succeeded — it successfully *planned*. The exit code reflects "did planning succeed," not "would anything change." So an author trusting the exit code reports **clean** when the answer is **dirty**. The dirty-bit lives in the *parsed output* ("1 newly installed"), not in the *exit status*.

That's the entire claim. It's not about ordering, not about current-vs-hypothetical state, not Claim A. It's: **`--dry-run`'s exit code answers "did the dry-run run," and an oracle needs "would the system change" — which for some tools requires parsing the dry-run's output, not reading its exit code.** Tools split into two kinds:

- **Convergent-dry-run** (`rsync -n`, `systemctl is-active`, `dpkg -s`): output/exit *already* encodes "would this change / is it already so." Mirror it directly; the contract works as you stated.
- **Planning-dry-run** (`apt-get --dry-run`, `terraform plan`, many `setup`-style tools): exit code = "planning succeeded," and "would it change" is in the structured output. Author must parse it into the bit.

So this isn't an argument against your CLI-mirror contract — it's one sentence to *add* to it: the oracle emits the dirty-bit by **interpreting** the tool (exit code *or* parsed output, whichever actually encodes change), and "forward `--dry-run` and trust `$?`" is a latent foot-gun for the planning-dry-run class specifically. Your existing fail-fast-on-unparsable rule mostly already covers the danger, because an author strict enough to refuse on unrecognized output won't blindly trust an exit code either — but it's worth naming the trap explicitly so authors of `apt`-shaped oracles don't fall in.

And the reason it's *low-stakes*: it's a per-oracle authoring caveat, caught the first time anyone tests `apt.check` against an already-satisfied-but-would-pull-deps host, and it only affects the planning-dry-run minority. Not a design-level concern — a line in the oracle-author guide. If that still feels like a distinction without a difference for the tools *you* actually care about, that's plausibly correct: SUSPECT most of your real targets (`systemctl`, `dpkg`/`apt` at the *is-installed* altitude, file predicates via `jq`/`cmp`) are convergent-dry-run or have clean boolean primitives, so you may never hit the planning-dry-run class in practice. In which case: noted, shelved, move on.
