# 22F — ui-A integration surface: what the multi-mode CLI strained

> Round-22, ui-A (ru-25 / ru-20 ui-3). Append-only; confidence-marked. The deliverable is
> WHAT STRAINED, not the green corpus. Builder note, single-crate scope (`crates/cli`); the
> tc-* flags below go UP to the conductor, not silently settled.
>
> Commit: `231861c` (the only commit on `ai/r22-uia` past the `70cfa7a` base). One file
> touched: `spike/crates/cli/src/main.rs` (the thin driver — exactly the scope boundary).

## §0 What was built

A leading-mode-token dispatch over the existing single pipeline. Four invocation shapes,
one kernel call (`parse → cfg::build → value::analyze → effect::classify_with_why_diags →
compile_probe → build_plan`), the mode routing ONLY stdout/stderr:

- `dorc probe --book=B [-o O]…` — emit the read-only probe artifact (round-trip phase 1) to
  stdout; reads NO stdin (no results exist yet); builds no plan.
- `dorc plan --book=B [-o O]… < results` — the eliding-apply to stdout + the FULL advisory
  render surface (why-lens, unresolvable readout, stage notes/warnings, refusals, digest) to
  stderr. The ru-20 ui-3 "doubly-emit cited sections + their warnings to the console."
- `dorc apply --book=B [-o O]… < results` — BYTE-IDENTICAL apply bytes to stdout; stderr
  carries ONLY the error floor + the decision-digest (the rec-1 receipt-free off-ramp).
- `dorc --book=B [-o O]… [--debug-argv] < results` (NO mode token) — the legacy round-trip
  (probe THEN apply on stdout, full disclosure on stderr) the e2e harness drives, kept
  verbatim. Zero corpus change; the harness was NOT touched.

Gate chain (unpiped rc, from `spike/`): fmt 0 · build 0 · clippy -D 0 · deny 0 · test 0
(cli 13 incl. the new `advisory_filter` pin; workspace ~464 pass / 1 pre-existing ignore) ·
`sh e2e/run.sh` ×2 **99/99 identical, EXIT 0 both** · typos 0.

## §1 Strain — where the modes don't compose cleanly

