# 16O — half-assed Option B: weirdos through the e2e front-end

> **Status (2026-06-06): spike, round-16 — the "does the architecture HANDLE weirdos"
> pass.** Minimum-effort / maximum-state-space: weird books + host-answers thrown at
> the whole apply-2 chain via the sh-mechanized CLI e2e (`spike/e2e/`), not polish.
> Append-only (…16N → 16O). HEAD `1a8f592`. Confidence-marked.

## 0. The finding (one line)
**Every weirdo falls to the SAFE direction — run, never wrongly elide — and never
crashes (`inv-no-throw`).** That IS "the architecture handles weirdos": under
unmodeled / un-oracled / poisoned / consumed / garbage inputs, the apply degrades to
"just run it" (`kFAIL-perform`), which is apply-1 (the trivial fallback). +SURE.

## 1. The weirdo cases (each a `cases/<name>/`, mechanized by `e2e/run.sh`, all green)
- **toprejected** — `for … done` (⊤-rejects, loud on stderr) + a trailing `apt-get
  install curl`. The loop's commands aren't leaves; the ⊤ poisons `curl` downstream ⇒
  **empty probe, everything runs** (curl runs despite a "converged" host-answer —
  poison wins). The whole chain stays total.
- **no-oracle** — a book with no `-o` ⇒ no effects known ⇒ empty probe ⇒ install runs
  (can't elide what you can't analyze).
- **kill-then-install** — `purge nginx; install nginx` ⇒ install is `EstablishWritten`
  (the note-162 O-1 wrong-skip guard) ⇒ runs despite "converged". End-to-end proof of
  the guard.
- **consumed-output** — `install nginx | tee log` ⇒ the observable gate (def-4) fires
  end-to-end: install runs despite converged+ambient+probed (stdout consumed).
- **garbage-stdin** — malformed probe-results (junk lines, bad verdicts, comments) ⇒
  parsed leniently, unrecognised facts fold to `Unknown` ⇒ run; no crash on junk.
- **guarded** — a multi-line `if true; then  apt-get install nginx; fi` + `echo done`
  ⇒ the establish is elided **in place** (commented), with `if`/`fi`/`echo` and the
  3-space indent preserved. Shows `render_apply` preserves control-flow structure.

## 2. Deliberately NOT done (the half-assed line)
- **Probe flakiness / unreliable-oracle (16A §3)** — a seeded-PRNG probe that returns
  `Unknown` with some probability — DEFERRED to a real round (needs `hostsim` PRNG
  work; not half-assed-cheap). The existing 64-seed DST + garbage-stdin cover the
  cheap host-answer weirdness.
- **Mutative-oracle catch** — `hostsim` already unit-tests the `kFAIL-withhold`
  monitor; an e2e mutative-probe needs the (unbuilt) executor. Deferred to Option C.
- Per the human: parser/lexer are "minimal to what works" — no weirdo here required
  expanding them (the multi-line `if`, pipes, redirs, `for`-⊤ all parsed/⊤-rejected as
  modeled). A weirdo that hit a syntax limit would be fixed by adjusting the fixture,
  not the grammar.

## 3. State
Whole workspace green (105 unit/integration + 1 ignored) · clippy clean · `e2e/run.sh`
**9/9** round-trips (3 baseline + 6 weirdos). `mise exec -- cargo build -p dorc-cli &&
sh e2e/run.sh`.

**NOTES INDEX:** …16M apply-2 compiler · 16N CLI round-trip + sh e2e · 16O (this —
weirdos: the architecture degrades safely to run).
