# 24S — Wrapper contexts: peel, the context algebra, and the evaluation environment (PROPOSAL)

AI-authored (Fable, research/planning session), 2026-07-10. Proposal-tier: **provisionally
acked for spike experimentation** (human, 2026-07-10 — ack given explicitly *for*
hands-on-experimentation purposes, not as settled design; the human states he will likely
not understand this work until he tries it). Prime target for adversarial analysis.
Synthesis of the `opaque-string-analysis-ceiling` research round (its context/transform
half; the payload half is BANKED, §10). Evidence base:
`.claude/research/opaque-string-analysis-ceiling/` — turn notes `turn01`–`turn07`;
21 graded sources in its `sources.json` (bracketed `[X-slug-year]` citations in this
document resolve there). Finding-slugs (`opaquesN-…`) resolve to those turn notes; they
are cited for traceability, not required reading. Plan-tier per Research/README: durable
synthesis, lightly kept-current, annotate corrections in place.

Reading rules: root docs and human rulings outrank this. Terminology per the r24 language
rulings (bare munged `__role` names; reverse-DNS kinds; the five per-line mechanisms —
elide/replace/guard/omit/descope — per KNOBS "Named mechanisms"). Every sh spelling below
is STRAWMAN: the *shapes* are proposed, no syllable is. Certainty markers used throughout.

---

## §0 — Problem, scope, and the impossibility ledger (read first)

**The problem.** Bounce/wrapper commands — `sudo CMD…`, `su -c 'STR'`, `env VAR=x CMD…`,
`chroot DIR CMD…`, `timeout N CMD…` — are, at HEAD, ordinary un-oracled commands: Opaque,
run-every-apply, total poison-walls. This is the corpus's #1 named capability gap
(`24R` flag-sudo-weight: both secondary-position halves promoted it independently; the
r25 field trial's `su - postgres -c '…'` was a *permanent* wall on the standard Debian
idiom; USER_STORY concedes "sudo lines still wall (honestly)"; personal machines sudo
per-line; the lint ecosystem actively *teaches* `sudo sh -c` as the blessed repair idiom
[A-shellcheck-sc2024-wiki-2026] [B-shellcheck-sc2232-wiki-2026], so "don't write that"
is not available). The seam was reserved long ago (17N §7's wrapper-context dq; 23J
lane-privilege; an-privilege-fact) but never designed. This document is the design.

**Impossibilities and hard fences, frontloaded** (if a later section seems to contradict
this ledger, the ledger wins):

- imp-1 **Probes never escalate.** The engine knows privilege, never acquires it
  (an-privilege-fact; kFAIL-withhold). Consequence: state readable only *inside* a
  context (peer-auth-gated reads, root-only files) is plan-time unknowable, forever;
  such sites cap at run-with-guard, honestly (§4, opaques6-finding2).
  **[RE-SCOPED — 2026-07-16, `notes/27C`, human-ruled: the defensible core is
  reuse-NEVER-ACQUIRE — the probe lane may re-use authority the connection already
  holds to execute tolerance-vouched oracle bodies in the site's denoted context,
  under the ternary escalation dial (`27C` §2). "Never acquires" stands (no prompts,
  no credentials; acquisition is a separate opt-in cell). The "plan-time unknowable,
  forever" consequence now applies only where the dial/vouch/entry bar is unmet.
  kFAIL-withhold (mutation) was never about privilege; the conflation is the corrected
  error.]**
- imp-2 **HEAD behavior is the floor, and silence never peels.** An un-oracled wrapper
  head does not peel — `plab CMD` (the round's durable strawman for a user's in-house
  wrapper) behaves byte-identically to HEAD: runs, walls. No engine default identifies
  anything across any boundary. There are NO unsafe defaults anywhere in this design
  (opaques6-finding9-floor-is-head).
- imp-3 **The vocabulary-gap residual is Dorc's-fault horizon.** A wrapper that moves a
  context dimension neither the engine's axis vocabulary nor its author knows about
  produces failures attributable to no human line — IMPLEMENTATION's
  errors-we-can't-attribute-are-our-fault horizon. Bounded (§3a) but not eliminable
  (opaques6-finding16).
- imp-4 **Irreducibly-dynamic payloads stay walls.** Loop-assembled command-strings and
  unknown-splices-in-syntax-position are cross-literature permanent cliffs; hole-parsing
  is unbuilt in ALL literature ([A-arceri-dynamic-code-2021]'s 47%-opaque result and
  sketched-only fix; opaques3-finding12). Out of scope here (banked payload frame, §10).
- imp-5 **Host is someday-maybe-never.** `ssh host 'STR'` decomposition violates the
  standing no-cross-host-facts/ordering/guarantees weld (human ruling,
  ruling-host-someday-never). Listed in the axis census for totality only.
- imp-6 **Execution-partiality is a non-axis.** "All commands are inherently partial
  from our perspective… any command can crash at any time. There are no promises in
  ops" (human ruling, ruling-partiality-dissolved). `timeout` gets no special
  machinery; whatever the claim algebra does about crash-mid-run covers it.
- imp-7 **The apply artifact is untouched** (§6d): user lines verbatim, always. This
  entire design lives in analysis + the probe lane.

**Why now / sequencing pressure:** the r24 queue-3b entity-algebra rebuild
(rul24-selector-pre-stdlib) is churning the coordinate representation NOW and should
reserve this design's two seams (§7 adjudication item A7) — cheap to reserve, expensive
to retrofit (opaques5-finding7).

---

## §1 — The feature at a glance

