# 16N — the CLI round-trip front-end + sh-mechanized e2e (viability proof)

> **Status (2026-06-06): spike, round-16 — stretch goal, the "Does Stuff" proof.**
> A minimal `dorc` CLI exercises the whole apply-2 chain on real shell-script files,
> mechanized in sh (no Rust harness, per the human). Append-only (…16M → 16N). HEAD
> `9e72af5`. Confidence-marked.

## 0. The front-end (`crates/cli/src/main.rs`, binary `dorc`)
`dorc --book=<book.sh> [-o <oracle.sh>]...` — one invocation, the full round-trip:
1. lift the `-o` oracles → one kind-index; parse+cfg+classify the book (shared interner);
2. **emit a read-only PROBE on stdout** (`compile_probe` → `ProbePlan::render_sh`): one
   `# probe: kind:entity` + the oracle's verbatim probe body, per probe-able fact;
3. **read probe RESULTS on stdin** — `kind:entity converged|diverged|unknown` per line
   (the simulated host; in a deployment these come from running the probe). An
   unreported fact ⇒ `Unknown` ⇒ run (kFAIL-perform);
4. **emit the ELIDING-APPLY on stdout** (`build_plan` → `Plan::render_apply`): the book
   verbatim with already-converged command-lines commented out.
Plumbing only — `inv-determinism` exempts `cli`; the kernel it calls is pure. Two new
`plan` bits: `pub fact_label` (so the CLI matches result-lines back to facts by the same
label the probe emits) and `Plan::render_apply` (book-faithful, line-granular — contrast
the flat `render_sh`).

## 1. The mechanization (`spike/e2e/`, sh — not Rust)
`e2e/run.sh` (POSIX): locates `target/{debug,release}/dorc[.exe]` (or `$DORC`), and for
each `cases/<name>/` feeds `probe-results.txt` on stdin to
`dorc --book=book.sh -o <each *.oracle.sh>`, diffing stdout against `expected.out`
(CRLF-normalized). `BLESS=1 sh e2e/run.sh` regenerates the goldens. Run: `sh e2e/run.sh`
(after `mise exec -- cargo build -p dorc-cli`). It is NOT a `cargo test` — it's the
sh-driven e2e gate.

## 2. Cases (each a self-contained subdir: book + oracle(s) + probe-results + golden)
- **converged** — lone `apt-get install nginx`, `package:nginx converged` ⇒ install elided.
- **diverged** — same book, `package:nginx diverged` ⇒ install runs (verbatim).
- **two-oracles** — `install nginx; systemctl enable nginx; echo provisioned`, two oracles
  (`-o` ×2, apt + systemd), both converged ⇒ both oracled lines elided, the un-oracled
  `echo` (pure builtin) runs. Exercises multi-oracle + cross-provider.
All 3 green.

## 3. What this proves (the viability bar)
The full source → analyze → compile-probe → (simulate host) → eliding-apply chain is wired
and runs on actual `.sh` files, end to end, at a TUI. NOT proven here: executing the probe
or apply on a real host (Option C executor), apply-3 targeting, fidelity of the probe
projection (it emits the oracle body with `$1` unbound — illustrative, not yet per-entity).

**NOTES INDEX:** …16L test audit · 16M apply-2 compiler · 16N (this — CLI round-trip +
sh e2e, the viability proof).
