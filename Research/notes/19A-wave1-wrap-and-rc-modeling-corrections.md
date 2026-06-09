# 19A — Round-19 wave-1 wrap + the five human design-corrections (the rc-modeling root)

> Re-seed orchestrator note. Wave-1 (gw-1 Half-B/F1, gw-4 corpus, the adversarial-crosscheck, gw-5
> fixes) outcomes, then the **five design corrections the human gave this session** — the durable,
> high-value part. AI-authored, confidence-marked. Trust R/D/I/K over this; the corrections below are
> the **human's rulings** (treat as authoritative direction, though their phrasing here is mine).
> Continues 197/198/199.

## 1. Wave-1 outcomes (all green: 41/41 e2e, workspace + clippy clean)

- **gw-1** (`fd9d0e0` F1 stopgap, `85c48a4` Half-B.1, `c408afd` notes/198): F1 under-execute fixed
  (guard-status blocks elision; `guard-elision-wrong` xfail → passing `guard-status-blocks-elision`);
  Half-B.1 = operand-bound FLAT interceptors; Half-B.2 (subsumption) NOT built, wall mapped to 3
  deferrals (query-category/`inc-7`, occurrence-typing/`inc-6`, structural-render). **Note §3 below
  reframes gw-1's F1 + interceptor as *misframed special-cases* per the human.**
- **gw-4** (`d057797`: 13 non-guard strain cases + notes/199): corpus 26→39. **Frame-inversion
  finding:** the non-guard surface is mostly GREEN — `notes/197 §5`'s "gaps" were test-*coverage*
  gaps, not engine-*capability* gaps. Genuine strains: `strain-R` (line-render mangles a one-line
  `case`-arm — render-fidelity, distinct from F1's classification layer), `strain-E` (the whole
  errexit precise-edge subsystem is e2e-invisible in apply-2 until the backward slice consumes it),
  `strain-S` (lifter checks probe-completeness per-file not per-kind ⇒ spurious `oracle-missing-probe`),
  and `. /etc/profile` is `Opaque` (sourcing a helper defeats downstream elision).
- **adversarial-crosscheck** (kp-1, un-seeded pair, both self-corrected a stale-base worktree to assess
  `c408afd`): **convergent** — F1 is a *structurally-safe tightening* (only ever adds to the block-set,
  read `May` ⇒ can only block, never skip; pre-fix counterfactual wrongly-elided, post-fix runs); S2
  (body-in-guarded-branch) sound; the interceptor render works. **The adversarial pass OVERTURNED the
  "&&/|| safe-for-conforming-oracles" disposition** (gw-1's, the neutral pass's, AND the orchestrator's):
  it proved via the CLI that a non-conforming establish (`useradd` rc 9, `mkdir`/`ln`/`docker network
  create`) as an `&&`/`||` left operand wrongly gets `Replace`-d ⇒ `useradd || mkdir` skips `mkdir` (a
  priority-1 `kFAIL-perform` under-execute). New convergent find **F-QUOTE** (unquoted operand in the
  interceptor render — wrong-entity + a `kFAIL-withhold` probe-injection). Adversarial-only (~SUSPECT,
  unverified): the subsumption wall may be *softer* than gw-1 mapped (same-cell narrow case buildable on
  existing primitives) — flagged, not banked.
- **gw-5** (`820b730` F-QUOTE shell-quote, `d42876f` doc, `ecf48da` under-execute pin): F-QUOTE fixed
  (single-quote-always, 28 goldens re-blessed render-only); the &&/|| under-execute pinned at the
  *disposition* level (`observable_matrix.rs` `#[should_panic]` xfail) + a render-coupled e2e tripwire
  (`andor-rc-vouch-wrong`), because the line-render *masks* the under-execute (keeps the `||` line
  verbatim). Harness limit surfaced: `cli parse_results` keys verdicts by first whitespace token ⇒ a
  spaced operand folds to `Unknown ⇒ Run` (safe).

## 2. Process datum — the practice earned its cost again

Three optimistic reads (gw-1's strain-log, the neutral crosscheck, the orchestrator's relay to the
human) all bought the same false assumption — "converged ⟹ rc 0" — and the *adversarial* pass, told to
distrust it, found the common trigger (`useradd`/`mkdir`) and proved it. Near-exact replay of the
round-19 `strain-8` datum. **`an-probe-shape`/`DP-3` ("capture the tool's *own* rc") was in the corpus
all along; the spike's elision still assumed rc-0-success.** Keep aiming `/adversarial-crosscheck` at
*invested* conclusions, un-seeded.

## 3. The five human design-corrections (this session) — the durable deliverable

These are the human's rulings; they reframe wave-1 and set wave-2's shape.

- **C-1 · the probe interceptor must pass the command's FULL argument string** (human, +SURE
  ruling). Dorc must NOT extract a subset of a command's argv (`command -v nginx` → `…check nginx`
  drops `-v`; `apt-get install -y nginx` → `package__check nginx` drops `install -y`) — that assumes
  the command's private arg-grammar Dorc cannot know (the `F7` argparse-zoo). The oracle's `check()`,
  which alone knows its command, is invoked with the **full** argv and does any extraction. The
  strawman already shows this (`id__check -nG deploy`; `id__check(){ command id "$@"; }`). **gw-1's
  kind-keyed, Dorc-extracted-entity render (`package__check nginx`) VIOLATES this** — it is the
  dangerous assumption. *Pending arg-grammar tooling (`q1-flaggrammar`), full-args is the only safe
  form.* Open: **q-1a** (does cell-keying / `resolve_entity` also stop relying on extraction, or only
  the probe invocation? — ~SUSPECT only the probe invocation for now); **q-1b** (mutator case: the
  check becomes provider-keyed `apt_get__check install -y nginx` with the oracle parsing apt's grammar
  to reach `dpkg-query` — confirm the shape).
