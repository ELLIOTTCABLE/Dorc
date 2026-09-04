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

## 6-tier-two-work-units

What is on the human's table, and when. Under `lean-punt-transport-to-its-own-round`,
"needed by" for the transport-round items reads "at that round's open".

| unit | contents | size | needed by | blocks |
|---|---|---|---|---|
| **W-0 mint the transport round** | ack the punt; name the round; move the candidates in `11-r31-re-homing-candidates`; leave r31 the kernel-critical residue | one ack plus a `ROADMAP`/`310` edit | before r31 opens | r31's shape |
| **W-1 Tier-1 small rulings** | `q1`, `q5`, `q6` typed; `q4` dissolves with `q2` | one short sitting | at the transport round's open | its Host-entry unit |
| **W-2 the `30W` §10 sitting, widened by `q3`** | the six `30W` rulings plus `10-q3-and-entailments` | one large sitting | before any context-slot product build (r31 or the transport round) | the re-key |
| **W-3 the argv-landing tracer model** | `lean-replace-wrapper-detection-with-argv-landing` ruled or nacked; `q2` under it; `$*`/IFS | one short sitting, then lane 1 | before lane 1's tracer work touches wrappers | r31 lane 1's wrapper-adjacent units |
| **W-A the synthetic head** | the head grammar per route; the line-budget fence as the one exception to `27C:route-conditional-tail`'s NACK; the head's reason annotation and `dorc why` address; headless faces stay requested forms; `nack-ship-both-forms` restated; fusion boundaries at reingest | one medium sitting | before a plan head is emitted (transport round) | plan emission there |
| **W-B editable transport lines** | `defer-host-identity-mismatch-posture` (yours); edited flags versus refag (the measured-standup floor); the `30W` witness becomes load-bearing | your thought, then a short sitting | before the head is admin-editable | nothing before that |
| **W-C the multi-target user story** | `lean-fleet-is-a-book`; three candidate users (one-box homelabber; conductor-book writer; cron drift monitor) and one test (does any need N targets on one command line); what `260` retires (`--hosts`, width cap as a flag, aggregate exit code); the `--fan-out` consent spelling as the central multi-host UX; overlaps `design-world-scope-surface` | one large sitting | before any multi-host work | nothing single-target |
| **W-D the capability system** | `prop-capabilities-one-vocabulary-two-keys`; public/private namespace; engine-protocol demands beside byte demands; `26N:open-census-needs-value-plane` | one large sitting | before `arc-host-capabilities` | nothing in r31 |
| **W-E the security review's inputs** | outer scratch holds the inner stream; security-floor flags move to stdlib; `q7`; `q6`'s forged-frame cell; edited destination; hostile inner host; the fused-session protocol if W-M rules it | the review's | after the core plan, before threading (human-typed order) | threading this note |
| **W-F transport-tier mechanics** | probe-artifact flatness versus nesting; inbound armouring per link; bastion as two entries; nested retry origin; pacing per destination; the `30X` entry-chain seam column | builder-tier with small conductor rulings | at the delivery build; return to LAST | nothing |
| **W-G stdlib prerequisites** | the ssh oracle (entry, lend_map, fidelity) and cp/cmp, small and focused; `prop-entry-forms-only-subtract-interactivity` as an authoring rule; `park-oracle-value-manipulation`; `park-oracles-knowing-stdin` | authoring, mostly | before hand-authoring the trial oracles | `arc-block-stdlib` |
| **W-M the M:N channel topology** | `10-q3-and-entailments` third hard thing: the batching key; the transit schedule (CFG-with-transits → CFG-with-sync-points); the apply-side fork (engine-emitted lines only · an admin opt-in spelling · probe-only) which reopens `26M`'s tabled apply-side transport in a narrow form; nesting, shell concurrency, reach failures | one large sitting | the transport round, after W-2 | that round's executor-shaped work |

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
- `res-the-destination-denotes-a-resolution-cell` [PROPOSED] — a destination string is a
  NAME; what it denotes at a line is the value of a controller-world cell — the Host
  kind's resolution of that name (STRAWMAN coordinate `sm.dorc.Host:host1.widget.tld@resolution`)
  — and that cell has a BACKING the Host kind's owner declares in sh, exactly as any
  other kind's stores are declared (`kind__state_stored_in`): the ssh client's
  configuration files, the hosts file, the resolver and name-service switch
  configuration, and the relevant environment. Silence declares nothing, so the backing is
  ⊤ and EVERY running mutator between two transit sites walls the resolution — the safe
  default. A modelled mutator with a footprint disjoint from the declared backing spares
  it only under the survival flag, as every other spared cell (`kSURVIVAL-trusted`;
  `rul-flag-is-razor-residue`). An opaque mutator is a total wall as everywhere. A
  converged day has no running mutator between the transits, so the resolution is
  unwalled by construction (an elided command casts no wall — the stage-3 story).

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
