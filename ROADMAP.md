# ROADMAP — upcoming work, and where each piece was designed

> AI-authored, AI-maintained. The human moves rows between sections by *typing* an ack in
> chat; nothing else does. Siblings: `TODO.md` (the human's own list, their voice) ·
> `TODO-ADDTL.md` (the porch — what models think the human should look at, unacked, owed by
> nobody) · `Research/LIVING_STATUS.md` (the state of in-flight work) · `Research/README.md`
> (the map of documents) · `SLUGS.md` (grep it for any slug).
>
> Rules. **Scheduled** holds only what the human has typed "next" on, and stays tiny —
> promises rot if not dispatched within days. **Owed** holds work the human has acked as
> real but not scheduled; each row points at its design-of-record and says what gates it.
> When a later design changes a row's shape, the row gets a one-line pointer to that design,
> never a paraphrase of it. When a row lands, delete it — the account moves to the README
> round-map; this file is never a work-log. `rNN:` xfail horizons are mechanical reminders
> of *when to be bothered by the machinery*, never promises: a row here may hold a pin's
> intent well enough to retire the pin, and no new horizon is minted to mirror a row.

## Scheduled

- **`arc-test-architecture-rebuild`** — IN FLIGHT. `notes/30X` §11 (lanes A–D, serial,
  checkpoints after A and C); conductor branch `ai/r30-30X-test-infra-decruft-conductor`,
  ledger `notes/30Xa`. Lane A building. The steering edits owed at its close are listed at
  the tail of `30X` §11. Live state: `LIVING_STATUS.md`.
- **`ceremony-r30-close`** — the human's, per `30O:the-schedule`: the human-run gate →
  `gate:arc` → the `CURRENT_ROUND` bump (`internal-tooling/src/xfail.rs`) → the prose queue
  (`mise run prose:census`) → ff `ai/main`. ~SUSPECT it lands after the rebuild folds, since
  the rebuild runs inside r30.
- **`round-r31-language-and-kernel`** — ANNOUNCED 2026-09-02, scope NOT ruled. The human's
  framing: a second language/kernel round touching very fundamental language concepts —
  everything invasive/kernel-related that positions Dorc for pivot-books, including some
  correctness/semantic changes in adjacent components. The candidates below are the
  conductor's synthesis with a lean each; **nothing is in until the human types it**.

  Language-surface designs banked in r30, unbuilt:
  - `cand-index-kinds-and-pivot-books` — `plans/30W`: its six §10 rulings first
    (`rule-unreserve-host-as-entered-index` collides with `271:rul-axis-vocabulary-v1`;
    then the incarnation-invariance razor), then build items 1–4 (context-slot
    generalization AFTER the `FactKey.context` as-built audit — retrofit-hostile; the
    index-kind stdlib; re-keying + expected-sever; the witness on `30S` rails); items 5–7
    ride the identity tier and the stdlib arc. Sitting record `notes/26M` (its live queue).
    Lean: IN — the round's spine.
  - `cand-finished-definitions` — `plans/30U`: the `disturbance_reaches` respell, the
    `unrelated` answer + settle gate, `nothing-else` record recognition, the reds; §10 is
    pending a successor's rewrite against the tree; register row `an-kind-reach` stands at
    spec-tier. Lean: IN.
  - `cand-authored-file-semantics` — `plans/30T` §10: `comp-routing-locator` →
    `comp-fs-binder-member` → `comp-claim-consumption`; `comp-backing-detector`
    (independent, every domain); `comp-identity-tier` (shares plumbing with
    `30P:mech-two-standups`); `comp-artifact-injectivity`; later
    `comp-channel-relative-speech` and `comp-content-establishment` (the write-if-changed
    idiom's own elision; the FORFEITS capture). Lean: IN for the first four, the rest by
    energy. (This supersedes the older "write-elision needs a vouch-holder" row: `30T`
    rules who authors the compare.)
  - `cand-environment-identity` — `plans/30S`: value-carried context keying + the
    pin-or-sever floor; `30S:seq-stdlib-gates-on-env-identity`; four Rust pins at horizon
    `r31` + the `env30-book-exports-meet-oracle-envelopes` loom. Lean: IN (`30W`'s witness
    rides these rails).
  - `cand-elision-regions` — `plans/30L`: design current; as-built is still the
    all-or-nothing CALL disposition (a `main "$@"` book forfeits the attention product);
    stage 1 (census + red battery) is measurement and runnable early; §14's sequencing
    (before artifact forms and golden promotion) is overtaken by the landed r30 artifact
    lanes — re-verify at pickup. Lean: IN.
  - `cand-payload-declaration-speech-act` — `notes/26M` queue item 2: the
    carrier-geometry survey BEFORE any spelling; fidelity as an authored pre-applied
    normalizer (ruled); custody-over-there; inline render whole-or-nothing (lean). Holes
    cardinal if missed: payload identity joining the `26C` Question identity;
    expansion-siting across the payload boundary; entry recipes carrying transport flags
    or declining. Lean: sitting IN; build by what it yields.
  - `cand-local-exec-mode` — local-exec as a supported mode (zero tests; also on `TODO.md`);
    the prerequisite of the personal-target path in `26M`. Lean: IN, first.
  - `cand-wait-ratification` — `26K:sit-wall-transparent-delay-loops` + `26Lb` §2's
    wait-is-a-guard reframe; the `StatusIterated` converged-at-entry carve is a
    license-widening — human-flag before any build. Lean: sitting only.

  Correctness/semantic repairs in adjacent components:
  - `cand-taught-decline-idiom` — `[ "$2" = "" ] || return 2` after a predict's entity bind
    renders Run with no diagnostic while the `if …; fi` spelling elides (builder-bisected,
    the `30S` pin lane); it is USER_STORY's taught first-hour shape. Adjudicate before any
    real verdict authoring. Lean: IN, first; small.
  - `cand-render-spine-feedback` — `30Ng:attn-render-refusal-feeds-the-spine`
    (human-flagged, unruled): render-refusal is a mutative-difference-causing decision and
    must feed back into the spine; the flagged lean is a rerun-to-fixpoint stage keeping the
    spine pure. Needs a sitting with a termination argument; hardens as artifact forms
    accrete. Lean: sitting IN.
  - `cand-acquired-source-weld` — `30R`'s acquired-source restructuring carries a
    temporary ack: one ordered role-carrying source vector, ordinal = table position. Weld or
    re-cut against `30I`'s occurrence account (repeated/multi-role/multi-target sources).
    Lean: IN; a small ruling.
  - `cand-model-local-in-tracers` — the ruled dialect is POSIX + `local`, yet the
    builtin-deny (`26J`) ⊤-degrades it and walls any realistic oracle body; kin: book-side
    `set --`. Lean: IN; small.
  - `cand-command-v-load-model` — `30I:pin-command-v-load-model`: which shell categories
    and floor variations the load model represents (evidence `notes/30Ic`). Lean: glance.
  - `cand-lane-riders` — residue that rides the next lane touching its seat, never its own
    lane: `30O:register-and-steering-debt` (the two-seat explicitness predicate · the
    frameless acquisition exposure · the `30Kb` residue · `doctor` over drvfs) · `307` §6c
    (wall narratives for non-leaf walls; delete `render-heredoc-refused`, bank
    guards-at-redirected-sites) · the `rc-vs-genkill-permanent-law` steering re-home.
  - `cand-r31-horizon-glance` — the pins already carrying an `r31` horizon
    (`mise run xfail:census`): `r31:kernel-punt-glance` (five; FORFEITS rows
    `forfeit-value-narrowing-by-test` · `forfeit-file-content-facts-from-exact-checks` ·
    `forfeit-content-establishment-by-known-write` · `forfeit-shell-parity-immunity-model`)
    · `r31:book-load-acceptance` (seven load-operand pins; `forfeit-book-dynamic-load-analysis`)
    · `r31:closure-custody` · the unscheduled `end-of-r31` set. Per pin: green it, retire it
    into a row here, or re-horizon — reminders, not scope.

  Adjacent, lean OUT unless pulled in: `arc-predict-contract` (below; its own trigger
  includes "before a hand-authored real-oracle trial", which the pivot target implies —
  ask) · `FORFEITS:forfeit-plain-sh-inclusion-analysis` (`30O` calls it the next
  language-surface round's entry point; it is not pivot-shaped) · `28Q:stage-iii-world-scopes`
  · the r26 revival.

## Owed, unscheduled

Human-acked as real; no date. Arcs first, then the sittings and rulings the human owes.

- **`arc-block-stdlib`** — zero non-fixture oracles; human-ruled 2026-07-27 as not blocking
  (scrappy hand-written oracles are part of the experiment). Gates, all engineering: the
  taught-decline adjudication · `arc-predict-contract` · `30S:seq-stdlib-gates-on-env-identity`
  · the `27Q` preconditions; the dialect-reach gate is RULED (`30Q` §5g). Pivot priority:
  transit verbs first (`26K` concl-boot-books), then timeout · ssh-keygen · curl · getent.
  On-ramp `notes/27Q`; authoring trap `24U` §2 (converged ≠ no-op adequacy).
- **`arc-predict-contract`** — `notes/30D` + `plans/30J`, design RULED, build
  deadline-triggered (`30J` §10: the earliest of the stdlib revival, a survival-authoring
  trial, third-party publication); `30D`'s return-2 drop still owes its §4.3
  typed-consequence ruling. Build nothing opportunistically before the trigger.
- **`arc-r26-revival`** (reactive/capture + multi-host) — `26B`/`26C` + `260`–`262`; after
  r30. Entry gates: the un-ruled R2-entry question (freeze-in-artifact vs
  structure-preserving folds) · `26C` §1's stability confirmation · a FRESH `26C` §7
  quiet-welding audit against the current kernel before R0 (`26C` verified the r27 tip only;
  NB `30L`'s iteration axis = loop-member, `26C`'s `iter=` = probe-iteration) ·
  `26B:need-quiescence-witness-at-mint` · `26B:split-semantic-versus-concurrency-holes` ·
  `26B:ask-trial-counts-capture-walls` (size from real `dorc why` critique first). Capture
  starves until `30D` stdout claims exist — correct by design. The epoch×capture boundary
  (`26L:finding-future-values-remain-future`) rides here; its single-shot half is r31's.
  `notes/26L`/`26Lb` are exploration-tier — cite no commitments from them.
- **`28Q:stage-iii-world-scopes`** — gated on `design-world-scope-surface`,
  `rule-incarnation-continuity-semantics` (below), and the ssh oracle (⇐ stdlib).
- **`arc-why-surface-deferred`** — `notes/30V` §6: curated tiers/registers/ranking
  (post-user), the staged-search CLI, `dorc apply --why`, `--live` re-probe, site-granularity
  (held on the frozen kernel), the graph-drawing library, the `dorc r` router, the record
  register (resolves against real output under a human eye, never more strawmen).
  Report-only kernel re-derivation is its own authorized round (`30R`).
- **`arc-durable-account-export`** — built and DISABLED (`plan::whylog::ACCOUNT_EXPORT`);
  `rul-durable-contents-reviewed-before-design` clears first; then rebuild against the
  receipt durable (`30Rk:the-account-export-died-with-its-lane`; pin
  `p-x-durable-account-export-is-enabled` parked Reserved).
- **`design-mh2-version-layer`** — `.claude/research/versioning-mh2/`, `270` §4.

  > on MH2: versioning (the simpler version of the concept, focused down to 'package-as-a-type-needing-special-attention, multi-providers, etc' ... and 'mapping oracle-written-for vs being-executed-on') needs close care, but it feels very deferrable compared to some of the critical core analyzer-design things that are affected by the "sh spelling" issues. continuing to defer.

- **`lane-flux-engine-hardening`** — UNSCHEDULED (human 2026-08-21); `300:lane-flux-engine-hardening`;
  any typesystem change it needs rides the Aeneas-prep facade work, never a Flux lane.
- **`rc-vs-genkill-permanent-law`** — revisit, not now, never strike (human 2026-08-23);
  `spike/CLAUDE.md rul-rc-reaches-genkill-only-through-decisions` still expects it from the
  influence round, which `30Qd` established will not produce it.
- **FORFEITS captures with a stated trigger** — `forfeit-certifier-trip-evicts-elisions`
  (trips observed in the field) · `forfeit-plain-sh-inclusion-analysis` (the splice and
  single-stream paste tiers) · `forfeit-book-dynamic-load-analysis` (glob loads;
  `30Q:dec-globs-unscheduled-this-round`).

Sittings and rulings the human owes:

- `design-world-scope-surface` — how authors describe context begin/end, transit respelling,
  ssh re-parsing, fan-out consent; on-ramp `28Q` §§3/8/10 + pin 11, `27C`, `24T`.
- `rule-incarnation-continuity-semantics` — when a recreated context is continuous with its
  predecessor (`28Q:res-incarnation-correlation-door`); `28Q` §3 + pin 4.
- the six `30W` §10 rulings, unless r31's opening sitting takes them.
- `tc-dollar-zero-is-script-anchored` (the flagship red `load30-point-havoc-and-script-relative`)
  · the hoist ACTION's two calls `tc-hoisted-dot-line-spelling` and
  `tc-t2-is-narrower-than-the-ladder-says`.
- `30J` §6.5 cross-family registration · `28Q:pin-closure-membership-and-diamond`.
- `26K:sit-stdin-copy-exec-amendment` — copy-then-exec vs pipe, before any `260` §5 change;
  the secret-push line is specimen two.
- the pivot-book cross-host-influence security review (opaque-review tier; OWED per `26M`
  `ack-cross-host-facts-scoping`, out of scope for the design work).
- `records-eight-emitter-decision` — the eight `records-*` codes with no production emitter:
  delete under no-compat, or wire into the records revival (`28L:rul-no-emitter-codes-are-blocked-rows`, `28N` §2).
- the why-surface close's human items (`notes/30Va`): the five orphan arrangement rows'
  deletion word (`mise run prose:orphans`) · `tc-address-refusal-is-a-datum-not-a-diagnostic`
  (held; the conductor recommends splitting no-match/row from multi-match/ask) · the
  soft-acked root-identity fold.
- the small-sittings queue: floors-ratification (`27U` §7) · decline-class starter-set
  (`27W` §0) · C8 operand-value display (`27U` §7) · prose-register schema (`282` §10) ·
  lint tc-leans (`27S` §5, `27T`) · `Consented`-knowability at first render (`28F`) ·
  why-carries-risk-flag + apply-header-vs-byte-floor (`28I`) · the `30M` wall-narration
  ratify-or-mint · `tc-plan-owns-its-source` · `tc-book-level-dot-locals-domain` ·
  `tc-redirect-refusal-dead-or-owed`.
- `kty-annot-punt` — the spike itself IS the kTYANNOT experiment (human 2026-07-12); "is
  not-using-EOL-comments livable?" is post-spike adjudication input, never upfront work.
- the standing prose queue (`mise run prose:census`) · the syncthing `.stignore` repair
  (`27U` §2).
