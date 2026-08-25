# 26Lb — meta-orchestration brainstorm ledger (the unwelding sitting)

> AI-authored (Fable, design-rubber-duck sitting, 2026-08-24; human present and
> adjudicating). Living ledger for a multi-front brainstorm over "what Dorc becomes in the
> medium-term" — the conductor-book / whole-infrastructure frame. The conversation rewinds
> between sections to save tokens; each section is SELF-CONTAINED and banks outcomes, not
> dialogue. Everything is conversation-tier unless marked `human-typed`/`human-ack`.
> Numbered `##` sections correspond to the story-index numbers below (some stories will be
> skipped). Independence fence: this author has NOT read `26L`/`26La`; overlap with them
> is convergence, not citation.
>
> **Standing caveat (human-directed): WILD BRAINSTORMING, the whole file.** Nothing in
> here is owed, scheduled, or design-authoritative — it exists for the human's own
> headspace of the product: fronts for eventual improvement, and maybe some cheap wins
> grabbed in the near-term. Cite nothing in this file as a commitment.

## The frame

Human-typed observations opening the sitting: (1) an orchestrator forbidding *all*
cross-host influence is useless as an orchestrator — "db is up, now stand up the web
host" is basic; (2) but sh is already a perfectly cromulent orchestrator at every
granularity we can reach (`until curl $host/ready; do sleep 30; done; ssh next
<standup.sh`); (3) Dorc's posture is already not-the-perfect-orchestrator — the real
positions are *under-using* a beloved sibling, or maximally using several siblings none
of which cover everything, with sh glue at the seams. Goal: maximal value-per-effort at
the fleet/meta-orchestration level; the admin writes a controller-local conductor-book
describing a whole infrastructure, invoking domain-owning orchestrators, sh as the
ossified executable record of the seams.

Conductor synthesis (+SURE on the distinction, conversation-tier on consequences) — two
multi-host topologies, two laws: the r26/`260` fan-out topology (one book to N hosts, N
independent plans) keeps the per-host partition law; the conductor-book topology (ONE
controller-local book whose lines address different machines) makes "cross-host
influence" ordinary single-CFG dataflow. The unweld is ~SUSPECT mostly "promote the
conductor-book topology to first-class." Priced breach: a hostile host's facts influence
lines executing elsewhere (`acc-forged-verdict-contained` given up). Containment strawman
(`lean-cross-host-facts-gate-never-license`): cross-host facts enter the gate/wait/value
plane, never the license plane — they may make lines wait or run, never elide. The
motivating stories only need the safe direction.

## The stories (index)

1. `story-preflight-authentication-collation` — auth is ambient + lazily checked, fails
   mid-mutation; the probe phase is structurally a preflight engine. **RULED; section 1.**
2. `story-readiness-waits-become-analyzed-facts` — hand-rolled until-loops are the
   highest-defect glue lines; vouched readiness facts; satisfied waits elide.
3. `story-cross-tool-handoff-plumbing` — terraform→ansible seams are stringly artifacts
   going stale silently; the capture lane + seam-kinds; derived (never stored) freshness.
4. `story-beloved-tool-steady-state-collapse` — delegation-oracle stdlib over the siblings'
   convergence verbs + `kPROBING` cost-banding; the 45-minute no-op Monday run collapses.
5. `story-run-from-the-middle-without-fear` — `kSCOPE-asked` + derived deps + probed entry
   preconditions = the checked version of copy-pasting lines 60–80.
6. `story-recovery-book-continuous-fire-drill` — cron'd `dorc plan restore.sh --exit-code`
   = zero-mutation weekly drill; runbook-viability drift alerts; 3am render = attention
   product at max stakes. (-GUESS best value-per-effort of the set.)
7. `story-fleet-rotation-strict-choreography` — generate→distribute→flip→revoke across
   hosts; cross-host facts gate the revoke toward WAITING (safe direction only); resume by
   measurement.
