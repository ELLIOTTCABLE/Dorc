> QUARANTINED — conductor-private adjudication ledger for the 26xxx crosscheck (Fable
> conductor = the plans' author; deflationary credal stance per
> crosscheck-adjudication-skepticism). Accretes per-lane as reports land; superseded by the
> final adjudication note. Raw lane reports live in the session transcript; do not cite
> this ledger as authority.

# 26xxx adjudication ledger

Target: plans/260+261+262 @ pin 361a57f. Lanes: fable-adversarial (running; pin-amendment
queued) · codex-neutral v2 (re-running) · codex-adversarial v2 (re-running) ·
deepseek-neutral (IN) · deepseek-adversarial (IN). Fable-neutral held by human.

Dispatch friction so far: both codex lanes fail-fasted on environment false alarms (benign
HOME git-config warning; literal `Test-Path plans/260` shorthand miss; pin-ahead after my
packet commits; one transient ACL blip) — v2 packets pre-clear all four (cc1909a). Two shim
stalls (backgrounded foreign run + ended turn) un-stalled via resume-with-foreground-wait.

## deepseek-adversarial — distilled + provisional dispositions

Meta "citation laundering" + F1 CRITICAL "22H cited as settled law but never built":
PARTIAL-MISREAD with a REAL KERNEL. The plans do state the fleet kernel is new-build (260
§2, §8 stage ladder builds the accumulators/seam 22H §2 enumerates). What IS consumed as
settled: 22H §3's fold-reuse facts (static-half purity; merge commutativity) — 22H marks
those +SURE/"survives". BUT (a) the plans' "settled law/substrate" register invites exactly
this misreading by a builder; (b) 22H's own header warns it is "likely UNDER-scoped" and
the plans nowhere carry that sizing warning. DISPOSITION: doc-fix — 260 §1/§2 states
plainly "the 22H engine does NOT exist; this round builds it; what's consumed is its
analysis, re-verified at stage-26-0 (merge_observable commutativity against current code)";
carry the under-scoped warning into sizing.

F2 CRITICAL "wire v1 = arch-multiplex-inband, regression from 142, must be replaced not
extended": PARTIAL — the strongest adversarial item. Right: v1 IS an in-band instance
(142's table: pristine=no); freeform CAN leak onto the records channel (sloppy oracle
bodies), so 142's structural-security property (signalling never shares a lane with
freeform) is genuinely absent at v1 — nonce-framing is the "backup" mechanism 142 itself
ranked secondary. My "nothing below contradicts it" overreaches at the transport layer
(grammar/kernel survive; emission+session topology is replaced in the growth move).
Wrong: "irrecoverable/rebuild-the-stack" — the growth costs were always the 142 endgame's
own costs; v1 creates the parser/grammar/kernel that persist. DISPOSITION: doc-fix, not
design-reversal — dec-26-wire-v1 re-labeled honestly as the degraded-start instance of
142's arch table (which 142 sanctions as fallback), name the migration steps + the
security posture gap explicitly; the executorless single-channel v1 stands for v1 scale.

