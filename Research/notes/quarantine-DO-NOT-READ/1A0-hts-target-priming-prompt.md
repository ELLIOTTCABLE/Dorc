You're the top-level agent for round 1A, a target-acquisition study — a side-quest off
the main research line (in-parallel agents will be working on the round-20/20
spike in another worktree). Your core job is twofold: understand this project
deeply at the high level and corral your subagents in the herding-cats sense — catching
their errors in cross-cutting judgement — and reach my overall goals effectively. Use
subagents liberally to protect your own context-window: you under-delegate by default,
so correct for it; keep your own window for adjudication, synthesis, and the
judgment-calls below. Your notes go to `Research/notes/1A[0-9A-Z]-descriptive-slug.md`,
append-only, a new numbered note per chunk of work; durable syntheses go to
`Research/plans/1A*-*.md`. Rich logging of what strained and where is a primary
deliverable — the record is the product, not a green checkmark.

The mission, in one breath: take a real-world, brutally-representative ops runbook —
"How To Secure A Linux Server" (H2SaLS), via the Ansible rewrite I use — and turn it
into (a) a careful plain-POSIX-sh rewrite that becomes our standing real-world test
target, and (b) then making a separate, subsequent durable capability-assessment
telling us which hard-to-implement things are also critical to making a real
runbook function end-to-end.

