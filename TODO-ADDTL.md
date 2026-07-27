(REWRITTEN AUTOMATICALLY, UNSTABLE. DO NOT REFERENCE IN OTHER DOCUMENTS.)

Dig through the design-docs in this repo; collate for me a list of 'undone design-work' that was either 1. mentioned by a human in-passing, but doesn't clearly map to any of the design-passes present; or 2. heavily pushed by a `plans/*.md` document, but seems like it may have gotten lost in the weeds. Sort higher: items with high design-consequences ("especially hard to unbake" or "can't be refactored"); and sort lower: items that seem known/deferred (in `TODO.md`, discussed by the human in recent design-passes, or clearly not-upfront-work.)

*Remove* complete items; they are in git history. Do not populate incomplete items with sub-lists of chunks that *are* complete, this should *only* mention incomplete work; it has a tendancy to become a work-log, which it very much should not be. Keep items *short*; the deails live in the design-docs they were written into / during.

Update-by-overwriting this section; keep this descriptive header/prompt, just replace items. Keep it short; collapse similar/related items into one entry; this shouldn't grow over ~10 items. It's to catch *major work*, not nits.

> Notes by me, the human, in >-blockquote.

## Stands between Dorc and live use

* [ ] **ssh-executor-pipe-completeness** — `dorc` must itself do the ssh'ing (`dorc apply host.tld <plan.sh`): host argument + system-`ssh` spawn at the `cli` edge, timeout-wrapped, captured stdout fed into the existing `dorc-records/1` admission path. Mechanize-ssh/executorless is settled law (`142:Resolution`), and `260` §5 (+`26A`) is THE adjudicated transport spec, consumable at N=1 (completion sentinel · config layering · host-key posture · timeouts); the single-channel whole-artifact cut is its sanctioned degenerate start (`260:dec-26-wire-v1`); proven shell at `Research/trial/apply/apply-run.sh`. Riders: real host/nonce/attempt into the records `Expect` (spike constants today — `plan/src/records.rs`); never silently weaken host-key verification; NB the usekeychain scar — composing with `~/.ssh/config` breaks on THIS Windows controller (`notes/26D` §4). Full seed: `notes/26D`.

* [ ] **live-acceptance-gate** — the closed loop (captured *real* probe output → `--results` → apply build) exists nowhere, even test-only: the e2e runner executes probe and apply separately, both against authored fixtures. Wire a bless/gate-tier (never hot-loop) real-ssh exercise — probe→records→plan→apply→verify against a local target. Substrate facts 2026-07-27: WSL2 Ubuntu present (real dash + apt-get); docker absent both sides (installing it is a human decision). WSL-sshd target first; container/throwaway-VPS tier opt-in later (`Research/trial/vultr.sh` is salvageable lifecycle tooling).

* [ ] **crlf-refuse-gate** — a Windows-authored book shipped to a Debian target dies on the kernel shebang exec (un-guardable at runtime); the ruled behaviour is refuse-loudly-with-the-one-line-fix, NEVER silent normalization (`260:dec-26-crlf`; `139` §5), re-checked on shipped bytes at apply time. Unbuilt; cheap; this exact authoring workflow (Windows checkout → Linux host) is live today.

## Demoted (real; does not block live testing)

