# 284 — round-28 user-sourced-information-tracking: owed/wanted mechanical work

Brief hand-off for whoever runs the secrets round. NOT security analysis — just the
taint-tracking *mechanical* surface round-28 touched or wants, so the secrets-round
lane knows what's already shaped and what's owed. Sited in quarantine per the human's
direction (adjacent to an opaque sibling's lane). AI-authored (Fable conductor,
2026-07-20). Keep this current-or-delete; do not over-grow.

## What round-28 BUILT that a taint system can key on

- **`core::tagged::Region::ForeignText(param)`** (the `282` tagged render, folded) — the
  render span map already CLASSIFIES passthrough text as a distinct region class,
  separate from our own template literals. Today it classifies conservatively via a
  `detail`-name heuristic (`is_foreign_param` in `core::catalog`), NOT a type. The class
  exists; the type-gating does not.
- **The `core::room` sealed-construction pattern** (aid evidence plane, r27) — the
  precedent for a value constructible ONLY at a controlled site and never from a string
  literal. The taint type wants this exact shape.

## What round-28 SPECIFIED but did NOT build (the killed phase-6 = de-passthrough)

The `282` §8 "passthrough-type-gated" work, deliberately NOT executed this round (human
routed the related work to the opaque sibling). Its mechanical shape, for reuse:

- Mint a **user-sourced-text type** under the sealed-room pattern — constructible ONLY at
  I/O edges (parser-input relays, tool stderr, host-captured bytes), never from an
  in-repo string literal. Passthrough catalog holes type to it; our own sentences
  physically cannot ride a passthrough hole (the property is enforced at the TYPE level,
  not by lint — `282:rul-passthrough-type-gated`).
- **Audit the 18 `detail`-heuristic ForeignText codes** (the tagged-render dispatch's
  list): `SiteUnresolvable, SyntaxUnsupported, SyntaxMalformed, CfgTopNode,
  CfgErexitUnknown, CfgInlineRefused, CfgBuiltinShadowed, EffectKindDisagreement,
  PredictOutOfDialect, PredictUnterminated, FootprintIncoherent, EscalationPolicy,
  CarriedAcrossSubstrateAxis, WrappedSiteAdoptionHint, WrapperEntryIncoherent,
  WrapperPeelIncoherent, WhylogCorrupt, AidUnloadedSiblingOracle`. Split them: the
  `syntax-*`/`predict-*` codes GENUINELY relay foreign bytes (wrap at their edge); codes
  like `escalation-policy`/`aid-unloaded-sibling-oracle` compose OUR words at emit sites
  and should de-passthrough into real templates (world-variant siblings where needed).
- **Convergence note (the double-duty)**: this same taint type is what
  `an-output-sanitization` will key on later — the tag serves both "don't let our words
  masquerade as foreign" (de-passthrough) and "sanitize foreign bytes before display"
  (the security lane). One type, two consumers.

## Adjacent, already-known sensitivities (pointers, not new work)

- **whylog is host-metadata-sensitive** even when secret-free (`AID-NEEDS:law-whylog-is-
  sensitive`; the durable carries invocation record + records stream + apply report).
  `an-output-sanitization` + the whylog contents are the security round's, not built.
- **captured host bytes re-entering later probe artifacts' argv** is a distinct injection
  surface no standing law covers (`26C:need-captured-bytes-ship-as-data`); r26-deferred,
  adjacent to any taint model.

## Not owed here

The errorloom crate's own foreign-text handling is a GENERIC transport concern (it
refuses to prose-bless a ParamValue/ForeignText region); it needs no Dorc taint type.
The taint work is entirely Dorc-side.
