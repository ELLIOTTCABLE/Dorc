(REWRITTEN AUTOMATICALLY, UNSTABLE. DO NOT REFERENCE IN OTHER DOCUMENTS.)

Dig through the design-docs in this repo; collate for me a list of 'undone design-work' that was either 1. mentioned by a human in-passing, but doesn't clearly map to any of the design-passes present; or 2. heavily pushed by a `plans/*.md` document, but seems like it may have gotten lost in the weeds. Sort higher: items with high design-consequences ("especially hard to unbake" or "can't be refactored"); and sort lower: items that seem known/deferred (in `TODO.md`, discussed by the human in recent design-passes, or clearly not-upfront-work.)

Update-by-overwriting this section; keep this descriptive header/prompt, just replace items. Keep it short; collapse similar/related items into one entry; this shouldn't grow over ~5 items. It's to catch *major work*, not nits.

## TODO:

* [ ] **MH2 — the version layer.** Named in the founding thesis as one of *two* co-equal must-haves (`archive/ansible-conversation-text.md:958`), then dropped: only an INTEGRATE line (`064:19`) + a day-1-sell aside (`083` Q-PROPERTY). No pass designs deriving the version-correct `foo.check`/effects from `install foo@3.5`, the `.version` verb, or version-bearing-fact normalization / API-node resolution. Distinct from the security round's version-drift/content-hash gate (`102`, *identity-for-soundness*, not *applicability*). High consequence (shapes the oracle-contract + fact-domain → retrofit-hostile) and genuinely lost, not deferred → top.

* [ ] **The Dorc language itself (the strict-superset).** `DESIGN` #1 ranks it "*most* critical when calibrated against lock-in," yet `041` only picks impl-language + parser-strategy; nothing designs what the superset adds/drops vs POSIX-sh, the `unsafe`/⊤ boundary syntax, or the "first-class language constructs" (`064` names `retries/until/delay` with nothing behind it). Deferred by `083 §8` (pulls it down) — but syntax is the canonical can't-unbake item once users write scripts, and you're at implementation.

* [ ] **Probe→apply gap + cross-host shared-state soundness.** `090` raised the OCC-style probe/apply *validation phase* (P3/F4) as a genuine architectural choice; `099` parked it at a pessimistic default, never sized. Cross-host hazards (write-skew, memoization-key soundness, unreachable≠converged) are scattered + unowned (`076 §3`/`§4b`, `090` D5); `kSTATE` is flagged "genuinely unsettled." The verdict-shape `(verdict, content-key, freshness)` is retrofit-hostile → decide the *shape* now even if built later.

* [ ] **Oracle-author DX (linter / LSP / authoring harness).** `DESIGN` #5 = "where the bulk of the work lies"; `088 §8` = "the lever on the #1 existential risk (`A-ORACLE`)… a first-class workstream, not a long-tail afterthought." Only the eBPF/trace slice (`077`/`078`) and the correctness-harness sketch (`021 §5`) exist — nothing on the author-facing experience. Lower lock-in (buildable later) + already-argued → sorts down, but highest-leverage omission.

* [ ] **Error-reporting + provenance pass.** Already `TODO.md` item 1 verbatim (→ known; sorts lowest). Provenance exists only as scattered engine *capability* (`055` why-trees, `077` leaf-id, `064` plan what+why); no pass on language/orchestrator error-message quality or the reporting-accuracy UX the human calls this tool "very sensitive" to.