F3 HIGH "order-free invariant contradicted by s3-3 oscillation": REJECT AS STATED —
conflates intermediate live-render states with final content. Merge is commutative:
disagreeing facts → ⊤ regardless of order; terminal plan is order-independent (the
invariant's actual claim); consent/apply happen post-fold-complete. ACCEPT a doc-fix: 262
§1 scopes the invariant to "final, fold-complete content" and names the intermediate-view
oscillation (s3-3) as outside it.

F4 HIGH "antichain-by-accident; scheduler ships on faith": CONVERGES with ds-neutral fd3.
ACCEPT — see fd3 disposition (the strongest cross-lane item so far).

F5 HIGH "DST never crosses the sim/reality boundary": PARTIAL, mostly KNOWN-PRICED (128's
mocked-edge doctrine; fc-3 real-transport = slow high tier). FALSE detail: "zero tests
cross it" — the gated ssh-localhost smoke exists (260 §7). ACCEPT: fatten the gated tier's
charter (real sshd; MaxStartups behavior; the EOF-no-rc cell; half-written-line kill) as a
named non-hermetic tier; acknowledge the boundary-bug class in §7 prose.

F6 HIGH "re-probe reads state not history; partial-apply recovery honesty gap": ACCEPT AS
HONESTY FIX (known-adjacent: the adequacy gap restated for recovery). s3-4 + dec-26-apply-
visibility gain a plain paragraph: re-probe localizes STATE; for effects no oracle
witnesses, Unknown persists and the operator decides; non-idempotent re-run risk = the
adequacy bound, not dissolved by the retry-file dividend. No mechanism change (per-leaf
markers stay with faithful-mode).

F7 MED ssh-config: MIXED. The Match-block-overrides--o claim is ~SUSPECT WRONG (ssh
first-obtained-wins; command-line read first) — verify cheaply, else reject. ACCEPT two
nits: run-dir (and ControlPath) mode-0700; document the fleet host-key bootstrap flow
(BatchMode + unseeded known_hosts fails loud → the accept-new flag or pre-seed guidance).

F8 MED "emission locus = the merge conflict point; 22H warned": NOTE-ONLY. The 22H warning
(don't build tape/reactivity against single-shot) is being heeded — r26 builds the stream
engine. Locus conflict is priced (flag-gated, bounded; goldens churn anyway per human).

F9 MED "additive-keys without negotiation infra": REJECT mostly — v1 emitter and parser
ship in the same binary (the artifact is compiled by the process that parses its records);
no cross-version wire exists; skew arises only for persisted artifacts re-run later, out of
scope by rec-1/transient-plan posture. ACCEPT one sentence stating that same-binary
property as the policy's justification.

F10 MED "touches()/deriv-probes don't exist": REJECT — factually wrong on this branch
(Stage 4/5 landed 2026-07-04; `deriv N coord=` records are in the goldens at the pin). The
r24 queue item is the typed-emission RESPELL of touches(), not the mechanism.

Withdrawn set (their own kills): kAGENTLESS-violation, HashMap-determinism, kSTATE/rec-5
violation, security-silence — all correctly withdrawn; notes that the package's fences
held under adversarial reading.

## deepseek-neutral — distilled + provisional dispositions

fd1 HIGH truncation "range" vs order-free records: ACCEPT (crisp, real). seen<sites gives
a COUNT; the missing SET needs the compile-time site census. Fix: state the coupling — the
fleet kernel receives the per-artifact site census from compile (it compiled the artifact);
missing-set = census \ received; kill the "boundary" language (it's a set, not a range).
Optionally sites= stays a count (census travels controller-side, not on the wire).

fd2 HIGH wedged un-timed task blocks its wave ⇒ all later waves lost to whole-artifact
timeout: ACCEPT CORE. Their "zero results arrive" sub-claim is wrong against my stated
partial-keep semantics (streamed records survive; loss = the blocked wave's stragglers +
all later waves) — still a real magnitude bug. Fix: per-task `timeout` for ALL classes
when the binary exists (drop the class-gating); document the untimed-fallback loss shape;
consider trap-on-artifact-timeout emitting the sentinel with seen-so-far (turns kill into
clean probed-partial).

fd3 HIGH build the h1-edge detection pass NOW even though it detects zero edges (+
synthetic-edge wiring test): ACCEPT — converges with ds-adv F4. Amendment to 261 P0/262
S0: the edge-extraction pass is a real compile step from day one; property pin = current
corpus yields zero edges; a synthetic injected edge must flow compiler→schedule→wave-split.
This turns antichain-by-accident into antichain-by-proof, which IS the plan's own stated
contract ("the compiler proves membership at emission") made real.

fd4 MED open-cap head-of-line starvation: ACCEPT, and the right fix is sharper than
theirs — the global open-cap=8 was wrong-SHAPED: MaxStartups is per-target-sshd; with one
connection per host it never binds. Redesign: per-target open pacing (default ~2; rarely
relevant) + the global width-cap governs concurrency; slow-connect hosts then can't starve
fast ones. (My cap was cargo-culted from many-connections-to-one-target fan-out lore.)

fd5 MED EOF-without-exit-status cell undefined: ACCEPT — add the cell to s3-4 (treat as
transport-loss ⇒ UnknownAfterLoss); heuristic table documented as diagnosis-only.

fd6 MED "round-robin-greedy" ≠ the class-spread claim: ACCEPT — my wording error; LPT
proper = assign-to-least-loaded-lane. One-word fix in 261 §3.

fd7 MED sites= per-book vs future per-host divergence: ACCEPT one line — document sites=
as "sites in THIS artifact" so semantics survive per-host specialization (future seam).

fd8 LOW no record-length guard vs PIPE_BUF: ACCEPT — emit-time length check; over-long
record truncates-with-marker ⇒ can't-tell (long deriv file: coords are the live case).

fd9 LOW cross-tier (fleet interleave × width>1 jitter) e2e case: ACCEPT (one case).

fd10 LOW fleet-wide re-probe to recover one host: ACCEPT AS WORDING — single-host re-probe
already exists (`dorc plan book.sh -H X`); recovery text points at it.

Their holds-up list (sans-io kernel; partition-law-unrepresentable; the invariant + pins;
kFAIL phase-keying at transport; additive-keys; never-assume-converged taxonomy;
ssh-subprocess call; the acceptance-pin set incl. heeding 22H §4's vacuity warning) — noted;
matches the package's own claimed strengths (weigh accordingly: same-corpus flattery risk).

## codex-adversarial (v2 run, complete; 51 cmds clean) — distilled + dispositions

fd1 HIGH in-band regression "property changed from structural separation to parser
rejection; cites the OOB law as if preserved": ACCEPT — converges with ds-adv F2 (now both
adversarial lanes, two lineages). Codex's phrasing IS the fix: dec-26-wire-v1 re-labeled as
the sanctioned 142-fallback instance; law-transport-shape's "v1 instantiates this narrowly"
softened to name the property change explicitly; migration steps enumerated. Design stands
for v1 scale; the CLAIM was dishonest as written.

fd2 HIGH "h1-by-later-wave is mechanism-free — values return to the CONTROLLER; a later
wave in the same shipped artifact cannot consume them": ACCEPT — THE sharpest finding of
the exercise, a real composition error in 261 §2. Verified by inspection: probe-readback
values land controller-side via records; wave ordering gives an on-host consumer nothing.
h1 edges have exactly TWO real resolutions: (a) in-artifact composition — the connected-unit
pattern generalized (producer's value captured host-locally, fed to the consumer inside one
compiled unit, 24J's shape); (b) controller-fold consumption (readback resolves plan-side
relevance/values; never feeds another on-host probe's argv in the same shipment). The
"consumer runs in a later wave" option is DELETED as a mechanism; waves exist for
width/pacing only. Rewrite 261 §2 h1 + §3 accordingly; the h1-edge extraction pass (2-lane
amendment below) detects edges to COMPILE-INTO-UNITS, not to wave-order.

fd3 MED truncation contract under-specified (multiple record families site/deriv/resolv/
reach + N.M sub-keys; counting records ≠ counting sites; the acceptance pin can go green
without proving its claim): ACCEPT — converges with ds-n fd1, upgrading it from wording-fix
to a real §2 redesign: census lives controller-side (the compiler's site table); a site is
received iff its terminal `site N` record arrived; supplementary families (deriv/resolv/
reach) are individually-optional degradation, not Unknown-triggers; `seen=` semantics
redefined or dropped; the pin restated against the census model. The one MEDIUM
design-level fix (everything else is doc/wording).

fd4 MED spine-inv-order-free lists "artifact" among protected content while width changes
artifact bytes; K=4 (261) vs width=1 (262 golden posture) defaults disagree: ACCEPT —
drafting incoherence, mine. Fix: invariant restated as "for a fixed compiled artifact,
runtime interleaving changes nothing; across width settings, record-SET and final plan are
identical while artifact bytes legitimately differ"; one defaults table (flag default 1
until the P4 flip package; product target 4).

fd5 MED h3 contention postponed past the widening it ships with (same daemon/dpkg/DNS
herded at width>1): ACCEPT-PARTIAL — pull h3-lite into P2: once the classifier exists,
use class-as-resource-key (daemon-class concurrency capped ~1 per wave); full resource-key
plumbing stays deferred; note the bounded exposure at K=4 honestly.

Withdrawals (sans-io kernel, per-host independence, no-apply-retry, ForwardAgent-default):
all correctly held. NB codex verified the incumbent parser's missing-site⇒Unknown fail-safe
at cli/main.rs:2989 unprompted — the verify-the-incumbent behavior emerged without the
carve-in.

## Cross-lane convergence tally (3/5 lanes complete: ds-n, ds-adv, codex-adv)

- h1-edge proof pass built-now (ds-adv F4 + ds-n fd3): 2 lanes, both stances — STRONGEST.
- Wire section weak zone (ds-adv F2 + ds-n fd1/fd8): overlapping cluster, different angles.
- Apply-loss honesty/completeness (ds-adv F6 + ds-n fd5/fd10): adjacent cluster.
- Everything else single-lane so far.

## Pending

Codex v2 ×2 + fable-adversarial reports; then the five-lane synthesis (fable-neutral held
unless human go). Verify-before-crediting still owed on: ssh Match-block precedence claim
(F7); merge_observable commutativity against current code (F1 disposition's re-verify).

## Lane-state update

codex-neutral is DEAD: the v3/final retry failed at turn start on OpenAI quota exhaustion
("Quota exceeded", exit 1, zero output) — plausibly self-inflicted (three prior half-runs,
the last ~471k input tokens). Recoverable only by human action (codex login / billing
top-up). Synthesis proceeds on four lanes (ds-n, ds-adv, codex-adv, fable-adv) unless the
human revives it; the codex stance-coverage is then adversarial-only, same as fable.
