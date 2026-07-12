# 27L — the language-item sitting: kWHICHSH scope · the `unsafe` hatch · the churn policy

AI-scribed (design-rubber-duck sitting, 2026-07-12), human-adjudicated. The durable record of
the TODO-ADDTL item-1 sitting ("the Dorc language proper" — human-selected as the
final-Fable-window design dive), run in parallel with block-settle under an explicit carve:
this sitting owns the **superset boundary**, the **kWHICHSH carve**, the **`unsafe` hatch**,
and the **epoch/churn policy**, plus two fold-ins (**`trap`**, **KNOBS registration**). It
does NOT own the analyzer stopping-point (`271` task 9), the mark/entity grammar (`271`
tasks 1/12), or the reentry token (`271` task 6 — closed there; its outputs are inputs here).

Authority: root docs and human-TYPED rulings outrank this; entries without "(typed)" are
conductor-drafted and awaiting ack (silence ≠ ack). Naming per `270` §1.

---

## Closed this sitting

### rul-kwhichsh-registered · rul-kwhichsh-oracle-scoped  (typed 2026-07-12)
`kWHICHSH` now exists in KNOBS (it had been cited for two rounds with no entry): poles
`kWHICHSH-minimum-lcd ↔ kWHICHSH-maximum-gcd` (pole names human-edited in place), human-voice
source = DESIGN's *"POSIX" sh* section, dialect lean = r23-h3 (`notes/23F`: dash-ish,
"POSIX2024-ish", `local` carved in). The scope ruling: **the knob as leaned is
ORACLE-scoped; book-acceptance is a separate, open question.** Far pole banked verbatim-ish:
even non-sh books (fish et al.) are imaginable someday, provided the thing-under-analysis
calls unix binaries, has positional arguments, and can itself invoke/consume stripped-POSIX
oracles. The public *why* of oracle-side strictness is shareability, as worded in the KNOBS
entry.

