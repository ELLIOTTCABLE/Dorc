# 20U — Round-20 overnight addendum: the post-close MVP wave

> Companion to `plans/20K` (the round-20 close report). 20K graded the charter; this
> covers everything after it, executed under the standing directive "proceed as far as
> you can; polish toward an MVP-without-network-access." Same hedging discipline; the
> wave's two hostile crosschecks did the grading where it matters. Commits
> `9c9a0a9..a6b562f` on `ai/spike3`; tree at close: 66 e2e cases zero-xfail under all
> six harness gates, ~348 unit/integration tests, fmt/clippy/typos clean.

## §1 What landed, in order

- **task-L1 — loops** (`5122589`, note 20M): for/while/until parse + lower to a real
  cyclic CFG (first back-edges; LoopHead, errexit-exempt conditions, in-loop render
  floor, break/continue ⊤-rejected). Literal-list loops left inv-top-reject's trigger
  set as a deliberate act.
- **task-B3 — word-splitting** (note 20N): POSIX §2.6.5 field-splitting of known
  literals under an IFS-pristine precondition, homed in `syntax::sem`; post-split
  glob-fires/tilde-doesn't rules dash-verified.
- **L1 hostile crosscheck** (note 20O): six findings; the keeper was find-1 — unquoted
  glob/tilde literals as wrong-concretes (`for f in *.conf` binds the literal, dash
  expands per-filesystem; driven to a wrong elision). The class the whole round exists
  to catch: wrong resolution licensing wrong elision, no floor beneath.
- **task-F1 — the L1-find fixes** (`1a9cea2`, note 20Q): the glob/tilde degrade with
  the per-context dash matrix traced precisely (glob does NOT expand on assignment-RHS
  but does at unquoted use; tilde DOES expand on RHS — implemented asymmetrically, as
  dash actually behaves); construct-trailing redirects (`done < file`) loud-⊤ instead
  of silently misparsing into a phantom command; for-wordlist termination dash-faithful
  both directions; condition-position break detection.
- **task-R — render assembler** (`fbda890`, note 20P): all 14 sh-emission sites
  consolidated into `plan::render` with per-emitter `GUARANTEE:` docs; zero golden
  churn; the quoting-bypass audit closed by the compiler (the lone single-quote routes
  through `sem::single_quote` at exactly one call-site); one seeded lint-expect
  self-ratcheted away.
- **task-F2 — scaffolding-safe render** (`60074a5`, note 20R): a Replace'd leaf sharing
  its line with `done`/`do`/`fi`/`then`/`else`/`esac` now substitutes in-situ instead
  of commenting out the keyword (which shipped a dash-broken apply — a mid-run host
  abort, the fail-before-network violation). Pinned refuse-the-license fallback for
  inexpressible shapes: never a broken artifact.
- **tc-group-closer ruled EXTEND** (orchestrator ruling, `e130c22`): subshell/group
  delimiters were the same demonstrated class; one pre-existing golden moved to the
  strictly-more-faithful in-situ form (`( true )`), re-blessed and inspected.
- **task-L2 — loop member-precision** (`2676ad3`, note 20S; the wave's payoff):
  Members-valued for-vars (a side-channel, not a lattice change), per-member check
  evaluation into fact-families, per-member probe records (`site N.M`), and the
  all-or-nothing in-loop license on the self-reach carve-out. `for pkg in nginx curl;
  do apt-get install -y "$pkg"; done` with both converged now elides, run-set provably
  empty — the original 209 brk-1 value case, end-to-end. Preconditions (20O find-6)
  fixed first: while-`$?` body-marking corrected against verified dash behavior;
  in-loop Queries excluded from probe compilation.
- **L2 hostile crosscheck → find-cd-pwd fixed** (`a6b562f`, note 20T): the self-reach
  fixed-point rationale judged sound under sustained attack; the one crack was
  upstream — `cd` rebinds `$PWD`/`$OLDPWD` while modeled as writing nothing, so
  `for PWD in …; do cd /tmp; install "$PWD"` wrongly elided. Fixed in
  `simple_writes_var` (+ `getopts`' implicit `OPTIND`/`OPTARG`, + conservative dynamic
  name-operands); three unit pins including the non-degrade pole.

## §2 The disaster-class ledger for the wave

Four genuine wrong-elision/broken-artifact finds, all fixed same-session: L1-find-1
(glob wrong-concretes), L1-find-2 (scaffolding-eating render → broken apply mid-run),
L1-find-3 (silent misparse of the most idiomatic loop shape), find-cd-pwd (implicit
var-write blind spot in the member override). Catch-source distribution, stated
honestly: hostile crosschecks 4, builder-in-flight discovery 1 (the BLESS
contamination, §3), pre-existing tests 0. The pattern from 20K §5 holds: the corpus
pins what we know; the crosschecks find what we don't. ~SUSPECT the next round should
budget crosschecks as a fixed fraction of build spend, not an afterthought (this wave:
two Fable passes ≈ 410k tokens bought all four priority-1s).

## §3 Process incidents (both now codified)

- **BLESS contamination** (caught live by task-F2): concurrent agents share one
  `spike/target/`; `BLESS=1` re-blesses ALL cases from whatever binary exists at that
  instant — a sibling's mid-flight buggy binary got baked into a golden, caught only
  because the builder diffed its own untouched cases. Now a standing rule in
  `spike/CLAUDE.md`: BLESS is orchestrator-only, never with agents in flight, diff
  inspected case-by-case.
- **SyncThing ghost-resurrection**: another device pushed back `*.sync-conflict-*`
  copies of a case dir task-L1 had legitimately deleted, leaving a husk directory that
  broke the harness ("dead engine" guard fired, correctly). Cleaned; content fully
  git-recoverable. fb-5 for the human: a stale copy of this worktree path exists on at
  least one other synced device — either exclude `.claude/worktrees/` from SyncThing or
  expect this class to recur (the conflict-file timestamps make cleanup mechanical, but
  an agent-run mid-resurrection wastes a debugging cycle).

## §4 Orchestrator rulings made under autonomy (review wanted)

- **tc-group-closer → EXTEND** (one golden re-blessed; the diff is strictly more
  faithful). Risk accepted: widening a just-landed mechanism same-day, judged low
  because the construct's spans include both delimiters (no loop-style span caveat).
- **tc-l2-singleton-member-family** (builder-flagged, I let it stand): a single-word
  `for f in nginx` loop now elides when converged — a deliberate behavior change from
  L1's blanket floor. ~SUSPECT right (it's just the N=1 member family); flagged because
  it changes a 20M-documented behavior.
