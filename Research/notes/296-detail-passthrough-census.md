# 296 — the `detail`-passthrough census (x2c)

AI-authored (Opus builder, lane x2c, 2026-07-30; worktree `.claude/worktrees/loom-x2c`, branch
`ai/r28-loom-x2c`). The audit `282` §8 and `28L:fnd-284-landed-state` asked for: every payload
field that rides a passthrough hole, classified as somebody else's bytes or as words we composed.
Measured at `6b23c5d8`, by reading every emit site.

Charter: `282:rul-passthrough-type-gated` · `AID-NEEDS:law-codes-vary-by-world-not-grammar` ·
`aid-is-the-describe-plane`.

## §1 — The count, and the correction to the inherited list

The hand-off list said 18 codes, keyed by the `detail` NAME. Both halves were wrong:

- **23 payloads carry a `detail` field**, not 18 (the five invocation/`dorc-sh`/transport codes
  arrived after that list was written).
- **The name is not the class.** `lint-tool-output-unparsable.output` is a genuinely foreign relay
  — an external linter's own bytes — and the `detail`-name heuristic never saw it. One member of
  the population was invisible to the very test that defined the population.

That is the finding the type-gate exists to make impossible, and it is why the census is by EMIT
SITE rather than by field name.

## §2 — The classification (23 + 1)

**(a) GENUINELY FOREIGN — 5.** Every one is a platform or tool speaking; none of these bytes were
composed on our side of the boundary.

| code | field | edge |
| --- | --- | --- |
| `cli-file-unreadable` | `detail` | `std::io::Error` (the residual read-failure arm) |
| `cli-shim-dir-unwritable` | `detail` | `std::io::Error` (the `--shim-dir` write) |
| `dorc-sh-script-unreadable` | `detail` | `std::io::Error` |
| `dorc-sh-exec-failed` | `detail` | `std::io::Error` (the exec of stock `sh`) |
| `lint-tool-output-unparsable` | `output` | captured external-linter stdout — NOT named `detail` |

**(b) OUR COMPOSED WORDS — 17.** Split by how many distinct sentences one code carries, because
that is what decides the remedy.

*One sentence, values interpolated ⇒ the sentence moves into the register with typed holes. Ten:*
`site-unresolvable` (mixed — see below) · `cfg-errexit-unknown` · `cfg-builtin-shadowed` ·
`effect-kind-disagreement` · `carried-across-substrate-axis` · `wrapped-site-adoption-hint` ·
`wrapper-entry-incoherent` · `wrapper-peel-incoherent` · `aid-unloaded-sibling-oracle` (its hole
was never a sentence at all — only a joined list) · `footprint-incoherent` (two, both constant).

*N distinct sentences under one slug ⇒ needs a typed reason, not a hole rename. Six, ~55 sentences:*

| code | distinct sentences | where |
| --- | --- | --- |
| `predict-out-of-dialect` | ~21 | `oracle/predict/parser.rs` `fail`/`fail_here` |
| `syntax-unsupported` | ~16 | `syntax/parser.rs` `unsupported`/`push_unsupported` |
| `syntax-malformed` | ~7 | `syntax/parser.rs` `push_malformed`/`expect_reserved` |
| `cfg-inline-refused` | 7 | `analysis/cfg.rs` — seven refusal paths |
| `predict-unterminated` | 4 | `oracle/predict/parser.rs` `true_with` |
| `whylog-corrupt` | 4 | `plan/whylog.rs` `corrupt` |
| `cfg-top-node` | 2 | `analysis/cfg.rs` depth-bound vs unsupported-construct |
| `escalation-policy` | 3 | `cli/main.rs` — one per escalation dial |

**(c) MIXED — 2.**

- `site-unresolvable` — our disclosure sentence PLUS two book-derived values (the named sites, the
  quoted first command). Split: sentence to the register, both values sealed.
- `transport-not-attempted` — two producers, one per world: the platform's spawn error
  (`transport/child.rs`) and our own sentence about an unusable run marker (`cli/transport_edge.rs`).
  Left a passthrough deliberately; see §4.

## §3 — What the seal is