Source material (read-only; ask me, don't guess, if a path is wrong):
- The Ansible rewrite, which includes additional idempotence and guarding, submoduled on my systems at:
  C:/Users/ec/System/Infrastructure/Vendor/HTSALSWA
- The original guide (web, mostly one long README; downloaded and graded locally as [B-imthenachoman-h2sals-2026])
- Site in Research/sources; and use `gh` and Kagi (and fetch, for non-GH sources) for the various involved investigation.

----
SAFETY (this block is non-negotiable and applies for the entire process; copy it
verbatim into the top of every subagent prompt):
- No git mutation outside this workspace; never, ever push. Local commits on this ai/*
  branch are encouraged — commit granularly, with `(AI …)` labels per the repo's style.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't mutate
  global state (no system packages or system config; workspace-local `mise`
  installs/config are fine).
- THE REWRITTEN RUNBOOK IS FROZEN EVIDENCE FROM BIRTH. It reconfigures SSH, firewalls,
  PAM, sysctl, fail2ban, unattended-upgrades: executing it (even partially, even
  "harmless" lines) on this machine could firewall or lock me out of my own hardware.
  It is NEVER executed, by you or any subagent, under any flag or fragment. Validation
  is `dash -n` (parse-only) plus careful reading, plus — for pure shell-logic questions
  only — scratch analogues built from `echo`/`true`/inert stubs under `PATH=mocks-only`,
  never the real artifact, never real command names on a live PATH.
- The Ansible play is likewise read-only reference; never `ansible-playbook` anything.
- Everything follows the repo's DST discipline where code is involved; quarantined
  material under `Research/notes/quarantine-DO-NOT-READ/` stays unread.
----

(The frozen-evidence rule has bitten on this very machine before — a subagent executing
a strawman with live `apt-get` calls. Control your subagents tightly here; this artifact
is the most dangerous-if-run file the repo will contain.)

Orientation, in this order — deliberate; don't dig into Research/ prospectively:
1. README.md, DESIGN.md, IMPLEMENTATION.md, KNOBS.md, TODO.md — human-written, final
   authority (KNOBS is AI-drafted but human-adopted; its welds stand).
2. AGENTS.md — the reading-guide governs everything under Research/; note especially
   the terminology-firming section (book vs oracle; "skip" is banned; fail-fast's local
   meaning), the two-users discipline, and exclusion-checking.
3. The spike's current truth: `Research/plans/20K-round20-take3-report.md` and
   `Research/plans/20U-round20-overnight-addendum.md` (the round-20 close + the
   MVP-wave addendum), then `spike/CLAUDE.md` (the inv-* invariant list and the
   standing human rulings) and `ANALYZER-NEEDS.md`. These tell you what the engine actually models at
   HEAD — load-bearing for your deliverable D4 below. Do not trust my one-line sketches
   over these; verify the modeled subset against the code/corpus where it matters.

GATE — before reading further or touching the target, synthesize back to me, brief and
confidence-marked: the two users and why this artifact sits on the admin side while
demanding engineer-side oracles; book vs oracle; "spelled in sh" and why the rewrite
must NOT invent annotations, YAML, or Dorc-specific idioms; the two phase-keyed
soundnesses (kFAIL) and why a wrong-concrete value is the disaster class; elision as
observable-preserving replacement; and a one-paragraph sketch of what the engine models
today vs what a real runbook will throw at it. Flag anything that doesn't sit right —
pushback is wanted. Wait for my go here; the rest describes your autonomous run.

The job — four deliverables, in dependency order; you own the how:

D1 — the rewrite. A faithful, complete, plain-POSIX-sh rendition of the runbook (the
Ansible play's task sequence as ground truth for ordering and conditionals; the guide
as rationale). Write it as a careful, experienced admin would write idiomatic sh — NOT
shaped to Dorc's currently-modeled subset (measuring that gap is the whole point), and
NOT artificially annotated: the natural guards a real admin writes (`grep -q line file
|| append`, `command -v tool`, `[ -f conf ] &&`) are exactly the metadata Dorc mines,
so write them where a human naturally would and no further. Strictly dash-clean POSIX
(no bashisms); `dash -n` gated; section-structured mirroring the guide with comments
mapping each section back to its guide anchor/URL. Known-hard spots to handle
deliberately and document (a note each, not silently): Ansible handlers/notify have no
sh analogue (linearize restarts as a real admin would — decide inline-after-change vs
end-of-section, once, consistently); `lineinfile`/`blockinfile`/`template` hide
idempotent file-editing semantics that explode into sed/awk/grep idioms — these are
likely your highest-yield strain findings; `when:` conditionals and variables need real
sh spellings. Placement: adjacent to the e2e corpus but OUTSIDE `spike/e2e/cases/`
(run.sh globs that dir; suggest `spike/e2e/corpora/H2SaLS/` — flag the choice to me).

A subagent with a clean context is likely a good fit for the above task;
idiomatic and not-Dorc-targeted are the important parts, as this will become a
conformance-gate for a large body of work.

Then, run a max-capability /adversarial-crosscheck pair across that deliverable. They're to
find *subtle logic errors* in the mapping from README/instructions to
Ansible/POSIX-sh; but not necessarily target security/logic errors in the
upstream source itself - the goal is a quality target for us, not a secure Linux
server.

D2 — the census. A mechanical extraction (write a small awk/grep script over D1; vibes
don't count) of (a) every external command/tool invoked, and (b) every non-trivial sh-language
construct used (functions, command-substitution, pipes, heredocs, redirects, set -e,
traps, case, loops, parameter-expansion forms...), each frequency-counted and
line-cross-referenced. This is the priority-list generator; make it durable & re-runnable over a general sh-corpus.

(Also an opportuntiy for an adversarial crosscheck; maybe slightly less
capable/expensive, as it's just tooling. Use your own judgement, not a
requirement.)

D3 — oracle seeds. For the top state-affecting commands by D2 frequency×criticality
(I expect the likes of: apt/dpkg, ufw, sshd/sshd_config editing, sysctl, systemctl,
useradd/usermod/chage, fail2ban, chmod/chown, the file-edit idioms themselves), seed an
oracle each in the repo's existing idiom — read several `spike/e2e/cases/*/
*.oracle.sh` first as the form's exemplars (check() argparsing its own command +
`oracle_probe_*` declarations; the engine does zero argstring parsing — the oracle owns
its own argv). Each seed must be rich enough to model the state-affecting behaviour
THIS runbook exercises — not necessarily the tool's whole surface. Where an oracle CANNOT model
something (un-probe-able state, inherently non-idempotent steps, cross-command ordering
contracts), that's not a failure to hide — it's a primary finding; record it with the
shape of what WOULD be needed.

D4 — the capability matrix (the durable payoff; goes in `Research/plans/`). Per
D2-item: frequency (this runbook; gut-feel generality is a secondary column, clearly
marked as gut-feel) × criticality (does end-to-end function of the runbook break
without it; is it ordering-load-bearing) × three SEPARATE difficulties — sh-rewrite
difficulty (how awkward was D1 here), oracle difficulty (how hard was/would-be D3), and
engine difficulty (vs the modeled subset at HEAD — cite the specific inv-top-reject
trigger or missing analysis: functions/brk-2, command-substitution/brk-4, etc). Top it
with a ranked yikes-list: the items where high criticality meets high engine-difficulty
— the things we must eventually crack for ANY real runbook to work — each with a
sentence on the cheapest path you can see. Confidence-mark throughout. This matrix will
steer roadmap; for its biggest claims, run a hostile crosscheck pair (high capability; clean contexts,
neutral + adversarial, compare) before you let them into the durable.

Boundaries and balances:
- This is an N-of-1 applicability probe, not market-validation. AGENTS.md's
  market-value-hole warning applies in full: score against THIS artifact only; no
  value-prop essays, no competitive analysis, no "Dorc should pivot" — out of scope.
- Don't fix the engine. Where the engine falls short, that's a matrix row, not a patch.
  (Trivial doc-pointer corrections: flag to me instead.)
- Two-users-gradient lens as a running question: for each guide section, is this book-material
  (target-specific, scrappy) or oracle-library-material (reusable correctness)? The
  boundary you draw is itself a finding — record where it's ambiguous.
- Exclusion-check your dismissals (AGENTS.md): re-test under the other phase, the other
  user, the other reliability before calling anything irrelevant.
- A subagent quietly resolving a cross-cutting judgment-call is a smell — pull those up
  to your level; a sentence in the current note on which way and why beats the artifact.

Process notes, inherited the hard way from prior rounds:
- Dispatch economics: Opus-class for well-scoped production (rewrite sections, oracle
  seeds, census tooling); Fable-class for the hostile crosscheck pairs and any
  adjudication that steers the matrix. Split large tasks; a 490k-token monolith taught
  us that. Verify-don't-relay: never repeat a subagent's claim into a durable without
  tracing it yourself; reserve note-slugs yourself when dispatching parallel work.
- Append-only notes; new numbered note per chunk; the latest note may stay mutable
  until superseded. Confidence-mark (+SURE/~SUSPECT/-GUESS/--WONDER). Greppable
  slug-ids for lists (`yikes-1`, `seed-3`); reuse KNOBS/chord slugs over coining new.
- Commit granularly with `(AI …)` labels per `.gitlabels`; never push; gates before
  commits where code is touched (`dash -n` for the artifact; the spike's cargo gates
  only if you touch the spike, which you shouldn't need to).
- If you get confused in a way better seeding from me would have prevented, note it and
  surface it at close — that meta-record is part of the deliverable.

When the gate has passed, tell me your rough plan-of-attack — brief, confidence-marked,
including your subagent fan-out shape and where you expect the matrix's hardest rows —
and go.
