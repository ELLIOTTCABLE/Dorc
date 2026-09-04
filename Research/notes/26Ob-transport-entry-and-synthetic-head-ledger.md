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
> scratchpad, uncommitted; its adjudication is §4.

## §0 — one screen

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
vocabulary, two key shapes). What is on the human's table, sized and dated, is §6.

## §1 — the pin, restated and landed

- `the-two-halves-of-the-pin` — (a) `half-book-argv-fidelity`: the site's own argv
  (`ssh h apt-get install nginx`) must reach apt-get's oracle argparse as the argv the
  REMOTE receives; (b) `half-probe-inbound-carrier`: the engine's probe segment (inner
  oracle body plus scaffold) must physically cross the link, since nothing on the
  controller exists on the far side.
- `res-guest-is-engine-scaffolding` [PROPOSED; the load-bearing move, Q1] — the entry
  form stays `ssh__enter() { ssh -T -o BatchMode=yes "$@"; }`; the engine's `"$@"` is
  `sh -c '<constant wrapper>'` with the inner stream on the entry call's stdin, the
  redirect being engine scaffolding at the CALL site, never inside the authored form. The
  wrapper is designed to survive exactly one re-parse (`26N:mech-constant-wrapper`). The
  entered context gets its own scaffold and sink (`26O:rul-every-context-carries-its-own-scaffold`).
  No preamble reopened, no carve, no `"$@"` law change.
- `res-fidelity-is-a-predict-claim` [PROPOSED; Q2] — `26M:axis-dialect-and-fidelity`
  already directed that fidelity is an authored normalizer, not a token class. For ssh the
  normalizer is the predict body: `dorc:sh -c "$*"` (the far end evaluates the space-join
  of the remainder, in sh). The engine descends through the `274` eval'er lane it already
  has; a resolved-literal join re-parses; a ⊤ word is a `24T` hole and walls, naming the
  variable; a word carrying a space re-parses into two, which is the truth. The
  engine-side "simple words" charset is WITHDRAWN (`ack-charset-is-tool-knowledge`).
- `res-channel-behaviour-is-measured` [PROPOSED; aligned with `26N` §4] — byte-clean
  stdin, pty on the path, fs-write, tree, interpreter set, fd-over-2, returns: measured by
  the standup wrapper's first exchange, disclosed on the marker line, never declared. An
  author writes `-T` so the measurement comes out right, as they write `sudo -n`; the
  engine never learns what `-T` means.
- `res-lend-keying-is-a-planner-hint` [PROPOSED] — a mapped Host lend re-keys every
  fs cell, so an outer scratch path is a name in another world; the emission planner may
  ASSUME bytes must be shipped. Both error directions are safe for the carrier decision
  (a dangling path fails to can't-say; over-shipping is governed by
  `rul-no-hopeful-transfer` and is licensed here only because the shipped bytes are
  zero-influence public code). Measurement confirms; never a license.
- `status-of-the-pin` — resolution PROPOSED, gated on Q1 and Q2 (§5). `28Q` §9 item 11
  and `26N:park-re-parse-carve-explainer` are answered by this section once the human
  types on Q1/Q2; `310:unit-host-index-and-entry`'s gate text then rewrites to point here.

## §2 — rulings and acks taken this sitting (the ack-ledger)

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
  first because shell analysis has no ship-it-and-see fallback. Parked (§7-adjacent, W-G).
- `ack-public-versus-private-capabilities` [TYPED, non-critical] — the public capability
  set will freeze; the private one churns; verbose names plus an internal mapping.
- `ack-ssh-and-cp-engine-blocking-stdlib` [TYPED, shrug] — ssh and cp/cmp are
  engine-blocking stdlib needs; start small and focused.
- `ack-dollar-zero-per-shell-process` [ACKED rephrase] — `$0` is a property of a shell
  process, set by its invocation; sourcing, functions, subshells inherit; an exec of a new
  shell resets it to what that invocation names. "Never transits" was wrong (sudo is a
  transit that may not start a shell). Weird edges (`exec -a`, `#!` variance) ride
  `FORFEITS:forfeit-shell-parity-immunity-model`.
- `lean-fleet-is-a-book` [LEAN, human] — native fleet operations are dominated by the Big
  Boys; small users write the fleet into a file, i.e. sh; the population needing N targets
  on one command line is small and shrinking. A user-story sitting is owed (W-C).
- `lean-transport-head-as-plan-lines` [LEAN, human, "not married"] — if the head can be
  spelled honestly as sh, show it as plan lines; budget about two to three lines per host.
  Every function lowered into the plan is a CLI flag the admin never types.
