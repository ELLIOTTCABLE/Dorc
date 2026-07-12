# 274 — The eval'er surface: the reentry token, descend-don't-license, and the probe-shipping split

AI-authored (Fable, the `270:block-settle` rubber-duck sittings, 2026-07-12), minted at
the human's direction as the comprehensive durable of the task-6 arc (eval'er
declaration spelling, né `24T` pin1 "licensed-code-carriers") — the `notes/272`/`273`
precedent: the arc was extensive redesign, large enough for its own document.
Note-tier, kept-current through block-settle. Authority: root docs and the `plans/271`
rulings ledger outrank this; ratification is MIXED and marked per-section — §12 is the
status table, read it before citing anything as settled. Companions: `plans/271` (the
rulings; its task-6 entries summarize and point here) · `notes/273` (the wrapper
surface this composes with; carriers were its named open coupling) · `plans/24T` (the
payload keystone: pins, quote-stage map, impossibility ledger — its pin1's
DECLARATION mechanism is superseded here; its fences all stand).

## §0 — The arc in one view (what died of the original question)

`24T` pin1 asked "how does a head's oracle DECLARE which operand is code?" The arc
dissolved the declaration into detection and then dissolved the naming problem into a
token we own:

- "carrier" → **eval'er** (typed rename; `271:rul-evaler-vocabulary`).
- A separate structure-member → **dead**: eval'er-ness is detected inside
  `cmd__predict()` (the human's argparse-divergence decider; the cohort audit found
  every divergence branch-terminal and ρ-confined; `271:rul-evaler-merge-no-structure-member`).
- The delegation head `eval` → **dead, typed** (wrong context record on every axis;
  token-collision with future transparent-context eval modeling; off-ramp leakage).
- Worldly heads (`sh`, `dash`) as the blessed spelling → **dead** after the
  quantifier audit: `dash` names a missing binary on huge host classes; `sh`
  mis-labels the author's epistemic situation (host-sh is the one thing they can
  never know; dorc-sh the one thing they can).
- The engine's grounding floor → shrunk from a blessed-head list to **one token we
  own**, fulfilling the human's prediction that the options would fall out of the
  semantics-proliferation problem rather than an invented bless list.
- The result is the human's own synthesis: the **`dorc:sh` prefix-marked head**
  (`271:rul-dorc-prefix-head-synthesis`), graded against the accumulated rubric and
  leading at ~60.

## §1 — The surface at a glance: three spellings, one design

| spelling | meaning | analysis | probe runtime | strip |
|---|---|---|---|---|
| `sh -c '…'` (bare) | the host's real sh; escape; "leave it alone" | DESCENDS for hints only; licenses NOTHING | untouched; host PATH resolution | untouched |
| `dorc:sh -c '…'` | the dialect's reentry; "dorc may do as it pleases" | full license | prefix rewritten → `dorc-sh`, resolved by the per-run shim | prefix-erased → bare `sh` |
| `dorc-sh …` (typed directly) | the runtime object; by-construction/multi-nest | NO analysis license | the shim (pinned evaluator), composes transitively via PATH | untouched — documented-dangle (typed ruling) |

Load-bearing properties:

- **Descend-don't-license** (bare `sh`): analysis reads the payload and produces
  hints ("this bit inside won't elide — did you want `dorc:`?"), but no elision,
  probing, or rearrangement is ever licensed past an unmarked head. Even a wrong
  parse of an unmarked payload cannot under-execute — the wrongness surface of
  unlicensed descent is hint-noise. The no-keyword option's omission-failure is
  structurally impossible. Enforcement tier (typed direction,
  `271:rider-invited-rooms-typing`): TYPESYSTEM, not test-pin —
  incorrectness-inexpressible type-differentiation between invited-room analysis
  (may mint licenses) and hint-only rooms (may not).
