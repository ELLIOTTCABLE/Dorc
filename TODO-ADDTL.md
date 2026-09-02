(REWRITTEN AUTOMATICALLY, UNSTABLE. DO NOT REFERENCE IN OTHER DOCUMENTS.)

Dig through the design-docs in this repo; collate for me a list of 'undone design-work' that was either 1. mentioned by a human in-passing, but doesn't clearly map to any of the design-passes present; or 2. heavily pushed by a `plans/*.md` document, but seems like it may have gotten lost in the weeds. Sort higher: items with high design-consequences ("especially hard to unbake" or "can't be refactored"); and sort lower: items that seem known/deferred (in `TODO.md`, discussed by the human in recent design-passes, or clearly not-upfront-work.)

*Remove* complete items; they are in git history. Do not populate incomplete items with sub-lists of chunks that *are* complete, this should *only* mention incomplete work; it has a tendancy to become a work-log, which it very much should not be. Keep items *short*; the deails live in the design-docs they were written into / during.

Update-by-overwriting this section; keep this descriptive header/prompt, just replace items. Keep it short; collapse similar/related items into one entry; this shouldn't grow over ~10 items. It's to catch *major work*, not nits.

> Notes by me, the human, in >-blockquote.

## Known-scheduled / in-flight (NOT this document's job; do not re-add below; prune freely)