8. `story-bootstrap-graph-quine-detection` — derive the inter-host precedence graph from
   one conductor-book; render cycles + the admin's break-point; pure aid-plane.
9. `story-controller-machine-is-a-host-too` — local-exec scope; binaries/env-reads derived
   from the AST at zero oracle cost; the book self-describes its runtime preconditions.
10. `story-migration-position-by-measurement` — long stateful cutovers; position re-derived
    by probing, never remembered; guards on the destructive tail.
11. `story-plan-diff-across-environments` — per-target plans diffed = structural
    world-comparison; plan-hash aggregation generalized; the one story where the fan-out
    partition law is exactly right as-is.
12. `story-approved-bytes-change-management` — byte-honesty + whylog receipts as the glue
    layer's change-management story; mostly packaging; pushes on
    `rul-durable-contents-reviewed-before-design` early and deliberately.

Cross-cutting demand ranking: conductor-book topology w/ gate-plane-only cross-host facts
(7,8,10,2) · capture lane (3,5,7,10) · wait/until modeling (2,6,7,10) · delegation stdlib +
cost-classes (4,6,9) · local-exec scope (9, all implicitly) · zero-licensing aid renders
(8,11). Stories 6/10/12 need almost no new machinery. `--WONDER`: ssh as a new *transit*
species of wrapper (peels argv prefix, runs remainder ELSEWHERE — violates `273`
wrapper-locality in exactly one dimension).

## 1 — preflight (the attention horizon) — CLOSED 2026-08-24

HUMAN-RULED shape: aid-plane, planning-time; mostly hint/lint/warn material with no
significant engine changes — ONE exception (the section's real find): the observable
model grows three demand channels, in exactly the form stdout is modeled — (1) Stdin,
(2) Tty, (3) the UNSPELLABLE channel: attention demands with no sh spelling at all
(hardware-key touch, biometric/OS dialogs — `op run`, browser-bounce auth). Channel 3
has no plumbing (cannot be piped/redirected/satisfied in sh), so per the
be-sh-or-very-not-sh rule it is the sanctioned seat for a pure per-arm claim; no
composition machinery — claims fold to per-site facts, and wrapper peeling composes it
free (`op run` is an ordinary peeling wrapper whose entry arm claims the demand).
Worked example (human's, yubikey): the ssh oracle parses ssh-config at probe time;
`sk-`-typed keys are visible in key material, so "this destination demands per-session
touch" is mintable — render: "the interactivity frontier is pushed out to <here>".
Product statements: "interactivity footgun 150 commands in" (a finding); the earned
"no remaining known interactivity footguns — step away now."

- `rul-three-attention-channels` (human-typed) — the channel model above; the exception
  to the otherwise hint/lint-only build.
- `rul-attention-is-positional` (human-typed) — the human is there, then leaves;
  seat-of-the-pants ops (distinct from stared-at critical deploys, no push-notification
  service). Temporal hazards (sudo ticket expiry, token TTL) are NOT a second model:
  they are the *reason* a line stays un-cleared, and an un-cleared line holds the
  horizon back.
- `rul-interactivity-is-local-books` (human-ack) — remote legs can't prompt (no pty,
  `-T`) and can't hold (stdin is the artifact; `26K:sit-stdin-copy-exec-amendment`
  gains a second consumer); the feature concentrates on local-exec/conductor books.
  Holds fence to controller scope; lint a `read` in a shipped book.
- `rul-admin-balances-attention-in-sh` (human-typed) — the admin authors the
  attention-vs-expiry balance (leading `sudo -v`, margins, gates); Dorc only (1) lifts
  it and (2) collates oracle-noticed bits. Exactly the existing shape. The
  non-interactivity vocabulary is largely native tool flags (`sudo -n`, `apt-get -y`,
  `ssh -oBatchMode=yes`, `terraform -input=false`, `DEBIAN_FRONTEND=noninteractive`) —
  the lift is argparse recognition; the hint is "add `-n`/`-y` here"; the oracle mark
  covers the flagless residue.