- **find-cd-pwd fix-shape**: unit-pins only, no e2e case — the corpus pins
  user-realistic shapes and `for PWD in` is perverse; the eligibility refusal is the
  entire fix surface. If you disagree, the e2e case is five minutes' work.
- **20O find-2 deferral sequencing**: held the scaffolding fix one slice so it landed
  on task-R's consolidated assembler instead of code about to be refactored. Worked
  cleanly (F2 reused R's emitter seam); recording it because deferring a demonstrated
  artifact-breaker even one slice is a judgment call with a real window of exposure.

## §5 Dispatch economics, updated from 20K

- Opus builds: F1 330k / F2 336k / R 204k / L2 538k tokens. L2 broke the 20K "split
  large tasks" heuristic and succeeded anyway — the refinement is that token-size
  predicts failure only when the agent owns the DESIGN; L2's design was fully
  pre-spelled in the brief (the license's four conjuncts, the rationale to preserve,
  every ambiguity pre-resolved to REFUSE), so its size was mechanical surface, not
  judgment surface. Restated heuristic: split tasks that would make an agent decide
  cross-cutting things, not tasks that are merely big.
- Fable crosschecks: 205k + 205k, consistently the wave's highest value-per-token
  (every priority-1). The hostile-identity briefing ("you believe there is a hole; a
  clean bill requires having genuinely tried") plus mandatory engine-vs-dash
  construction discipline is doing the work; keep both.
- Two agents on disjoint surfaces ran concurrently without conflict (F1×R, F2×F1)
  EXCEPT through the shared `target/` (§3) and the e2e tree (F1 observed F2's cases
  mid-flight as count flux). Disjointness must include build artifacts and goldens,
  not just source files.

## §6 Round-21 inheritance (my recommendation, in priority order)

1. **Leaf-exact render** — the one design choice that has genuinely fought us: three
   carve-out waves (T14 case-arms, F2 scaffolding, group-closer) all exist because
   "the line" is the wrong substitution unit, and `Channel::StatusRenderFloor` exists
   ONLY because an if-guard can't be substituted in-situ. task-R conveniently fenced
   the whole emission surface into one module; a span-based rebuild retires the
   carve-out family AND the StatusRenderFloor channel. -GUESS task-sized, not
   round-sized.
2. **brk-2 function inlining** (budget-bounded) — unlocks the 207 wrapper-pun
   direction and is the biggest modeled-subset gap a real book hits (oracle helper
   functions are currently Opaque poison).
3. **Partial-member list-rewriting** (tc-l2-member-list-not-rewritten): the deferred
   half of L2's value — one diverged member currently runs ALL members.
4. **hostsim DST at scale** — the seeded crate is the highest-leverage unexercised
   asset given the whack-a-mole prognosis (20A): seeded-random book/oracle generation
   against the differential gate-5 harness.
5. **cm-1 reassessment** + the 20O find-6 latents (they unfreeze with any further
   floor-lifting), + the charter-level tension 20K flagged: this spike has drifted
   exploratory→MVP; round-21 should pick consciously.

## §7 Standing human-pending ledger (unchanged through the wave)

tc-query-bare-elision · tc-perselector-wrapper-scheme · root-`KNOBS.md` kELISION fix
(edited at the repo root per direction, deliberately uncommitted, awaiting review) ·
the 207 errexit/YOLO tension (genuine design fork; YOLO-mode framing drafted) · st-2
one-vs-two-declarations at dq-kOOB · plus §4's four autonomy-rulings.