- **C-2 · `if/then` ≡ `&&`; the real axis is rc-polarity, tracked in the CFG** (human, +SURE).
  `if cmd; then B` ≡ `cmd && B`; `if ! cmd; then B` ≡ `cmd || B`. gw-1's F1 fix special-cased
  `if`/`elif` (block status) vs `&&`/`||` (unmarked) — that *split of one phenomenon* is exactly what
  left the &&/|| under-execute. The difference is **boolean polarity** (`!`, `&&`-vs-`||`), which
  belongs in the CFG as a per-consumer **rc-expectation** (success-expecting vs failure-expecting);
  eliding a status-consumed command must honor its expected rc. One uniform rule subsumes F1 + the
  &&/|| finding. gw-1's two special-cases are a band-aid for this general model.
- **C-3 · errexit is honored, not special-cased-as-vouched** (human, +SURE). Under `set -e` every
  command's rc is consumed (non-zero ⇒ abort) = an implicit success-expecting guard per line.
  gw-1's "errexit-status stays vouched, still elides" carries the identical rc-vouch unsoundness
  (eliding `useradd` to `:`/rc-0 hides the abort it would raise). Currently *masked* (`set -e` is
  `Opaque` ⇒ poisons downstream ⇒ nothing reaches the status question — gw-4 `strain-E` / neutral
  pass) so it is a latent design error, not a live bug — but the framing is wrong. errexit is just
  another rc-consumer under C-4.
- **C-4 · return-code must be FULLY MODELED; its directionality is oracle-contracted** (human, +SURE
  — THE ROOT). rc is not bool 0/1, and which rc value means "converged" is **declared by the oracle
  per check/command**, never assumed. This subsumes C-2 + C-3 and gw-1's Wall-1: a *query*
  (`command -v`) is a check whose rc maps to a verdict *without mutating*; a non-conforming establish
  (`useradd` rc 9) declares rc-9 = converged; the CFG tracks each consumer's **expected** rc
  (polarity), the oracle declares each command's **actual** rc→verdict mapping, and elision is sound
  iff they reconcile. **This is also Half-B subsumption** (branch-resolution = reconcile the guard's
  declared rc→verdict with the probe). Nearest corpus: `an-probe-shape` / `DP-3` ("capture the tool's
  own rc") + the three-valued verdict `T5`; C-4 sharpens them from "capture rc" to "model rc + contract
  its directionality." This is wave-2's center of gravity (high-lock — re-keys the oracle/effect
  contract).
- **C-5 · rendering: structural + reconstruction-metadata, two outputs** (human; OUT-OF-SCOPE this
  spike, docs-notation). The render becomes AST-structural but carries enough metadata (line/col,
  whitespace tokens) to reconstruct the original; reconstructions must be *minimal*; one AST serves
  two outputs — (a) a **minimal by-AST print** (ship only the active commands/expressions over ssh)
  and (b) a **full textual reconstruction** with ANSI grey-out escapes progressively wrapping the
  components/sub-expressions being eliminated/substituted *as probe data streams back onscreen*. This
  is the resolution of the line-vs-structural tension (gw-4 `strain-R`, `seam-prov`/`an-render-modes`);
  the current line-granular render is the disposable interim. The leaf-exact render will flip gw-5's
  `andor-rc-vouch-wrong` tripwire (desired signal).

## 4. Reframed wave-2 + open decisions (for the human)

The corrected core is **C-4 (model rc + oracle-contract its directionality) + C-1 (full-args to the
check) + C-2/C-3 (unified rc-consumer polarity in the CFG)** — one contract that unifies F1, the &&/||
under-execute, errexit, and Half-B subsumption. High-lock. Nothing reverted from wave-1; gw-1's F1 +
interceptor stand as interim special-cases superseded *in design* by C-1..C-4.

Open (human):
- **q-1a / q-1b** (C-1 interceptor shape — gate a clean rebuild).
- **wave-2 go:** build the C-4 rc-modeling contract next (unblocks everything, high-lock) vs the
  orthogonal lower-lock tracks (gw-3 backward/apply-3 — ch-scope-locked, and gw-4 found errexit is
  dead-weight until it lands; recursion-smoke/seam-finite). Cheap precursor: **fork-B** (verify the
  adversarial's ~SUSPECT "same-cell subsumption is soft" claim) to size the C-4 build.