- `rul-no-engine-environment-mutation` (human-typed) — the engine never nulls stdin/tty
  per-region; the admin spells `exec </dev/null` and Dorc LIFTS it as an authored
  attention-boundary. Caveat held: tty-prompters bypass stdin. Lint family minted:
  spelled boundary vs claimed demand ("oracle-marked stdin-listener below your
  `exec </dev/null` — it will fail there").
- `rul-notifications-out-of-scope` (human-typed) — stall/completion summons is
  book-material (curl your own push service); no analysis content. General principle on
  record: apply-time Dorc barely exists by intent (super-dumb executor); features lean
  planning-time.

Horizon mechanics: silence = may-prompt (⊤); horizon = the point after which no
un-cleared site remains, per-plan post-fold (elision moves it earlier; drifted days
honestly re-extend). Announcement is STATIC (plan render); live mid-apply announcement
collides with `dec-26-apply-visibility` — revisit only with local-exec's execution model
or an emission split.

Close-out residue (limited, ack'd): per-channel frontiers collapse to one rendered
horizon (min), detail on ask · in fan-out mode Dorc's OWN sessions are the one non-site
attention source (front-load + BatchMode fail-not-prompt; don't weld the channel model
site-only) · ssh-config parsing is best-effort + declines (Match hairiness; agent-side
`ssh-add -c` flags undetectable). Warts held: auth facts are the volatile-keyed vouch
class (valid-now ≠ refresh-is-noise — margins or declines; the flagship preflight
oracles are the stdlib's HARDEST vouches) · verdict/predict bodies EXPLICITLY
non-interactive (guards run in the apply environment) · probe-side mirror: "N sites
unprobed — missing auth" is the plan-phase preflight, existing kWARN-rich machinery.

## 2 — readiness waits: wait-elision, not wait-modeling — CLOSED 2026-08-24

The story: conductor-books are full of hand-rolled waits (`until pg_isready -h "$db"; do
sleep 2; done`, deadline-arithmetic variants, `timeout 300 sh -c 'until …'`) sequencing
asynchronous mutations. First-cut analysis over-weighted the diverged case and mis-called
waits "guard-tier forever"; the human nack'd; the corrected frame:

- `frame-wait-is-a-guard-spelled-as-a-loop` — wait-elision is stage-1 guard-lift logic in
  loop syntax: probe the condition through its oracle's own argparse, prove the body dead
  (zero iterations), the whole construct goes. Ordinary license — Must-grade fact +
  probe-safety vouch + straight-line-above — NO survival tier, no footprints, no flag.
  Interdependent chains (`start A → wait A → start B → …`) collapse under the existing
  elided-casts-no-wall induction; the book's own sequence carries the interdependence.
- `frame-two-outcome-taxonomy` — waits are {elide, run}; the guard tier is structurally
  redundant for them (a wait IS a guard; guarding one double-checks).
- `frame-regimes-never-overlap` — a wait can only elide on days its fact is stable: a
  diverged upstream mutator RUNS, so the wait stays and self-guards. The mid-transition
  world (where readiness-sampling is information-free) is exactly the world offering no
  elision anyway, dissolving the first-cut hermeticity objection. Residue: flappy or
  wallclock-keyed predicates stay the author's vouch-judgment (the package-index class).
- `frame-conductor-cost-model` — at the meta-orc tier a kept converged wait's check-tax is
  a SERIAL network round-trip or cloud-CLI spawn, per wait, in apply order; the probe runs
  the same predicates in parallel. That, plus ~3–5 attention-lines × dozens of waits, is
  the value — not sleep-granularity wall-clock.

Machinery sketched (conversation-tier): wait-shape recognition (`until P` / `while ! P` /
deadline-arith / attempts-counter / `timeout`-wrap) with the condition routed through the
predicate-command's oracle exactly as guard-lift; the pure-delay-body proof (shared with
`26K`'s wall-transparency increment — one proof, two consumers; structurally excludes
RETRY-loops, whose body or condition contains the mutation); `omit` swallows the
deadline/clock residue as probe-proven-dead branches (no clock modeling, ever);
`StatusIterated` gains a converged-at-entry carve (zero iterations ⇒ one reproducible
rc-0) — a DELIBERATE license-widening of a named invariant, human-flag before any build;
first-party blocking waiters (`aws … wait`, `kubectl wait`, blocking `systemctl start`)
need only ordinary delegation vouches; bare `sleep N` never elides, never walls (stdlib
pure-delay), and carries the ladder-hint ("name the fact and this line can vanish").
Independent cheap win regardless of elision: the wait-defect lint family (no deadline;
`curl` without `-f`; `| grep -q` sigpipe-flap; sleep-before-check; no progress output).

Human-typed dispositions (2026-08-24):
- `rul-toctou-is-a-horizon-not-a-vouch` (human-typed) — TOCTOU is out-of-scope BY FIAT,
  for everybody: an explicit horizon, a bought focus-cost of the spike, a meaningful
  value-loss, suspected owed a someday-revisit. It is NOT "covered by the oracle vouch";
  never conflate the two. (First-cut text here did; corrected.)
- `rul-wait-scope-is-just-shell-modeling` (human-ack; gentle scope-nack) — the takeaway:
  this story is fully covered by continuing to fully model shell in the ways already
  suspected/planned; no bespoke wait machinery is owed. Concrete residue: an XFAIL over
  the `until …; do sleep …; done` shape, nothing more.
- `lean-survival-depriority-for-metaorc` (human-typed lean) — survival is "a party-trick
  for homelabbers"; the defensive real-world metaorc admin (IaC-glue over established
  Terraform/Ansible) never types `--risk-faultless-skips`. Value focus for this cohort:
  straight-line elision on large, converged conductor-books. (A cohort lean, not a
  kHALVES re-litigation.)

Spawned: #2b — curl and *endpoints* (the transport-family problem); taken in-chat first.

## 2b — curl and endpoints: the verbless noun — CLOSED 2026-08-24

(Arc note: "pick 1" of the axis-survey — env-retargeting via `export AWS_PROFILE` — spun
out into a fundamental modeling bug: `export` must havoc the environment model and does
not. Live xfails with wrong-elisions pinned; design-fix planned in a sibling document.
This section banks the curl half only.)

- `frame-mint-the-verb-for-the-verbless-noun` — HTTP is the one major noun-space with no
  CLI at all, which is why it (uniquely) rhymes with nothing else. Canonical END-STATE
  (human-typed): the defensive sh helper per http-resource/owner/SPEAKER — a tiny sh SDK
  (`vultr() { curl -sf -H "auth…" "https://api.vultr.com/v2/$1" …; }`) plus ordinary
  `__role` members, one family per speaker, users off-ramping into the world with it.
  Registry-metadata spelled as live self-checking code; fourth-party churn concentrated
  at one attributable seam; probe-safety is the ordinary structural body-vouch scoped to
  the author's own curl invocations — an honest author exists per-speaker, never for
  curl-the-family. No URL dispatch axis; `curl__` stays minimal; contention dissolves.
- `frame-accidental-spellings` — daily idioms already carry the epistemics: `api()`
  helpers (the end-state pre-invented); `-f` as an rc-semantics claim; `--retry`/PUT as
  replay-safety claims; `| jq -e '.status=="x"'` as a selector expression; `/ready`-family
  endpoints as the fourth party's OWN authored convergence verbs (the k8s-forced norm =
  our delegation surface); `If-Match` as freshness carriage.