- **`dorc:sh` is grammar-valid, world-invalid**: colon is an ordinary word
  character, so every sh tokenizer parses it as a command word — and under a stock
  shell it fails LOUD (127, naming the token), strictly better than trailing marks'
  silent corruption. It is ANNOTATION SYNTAX, not in-semantic meaning: no nested
  occurrences — annotation-syntax inside opaque body-blobs is a plan-time
  parse-failure-tier warn/error (typed, `271:rul-no-nested-annotation`).
- **The shim**: a per-run, host-constructed `dorc-sh` on a PATH-prepend, carrying
  the session-resolved evaluator. Pinning lives in the shim, not in per-site text —
  so shipped probe text is HOST-INDEPENDENT (a DST/golden dividend, §5).
- **Transitive composition is opt-in, not accidental**: row 3 is a real PATH object
  that composes through `xargs`, `find -exec`, and nesting by ordinary Unix
  resolution. The former rule-transitive-scope question collapsed into "row 3
  exists: yes."
- **Off-ramp**: strip = prefix-erasure (mark-erasure class; the
  rul24-totalistic-munge carve shrinks to the shebang-runner rewrite only); the
  stripped file's bare `sh` has real-world semantics; typed `dorc-sh` dangles
  loud-127 by the author's documented buy-in ("half-strip is worse than no-strip";
  bash, perl, or sh invoking `dorc-sh` post-uninstall all behave identically).

## §2 — Mechanism-native vs content-claimed (the reentry ρ-split; NEW at minting)

Minted while strawmanning (2026-07-12, conductor; wants the human's eye): the
reentry form's CHILD-CONTEXT MECHANICS are dialect-defined — fresh shell options
(`24T:L1`), export-only inheritance as a mechanism (`24T:L2`), positional binding
(`24T:L3`), stdin wiring — the author does not claim them; owning the token is what
makes them definable. But the ρ CONTENT the guest sees is claimed by the standing
env-idiom ladder (`271:rul-env-claim-inversion`) exactly as for any delegation:

- bare `dorc:sh -c "$code" "$@"` — claims NOTHING about the environment (⊤);
- `env dorc:sh -c "$code" "$@"` — full ambient passthrough;
- `VAR=x dorc:sh -c …` — per-variable claims, rest ⊤;
- `env -i VAR=x … dorc:sh -c …` — exactly-these.

Consequence visible in the flagship strawmen: sh's own oracle spells
`env dorc:sh -c "$code" "$@"` (a real `sh -c` child sees the ambient exported env —
a true, typed, positive claim), while su's login arm spells the bare form (login-file
environment is host state — claims nothing), and the one-glyph difference between
those two lines IS the design working. Post-rewrite the `env`-headed form execs the
shim as a file, so the env-cannot-exec-functions landmine stays dissolved here.

## §3 — The eval'er oracle surface (what an author writes)