- r30 round-close ceremony (human's gate)
- 30D, dropping return-2 (charter owes the §4.3 typed-consequence ruling; its record
  species born under `30R`'s recorded/live typestates — replayed records stay inert)
- 30J, the family-dialect repair

## Stands between Dorc and *useful* live use

*(The 2026-07-27 trio — ssh executor, live-acceptance gate, CRLF gate — is BUILT and folded on `ai/r26-unify`; Dorc has now run for real against a VPS and a container. What stands now:)*

* [ ] **the acquired-source model restructuring is UNWELDED (`30R`)** — the receipt's source
  rows want a truthful ordinal, which the invocation model's two source categories could not
  carry, so they were collapsed into one ordered role-carrying vector. Ruled and built: the
  ordinal is deterministic acquired-source table position, not dynamic load-occurrence order,
  and the role is table classification only. The ack on the restructuring itself is explicitly
  temporary and wants grounding from outside this arc before it welds — repeated/multi-role/
  multi-target source semantics still belong to `30I`'s occurrence account, and a decision to
  keep it is a decision about where that account's boundary sits.

* [ ] **the render↔spine feedback shape (`30Ng:attn-render-refusal-feeds-the-spine`)** —
  human-flagged 2026-08-21, unruled: render-refusal ("won't elide: required syntax") is a
  mutative-difference-causing decision and must feed BACK into the decision spine; the
  flagged lean is a second rerun-to-fixpoint stage keeping the spine pure (render may
  refuse, never change — it mints new decisions and the cycle repeats). As-built is
  partway (refusals are settlement inputs; `Plan::decided` takes render answers once).
  Hard to unbake once artifact forms accrete on the current shape; needs a sitting with a
  termination argument. (The bundle front-lift is NOT a consumer of this: lifted only under
  a proven closed set, it changes no resolution and needs no settlement re-modelling —
  human-directed 2026-08-21, its own lane.)

* [ ] **the taught decline idiom silently defeats elision** — builder-measured 2026-08-24
  (the `30S` pin lane): `[ "$2" = "" ] || return 2` immediately after a predict's entity
  bind classifies fine but always renders Run, zero diagnostic — while the `if …; then …;
  fi` spelling elides (bisected against `observable_matrix.rs`). The `||`-form is the
  USER_STORY stage-3 taught idiom, so either the tracer has a guard-recognition bug or
  intended rc-consumption semantics are silently eating the taught shape; un-adjudicated,
  and the silent-no-diagnostic part is the real offense either way. Adjudicate BEFORE any
  real verdict-authoring starts — it is the taught first-hour shape.

* [ ] **write-elision needs a vouch-holder (`26K:sit-redirect-routing`; surviving half
  banked `26Lb` §3)** — `30D` closed the channel-claim leg only: routing, File-coordinate
  binding, and who authors the compare (fs-kind stdlib verdict / tool oracle / lifted
  admin idiom only) stay unruled, and the engine may never synthesize the compare guard
  (rul-ternary-verdict) nor elide a bare `> file` on byte-equality. Unlocks sh's most
  common mutation idiom (write-if-changed); channel-adjacent, so the predict arc must
  either sit it or explicitly reserve its seat.

* [ ] **the `30L` elision-regions build is unbuilt beyond stage 0** — the design is current
  (`plans/30L`, rewritten 2026-08-20+; only `stage-member-lane-ruling` satisfied), but
  as-built remains the all-or-nothing CALL disposition: the disciplined
  `main() { …; }; main "$@"` shape forfeits the whole attention product to one live body
  line. Its §14 sequencing (land BEFORE artifact-form reification + corpus-wide golden
  promotion, the surfaces that harden leaf-only edit identity) may be partially overtaken
  by the landed r30 artifact lanes — re-verify at pickup. Stage 1 (census/budget sizing +
  red-first battery) is measurement-not-machinery and runnable early. Kernel-tier: queues
  behind `30R` by standing rule.

* [ ] **the `command -v` load model (`30I:pin-command-v-load-model`)** — `command -v` stays a meaningful, idiomatic, supported dorc-lang route for asking what a shell resolves under a name, and is explicitly NOT forfeited; it is simply not the basis for exact-package guard recognition, which now rides the variable sentinel. Owed: which shell categories and floor/run-target variations the load model represents, and when that wider question may participate in exact guarded-source recognition. Evidence and floor measurements: `notes/30Ic` (output-slash classification measured-refuted — `PATH=:` makes both pinned floors print a bare name for an external; aliases can carry slashes; posh exposes neither aliases nor reserved words).

* [ ] **model-`local` in the tracers** — the builtin-deny (`26J`) now ⊤-degrades `local`, but the ruled dialect is "POSIX + `local`": any realistic oracle body using it walls its site. Safe, but caps value the moment real oracles get written; modeling local-assignment is the obvious small next increment. Second-order kin: book-side `set --` sub-form (`26J` residue).

* [ ] **live-surface polish (sharp edges handed over)** — `--host` absent from `--help` (loom-editable chrome) · `--results`/`--help` grammar mismatch stands. None blocks usage; papercuts catalogued in the CONTRIBUTING draft + `notes/26F`.

## Owed for pivot-books that analyze, and work, correctly

*(Minted 2026-08-31 from the pivot/reactive up-to-speed synthesis. Standing frame, acked same
day: a pivot is sh-spelled or it is nothing — no native non-sh ordering/pivot/orchestration
construct will exist; what remains open is how far Dorc LIFTS sh-spelled orchestration.)*

* [ ] **`26K` §0b — the pivot kernel sitting: design half largely DONE (`notes/26M`,
  `plans/30W`); build half + rulings owed** — the worldview settled on index-kinds: Host
  entered via ssh's mapped lend + the Host binder; Boot/Machine/LoginSession measured; a
  "transit" is a mutator of an index-value cell; no locus/axis/region vocabulary exists
  (see `30W`). Still owed: local-exec as a supported mode (zero tests; the prerequisite) ·
  wait ratification (`26K:sit-wall-transparent-delay-loops` + `26Lb` §2's
  wait-is-a-guard reframe; the `StatusIterated` converged-at-entry carve is a
  license-widening, human-flag before build) · the six `30W` §10 rulings,
  `rule-unreserve-host-as-entered-index` first (it collides with
  `271:rul-axis-vocabulary-v1`'s reserved-never `host`), then the razor question for
  incarnation-invariance (the cost-collapsing one) · the `FactKey.context` as-built audit
  before the context-slot generalization (retrofit-hostile).
* [ ] **the payload-declaration speech-act (`26M` §payload)** — the one engine core under
  BOTH carrier forms (adjacent-book dispatch and inline payload; equal priority,
  human-ruled): the carrier-geometry survey owed before any spelling · fidelity as an
  authored, pre-applied normalizer function (ruled) · custody-over-there · inline render
  whole-or-nothing (~lean). Language-surface holes, cardinal-capable if missed: payload
  identity must join the `26C` Question identity; expansion-siting across the payload
  boundary (which world evaluates each `$X`); entry recipes must carry the site's
  transport flags or decline.
* [ ] **`26L:finding-future-values-remain-future` — the epoch×capture boundary** — no document
  rules what the engine may claim below an un-fired transit, nor how captures
  (`IP=$(doctl …)`) feeding post-transit argv behave; `26K`'s planned-transit-as-analysis-event
  has never been reconciled with `26B`/`26C`'s monotone-knowledge frame (which models
  refinement, never authored invalidation). The single-shot posture belongs to the §0b
  sitting; the reactive half to the revival.
* [ ] **`26K:sit-stdin-copy-exec-amendment`** — copy-then-exec vs pipe; load-bearing for pivots
  (the secret-push `<./secrets/…` line is specimen two —
  `pivot-vps-standup.note:chafe-book-lines-consume-the-artifact-channel`); human suspicions
  on the downsides banked; a design sitting before any `260` §5 change.
* [ ] **`26Lb:lean-cross-host-facts-gate-never-license` — scoping + the owed review** — the lean
  is scoped (human, 2026-08-31) to NATIVE orchestration features (which include per-host
  *different* books); within a pivot/local book, cross-host dataflow is ordinary sh and
  largely un-preventable — punt. A security/threat-model review (opaque-review-tier) of
  pivot books' cross-host influence is OWED and explicitly out-of-scope for the current
  design work.
* [ ] **`26B:gate-binding-site-coherence` — the r26-revival entry gates** (revival-owned, after
  the r30 close) — the un-ruled R2-entry question (freeze-in-artifact vs
  structure-preserving folds) · the `26C` §1 stability-resolution formal confirmation · a
  FRESH `26C` §7-style quiet-welding audit against the r30 kernel BEFORE R0 (26C verified
  against the r27 tip only; `28Q`/`30K`/`30L`/Spine/`30R` landed since; NB `30L`'s iteration
  axis = loop-member while `26C`'s `iter=` = probe-iteration — two axes, one word) ·
  `26B:need-quiescence-witness-at-mint` (executor-era termination detection, mechanism
  unowned) · `26B:split-semantic-versus-concurrency-holes` (the batch driver cannot surface
  the transport holes; the `142`-shape executor work owns them).
* [ ] **`26B:ask-trial-counts-capture-walls`** — the capture lane's sizing evidence was never
  collected (the r25 ceremony died); count dynamic-value walls in the live-run era, and size
  `26C` §5b's narration depth from real `dorc why` critique, before the revival invests.
  Rider: the lane starving until `30D` stdout claims exist is correct-by-design (`26Lb` §3).
* [ ] deletes-are-hard (`061` theme 6) — the establish/kill model's product argument; sitting-visible.

## Demoted (real; does not block live testing)

* starter-oracle-stdlib — zero non-fixture oracles exist; important-and-pending, NOT blocking (human-ruled 2026-07-27: stdlib/multihost/first-blood ceremony have mostly stood in the way of experimenting; scrappy hand-written oracles are part of the experiment itself). The verdict-only tier is authorable against in-tree mechanisms once the taught-decline idiom (above) adjudicates; predict-bearing families wait for the scheduled `30D`/`30J`. Pivot priority when revived: transit verbs lead (an unmodeled, unguarded transit walls every day — `26K` concl-boot-books), then the pivot wall-cluster (timeout · ssh-keygen · curl · getent). On-ramp when picked up → `27Q` (§2 preconditions discharged); authoring trap: converged≠no-op adequacy, unmeasured → `24U` §2.
* slow-planner-cost-model — foreign convergence/preview checks can take minutes, and planning-duration is itself staleness; no design exists; must be sat before the terraform/ansible-class delegation oracles are authored → `26L` §11, `KNOBS:kPROBING` (check-tax) · `26Lb` frame-conductor-cost-model (a kept converged wait's check-tax = one serial round-trip per wait, in apply order).
* book-acceptance-carve — the value-ladder for accepting unmarked/bash/zsh-ish *runbooks* (never oracles) is undesigned/unowned; design inputs banked → `276:rul-kwhichsh-oracle-scoped`, `276`.
* probe-safety-backstop — seccomp `socket(AF_INET)` observe + `--faithful` one-leaf-one-exec, both unowned; probe honesty rides author discipline until then → `077`, `24O` item-13.
* posh-leg-of-the-floor-is-unexercised — `printf` is not a posh 0.14.1 builtin, so under the corpus's `PATH=mocks-only` rail no shipped oracle body's emissions have ever run under posh: the corpus half of the `kWHICHSH` weld's "dash ∩ posh identically" promise is dash-shaped. The opt-in `mise run test:floor` lane (r28) proves six sentinel manifests only; corpus-wide posh coverage of emitter bodies is unowned → `spike/CLAUDE.md` floor-differential-lane-opt-in, `28P`.
* oracle-author-quality-bars — wrapper bar, carrier bar, adjudicability build-list (must land before kinds go community-shared) → `24S:A6`, `24T:P-A4`, `24S:A4`, root `AID-NEEDS.md`.
* kty-annot-punt (human-ruled 2026-07-12, recorded here to outlive the chat) — the spike itself IS the kTYANNOT experiment; "is not-using-EOL-comments livable?" is post-spike adjudication input, never upfront work; if inline survives, it must prove worth forgoing the comment-adjacent tooling ecosystem.
* precision-identity-residue — partial-member convergence · may-alias-default ruling (must never flip silently) · uniqueness-bit build → `277` §5/§6, `24O` item-25.
* kstate-and-cross-host — the `(verdict, content-key, freshness)` shape is retrofit-hostile, decide-shape-early; riders: host-as-adversary, wall-clock-keyed classes, rec-5, the `261:dec-timing-cache` fence question → `23O` §5.
* wrapper-payload-residuals — fs-view Hard cell (sequenced behind netns) · guard-insertion under ELEVATED lanes · become/doas prior-art ack → `27C`, `24S` §3b, `23J`.
* locator-dag-n-tier — per-host-forking DAG + transport-minted session correlation; first consumer is the multi-host era; re-grade the moment the ssh executor grows past one host → `111` dac-A.
* receipt-sensitivity — output-sanitization/secret-taint unbuilt while receipts are default-ON and hold raw host metadata; acceptable for a throwaway box, re-grade before real estates → `AID-NEEDS:law-receipts-are-sensitive` (née law-whylog-is-sensitive).
* why-surface-close-residue (conductor-closed 2026-08-31, the r30/30R close arc) — unwitnessed branches: the comparison seat's authentication asymmetry + non-following read (no fixture world reaches them) and the `vouched-severally` committee voice-set · the loom `run:`-lane widening for receipt-driving cases (deferred; the declared-needle gate serves) · a typed order accessor for an explicit `--receipt <file>` renamed out of the store grammar (lean banked; refusal ships) · whether the pre-corpus bespoke whole-product `.rs` tests (`receipt_route.rs`, `spine_baseline.rs`) migrate into the corpus · five orphan arrangement rows awaiting the human's deletion word (`mise run prose:orphans` lists them) · `tc-address-refusal-is-a-datum-not-a-diagnostic` (held: no-match-renders-a-row vs fails-fast; conductor recommends splitting no-match/row from multi-match/ask) → conduct ledger `30Va`.
* records-eight-emitter-decision — the last 8 ratcheted codes (`records-*`) have NO production emitter (`records::deframe` is test-only; the framed-admission path is the live intake): human call — delete under no-compat, or wire in the r26 records revival → `28L:rul-no-emitter-codes-are-blocked-rows`, `28N` §2.
* loom-post-arc-residue — the priced-and-declined per-fragment-owners remedy (31 lock-tier components) · the blocked-on-emit/render-seat sets · the run-lane edit loop for cli/tests homes → all enumerated with reasons in `28N` §3; revisit only if the lock hand-seed flow chafes in practice.
* human-root-doc-queue (his voice) — fix-gsub-strip-claim · fix-flag-gloss-composition-not-contradiction · fix-kwhichsh-hedge-and-scope · fix-marker-gate-absent · smalls (skip-vs-elide render divergence, arity-gate idiom, typos) · "three possible outcomes" enumerating four · the dq-kOOB stamp line → the 2026-07-17 fix-review cut (full text in this file's git history).
* pending-ruling-queue (small sittings) — floors-ratification (`27U` §7) · decline-class starter-set (`27W` §0) · C8 operand-value display (`27U` §7) · prose-register schema (`282` §10; W4 landed in the r28-unify worktree, so the W5⇄W4 interlock is resolved and the sitting has transcript faces) · lint tc-leans (`27S` §5, `27T`) · `Consented`-knowability at first render (`28F`; its `--no-whylog` spelling half is dead — the receipt flag family landed) · why-carries-risk-flag + apply-header-vs-byte-floor (`28I`) · syncthing `.stignore` repair (human-owned, `27U` §2).
* flux-engine-hardening — the penciled refinement-type (Flux) defense-in-depth instrument for the churny engine tier (intake byte budgets, span/interval arithmetic — the tier Kani/Lean deliberately do not reach); explicitly UNSCHEDULED (human, 2026-08-21: "enough to do in r30"); any typesystem change Flux would need rides the Aeneas-prep facade work, never a Flux lane → `300:lane-flux-engine-hardening (née §2)`.
* veto-sweep-routed-residue (conductor-closed 2026-08-22, `307` §6c) — mint wall narratives for non-leaf walls per `kWARN` (nothing renders until `289:seam-narrative-render-unconsumed` closes) · delete the unreachable `render-heredoc-refused` path and bank guards-at-redirected-sites as the value item it implied (those sites run today, the safe direction) → both ride the next lane touching `plan::world`'s wall seat / the guard-mint seat.
* seams-grab-bag — streaming/TUI (rides weft) · retries/until · serial non-preclusion · escape-hatches + veto polarity · secrets timing (`26B:need-scrub-before-freeze`) · `24R` cheap-adds → each pointer is the live re-entry.

* rc-vs-genkill-permanent-law (human-directed 2026-08-23: revisit, not now; never strike) — `spike/CLAUDE.md rul-rc-reaches-genkill-only-through-decisions` still says the permanent law "is expected out of the influence implementation round"; `30Qd` §map-10 item 7 established that `lane-influence-carriage` carries accounting only and will NOT produce it. The clarifier stands; the wider law is unowned. Re-home the expectation at the next steering edit (point here), then revisit once the influence threading has survived a few rounds of churn.
* r30-close-out-residue (conductor-closed 2026-08-23, `30Q` §5e) — one explicitness predicate at two seats (`cli::artifact::operand_is_explicit` reads the AST word, `funcenv::ResolvedHead::explicitness()` the resolution; the cli seat should read the marker) · an early-round acquisition reaches the frameless `build_dialect` scan and `HelperIndex` custody with no positional question asked (`30Qf:tc-acquisition-outlives-the-clobber`; flag-bounded) · a refused body's OWN `.` lines keep authority (`vacuous-entry-fold`) · a HELD cross-file oracle body's own `cd` is invisible to the cwd domain (`30Qf:tc-a-held-body-is-modelled-as-text-not-as-shell-state`) · `doctor`/`doctor:unused` from WSL walk every worktree with per-worktree `git` calls across drvfs (3–36 s each) → each rides the next lane touching its seat.

## Deferred arcs (pointers only)

MH2 version layer → `.claude/research/versioning-mh2/`, `270` §4 · r26 reactive/capture + multi-host revival → `26B`/`26C`, `270` §5 · r25 field-trial ceremony → superseded by the informal live run; tooling salvage at `Research/trial/` · DST rung ladder → `128` §3/§7 · r11/r12 research banks → `111` §4, `128` §8.

> on MH2: versioning (the simpler version of the concept, focused down to 'package-as-a-type-needing-special-attention, multi-providers, etc' ... and 'mapping oracle-written-for vs being-executed-on') needs close care, but it feels very deferrable compared to some of the critical core analyzer-design things that are affected by the "sh spelling" issues. continuing to defer.