- `frame-nightmares-license-lift` — the walls are already daily runtime failures everyone
  ignores (YOLO GETs, wrong-environment strikes via DNS/VPN state, POST-replay
  double-creates, 429 storms, token expiry mid-run). Top-of-curve package sketched: no
  unvouched read; endpoint-identity continuity via curl's own unused
  `--resolve`/`--pinnedpubkey` (wrinkle: pins can't enter guard argv per
  guards-mint-no-values — the sanctioned shape is a DREP-style engine-supplied env value,
  author-consumed; sketch-tier); content-keyed mutations; delegate to `/ready`. Every
  rung degrades to today's exact status quo.
- `frame-four-category-split` (supersedes the own/famous two-way; prevalence estimates
  reading-tier, corpus-measurable cheaply): FETCH (plurality; CDN far side; escapes the
  fourth party — convergence is local file+checksum, existing guards lift today, and the
  value-hint IS the supply-chain nag) · SELF-HOSTED-ADMIN (Grafana/ES/Consul/RabbitMQ/
  Prometheus/Keycloak: famous semantics, owned instance, probeable version, unmetered —
  community helper economics WORK; extend the bootstrap-stdlib list here) · OWN-RESOURCE
  (honest vouch; the foobar story verbatim; the curve may extend INTO the service — a
  richer `/ready`) · SAAS-FAMOUS (cliffs concentrate, then shrink by subtraction:
  official CLIs are ordinary command families; notify is always-run; the residual cliffy
  cell is CLI-less-SaaS non-owners, e.g. DNS-records-by-curl — a corner, not a cohort).