- `lean-inline-heredoc-default` [LEAN, human; conductor-refined] — a single-file book
  stays a single-file plan: the body inline in a heredoc under the head; `dorc:scp` only
  for siblings the user themselves decomposed into.
- `defer-host-identity-mismatch-posture` [TYPED defer] — an edited destination is an
  explicit user act; warn/lint yes, deny maybe not; the human wants to think it through.
  Shelf thought (conductor, not pushed): withhold by default on witness mismatch, with a
  typed consent that proceeds by demoting every elision to its guard.

## §3 — proposals awaiting the human's word

- `prop-transport-wrapper-authored-surface` [PROPOSED] — a transport oracle authors at
  most three ordinary members: `cmd__enter` (`"$@"` verbatim; non-interactive by
  construction), `cmd__lend_map` (Host mapped to the destination; every other dimension
  enumerated), `cmd__predict` carrying the fidelity claim (`dorc:sh -c "$*"` for
  ssh-class; bare `"$@"` for exec-class). Nothing else. §8 unrolls it.
- `prop-entry-forms-only-subtract-interactivity` [PROPOSED] — an entry form may add
  non-interactivity and remove interactivity (`-T`, `BatchMode`, strip a site's `-t`);
  every other flag passes through verbatim. Bounds the one genuine probe/apply transport
  divergence (`26M:hole-probe-path-transport-divergence`). Author's argparse enforces;
  quality bar lints; attributed when wrong.
- `prop-cli-target-is-a-synthetic-head` [PROPOSED, built on the human's lean] — a CLI
  target is one Dorc-emitted head on an otherwise local plan; the head is the world
  boundary (`30W`); local-exec is the headless degenerate case. Spellings by route:
  file-backed `dorc:ssh -T web1 sh /dorc/plan.sh <<'DORC' … DORC` (body lands at that
  path; `$0` absolute; stdin EOF); in-memory `dorc:ssh -T web1 sh -c "$(cat <<'DORC' …
  DORC)"` (`$0` is `sh`; stdin free); `dorc:scp ./lib web1:/dorc/ &&` prefix only when
  siblings exist. `sh -s <<'EOF'` is REJECTED as a spelling: under it the body's stdin is
  the body, the founding bug written into the model. The head's spelling IS the route
  disclosure (`26N:mech-route-selection` commits the route at plan time).
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
  the r31 rule; do not weld it. Request/response channels consume the guest elsewhere and
  inline it; the general law is "the entry body consumes the guest exactly once and the
  fidelity claim says what evaluates it".
- `prop-plan-is-a-two-world-file` [PROPOSED; restates `26N:nack-ship-both-forms`] — the
  reviewed artifact is the whole controller-side plan; the head describes and drives the
  shipping; the shipped bytes are exactly the lines below the head. Headless forms
  (cloud-init payloads, `dorc-run` books) remain requested forms.

## §4 — the scout's inventory, adjudicated (maximum skepticism)

Seven session shapes the corpus already has, all absorbed by one sentence — an authored
line is never a session; the entry form is a session factory the engine invokes as often
as its protocol needs, each exchange with its own controller-minted identity; the only
lines that ARE sessions are the user's own at apply: probe segment per (host, context[,
iteration]) · probe retry (`attempt=`, discard wholesale) · reactive iterations (fresh
nonce, `iter=`; connection reuse invisible to the engine, ControlMaster pinned off) ·
probe concurrency (`142` batches; `sibling-session` a link capability) · apply-time
sibling session (live mode) · guard at a remote site (one call per guarded line) · the
user's line at apply (observed only).