* starter-oracle-stdlib — zero non-fixture oracles exist; important-and-pending, NOT blocking (human-ruled 2026-07-27: stdlib/multihost/first-blood ceremony have mostly stood in the way of experimenting; scrappy hand-written oracles are part of the experiment itself). On-ramp when picked up → `27Q` (§2 preconditions discharged); authoring trap: converged≠no-op adequacy, unmeasured → `24U` §2.
* book-acceptance-carve — the value-ladder for accepting unmarked/bash/zsh-ish *runbooks* (never oracles) is undesigned/unowned; design inputs banked → `276:rul-kwhichsh-oracle-scoped`, `276`.
* probe-safety-backstop — seccomp `socket(AF_INET)` observe + `--faithful` one-leaf-one-exec, both unowned; probe honesty rides author discipline until then → `077`, `24O` item-13.
* oracle-author-quality-bars — wrapper bar, carrier bar, adjudicability build-list (must land before kinds go community-shared) → `24S:A6`, `24T:P-A4`, `24S:A4`, root `AID-NEEDS.md`.
* kty-annot-punt (human-ruled 2026-07-12, recorded here to outlive the chat) — the spike itself IS the kTYANNOT experiment; "is not-using-EOL-comments livable?" is post-spike adjudication input, never upfront work; if inline survives, it must prove worth forgoing the comment-adjacent tooling ecosystem.
* precision-identity-residue — partial-member convergence · may-alias-default ruling (must never flip silently) · uniqueness-bit build → `277` §5/§6, `24O` item-25.
* kstate-and-cross-host — the `(verdict, content-key, freshness)` shape is retrofit-hostile, decide-shape-early; riders: host-as-adversary, wall-clock-keyed classes, rec-5, the `261:dec-timing-cache` fence question → `23O` §5.
* wrapper-payload-residuals — fs-view Hard cell (sequenced behind netns) · guard-insertion under ELEVATED lanes · become/doas prior-art ack → `27C`, `24S` §3b, `23J`.
* locator-dag-n-tier — per-host-forking DAG + transport-minted session correlation; first consumer is the multi-host era; re-grade the moment the ssh executor grows past one host → `111` dac-A.
* whylog-sensitivity — output-sanitization/secret-taint unbuilt while the whylog is now default-ON and holds raw host metadata; acceptable for a throwaway box, re-grade before real estates → `AID-NEEDS:law-whylog-is-sensitive`.
* catalog-ratchet-burn-down — 35 ratcheted codes' frozen examples have no mechanical tie to their emitters (silent drift, CORRECTNESS-graded); plus the one allowlist-row ask → `28F:finding-caseless-example-drift`, `28F:ask-munge-byte-render`.
* human-root-doc-queue (his voice) — fix-gsub-strip-claim · fix-flag-gloss-composition-not-contradiction · fix-kwhichsh-hedge-and-scope · fix-marker-gate-absent · smalls (skip-vs-elide render divergence, arity-gate idiom, typos) · "three possible outcomes" enumerating four · the dq-kOOB stamp line → the 2026-07-17 fix-review cut (full text in this file's git history).
* pending-ruling-queue (small sittings) — floors-ratification (`27U` §7) · decline-class starter-set (`27W` §0) · C8 operand-value display (`27U` §7) · prose-register schema (`282` §10; W4 landed in the r28-unify worktree, so the W5⇄W4 interlock is resolved and the sitting has transcript faces) · lint tc-leans (`27S` §5, `27T`) · `Consented`-knowability at first render + `--no-whylog` spelling (`28F`) · why-carries-risk-flag + apply-header-vs-byte-floor (`28I`) · syncthing `.stignore` repair (human-owned, `27U` §2).
* seams-grab-bag — streaming/TUI (rides weft) · retries/until · serial non-preclusion · escape-hatches + veto polarity · secrets timing (`26B:need-scrub-before-freeze`) · `24R` cheap-adds → each pointer is the live re-entry.

## Deferred arcs (pointers only)

MH2 version layer → `.claude/research/versioning-mh2/`, `270` §4 · r26 reactive/capture + multi-host revival → `26B`/`26C`, `270` §5 · r25 field-trial ceremony → superseded by the informal live run; tooling salvage at `Research/trial/` · DST rung ladder → `128` §3/§7 · r11/r12 research banks → `111` §4, `128` §8.

> on MH2: versioning (the simpler version of the concept, focused down to 'package-as-a-type-needing-special-attention, multi-providers, etc' ... and 'mapping oracle-written-for vs being-executed-on') needs close care, but it feels very deferrable compared to some of the critical core analyzer-design things that are affected by the "sh spelling" issues. continuing to defer.
