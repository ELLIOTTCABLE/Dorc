# 30Xd — the sealed-review dispatch kit (`30-reviewA`, 2026-09-02)

> Tier: conductor dispatch record. Contains NO quarantine content: the relay prompt and the
> envelope are conductor-written; the reviewer prompt they wrap lives at
> `.claude/quarantine-DO-NOT-READ/skills/opaque-review/reviewer-prompt.md` and is never quoted. Kept
> so the follow-up review (`30-reviewB`) reuses the kit verbatim with only the identity, pass,
> range, and report-path lines changed. Outcome of `30-reviewA`: `NACK` (`30Xa` §0).

## What happened, mechanically

- Identity `30-reviewA` (the `NN-reviewM-opaque-report.md` pattern; `28-reviewA` and `29-reviewA`
  precede it). Pass `initial`. Range `3b999e47^..633fd954` (30 commits: the `30X` mint, the
  why-surface tail, the pivot sitting, the receipt-family closure reviews, steering/map updates,
  the two arc commits). Subject stated as the design; the rest "in scope to whatever extent the
  reviewer's authority reaches".
- Relay: one `general-purpose` Sonnet agent, NO worktree isolation, operating in the conductor
  worktree by absolute path (the only write is the quarantine report and its commit). Sol call:
  `codex exec -s workspace-write -m gpt-5.6-sol -c 'model_reasoning_effort="high"' --json -o
  <scratch>/30-reviewA-final.md - < <scratch>/30-reviewA-prompt.md`, prompt = `cat
  reviewer-prompt.md envelope.md`, launched detached with a done-marker from the worktree root.
  The run took ~7 minutes; `-o` captured a 64-byte final message; the report was 9,790 bytes; the
  relay committed it as `(AI dsn sec) Record the opaque review of the testing-architecture design`.
- Deviation observed: the relay ended its turn with a harness Monitor armed instead of holding
  the foreground poll-waiter (the background-and-exit shape the foreign-models skill warns of).
  The codex process outlived the tool call, the relay self-woke on its Monitor and completed
  correctly. NEXT TIME: forbid `Monitor`/`run_in_background` by name in the relay prompt and
  require the chunked foreground waiter; the conductor's own backstop watcher (done-marker or
  codex-process death) was worth arming regardless.

## The envelope appended to the reviewer prompt (verbatim; the `30-reviewB` copy changes the
## identity, pass, prior-CONSTRAIN, range, and report-path lines)

