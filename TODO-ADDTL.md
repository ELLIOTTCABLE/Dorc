(REWRITTEN AUTOMATICALLY, UNSTABLE. DO NOT REFERENCE IN OTHER DOCUMENTS.)

Dig through the design-docs in this repo; collate for me a list of 'undone design-work' that was either 1. mentioned by a human in-passing, but doesn't clearly map to any of the design-passes present; or 2. heavily pushed by a `plans/*.md` document, but seems like it may have gotten lost in the weeds. Sort higher: items with high design-consequences ("especially hard to unbake" or "can't be refactored"); and sort lower: items that seem known/deferred (in `TODO.md`, discussed by the human in recent design-passes, or clearly not-upfront-work.)

*Remove* complete items; they are in git history. Do not populate incomplete items with sub-lists of chunks that *are* complete, this should *only* mention incomplete work; it has a tendancy to become a work-log, which it very much should not be. Keep items *short*; the deails live in the design-docs they were written into / during.

Update-by-overwriting this section; keep this descriptive header/prompt, just replace items. Keep it short; collapse similar/related items into one entry; this shouldn't grow over ~10 items. It's to catch *major work*, not nits.

> Notes by me, the human, in >-blockquote.

This file is the *porch* of `ROADMAP.md`: only items the human has NOT typed an ack on. Nothing
here is owed or promised by anyone. When the human acks a row it moves to the roadmap (owed;
scheduled only on a second, explicit "next"); when they nack it, delete it. Anything that
already sits in `TODO.md` (the human's list) or in a roadmap row does not belong here.
Engineering residue that rides a future lane belongs beside that lane's design-of-record or
in `30O:register-and-steering-debt`, not here.

## Design consequences (hard to unbake if missed)

* [ ] **kernel-shape residue to decide early** — the `(verdict, content-key, freshness)`
  shape for kSTATE/cross-host is retrofit-hostile (`23O` §5; riders: host-as-adversary,
  wall-clock-keyed classes, the `261:dec-timing-cache` fence) · the may-alias default
  ruling must never flip silently, plus partial-member convergence and the uniqueness bit
  (`277` §5/§6, `24O` item-25) · the `30W` context-slot generalization is the same class
  (that one IS on the roadmap; these are its unacked siblings).
* [ ] **slow-planner-cost-model** — foreign convergence/preview checks can take minutes and
  planning-duration is itself staleness; no design exists; wants sitting before the
  terraform/ansible-class delegation oracles are authored (`26L` §11, `KNOBS:kPROBING`
  check-tax, `26Lb` frame-conductor-cost-model, `26M` queue item `q-entry-economics`).
* [ ] **probe-safety-backstop** — the seccomp `socket(AF_INET)` observe and the
  `--faithful` one-leaf-one-exec are both unowned; probe honesty rides author discipline
  (`077`, `24O` item-13). The human's own `TODO.md` distrusts seccomp-in-core without a
  threat model — this row is the reminder that *nothing* holds the line today.

## Pre-publication and quality bars

* [ ] **oracle-author-quality-bars** — wrapper bar, carrier bar, adjudicability build-list
  must land before kinds go community-shared (`24S:A6`, `24T:P-A4`, `24S:A4`, `AID-NEEDS.md`).
* [ ] **wrapper-payload-residuals** — the fs-view Hard cell (sequenced behind netns) ·
  guard-insertion under ELEVATED lanes · become/doas prior-art ack (`27C`, `24S` §3b, `23J`).
* [ ] **deletes-are-hard** — the establish/kill model's product argument (`061` theme 6);
  sitting-visible, never sat.

## Re-grade at the next estate change

* [ ] **receipt-sensitivity** — output sanitization / secret taint are unbuilt while receipts
  are default-ON and hold raw host metadata; fine for a throwaway box, re-grade before real
  estates (`AID-NEEDS:law-receipts-are-sensitive`).
* [ ] **locator-dag-n-tier** — the per-host-forking DAG + transport-minted session
  correlation; first consumer is the multi-host era; re-grade the moment the ssh executor
  grows past one host (`111` dac-A).
* [ ] **posh-leg-of-the-floor-is-unexercised** — `printf` is not a posh builtin, so under the
  corpus's `PATH=mocks-only` rail no shipped oracle body's emissions have ever run under posh;
  the opt-in `mise run test:floor` lane proves six sentinel manifests only
  (`spike/CLAUDE.md` floor-differential-lane-opt-in, `28P`).

## Testing residue the `30X` rebuild may subsume (check before re-raising)

* [ ] the why-surface close's unwitnessed branches (the comparison seat's authentication
  asymmetry + non-following read; the `vouched-severally` voice-set) · the loom `run:`-lane
  widening for receipt-driving cases · a typed order accessor for `--receipt <file>` · whether
  `receipt_route.rs`/`spine_baseline.rs` migrate into the corpus (`30Va`; lanes B/C of `30X`
  touch all of these) · the DST rung ladder (`128` §3/§7).

## Smaller, still model-flagged

* [ ] **human-root-doc-queue (his voice)** — fix-gsub-strip-claim ·
  fix-flag-gloss-composition-not-contradiction · fix-kwhichsh-hedge-and-scope ·
  fix-marker-gate-absent · smalls (skip-vs-elide render divergence, arity-gate idiom, typos) ·
  "three possible outcomes" enumerating four · the dq-kOOB stamp line → the 2026-07-17
  fix-review cut (full text in this file's git history).
* [ ] **seams-grab-bag** — retries/until · serial non-preclusion · escape-hatches + veto
  polarity · secrets timing (`26B:need-scrub-before-freeze`) · `24R` cheap-adds; each pointer
  is the live re-entry. (Streaming/TUI is on `TODO.md`.)
* [ ] **docs-tree gaps after the 2026-09-02 re-synthesis** — `writing-oracles/03` lacks the
  `30S` pin-or-sever obligation (a new subsection, not a small fix) · `04` lacks the
  mark-block / trailing-bind paragraph · the contract page's §5k lists three unruled
  identity-tier members with "write nothing against these yet" (cut if absence is preferred).
* [ ] **`26L` §15's owed investigations** (real glue strawmen, live sibling docs, the minimal
  native-runner shape, preview contract, planner costs) — exploration-tier; the human's
  standing lean is punt-for-now; listed so the next meta-orchestration sitting starts there.