Bites (real consequences): `bite-attribution-scope-goes-multi-in-r31` (Q6) ·
`bite-repeated-probing-tripwire-is-live` (`rul-repeated-probing-reviewed-before-design`
names multi-target; the human's review moment satisfies it if nothing builds first) ·
`bite-partition-becomes-keying` (`260` s3-1 structural isolation becomes `30W` keying;
`26B:seam-per-host-partition` foresaw it) · `bite-refusal-scope-under-nesting` (Q7) ·
`bite-bastion-pacing-is-unknowable` (§7) · `bite-nested-retry-origin` (§7) ·
`bite-dst-seam-needs-entry-chains` (§7).

Rewrites, not contradictions: `260:dec-26-hosts-spelling` (hosts enter the plan as one
Dorc-emitted head; the kOOB spirit holds) · `260:law-seam-1` (binds engine artifacts per
context; user lines were never engine units) · `an-loc-session` (controller-minted,
carried by the scaffold) · the `26Lb` transit-species musing (confirmed and resolved by
the fidelity claim) · the `26Lb` dispatch-elision finding (a scope note: the
whole-payload dispatch form is a Complex Dispatch with a whole-line vouch and no descent;
the argv form descends through the ssh oracle; both coexist). Scout misattribution
corrected: that finding lives in `26Lb` (brainstorm-tier), not `27Xf`.

## §5 — TIER 1: open questions that shape the core semantic (kernel-affecting first)

Each: the question · why it touches the analysis kernel · what it gates · the conductor's
lean · size of the ruling.

- **Q1 `q-guest-is-engine-scaffolding`** — may an authored entry form's guest slot carry
  engine scaffolding (the constant wrapper) rather than an oracle function invocation?
  Kernel: no (emission/probe composition), but it is the pin itself and every other
  answer rests on it; `rul-only-oracle-bytes-ship` and `probe-composition-walls` read as
  compatible (scaffolding plus oracle bytes; entry through the authored form). Gates
  `310:unit-host-index-and-entry`. Lean: yes. Size: small.
- **Q2 `q-transport-fidelity-via-predict`** — may a wrapper's predict be eval'er-shaped
  (`dorc:sh -c "$*"`) rather than peel-shaped (`"$@"` in command position), with the
  code operand the joined site remainder? Kernel: YES — wrapper detection widens from the
  `273` tautology to "the argument slot reaches an evaluator"; the value plane must model
  `$*` (join under IFS) as concat-of-constants on the `24T` ladder; dual-peel coherence
  (`273` §5) must accept `"$*"`-in-code-operand as the tail position; ρ under a bare
  `dorc:sh` is ⊤ for the remote (correct; harmless for the argv form since every expansion
  happens on the controller). Gates `unit-host-index-and-entry`; touches
  `oracle`'s lift and `analysis::value`. Lean: yes; it is the `24T`/`274` machinery
  applied once more. Size: medium.
- **Q3 `q-transport-lend-values`** — what does a transport wrapper's `lend_map` emit for
  dimensions it cannot name from argv? The login user of `ssh web1` (no `user@`) is host
  state; a ⊤-valued mapped lend identifies with nothing, so two `ssh web1` sites would
  not co-refer and an establish on one could never license the other — a real value loss.
  Likewise fs-view and netns under a new Host are fresh per-Host entities, and
  `27C:rul-dimension-owned-compose-ops`'s caller-relative fs-view composition must compose
  under a Host reset. Kernel: YES — `30W` item 1 (the context slot as a product over
  index-kinds), item 2 (the Host binder's measured identity), the compose ops. Lean: a
  ⊤-valued entered index whose kind has a measured witness resolves to the witness after
  standup (the login user is part of the Host binder's measured identity; substrate
  dimensions are per-Host entities minted by the binder), so all sites through one entry
  share a key. Gates `unit-context-slot-product` design. Size: medium-to-large; wants
  the `30W` §10 sitting.
- **Q4 `q-argv-form-carve-typing`** — the "simple words are identity, else decline" carve
  `310`/`26N` §7 lean on. DISSOLVES if Q2 rules yes (the re-parse is derived, and what
  declines is what the `24T` ladder walls). If Q2 rules no, this returns as a charset the
  human already refused. Size: nil or small.
- **Q5 `q-fan-out-consent-scope`** — `28Q` §3's typed carve gates probe entry into
  book-mentioned hosts on explicit consent. Does it also gate guard-at-apply entry (a
  guard opens a session the user's own line would have opened anyway)? Kernel: settle/plan
  (guard licensing). Lean: consent gates probe entry only; apply-time guard entry rides the
  apply's own license. Gates the same unit. Size: small. Ties to
  `design-world-scope-surface`.
- **Q6 `q-attribution-scope-goes-multi-in-r31`** — `rul-attribution-is-controller-minted`
  names its re-entry trigger (a second scope representable in one run); Host entry in
  r31 fires it. Ruling needed: that scope-carrying becomes scope-checking in the
  Host-entry unit, not at stage-iii; each exchange bakes (nonce, attempt, iter, context
  literal); the deframer refuses frames whose scope is not the exchange it opened
  (`26O:rul-frames-are-written-at-minting` already says CHECK). Kernel: intake. Lean:
  yes, in r31. Size: small ruling, medium build.
- **Q7 `q-refusal-scope-under-nested-contexts`** — `309`'s whole-target-down plus its
  explicit punt on continued probing under refusal: when an entered context's intake is
  refused, is "the target" that world or the whole plan? For a one-line synthetic head
  they coincide; for a pivot book they do not. Kernel: intake/settle. Lean: none offered;
  security-adjacent. Routed to the opaque review (W-E). Size: medium.

## §6 — TIER 2: work-units for later sittings (what is on the human's table, and when)

| unit | contents | size | needed by | blocks |
|---|---|---|---|---|
| **W-1 Tier-1 rulings** | Q1, Q2, Q5, Q6 typed; Q4 dissolves with Q2 | one short sitting | before `310:unit-host-index-and-entry` | r31 lane 3's tail |
| **W-2 the `30W` §10 sitting, widened by Q3** | the six `30W` rulings plus `q-transport-lend-values` | one sitting | before `310:unit-context-slot-product` | r31 lane 3 entirely (already the gate) |
| **W-A the synthetic head** | the head grammar per route; the line-budget fence as the one exception to `27C:route-conditional-tail`'s NACK; the head's reason annotation and `dorc why` address; headless faces stay requested forms; `nack-ship-both-forms` restated; fusion boundaries at reingest | one medium sitting | before `310:unit-delivery-shape` emits a plan head; may ship read-only first | r31 lane 2's plan emission |
| **W-B editable transport lines** | `defer-host-identity-mismatch-posture` (yours); edited flags versus refag (the measured-standup floor); the `30W` witness becomes load-bearing | your thought, then a short sitting | before the head is admin-editable; not before | nothing in r31 if the head ships read-only |
| **W-C the multi-target user story** | `lean-fleet-is-a-book`; three candidate users (one-box homelabber; conductor-book writer; cron drift monitor) and one test (does any need N targets on one command line); what `260` retires (`--hosts`, width cap as a flag, aggregate exit code); the `--fan-out` consent spelling as the central multi-host UX; overlaps `design-world-scope-surface` | one large sitting | before any multi-host or `arc-r26-revival` work; not before | nothing in r31 single-target |
| **W-D the capability system** | `prop-capabilities-one-vocabulary-two-keys`; public/private namespace; engine-protocol demands beside byte demands; `26N:open-census-needs-value-plane` (already the next `26N` sitting) | one large sitting | before `arc-host-capabilities`; r31 only reserves the seat | nothing in r31 |
| **W-E the security review's inputs** | outer scratch holds the inner stream; security-floor flags move to stdlib; Q7; Q6's forged-frame cell; edited destination; hostile inner host | the review's | after the core plan, before threading (human-typed order) | threading this note into the corpus |
| **W-F transport-tier mechanics** | probe-artifact flatness versus nesting; inbound armouring per link; bastion as two entries; nested retry origin; pacing per destination; the `30X` entry-chain seam column | builder-tier with small conductor rulings | at `310:unit-delivery-shape` build; return to LAST | nothing |
| **W-G stdlib prerequisites** | the ssh oracle (entry, lend_map, fidelity) and cp/cmp, small and focused; `prop-entry-forms-only-subtract-interactivity` as an authoring rule; `park-oracle-value-manipulation`; `park-oracles-knowing-stdin` | authoring, mostly | before hand-authoring the trial oracles | `arc-block-stdlib` |

Reading the table: r31's kernel lane is unblocked by W-1 and W-2 (about two sittings) plus
the review; W-A is a third, medium sitting before the plan head is emitted; W-B, W-C, W-D
are separate sittings that block nothing in r31 for a single target.

## §7 — dropped as local (decide at build; will not be re-raised as design)

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
  already homed at `26M:q-entry-economics`.

## §8 — the transport wrapper's authored surface, unrolled (the sitting's core angle)

The worked site, argv form, from the `26Oa` shape without its payload:

```sh
ssh -J bastion "admin@$ip" sudo apt-get install -y nginx     # STRAWMAN site; $ip literal above
```

The three authored members (all STRAWMAN spellings; the member names are settled `27C`/`273`
names):

```sh
# dorc-lang/v0.2 — the stdlib ssh oracle, minimal
ssh__lend_map() {
   while :; do case "${1-}" in
      -J|-p|-i|-F|-l|-o) shift 2 ;;      # transport flags: pass, never interpret
      -t|-T|-*)          shift ;;
      *) break ;;
   esac; done
   printf '%s\n' "$1"   : lends host      # mapped: the destination operand is the Host value
   shift
   "$@"                                   # the peel boundary
}
ssh__predict() {
   while :; do case "${1-}" in            # same peel
      -J|-p|-i|-F|-l|-o) shift 2 ;;
      -t|-T|-*)          shift ;;
      *) break ;;
   esac; done
   shift                                  # the destination
   dorc:sh -c "$*"                        # the fidelity claim: the far end evaluates the join, in sh
}
ssh__enter() {
   ssh -T -o BatchMode=yes "$@"           # non-interactive by construction; site flags pass through
}
```

The derivation chain the engine runs, each step an existing mechanism:

1. `step-peel` — the site's argv flows through `ssh__lend_map`'s argparse
   (`rul-argv-flows-bytes-do-not`); the Host index is set to the resolved destination
   (`30W` §5, an entered index); every other dimension's value is Q3's business.
2. `step-fidelity` — the same argv through `ssh__predict` reaches `dorc:sh -c "$*"`: an
   eval'er site (`274`) whose code operand is the space-join of the remainder. With every
   word literal it is a resolved literal on the `24T` ladder; the engine re-parses it as
   sh and obtains the remote argv: `sudo apt-get install -y nginx`. A ⊤ word (an
   unquoted, unresolved `$pkg`) is a syntax-position hole ⇒ site-local wall naming the
   variable (`24T:pin3`). A word with a space re-parses into two, faithfully.
3. `step-inner-descent` — the remote argv is analysed in the Host=web1 world as if it were
   a book line there: `sudo` peels (user mapped to root, fs-view full lend of the REMOTE
   view), `apt-get` routes to its oracle; its cells key by (Host=web1, user=root, …).
4. `step-probe-entry` — the probe enters the denoted context by composing entry forms
   outermost-first (`27C` §3): `ssh__enter -J bastion admin@web1 <engine guest>`, where the
   engine guest is `sh -c '<constant wrapper>'` and the inner segment rides the call's
   stdin from the outer scratch; inside, the wrapper materialises its own scaffold and
   runs `sudo__enter -n <the apt-get verdict body>`. Every executing byte is either
   oracle-authored or engine-owned scaffolding; the admin's bytes never ship.
5. `step-standup-measures` — the wrapper's first exchange measures the link (byte-clean
   stdin, pty on path) and the context (fs-write, interpreters), discloses them on the
   marker; the engine's stated needs (no-pty for probing; a lane control owns) are joined
   against them; a missing need declines entry for that context ⇒ guard/run.
6. `step-fact-keying` — the measurement returns framed with the controller-baked context
   literal (Q6 checks it); the fact lands keyed (Host=web1, user=root, …); its
   `establish` reaches downstream sites in the same world by ordinary reach.
7. `step-apply` — the user's line runs as written (apply never re-transports). If the site
   cannot elide, the guard is `( ssh__enter -J bastion admin@web1 <carrier with
   sudo__enter -n <check>> ) || ssh -J bastion "admin@$ip" sudo apt-get install -y nginx`
   — the check in the denoted context via the same entry chain, the fall-through verbatim.
   One session per guarded line (the check-tax, `26M:q-entry-economics`).
8. `step-what-the-engine-never-did` — it never parsed an ssh flag, never knew ssh joins
   with spaces, never knew `-T` means no pty, never decided which shell the far end runs
   (the author's `sh` in `dorc:sh -c "$*"` is that claim, attributed), never applied a
   charset. A mosh oracle differs by one word; a `docker exec` oracle's predict says
   `"$@"` instead of the join; a serial console has no guest slot and lands on the paste
   rung, honestly.

What it costs, said once: the ssh oracle is engine-blocking (`ack-ssh-and-cp-engine-blocking-stdlib`);
the value plane must model `$*`; wrapper detection widens (Q2); `lend_map` needs an answer
for host-state dimensions (Q3); and every exchange is a session the user never wrote (§4's
sentence), which is the honest shape of a transport.

## §9 — register state

Touched this sitting: `spike/CLAUDE.md` (`rul-no-hopeful-transfer`, host evidence &
controller attribution). Owed at threading (after the review; not done): `ROADMAP.md`
(the `28Q:pin-ssh-entry-shape` gate text, `arc-block-stdlib`'s ssh re-grade, the
`sit-multi-target-user-story` row) · `plans/310` (lane 3's gate; the head in lane 2) ·
`plans/28Q` §9 item 11 status · `notes/26N` §10 `park-re-parse-carve-explainer` (answered
here) · `plans/26O` §7/§10 pointers · `KNOBS:kBOOT`/`kCOMMS` pointers ·
`ANALYZER-NEEDS` (a fidelity-claim row; the `$*` value-plane need) · `ORACLE_PROVIDES`
(the entry/fidelity shapes under provides-context-entry and provides-payload-declaration)
· `TODO-ADDTL` (the user-story porch item). The scout inventory stays in the scratchpad
until something in it is cited as load-bearing.