- `finding-the-cliff-is-the-read-side` — human nack folded in: wrapping IS annotation,
  spelled-in-sh (naming foobar's function was also a "migration"); the earlier
  wrap-is-migration framing is retracted. The genuine cliff sits one step further in:
  authoring the verdict body's READ-SIDE (query surface + response shape + comparison).
  Who pays decomposes by category: nobody (fetch) / the admin-as-owner, cheaply (own) /
  the community, once, container-testably (self-hosted) / genuinely cliffy only for
  CLI-less-SaaS non-owners.
- Self-adversarial round, dispositions: the SMELLS (honest-wall-as-default-experience ·
  hint-plane argv-blindness · vouch-laundering-for-third-parties · SDK-as-annotation
  where an official CLI exists) PUNTED by the human as indictments of core Dorc equally,
  except `hold-kstate-metered-probes` (human: genuine interest and concern; many
  performance-flavoured concerns at this altitude; no good sh-memoization idea; addressed
  elsewhere in this saga). Post-split, metering quarantines to the SaaS category alone.
- Residues standing: `residue-capture-jq-subset` — the universal defensive idiom
  (`curl … | jq -e … || mutate`) lifts 0% until captured-stdout value-flow plus a SMALL
  jq subset (`.field`, `== literal`, `-e`, `length>0`) exist; the split re-aims that as a
  modest increment serving the two healthiest categories, not "analyze jq".
  `residue-notify-walls` — a mid-book notify-POST never converges ⇒ permanent wall;
  categorical host-state-inertness is UNSOUND (the webhook-triggers-CD-that-redeploys-
  your-fleet counterexample); shaves: the tail-placement hint (already idiomatic — walls
  cast downward only) + attributed wall prose; the full fix is survival-shaped, so this
  cell is a quantified cost of the cohort-scoping. NB human clarification (typed):
  survival is LIVE and a huge focus of the project; "parked" in this arc meant
  out-of-THIS-discussion (cohort relevance) only.

`takeaway-no-pivot-no-new-machinery` (human-typed close): no particular pivot; no
machinery not already owed for other reasons — the stdlib (protocol-layer curl, fetch/fs
tier, admin-API helper families), the capture lane + jq subset, the respell-hint/kWARN
lane, and pick-1's export-havoc fix in its sibling document.

## 2b-sidequest — sh-env identity: BANKED → `plans/30S` (2026-08-24)

The #2b endpoint thread surfaced the env-retargeting hazard (`export AWS_PROFILE=…`
silently re-aiming byte-identical downstream sites), which grew into its own arc and
is banked as design-of-record at `plans/30S`: the three measured as-built findings
(prefix stripped at the dispatch seam · ρ-fold value-blind, `27K` §8's disclosed debt
· exports never fence transport), the ruled pin-or-sever envelope model
(positive-speech-only; engine owns sh-resolution vars, human-typed; platform
describers own loader/locale; `env -i` as "idiomatic+" off-ramp, human-typed), the
refused alternatives, the stdlib sequencing constraint, and the pinned reds. Not
repeated here. Still PENDING from #2b proper, in-chat: curl/HTTP endpoint routing,
noun-space dispatch, and the construction-vs-recognition thesis's remaining fronts.