### rul-unsafe-is-bare-sh  (typed 2026-07-12 — "I see no reason to grow warts and cruft for a second version of that"; closed)
The long-owed Rust-`unsafe`-equivalent escape hatch (STALENESS-AUDIT drift-language; `23O`
§5's "human-required hatch... in no doc") is **discharged by identification, not by
building**: the bare-real-shell-head escape of `271:rul-dorc-prefix-head-synthesis` IS the
hatch — an unmarked `sh` head walls its payload deliberately; analysis descends hint-lane
only and licenses nothing. No second construct will ever exist for this. Riding notes:
multi-line regions fall out with zero new syntax as `sh <<'EOF' … EOF` (already inside `24T`
pin1's stdin-shape scope; real sh; single-quoted delimiter, quote-hazard-free); and the
wished-for taint semantics (loses CFG-totality; subgraph permanently unskippable) is exactly
the standing unmodeled-wall machinery — there is nothing to build.

### rul-verdicts-never-stable  (typed 2026-07-12, emphatic) — the churn policy
Dorc does **not** promise cross-version verdict stability for `dorc plan`; the core mode
explicitly gets better without notice. The `# dorc-lang/vN` marker gates **language-warts
only** — it exists to keep the syntax parsable and agile through redesign — and is never a
promise to reproduce **semantic** warts to keep somebody's CI green. This discharges
`24O:dq-verdict-churn-policy` at design tier. The stability ledger, stated once:
**syntax = marker-gated · `__role` names = permanent (`24M`) · verdicts =
unstable-and-improving, disowned.**

Banked names (typed: "bank both, we'll roll with them"):
- **plan-as-API** — the named failure-mode: treating plan output as a cross-version-stable
  interface (canonical nightmare: admins CI-gating on plan shape).
- **verdict-pinning** — the named, *disowned* someday-feature ("someday we may offer
  `dorc <scaryfeature>`; we don't now"). Human sizing on record: genuinely useful, no
  visible route through the murk of ops; a Hard Problem "on the order of ~all of the rest
  of Dorc combined."
- Rider (conductor-drafted, un-nacked): `dorc plan --exit-code` (the `24R` §2d cheap-add)
  *inherits* verdict-churn — a smarter engine or one newly-installed oracle legitimately
  flips run→elide and changes the exit code — so its contract must be worded as gating
  **divergence-of-world**, never plan shape. The wording obligation travels with whoever
  builds the flag.

### The book-tolerance shape  (typed leans; the design itself stays tabled with zsh/bash)
- **lean-book-tolerance-binary** (typed): for artifact-modifying tiers there is no semantic
  middle-ground — either FULL dialect support (a real per-shell semantic model: rc/flow/
  expansion at errexit-door grade) or parse-warn-comment only. "Inject POSIX into the middle
  of a bash book and expect sanity" is rejected as a papered-over Hard Problem: license-
  bearing edits need dialect-accurate semantics of the *surrounding* code (`pipefail`
  pipeline-rc, `((0))`, `[[`, arrays-feeding-argv), not merely evaluator-compatibility of
  the inserted bytes.
- **direction-drift-report-tier** (conductor-drafted, awaiting ack): the one middle tier
  that survives the binary because it is license-free — parse + probe + *display only*: a
  drift-report plan that mutates no artifact and licenses nothing. Same pattern as the
  bare-head hint-lane row of `271:rul-dorc-prefix-head-synthesis`, applied at book scale;
  the `24R` gap-inhost-drift position needs nothing more.
- **lean-byte-honesty-is-the-priced-feature** (typed lean): POSIX books (and any future
  fully-supported dialect) keep the byte-honest plan — the on-screen bytes ARE the bytes
  that ship; never traded away. A future trampoline tier for foreign dialects would *pay*
  byte-honesty as the named cost of that shell choice (per-site trampoline ceremony cannot
  be honestly shown at full fidelity); the drift-report tier edits nothing, so the question
  never arises there. Two mechanisms accepted over one, on user-attention/honesty grounds.
- **banked — finding-guard-bytes-travel** (tabled): Dorc-inserted guard/oracle bytes execute
  under the *book's* shell. Mostly a zsh knife (word-splitting inversion), largely not a
  bash one (evaluator-compat holds for defensive POSIX bodies; bash's gap is
  license-analysis, above). Banked mitigation shape: the reentry trampoline
  (`( dorc-sh -c '…check…' ) || original-line`), which composes with `271` task 6's
  shim/binding machinery.
- **banked — strain-inverse-wall** (human): the sharpened wall "probes are book-cfg +
  oracle-bytes, never book-bytes + oracle-bytes" may want an inverse twin — oracle code
  never transports into book code; only synthesized cfg does. Test it whenever zsh/bash
  unparks.
- **zsh grading** (human): zsh-loss is not-a-cheap-loss. The parser half is bounded,
  LLM-responsive, differentially-testable work (two living binaries to test against); the
  full-support gate is the *semantic model*, never the grammar. Expect site-finding-grade
  zsh parsing to be cheap whenever wanted.
- **errand-render-kit-floor** (banked; human near-typed "almost want to try it"): a
  throwaway cost-sizing spike for a build round — one implementor, brief: "figure out how
  to support fish/zsh/ksh/csh site-finding + connective-render all at once with minimal
  effort; give up and throw your work away if the going gets rough." Natural slot: after
  block-rebuild's render churn settles. Sizes the trampoline tier without designing it.
- Prior-art note for any of the above: Vendor/ already carries mvdan-sh (interleaved
  POSIX+bash+mksh grammar) and tree-sitter-bash (error-tolerant by construction); an
  error-tolerant parser posture is also the substrate any future LSP needs (TODO.md:8–11).

---

## Still open in this sitting's carve

- **THE superset-boundary statement** — the primary question; deliberately last.
- **trap** (fold-in; genuinely undesigned — `plans/064`:13's one-liner is the entire corpus
  treatment; `set -e` earned round 20V, trap never did). Proposed close (conductor, cheap):
  (t1) fixture-errand — pin what tip actually does on a trap-registering book
  (silently-ordinary-command would be a soundness bug; wall is fine); (t2) v1 disposition —
  trap is recognized-but-unmodeled: registration walls loudly with a named hint; deeper
  modeling (handler body as implicit may-run edges at every subsequent point) banked as a
  language-round item. Softener, unverified: elision's StandIn rc-reproduction means
  trap-ERR firing behavior is ~preserved under elision, so the deep interaction is narrower
  than it first looks.
- **Awaiting-ack ledger:** direction-drift-report-tier · the `--exit-code` wording rider.
  (The KNOBS entry itself was human-edited in place — counts as acked.)

## Pointers
KNOBS `kWHICHSH` (registered; human-edited) · TODO-ADDTL item-1 (this note is that
sitting's durable record) · `plans/271` (the adjacent block-settle ledger; carve respected)
· `notes/24O` item-7 (churn — discharged at design tier here) · `notes/23O` §5 language
item (`unsafe` hatch discharged; superset statement still owed).
