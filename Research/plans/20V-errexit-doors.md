# 20V — errexit-elision recovery: the doors program

> Distilled from the orchestrator⇄human discussion at round-20 close (post-20U).
> Design-direction grade: the build-consequences are encoded in the round-21 charter,
> but the UX-shape adjudication CONTINUES IN PARALLEL with round-21 and rulings may
> land mid-round — each item below is marked RULED / LEANING / OPEN accordingly.
> Context: YOLO-mode (207) is set aside by the human as an escape-hatch, not pursued;
> this program is "best-effort under correctness," pursued harder.

## §1 The blocker, as five interlocking welds (all individually correct)

weld-1 (C-3, ruled 19A): under `set -e` every command's rc is consumed — errexit is an
ordinary status-consumer. weld-2 (inv-one-observable): a replacement reproduces every
consumed channel's predicted value. weld-3 (definitional, HARD): probes never run
mutators, so probe-provenance for a mutator's rc cannot exist. weld-4
(fork-mutator-rc, chosen-SOFT): a mutator's rc has no *sanctioned* source — a
default-deny whose un-welding hinge is the reserve clause already in
inv-probe-sourced-values ("…or an oracle-declared fact the human has explicitly
sanctioned — none currently exist"). weld-5 (no fabricated values): the stand-in may
not mint rc=0. Conjunction: bare mutators under `set -e` never elide. Why weld-5
stands: `useradd alice` on a converged host exits 9; `mkdir` exits 1 — "converged ⇒
the re-run exits 0" is tool-specific knowledge, false as a law.

## §2 The canary reframe (the load-bearing observation)

On a converged host the mutation is a no-op (probed); the consumed rc's only remaining
job is to crash the book if the *environment* is sick (lock held, disk full, dpkg db
corrupt). Under `set -e`, a converged mutator IS an environment-health canary.
Errexit-elision is therefore canary-removal — an INTENT question (is this crash
load-bearing for this admin?), unanswerable by any analysis, only assignable to a
deciding party. dq-errexit-1 (OPEN): is canary crash-fidelity the ONLY cost-species of
converged-mutator elision? Everything downstream re-opens if a second species exists.

## §3 Provenance taxonomy for a consumed mutator-rc (claimed complete)

probe-observed (impossible for mutators, definitionally — waiting on better analysis
is NOT an option) · already-guarded (door-1) · dead (door-3) · declared (door-2) ·
constructed (door-4). Each arm is the negation of the others; no sixth arm.

## §4 The doors

- **door-1 — guard folds (EXISTS; extend reach).** A probe-observed Query rc folds its
  guarded construct: `grep -qx 'PermitRootLogin no' f || { sed -i …; systemctl restart
  sshd; }` — probe says grep rc=0 ⇒ the whole block is dead control-flow ⇒ fold to a
  stand-in reproducing the MEASURED 0. Cascades: the sed AND the restart elide as
  unreachable, needing no rc-provenance of their own — the Ansible handler/notify
  semantic falls out of plain control-flow analysis. brk-2 inlining extends this to
  oracle-shipped wrapper functions (`apt_install() { dpkg -s "$1" || apt-get install
  -y "$1"; }`) — elision provable from the sh, zero new trust.
- **door-3 — rc-deadness (BUILD; free, zero new trust).** `cmd || true`: errexit never
  sees the left rc (exempt context); the `||` reads it but both continuations are
  observably identical ⇒ consumed-in-form, dead-in-fact ⇒ a ⊤ prediction is harmless,
  any stand-in rc is faithful. This is the admin's own spelled-in-sh "not
  load-bearing" — and at HEAD it is refused (⊤ + StatusRelaxable ⇒ block): we
  currently refuse the one line-shape whose author explicitly opted out. A Status-
  channel refinement ("consumed-but-invariant"), not a policy change. Generalization
  (don't over-build): any consumer whose branches rejoin immediately with identical
  observables.
- **door-2 — declared converged-run (DESIGN + BUILD behind seam).** The oracle
  declares the converged-run observable as tool-knowledge — sh spelling ≈ a vouched
  body the stand-in becomes (`oracle_converged_run_package_install() { printf '%s is
  already the newest version.\n' "$1"; return 0; }`). This sanctions weld-4's reserved
  slot. It is the system's FIRST COUNTERFACTUAL claim-type and must not be conflated
  with check-claims: a check reads the present world (probe-able, fresh); this
  predicts an unexecuted run's behavior (version-sensitive, stale-able, and
  disaster-class-shaped when wrong). Static: trusts the probe lane's freshness
  (TOCTOU-WONTFIX parity with today's plain elision). Spelling is kTYANNOT-adjacent:
  acceptable-debt inline form for the spike; ratification OPEN.
- **door-4 — the constructed guard (BUILD; the keystone).** Transform a bare oracled
  mutator: `apt-get install -y nginx` ⇒ `dpkg -s nginx >/dev/null 2>&1 || apt-get
  install -y nginx` — the left side is the kind's own structurally-vouched probe body.
  Four-world trace: converged∧healthy ⇒ list-rc 0 with LIVE provenance (re-measured at
  apply time); diverged-since-probe ⇒ the real mutator runs — kFAIL-perform BY
  CONSTRUCTION, strictly better than static elision under TOCTOU drift;
  converged∧env-sick ⇒ the canary is suppressed (the §2 trade, same residue as
  door-2, no worse); lying-check ⇒ under-execute, but that is the PRE-EXISTING root
  trust every elision already rests on — unwidened. Perf honesty: not elision-to-true;
  it replaces a slow remote no-op (apt's seconds of lock+resolver) with a millisecond
  read — under network-dominance that is most of the available win, in-line and
  order-sacred. Crucially NOT an elision under the current license vocabulary: a NEW
  license category (guard-insertion), mintable only when the kind's oracle declares
  converged-run-equivalence (door-2's declaration) AND the site's non-Status channels
  pass the existing consumption gates. Keeping it a separate category is what stops it
  eroding weld-5. The artifact diff is idiomatic sh the admin could have written —
  meaningful after abandoning Dorc. Rides arch-1's span-render; door-1-via-wrappers
  (post-brk-2) validates its semantics analytically before declarations are trusted
  for bare-style books. dq-errexit-3 (OPEN): guard-insertion runs oracle-authored
  commands at sites the book didn't spell — same trust as probes-run-oracle-code
  extended to apply, or a line crossed?
- **Dead doors (recorded so nobody re-derives them):** probe-runs-the-mutator
  (definitionally void); `set +e`-as-relax-signal (anti-door: spelled in sh but means
  the opposite — pursuing it teaches admins to weaken their books).

## §5 The precedence proposal (LEANING; dq-errexit-2 OPEN)

**admin-explicit > oracle-default > engine-conservative.** An explicit handler
(`|| { …; exit 1; }`, a `$?`-read, any observable continuation) marks the rc live —
doors 2/4 refuse; no declaration overrides the admin's own sh. An explicit `|| true`
is door-3 — free. The bare middle takes the KIND's declared default (door-4/2 if the
oracle declared, else runs) — the engineer answers the intent question tool-by-tool;
the admin retains per-line override in BOTH directions using ordinary idioms that
mean the same thing without Dorc. Degradation proportional to omission; sharpening
in-language; no YAML. dq-errexit-2 (OPEN, the central UX fork): may the
*oracle-author* own the bare-middle default, vs engine-global (YOLO's territory) or
admin-per-book (smells like configuration-not-code)? Disclosure floor regardless:
elided-under-declaration sites are recorded (verdict lane + artifact comments) so
post-hoc failure attribution can say "line 14 elided per package-oracle's
converged-claim."

## §6 The effort-rung gradient (the answer-shape to "how do we degrade gracefully")

r-0 bare + un-oracled ⇒ runs, full stop · r-1 admin writes `|| true` ⇒ door-3, free ·
r-2 oracle ships check+probe ⇒ door-1 folds guarded blocks, and (post-brk-2) wrapper
calls — zero new trust · r-3 oracle adds the converged-run declaration ⇒ bare lines
guard-transform or statically elide (doors 4/2), disclosed · r-4 admin adopts the
oracle's wrappers ⇒ r-3's behavior provable-from-sh, no trusted declaration · r-5
scoped-mutation probes — vouched mutation-boundaries for tools with no non-mutating
mode (the jank-webapp class): a SECOND new claim-type, larger trust-surface than r-3
(a boundary the engine can't verify, on the kFAIL-withhold side), explicitly NOT
round-21; needs its own design round. Each rung: bounded human effort buying
disclosed, bounded elision; no cliffs between rungs.

## §7 Consequence for the north star

With doors 1+3+4: the reachable elision ceiling on an errexit book stops being
guard-idiom density (a property of the book we must not game) and becomes
**oracle coverage × declaration coverage** — the dependency the thesis wants (value
scales with the engineer-side library; books stay plain sh). The honest residue: the
canary trade is real and irreducible; zero-behavioral-delta is available only as
doors 1+3, and that product is the lame one. The dashboard must therefore attribute
every elision to its door (fold / dead / transform / static-declared) so the 80%
question decomposes into measurable, separately-ownable terms.

## §8 Open ledger (the human's; do not settle in-round)

dq-errexit-1 (canary-only cost-species?) OPEN · dq-errexit-2 (oracle owns bare-middle
default?) LEANING-yes, unratified · dq-errexit-3 (guard-insertion's trust status)
OPEN · the declaration's concrete sh spelling OPEN (inline acceptable-debt form
sanctioned for the spike only) · r-5 scoped-mutation contract FUTURE. Mechanism built
in-round goes behind policy-seams (config-in-code, one module) so mid-round rulings
hot-swap without rework.
