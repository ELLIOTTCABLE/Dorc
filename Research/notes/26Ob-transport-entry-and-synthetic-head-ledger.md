# 26Ob — Transport entry and the synthetic head: the `28Q:pin-ssh-entry-shape` sitting ledger

> AI-authored (Fable, design-rubber-duck sitting with the human, 2026-09-04; live and
> banked mid-sitting because the work outgrew one context window). Notes-tier LEDGER: the
> sitting's rulings, proposals, open questions, and their triage into work-units, so a
> successor (or the human, returning) knows what is settled, what is on the human's table,
> how large each piece is, and when it is needed. Grades: **[TYPED]** the human typed the
> substance · **[ACKED]** put and acked as read · **[LEAN]** the human's stated lean, not a
> ruling · **[PROPOSED]** conductor synthesis awaiting the human's word. Nothing here is
> ruled unless marked TYPED or ACKED. Authority: root docs, `spike/CLAUDE.md`, the welds,
> `plans/27C`, `plans/30W`, `notes/26N`, `plans/26O`, and `plans/310` outrank this; when a
> work-unit below is ruled, its design-of-record is a stamped plan and this note keeps only
> the trail. Sibling of `26O` (channels) in the r26 transport lineage. The scout inventory
> this sitting commissioned (181 transport/session entries) lives in the conductor's
> scratchpad, uncommitted; its adjudication is `4-the-scout-inventory-adjudicated`.
> Flag spellings in strawmen are long-form by standing request (`-o RequestTTY=no` is "no
> pseudo-terminal"; `-o BatchMode=yes` is "never prompt"; `-o ProxyJump=bastion` is "hop
> through a bastion").

## 0-one-screen

The pin asked how an `ssh` line can be a `27C` entry form when `"$@"` verbatim in command
position is the only entry shape and ssh re-parses its remainder on the far side. The
sitting's answer, PROPOSED: neither reopen the in-guest preamble nor carve ssh out. The
entry form stays `"$@"` verbatim; what the engine puts in the guest slot is its own
transport scaffolding (the `26N` §2 constant wrapper), and the re-parse is not engine
knowledge but the ssh oracle's own predict body stating what the far end evaluates
(`dorc:sh -c "$*"`), consumed through the existing eval'er descent. Everything else the
engine needs from a transport is measured (capabilities) or derived (keying), never
declared. Around that answer the sitting grew four larger things: an invariant on what may
ever cross a connection prospectively (`rul-no-hopeful-transfer`, TYPED, in
`spike/CLAUDE.md`); a mental model in which the CLI target becomes a Dorc-emitted,
editable, honest head on the plan (`dorc:ssh … <<'DORC'`), so every plan is one shape; a
lean that a fleet is a book, not a CLI list; and a capability-system sharpening (one
vocabulary, two key shapes). The human's closing lean [LEAN, 2026-09-04]: nearly all of
this is PUNTED from r31 and is a round of its own (the transport/orchestration round in
the `26N`/`26O` lineage); what stays in r31 is what is kernel-critical — threaded through
analysis — and `11-r31-re-homing-candidates` lists what likely moves. What is on the
human's table, sized and dated, is `6-tier-two-work-units`.

## 1-the-pin-restated-and-landed

- `the-two-halves-of-the-pin` — (a) `half-book-argv-fidelity`: the site's own argv
  (`ssh h apt-get install nginx`) must reach apt-get's oracle argparse as the argv the
  REMOTE receives; (b) `half-probe-inbound-carrier`: the engine's probe segment (inner
  oracle body plus scaffold) must physically cross the link, since nothing on the
  controller exists on the far side.
- `res-guest-is-engine-scaffolding` [PROPOSED; the load-bearing move, `q1`] — the entry
  form stays `ssh__enter() { ssh -o RequestTTY=no -o BatchMode=yes "$@"; }`; the
  engine's `"$@"` is `sh -c '<constant wrapper>'` with the inner stream on the entry
  call's stdin, the redirect being engine scaffolding at the CALL site, never inside the
  authored form. The wrapper is designed to survive exactly one re-parse
  (`26N:mech-constant-wrapper`). The entered context gets its own scaffold and sink
  (`26O:rul-every-context-carries-its-own-scaffold`). No preamble reopened, no carve, no
  `"$@"` law change.
- `res-fidelity-is-a-predict-claim` [PROPOSED; `q2`] — `26M:axis-dialect-and-fidelity`
  already directed that fidelity is an authored normalizer, not a token class. For ssh the
  normalizer is the predict body: `dorc:sh -c "$*"` (the far end evaluates the space-join
  of the remainder, in sh). The engine descends through the `274` eval'er lane it already
  has; a resolved-literal join re-parses; a ⊤ word is a `24T` hole and walls, naming the
  variable; a word carrying a space re-parses into two, which is the truth. The
  engine-side "simple words" charset is WITHDRAWN (`ack-charset-is-tool-knowledge`).
- `res-channel-behaviour-is-measured` [PROPOSED; aligned with `26N` §4] — byte-clean
  stdin, pty on the path, fs-write, tree, interpreter set, fd-over-2, returns: measured by
  the standup wrapper's first exchange, disclosed on the marker line, never declared. An
  author writes `-o RequestTTY=no` so the measurement comes out right, as they write
  `sudo -n`; the engine never learns what the flag means.
- `res-lend-keying-is-a-planner-hint` [PROPOSED] — a mapped Host lend re-keys every
  fs cell, so an outer scratch path is a name in another world; the emission planner may
  ASSUME bytes must be shipped. Both error directions are safe for the carrier decision
  (a dangling path fails to can't-say; over-shipping is governed by
  `rul-no-hopeful-transfer` and is licensed here only because the shipped bytes are
  zero-influence public code). Measurement confirms; never a license.
- `status-of-the-pin` — resolution PROPOSED, gated on `q1` and `q2`. `28Q` §9 item 11
  and `26N:park-re-parse-carve-explainer` are answered by this section once the human
  types on them; `310:unit-host-index-and-entry`'s gate text then rewrites to point here
  — or, under `lean-punt-transport-to-its-own-round`, that unit moves out of r31.

## 2-rulings-and-acks-taken

- `rul-no-hopeful-transfer` [TYPED] — written into `spike/CLAUDE.md` (host evidence &
  controller attribution): no admin data transits any connection in any direction without
  a `Must`-grade analysis feeding it; the one exception is uninfluenced, pure, public
  dorc-lang bytes. Over-shipping is a leak, not waste. Recorded because three conductors in
  a row assumed otherwise.
- `ack-plan-lines-stay-predictive` [TYPED, hard] — synthetic lines render as sh only when
  plain sh can state honestly and completely what will happen; otherwise they are not shown
  as if they were sh.
- `ack-synthetic-lines-stay-segregated` [TYPED, hard] — emitted synthetic lines keep their
  extra knowledge in memory (provenance, generation, the right to mutate); the drop to sh is
  at emission, never early; they are guard-species, editable, and outside
  `rul-no-hopeful-transfer`'s admin-bytes clause.
- `ack-we-mint-the-universe` [TYPED] — the mental model for CLI-arguments-becoming-book-lines
  is Dorc's to define, held like pre-source (as-if-prepended; segregated); no real sh exists
  to hold parity with, so `rul-unsure-falls-toward-sh-parity` binds authored sh, not this.
- `ack-prefix-door-scoped-to-off-ramp-surfaces` [TYPED] — the `dorc:*` prefix door
  (`274` §9) is closed only where the off-ramp is maintained (oracles, books). Plans are
  ephemera; `dorc:` heads in plans are fine, and a plan being shell-invalid is borderline a
  feature (discourages stale plans, TOCTOU).
- `ack-honest-but-unglamorous` [TYPED] — the sitting's tiebreaker: a mental model the
  author can hold continuously, local, consistent, predictable, beats a clever mechanism.
- `ack-nesting-blind-semantics` [TYPED] — the semantic never special-cases nested entries;
  depth is invisible by construction (edge capabilities meet along the chain).
- `ack-security-and-refag-breakage-later` [TYPED] — build the agnostic mechanism first;
  the owed properties (security, secrets, influence, transport integrity) then break it
  wherever they must; opaque review after the core plan, before threading into the corpus.
- `ack-capabilities-frame-probe-needs` [TYPED] — "probe wants no pty" and its kin are
  capability NEEDS the engine states, then ASKS for by invoking the entry form; which
  capabilities exist and how the stdlib satisfies them is later work.
- `ack-charset-is-tool-knowledge` [TYPED] — no engine-side parsing or sanitising of sh
  arguments for transport; withdrawn (see `res-fidelity-is-a-predict-claim`).
- `ack-fusion-refuses-at-reingest` [TYPED] — the synthetic head is a closed grammar Dorc
  owns; a mutation or decoupling it cannot honestly represent refuses at plan re-ingest,
  before network.
- `ack-inbound-is-standup-in-reverse` [TYPED] — inbound delivery needs the `26O` juggling
  in reverse: reserve the right to fold, armour, unfold per link; promise best-effort
  segregation only. Disliked, unavoidable.
- `ack-naive-verbatim-guest-known-hole` [TYPED] — `"$@"` verbatim in command position
  and "a `read` is data" both forbid oracle-side value manipulation; the hole bites here
  first because shell analysis has no ship-it-and-see fallback. See
  `lean-replace-wrapper-detection-with-argv-landing`.
- `ack-public-versus-private-capabilities` [TYPED, non-critical] — the public capability
  set will freeze; the private one churns; verbose names plus an internal mapping.
- `ack-ssh-and-cp-engine-blocking-stdlib` [TYPED, shrug] — ssh and cp/cmp are
  engine-blocking stdlib needs; start small and focused.
- `ack-dollar-zero-per-shell-process` [ACKED rephrase] — `$0` is a property of a shell
  process, set by its invocation; sourcing, functions, subshells inherit; an exec of a new
  shell resets it to what that invocation names. "Never transits" was wrong (sudo is a
  transit that may not start a shell). Weird edges (`exec -a`, `#!` variance) ride
  `FORFEITS:forfeit-shell-parity-immunity-model`.
- `ack-ifs-and-star-modelling-just-do-it` [TYPED] — modelling `"$*"` (the join of the
  remaining argv under the first character of IFS) and IFS itself in the tracers are
  improvements to get done, no longer deferred; lane-1 material (`310:lane-tracers-and-records`).
- `lean-fleet-is-a-book` [LEAN, human] — native fleet operations are dominated by the Big
  Boys; small users write the fleet into a file, i.e. sh; the population needing N targets
  on one command line is small and shrinking. A user-story sitting is owed (W-C).
- `lean-transport-head-as-plan-lines` [LEAN, human, "not married"] — if the head can be
  spelled honestly as sh, show it as plan lines; budget about two to three lines per host.
  Every function lowered into the plan is a CLI flag the admin never types.
- `lean-inline-heredoc-default` [LEAN, human; conductor-refined] — a single-file book
  stays a single-file plan: the body inline in a heredoc under the head; `dorc:scp` only
  for siblings the user themselves decomposed into.
- `lean-punt-transport-to-its-own-round` [LEAN, human, 2026-09-04] — nearly all of this
  is too much architecture for r31; the `26N`/`26O`/this-note lineage is a round by itself.
  Keep in r31 only what is kernel-critical (threaded through analysis); re-home the rest.
  Candidates: `11-r31-re-homing-candidates`.
- `ack-entry-verbatim-cannot-hold` [TYPED 2026-09-04] — `"$@"`-verbatim-in-command-position
  as the ONLY entry shape cannot hold either: a wrapper may enter with one word of its
  argv and consume the rest itself, or transform the guest before passing it on. The
  replacement is the same one as for predicts (`lean-replace-wrapper-detection-with-argv-landing`,
  applied to entry bodies): an entry form is a body in which the engine's GUEST lands
  exactly once, at an evaluating landing (command position, or a `dorc:sh` code operand
  under a fidelity claim), through modelled transformations; a guest consumed as data,
  duplicated, or dropped is not an entry form and declines. `27C`'s punted in-guest
  preamble becomes one more landing (`sudo -n sh -c 'umask 022; exec "$@"' _ "$@"` lands
  the guest as the positionals of a `dorc:sh` payload) and is supported when the payload
  lane supports positional binding (`24T` L3), never as a special rule.
  `prop-guest-slot-generality-reserved` reads under this: nothing to reserve, the general
  law IS the rule.
- `lean-nack-argument-slot-reaches-an-evaluator` [LEAN nack, human] — widening wrapper
  detection to "the argument slot reaches an evaluator" would further entrench a shaky
  model ("but now you can't modify, reorder, or thread arguments"); the old model may be
  leaky and want replacement. Conductor's opinion:
  `lean-replace-wrapper-detection-with-argv-landing`, below.
- `defer-host-identity-mismatch-posture` [TYPED defer] — an edited destination is an
  explicit user act; warn/lint yes, deny maybe not; the human wants to think it through.
  Shelf thought (conductor, not pushed): withhold by default on witness mismatch, with a
  typed consent that proceeds by demoting every elision to its guard.
- `ack-guards-reach-elisions-witness` [TYPED, hard, 2026-09-04] — "Dorc guards anything
  it can reach but only elides what it can witness" is ground truth; the direction of
  travel is to EXPAND what identities Dorc can witness, never to soften the rule.