## 3 — cross-tool handoff plumbing (capture) — CLOSED 2026-08-25

Three-angle pass over the handoff-seam story (`terraform output | jq` → inventory →
consumer), with 26B/26C/275/26K/30D held in one context (first full-cluster read in a
while). Detail in chat; this is the fixpoint bank.

- refuted-as-pitched: capture-as-dataflow with derivation tracking dies on acked law —
  the seam value is minted at APPLY time (past the consent cut; walls correctly refuse
  folds below the mutating upstream line); the transformation is engine-opaque
  (`26C:law-host-boundary-severs-provenance`); persisted derivation records are
  kSTATE-parked. Payloads are not facts.
- human rulings this sitting: freshness is plain sh's job (mtime/datestamps/headers —
  spelled, then lifted; doctrine holds). jq/templates are ordinary `__predict`
  machinery run at the site (define the non-mutative subset, run it) — never engine
  modeling. Second-product smell is a caution, not a kill (architecture-so-far ≠
  product-forever; prefer buying many behaviours at one uniform principled cost).
  Oracle-provided lint/warn is on the table — the only community-scalable lint
  architecture (long-deferred). Capture STARVING under 30D's stdout-default-declined
  is CORRECT behaviour: unclaimed stdout means Dorc must not muddle; hostname-class
  read-onlys get thirty-second oracles. TOCTOU is mostly correctness, partly PERF
  ("probed, then guard anyway" = worst perf outcome) — maps to
  `26B:need-cancellation-finality-gate`; static early-out = elision ruled out early
  ∧ no consumed observables ⇒ never mint the probe-want.