One sentence: **wrapper commands become decomposable — the engine peels the nested
command out, analyzes it under a transformed evaluation environment, keys its facts into
context-qualified coordinates, and composes verdicts across the boundary only where an
attributable declaration licenses it.**

Two orthogonal components (opaques5-finding2, the round's central division; each has a
different consumer and neither reduces to the other):

1. **The evaluation environment, ρ** — the var-state {env-vars, positionals, cwd} under
   which the nested command and its oracles *evaluate*. Acts at evaluation time
   (VarState → VarState, an interpreter environment, before any cell is named).
   Consumers: the nested code's value plane; the nested tool's oracle bodies when traced;
   the shipped probe (via replication, §6b). Pipeline: ρ → values → argv/paths → cell
   names.
2. **The context/coordinate algebra** — per-axis, per-kind identification of cells
   *across* the boundary. Acts at comparison time (Cell ↔ Cell, an equivalence on the
   store, after names are computed). Consumers: the elide-weld (does this vouch's yes
   speak of the site's cell?), footprint×backing intersections, disjointness/survival.

The algebra decomposes as **axes(wrapper-declared) × sensitivity(kind-declared)**
(opaques5-proposal1 / opaques5-finding1):

- A **context** is a point moved along a small, engine-owned, *versioned-closed* axis
  vocabulary (§3a): {user, fs-view, host(reserved-never)} on the coordinate side, plus ρ
  on the environment side. A wrapper's oracle declares which axes its tail moves and to
  what (`sudo` moves user→root-or-`-u`-value and transforms ρ; `chroot` moves fs-view;
  `nice` moves nothing).
- Whether a moved axis *changes a coordinate's referent* is **kind** knowledge, declared
  per-axis by the kind-owner as a **trichotomy** (opaques6-finding1):
  `invariant` (same cell across the move — buys identity-bridges AND probe-outside
  licenses), `sensitive` (provably-partitioned cells per axis-value — buys disjointness
  and correct per-value keying), `sensitive-with-map` (same-referent-under-rename — an
  oracle-authored map, resolve()'s sibling), or **silence** (may-alias: no
  identification in EITHER direction — no sameness for establishment-flow, no
  disjointness for survival; walls; safe; costs nothing).
- **Composition:** wrapper nesting composes axis-moves; a site's verdict is the weakest
  link in its chain; decomposition recurses with a nesting bound (practice: resholve
  recursive since 0.6.0 [A-resholve-manpage-2023]; literature: Unevalizer k=1 sufficed
  [A-unevalizer-2012]). A wrapper line is TWO separately-tracked nodes — the wrapper
  node (own footprint, own observables) and the peeled inner node — union-modulo-
  structure, pipe-like (human steer; the `$()`/24E §14 shape).

The floor of every piece is HEAD-identical; every declaration is a monotonic, per-axis,
per-kind, per-wrapper opt-in; every cross-boundary verdict cites a four-link attribution
chain (§4a).

---

## §2 — The feature from each chair (the user-story view)

Worked example throughout (the round's walkthrough book; steady state = converged):

```sh
#!/bin/sh
# deploy.sh — run as alice on web1
set -eu
sudo apt-get update
dpkg -s nginx >/dev/null 2>&1 || sudo apt-get install -y nginx
sudo cp ./nginx.conf /etc/nginx/nginx.conf
sudo systemctl enable --now nginx
crontab -l 2>/dev/null | grep -q backup || printf '0 4 * * * backup-home\n' | crontab -
sudo crontab -l 2>/dev/null | grep -q renew || printf '0 3 * * * cert-renew\n' | sudo crontab -
```

Six tool-sites; lines 7/8 are the deliberate pair (same tool, two users). All renders
below ILLUSTRATIVE.

### §2a — The admin (zero effort, by construction)

The admin never learns this feature exists. Their book is analyzed as-written; env-opaque
or wrapper-heavy books lose *value* (walls, with named hints), never execution fidelity.
The enhancement arrives as stdlib content — the same install that ships apt/dpkg oracles
ships `sudo`'s wrapper-oracle and the stdlib kinds' context declarations
(opaques6-finding9: engine defaults stay safe and opinion-free; the stdlib carries the
opinions; the USER_STORY stage-1 move applied to the context axis).

What they see, by stage of stdlib coverage (compressed from the round's walkthrough):

- **Stage A (HEAD today):** all six sites run; total walls; attention 6/6.
- **Stage B (sudo wrapper-oracle only, no kind declarations):** nothing elides yet —
  but hints become surgical ("line 4 blocked only by: kind `sm.dorc.Package` has no
  user-axis declaration; 1 line would unlock 2 sites"), and under the standing
  footprint flag, walls become KIND-granular (§4c) instead of total.
- **Stage C (+ stdlib kind invariance lines):** lines 3–6 elide when converged. The
  crown jewel is line 4: the admin's own hand-written pre-Dorc guard
  (`dpkg -s … || sudo apt-get install …`) — whose sh already asserts the cross-boundary
  identification (unprivileged read deciding a privileged mutation) — lifts THROUGH the
  boundary, exactly as its sudo-free twin lifts at USER_STORY stage 1. Attention 6→2.
- **Stage D (+ cron declares user-sensitive):** line 7 (alice's own crontab) elides
  normally. Line 8 (root's crontab) is correctly keyed to `cron(root)`, is honestly
  UNPROBEABLE from outside (imp-1; the sensitivity itself is what says alice's
  `crontab -l` doesn't read root's cell), stays in the plan with the admin's own guard
  as the apply-time protection, with the reason printed. **[27C, 2026-07-16: under the
  re-scoped imp-1, line 8 becomes an ordinary measured site when dial×vouch×entry
  align — the entered check reads root's crontab; "unprobeable" now describes only the
  unmet-bar cells.]** AND the disjointness dividend:
  a drifted day where line 8 really runs no longer walls anything else (`cron(root)` ⊥
  everything).
- The `-u` variant (`sudo -u postgres psql …`): axes carry VALUES; cells key to
  (user=postgres); same machinery, no special case.

### §2b — The wrapper-oracle author (`sudo`, `env`, `su`, `nice`, … — small class, pay once)

A wrapper's oracle is per-TOOL family (argv-keyed, 24G §2 — it alone knows its flag
grammar) and answers, per matched argv shape (STRAWMAN shapes, not syllables):

```sh
sudo__context() {
   # 1. PEEL: where the tail begins (after -u USER, -E, --, …) — argparse it owns
   # 2. AXES: user → root (or the -u operand's value); fs-view unchanged
   # 3. ρ-TRANSFORM: env scrubbed-except-survivors {TERM,PATH,HOME→target,USER,LOGNAME};
   #    cwd preserved
   # 4. SELF: my own observables (auth-timestamp refresh) elide-with-the-tail (a vouch)
}
```

Contract points, each load-bearing:

- **Peel is licensed, never inferred.** No context-function ⇒ no peel ⇒ HEAD behavior.
- **Per-axis silence = unknown-move (⊤)** — safe, value-destroying; so totality across
  the (small) vocabulary is a value incentive, not a soundness clause
  (opaques7-finding2). A ⊤-moved axis identifies with NOTHING — not even another
  ⊤-context: two `sudo -u "$DBUSER"` sites with unresolvable `$DBUSER` do NOT share a
  cell statically (opaques6-finding5; stated so nobody later "optimizes" ⊤-contexts
  into equality). Diagnostic names the unresolved variable.
- **The expressibility clause:** declare axes only if the tool's context effects are
  fully expressible in the current vocabulary; else decline-to-peel. Converts
  known-inexpressible cases into pointable mis-declarations; the residual
  (vocabulary-gap × author-ignorance) is imp-3 (opaques6-finding16).
- **ρ-transforms are modeled PER-VARIABLE** (scrub-lists/keep-lists), never as one
  atomic "env" value — env is a *container* whose members include coordinate-determining
  selectors ($PGDATA, $KUBECONFIG, $AWS_PROFILE, $GIT_DIR, $DOCKER_HOST…), arguably the
  biggest coordinate-relevance risk in the whole context surface (opaques7-finding8).
  Statically-groundable claims only: `env VAR=x` is gold (value in argv); sudoers
  env_keep / `su -` login files are host state ⇒ ⊤ absent a claim. A maximal sudo
  oracle turns scrub into concrete *absence* (stronger than ⊤). (-GUESS seed: derive
  per-host sudoers survivors via `sudo -l` at probe time, a touches()-style host-run
  arm — opaques7-finding23.)
- **Wrapper self-effects are the wrapper node's own** (opaques6-finding4): eliding the
  line elides sudo's auth-timestamp refresh, `nohup`'s nohup.out, `time`'s output.
  Identity-transform ≠ observable-free; the wrapper oracle vouches its own observables
  elidable-alongside-the-tail (the existing oracle-vouches-for-itself rule), and
  carries its own footprint for the wrapper node (the line's footprint = union-modulo-
  structure of wrapper node + inner node).
- The `env`-style argv→ρ mapping (`env VAR=x CMD`) is exactly this family: parse own
  argv, emit ρ-assignments whose values are already outer-expanded, name the tail head.

### §2c — The everyday tool-oracle author (postgres as specimen; the kBURDEN referendum)

**The headline (opaques7-finding16): a tool oracle NEVER mentions a wrapper.** No
"under sudo" arms, no context conditionals, no wrapper names, anywhere, in any per-TOOL
function. The postgres author's HEAD-era oracle is the same file after this ships;
composition (wrapper's declarations × kind declarations × their ordinary context-free
functions) is entirely engine-side. This is a DESIGN PROPERTY, unproven against build
contact — if implementation ever forces one wrapper-aware arm into a tool oracle, the
whole kBURDEN story must be re-audited (this is the referendum the spike should watch).

What actually happens to their material under a wrapped site:

- Their `touches()`/check bodies are traced under the installed ρ: a body reading
  `"${PGDATA:-/var/lib/postgresql/16/main}"` under `env PGDATA=/srv/pg …` resolves
  concretely-correct with zero author effort — at HEAD that site was a wall, so their
  oracle becomes MORE correct than before, for free. Under scrubbed-sudo the same body
  resolves to its default, also correct (opaques7-finding17-as-corrected).
- Their shipped checks gain runtime coherence via whole-ρ replication (§6b) — which
  requires NOTHING from them: no enumeration of their tool's env-reads, visible or
  internal (opaques7-finding20; the enumeration objection and its disposal, §4b).
- The **bite-map** by tool class (opaques7-finding18): (1) ARGV-PURE tools (apt, dpkg,
  cp, systemctl, ufw — most of the stdlib): owe NOTHING; full sudo value via kind
  invariance lines alone. (2) SELECTOR-VAR tools (postgres, git/GIT_DIR,
  docker/DOCKER_HOST, kube, aws): the env-explicit-check norm (`-h "$host"` flags /
  explicit `${VAR:-default}`) is analyzer-precision hygiene only (post-repair, NOT
  coherence-load-bearing), lint-nudged, never gated. (3) CREDENTIAL-GATED state
  (peer-auth reads, root-only files): no obligation exists because no effort helps —
  rc≥2/can't-say, the imp-1 honest cap; the r25 `su - postgres -c 'psql …'` line is
  class-3, correct disposition run-with-guard, no author effort demanded or wasted.
- Their remaining lever is the family's ordinary decline: `return 2` for shapes/
  contexts they won't answer for (the stage-4 arity-gate discipline, unchanged).

### §2d — The kind-owner (per-axis context topology; <10% cohort, high community effect)

One declaration per (kind × axis) the owner can answer, in the trichotomy of §1.
Concretely for the stdlib: `sm.dorc.Package` user=invariant (one dpkg database,
whoever asks) but fs-view=SENSITIVE (a chrooted dpkg reads a different database);
`sm.dorc.File` user=invariant (post-expansion paths are absolute), fs-view=
sensitive-with-map (the rebase map — BUT see §3b: scalar rebase is dead; the honest map
is bounded); `sm.dorc.Cron` user=sensitive (per-user crontabs). Spelling: the human's
lean (NOT settled law) is newly-minted first-class declaration syntax for the static
trichotomy — it passes DESIGN's be-very-not-sh test (no off-ramp value; produces nothing
on a server) — with runtime-valued maps staying sh-bodied (the carve). NOTE the
load-bearing coupling: minted machine-read declarations are exactly what makes the kind
contract ADJUDICABLE, on which the trust-tier position depends (§4d,
opaques6-finding18). Kind-side axes exclude ρ-side dimensions: a kind cannot be
"env-sensitive" in the coordinate algebra — env reaches coordinates only through values
(opaques7-finding3).

**No cliff at kind-minting** (opaques7-finding1): declarations are per-axis OPTIONAL;
silence on any axis = the may-alias floor; minting a bare kind = today's exact behavior.
The feared "I declared fs topology, wasn't thinking about user" fumble produces walls,
not wrongness — dislocated other-author under-execution requires a POSITIVE false
declaration, which is a pointable line (razor-passing, §4d).

---

## §3 — The axis vocabulary and the two hard axes

### §3a — Versioned-closed, engine-owned (the census verdict)

The vocabulary question was attacked empirically (the round's mining lane; ~40 context
dimensions enumerated from the kernel/POSIX process-attribute surface, the OCI runtime
config, init-system exec-context docs, and checkpoint/restore checklists
[A-oci-runtime-config-2026] [A-man7-namespaces-2026] [A-man7-credentials-2026]
[A-man7-nsenter-2026] [C-criu-checkpoint-restore-2012]). Adjudicated census
(opaques7-finding5): the COMMON coordinate-relevant dimensions beyond
{user, fs-root, env, host} are exactly {cwd, mount-topology, fd-redirection} — of which
fd-redirection is already ctx-outer (shell redirections, modeled natively), cwd is
already a ρ component, and mount-topology folds into fs-view (§3b). The rare tail
(ipc-ns, pid-ns) is near-extinct in ops sh. Verdict (opaques7-finding9, ~SUSPECT-strong,
pending human ruling): class-(a) is NOT closed-forever (time-ns 2020 proves the kernel
keeps minting) but IS **closeable-by-versioning**: finite and standards-enumerated at
any instant, growing at kernel speed (~1 primitive per 2–4 years, with user-ns/cgroup-ns/
time-ns ALL outside the coordinate-relevant class), and — the operative property —
**users never mint axes** (no symbol-grounding round 2; kinds failed closedness because
the WORLD mints them; axes are minted by ONE slow upstream, opaques7-finding6). So: the
axis vocabulary is engine-owned, tied to the dialect version; wrappers whose effects
exceed it decline-to-peel (expressibility clause); gaps are imp-3's priced horizon.

Proposed v1 vocabulary: coordinate axes {user, fs-view} + reserved-never {host}; ρ
components {env-vars(per-variable), positionals, cwd}. Cross-cutting mechanisms that are
NOT axes: outer-residue (redirections/prefix-assignments evaluate in the OUTER context —
[A-shellcheck-sc2024-wiki-2026]'s lesson; the half-built `$()`-scope machinery is the
engine cousin); partiality (imp-6); locks (parked, r26-adjacent).

### §3b — fs-view is not a scalar (and the bounded honest ladder)

Three independent confirmations (Nix's string-replace-unsound warning
[A-nix-store-path-2026]; the cutpoint literature's whole apparatus
[A-rinetzky-local-heaps-2005]; the census: bind-mounts and overlay-upperdirs repoint
referents with root fixed, opaques7-finding7): the real filesystem referent is the
mount-table + cwd + fd-table, and **naive prefix-rebase of path coordinates is
officially the cheap-but-WRONG rung** (internal symlinks/self-references are exactly
Nix's embedded-checksums). The honest ladder for fs-view maps: enumerated-bridge ⊑
in-context-map (needs privilege ⇒ usually unavailable, imp-1) ⊑ may-alias floor.
Bind-mounts are the cutpoint-analog proper — within-context aliasing minted by mount
state, invisible to any per-boundary bridge (opaques5-finding11); the literature's two
stances are TRACK (precision-collapsing) vs FORBID-and-DETECT, and Dorc's structural
advantage is a third: DETECT-and-DEGRADE (probe-lane mount read ⇒ may-alias within the
fs kind ⇒ demote) — affordable because Dorc always has the just-run-the-line bottom
that shape analysis lacked (opaques5-finding9). fs-view is the Hard-Problem cell the
human's dovetail note pointed at; this proposal RESERVES it (seams, floors, the ladder)
and builds user + ρ first (the turn-3 priority sort; fs-view's near-term deliverable is
the queue-3b seam, §7 A7).

### §3c — Prior-art posture (why this family)

The design is squarely the **forbid-and-guard** family — undeclared crossing ⇒ no
identification; sensitive-with-map = the enumerated legitimate crossing — which both
literatures independently bless as the cheap/modular stance: SAS05's cutpoint-freedom
buys reusable summaries; Bazel makes boundaries sound by ENUMERATING what crosses,
never rewriting; Nix guarantees referential integrity only within one canonical root
(opaques5-finding8 [A-rinetzky-cutpoint-free-2005] [B-bazel-sandboxing-2026]
[A-nix-store-path-2026]). The eval-decomposition literature supplies the recursion
shape (decomposed content becomes more CFG inside the same fixpoint, nesting-bounded
[A-unevalizer-2012]) and the per-site refusal posture (opaques3-finding11); smoosh
supplies the formal grounding that re-evaluation under a context IS read/eval-loop
entry parameterized by {parse-context, environment} — i.e., ρ is what the semantics
already says re-entry requires [A-greenberg-smoosh-2020] (opaques3-finding14).

---

## §4 — The correctness model

### §4a — The attribution chain (the load-bearing structure)

Every cross-context elision cites a CHAIN, each link a particular line by a particular
author: **(L1) wrapper's peel+axes+ρ-transform → (L2) the site's cell, engine-supplied
from the tool oracle's binding → (L3) the kind's per-axis declaration → (L4) the tool's
converged-vouch + the probe measurement.** Silence at any link is that link's floor
(wall/guard/run) — never a crash, never a borrowed assumption. Each enhancement rung
adds exactly one named link and unlocks exactly the value that link licenses: monotone
value, monotone NAMED liability, zero unattributed-risk growth — the 233-family posture
extended one axis. The why-lens renders the chain per elision (illustrative):

```
$ dorc why --last web1:14
 14  # sudo pipx install certbot-tool     # elided: converged
 L1 peel+axes  sudo__context(): tail 'pipx…'; user→root, env→scrub  [sudo.oracle.sh:41]
 L2 site cell  sm.dorc.Package : certbot-tool #installed            [pipx.oracle.sh:12]
 L3 identify   sm.dorc.Package user=INVARIANT                       [base-library/kinds:7]
 L4 verdict    pipx__is_converged() rc=0; probe as 'alice' 09:14Z   [pipx.oracle.sh:19]
```

### §4b — Failure catalog (every mode, its bite, its plug)

- **Wrong peel** (argparse mis-parse: `sudo -u postgres psql` → head "postgres", a real
  command ⇒ wrong oracle ⇒ wrong vouch reachable): plugs — engine cross-check (peeled
  head must resolve as a command under ρ; disagreement ⇒ demote-to-wall + diagnostic);
  lying-peel sweep axis; wrapper argparse lints (the R2-MULTIOP cousin)
  (opaques6-finding3).
- **Wrong axes / wrong ρ-claim** (claims env-preserved when scrubbed): value-plane
  wrongness flows to wrong argv/coordinates; pointable line in the wrapper oracle;
  lying-axes sweep. Unmentioned axis = unknown-move ⇒ safe (opaques7-finding2).
- **Wrong invariance** (kind says user-invariant; state is per-user): wrong
  cross-context elision, under-execute — the sharp-knife tier, same treatment as
  resolve()/footprints: attributed by name, lying-declaration sweep axes
  (opaques5-finding5).
- **Wrong bare-sensitive** (declared partitioned, actually shared): wrong DISJOINTNESS
  ⇒ wrong survival past a wall that touched the cell ⇒ under-execute. BOTH trichotomy
  poles are priced; the knife is symmetric (opaques6-finding6).
- **Wrapper self-effects omitted**: the elided line's wrapper-node observables vanish
  un-vouched — hence the self-vouch requirement (§2b; opaques6-finding4).
- **⊤-context**: identifies with nothing, including itself (opaques6-finding5).
- **The mis-binding worst case** (opaques6-finding14): base-library declares
  `sm.dorc.Package` user=invariant (true for dpkg); a third-party pipx oracle binds
  pipx apps (which live in `~/.local`) to that kind; probe-as-alice reads her view;
  invariance identifies it with root's cell; `sudo pipx install` wrongly elides. With a
  CRISP machine-read kind contract the why-chain adjudicates mechanically: the binding
  violated the kind's declared topology ⇒ culprit = the pipx emission line. Pointable.
  With a FUZZY contract it is UNADJUDICABLE — the wrongness lives only in the union of
  two defensible lines (opaques6-finding15). This is what makes contract adjudicability
  load-bearing (§4d).
- **The env-coherence kill and its repair** (opaques7-finding19/20 — human-caught, the
  round's sharpest correction): per-visible-variable probe-pinning is UNSOUND — the
  engine cannot see a black-box binary's getenv surface (`psql` reads PGHOST with zero
  `$PG…` characters in the oracle body), so a bare-shipped check consults the AMBIENT
  probe env while the site acts under the transformed env; a wrong-CONVERGED verdict
  licenses an unattributable wrong elision (the check was correct under its implicit
  coherent-env precondition; the ENGINE composed the vouch across a boundary the vouch
  never claimed to cross). Visible-reference pinning would demand full env-read
  enumeration — omission-shaped, unacceptable. THE REPAIR: **whole-ρ replication**
  (§6b) — no per-variable knowledge needed by anyone.
- **Residuals of the repair** (opaques7-finding23, all priced): sudoers env_keep makes
  sudo's transform APPROXIMATE (the wrapper oracle's positive claim surface; rare);
  euid-derived behavior (getpwuid, peer-auth) cannot be faked by env-replication
  (already the class-3 cap); checks erroring under the replicated env ⇒ can't-say ⇒
  run.
- **Direction-of-harm note**: a wrong-DIVERGED verdict merely runs the line — baseline
  bare-sh behavior; Dorc inserts no commands and never promised prevention. The
  Dorc-specific sin is exclusively the elide direction; every mechanism above is aimed
  at it (opaques7-finding22).

### §4c — Bounds under total silence (what stage B licenses)

Even with zero kind declarations, a peeled site's poison is bounded KIND-granularly
(opaques6-finding8): kinds are type-plane, closed at lift (typed emission — even an
unrunnable dynamic reach-arm yields its target kind statically), so an undeclared
context-move can scramble ENTITY identity but never the kind-set. Poisoned =
touches()-kinds ∪ statically-typed reach co-domains, with TOTAL within-kind may-alias
across the boundary (text-difference proves nothing under an undeclared move — chroot
remaps referents; text-sameness proves nothing either); spared = every other kind.
Invariance declarations then collapse the boundary per-kind, restoring entity-level
comparison. Footprint-fed survival stays behind the standing flag throughout; without
it, a running line is an honest wall and stage B's product is hints.

### §4d — Trust tiers, the razor, and the one open gate question

The governing law-candidate (human-minted this round; supersedes the earlier
pathology-scope wording): **razor-attributable-line — "are all the failures introduced
by this mode directly and singly attributable, dorc-why style, to a particular line of
code from a particular human?"** Omission-failures (the touches() class) fail it —
attributable to a function, but no line SAYS the false thing; positive mis-assertions
pass it. Applied here: gate-1 (creation-as-opt-in) is satisfied by context declarations
identically to touches() (opaques6-finding12); hardcoding sudo instead of this algebra
would NOT avoid the failure classes — it would relocate every assumption into the
engine, unattributable-by-construction, the maximal razor violation
(opaques6-finding17); and the vocabulary being closed + per-axis-total-answerable is
what keeps wrapper declarations omission-free WITHIN the vocabulary
(opaques6-finding16 as refined by opaques7-finding2).

**The open question — the gate-2 flag tier for steady-state cross-context elision**
(NEEDS HUMAN ADJUDICATION, §7 A2). Proposed reformulation (opaques6-finding13): unify
the trust flag by GATED OUTCOME, not claim-type — the flag gates SURVIVAL (keeping an
elision past a RUNNING mutation on traveled claims), whatever claims feed it; so
invariance-fed survival on a drifted day still pays the flag, while steady-state
invariance-elision (no runner between probe and site) rides the vouch tier. Arguments
for (recorded in full in the round notes as opaques6-sell1..5): the kind-subscription
relationship is AWARE (binding to a kind is subscribing to its published topology —
mistakes are adjudicable within a chosen relationship, the vouch-class, not the
mutually-unaware-union class); no bridged mutation (the flagged class's anatomy is an
unfalsifiable open-world negative about a running black-box; steady-state
identification bridges nothing; the probe→apply window is whyrun2's accepted residual);
falsifiability (invariance is differentially TESTABLE — mutate-as-A/probe-as-B,
container fixtures — where footprint completeness never is; testable classes have gone
unflagged historically); the guard-incoherence concrete (the admin's own
`dpkg -s || sudo apt-get install` guard asserts the identification in their own sh —
flag-gating invariance makes the word "sudo" toggle a trust ritual on identical
authored semantics); and flag-dilution (gating everyday sudo value guarantees
book-top boilerplate, destroying the flag's signal exactly where it matters). Honest
limits (opaques6-selllimit1/2): bare un-guarded lines lean on the weaker subset of
arguments; and the whole position is **CONDITIONAL — vouch-tier IF AND ONLY IF
context-topology is a machine-adjudicable clause of the kind contract**
(opaques6-finding15); if the adjudicability machinery (§7 A4) isn't built, the razor
itself says flag it. (A retraction is on record: "typing sudo = consent to elision" was
intent-inflation and is withdrawn; the guard-form of the argument is the defensible
core.) Evolution-skew (correct claims diverging over years) is out-of-scope by ruling —
general, known, unchanged by this work; nearest answers MH2/binary-sha tethering.

---

## §5 — The monotonic-enhancement ladder (and the no-cliff rules)

```
rung  who acts (one party)          buys                                cannot hurt
0     nobody                        nothing (HEAD-identical)            —
1     wrapper oracle (stdlib)       sight; surgical hints; kind-bounded walls (§4c)   wrapper-free books
2     kind-owners, 1 line/axis      per-kind cross-boundary elision; probe-outside     undeclared kinds/axes
3     kind-owners (sensitive)       correct per-value cells; disjointness/survival     invariant kinds
4     kind-owners (maps; fs-view)   enumerated-crossing precision (the Hard cell)      map-less kinds
```

No-cliff rules (both surfaces): kind declarations per-axis OPTIONAL, silence=floor
(opaques7-finding1); wrapper per-axis silence = unknown-move, totality is
value-incentivized not required (opaques7-finding2); no engine default ever identifies.

The `plab` ladder (a user's in-house wrapper, stdlib-but-nothing-else world;
opaques6-finding10): rung 0 — safe, valueless, and plab ROBS its tail of the stdlib
(`plab apt-get install nginx` hides the apt-get entirely; the hint says so). rung 1 —
the two-minute act (the foobar-stage-3 analog): peel + "no axes moved" for the common
log/retry/audit wrapper ⇒ NO boundary exists ⇒ the entire stdlib lights up for the tail
with zero kind-owner involvement. rung 2 — the self-vouch (own observables noise-when-
tail-converged) ⇒ full elision parity with bare lines. rung 3 — only if plab genuinely
moves context: declare the move; the stdlib's already-shipped declarations light up
automatically.

---

## §6 — Probe machinery (where all the new soundness work lives)

### §6a — Probe-outside and its license

Probes never escalate (imp-1). A kind's `invariant` declaration is simultaneously the
identity bridge AND the probe-outside license — invariance means the probe reads the
same referent without the context (opaques5-finding3: one declaration, both uses).
Sensitive kinds are self-declaredly probe-outside-UNLICENSED for other axis-values ⇒
the imp-1 cap: plan-time can't-tell, run-with-guard, reason printed
(opaques6-finding2). Elision under wrappers is structurally invariant-kind value.

### §6b — Whole-ρ replication (probe coherence without enumeration)

The engine ships every check for a wrapped site CLOSED under the wrapper's declared
ρ-transform — morally `env -i TERM=… PATH=… HOME=/root USER=root <check>` — so every
env-read the check makes (visible in sh or buried in a binary) reads act-context
values. Nobody enumerates anything: not tool authors (their getenv surface is
irrelevant), not wrapper authors (they declared the transform anyway)
(opaques7-finding20). Env+cwd replication is privilege-FREE, which yields the per-axis
probe-construction rule (opaques7-finding21): **replicate where free (env, cwd);
declare-or-cap where privileged (user, fs-view); never (host).** A ⊤-component in ρ
makes the closure unbuildable ⇒ unshippable ⇒ can't-say ⇒ run. Checks that ERROR under
the replicated env ⇒ can't-say ⇒ run. (A -GUESS design-seed, partially subsumed:
hermetic probe execution — one canonical minimal env for ALL checks — remains
attractive for the unwrapped-site ambient-contamination case; costs unexamined;
opaques7-seed1.)

### §6c — Analyzer-side tracing under ρ

Oracle bodies (checks, touches()) are traced against the installed ρ for the site:
`${PGDATA:-default}` resolves per the transform (concrete absence under scrub-claiming
wrappers; the argv value under `env VAR=x`; ⊤ where unclaimed). ⊤ flows produce
entity-⊤ ⇒ the kind-level bound — sound, value-losing only for var-dependent facts
(opaques7-finding13/14: the famous idioms are argv-literal and need zero env
machinery; ρ's blast radius is per-textual-reference, not per-command, because peeled
argv was already outer-expanded).

### §6d — The probe-lane-only boundary (invariant; human-acked)

Replication/closure lives EXCLUSIVELY in the probe lane — Dorc-constructed check
invocations (DESIGN's collate-and-deploy; the an-wrappable-seam `<wrapper> -- <leaf>`
indirection is the exact hook, a 077-era live constraint). The apply artifact is
untouched: user lines verbatim; elisions comment-out; guard tier unchanged.
Machine-inserted guards need no closure BY CONSTRUCTION — an in-sequence guard-check
runs in the same shell stream, same environment, immediately before the original
command (the no-staleness argument, extended to env-coherence). The single future
intersection — guards at WRAPPED sites — is parked behind the 23J lane-privilege seam
(opaques6-finding7: this design inserts NO Dorc-authored checks under any wrapper; the
admin's own guards carry apply-time protection; if that ever changes, only the
machine-authored check-half would be closed, never user bytes) (opaques7-finding24).

---

## §7 — Adjudication list (human decisions this proposal owes its keeper)

- **A1 — the versioned-closed axis vocabulary** (§3a; opaques7-finding9): ratify
  engine-owned, dialect-versioned axes; pick v1 = {user, fs-view} + ρ; name the
  host/network-ns overlap away someday; ipc/pid via expressibility clause.
- **A2 — the gate-2 tier for steady-state cross-context elision** (§4d): flag vs
  vouch-tier vs tier-by-relation; the proposal's lean is vouch-tier-conditional-on-A4,
  with survival's flag untouched. THE central trust ruling.
- **A3 — the trichotomy declaration spelling**: minted first-class syntax for the
  static trichotomy (the human's lean, kOOB-compatible: it is engine-config, not
  world-config… this needs explicit kOOB adjudication — it is NEW SYNTAX, not sidecar
  config, but the redline deserves a deliberate reading) vs sh-bodied; maps stay
  sh-bodied either way (the carve).
- **A4 — the adjudicability build-list** (opaques6-finding18, load-bearing for A2):
  machine-read topology clauses on kinds; binding-side smell lints (probe body reads
  user-scoped locations while binding to a user-invariant kind); differential discharge
  in stdlib CI (mutate-as-A/probe-as-B per kind); the why-report adjudication rule.
  Must land before kinds go community-shared; private single-author kinds collapse
  trivially meanwhile.
- **A5 — co-reference unification** (opaques5-finding4): a context bridge is a
  functorial co-reference family (24F §5's scope-gated seed, quantified over a kind);
  design them as ONE mechanism when either unparks, or two mechanisms will exist for
  one identification act.
- **A6 — wrapper-oracle quality bar**: the peel cross-check, argparse lints,
  self-vouch/self-footprint requirements, expressibility clause — a 252-§9-memo-2-style
  checklist item set for any stdlib wrapper brief.
- **A7 — the queue-3b seam reservation (TIME-SENSITIVE)** (opaques5-finding7): the
  entity-algebra rebuild should reserve (i) a context-qualifier in the coordinate
  representation (qualifier segment vs per-context spaces-with-bridges — queue-3b's
  fork to take) and (ii) room for per-kind per-axis topology declarations in the
  kind-owner surface. Cheap now, retrofit-hostile later. Relay to the r24 conductor.
- **A8 — razor registration**: razor-attributable-line as durable law (candidate for
  the rulings ledger / KNOBS-adjacent registry); it generalizes beyond this feature.

## §8 — Broad implementation plan (spike-oriented; NOT a build brief)

Touched design concepts, roughly in dependency order:

- **Wrapper-oracle role surface**: the context function family (peel + axes +
  ρ-transform + self-vouch/self-footprint), per-TOOL keyed; lift + lint tier.
- **Context regions in the CFG/value plane**: generalize the existing subst-scope
  machinery (ScopeEnter/Exit + scope-clobbers, notes/219 q-1.c) into a region carrying
  the context record; install ρ for the region (per-variable env map; positional
  seeding; cwd); outer-residue stays outside the region (redirections/prefix-assigns).
- **Wrapper/inner node split**: two tracked nodes per wrapper line; union-modulo-
  structure footprints; inner node classified with the site argv (already outer-
  expanded) under the region's ρ.
- **Coordinate qualification + the algebra**: context-qualified cells; the per-axis
  trichotomy lookup at every cross-boundary comparison (elide-weld cell-sameness;
  footprint×backing; disjointness); may-alias floors; ⊤-context = identifies-with-
  nothing; kind-granular bounds from typed emission (§4c).
- **Kind-owner surface**: per-axis topology declarations (spelling per A3); parse,
  lift, consume; duplicate/conflict handling per the existing two-hard-error-categories
  posture.
- **Probe compiler**: probe-outside licensing via invariance; whole-ρ closure at the
  wrappable-leaf seam; unbuildable-closure ⇒ can't-say; check-error-under-closure ⇒
  can't-say.
- **Diagnostics/hints (kWARN-rich)**: the dq-family for this feature — unmodeled-
  wrapper-robs-tail, missing-kind-axis-declaration (with unlock counts, the first-wall-
  hint pattern), unresolved-context-variable, peel-disagreement, expressibility-decline;
  why-lens chain rendering (§4a).
- **DST/sweep axes**: lying-peel, lying-axes, lying-invariance, lying-sensitive,
  lying-ρ-claim; the differential discharge harness (mutate-as-A/probe-as-B) as the
  invariance-testability net; e2e fixtures per ladder rung (the walkthrough book is the
  natural fixture seed; the plab ladder as the third-party case).
- **Invariants that must hold at every stage** (test-pinnable): user bytes never
  rewritten (§6d); silence never identifies; ⊤-context ≡ nothing; every cross-context
  elision renders its full chain; probes never escalate; rung-0 behavior byte-identical
  to HEAD goldens.
- **Suggested spike staging**: W1 peel + identity-wrappers (nice/nohup) + all floors +
  diagnostics (zero new trust surface; proves the region machinery); W2 sudo (user axis
  + ρ-scrub) + stdlib user-invariance lines + probe-outside + replication (the value
  headline; the walkthrough book elides); W3 sensitive kinds (crontab pair; disjointness
  dividend; honest caps); W4 `env` (per-variable ρ refinement; selector-var precision);
  fs-view stays seam-only. Each stage's rung-0 regression: wrapper-free corpus goldens
  byte-stable.
- **Sequencing vs the r24 queue**: after the respell + typeless floor (both churn
  classify), and coordinated with queue-3b (A7); the wrapper-oracle surface should be
  authored once against the post-respell dialect. -GUESS: W1–W2 are a coherent
  spike-sized unit; W3–W4 follow adjudication of A2/A3.
- **Risks, named**: the finding16 no-wrapper-awareness referendum (watch for any forced
  wrapper-aware arm in a tool oracle); golden churn at the classify boundary; the A3
  syntax decision gating stdlib authoring; scope-creep toward fs-view (the Hard cell —
  resist until user+ρ prove the machinery); adversarial-analysis findings against §4d's
  tier argument (expected, invited).

## §9 — Explicitly out of scope (fenced, not forgotten)

The banked payload frame (literal-string re-parse mechanics, span-offset provenance,
positional binding at `sh -c 'STR' name args`, guard-synthesis-into-payloads;
turn03 Table 1 as recalibrated — pay-litstring is the natural W5+, composing with this
design's ρ but gated on its own costs); irreducibly-dynamic payloads (imp-4); the 219
capture lane (`$(…)` value-carriage — opposite dataflow direction, owns its own design);
host (imp-5); privilege *semantics* and the become/doas ecosystem collation (the
hard-deferred round-2 topic); guard-insertion under wrappers (23J); secrets;
per-iteration loop verdicts (atomic-command axiom); locks (r26-adjacent).

---

*Corrections log (annotate in place per plan-tier convention):*

- 2026-07-10 (r24 close-out; `plans/270`): the §7 adjudication list is now SCHEDULED — A1/A2/A3
  sit on the `270:block-settle` design-pass agenda (A2 unified with the human's
  `--trust-footprints` outcome-rename as `270:adj-survival-flag-outcome`; A3+A1 as
  `270:adj-trichotomy-spelling`/`270:adj-axis-vocabulary`); **A7 is DISCHARGED-into-plan** —
  the entity-algebra design note (`270:block-settle`) carries the two seam reservations as a
  stated obligation; A6 + the `24T:P-A4` carrier items ride the standing quality-bar checklist
  (P-A4 soft-acked 2026-07-10). §8's "sequencing vs the r24 queue" resolves to: W1–W2 =
  `270:block-context`, after `270:block-rebuild` (which now also carries the
  `262`-§2-minimum wire import); W3–W4 float behind the A2/A3 rulings. The stdlib-blocker
  ruling (human, same date): stdlib-authoring waits on W1–W2 — the §2c no-wrapper-awareness
  referendum must survive build contact before the ~40 oracles are authored.