- `ack-no-hopeful-transfer-binds-the-probe` [TYPED, restated 2026-09-04] — a full
  Must-grade analysis with no holes is necessary for any admin-authored byte (site argv
  included, through the author's argparse) to reach anywhere, the probe phase included.
  Squared with "even the machine being talked to may be unknown", lightly-modelled
  worlds may lose probing for half a book. The human's tentative read: that set
  probably overlaps the set an efficient analysis would decline to probe anyway ("can't
  tell anything, it will guard"); each rule stands on its own. Conductor's read: the
  overlap is total for the LICENSE consumer (an unknown-world site cannot key its facts,
  so it could only ever guard) and partial for the AID plane (those sites lose the
  plan-time drift display and render "runs: world unknown" rather than "diverged").
- `ack-batching-is-derived-tentative` [TYPED, tentative, 2026-09-04] — `10a`'s derived
  batching key (spelling × unwalled resolution × entry chain) tentatively acked; the
  human could not yet hold the whole of it at once.
- `ack-two-contracts-outside-versus-unanalyzable` [TYPED 2026-09-04] — outside-of-Dorc
  changes (a DHCP lease renegotiated mid-deploy by the world) are a FUNDAMENTALLY
  DIFFERENT problem from within-Dorc-but-unanalyzable changes (the book told DHCP to
  change, Dorc gave the admin oracles and footprints to say so, and they failed). The
  first is a horizon, declared carefully, never walked past silently; the second is a
  contract with attributable repair. Both are UX- and contract-sensitive; neither is
  ignored. "We don't handle all of this" is not what was ruled; "we decide granularly what
  we contract" is.
- `ack-identity-is-defensive-never-intent` [TYPED 2026-09-04] — host identity is never
  used to guess user intent ("did they mean a new machine between these lines?"); it is
  used to be helpfully defensive about the wild ops world WITHIN a Must-grade, full,
  static picture of intent built against the contract on how intent is inferred from
  the user's work.
- `ack-entered-kind-is-declared-never-host` [TYPED 2026-09-04] — "whatever the Host
  kind's owner can measure" forks immediately: different kinds of entered thing have
  different identity capabilities and DISJOINT identity namespaces. Host must not be
  special-cased any more: the entered kind is whatever the entry form declares to have
  been entered, user-minted, disjoint by name (`my.org.SerialWidget` keys identity one
  way, `sm.dorc.LinuxHost` another, `org.docker.Container` another). The human expects
  this to align with the machinery naturally; conductor's read: it is `30W` item 7
  (verb surfaces accepting kind coordinates, `: lends org.docker.Container`) plus
  `30W:rule-dissolve-closed-axis-and-substrate-vocabularies`, and it removes the
  transport-side REASON for blessing Host at all (`30W` §1 blessed Host because
  "transport forces it to know destinations address hosts"; under this sitting transport
  is an oracle). File stays blessed (sh semantics force it). ~SUSPECT Host un-blesses;
  `q8`.
- `ack-prospective-admin-bytes-opt-in-for-aid` [TYPED 2026-09-04] — a carve-out from
  `rul-no-hopeful-transfer`, explicit and immediate: the AID plane (the plan-time drift
  display for sites whose world is unknown) may ship admin bytes prospectively ONLY
  behind an upfront, consumed opt-in of the posture kind (`lean-cohort-defaults-posture`
  material); never by default. The division stays careful and alive and is never crossed
  without consuming the opt-in.
- `ack-target-not-machine` [TYPED 2026-09-04] — "machine" is past the bounds of any
  sane, could-be-assumed-by-everybody barrier; the noun is TARGET. Every "is or is not
  the same target" carries a rider: to which category of admin is THIS the definition
  under which "Dorc will keep probing and applying to the same target for you" is what
  they expect. Sameness is a cohort-declared expectation, never a fact about hardware
  (the liquid-nitrogen RAM transplant is the test case).
- `nack-disk-stamp-is-e-stop-only` [TYPED 2026-09-04] — a Dorc-minted identifier
  written into the target's scratch is weak for anything but an emergency stop: trace
  any `Must`-disjoint that licenses or causes a mutation and a broken target has been
  handed authorization. It licenses NOTHING in either direction (no `same`, no
  `disjoint`); a mismatch may stop an apply; a match affirms nothing. The first draft's
  "mismatch is strong evidence" is withdrawn.
- `ack-witnesses-verify-intent-only` [TYPED 2026-09-04] — every identity mechanism has
  exactly one job: VERIFYING DORC'S MODEL OF USER INTENT, because Dorc cannot ask. If the
  book wanted the target cycled between two lines but kept its address, the witness is
  what detects that and dispatches the correct sureness into the lattice. Never a guess
  about intent; never (in this sitting) security reasoning.
- `req-terminology-as-policy-against-platform` [TYPED 2026-09-04] — when a term is
  made precise, define it as a UNION of the general policy (what is conceptually applied
  to any weird target to decide how to treat, test, and promise about it; the yardstick
  for our own success) against how a user sees what was decided on a common platform
  (init's lifetime on Linux; a RAM-resident process on a machine). About twenty percent
  more general than "the OS instance"; not over-generalised beyond that.
- `ack-rich-internals-keyed-by-the-matrix` [TYPED 2026-09-04] — the internal
  representation stays RICH: values are keyed by, and decisions made from, the full
  identity matrix (`10e`), because cohort defaults or flags may request a different
  analysis, and because it provides a fallback promise for the worst cases
  (a bundle dropped into a sibling tool's generated script; Dorc-as-child). The pin
  (`10f`) is the default promise and the row the public word "target" binds to, never the
  internal key.
- `ack-the-target-pin-provisional` [ACKED provisionally, "I think I mostly ack this",
  2026-09-04] — the pin of `10f` (a target is one continuous period of existence of the
  entered thing) is accepted as the working core concept; its NAME is unchosen
  (`prop-tenure-as-the-name`); the human reserves the right to reject it in favour of
  front-loading the whole comparison matrix if the ops world proves too shattered.
- `req-ledger-becomes-design-docs` [TYPED 2026-09-04] — this ledger will have to turn
  into design documents: rewrites or updates where a design changed, and a new one where
  the material is simply new. Which is which is not yet known; `13-design-doc-destination-candidates`
  enumerates the candidates so the next sitting starts there.
- `lean-cohort-defaults-posture` [LEAN, human, not ruled] — the sitting is walling off
  oceans of day-one value, correctly; this will badly want a one-time posture setting
  (`--hardened` versus a homelab-nice bundle, the security-posture setting `26O` names
  as NYI) behind which sit "sane but technically unsound" assumptions about hosts and
  networks — "assume IP addresses are sane", and kin. Not `--risk-faultless-skips`
  material. Conductor's shape, `q9`: each assumption is a named, disclosed FALLBACK
  identity generator at a lower trust tier (spelling-equality standing in for a declined
  identity read), consented once, attributed to the posture in every why-chain.

## 3-proposals-awaiting-the-humans-word

- `prop-transport-wrapper-authored-surface` [PROPOSED] — a transport oracle authors at
  most three ordinary members: `cmd__enter` (`"$@"` verbatim; non-interactive by
  construction), `cmd__lend_map` (Host mapped to the destination; other dimensions per
  `10-q3-and-entailments`), `cmd__predict` carrying the fidelity claim (`dorc:sh -c "$*"`
  for ssh-class; bare `"$@"` for exec-class). Nothing else. `8-the-authored-surface-unrolled`.
- `lean-replace-wrapper-detection-with-argv-landing` [PROPOSED, in answer to the nack] —
  drop "wrapper-ness is detected by `"$@"` in command position" as a CATEGORY and replace it
  with an ANALYSIS the tracer already half-does: thread each site argv word through the
  predict body's modelled transformations to wherever it LANDS, and let the landing carry
  the semantics — command position (exec verbatim), a `dorc:sh` code operand (re-parsed
  under the fidelity claim), an output line (`printf`), data (`read`/stdin), or an
  unmodelled transformation (⊤). Wrapper, eval'er, and runtime-data stop being three
  families and become three landings of one flow; `273`'s "verbatim, once, locally" becomes
  a derived property of the command-position landing rather than a family definition; and
  the statically-knowable small mutations the human wants (`${1#--}`, constant-prop, a
  closed modelled set of `sed`/`jq`/`grep` someday) are ordinary modelled transformations
  on the way to a landing. Kernel-critical (the tracer IS analysis); lane-1-shaped. The
  entry-form rule (`"$@"` verbatim in command position) is untouched: it binds the
  probe-side ENTRY member, not the predict.
- `prop-entry-forms-only-subtract-interactivity` [PROPOSED] — an entry form may add
  non-interactivity and remove interactivity (`-o RequestTTY=no`, `-o BatchMode=yes`,
  strip a site's `-t`); every other flag passes through verbatim. Bounds the one genuine
  probe/apply transport divergence (`26M:hole-probe-path-transport-divergence`). Author's
  argparse enforces; quality bar lints; attributed when wrong.
- `prop-cli-target-is-a-synthetic-head` [PROPOSED, built on the human's lean] — a CLI
  target is one Dorc-emitted head on an otherwise local plan; the head is the world
  boundary (`30W`); local-exec is the headless degenerate case. Spellings by route:
  file-backed `dorc:ssh -o RequestTTY=no web1 sh /dorc/plan.sh <<'DORC' … DORC` (body
  lands at that path; `$0` absolute; stdin EOF); in-memory `dorc:ssh -o RequestTTY=no web1
  sh -c "$(cat <<'DORC' … DORC)"` (`$0` is `sh`; stdin free); `dorc:scp ./lib web1:/dorc/ &&`
  prefix only when siblings exist. `sh -s <<'EOF'` is REJECTED as a spelling: under it the
  body's stdin is the body, the founding bug written into the model. The head's spelling
  IS the route disclosure (`26N:mech-route-selection` commits the route at plan time).
- `prop-cli-target-through-the-entry-form` [PROPOSED] — the engine keeps no transport
  spelling of its own; the CLI target is entered by composing the stdlib ssh entry form
  with the engine's guest, exactly as an in-book site is. Consequences: the `260` §5
  invocation line and its security-floor flags become stdlib oracle content; the stdlib
  ssh oracle becomes load-bearing for every remote plan (`arc-block-stdlib` re-graded).
- `prop-n-targets-are-n-plans` [PROPOSED; possibly superseded by `lean-fleet-is-a-book`]
  — never N hosts in one synthetic book (`no-reorder-ever` would serialise them; the
  structural per-host partition of `260` s3-1 would weaken to keying). If the fleet is a
  book, this question dissolves into W-C.
- `prop-capabilities-one-vocabulary-two-keys` [PROPOSED; sharpens
  `26O:def-context-and-link-capabilities-are-distinct`] — one closed engine vocabulary;
  node-keyed (a context) and edge-keyed (a link, the inner context as entered from the
  outer) answers kept typed apart because edge answers MEET along the entry chain and
  node answers do not; demands come from bytes and from the engine's own protocol alike;
  `lend_map` is never a capability (a vouch-tier keying claim that can only pre-block).
- `prop-guest-slot-generality-reserved` [PROPOSED] — keep `"$@"`-in-command-position as
  the entry-form rule; do not weld it. Request/response channels consume the guest
  elsewhere and inline it; the general law is "the entry body consumes the guest exactly
  once and the fidelity claim says what evaluates it".
- `prop-target-is-one-tenure` [PROPOSED; provisionally acked as the concept,
  `ack-the-target-pin-provisional`] — the pinned meaning of "a target" for docs and
  promises: one continuous period during which the entered thing exists and runs,
  between the events its kind's owner declares as its beginning and its end; identity is
  continuity of existence, witnessed live, never a name and never a disk. Full statement,
  consequences, and the case against: `10f`.
- `prop-tenure-as-the-name` [PROPOSED; name unchosen] — the ops community has no term
  of art for exactly this. Distributed systems uses INCARNATION for precisely "one
  lifetime of a re-creatable process, disambiguated from its predecessors" (SWIM
  incarnation numbers; TCP connection incarnations, `28Q`'s source; Erlang distribution's
  node "creation"); no PLT collision is known to the conductor (PLT has activation and
  instantiation), but the corpus already uses "incarnation" for the broader LINEAGE
  relation and the human wants a word that can never be confused. Every obvious word
  collides with something of ours (session, instance, run, spell, sitting, generation,
  epoch, era, boot). Candidates that collide with nothing in ops, PLT, distsys, or this
  corpus: **`tenure`** (conductor's lean: "within one tenure", "a tenure boundary",
  "tenure-keyed cells" replacing "boot-keyed", "a tenure witness") · `stint` (shorter,
  more casual). If `tenure` is taken, `incarnation` stays the corpus-internal name for
  the lineage relation (`28Q:res-incarnation-correlation-door`: which prior tenure a new
  one continues), and "boot" leaves the public vocabulary.
- `prop-plan-is-a-two-world-file` [PROPOSED; restates `26N:nack-ship-both-forms`] — the
  reviewed artifact is the whole controller-side plan; the head describes and drives the
  shipping; the shipped bytes are exactly the lines below the head. Headless forms
  (cloud-init payloads, `dorc-run` books) remain requested forms.

## 4-the-scout-inventory-adjudicated

Seven session shapes the corpus already has, all absorbed by one sentence — an authored
line is never a session; the entry form is a session factory the engine invokes as often
as its protocol needs, each exchange with its own controller-minted identity; the only
lines that ARE sessions are the user's own at apply: probe segment per (host, context[,
iteration]) · probe retry (`attempt=`, discard wholesale) · reactive iterations (fresh
nonce, `iter=`; connection reuse invisible to the engine, ControlMaster pinned off) ·
probe concurrency (`142` batches; `sibling-session` a link capability) · apply-time
sibling session (live mode) · guard at a remote site (one call per guarded line) · the
user's line at apply (observed only).

Bites (real consequences): `bite-attribution-scope-goes-multi-in-r31` (`q6`) ·
`bite-repeated-probing-tripwire-is-live` (`rul-repeated-probing-reviewed-before-design`
names multi-target; the human's review moment satisfies it if nothing builds first) ·
`bite-partition-becomes-keying` (`260` s3-1 structural isolation becomes `30W` keying;
`26B:seam-per-host-partition` foresaw it) · `bite-refusal-scope-under-nesting` (`q7`) ·
`bite-bastion-pacing-is-unknowable` (`7-dropped-as-local`) · `bite-nested-retry-origin`
(`7-dropped-as-local`) · `bite-dst-seam-needs-entry-chains` (`7-dropped-as-local`).

Rewrites, not contradictions: `260:dec-26-hosts-spelling` (hosts enter the plan as one
Dorc-emitted head; the kOOB spirit holds) · `260:law-seam-1` (binds engine artifacts per
context; user lines were never engine units) · `an-loc-session` (controller-minted,
carried by the scaffold) · the `26Lb` transit-species musing (confirmed and resolved by
the fidelity claim) · the `26Lb` dispatch-elision finding (a scope note: the
whole-payload dispatch form is a Complex Dispatch with a whole-line vouch and no descent;
the argv form descends through the ssh oracle; both coexist). Scout misattribution
corrected: that finding lives in `26Lb` (brainstorm-tier), not `27Xf`.

## 5-tier-one-open-questions

Kernel-affecting first. Each: the question · why it touches the analysis kernel · what it
gates · the conductor's lean · size of the ruling.

- **`q1-guest-is-engine-scaffolding`** — may an authored entry form's guest slot carry
  engine scaffolding (the constant wrapper) rather than an oracle function invocation?
  Kernel: no (emission/probe composition), but it is the pin itself and every other
  answer rests on it; `rul-only-oracle-bytes-ship` and `probe-composition-walls` read as
  compatible (scaffolding plus oracle bytes; entry through the authored form). Gates
  `310:unit-host-index-and-entry` (or its re-homed successor). Lean: yes. Size: small.
- **`q2-transport-fidelity-via-predict`** — may a wrapper's predict state what the far
  end evaluates (`dorc:sh -c "$*"`), with the code operand the joined site remainder, and
  have the engine descend through it? Kernel: YES — under
  `lean-replace-wrapper-detection-with-argv-landing` this is one landing kind (a `dorc:sh`
  code operand) rather than a detection widening; the value plane must model `$*` (done
  per `ack-ifs-and-star-modelling-just-do-it`); dual-peel coherence (`273` §5) must read
  the landing, not a `"$@"` token; ρ under a bare `dorc:sh` is ⊤ for the remote (correct;
  harmless for the argv form since every expansion happens on the controller). Lean: yes,
  as a landing. Size: medium; lane-1-shaped.
- **`q3-transport-lend-values`** — what does a transport wrapper's `lend_map` emit for
  dimensions it cannot name from argv, and what makes two spellings of one host the same
  world? Worked in `10-q3-and-entailments`. Kernel: YES — `30W` item 1 (the context slot
  as a product over index-kinds), item 2 (the Host binder's measured identity), the
  compose ops, and a new derived output (the transit schedule). Gates
  `unit-context-slot-product` design. Size: large; wants the `30W` §10 sitting, and is
  the first sitting of the transport round if it is minted.
- **`q4-argv-form-carve-typing`** — the "simple words are identity, else decline" carve
  `310`/`26N` §7 lean on. DISSOLVES if `q2` rules yes (the re-parse is derived, and what
  declines is what the `24T` ladder walls). Size: nil.
- **`q5-fan-out-consent-scope`** — `28Q` §3's typed carve gates probe entry into
  book-mentioned hosts on explicit consent. Does it also gate guard-at-apply entry (a
  guard opens a session the user's own line would have opened anyway)? Kernel: settle/plan
  (guard licensing). Lean: consent gates probe entry only; apply-time guard entry rides the
  apply's own license. Size: small. Ties to `design-world-scope-surface`.
- **`q6-attribution-scope-goes-multi`** — `rul-attribution-is-controller-minted` names
  its re-entry trigger (a second scope representable in one run); Host entry fires it
  wherever Host entry lands. Ruling needed: scope-carrying becomes scope-checking in the
  Host-entry unit; each exchange bakes (nonce, attempt, iter, context literal); the
  deframer refuses frames whose scope is not the exchange it opened
  (`26O:rul-frames-are-written-at-minting` already says CHECK). Kernel: intake. Lean:
  yes. Size: small ruling, medium build.
- **`q7-refusal-scope-under-nested-contexts`** — `309`'s whole-target-down plus its
  explicit punt on continued probing under refusal: when an entered context's intake is
  refused, is "the target" that world or the whole plan? For a one-line synthetic head
  they coincide; for a pivot book they do not. Kernel: intake/settle. Lean: none offered;
  security-adjacent. Routed to the opaque review (W-E). Size: medium.
- **`q8-entered-kind-forking`** — the entered kind is declared by the entry form
  (`ack-entered-kind-is-declared-never-host`): `lend_map` lends a user-minted kind
  coordinate, each kind owns its identity read and its resolution-backing declaration,
  namespaces disjoint by name. Kernel: YES — `30W` item 7 (kind coordinates on `lends`),
  `30W:rule-dissolve-closed-axis-and-substrate-vocabularies`, and whether `30W` §1's Host
  blessing dissolves (File alone stays blessed). Also the vantage dimension
  (`res-resolution-is-vantage-keyed`): whether vantage is an index-kind like the rest or
  an engine-owned dimension. Lean: kinds all the way; Host un-blesses; vantage is a kind.
  Size: medium; rides W-2.
- **`q9-cohort-defaults-posture`** — the one-time posture setting behind which
  "sane but unsound" host/network assumptions sit (`lean-cohort-defaults-posture`).
  Kernel: no (a fallback identity generator at a lower trust tier, consented once,
  attributed) but it touches the compare chokepoint's generator list. Lean: the shape
  above; which assumptions are in the bundle is a user-story question (W-C). Size:
  small ruling, medium design. Rides W-B/W-C.
- **`q10-the-target-pin-and-its-name`** — ratify `prop-target-is-one-tenure` and choose
  the name (`prop-tenure-as-the-name`). Kernel: YES — the world coordinate's identity
  semantics (`10b`'s per-index relation for the tenure kind), the Boot index-kind of
  `30W` §1 renamed and generalised to the tenure of any entered kind, the continuity
  relation (`28Q:res-incarnation-correlation-door`) becoming a DECLARED generator that
  disk- and hardware-class reads feed, and `260` §5's host-key continuity re-read as a
  continuity witness rather than an identity. Size: small ruling, medium threading;
  rides W-2.
- **`q11-compare-laws-over-worlds`** — the three asks in `10b`
  (`ask-placeholder-per-entry-chain` · `ask-store-trichotomy-composes-compare` ·
  `ask-names-never-separate`). Kernel: YES, and minispec-tracked (`28Q` §7: any stage
  that moves compare semantics lands there first). Size: small rulings; the build is
  the retrofit-hostile one. Rides W-2; the conductor's read is that the third is already
  implied by `never-derive-separation`.

## 6-tier-two-work-units

What is on the human's table, and when. Under `lean-punt-transport-to-its-own-round`,
"needed by" for the transport-round items reads "at that round's open".

| unit | contents | size | needed by | blocks |
|---|---|---|---|---|
| **W-0 mint the transport round** | ack the punt; name the round; move the candidates in `11-r31-re-homing-candidates`; leave r31 the kernel-critical residue | one ack plus a `ROADMAP`/`310` edit | before r31 opens | r31's shape |
| **W-1 Tier-1 small rulings** | `q1`, `q5`, `q6` typed; `q4` dissolves with `q2` | one short sitting | at the transport round's open | its Host-entry unit |
| **W-2 the `30W` §10 sitting, widened by `q3`, `q8`, `q10`, `q11`** | the six `30W` rulings plus `10-q3-and-entailments` whole: the pin and its name, the compare laws over worlds, the entered-kind forking, Host un-blessing | one large sitting, possibly two | before any context-slot product build (r31 or the transport round) | the re-key |
| **W-3 the argv-landing tracer model** | `lean-replace-wrapper-detection-with-argv-landing` ruled or nacked; `q2` under it; `$*`/IFS | one short sitting, then lane 1 | before lane 1's tracer work touches wrappers | r31 lane 1's wrapper-adjacent units |
| **W-A the synthetic head** | the head grammar per route; the line-budget fence as the one exception to `27C:route-conditional-tail`'s NACK; the head's reason annotation and `dorc why` address; headless faces stay requested forms; `nack-ship-both-forms` restated; fusion boundaries at reingest | one medium sitting | before a plan head is emitted (transport round) | plan emission there |
| **W-B editable transport lines** | `defer-host-identity-mismatch-posture` (yours); edited flags versus refag (the measured-standup floor); the `30W` witness becomes load-bearing | your thought, then a short sitting | before the head is admin-editable | nothing before that |
| **W-C the multi-target user story** | `lean-fleet-is-a-book`; three candidate users (one-box homelabber; conductor-book writer; cron drift monitor) and one test (does any need N targets on one command line); what `260` retires (`--hosts`, width cap as a flag, aggregate exit code); the `--fan-out` consent spelling as the central multi-host UX; overlaps `design-world-scope-surface` | one large sitting | before any multi-host work | nothing single-target |
| **W-D the capability system** | `prop-capabilities-one-vocabulary-two-keys`; public/private namespace; engine-protocol demands beside byte demands; `26N:open-census-needs-value-plane` | one large sitting | before `arc-host-capabilities` | nothing in r31 |
| **W-E the security review's inputs** | outer scratch holds the inner stream; security-floor flags move to stdlib; `q7`; `q6`'s forged-frame cell; edited destination; hostile inner host; the fused-session protocol if W-M rules it | the review's | after the core plan, before threading (human-typed order) | threading this note |
| **W-F transport-tier mechanics** | probe-artifact flatness versus nesting; inbound armouring per link; bastion as two entries; nested retry origin; pacing per destination; the `30X` entry-chain seam column | builder-tier with small conductor rulings | at the delivery build; return to LAST | nothing |
| **W-G stdlib prerequisites** | the ssh oracle (entry, lend_map, fidelity) and cp/cmp, small and focused; `prop-entry-forms-only-subtract-interactivity` as an authoring rule; `park-oracle-value-manipulation`; `park-oracles-knowing-stdin` | authoring, mostly | before hand-authoring the trial oracles | `arc-block-stdlib` |
| **W-M the M:N channel topology** | `10d`: the batching key; the transit schedule (CFG-with-transits → CFG-with-sync-points); the apply-side fork (engine-emitted lines only · an admin opt-in spelling · probe-only) which reopens `26M`'s tabled apply-side transport in a narrow form; the executor as the portable tenure witness and the sibling rendezvous (`10e`); nesting, shell concurrency, reach failures | one large sitting | the transport round, after W-2 | that round's executor-shaped work |

Reading the table: if the punt is acked, r31 needs W-0 and W-3 (short) and keeps its
non-transport lanes; W-2 and W-M are the transport round's first two sittings; W-A, W-B,
W-C, W-D follow in that round; W-E sits between the core plan and threading.

## 7-dropped-as-local

Decide at build; will not be re-raised as design.

- `drop-herestrings` — bash `<<<` is a payload carrier for bash books; rides
  `seam-payload-forms`' own list.
- `drop-lone-copy-line-semantics` — whether `dorc:scp` alone means deliver-without-run;
  local to the head grammar.
- `drop-head-why-address-spelling` — the head's `dorc why` address; local to the render.
- `drop-bastion-pacing` — pace per destination; a `MaxStartups` refusal lands on the
  existing bounded probe retry; a later endpoint-identity hint from the stdlib is safe in
  both directions.
- `drop-nested-retry-origin` — retries are controller-driven; a severed exchange launched
  from a remote outer reads can't-say, disclosed.
- `drop-dst-entry-chain-column` — the `30X` transport seam grows a nested-exchange script;
  builder's.
- `drop-dollar-zero-weird-edges` — `exec -a`, `#!` variance; the parity-immunity forfeit.
- `drop-guard-economics-at-remote-sites` — one session per guarded remote line; economics
  already homed at `26M:q-entry-economics`; W-M's fusion changes the count.

## 8-the-authored-surface-unrolled

The worked site, argv form, from the `26Oa` shape without its payload:

```sh
ssh -o ProxyJump=bastion "admin@$ip" sudo apt-get install -y nginx   # STRAWMAN site; $ip literal above
```

The three authored members (all STRAWMAN spellings; the member names are settled `27C`/`273`
names; the `user` line anticipates `10-q3-and-entailments`):

```sh
# dorc-lang/v0.2 — the stdlib ssh oracle, minimal
ssh__lend_map() {
   while :; do case "${1-}" in
      -o|-F|-l|-p|-i) shift 2 ;;          # transport options: pass through, never interpret
      -t|-T|-*)       shift ;;
      *) break ;;
   esac; done
   printf '%s\n' "$1"          : lends host    # entered: the destination operand is the Host value
   printf '%s\n' "$(id -un)"   : lends user    # measured at entry: whoever the login resolves to
   shift; "$@"                                 # the peel boundary
}
ssh__predict() {
   while :; do case "${1-}" in                 # same peel
      -o|-F|-l|-p|-i) shift 2 ;;
      -t|-T|-*)       shift ;;
      *) break ;;
   esac; done
   shift                                       # the destination
   dorc:sh -c "$*"                             # the fidelity claim: the far end evaluates the join, in sh
}
ssh__enter() {
   ssh -o RequestTTY=no -o BatchMode=yes "$@"  # no pseudo-terminal, never prompt; site options pass through
}
```

The derivation chain the engine runs, each step an existing mechanism:

1. `step-peel` — the site's argv flows through `ssh__lend_map`'s argparse
   (`rul-argv-flows-bytes-do-not`); the Host index is set to the resolved destination
   (`30W` §5, an entered index); the login user is a measured value (`q3`).
2. `step-fidelity` — the same argv through `ssh__predict` lands in `dorc:sh -c "$*"`: a
   `dorc:sh` code-operand landing whose value is the space-join of the remainder. With
   every word literal it is a resolved literal on the `24T` ladder; the engine re-parses it
   as sh and obtains the remote argv: `sudo apt-get install -y nginx`. A ⊤ word (an
   unquoted, unresolved `$pkg`) is a syntax-position hole ⇒ site-local wall naming the
   variable (`24T:pin3`). A word with a space re-parses into two, faithfully.
3. `step-inner-descent` — the remote argv is analysed in the Host world as if it were a
   book line there: `sudo` peels (user mapped to root; fs-view a full lend of the REMOTE
   view), `apt-get` routes to its oracle; its cells key by (Host, user, …).
4. `step-probe-entry` — the probe enters the denoted context by composing entry forms
   outermost-first (`27C` §3): `ssh__enter -o ProxyJump=bastion admin@web1 <engine guest>`,
   where the engine guest is `sh -c '<constant wrapper>'` and the inner segment rides the
   call's stdin from the outer scratch; inside, the wrapper materialises its own scaffold
   and runs `sudo__enter -n <the apt-get verdict body>`. Every executing byte is either
   oracle-authored or engine-owned scaffolding; the admin's bytes never ship.
5. `step-standup-measures` — the wrapper's first exchange measures the link (byte-clean
   stdin, pty on path) and the context (fs-write, interpreters), discloses them on the
   marker; the engine's stated needs (no-pty for probing; a lane control owns) are joined
   against them; a missing need declines entry for that context ⇒ guard/run.
6. `step-fact-keying` — the measurement returns framed with the controller-baked context
   literal (`q6` checks it); the fact lands keyed (Host, user, …); its `establish` reaches
   downstream sites in the same world by ordinary reach.
7. `step-apply` — the user's line runs as written (apply never re-transports; W-M may
   narrow this). If the site cannot elide, the guard is `( ssh__enter -o ProxyJump=bastion
   admin@web1 <carrier with sudo__enter -n <check>> ) || ssh -o ProxyJump=bastion
   "admin@$ip" sudo apt-get install -y nginx` — the check in the denoted context via the
   same entry chain, the fall-through verbatim.
8. `step-what-the-engine-never-did` — it never parsed an ssh option, never knew ssh joins
   with spaces, never knew `RequestTTY=no` means no pty, never decided which shell the far
   end runs (the author's `sh` in `dorc:sh -c "$*"` is that claim, attributed), never
   applied a charset. A mosh oracle differs by one word; a `docker exec` oracle's predict
   lands the remainder in command position instead of a code operand; a serial console has
   no guest slot and lands on the paste rung, honestly.

## 9-register-state

Touched this sitting: `spike/CLAUDE.md` (`rul-no-hopeful-transfer`, host evidence &
controller attribution). Owed at threading (after the review; not done): `ROADMAP.md`
(the `28Q:pin-ssh-entry-shape` gate text, `arc-block-stdlib`'s ssh re-grade, the
`sit-multi-target-user-story` row, the transport round if minted) · `plans/310` (the
re-homing) · `plans/28Q` §9 item 11 status · `notes/26N` §10
`park-re-parse-carve-explainer` (answered here) · `plans/26O` §7/§10 pointers ·
`KNOBS:kBOOT`/`kCOMMS` pointers · `ANALYZER-NEEDS` (a fidelity-claim row; the `$*`
value-plane need; the transit-schedule output) · `ORACLE_PROVIDES` (the entry/fidelity
shapes under provides-context-entry and provides-payload-declaration) · `TODO-ADDTL` (the
user-story porch item). The scout inventory stays in the scratchpad until something in it
is cited as load-bearing.

## 10-q3-and-entailments

The human's three hard things, worked. Everything here is [PROPOSED] unless marked.

### 10a-when-two-spellings-are-one-world

- `hard-thing-one` [human, restated] — `host1` on line X and `host1` on line Y are not
  known to be the same thing without a principled proof with clear horizons, residue, and
  admin-explicable semantics; the same family of question as authored host resolution
  (`30W` §5; `26M:ack-authored-host-sameness-parallel`) and as file/filesystem
  containment (`30T`, `30W` `overlaps`). The human's challenge to the first draft of this
  section [TYPED 2026-09-04]: the engine cannot know that the mutation-time target of
  `host1.widget.tld` was not changed between two call sites by something opaque in the
  script; it must PROVE it, and the proof must involve oracles, contracted claims about
  the Host kind, and a horizon. Conceded; the draft's "batching by spelling is
  refag-clean" was a hole, and is replaced below.
- `res-the-destination-denotes-a-resolution-cell` [PROPOSED; "calling-world" per the
  human's nit, 2026-09-04] — a destination string is a NAME; what it denotes at a line is
  the value of a CALLING-WORLD cell — the entered kind's resolution of that name, keyed by
  the VANTAGE the call is made from (STRAWMAN coordinate
  `sm.dorc.LinuxHost:host1.widget.tld@resolution`, context = the calling world's vantage)
  — and that cell has a BACKING the kind's owner declares in sh, exactly as any other
  kind's stores are declared (`kind__state_stored_in`): the ssh client's configuration
  files, the hosts file, the resolver and name-service switch configuration, and the
  relevant environment. Silence declares nothing, so the backing is ⊤ and EVERY running
  mutator between two transit sites walls the resolution — the safe default. A modelled
  mutator with a footprint disjoint from the declared backing spares it only under the
  survival flag, as every other spared cell (`kSURVIVAL-trusted`;
  `rul-flag-is-razor-residue`). An opaque mutator is a total wall as everywhere. A
  converged day has no running mutator between the transits, so the resolution is
  unwalled by construction (an elided command casts no wall — the stage-3 story).
- `res-resolution-is-vantage-keyed` [PROPOSED; falsehoods 1, 2, 12, 13, 63] — a name is
  meaningful only from a vantage: a bastion-only name does not resolve from the
  controller at all; a split-horizon name resolves differently from inside and outside;
  an RFC1918 literal (`10.0.0.5`) names different machines behind different NATs. So the
  resolution cell is keyed by the calling world's network vantage, and a jump-class link
  (`-o ProxyJump=bastion`, where the far sshd resolves the next hop's name inside its own
  network) MOVES the vantage without entering a shell context: the ssh oracle's `lend_map`
  must lend a vantage for that shape, and the jumped name's resolution cell is keyed at
  the bastion's vantage, itself a name resolved at the calling vantage — a chain, nesting-
  blind. `30W` §5 already keys edge facts (reachability, endpoint state) by vantage and far
  end; this extends the same key to resolution. An IP literal is a name like any other
  (the ssh client's `Host` stanzas can remap it; NAT re-uses it), never self-resolving.
- `res-resolution-is-set-valued-and-landing-is-measured` [PROPOSED; falsehoods 5, 6,
  14, 54–56] — a name may resolve to SEVERAL addresses (dual-stack, round-robin, a load
  balancer) and which one a connection lands on is chosen by the client per attempt. The
  resolution cell's value is therefore a set, and the machine actually reached is a
  per-attempt MEASUREMENT (the identity read, `10c`), never a fact the cell licenses. One
  probe standup lands on one member and measures it; the apply's witness re-measures; a
  different member with a different identity is an integrity withhold. Batching within one
  session is what makes the probe's members consistent.

  ```sh
  sm_dorc_Host__state_stored_in() {                        # STRAWMAN; the stdlib Host owner
     case "${2-}" in
     resolution) printf '/etc/hosts\n'          : fs
                 printf '/etc/resolv.conf\n'    : fs
                 printf '/etc/nsswitch.conf\n'  : fs
                 printf '/etc/ssh/ssh_config\n' : fs
                 printf '%s\n' "$HOME/.ssh/config"  : fs   # the honest read of HOME, 27C:idiom-honest-read
                 ;;
     esac
     # deliberately NO `stored nothing-else`: external name service is a store the owner
     # cannot enumerate, so the resolution stays open-world and collides with any unknown
  }
  ```

- `res-batching-is-derived-not-assumed` [PROPOSED; replaces the withdrawn spelling key]
  — two transit sites may share one probe standup iff (i) their transport argv after the
  oracle's peel is byte-identical, (ii) the resolution cell is UNWALLED between them by
  the ordinary reach machinery over its declared backing, and (iii) they share an entry
  chain. (i) is syntactic; (ii) is the same wall/reach/survival computation every other
  cell gets; (iii) is the composition algebra. Nothing here is a claim about machines:
  it is a claim that no book line the engine can see disturbed what the name denotes, on
  the kind owner's word about where that denotation lives. Batching within ONE session
  is additionally safer than two sessions: an open connection cannot change machines.
- `res-external-resolution-is-the-integrity-plane` [PROPOSED] — what the backing cannot
  enumerate (the name service, DHCP, a load balancer) is UNATTRIBUTED churn, and the
  corpus already carved "which machine am I talking to" out of the same-host-drift
  WONTFIX (`toctou-scope`) and into the integrity plane (`260` §5 host-identity stop-3;
  `rul-integrity-failure-withholds-mutation`: not knowing whether we are still talking to
  the world we think we are). Integrity is established by a WITNESS re-measured at apply
  (`10c-what-host-identity-is`), and a witness needs Dorc's own scaffolding on the far
  side.
- `fnd-elision-needs-a-witness-a-guard-does-not` [PROPOSED; the load-bearing
  consequence] — a guard at a remote site runs its check IN the world the apply reaches
  (`( ssh__enter … <check> ) || <the line>`), so it is sound without any identity
  argument. An elision at a remote site is a Dorc act (a line removed) resting on facts
  measured in the world the PROBE reached; under `26N:rul-dorc-acts-are-withhold-shaped`
  it is withhold-shaped unless the apply can witness that it reaches the same world.
  Where Dorc owns the far-side standup (the CLI target's head; a Dorc-owned transit) the
  witness runs at apply standup and elision is licensable; where the transit is the
  admin's bare `ssh` line, nothing of Dorc's runs on the far side, no witness exists, and
  the ceiling is GUARD. Said plainly for the product: Dorc guards what it can reach and
  elides only what it can witness; hand Dorc the transit and it can witness. This makes
  the admin opt-in spelling of `10d` (`fork-admin-opt-in-spelling`) the elide-half for
  pivot books, not a luxury. Parity check against `sudo`: a `sudo` site elides today
  without a witness because its "which world" question is local (the user referent's
  disturbances are claimed by local oracles such as `useradd`, and the residue is
  same-host drift under the WONTFIX); Host is the one dimension whose "which world" is
  external, which is why `260` treated it differently already.
  <!-- /* superseded by §14 `cor-standup-witness-licenses-bare-line-elision` (2026-09-04,
  human-caught): the apply STANDUP may enter the elided world through the same entry form
  and run the witness; the guard ceiling at bare lines does not hold. */ -->
- `res-aliases-and-clones` [PROPOSED] — different spellings that measure as one machine
  do not merge at v0 (`FORFEITS:forfeit-no-host-merging`): they probe twice and share no
  facts; a why-line may say so, never a license. Cloned machines that present identical
  identity reads are treated as one, and that is the admin's residue, disclosed by the
  kind owner's horizon sentence.
- `res-position-is-ordinary-reach` — `host1@L3` versus `host1@L9` also differ by
  position: things happened between them. That is the world plane's ordinary machinery:
  a CLAIMED index disturbance between them (a modelled reboot's footprint on the Boot
  index-value cell; a write to a declared resolution store) re-keys downstream
  (`30W` §4); nothing claimed means the same world persists (`30W` §9: exits are never
  inferred); UNCLAIMED churn is the witness's job.

### 10b-worlds-are-coordinates-not-partitions

- `hard-thing-two` [human, restated and sharpened, TYPED 2026-09-04] — "part of the key"
  is observably not enough: the engine holds Must-fed and May-fed requirements in
  separate keys, and key inequality does not differentiate PROVABLY DISTINCT from
  UNKNOWN. Because the source of "are these hostnames the same" is itself a lattice
  value, two keys being different must never be read as disjointness when the hosts may
  be the same, and thus the same file, the same cell.
- `retraction-partitioned-lattice` — the first draft's "one product lattice partitioned
  by world" was wrong as an implementation model, for exactly that reason: a partition
  invites the settle/wall seat to treat cells in other partitions as untouched by a
  mutation here.
- `res-worlds-compare-through-the-chokepoint` [PROPOSED; kernel-critical] — a world is a
  COORDINATE in a cell's key, and two cells' worlds relate only through the same ternary
  chokepoint entities do (`compare-consumer-map`): `same`, `provably-disjoint`,
  `unrelated`, `unknown`. Key inequality yields NEITHER `same` nor `disjoint`; it yields
  `unknown`. The two consumers read it in opposite directions (`273` §4's inversion):
  TRANSPORT of a fact from `host1@L3` to `host1@L9` needs `same` (a vouch-tier
  generator: the Host owner's measured identity, `10c`); SPARING — letting a fact
  survive a mutation in a world spelled differently — needs `provably-disjoint` (the
  referent-transparent inequality of measured identities, under the flag). Absent both,
  a mutation at `ssh 10.0.0.5 …` WALLS facts measured through `ssh host1 …`, because
  the engine cannot know they are two machines. This is `never-derive-separation`
  applied to the world coordinate: address-inequality (a different spelling, a different
  key) is not referent-inequality. Consequence for `30W` item 1's build: the settle and
  wall seats must compute kill-traffic ACROSS all worlds and consult the world relation
  per pair, never key equality; only the transport consumer may read key equality (as a
  cheap pre-filter for `same`, since equal keys with an unwalled resolution are the
  batching case of `10a`).
- `res-the-join-is-the-compare-chokepoint` — the "later join/meet point" the human
  describes is not a lattice join; it is a compare verdict consumed by exactly one
  consumer, with `unknown` the default and `set-lifting-universal-meet` making any
  unknown member collide. Must/May shapes live inside each cell unchanged; the world
  relation's own grade (a measured identity is an observation; its "sameness" rests on
  the owner's referent-transparency declaration) is vouch-tier for transport, as `30W` §2
  already has it.
- `res-measured-index-value-is-a-placeholder-per-entry-chain` [PROPOSED; the one
  genuinely new object] — a context is a finite map from index-kinds to index-values; an
  absent kind means the caller's value (the identity element,
  `27C:rul-dimension-owned-compose-ops`); the empty map is the ambient world. An
  index-value has one of three provenances: ENTERED (a literal that flowed through a
  `lend_map`: `user=root`, `host=web1` as a name); MEASURED (a value the kind owner's
  identity read binds at intake, which the analysis holds as a placeholder until the
  standup answers); FRESH (⊤, nothing lent and nothing measurable). The placeholder is
  `sk(E, K)`: "the value of index-kind K under entry chain E", E being the syntactic
  entry chain (spelling after the peel, vantage, resolution unwalled across the span).
  Two sites under one E share one placeholder BEFORE anything is measured — an
  existential instantiation, a Skolem constant: the value is unknown, but there is one,
  and both sites see it. That is what lets analysis key the sites together, plan one
  standup, and let an establish at line 3 reach line 9 pre-network. After the standup
  binds `sk(E, K)` to a measured token, different chains' placeholders compare by token.
- `res-per-index-relation-table` [PROPOSED] — `rel_K(v1, v2)` per index-kind K:
  both absent → `same` · one absent, one present → `unknown` · both entered names →
  `same` iff names equal AND vantages equal AND the name's resolution cell is unwalled
  between the sites, else `unknown`, NEVER `disjoint` (names are names, `30W` §1) · both
  placeholders → `same` iff the same `sk(E, K)`; both bound → `same` iff tokens equal
  and the kind is declared referent-transparent, `disjoint` iff tokens differ and the
  kind is referent-transparent, else `unknown`; an unbound placeholder against anything
  else → `unknown` · ⊤ against anything, itself included → `unknown`
  (`top-identifies-with-nothing`).
- `res-cell-level-relation-is-the-filtered-meet` [PROPOSED] — for cell-kind C at
  contexts c1 and c2: for each index-kind K present in either, consult C's owner's
  store declaration against K (`30W` §4's trichotomy): INVARIANT across K → K is
  irrelevant to this cell, skip it (a volume mounted on two targets stays one referent);
  KEYED by K → take `rel_K`; ⊤ (owner silent) → `unknown`. Then meet across the relevant
  kinds: `same` iff every relevant K is `same`; `disjoint` iff any relevant K is
  `disjoint`; else `unknown`. Consumers unchanged: transport wants `same`; sparing wants
  `disjoint` under the flag; `unknown` is safe for both; any unknown member collides.
- `res-two-identity-tiers-map-onto-two-dispositions` [PROPOSED; closes the soundness
  worry about the placeholder tier] — an analysis-time `same` between two sites under
  one entry chain licenses transport, and transport feeds elision at the later site. That
  is enough exactly where `ack-guards-reach-elisions-witness` already confines elision:
  a Dorc-owned transit runs both sites in one session (same target by construction) and
  the CLI target's head runs under a witness that re-measures at apply. At an admin's
  bare transit line elision is withheld regardless (guard ceiling) and a guard measures
  in place, so transport there is moot. The syntactic tier licenses nothing the witness
  or the single session does not stand behind.
- `ask-placeholder-per-entry-chain` · `ask-store-trichotomy-composes-compare` ·
  `ask-names-never-separate` — the three typed acks W-2 needs to cut the re-key
  (`q11`). The conductor's read: the third is already implied by
  `never-derive-separation` and only needs restating over the world coordinate.
- `bld-consequences-for-the-re-key` — the compare chokepoint consults the store
  trichotomy per index-kind and never key equality; the settle and wall seats compute
  kill-traffic ACROSS worlds with the world relation per pair (a mutation at
  `ssh 10.0.0.5 …` walls a fact measured at `ssh host1 …` unless their measured
  identities differ); fact keys are minted with placeholders and resolved before
  settlement, which is the reactive-era shape (`26C` item 3, host-captured values
  entering the value plane) arriving one round early in its single-shot form; `30S`'s
  ρ-fold must be world-scoped (an `export` on the controller never fences a remote
  world's facts, since env does not cross `ssh`) — the context-slot audit records it.

### 10c-what-host-identity-is

- `hard-thing-identity` [human, TYPED 2026-09-04] — "host identity" has been discussed
  and never defined. Refag: what IS it? An oracle says `open_serial_port COM1` and dumps
  sh down it — what is the identity, how does Dorc know, and is a Linux-specific
  mechanism being assumed? The human does not know how a Linux host is uniquely
  identified besides its hostname.
- `res-the-engine-has-no-notion-of-host-identity` [PROPOSED] — none, and it must not: an
  identity is whatever the Host kind's OWNER can measure in the entered context and is
  willing to answer for, spelled as an ordinary authored member (STRAWMAN name
  `sm_dorc_Host__identity`), executed in the denoted context at the latest sound phase
  (`30W` §6, ask-the-world), returning bytes the engine compares for EQUALITY ONLY as an
  opaque token (`inv-referent-agnostic`), or declining (rc ≥ 2) which reads as `unknown`
  — the safe bottom for both consumers. The horizon is the owner's, in one sentence per
  read. There is no universal identifier; there are candidate reads, each partial:

  ```sh
  sm_dorc_Host__identity() {                          # STRAWMAN; the stdlib Host owner, Linux arm
     m=$(cat /etc/machine-id 2>/dev/null) || return 2          # systemd-era, 128-bit; cloned images share it
     b=$(cat /proc/sys/kernel/random/boot_id 2>/dev/null) || return 2   # per boot; a fresh incarnation
     printf '%s %s\n' "$m" "$b"                                 # the token; equality means same machine, same boot
  }
  ```

  Candidates the owner may compose, honestly graded: the machine identity file
  (systemd-era Linux; absent on BSD, busybox routers, macOS; documented as confidential;
  clones share it) · the per-boot random identity (Linux; unique per incarnation; a
  snapshot restored mid-boot is the residue) · the DMI product identifier (root-readable;
  hypervisor-assigned; clones may share) · the platform identifier on macOS · the ssh host
  keys — NOT a machine identity but an ENDPOINT witness the ssh oracle may contribute
  (`30W` §5: edge facts are keyed by vantage and far end), cloned across images, absent
  on a serial line · the hostname, the weakest (mutable, non-unique) and the one thing
  every host has. A serial console with nothing readable declines ⇒ `unknown` ⇒ every
  transit is its own world ⇒ no transport, no elision past it, guards still work: honest
  but unglamorous, and correct.
- `res-the-witness-is-the-identity-read-re-run` [PROPOSED] — the `30W` §4 witness is
  nothing more than the same authored read executed again at apply standup and after
  every fired index disturbance, compared as an opaque token to the probe's; inequality
  is an integrity withhold, never a verdict input. The one case needing no witness at
  all: probe and apply over ONE open session (a fused transit; live mode's single
  session), where "same machine" holds by construction — an argument for
  engine-owned transits that `10d` records.
- `res-no-linux-assumption-in-the-engine` [PROPOSED] — the Linux-specific mechanisms
  above live in the stdlib Host oracle's Linux arm; a BSD arm reads its kernel
  environment, a macOS arm its platform registry, a busybox arm may decline. The engine
  ships the arm, compares tokens, and never learns what a machine-id is.

### 10d-m-to-n-channels-and-the-transit-schedule

- `hard-thing-three` [human, restated] — the engine must control channels M:N: stand up
  connections that correlate to no admin line (probing, out-of-band control during
  apply), AND reduce churn by dispatching many admin transit lines that provably enter
  the same world, serially related in the CFG, through one standup and scaffold, probed
  in one round. The ideal topology: an engine-owned mapping from a CFG-with-transits to a
  CFG-with-sync-points, where a remote stands up once, materialises, runs the first
  wrapped command, and waits with the tunnel open for the next.
- `res-probe-side-is-already-this` — at probe the design already is M:N: one entered
  segment per (host, context[, iteration]) (`27C` batching; `26C` item 10), channels are
  batches (`142`), waves are pacing (`261`), and no admin line owns a session. The
  batching key of `10a` is what lets many admin transit lines share one standup. A
  persistent session across reactive iterations is the controller-owned per-run channel
  `260` §5 reserved, a link capability (`sibling-session`), invisible to the admin.
- `res-the-transit-schedule-is-an-analysis-output` [kernel-critical] — mapping a
  CFG-with-transits to sync-points is analysis: for each batching key, the set of transit
  sites and their SERIAL ORDER in the CFG (dominance; no `&`-branch between them). Sites
  in parallel branches are not serially related and get separate sessions (or `142`
  batches with `&`); sites on one serial path may share a session. This is a new derived
  output beside the wall/reach outputs, consumed by the emission planner and the apply
  driver, never a license. Nesting-blind: a bastion is one more hop in the edge meet and
  its own batching key.
- `res-apply-side-is-the-fork` [the open question W-M owns] — at apply, the corpus's
  posture is that the user's lines run as written and never re-transport
  (`kBACKFLIPS`; `26M`'s tabled exploration with the human's three cost-side nacks). The
  human's ideal (a remote waits with the tunnel open for the next wrapped command) IS
  apply-side engine transport for admin lines. Three narrow forms, one must be chosen:
  (i) `fork-engine-emitted-lines-only` — fusion applies only to lines the engine emits
  (synthetic heads, guards); admin transit lines keep their own sessions. Sufficient for
  CLI targets and probes; buys nothing for admin-authored pivot books.
  (ii) `fork-admin-opt-in-spelling` — an admin marks a transit line as Dorc-owned
  (`dorc:ssh …` in a BOOK), reviving the `274`-triplet at transit lines: bare `ssh` runs
  verbatim, one session each; `dorc:ssh` hands the transit to the engine's fused session.
  Strip erases the prefix and the off-ramp is "one session per line" — semantically a
  superset (more logins, otherwise equivalent). Reopens, narrowly, the `dorc:`-in-books
  kill (`274` §7, probe-apply divergence) — for an engine-run line there is no divergence.
  (iii) `fork-probe-only-m-to-n` — the current posture, unchanged at apply.
  Conductor lean: (ii), because the human's ideal is (ii)-shaped and the off-ramp is a
  true superset; but it is the tabled question reopened and wants its own sitting.
- `res-fused-session-observables` — what a fused session must preserve to be an honest
  implementation of "N separate ssh lines": each remainder evaluated in a FRESH child shell
  (`sh -c "$remainder"`) so no cwd, variable, or umask leaks between admin lines; each
  remainder's rc returned to the controller's CFG at its line (a lockstep protocol: the
  controller runs local lines, sends the next remainder at each transit, waits for its
  marker); streams routed per `26O`'s planner. What it cannot preserve, disclosed: one
  login instead of N (auth-log lines; the entry form author's vouched residue), and
  login-versus-non-login shell profile differences per remainder (a fidelity claim the
  stdlib author makes, attributed).
- `res-generalisations` — nested/bastion: an executor speaks to its parent the same wire
  the controller speaks to it (`26O:rul-frames-are-written-at-minting`, forwarded never
  re-framed), so the topology recurses. Shell concurrency: the transit schedule's
  serial-relation is the only fusion license; parallel branches stay parallel sessions.
  Reach failures: a marker per remainder tells exactly which admin lines ran; the failing
  line's rc reaches the controller's CFG as the admin spelled it (`26Oa`'s
  `|| [ "$?" -eq 255 ]` expected-sever idiom, `30W` §4 derives the expected case);
  subsequent transits to that key re-standup and re-witness, or withhold.
- `res-what-is-kernel-critical-here` — the batching key derivation (from the peel), the
  transit schedule (serial relation over the CFG per key), the measured-identity witness
  as an integrity input, and the world coordinate compared through the chokepoint
  (`10b`). Everything else in `10d` is transport-tier (the executor protocol, markers,
  sessions) and belongs to the transport round. `10a`'s witness finding narrows the
  apply-side fork: without a Dorc-owned far side there is no witness, so the admin
  opt-in spelling (ii) is what buys elision past admin transits at all.

### 10e-witness-inventory-and-identity-classes

Every way a target can say "I'm me", classed by WHERE the identity lives; each class is
one cohort's meaning of "same target" and carries its own horizon. Nothing lives outside
these four, which is why the hunt for an identity independent of disks and names ended at
"running RAM space".

| class | generators | what `same` means | horizon | cohort whose word it is |
|---|---|---|---|---|
| RAM-resident | the executor nonce (`res-executor-nonce-is-a-tenure-witness`); the kernel's per-boot identity; a held session; the sibling rendezvous (`ipc-kernel-local-rendezvous-with-challenge`) | same continuously-running instance | a snapshot restored twice (one RAM image, two instances) | the VM operator; the container operator; the homelabber, until a reboot |
| hardware-bound | a TPM; a DMI serial | same box | migration; virtual TPMs | bare-metal ops |
| disk-resident | the machine identity file; ssh host keys | same install | cloning | the image-and-fleet cohort |
| name-resident | hostname; address; the spelling | same name | renumbering; split horizon; NAT reuse | the homelabber's default expectation |

- `res-executor-nonce-is-a-tenure-witness` [PROPOSED] — a short-lived receiver
  process that outlives a tunnel drop and accepts the next tunnel, holding a
  controller-minted nonce in memory and by design never writing it, establishes on
  answering: the process that received the nonce is the process answering, so the far
  end is one continuous instance since the probe, across the drop. That is `same` on the
  tenure axis and only that axis — nothing about disk (swappable under a running
  kernel), addresses (renumbered under it), or the box (migrated under it). Presence
  affirms `same` for tenure-keyed cells (the owner's `: process` stores, tmpfs,
  `Service@active`); absence is `unknown` for every axis, since a missing executor cannot
  distinguish "same target, restarted" from "different target" from "the executor died".
  Asymmetric and honest; presence is NOT weak the way the disk stamp's is, because RAM is
  not multiply-mounted. It is the kernel's per-boot identity re-derived portably (same
  class, same strength, same snapshot horizon) at the price of a resident process across
  the consent gap plus a rendezvous; its lifetime is a natural plan-validity window (when
  it dies, tenure-keyed elisions honestly demote). It is the same object the M:N topology
  needs (wait with the line open for the next command), so it is one more job for the
  `kCOMMS` executor pole with a bounded lifetime inside the agentless weld's spirit —
  never a durable agent. For containers it is the ONLY portable tenure witness, since the
  kernel's per-boot identity is kernel-wide and shared by every container on a host.
- `nack-disk-stamp-is-e-stop-only` — see `2-rulings-and-acks-taken`; a Dorc-written
  identifier in the target's scratch is a brake, never a generator; presence is defeated
  by a shared mount, absence by a cleaned or volatile filesystem; and it costs a
  resident file across the consent gap (softening `SIBLINGS`' "leaves nothing resident"),
  residue from abandoned plans that `rul-probe-writes-only-what-it-owns` forbids this
  run from reaping, and a receipt-contents change.
- `ipc-kernel-local-rendezvous-with-challenge` [PROPOSED] — for concurrent siblings
  within one run: sibling A listens on a UNIX-domain socket (or a FIFO) inside its own
  owned scratch; sibling B, arriving on its own tunnel, connects at the controller-literal
  path and asks A to answer with the nonce the controller shipped to A. Success proves B
  is on the same kernel as A. Resistant to sane-world target-splitting structurally:
  sockets and FIFOs are kernel objects, not file contents, so on a shared or
  multiply-mounted scratch the file appears elsewhere but a connect from there fails
  (+SURE for sockets; ~SUSPECT for FIFOs); a round-robin or twin-estates sibling that
  landed elsewhere finds no listener and fails safe; a stale socket file fails safe. On
  Linux the abstract socket namespace keys by network namespace instead of by path.
  Refag holds: this is knowledge about Dorc's own scaffolding's environment
  (`26N:fence-capabilities-are-engine-vocabulary`), measured by the attempt.
- `ipc-the-only-payload-is-identity` [PROPOSED] — nothing else crosses between
  siblings: A's facts already reached the controller on A's lane and the controller
  minted both siblings, so B learns "I am where A was" from the handshake and A's facts
  transport to B's world by `same`. Sibling-to-sibling data on the target is
  `26B:watch-dependent-chain-scheduling`'s host-computed-coordinates hazard; left to the
  review.
- `ipc-multiplexing-makes-it-moot` — sibling sessions on one multiplexed connection are
  the same far endpoint process by construction; the rendezvous is the fallback where the
  `sibling-session` link capability is absent. The controller-owned per-run channel
  `260` §5 reserved is the M:N topology's home.
- `hor-same-kernel-is-not-same-target-for-container-cohorts` — a scratch path
  deliberately bind-mounted into a container lets the container's sibling reach the
  host's listener; the controller-literal scratch root makes that an admin's deliberate
  act; the finer read (a cgroup or namespace identity) is that cohort's kind read.
- `rdg-writes-only-what-it-owns-is-run-scoped` — a rule-reading, not a change:
  `rul-probe-writes-only-what-it-owns` is scoped to the RUN, not the process, so B
  connecting to A's socket is a read of a sibling's owned object within one run, never an
  unowned pathname.

### 10f-the-target-pin

- `def-target-is-one-tenure` [PROPOSED; provisionally acked as the concept; name open]
  — stated as `req-terminology-as-policy-against-platform` demands, policy against
  platform: A target is one continuous period during which the thing Dorc entered exists
  and runs, between the events its kind's owner declares as its beginning and its end.
  The policy, applied to any weird thing we enter and used as our own yardstick: identity
  is continuity of existence, witnessed live, never a name and never a disk. What a user
  sees on a common platform is the concrete event pair — on Linux, init's lifetime; on a
  VM, boot to shutdown; on a container, start to stop; on a widget, power-on to
  power-off; on something stranger, whatever its oracle's owner says begins and ends it.
  Across a boundary nothing carries over except what each kind's owner declares survives,
  checked by a continuity witness the owner names. Where Dorc cannot witness continuity,
  it guards.
- `why-this-row-of-the-matrix` — it is the only identity every witness found
  establishes NATIVELY: a held session by construction; the executor nonce and the
  per-boot identity across drops and the consent gap; the rendezvous between siblings;
  measured tokens across entry chains. Every other row is a READ with a clone,
  migration, or renumbering horizon. The pin is the one thing the machinery is already
  good at, which is what "no lies" requires.
- `why-this-promise-cohort-by-cohort` — the VM operator means an instance; the
  container operator a container lifetime, which is an init lifetime; the homelabber "my
  box, running", and never notices the difference until a reboot; the RAM transplant is
  the same target to everyone but the bare-metal cohort. Twin estates come out RIGHT for
  free: two instances with cloned disk identities are two targets because they have two
  lifetimes; round-robin members are two targets; aliases reaching one instance are one
  target, witnessed, the measured-tier merge `30W` §5 already permits.
- `res-restart-is-a-different-target-and-continuity-is-a-declared-relation` — under
  the pin "same machine, rebooted" is not a hard question; it is a different target. The
  relationship between old and new is a separate, declared, opt-in relation —
  CONTINUITY, `28Q:res-incarnation-correlation-door` exactly — and it is where the disk
  and hardware classes live: a machine identity or host-key equality is a continuity
  witness across tenures, consumed only through the kind owner's invariance line for the
  cells that survive a restart. `30W` §7's patch day is the pin in action already
  (packages and persistent files carry boot-invariance and elide across the reboot;
  `Service@active` is tenure-keyed and guards); nothing new is built, the pin names why
  that render is right.
- `where-everything-else-goes` — HORIZON: a snapshot restored twice; a deliberately
  shared scratch mount into a container; one sentence each, disclosed. CONFIGURATION: the
  continuity relation a cohort wants across restarts — same install (machine identity,
  host keys), same box (a hardware read), same name (spelling, the homelab posture) —
  the cohort-defaults setting (`q9`), the only place the other rows reach a default user.
  SPECIAL ENTRY: transports with no RAM-class witness across sessions (a serial line with
  no live process; no-return channels) — guard-only faces, honest. EXPERT LEVERS:
  per-kind invariance lines; the identity tier's per-aspect relations; container-cohort
  reads; the survival flag.
- `csq-restart-during-review-demotes` — a target that restarts during plan review is a
  new target, so the plan's tenure-keyed elisions demote to guards at apply; the safe
  direction; the why-line says "the target restarted".
- `csq-host-key-continuity-is-re-read` — `260` §5's host-key continuity becomes a
  continuity witness across sessions rather than an identity; the executor nonce (or the
  per-boot identity read) takes its old job.
- `case-against-the-pin` [recorded at the human's request] — it surprises exactly the
  cohorts whose intuition is "same box" or "same name" when a mid-review restart costs
  them elisions, and it makes the executor (or a per-boot read) load-bearing for
  cross-session elision on every target. Against front-loading the whole matrix on every
  concept a user touches: the pin does not hide the matrix; it chooses the row the
  default promise is made on and turns every other row into a named relation an expert
  declares; the default user never sees the matrix; the product statement stays sayable
  in two sentences. ~SUSPECT the pin wins; the human reserves the rejection.
- `fallback-promise-for-headless-faces` — a bundle dropped into a sibling tool's
  generated script, with no session and no executor, has no tenure witness, so the
  promise there is the guard-half only, unless that face's owner declares a read of the
  sibling's own identity (a cloud instance identifier handed to the payload). Consistent
  with headless faces being guards-only already.

### 10g-twin-estates-the-second-design-target

- `def-twin-estates` [named at the human's request as the second first-class design
  target beside the bastion] — two or more sites stamped from one image and one address
  plan, each reachable only through its own jump host, and meant to be converged by the
  same book. Every spelling collides ACROSS sites (`10.0.0.5`, `web1`, `web1.internal`
  each name a different target per vantage; the cloned images share host keys and machine
  identities, so even the disk-class read cannot tell the twins apart, only the vantage
  or the tenure can); every target has several spellings WITHIN a site (short name,
  FQDN, v4 literal, v6 literal, an ssh-config alias); DHCP inside the sites moves
  addresses mid-run; periodic re-imaging yields tenures with identical disk identities.
  Forces falsehoods 1, 2, 5, 6, 12, 13, 14, 15–18, 32, 49, 50, 54–56, 63 at once;
  exercises both failure directions of world identity (wrongly merging the twins; wrongly
  splitting one target's spellings). Real shapes: franchise branches; a customer-site
  replica of a lab; dev/staging/prod cut from one Terraform module; two houses with the
  same router defaults. Bastion tests reach; twin-estates tests naming and identity.

## 11-r31-re-homing-candidates

Under `lean-punt-transport-to-its-own-round` [LEAN]; for the human's ack, one by one;
nothing edited in `ROADMAP` or `310` yet.

- Moves to the transport round: `310:unit-host-index-and-entry` (Host entry; the pin) ·
  `310:unit-measured-index-kinds-and-witness` (the Boot/Machine witness) · the plan-head
  emission half of `310:unit-delivery-shape` (the constant wrapper and file-backed route
  themselves may stay: they fix the founding stdin bug for local-exec pivot books) · the
  `arc-block-stdlib` ssh/cp priority (now this round's first authoring).
- Stays in r31 as kernel-critical: `310:unit-context-slot-audit` (read-only) ·
  `unit-env-identity` (`30S`) · `unit-unrelated-and-settle-gate` (`30U`) ·
  `unit-invariance-line-kind-targets` · lane 1 whole, PLUS `ack-ifs-and-star-modelling-just-do-it`
  and, if ruled, the argv-landing tracer model (W-3) · lane 4 whole · local-exec mode.
- Held: `310:unit-context-slot-product` (the retrofit-hostile re-key). Building it
  before `q3` is settled risks re-doing it; the human's call whether r31 builds it on the
  two-variant slot with the product reserved, or the transport round builds it.
- `ROADMAP` rows whose r31 siting reads optimistic now: the argv-form `ssh` fidelity carve
  "rides Host entry" (moves with it) · `28Q:pin-ssh-entry-shape` as an r31 gate (becomes
  the transport round's first ruling).

## 12-falsehoods-we-nearly-reinvented

`Research/falsehoods-networks.md` read against this ledger at the human's direction
(2026-09-04), topology, reach, and naming only (security items deferred with the review).
Each row: the falsehood's number(s) · the refag-shaped version of it this sitting was
about to assume · where it now lands.

- `false-a-name-resolves-the-same-everywhere` (1, 2, 12, 13, 63) — assumed: a
  destination string denotes one thing wherever it is spelled. Landing:
  `res-resolution-is-vantage-keyed` — resolution is a calling-world cell keyed by vantage;
  jump links move the vantage; bastion-only and split-horizon names are unevaluable at the
  wrong vantage, honestly; an RFC1918 literal is not globally unique.
- `false-a-name-resolves-to-one-machine` (5, 6, 11, 14) — assumed: one name, one
  machine. Landing: `res-resolution-is-set-valued-and-landing-is-measured` — dual-stack
  and round-robin make the value a set; the landing is measured per attempt; the witness
  catches a different member at apply.
- `false-the-machine-behind-an-address-stays-put` (8, 14, 54, 55, 56) — assumed: what
  we probed is what we apply to. Landing: the identity read and its witness (`10c`),
  under `ack-two-contracts-outside-versus-unanalyzable` — outside churn is a declared
  horizon with the witness as defence; within-Dorc churn is the resolution cell's
  declared backing and the ordinary walls.
- `false-an-ip-literal-needs-no-resolution` (2, 37) — assumed: `10.0.0.5` and
  `127.0.0.1` are self-evident. Landing: an IP literal is a name (client config can remap
  it; NAT re-uses it; `127.0.0.1` is not the only loopback and "localhost is me" is a
  measured identity like any other, or a cohort-default assumption, `q9`).
- `false-reachability-is-vantage-free` (15, 16, 17, 18, 49, 50) — assumed: if the
  controller can reach it, the bastion can, and vice versa; if it does not answer it does
  not exist. Landing: reach facts keyed by (vantage, far end), `30W` §5, already; the
  `until ssh … true` connection dance is the ssh oracle's own fact at its vantage
  (`26K:ack-connection-dance-oracles-core`), reachable ≠ provisioned; failure to reach is
  can't-say, never absence.
- `false-a-hardware-identifier-identifies-a-machine` (28–33) — assumed, briefly, in
  the first draft of `10c`: DMI identifiers, addresses, or MACs as identity. Landing: no
  engine notion of identity; the kind owner's read, partial by construction, horizon in
  one sentence; clones and non-unique hardware identifiers are that horizon.
- `false-the-path-is-stable-during-a-run` (19, 25, 27, 46, 47) — assumed by any
  sync-point protocol that waits with a tunnel open (`10d`): latency bounded,
  keepalives free, stream boundaries meaningful. Landing: injected timeouts and the
  wedged/severed cells (`260` s3-4/s3-6) are the model; framing is line-terminated with
  the terminal token (`262` §2); the M:N protocol, when designed, inherits both and
  promises no liveness.
- `false-the-fleet-is-known` (60) — assumed by any invocation-plane host list. Landing:
  `lean-fleet-is-a-book` and `an-inventory-input`'s consume-never-author; a fleet is
  whatever the book says, and only that.
- Not reinvented, worth saying: nothing in this sitting assumed IPv4, a DHCP server, a
  default gateway, the internet, or a DNS server exists (7, 9, 10, 13, 17, 18): the
  design never resolves anything itself, so those live entirely in the transport
  oracle's world and the admin's book.

## 13-design-doc-destination-candidates

Per `req-ledger-becomes-design-docs`: where each cluster of this ledger most plausibly
lands when it is promoted. Candidates only; the human decides rewrite-versus-new; nothing
here is threaded yet.

- `dest-new-plan-transport-entry` — NEW design-of-record for the genuinely new material:
  the transport oracle's authored surface (`8`), the pin and its name (`10f`), the witness
  inventory and identity classes (`10e`), the resolution cell keyed by vantage and
  set-valued (`10a`), the synthetic head (`prop-cli-target-is-a-synthetic-head`,
  `prop-plan-is-a-two-world-file`), twin-estates (`10g`). Probable ID: the next in the
  `26O` lineage or the transport round's own charter, at the human's choice.
- `dest-rewrite-30W` — `plans/30W`: the Boot index-kind generalised to the tenure of any
  entered kind; index-value provenances (entered / measured-placeholder / fresh); the
  per-index relation and the filtered meet (`10b`); Host un-blessed (`q8`); the vantage
  as an index-kind; `lends` accepting user-minted kind coordinates (item 7, already
  there).
- `dest-update-27C` — `plans/27C`: the entry-form rule generalised from `"$@"`-verbatim
  to "the guest lands once at an evaluating landing" (`ack-entry-verbatim-cannot-hold`);
  `prop-entry-forms-only-subtract-interactivity`; the in-guest preamble punt dissolved
  into the payload lane.
- `dest-update-273-and-274` — `notes/273`/`274` are notes (historical), so the
  argv-landing model (`lean-replace-wrapper-detection-with-argv-landing`) wants a
  superseded-by pointer there and its own home in the new plan or in `plans/281`'s
  successor.
- `dest-update-28Q` — `plans/28Q`: §0 vocabulary (tenure beside incarnation, or
  replacing it publicly); §9 item 11 resolved; §3's re-parse carve pointing at the
  fidelity claim.
- `dest-update-26N-and-26O` — `notes/26N` §10's `park-re-parse-carve-explainer`
  answered (superseded pointer); `plans/26O` §7/§10 pointers to the new plan; §2's
  capability tables gaining the two-key sharpening
  (`prop-capabilities-one-vocabulary-two-keys`).
- `dest-update-260` — `plans/260` §5: host-key continuity re-read as a continuity
  witness (`csq-host-key-continuity-is-re-read`); `dec-26-hosts-spelling` rewritten; the
  fleet-kernel sections re-graded under `lean-fleet-is-a-book` when W-C rules.
- `dest-root-registers` — `KNOBS:kBOOT`/`kCOMMS` pointers; `ANALYZER-NEEDS` rows
  (fidelity claim; `$*`/IFS; the transit schedule; measured index values; the
  resolution cell); `ORACLE_PROVIDES` (entry, fidelity, identity read, resolution
  backing); `FORFEITS` (no new rows — every withhold here is a transport condition or a
  horizon, not an analysis limitation); `SIBLINGS` ("leaves nothing resident" if the
  executor lands); `TODO-ADDTL` (the user-story porch item); `ROADMAP`/`310` per
  `11-r31-re-homing-candidates`.
- `dest-spike-claude-md` — steering: `rul-no-hopeful-transfer` (landed);
  `ack-guards-reach-elisions-witness` as a standing invariant once typed into a plan; the
  `writes-only-what-it-owns` run-scoping reading; the two-contracts framing as a
  horizon-declaration rule.

## 14-the-r31-prep-continuation-sitting

2026-09-04, later the same day; Fable design-duck, the human present. Grades as above.
Conduct typed this sitting: design documents may be read freely; code only through a
scout, never hunted; early-r26 documents are SUSPECT on current direction; as-built
informs build sizing only, never the design.

- `cor-standup-witness-licenses-bare-line-elision` [CORRECTION, human-caught] —
  `fnd-elision-needs-a-witness-a-guard-does-not` and
  `res-two-identity-tiers-map-onto-two-dispositions` over-claimed. The apply STANDUP may
  enter every world the plan elided in, through the same entry form and the same consent
  the probe used (`27C` apply-lane entry; `30W` §4's witness at apply standup; `26M`'s
  marker-free witness half), run the identity read once per placeholder, and compare
  tokens; a mismatch is integrity (stop / re-plan), never a verdict input. So a bare
  `ssh h cmd` site elides on the probe's facts under one placeholder, witnessed at
  standup. The guard ceiling holds only where apply entry is impossible, which is exactly
  where probing was impossible, so nothing was elided there. Not TOCTOU creep: one read,
  one boundary, integrity-consumed; creep would be a re-witness per elided site or a
  freshness window. Residue after the witness with no in-book cause is the same horizon
  as same-host drift.
- `ack-stdlib-walls-are-value` [TYPED] — narrow stdlib dependencies falling out as walls
  is a good thing (authority lands with the knowledgeable); the minimal Host/Boot reads may
  be dumb-as-rocks, enough to validate the machinery on synthetic hosts.
- `ack-less-elision-from-kernel-work-is-positive` [TYPED] — kernel work that stops
  things eliding is usually product improvement; the value is bought back by very-deferred
  work (stdlib authorship, narrow opt-ins, hints and lints).
- `lean-metaorchestration-consent-flag` [LEAN; explicitly unruled; post-r31] — the
  survival flag's old semantic is likely dead as named; a spiritual successor is probably
  owed ("yes, I intend Dorc as a metaorchestrator: write code I did not author that steps
  into my ssh remotes and re-uses my sudo authority"); without it, more walls, and the
  product is a linter plus one-hop ssh plus straight-line elision. Options, defaults, and
  consents punted past r31.
- `ack-pools-are-the-admins` [TYPED] — set-valued resolution is the admin's; Dorc plays
  no guessing games about how they address one machine.
- `ack-cross-world-wall-is-the-floor` [TYPED] — a fired transit on `web1` walling
  `web2`'s facts (names never separate) is intentional under safe defaults; a posture
  option follows later (literal-hostname comparison, or trust-IPs with
  expect-route-from-controller and DNS resolution).
- `nit-batching-key-carries-entry-definition-identity` [human nit; conductor read] — E
  must key on the funcenv identity (`28Q` P1's DefinitionId) of the entry form; two
  implementations of `ssh__enter` never share an E. Conductor: the entry-chain half is
  funcenv's for free; the VALUE namespace (what `web1` denotes) is the lent kind's, by
  name, answered by that kind's owner. Both hold. A ⊤ vantage under a silent definition
  identifies with nothing across definitions and is safe within one.
- `fnd-batching-key-is-syntactic-wall-is-rekeying` [PROPOSED; simplifies
  `res-batching-is-derived-not-assumed`] — drop conjunct (ii): batch by (peeled spelling ×
  vantage × entry chain with DefinitionId). The resolution cell is the Host index-value
  cell's backing (⊤ until a Host owner declares stores); a running mutator between two
  same-spelled transits re-keys the lower one to a fresh unmeasured placeholder by
  `30W` §4's one rule. Monotone with the settle loop; answers `hard-thing-one` without any
  probe-time proof.
- `fnd-entered-arm-holds-a-value-plane-object` [PROPOSED; shape-critical for the r31
  slot] — the Entered index-value holds a value-plane object, never a string: captures
  (`ip=$(…)`; `ssh "admin@$ip"`) will flow into destinations, and sharing a placeholder is
  value-flow equality.
- `fnd-token-carriage-is-orthogonal-to-the-slot` [PROPOSED] — the slot binds a
  placeholder to an opaque token; where the token travels from the probe invocation to the
  apply invocation (the plan head; the transport's own memory, e.g. known_hosts) is
  transport or plan-IR, never a receipt (`receipts-not-a-cache`).
- `fnd-below-a-fired-transit-three-rungs` [PROPOSED; walked on the patch-day book] —
  honest mode: everything below the transit guards (coarse in position; invariance lines
  unconsulted). Under the survival flag: boot-invariant cells survive on claims through the
  filtered meet (`30W` §7's render), boot-keyed cells guard, no net. Fusion: a re-witness
  after the transit adds the net, never the license. Across worlds, honest mode walls other
  names too; only differing referent-transparent tokens under the flag spare them.
- `fnd-dorc-owned-admin-transit-hits-three-typed-rules` [PROPOSED] — every route to a
  Dorc-owned transit at an ADMIN line meets a typed rule: `dorc:ssh` in a book
  (`ack-prefix-door-scoped-to-off-ramp-surfaces` closes the door for books; `274` §7);
  fusing bare `ssh` lines under a posture flag (`KNOBS:kBACKFLIPS`: login-shell fidelity is
  not identity relocation); a parallel witness session synced by synthetic plan lines
  (`27C:route-conditional-tail`'s NACK, unless the head's line-budget exception covers
  it). What such a transit buys is the net, pool landing, fewer logins, and per-leg sever
  handling, never the license. `res-apply-side-is-the-fork` therefore leans (iii),
  probe-only, with the CLI head the one Dorc-emitted transit; the third route is the one to
  watch.
- `open-monotonicity-under-placeholder-merge` [builder-tier check, not a sitting] —
  whether the settle loop's grow-only proof survives placeholders that merge when a wall
  between them elides; the `26C` §7 quiet-welding audit is the instrument.
- `lean-witness-is-a-budgeted-backstop` [LEAN, human; near-typed] — the witness does not
  try to close TOCTOU; it is one carefully-budgeted backstop against two catastrophic
  modes: a plan re-run a week later against a newly-provisioned instance by accident; and
  outside-Dorc churn Dorc was not taught to know about (the Primary Orchestrator moving
  boxes, identities, transports, routings between plan and apply — the dorc-as-child cell;
  no specific shape in mind). NOT a per-elision identity re-check at every point a plan
  might have elided.
- `lean-no-fail-fast-after-walk-away` [LEAN, human, explicitly unsure] — once the first
  mutation is dispatched, proceed-and-flag: nothing can rank a half-complete step against a
  finished run resting on wrong assumptions halfway through, so the leavings are
  performance and annoyance ("if critical, the admin is watching and can stop it; if not,
  stopping 30 minutes of work after 5 with half the infrastructure down over an ephemeral
  wrong rc is worse"). Probably wants an admin option someday.
- `cor-no-per-elision-rewitness-was-proposed` [clarification of the conductor's own
  words] — nothing above proposes re-checking identity at elided sites; the only mid-book
  re-witness anywhere in the corpus is `30W` §4's "after every fired index disturbance",
  which on bare lines cannot run and which the conductor described only as fusion's
  value-add. Per-elision re-checks and freshness windows are the TOCTOU creep the
  correction above fences.
- `fnd-two-witnesses-at-artifact-boundaries` [PROPOSED; amends `30W` §4 in the rewrite] —
  exactly two witnesses, both at artifact boundaries, both marker-free: the STANDUP witness,
  before the first mutation, which may ACT (stop, re-plan — the fail-fast window is open
  and stopping costs nothing); and the FINAL-VERIFY witness, after the apply, which may only
  NARRATE (receipt and `dorc why`: "the target changed identity during the apply; the
  survivals below line 6 rested on <author>'s claim"). No mid-book witness that acts:
  under `nack-ship-both-forms` an elided line cannot be un-elided at runtime, so a
  mid-book mismatch has only stop (fail-fast after walk-away) or proceed-knowingly-wrong,
  and `rul-divergence-proceed`'s "no second-guess layer above guards" already forbids the
  acting form. Drop "after every fired index disturbance" from the acting witness; keep it
  as narration. The "admin option" is the existing honest-versus-flag choice; the flag's
  plan-time disclosure gains one line: "survivals below a transit that fires rest on
  <author>'s claim with no runtime check".
- `hor-outside-churn-during-apply` — outside-Dorc churn WHILE the apply runs is the
  declared horizon; the standup witness bounds it to "before the apply began". Where a
  Primary Orchestrator promises quiescence to a chunk it embeds is
  `26N:front-embedding-contracts`, still needing investigation; no shape is asserted here.
- `lean-transports-are-dot-tier` [LEAN, human; "don't change anything yet"] — treat
  commands with an entry form as the orchestrator's domain, excluded from `kBACKFLIPS` on
  the `.` model: reserve the right to munge what the line CARRIES under a sh-semantics
  promise, never a promise of untouched bytes. Conductor: this is required, and typed
  already in substance — `ack-carrier-form-neutrality` × `kHALVES` means the inline
  payload form must take guards, which are byte edits inside a quoted string (`26M`'s
  buys-list, "guard-parity at dispatch sites"; its v1 whole-or-nothing render punted it).
  Lives at `310:seam-payload-forms`. It does NOT entail fusion: the rewritten payload still
  rides the admin's own `ssh`.
- `fnd-no-correctness-requirement-for-fusion` [PROPOSED; the hunt for the half-remembered
  forcing function] — candidates checked and each found to be a value-add or a cost, never
  a requirement: mid-book re-witness (withdrawn above); pools (the admin's); per-leg sever
  versus remote 255 (the admin's recognised idiom); day-zero sites below a creator
  (availability; guards enter after it ran); live first contact (the admin's `-t`);
  identity-by-construction in one session (the standup witness replaces it). ~SUSPECT the
  remembered shape is the pivot strawman's own heredoc form, where one tunnel is the
  ADMIN's spelling and Dorc's job is the interior rewrite. The tunnel count is the admin's
  spelling; converting the argv form into one session is `kBACKFLIPS-compile-to-fit` and
  `26M`'s nack two. Lean: decline fusion; `res-apply-side-is-the-fork` stays (iii).
- `fnd-scaffold-at-standup-makes-remote-guards-cheap` [PROPOSED; dissolves the "ugly"
  objection] — the naive remote guard is a per-guard full standup (the check body cannot
  ride argv or a scaffold heredoc, so it streams from a controller scratch file), which
  pushes every argv-form pivot book into the multipart form. Instead, the apply-standup
  entry Dorc already makes for the witness lays that world's scaffold once (every guard
  body the plan needs there, materialised and cksum-verified under the owned scratch);
  each guard is then one plain entry with a path argument
  (`( ssh__enter web1 sh /dorc/<nonce>/guard-8.sh … ) || <the line>`), failing safe to the
  admin's line when stripped; final-verify removes it by manifest and narrates; a sever
  leaves disclosed residue. Nothing resident past the run, no process, no mediated admin
  line, one-line guards, logins multiplexed by the admin's own ssh configuration. Fusion's
  remaining buys are one login instead of N, a real per-leg rc, and landing once for pools.
  DEMOTED same day to a fallback (large bodies; wrapped guards): the primary shape is the
  next entry.
- `fnd-dot-tier-puts-guards-inside-the-carrier` [PROPOSED; the attention product's own
  argument for `lean-transports-are-dot-tier`] — under the lean a remote guard rides INSIDE
  what the admin's transit already carries, never a second tunnel: in the heredoc form the
  payload is a book fragment in the remote world and the guard is an ordinary in-sequence
  guard there (a front-lifted preamble at the payload's top defines the bodies); in the
  argv form the remainder is a one-line payload and the ssh oracle's fidelity claim
  (`dorc:sh -c "$*"`) licenses rewriting the join to `sh -c '( check ) || <join>'` —
  payload text through the declared fidelity function, the licensed side of `26M`'s
  inflection line, using only the edit classes `30P` enumerates for local plans. A transit
  whose whole payload elides elides whole (pure apart from the entry's vouched residue).
  Walked on a realistic book (heredoc to `web1`; local `scp`; heredoc back; `curl`): day-N
  default render shows the wall, the second tunnel with one interior guard, and the curl;
  the first tunnel vanishes; Dorc-owned apply connections = the standup witness, once.
  The two-tunnel guard render exists only WITHOUT the lean. Wrapped guards inside payloads
  inherit the existing snapshot-blob ugliness (not a transport cost).
- `ans-nonce-path-is-guaranteed-by-refuse` — the scaffold root is a controller literal,
  the nonce minted at plan time, the path carried in the plan's text; apply standup
  exclusive-creates exactly it or refuses pre-mutation; final-verify removes by manifest;
  stripped, a missing path fails the check and falls through. Guaranteed by the refuse,
  not the nonce.
- `ans-prompt-count-under-the-lean` — Dorc adds one login per world at probe (the
  standup) and one or two at apply (witness; final-verify when there is something to
  remove or narrate); guards add none; every elided transit REMOVES one of the admin's own
  logins. `260`'s `ControlMaster=no` pin still rides an existing master from the admin's
  config, so under `ControlMaster auto` Dorc's entries cost zero extra prompts; the
  ssh-ident / no-agent cohort pays the small constant. [human: an ick, not a nack.]
- `ack-transits-carry-guards-never-take-them` [human-stated 2026-09-05, "mostly
  convinced"; conductor-confirmed] — a transit is never the subject of a guard, only the
  carrier of guards; its own outcomes are run (as carrier), elide whole (everything it
  carries elided ⇒ pure apart from the entry's vouched residue), or wall (opaque
  remainder). The outer-guard form is DOMINATED on tunnel count (1 versus 1 converged,
  2 versus 1 diverged) and never chosen. Edges: TRANSFER verbs (`scp`/`rsync`) have no
  interior, so a guard is a tunnel per site — but authored inside the oracle's body on its
  vouch and declinable, never engine-minted; today they wall.
- `fnd-dorc-tunnel-count-is-o-worlds` — Dorc's own tunnels are O(distinct worlds): one
  standup per world at probe, one witness (plus optional final-verify) per world at apply;
  never O(transit sites). Bounded by twice the admin's own distinct-destination footprint.
  Bastion `MaxStartups` throttles the parallel standups onto the bounded retry
  (`drop-bastion-pacing`); a `MaxSessions 1` bastion is the contention cell, a link
  capability (`26O` §2's sibling-session column).
- `fnd-wrapper-self-effects-three-classes` [refag deliberately broken at the human's
  direction, 2026-09-05; from training data] — a middleman's own effects sort into:
  (1) RESIDUE no book line consumes (auth log, journal, `wtmp`/`lastlog`/`utmp`, history,
  sudo iolog, `doas` persist, a `ControlPersist` master, `will-cite`, `nohup.out`,
  `script`'s typescript); (2) STATE a later line consumes (`ssh` populating `known_hosts`
  under `accept-new`; `sudo -n` refreshing the credential timestamp; `pam_mkhomedir`;
  `pam_systemd`'s session scope; `ip netns exec`'s `/etc/netns` bind-mounts;
  `arch-chroot`'s mounts; `systemd-run --unit` transient units; `ssh -R` listeners);
  (3) PROVISIONING on first use (`gcloud compute ssh` keypair + metadata push; `az ssh`
  certificate; `mise exec` auto-install; `nix shell` build; `docker run` pull; `salt-ssh`
  thin dir; `mosh` server; `tsh` renewal). The taxonomy stands as AUTHORING GUIDANCE; the
  conductor's sharpening that the engine defines residue as class (1) is NACKED
  (`nack-residue-is-not-engine-defined`). Classes (2) and (3) are effects the AUTHOR
  chooses either to model (`is_converged` / `disturbs` on the wrapper family) or to
  document as residue for consuming admins; either is legal; the `ssh web1 true`
  connection dance elided above a first-contact `scp` is the shape that breaks once and
  gets attributed. Probe entries stay clean of (2)/(3) by construction (`sudo -n`; strict
  host-key checking is the default; `accept-new` is the admin's consent).
- `rul-transit-guard-is-dominated-by-construction` [PROPOSED restatement of
  `ack-transits-carry-guards-never-take-them`, obviously-true form] — a guard is a runtime
  elision; for wrapper W over payload P, skipping the whole line skips W's entry and all of
  P. P's runtime elision is available INSIDE the carrier at no extra connection, landing by
  the admin's own flags (no entry-siting vouch needed). W's entry, for any W that may have
  an entry form, reconciles its own class-(2)/(3) state on the way in, so running the line
  as carrier already performs W's reconciliation; skipping it saves one entry's residue
  while the outer check spends one entry. Never cheaper, never sounder, +1 tunnel diverged
  ⇒ never emitted. W's own convergence gates ONE plan-time decision, whole-line elision,
  where its establishes are ordinary facts walled by up-book mutators. Entry-less
  dispatchers (`at`, `sbatch`, `tmux new`, `nohup &`) are never entered; an authored
  line-level verdict there is a LOCAL check on the dispatcher's own state, no tunnel.
- `ex-transports-that-fall-from-elide-to-carrier` [the human's exercise] — real
  transports an admin wants elided that must fall back on an up-book happening, each
  falling to run-as-carrier-with-interior-guards, never an outer guard: `gcloud compute
  ssh` below a `project-info add-metadata ssh-keys=` rewrite; `docker run` below
  `docker system prune`; `mise exec` below `mise uninstall`; `ssh` itself below
  `ssh-keygen -R` or below a reimage that rotates the host key (the `known_hosts` fact
  keyed by the remote identity; the witness's territory in a local file's clothes).
- `aid-entry-residue-disclosure` [candidate aid row] — the plan header's authority
  disclosure states how many entries Dorc makes and that they appear as logins; Dorc's
  `sudo -n` probes extending the admin's sudo window gets its own sentence. Aid-plane,
  never a guard.
- `ack-shape-of-the-minimal-transport-model` [TYPED 2026-09-05, "I'm convinced, this is
  a good shape"] — the human acks the preceding three exchanges: the standup witness
  licensing bare-line elision; no fusion; transports carry guards and never take them; the
  three self-effect classes and the residue narrowing; the by-construction restatement.
- `rul-descent-implies-manipulation` [TYPED 2026-09-05, the human's words, near-verbatim]
  — "we need to manipulate anything we can recognize as sh, as sh; it is meaningless to
  analyze anything we can't modify; therefore the prior decision that we could ANALYZE
  inside descent-authorized argv/stdin bodies must also become a contracted ability to
  MANIPULATE them." A principled but big step: the same manipulation as the outer script,
  the same mental model for users; pitfalls are parsing/construction nits and cross-shell
  specifics; "worth it". Slightly owed from prior sittings, reached by a different path.
- `fnd-oracles-consume-stdin-they-never-introspect-it` [PROPOSED; answers
  `26N:park-oracles-knowing-stdin-stdout` YES] — oracles get no stdin/stdout introspection
  (sh cannot say heredoc-versus-pipe portably; the engine already knows); the engine runs
  the predict body UNDER the site's geometry and the body's own reads, redirects, and
  delegations ARE the claims (`cat` reads; `"$@"` inherits; `"$@" </dev/null` severs;
  `env dorc:sh -s "$@"` declares stdin-as-code). Engine obligations, exactly: (1) a
  per-site STDIN VALUE (heredoc bytes with expansions sited; a file name whose contents are
  world-state; the producer's PREDICTED stdout in a pipeline; ⊤); (2) stdin/stdout as a
  DIMENSION in the peel fold like ρ (inherited / severed / replaced-by-value / ⊤; innermost
  wins), crossing links on `26O`'s byte-clean-stdin capability; (3) checks fed by predicts,
  never by real producers (a pipeline guard duplicates only predicts; the whole pipeline
  runs on fall-through).
- `srv-stdin-stdout-idioms-four-patterns` [the survey] — (A) stdin-as-code: heredoc-to-ssh
  descends as a Host-world fragment; `curl | sh` is the canonical blind act (⊤ producer);
  foreign dialects (`psql -f -`, `nft -f -`) delegate; one `case` arm per reader.
  (B) stdin-as-data into a mutator (`| sudo tee`, `cat >f <<EOF`, `crontab -`,
  `uci batch`, `chpasswd`, `tar -xf -`): the verdict body reads stdin as the tool does and
  compares against a read; heredoc-WRITE sites elide-or-run, never guard (the guard would
  print the config twice; the emitter's heredoc refuse-home stands for that reason);
  declining is often right while the footprint is still the value (`tar -xf - -C /dst`
  disturbs the subtree; `chpasswd` declines forever). (C) stdout-out: existing law (status
  trichotomy; capture ⊤ until r26; redirect routing + binder; `predicts stdout`); `xargs`
  duplicates the guest ⇒ not an entry form; a member loop under a predicted producer, Open
  under ⊤; decline at v0. (D) wrapper stdin policy (`sudo` inherits; `ssh -n` severs;
  `docker exec` severs unless `-i`; `systemd-run` severs unless `--pipe`): the wrapper's
  predict spells it as the redirect on its guest, one per arm — turns the
  `docker exec c sh <<EOF` silent-EOF bug into a plan-time hint for free; the pty (`-t`) is
  the one non-sh-native piece and stays `26O`'s lend.
- Riders held for the human, not builders: heredoc/file values reach the probe lane under
  the argv-value rule (`rul-no-hopeful-transfer`: Must-identified world, the body
  Must-reads); secrets inside heredocs = the quarantined topic, flagged only; herestrings
  and process substitution = bash books; expansion siting is the hard parser work and
  stays at `310:seam-payload-forms`. Pitfall list for the quality bar: `$(cat)` strips
  trailing newlines; `sudo -S` partial-consumes; `use_pty` puts a pty on stdin's path;
  `<<-` tab stripping; CRLF.
- `ack-stdin-survey` [TYPED 2026-09-05] — the human acks the survey turn whole, with the
  four observations below.
- `ans-stdin-as-code-license-chain` [confirmed understanding] — three conditions: the
  innermost READER's matched arm delegates to `dorc:sh` in a stdin-reading shape (`-s`, or
  bare); every wrapper between the value and the reader INHERITS stdin on its matched arm
  (no `cat`/`read` before the guest, no `</dev/null`, no pipe into the guest) and the
  link's byte-clean-stdin capability holds (`-t` ⇒ ⊤); the descent lands in the reader's
  world (the peel's lends). `sh -c '…' <<EOF` is argv-code whose interior reads stdin as
  DATA. Descent = manipulation under `rul-descent-implies-manipulation`.
- `fnd-elide-not-guard-is-placement-not-safety` [PROPOSED; answers the human's ask] — no
  check found that is correct at probe yet unsafe in sequence; a guard is never less safe
  than elision, and a non-idempotent command WANTS the guard; the heredoc-write refusal
  was attention (duplication), not safety. The real class is check-tax/placement ("answer
  once in parallel at probe; never in-line forever"), which the matrix cannot say today
  (`kCONTRACT-RUNGS`: unmarked = guard-and-elide; opt-down reserved, none minted). Cheapest
  spelling, on the `test -t` precedent: a controller-decided lane fact the body tests
  (`[ "${DORC_LANE-}" = probe ] || return 2`, NAME STRAWMAN), folded statically so the
  engine emits RUN at guard sites, never a dead guard; engine-supplied value on the
  `DREP_V1` precedent; off-Dorc unset ⇒ decline. The first opt-DOWN spelling; loses value,
  never safety. Human-reserved: whether a variable at all.
- `prop-single-shot-capture-into-r31` [PROPOSED; the human: "feels backwards"; conductor
  agrees, +SURE better than yesterday's cut] — capture's single-shot half IS the
  placeholder species for one value: `x=$(producer)` binds at probe by running the
  producer's predict (a pure read with a `30D` stdout claim, lane 1), flows into Entered
  index-values as a value-plane object, is walled by any mutator between binding and use
  (`275`'s patrol), and is re-evaluated at apply standup so the witness can enter a
  destination named mid-book. The capture line itself always runs (eliding it unbinds the
  variable). None of `26B`'s concurrency holes apply to one binding at one probe. Pull it
  into r31 as the kernel lane's tail after lane 1's stdout claims; it is the Measured
  arm's first real consumer (retires `risk-measured-arm-built-blind`) and makes `26Oa`'s
  first line stop being ⊤. The REACTIVE half (re-probing as values arrive) stays r26; the
  `26C` §7 audit runs once, before the arm. `ROADMAP:arc-r26-revival`'s "capture starves
  until 30D" line re-reads accordingly.
- `park-xargs-as-stdin-member-loop` [parked exercise, not a sitting] — `xargs` is a
  member loop whose member list is a value (the producer's predicted lines) rather than a
  literal; it asks whether `30L`'s iteration axis can take a value-plane object, the same
  question capture asks of the Entered arm. Common; worth minor attention later.
- `nack-lane-decline-as-a-plan` [TYPED 2026-09-05; the human changed their mind three
  times while typing] — NACK on encoding `fnd-elide-not-guard-is-placement-not-safety`'s
  lane-decline spelling as a plan; it must not get built without thought. Lean recorded:
  an AUTHOR cannot reasonably say "don't pay this check at apply" — that decides, for the
  admin, a risk-class specific to their TOCTOU, timing, and architecture, uncomfortably
  close to an engineer declaring some mutation "residue" for everyone; the contract should
  stay LOW-RESOLUTION about timing with the admin; oracles are read-only, handle-or-decline
  the world as it stands, expect no particular state handed to them, and are maximally
  defensive; no case seen where ahead-of-time specialization is good for everyone else.
  The check-tax returns to `KNOBS:kPROBING` (engine/admin economics), never an authored
  surface.
- `nack-residue-is-not-engine-defined` [TYPED 2026-09-05] — NACK on the sharpening
  "residue = effects no book line consumes". Residue is an ORACLE-PRESENTED contract: not
  the oracle's contract to Dorc, but what an author must document for a consuming admin.
  The only control over residue in the ops world is mutual agreement: the engineer,
  elbow-deep in the tool and its weirdnesses, makes judgement calls and decides how, where,
  and when to surface them; the admin, who has not read how ssh handles world-writable
  tmpdirs mounted across two hosts, maybe reads that and decides whether the residue
  matters to them, or, far more likely and borderline the whole point of Dorc, IT BREAKS,
  because nobody could realistically have known. What Dorc adds is that it breaks ONCE, at
  minimal cost, with the tools to see which statement about the world was untrue or a poor
  assumption. Not a fully principled stance (plenty of dangers are invisible to Dorc); the
  consolation is that the set of dangers under Dorc is strictly SMALLER than under
  pretending to pay attention to all of it, and that Dorc's visible stability outweighs
  the inevitable blaming-of-Dorc for failures inherent to ops. Consequence: the engine
  never defines residue; the author models or documents; Dorc attributes.

## 15-state-of-play-at-close

2026-09-05, the sitting's fixpoint. Pointers only; content lives above.

- TYPED this sitting: `lean-witness-is-a-budgeted-backstop` · `lean-no-fail-fast-after-walk-away`
  · `ack-stdlib-walls-are-value` · `ack-less-elision-from-kernel-work-is-positive` ·
  `ack-pools-are-the-admins` · `ack-cross-world-wall-is-the-floor` ·
  `ack-shape-of-the-minimal-transport-model` (the standup witness licenses bare-line
  elision; no fusion; transits carry guards, never take them; the three self-effect classes
  with `27C`'s residue narrowed to class one) · `rul-descent-implies-manipulation` ·
  `ack-stdin-survey` · `nack-lane-decline-as-a-plan` · `lean-metaorchestration-consent-flag`
  (post-r31) · `lean-transports-are-dot-tier` (absorbed by the ruling).
- PROPOSED, shaped, each awaiting one typed line: `fnd-two-witnesses-at-artifact-boundaries`
  (amends `30W` §4) · `prop-single-shot-capture-into-r31` · `fnd-batching-key-is-syntactic-wall-is-rekeying`
  · `fnd-entered-arm-holds-a-value-plane-object` · `fnd-token-carriage-is-orthogonal-to-the-slot`
  · `fnd-scaffold-at-standup-makes-remote-guards-cheap` (fallback only) · the punt
  (`lean-punt-transport-to-its-own-round`) with the r31 re-cut it implies: Host entry and
  the measured-index witness move to the transport round; the `sudo`-case sink to lane 2;
  `$*`/IFS to lane 1; capture to lane 3's tail; the "elide when converged" acceptance line
  leaves with Host entry.
- OPEN or parked: the `30W` §10 sitting widened by `q3`/`q8`/`q10`/`q11` (W-2, still the
  one sitting before the re-key) · the argv-landing tracer model (W-3) · tenure's name ·
  the security review (W-E) · fleet-is-a-book (W-C) · the cohort posture (`q9`) · the
  sinkless record policy · live mode · the capability census (W-D) · `xargs` · `27C`'s
  entry-form rule text against `ack-entry-verbatim-cannot-hold`.
- DURABLE at close without further ack: this ledger; a scope pointer under
  `KNOBS:kBACKFLIPS`; the superseded pointer on `26N:park-oracles-knowing-stdin-stdout`;
  three `ANALYZER-NEEDS` rows (`an-stdin-value-per-site`, `an-stdin-stdout-peel-dimension`,
  `an-single-shot-capture` at O).
- ACKED IN DIRECTION 2026-09-05, sized by the sibling conductor who owns `310` and
  `ROADMAP`: the punt (`lean-punt-transport-to-its-own-round`) and some amount of
  single-shot capture into r31 (`prop-single-shot-capture-into-r31`). The human carries the
  re-cut to the sibling once no major design item blocks definitely-r31 material; this
  ledger never touches `310` or `ROADMAP`.
- NACKED 2026-09-05: the residue reading (`nack-residue-is-not-engine-defined`); no `27C`
  edit.
- DURABLE only after a typed ack: `ack-two-boundary-witness` — `30W` §4 rewritten in
  place (standup asserts and may withhold; final-verify re-reads for the receipt only; no
  assertion after a fired disturbance). Also awaiting a typed line, each explained in chat
  2026-09-05: `fnd-batching-key-is-syntactic-wall-is-rekeying` ·
  `fnd-entered-arm-holds-a-value-plane-object` · `fnd-token-carriage-is-orthogonal-to-the-slot`.
  The entry-form rule supersession waits for the transport round.
- `list-must-sits-before-kernel-work` [consolidated; the human asked where it was] —
  (W-2) the `30W` §10 sitting widened by `q3`/`q8`/`q10`/`q11`, whose retrofit-hostile
  outputs are exactly three shape rulings: the slot keyed by `KindId` (index-kinds are
  kinds; Host unblesses; vantage is a kind); index-values carrying provenance (Entered /
  Measured-placeholder / Fresh); worlds compared only through the chokepoint, never key
  equality. The six `30W` §10 rulings fall out of those. (W-3, short, foldable into W-2)
  whether wrapper-ness is a landing rather than a category, before lane 1's tracer work
  touches wrappers. Capture adds NO sitting (it rides W-2's placeholder species; the
  `26C` §7 audit is builder-tier). Everything else in `6-tier-two-work-units` is the
  transport round's and blocks nothing in r31.