`aid::foreign` holds two types, because hostility and encoding are orthogonal
(`sinv-hostile-sensitive-orthogonal`):

- `ForeignBytes` — the raw seal. Private field; constructors are `from_os_error(&std::io::Error)`
  (typed edge, needs no fence) and `from_io_edge(&str)` (the unavoidable bare-str edge, loudly
  named and lexically fenced by `foreign_edge_constructor_is_fenced`, the `admit_fixture_records`
  precedent). No raw accessor: the only ways out are the two sink encoders.
- `ForeignText` — the encoded seal, what render parts store. Constructible only from a
  `ForeignBytes` through the display seat, plus one crate-private `already_encoded` door for the
  weft span map handing its own foreign runs back.

`Said::Foreign` and `RenderPart::ForeignText` carry `ForeignText`, closing the effectively-public
variant-field hole. `ParamText::{Ours, Foreign}` is what `params_of` yields, so the catalog decides
a hole's class by the VALUE's type; `is_foreign_param` is deleted.

Bounded deviation, stated plainly: `weft::Run::foreign` still takes `impl Into<String>`, because
`weft-deps-nothing` forbids weft knowing a Dorc type. The seal binds one level up at
`aid::weave::foreign`, which is the only Dorc-side route into it.

## §4 — Landed, and residue

LANDED: the seal · the type-gate · the five (a) relays wrapped at their edges · the ten
one-sentence de-passthroughs · the `TopCause::describe` migration (7 phrases out of the DECIDE
plane into arrangement components via `top_cause_slug`, the `remediation_hint_slug` shape; `core`
now holds zero user-facing strings).

RESIDUE, with its reason:

- **`296:tc-many-sentences-one-slug`** — the six N-sentence codes (~55 sentences) are NOT split.
  The brief's remedy is world-variant sibling codes, and that remedy is structurally blocked at
  this scale: `defining-case-catalog` gives every code exactly one defining case and the
  `DEFINING_CASE_RATCHET` is SHRINK-ONLY, so ~55 new codes demand ~55 new defining cases and the
  ratchet cannot absorb them. The proportionate shape is the one this lane just proved on
  `TopCause`: a typed reason enum in the emitting crate, an enum→slug map in `aid`, and one
  arrangement prose-component per reason — zero new codes, zero ratchet movement, and every
  sentence lands in a registry. Choosing between the two is a conductor ruling, so it is flagged
  rather than settled (`inv-superposition`).
- **`296:tc-transport-not-attempted-is-two-worlds`** — splitting it is a sibling pair, not a hole
  rename, and its register is `[unwritten:]` today so nothing renders either sentence. Left alone
  rather than silently dropping the platform's spawn error.
- **`296:fnd-registry-words-escape-the-ascii-law`** — reclassifying a hole from foreign to ours
  changed its encoding (the measured sink escapes; the plain one blanks), which surfaced `⊤` inside
  two `cfg.rs` details and `⊄` inside `survival.rs`'s footprint detail. All three are respelled and
  their three `ASCII_SWEEP_ALLOWLIST` rows deleted — the sweep's shrink-only ratchet moved in the
  right direction as a side effect. But the same glyph sits in `unmodeled-wall-inventory`'s MESSAGE
  REGISTER, and registry words are never encoded, so it reaches output raw — a `weft-ascii-forever`
  hole the allowlist still excuses rather than closes.

## §5 — Census delta at the X2b fold (`e0c0530d`)

X2b's 22 new cases + 4 survival cases minted NO new payload struct and no new `detail`-class field
(`git diff` over `aid/src/diag.rs` across the fold is empty of `pub struct` / `pub detail`), so §2
is complete at the merged state. What they DID mint is honest firing worlds for four codes this
lane touched — `effect-kind-disagreement`, `wrapped-site-adoption-hint`, `wrapper-entry-incoherent`,
`footprint-incoherent` — so those examples are now re-derived from a real emit rather than a
fixture. Count at the merged tip: 10 `detail: String` remaining, exactly the deferred N-sentence set
plus `transport-not-attempted`; 4 `detail: ForeignBytes` plus `LintToolOutputUnparsable.output`.