All prior task-6 rulings compose here: no structure member (detection inside
`cmd__predict()`); the env-claim inversion; the shapes `-c`/`-s`/file/bare-stdin;
whole-shape decline by `return 2`; the shell-identity vouch as ordinary authored
judgment (su's delegation to `dorc:sh` IS the claim "the target's login shell
evaluates this sh-compatibly"; `-s`-pinned shapes declined until modeled). Tool
oracles never mention eval'ers; eval'er oracles never mention tools — the
no-awareness referendum extends to carriers unchanged (`24S` §2c). Books never
change: a book's `sh -c` site decomposes through the STDLIB sh-oracle's vouch, not
through any engine opinion about the token `sh`.

## §4 — Strawmen (brief; the full walked set is in the 2026-07-12 dialogue)

```sh
# dorc-lang/v0.1
# the stdlib sh carrier-oracle (the workhorse)
sh__predict() {
   while :; do case "${1-}" in
      -c) code="${2-}"; shift 2
          env dorc:sh -c "$code" "$@"    # reentry; $0/$@ per POSIX; env = true claim
          return ;;
      -s) shift; env dorc:sh -s "$@"; return ;;   # stdin-code; operands are $1...
      --) shift; break ;;
      -*) return 2 ;;                    # set-flag clusters elided in strawman
      *)  break ;;
   esac; done
   [ $# -gt 0 ] && { env dorc:sh "$@"; return ;}  # sh FILE args…
   env dorc:sh                                    # bare sh: stdin-code
}

# su — the straddle, fully spellable at last
su__predict() {
   login=false preserve=false target=root code=''
   while :; do case "${1-}" in
      -|-l|--login) login=true; shift ;;
      -m|-p|--preserve-environment) preserve=true; shift ;;
      -c) code="${2-}"; shift 2 ;;
      -s|--shell) return 2 ;;            # pinned-shell shapes: modeled later or never
      --) shift; break ;;
      -*) return 2 ;;
      *)  target="${1-}"; shift; break ;;
   esac; done
   [ -n "$code" ] || return 2            # interactive shapes: decline
   if [ "$preserve" = true ]
   then env dorc:sh -c "$code" "$@"      # -m/-p: ambient passes through (true claim)
   elif [ "$login" = true ]
   then dorc:sh -c "$code" "$@"          # login files = host state: claims NOTHING
   else USER="$target" LOGNAME="$target" dorc:sh -c "$code" "$@"
   fi                                    # plain su: per-var claims; rest T (under-claims, safe)
}
```

Book flows (admin lines, unchanged forever): `echo "$m" | sudo sh -c 'cat >> /etc/motd'`
= pipeline ∘ sudo's lend/predict ∘ sh-oracle reentry — the `24T` §1 acceptance shape;
`su - postgres -c 'psql …'` = keyed by lend_map, payload analyzed under ρ-⊤
(argv-literal analyzes whole), psql credential-gated ⇒ honest run-with-guard cap;
`find … -exec sh -c 'nginx -t -c "$1"' _ {} \;` = the taught-fix cell — literal
payload, runtime-⊤ positional, bounded by nginx's oracle. Row-3 by-construction:
`… | xargs -r -n1 dorc-sh -c 'systemctl is-active --quiet -- "$1"' _` — pinned
evaluator, no analysis, documented dangle. Escape: a deliberate bare
`sh -c 'echo "$KSH_VERSION"'` stays untouched, hint fires once.

## §5 — Probe-lane shipping and the DST story

The split (`271:direction-evaler-probe-shipping-split`, as corrected by the human's
three nits): the honest axis is whose semantic bytes ship and how much recontexting
they suffer — never transform-presence; a probe always perturbs the in-sh world it
measures; bare-ship buys one divergence class (evaluator-identity over the payload
interior). Bare lane = proof-gated (guard-lift precedent). Transform lane =
task-14-gated; hoist stands DOUBTED (the human's block-integrity counter-shape:
prefer moving blocks whole under their executor); subshell-eval lowering demoted to
last by the observable-transforms gradient. The transitive class is served by row 3,
by explicit opt-in.

DST (`271` carries the compressed record): shipped text host-independent (rewrite
targets the fixed NAME; variance confined to the shim's one line — goldens stay
host-agnostic); every host interaction is a transport-seam event (handshake · atomic
write-then-rename · smoke-test · execs · cleanup); shim materializes ONCE at
session-establishment (happens-before all probes by protocol — no new ordering
class); temp naming run-id-derived (no mktemp randomness; stale dirs inert); the
failure lattice drains to 126/127 ⇒ ≥2 ⇒ can't-say ⇒ run; the preamble smoke-test
converts scattered failures into one session-level shimless-degrade (marked-reentry
probes pre-degrade without shipping — also the whole Windows/noexec story, honestly);
in-memory simulator registers the shim as a command keyed to the materialization
event, real-dash fixtures materialize actually, the differential bridges.

## §6 — Laws minted en route (wider than eval'ers; registration routed)

- **observable-transforms gradient** (human): probe design was always
  transforms-all-the-way-down, ignorable while the set was fixed and unobservable
  under read-only; observable transforms add teachability and a trust-gradient to
  licensure. → **candidate-probe-body-contract** (the author-facing execution
  contract as transform-admission criterion) and the menu's second gate.
- **fair-attribution** (human-endorsed, typed; root-docs registration CLAIMED BY THE
  HUMAN): errors attributed-but-unfairly (we misled the author) are WORSE than
  unattributable errors; "DX is the CORRECTNESS product." Caveat typed: does not
  trump lint-never-drives-design.
- **invited-rooms typing** (typed direction): license-plane type-differentiation,
  incorrectness-inexpressible.
- **quantifier-audit** (method note): the works-for-all→works-for-one challenge was
  wrong in mechanism (∀→∃ eliminates at deployment; visibility ≠ quantification;
  availability-failure ≠ semantic-drift) and right in direction (epistemic labeling).

## §7 — Considered and discarded (the graveyard, with killers)

`eval`-as-spelling (typed kill: context record wrong ×4; token collision; leakage) ·
`dash` default (absent on RHEL/Alpine/BSD classes) · worldly blessed-head list
(epistemic mislabeling; engine worldly opinion) · separate structure-member (the
argparse-divergence decider) · bare-delegation-means-passthrough and the
`: env-unclaimed` mark (both superseded by the env-claim inversion) ·
transform-blessed-real-`sh` (substitution inside authored sh; survives only as the
mark-licensed variant inside the dead marked-sh bundle) · no-keyword
(omission-reachable quiet drift; razor-failing) · subshell-eval lowering (the
gradient's own worst-case scenario; engine-lowering only, probably never) ·
`dorc`-token in BOOKS (probe-apply divergence — the human's own kill; its
oracle-body descendant became row 2) · host-shipped persistent shim (agentless
ethos; superseded by the per-run form) · a colon-named shim file for string-interior
marked forms (colon filenames die on Windows-adjacent targets; superseded by
rul-no-nested-annotation anyway) · bundle-marked-sh (resurrected once, then
superseded by the synthesis, which keeps its strip-purity while fixing its
head-labeling) · bundle-own-token-loud (word-shaped keyword foreclosed the shim and
transitives; grammar event).

## §8 — The rubric and the synthesis's grades

Constraints accumulated across the arc: child-context fidelity · epistemic labeling
· fair-attribution/teachability · no-engine-defaults · the razor (positive acts,
never omission) · off-ramp/run-blind · pinning-by-construction · loudness ·
task-14 surface · agentless/DST · grammar-cost · escape-spelling · transitive scope.
Grades (from `271:rul-dorc-prefix-head-synthesis`): fidelity ✓ · labeling ✓ (best of
field) · razor ✓✓ (descend-don't-license) · teachability ✓- (rows 2/3 one-glyph
trap → did-you-mean hint owed) · no-defaults ✓- (hint-descent needs the
invited-rooms fence) · off-ramp ✓- (row-3 documented-dangle, ruled) · pinning ✓ ·
task-14 ✓ · agentless/DST ✓- (post-walkthrough; two pins + one honest degrade) ·
grammar ✓- (a word, not a keyword; new prefix-mark position, kOOB glance owed) ·
escape ✓ · transitive ✓ (opt-in).

## §9 — Soft spots (honest; attack here)

- The **shim residuals**: writable-fs on stripped targets (⇒ shimless sessions —
  honest but value-zero for reentry there); PATH-weaving through AUTHORED `env -i
  PATH=…` scrub bodies (an author's explicit PATH kills the prepend ⇒ marked
  delegations 127 ⇒ safe but surprising; hint owed; blessed-idiom-set question).
- **Rows 2/3 one-glyph confusion** (`dorc:sh` vs `dorc-sh` differ by analysis
  license) — the did-you-mean hint is owed, or a row-3 rename considered.
- **Hint-descent quality on misparses**: hints derived from a wrong parse of an
  unlicensed payload can mislead (kWARN-tier, never correctness).
- **The prefix generalization door**: `dorc:*` scoped-or-shut at minting is stated
  but not yet formally stamped; meanwhile the human's command-word-dorcisms thread
  leans the OPPOSITE way (revisit old syntax toward command-word spellings) —
  these must be reconciled deliberately, not by drift.
- **su's plain-shape under-claiming** (USER/LOGNAME only, rest ⊤) loses value on
  env-dependent payloads under plain su; the getent query-arm remains the wanted
  extension.
- **Set-flag forwarding** (`sh -ec '…'`) needs clustered-flag recognition in both
  the oracle argparse and the reentry grammar; fiddly, build-facing.
- **The `24T` §6 asserted-semantics ledger (L1–L7)** now discharges against the
  reentry form and the shim — the differential obligations transfer, they do not
  disappear.
- **Probe-form composition remains task-14-gated**: nothing here ships oracle
  bodies as stand-ins yet; the synthesis only settles what the reentry point IS.
- The **no-awareness referendum** (tool oracles never mention eval'ers) is still
  unproven against build contact — same stop-the-block watch-item as `24S` §2c.

## §10 — Residuals routed

Block-context implementation-planning: the reentry grammar (shapes, clustered
flags) · shim construction/smoke-test/cleanup · the descend-for-hints machinery ·
the did-you-mean hint · dq-annotation-in-blob (rul-no-nested-annotation's error) ·
the probe-body contract page. Task 14: the shipping transform lane; PATH-weave;
scaffolding-not-semantics. Task 15: the general semantics-proliferation stance (the
eval'er-local fragment is settled by ownership; grep-vs-grep remains). Task 12 /
entity-algebra note: the prefix-mark position joins the grammar surface; the
command-word-dorcisms thread parks there. Stdlib briefs: sh/dash/bash carrier
oracles + su's pair, authored against THIS surface. `notes/273` §8's identity-wrapper
bare-`"$@"` line gains the `env "$@"` annotation owed by the inversion.

## §11 — Open couplings

task-7 (capture): stdin-code value-carriage and the captured-bytes rule share the
value-plane lane · task-8: the survival flag's adjudicability condition re-reads
against descend-don't-license (hint-lane facts must never feed survival) · task-14:
gates §5's transform lane and the only-oracle-bytes law · the field-trial book's two
walls (`su - postgres -c`; the hostname guard) are now both in-design — the su line
via this surface, the capture via task 7.

## §12 — Status table

| component | status |
|---|---|
| eval'er vocabulary (né carrier) | TYPED (`271:rul-evaler-vocabulary`) |
| merge: no structure member | DRAFTED-awaiting-ack (decider was the human's; audit conductor's) |
| env-claim inversion (idiom ladder) | DRAFTED-awaiting-ack (human's proposal, conductor-endorsed; gap re-scored) |
| eval killed as spelling | TYPED |
| fixed-set floor direction | TYPED (refined to one own-token by the synthesis) |
| the `dorc:sh` synthesis (three spellings) | human-strawmanned; shaping TYPED (four dispositions); formal stamp owed at task-6 close |
| descend-don't-license | shape settled in the synthesis; invited-rooms TYPESYSTEM tier TYPED as direction |
| row-3 documented-dangle | TYPED |
| no-nested-annotation | TYPED |
| strip = prefix-erasure + shebang rewrite only | follows from typed row-3 ruling; totalistic-munge carve wording owed formal ratification |
| pin-transform + shim + session-establishment + run-id naming + smoke-test | conductor design, DST-walked; unbuilt |
| mechanism-native vs content-claimed ρ-split (§2) | NEW at minting; conductor; wants the human's eye |
| probe-shipping split; hoist-doubt; transitive-by-row-3 | direction-tier; task-14-coupled |
| observable-transforms gradient · fair-attribution · probe-body contract | human-voiced / human-endorsed; registration routed (root docs = his) |