- surviving reduced story: seam-SHAPE preflight (shape checkable while value volatile:
  renamed key / wrong jq path / unauthenticated are world-independent) · guard-tier
  rederivation (re-derive-and-compare in place; a guard's in-sequence position closes
  TOCTOU by construction; the admin's write-if-changed idiom lifts, Half-B) ·
  unguarded-capture lint · why-narration at consumption distance
  (`26C:need-why-explanation-lane`) · oracle-marked sensitivity (priced: breaches
  `275` §9's empty authored surface).
- synthesis cells:
  - cell-stdout-claims-starve-the-capture-lane — RESOLVED by human as
    correct-by-design (above). Registry lag FIXED this sitting: spike/CLAUDE.md
    role-menu respelled to 30D's algebra; Research/README capture-cluster line now
    points at 30D; `275` §2 carries a superseded-in-part note.
  - cell-write-elision-needs-a-vouch-holder — OPEN; the real remaining content of
    `26K:sit-redirect-routing`. 30D closes the channel-claim leg only; routing and
    File-coordinate binding stay unruled; the engine may never synthesize the compare
    guard (rul-ternary-verdict) nor elide a bare `> file` on byte-equality (mtime
    churn; converged≠no-op needs a judgment-holder). Remaining question: who authors
    the compare (fs-kind stdlib verdict / tool oracle / lifted admin idiom only) and
    how it composes with the producer's predict. 26K on hold with r30 (human:
    machinery incomplete, not forgotten).
  - cell-drep-answers-the-why-spelling-corner — cheap candidate ruling: oracle
    lint/why-contributions = DREP feedback-family records
    (`30D:rul-drep-is-general-oracle-oob`), decision-inert via the two-plane seal,
    attributed claimed-by-oracle-X; would close `26C:feeder-oracle-why-metadata`'s
    open spelling corner.
  - cell-guard-first-probe-economics — rider for the capture-revival brief:
    probe-want minting consults dispositions (the finality gate's static case), not
    just ⊤-ness.
- checked-coherent, no action: 30D's static-authority/runtime-confirms discipline vs
  `26C` severed-provenance + controller-minted attribution; the until-loop direction
  is orthogonal throughout.

## bank-recovery-drill-too-big-for-us — the DR/recovery-runbook niche (examined 2026-08-25)

Story examined: `story-recovery-book-continuous-fire-drill` — cron a recovery book's
`dorc plan --exit-code` as continuous runbook-viability validation; the plan render as
the mid-incident attention product. Verdict (human, hard side-eye not hard nack): the
niche is Too Big For Us in the short/mid-term — and probably *correctly* so, not merely
unaffordable. The served version of the space is sandboxed actual-execution (Veeam
SureBackup, ASR/SRM test-failovers): expensive because that is what verification of
recovery genuinely costs. The cheap probeable slice is the lemon version, and the
remaining unowned slice (semantic/flag/topology rot) is AI-agent-shaped work — Dorc's
only sane posture there is substrate (a tool an agent calls), never owner.

Survivors of the critical pass, banked:

- crit-incident-time-is-maximal-volatility (BANKED, human: blood-on-wall) — incident
  time is, by definition, the moment of maximal world-state churn: the hermeticity
  precondition (`KNOBS:kVOLATILES`) and probe-time→apply-time staleness are maximally
  violated exactly when stakes are maximal. The correlation is structural, not
  incidental — and users may not spot it either. Candidate README-disclaimer material
  (a thought for the record; NOT an owed task).
- crit-guard-half-suspect-under-fire (human extension) — even the guard tier degrades
  under firefighting: (1) the check-tax inverts — check-then-command costs most
  precisely while the downtime clock ticks; (2) convergence ≠ repair: "converge, from
  new/empty or from near-correct" and "repair from arbitrary unknown damage" are
  fundamentally different problems, and an oracle's vouch is authored for exactly ONE
  of them. A convergence-tier yes may not survive transplantation to the repair tier.
- crit-fire-drill-without-fire — probes never mutate (welded), so Dorc structurally
  cannot inject the failure; a drill without fire validates the *healthy* world, and
  the incident is the world leaving that validated state.
- crit-false-assurance-is-the-product-inverted — a green precondition badge displaces
  real drills; the audit-checkbox buyer *wants* the false assurance; a wrong "you're
  fine" here surfaces at maximum stakes with maximum attribution to us.
- crit-probing-the-backup-estate-is-an-attack-path — a standing credentialed probe
  path from the controller into deliberately-isolated backup/secondary estate is a
  security regression (the isolation exists to fence off exactly the controller).
- crit-adverse-selection-of-oracle-quality — oracles are debugged by use; DR paths are
  the least-used, so a recovery book leans on the worst-calibrated vouches in the
  library.
- Honest counter-evidence, recorded: GitLab 2017 was entirely probeable-precondition
  rot — the probeable slice covered the marquee incident; but the base rate is unknown
  (-GUESS: lower today), the post-2017 gap was absorbed by ordinary monitoring, and
  the industry's drawn lesson was restore-testing, not precondition-checking.

mindset-avoid-resembling-disaster-tooling (human-stated 2026-08-25, for the record; no
owed work, nothing foresworn permanently): be careful of Dorc surfaces even *looking
kinda like* disaster-tooling from the wrong angle, in an unfortunate light, unless the
surface was *designed* to be that. The failure mode is accidental entry by resemblance.

What survives, deliberately anticlimactic (no build, no vocabulary): a recovery book is
just a book; `plan --exit-code` stays a check-command some OTHER monitor may run (Dorc
is never the monitoring loop — monitoring is stateful and alien to kSTATE/kAGENTLESS);
backup tools' genuinely read-only native verbs (`restic check`, `borg check`) are
ordinary stdlib candidates on their own merits; the words "fire drill" / "DR" /
"disaster recovery" appear in no render, hint, or pitch.