- 22F-fd1 (+SURE) — **rec-1's two surfaces ALREADY physically existed; ui-A only NAMED them.**
  The pre-ui-A single-shot driver was *already* emitting the byte-floored artifact on stdout
  and the receipt/disclosure plane on stderr in one shot. The plan/apply split is therefore
  NOT a new architectural seam — it is a *projection selector* over one run: "print probe?
  print apply? show advisory plane?" are three independent booleans, and the four modes are
  three of the eight cells. This is the cleanest possible confirmation that the
  artifact-vs-render contract (rec-1) is real and was honored upstream of any UI: the UI fell
  OUT of the plane split, exactly as ru-20 predicted ("UIs are consumers, never contract
  subjects"). The corollary strain: there is NOTHING ui-shaped forcing the engine here — the
  whole mode surface is ~40 lines of routing. That is the *point* (ru-25: the spike hasn't
  accumulated enough cruft to make UI work unrepresentative), but it also means ui-A surfaced
  no engine-design tension by itself — the tensions it surfaced are all about the RENDER
  surface's contract (below), which is where ui-B (streaming) will live.

- 22F-fd2 (~SUSPECT, the load-bearing judgment) — **`apply`'s receipt-free console is a
  per-SEVERITY cut, and that is a genuine design choice, not a derivation.** rec-1 says the
  artifact is byte-floored & receipt-free and the render surface carries disclosure. But
  `apply` (the off-ramp) still has a console (stderr), and the question "what may it carry?"
  has no forced answer. The built line: `apply` keeps ERROR-severity diagnostics (incl. the
  arch-1 d-6 render refusals — a shippable artifact must never SILENTLY ship having refused a
  licensed elision) + the decision-digest (identity-plane, an always-on integrity signal —
  NOT a receipt), and DROPS warnings + notes + the why-lens + the unresolvable readout (the
  advisory plane). `plan` keeps everything. Rationale: the error floor keeps `apply` honest
  while receipt-free (it does not go blind); the digest is identity not receipt; everything
  else is advisory disclosure the off-ramp user did not ask for. → **tc-apply-receipt-floor**
  (flagged): is "advisory-suppressed, error-kept, digest-kept" the right cut for the off-ramp
  console? Three sub-questions the conductor/human should rule on:
  - 22F-fd2a — should `apply` carry the digest AT ALL? I kept it (drift signal, cheap,
    identity-plane). An argument against: a pure off-ramp `.sh > out.sh 2>/dev/null` user
    never reads stderr, so it is harmless either way; but a CI diffing `apply` stderr would
    see the digest move on any identity change, which is arguably *desirable* there.
  - 22F-fd2b — should `apply` carry WARNINGS (not just errors)? I dropped them. The gate-3
    floor only keys on `error[`, so dropping warnings is corpus-safe; but a warning is
    arguably "you're about to ship something suspicious," which an off-ramp user might want.
    The cut is defensible either way; I chose the stricter receipt-free reading.
  - 22F-fd2c — the render REFUSALS (d-6 heredoc) are Errors, so they cross even `apply`'s
    floor. That is deliberate (never silently ship a refused-elision artifact) and I believe
    correct, but it means `apply` is not *literally* silent on a refusal case — flag it so
    nobody later "cleans up" `apply` to zero-stderr and reopens the silent-ship hole.

- 22F-fd3 (~SUSPECT) — **`probe` mode has no plan, so no decision-digest — an asymmetry.**
  The digest hashes the plan+probe identity plane; `probe` builds no plan (it is phase 1,
  pre-results), so it emits no digest. → **tc-probe-no-digest** (flagged, low-stakes): the
  probe artifact bytes are themselves a deterministic function of the inputs, so a
  probe-only drift signal COULD hash just the probe. I did not build one (no current
  consumer; the round-trip digest already covers the probe via `decision_digest(plan, probe,
  …)`). Defer unless a probe-only drift signal is wanted.

- 22F-fd4 (+SURE, mild) — **the two-phase round-trip across separate invocations works, but
  the wire is unversioned and the harness never exercises the SPLIT form.** `dorc probe …`
  emits records-emitting sh; run it on a host, pipe its stdout into `dorc plan/apply … <
  records`. The format is the same `site <leafid> effect=… rc=…` grammar `parse_results`
  already consumes — so the split-phase round-trip is real (I verified `probe` stdout is a
  standalone `dash -n`-clean artifact, and plan/apply read the same records). BUT: the e2e
  corpus only ever drives the *combined* round-trip (probe+apply in one process, results fed
  from a fixture file), so the SPLIT path (`probe` output actually executed, its records fed
  to a separate `plan`/`apply`) is UNTESTED end-to-end here. → **tc-probe-results-roundtrip**
  (flagged): the leafid-keying is the contract that lets a separately-run probe's records
  bind back to a separately-run plan's leaves; `inv-site-keyed-results` guarantees the id
  spaces match WITHIN one binary+inputs, and since plan/apply re-derive the same `site_order`
  from the same book, the ids are stable across the two invocations *as long as the book is
  byte-identical between the probe call and the plan call*. That invariant is currently
  implicit. ru-18's probe-TAPE versioning (cer-2) is the natural home for making it explicit;
  ui-A did not build it (out of scope), but the split-phase CLI is the first thing that can
  actually DESYNC on it (run probe against book v1, plan against book v2 → leafids silently
  re-map → wrong elision, kFAIL-perform-unsafe). Recommend a book-hash echo on the probe
  artifact + a plan-side check before any split-phase mode is blessed for real use.

## §2 What should reshape ui-B (streaming) or arch-4

- 22F-fd5 (+SURE) — **ui-B's streaming surface IS the `plan` advisory plane, incrementalized.**
  ui-A made concrete that the "render surface" is, today, a batch of stderr lines emitted
  AFTER the whole pipeline finishes (stage diags, then probe, then plan, then why-lens, then
  digest — a strict sequence). `plan` is the mode whose console is the human-facing preview;
  ui-B's streaming proof (ANSI updates as probes return) is precisely *this console, but
  emitted incrementally as the probe phase resolves per-site* rather than all-at-end. So ui-B
  should extend `plan` (not the round-trip, not `apply`): the streaming unit is the per-site
  advisory line, and the logical-clock/timing dependency the human expects (ru-25) is the
  per-site probe-return ordering. The byte-floored artifact (`apply`) is explicitly NOT a
  streaming surface — it is the final off-ramp; keep ui-B off it.

- 22F-fd6 (~SUSPECT) — **the advisory-vs-error severity cut (fd2) is the contract ui-B must
  also honor.** Whatever ui-B streams, the same rule should hold: the streamed pretty-render
  is a `plan`-tier surface (full advisory), and an `apply`-tier streamed render (if one ever
  exists) stays receipt-free. ui-B should consume the SAME `advisory_filter` decision, not
  invent a second severity policy — else the two render surfaces drift (the exact
  two-sources-of-truth hazard dac-B warns about, one level up).

- 22F-fd7 (-GUESS) — **arch-4 (the probe-tape replay gate) and the split-phase CLI want the
  same book-identity check (fd4).** arch-4's cer-2 (tape version-tag + binary-hash, replay
  REFUSES on mismatch) and the split-phase leafid-stability invariant are the same property
  viewed from two angles: "the thing that produced these records must match the thing
  consuming them." If arch-4 builds a book/input hash into the tape, the split-phase CLI gets
  its desync guard for free (the probe artifact carries the hash; plan checks it). Worth
  cross-locking when arch-4 is specced, rather than building two hashes.

## §3 Process / residual

- The diag_tidy catalog-completeness gate (226 §1, finding-gate-exists) FIRED on the new
  cli test's throwaway `DiagCode("…")` fixtures — caught that they were neither allow-listed
  nor migrated. Resolved by using the recognized fixture slugs (`x-err`/`x-warn`/`x-note`,
  `is_test_fixture_slug`). +SURE this is the gate working AS DESIGNED (it scans the whole
  tree incl. cli tests) — minor evidence that the tree-wide reachability half of the
  catalog gate is real and not a no-op.
- No engine/kernel change; no human-doc edit; no BLESS; no push. The legacy invocation is
  byte-for-byte the harness's, so the 99/99 corpus is untouched by construction (the round-
  trip path through `run()` is unchanged modulo the `report` → `report_at(advisory=true, …)`
  rename, which is identity for the round-trip's `advisory=true`).
- Spike/CLAUDE.md drift unaddressed (not my file to edit): it still says "99-case corpus"
  in one place and the count is 99 at HEAD, so that one is currently accurate; flagging only
  that I did not touch it.
