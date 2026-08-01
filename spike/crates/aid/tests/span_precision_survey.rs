//! The **caret-span-precision survey** (`AID-NEEDS:aid-caret-span-precision`, `24H` ack-8) — the
//! authoritative record of every diagnostic code's primary-span granularity and the CARET dispatch's
//! per-code verdict. This is a documentation fixture (the `aid/tests/` sanctioned home from the
//! dispatch brief), not a Research note; it is current-truth and rewritten in place when a code's
//! span granularity changes. The live enforcement lives in `diag_tidy.rs` (`SPANLESS_SITE_PAYLOADS`);
//! this file is the human/agent-readable WHY behind that allow-list's membership.
//!
//! Columns: code · current primary-span granularity · is a tighter honest span cheaply available at
//! the emit site? · verdict. Pins: `never-synthesize-a-span` (no code ever gets a fabricated span to
//! satisfy a count) · `law-lineno-identity` (`SourceFileId` disambiguates which file a span indexes)
//! · "coarsest-true beats precise-wrong" (a whole-mark/whole-command span that is CERTAINLY right
//! outranks a sub-token span that requires fragile source re-derivation).
//!
//! # A. Spanned codes (production `Diag::new`) — the tighten survey
//!
//! | code | current primary span | tighter cheaply available? | verdict |
//! |---|---|---|---|
//! | cmdsub-operand-top | whole command node span | operand word span — but the emit sees only resolved `ValueOf`s (no per-operand AST span); tightening needs value/effect re-threading | DEFER (kernel scope-growth; flagged `inv-superposition`) |
//! | site-unresolvable | site command span | no (the whole command runs) | KEEP |
//! | render-heredoc-refused | heredoc-bearing command span | no | KEEP |
//! | cmdsub-inner-nonleaf | inner command AST span | already the inner command | KEEP (tight) |
//! | redir-target-top | redirect command node span | the redirect target is `$(…)`-⊤ (no literal token to point at) | KEEP (coarse-true) |
//! | depth-2-positional-unthreaded | call site span | no | KEEP |
//! | syntax-unsupported | parser-provided construct span | no | KEEP (tight) |
//! | syntax-malformed | parser-provided construct span | no | KEEP (tight) |
//! | cfg-top-node | node AST span | no | KEEP |
//! | cfg-inline-refused | call site span | no | KEEP |
//! | cfg-builtin-shadowed | funcdef `name_span` | no | KEEP (tight) |
//! | predict-out-of-dialect | construct / EOF span | no | KEEP |
//! | predict-unterminated | EOF zero-width span | no (honest end-of-input) | KEEP |
//! | munge-name-invalid | emitted-name span | no | KEEP (tight) |
//! | munge-name-collision | source-name span | no | KEEP (tight) |
//! | reserved-namespace-squat | funcdef `name_span` | no | KEEP (tight) |
//! | missing-dialect-marker | first dialect construct span | no | KEEP (tight) |
//! | tolerates-unknown-dimension | whole mark span | the unknown token — but the mark stores `entity` as a bare String fragment with no per-token span; sub-token offset needs fragile source re-derivation | KEEP (the mark IS the honest unit) |
//! | tolerates-over-identity-dependence | vouch/mark span | no | KEEP |
//! | heavy-context-no-tolerance | body span | no | KEEP |
//! | lend-map-unknown-dimension | whole mark span | same as tolerates-unknown-dimension | KEEP |
//! | carry-netns-on-net-kernel-forbidden | store funcdef `name_span` | the `invariant:netns` line is not in scope; the store funcdef IS the offending declarant | KEEP |
//! | mark-brace-verdict-single-cell | mark span | the mark is already the small offending unit | KEEP |
//! | wrapper-entry-incoherent | entry funcdef `name_span` | no | KEEP |
//! | wrapper-peel-incoherent | predict funcdef `name_span` | no | KEEP |
//! | carried-across-substrate-axis | carried book site span | no | KEEP |
//! | wrapped-site-adoption-hint | wrapped site span | no | KEEP |
//! | footprint-incoherent (own-coord canary) | book command span | no | KEEP |
//!
//! **Survey conclusion (materially revises the dispatch map's "~8–15 tighten" estimate):** the cheap
//! sub-line tighten surface is essentially EMPTY. Almost every spanned code already anchors at its
//! honest granularity (funcdef `name_span`, parser construct span, whole mark). The three whole-mark
//! codes and `cmdsub-operand-top` are the only conceptual tighten candidates, and NONE is cheap: the
//! mark codes have no per-token span in the AST (the entity is an unstructured String fragment), and
//! `cmdsub-operand-top` would require re-threading operand spans through the value/effect kernel
//! (scope-growth, deferred + flagged). No span is widened or synthesized to hit a count.
//!
//! # B. Spanless codes plumbed to a real span by this dispatch (Part 1)
//!
//! | code | plumbed-to span | mechanism |
//! |---|---|---|
//! | touches-escalated | book command span | `d.node` → `ast.node(cfg.node(node).ast).span` in `merge_derived_footprints` |
//! | deriv-family-incomplete | book command span | same (`d.node`) |
//! | footprint-incoherent (malformed-derived site) | book command span | same (`d.node`); now matches its own-coord canary sibling, which was already spanned |
//! | resolver-conflict | oracle funcdef `name_span` (first declaring file) + `SourceFileId` | `ResolverSet::get(kind).name_span`; report bucketed per oracle file |
//! | resolver-provider-collision | oracle funcdef `name_span` + `SourceFileId` | same |
//! | reaches-conflict | oracle funcdef `name_span` (first declaring file) + `SourceFileId` | `ReachesSet::get(kind).name_span`; report bucketed per oracle file |
//! | reaches-provider-collision | oracle funcdef `name_span` + `SourceFileId` | same |
//!
//! # C. Spanless codes that STAY spanless (never synthesize)
//!
//! - **dangling-reference** — DEFERRED, not synthesized. The dispatch map grouped it with the
//!   resolver funcdef codes, but it does NOT emit from a `per_kind` loop and its honest span is the
//!   BOOK coordinate's origin site, not any oracle funcdef (pointing at the resolver funcdef would
//!   misattribute — the fix is a book entity typo or a genuinely-absent host entity, never the
//!   resolver). Its emit context (`dangling_diagnostics`) holds only the dangling `EntityCoord`, not
//!   the originating `CfgNodeId`; an honest plumb needs a coord→origin-node back-map threaded through
//!   `collect_resolver_coords`/`build_resolutions`. Left spanless with reason rather than pointed at
//!   the wrong bytes.
//! - **Genuinely siteless** (whole-stream / whole-file / whole-run verdicts): `cfg-errexit-unknown`,
//!   `effect-kind-disagreement`, every `records-*` framing fault, `escalation-policy`, every
//!   `whylog-*` durable-file refusal, `aid-unloaded-sibling-oracle`. These stay on
//!   `SPANLESS_SITE_PAYLOADS` permanently.