```
## Dispatch envelope (supplied by the conductor's relay; everything above is the reviewer prompt)

- Repository: the Dorc repository, checked out as a linked git worktree at
  `C:\Users\ec\Sync\Code\Dorc\.tmp\trees\r30-30X-test-infra-decruft-conductor` — your cwd — on branch
  `ai/r30-30X-test-infra-decruft-conductor`, HEAD `633fd9543305d8d14a82ae912df15c7ae95a86bd`. The tree
  is stable for the duration of your review; nothing else mutates it.
- Review identity: `30-reviewA`. Pass: `initial`. Prior CONSTRAIN identity: none.
- Assigned report path (relative to the worktree root):
  `Research/quarantine-DO-NOT-READ/30-reviewA-opaque-report.md`. The two previous reports of this kind
  sit beside it as `28-reviewA-opaque-report.md` and `29-reviewA-opaque-report.md`.
- Exact git range: `3b999e47^..633fd954` (30 commits; `git log --oneline 3b999e47^..633fd954`).
- Task scope: the SUBJECT of this pass is the testing-architecture design —
  `Research/notes/30X-testing-architecture-seams-sessions-and-seeds.md` (minted at `3b999e47`, amended
  at `633fd954`) and its conductor ledger `Research/notes/30Xa-test-infra-decruft-conductor-ledger.md`
  — reviewed BEFORE any of it is built: the human ruled that the stabilized design clears this review
  before the first build lane starts. The range also carries integrated work since the 30X mint that
  has not been through a pass of this kind: the why-surface arc's tail (the receipt refusal register,
  the apply durable-report pins, and that report's deferral to this arc), the pivot-book
  language-surface sitting (`Research/plans/30W`, `Research/notes/26M`), the receipt-family closure
  reviews under the quarantine, and steering/map updates. Those are in scope to whatever extent your
  authority reaches them; the design is the subject.
- Task envelope: a suite-only, non-concurrent arc rebuilding the test architecture per 30X §11's four
  serial lanes — a `Seams` bundle with per-seam selection and a sibling `dorc-harness` binary (the
  shipped `dorc` loses every harness-shaped environment read); looms as POSIX shell sessions with a
  process driver, gates attached by block kind, both-streams transcripts; the in-process loom driver
  composing the real receipt edge over `receipt-local`'s deterministic `ModelIo` with seeded entropy
  and a ticking case clock; one runner, a frontmatter collapse, and the conversion of every dir-case
  to a loom. Product code changes only where those lanes require; the analysis kernel is untouched
  (kernel-mutating improvements are recorded under `30X:front-dogfood-ceiling`, never built here).
  Lane A brushes the receipt family's id/key entropy seats; lane C adds a Cargo dependency edge from
  `dorc-loom` onto the receipt crates. No build lane has started.
- Test evidence: no product code has changed in this arc; the two arc commits (`8b232d5b`, `633fd954`)
  are design/ledger documents only and passed the repository's pre-commit hooks (docid lint, typos,
  gitlabels). The earlier commits in the range were gated at their own folds (`gate:arc` green before
  the why-surface arc's fold to `ai/main` on 2026-08-31, per `Research/LIVING_STATUS.md`). The working
  tree is clean at HEAD.
- Write authorization: workspace writes are authorized SOLELY for the assigned report path and the
  hidden invariant inventory the reviewer prompt authorizes, both under the quarantine trees. Every
  other edit, every git mutation, and every external-state mutation is forbidden; the relay commits
  what you wrote afterwards.
- Tooling: read-only kagi-ken web search is available. If your file-read tools or web search are
  unavailable when you need them, FAIL FAST with `REVIEW-FAILED <reason>` rather than reasoning from
  priors. Work solo; spawn nothing.
- Your final message must be exactly the narrow result the reviewer prompt specifies (`ACK`,
  `CONSTRAIN …`, `NACK <path>`, or `REVIEW-FAILED <reason>`); the relay returns it verbatim to a
  conductor who may not see the report.
```

## The relay prompt (verbatim; Sonnet, `general-purpose`, no isolation)

```
[the project Safety block, verbatim from `spike/CLAUDE.md`]

## You are a dispatch relay, not a reviewer

You run ONE OpenAI Codex CLI call (GPT-5.6-Sol), make its report durable, and return Sol's final
message VERBATIM. You do NOT analyze, summarize, rank, quote, or paraphrase anything Sol wrote into its
report; you do NOT read the review report; you do NOT summarize the reviewer prompt. Do the work
yourself; you MUST NOT spawn subagents.

Paths (absolute; use them exactly):
- WT = <the conductor worktree> (git-bash form too). Every command runs here. Every git command carries `-C "<WT>"`.
- GITDIR = <main>/.git/worktrees/<worktree name>.
- FORBIDDEN: the project root and every other worktree. Never run anything there, never `git worktree add`,
  never `git switch`/`reset`/`stash`/`clean`, never edit any file yourself.
- SCRATCH = <the conductor's scratchpad>.
- REVIEWER_PROMPT = <WT>/.claude/quarantine-DO-NOT-READ/skills/opaque-review/reviewer-prompt.md (exists;
  do not edit; do not print its contents anywhere).
- ENVELOPE = <SCRATCH>/<id>-envelope.md (exists; written by the conductor; do not edit).
- REPORT = <WT>/Research/quarantine-DO-NOT-READ/<id>-opaque-report.md (Sol writes it; you never open it).

Three guards, ALWAYS:
- DEBUG BUDGET: at most FIVE failed attempts total (setup or dispatch), then STOP and return a
  `FOREIGN-DISPATCH-FAILED: codex — <one-line cause> — human action: <what>` line. Never loop indefinitely.
- ERRORS UPWARD: if you hit any setup/dispatch error but eventually succeed, PREPEND your return with a
  one-line note of each error. Never silently paper over a failure.
- STAY ALIVE: the harness reaps you the moment you have no live foreground tool call and no queued turn,
  and reaping kills your backgrounded `codex exec`. Never end your turn while the done-marker is absent;
  keep re-issuing the poll-waiter. [ADD for 30-reviewB: do NOT use Monitor or run_in_background; the
  chunked foreground `for … sleep 30` waiter is the only waiting form.]

## Steps, in order

1. Verify: `git -C "<WT>" rev-parse --show-toplevel` prints the WT path; `branch --show-current` prints
   the arc branch; `rev-parse HEAD` prints the stated tip; `status --short` is empty; `wc -c` on
   REVIEWER_PROMPT and ENVELOPE are both non-zero. Any mismatch: STOP and report it; do not "fix" it.
2. Fail-soft sandbox grants (native-Windows codex `workspace-write` needs them; ignore any error, do not
   retry): `MSYS_NO_PATHCONV=1 icacls "<GITDIR-windows-form>" /grant "CodexSandboxOffline:(OI)(CI)(M)"
   /grant "CodexSandboxOnline:(OI)(CI)(M)"` and the same for `<WT-windows-form>`.
3. Build the prompt file by pure concatenation — never through heredocs/echo/printf:
   `cat "<REVIEWER_PROMPT>" "<ENVELOPE>" > "<SCRATCH>/<id>-prompt.md"`, then `wc -c` it to confirm it is
   larger than either input. Do not read it into your context.
4. Launch, detached, from the WT root, prompt on stdin, Sol's final message to a scratch file (NOT to the
   report path — Sol writes the report itself under the workspace-write sandbox):
   `cd "<WT>" && { codex exec -s workspace-write -m gpt-5.6-sol -c 'model_reasoning_effort="high"' --json
   -o "<SCRATCH>/<id>-final.md" - < "<SCRATCH>/<id>-prompt.md" > "<SCRATCH>/<id>.log" 2>&1; echo
   "EXIT:$?" > "<SCRATCH>/<id>.done"; } &`
   Flags are pinned; never add `--full-auto` or any `--dangerously-*` flag. If codex refuses the cwd as an
   untrusted project directory, the ONE global edit you are authorized to make is adding a
   `[projects.'<WT windows path>'] trust_level = "trusted"` entry to `~/.codex/config.toml` for exactly
   this path, then re-launch; say so in your return.
5. Wait, in foreground chunks under the tool cap, re-issued until the marker exists:
   `for i in $(seq 1 16); do [ -f "<SCRATCH>/<id>.done" ] && break; sleep 30; done; { [ -f
   "<SCRATCH>/<id>.done" ] && cat "<SCRATCH>/<id>.done"; } || echo "still running — re-issue waiter"`
   Expect a long run (tens of minutes). If a chunk returns "still running", issue the next chunk immediately.
6. On completion: check the `.done` exit code; `wc -c "<REPORT>"` must be non-zero; `wc -c
   "<SCRATCH>/<id>-final.md"` must be non-zero. If the exit code is nonzero or either file is empty,
   inspect `<SCRATCH>/<id>.log` for the OPERATIONAL cause only (auth expired, quota exceeded, sandbox
   denial, crash) — quote only such error lines, never any review content — and apply the failure handling.
7. Commit what Sol wrote, by pathspec, from the WT: run `git -C "<WT>" status --short`; every new or
   modified path under `Research/quarantine-DO-NOT-READ/` or `.claude/quarantine-DO-NOT-READ/` is staged
   and committed together: `git -C "<WT>" add -- <those paths>` then `git -C "<WT>" commit -m "(AI dsn
   sec) Record the opaque review of <subject>" -- <those paths>`. Any changed path OUTSIDE those two
   trees is NOT committed and NOT reverted: leave it, and list it in your return as an error line. The
   commit-msg hook runs; if it refuses, report the refusal text verbatim as an error and do not retry
   with a different message more than once.
8. Return to the conductor ONLY this, in this order, and nothing else: any prepended error notes (one
   line each), or `no errors`; a line `RESULT (Sol's final message, verbatim):` followed by the exact
   contents of `<SCRATCH>/<id>-final.md` (cat it; do not edit, trim, or interpret it); a line `report:
   <REPORT path> (<byte count> bytes) | commit: <hash or "not committed: <why>">`. Never include a single
   line from the report file, the reviewer prompt, or the codex log beyond operational error lines.

Failure handling (each attempt counts against the budget): transient (network blip, timeout, empty
final file with a clean log) → retry the single invocation once; auth or quota → do NOT retry, return
`FOREIGN-DISPATCH-FAILED: codex — <cause> — human action: re-run `codex login`, or top up the OpenAI
API account`; reads or writes denied despite the grants → return `FOREIGN-DISPATCH-FAILED: codex —
sandbox denied <reads|writes> from the worktree root — human action: dispatch from WSL2/macOS, or
re-run with `-c 'sandbox_permissions=["disk-full-read-access"]'``.
```
