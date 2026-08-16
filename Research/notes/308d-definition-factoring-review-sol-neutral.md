## Findings

### 1. The indices are source-keyed, not definition-keyed

Severity: medium  
Location: `spike/crates/oracle/src/lib.rs:212`, `spike/crates/oracle/src/verdict.rs:90`, `spike/crates/analysis/src/funcenv.rs:251`

Invariant/plan text:

> “Every derived row — a check, a cell declaration, an argparse arm-model, an enrolled dialect token, a footprint claim — is keyed by the DefinitionId that produced it: (SourceFileId, span, custody).”  
> — `Research/plans/28Q-context-kernel-unification.md:87`

> “A query at site S = live_definition(frame(S), name) → read THAT definition’s rows.”  
> — `Research/plans/28Q-context-kernel-unification.md:100`

The implementation instead keys `KindIndex` cells by `(usize, ProviderId, Symbol)` and verdicts by `(usize, ProviderId)`:

```rust
effects: BTreeMap<(usize, ProviderId, Symbol), Vec<EffectCell>>
by_provider: BTreeMap<(usize, ProviderId), Predict>
```

Here `usize` is a source index, not a `DefinitionId`. `DefinitionTable::provenance_of` subsequently tries to reconstruct a definition identity from `(file, role-name)`. If the file contains more than one definition of that role, it returns `Ambiguous` because the lift has already collapsed the definitions into one source-level row.

+SURE: this does not implement the ratified row-keying mechanic. It implements source-factored indices plus a late provenance join.

A concrete world is a single source that defines, removes, and redefines a role:

```sh
widget__is_converged() {
   return 2
}
widget sync alpha

unset -f widget__is_converged
widget__is_converged() {
   widget status "$2"
}
widget sync beta
```

The function environment can distinguish both definitions by span and can name the positionally live one at each call. The dialect-derived indices cannot: both bodies occupy the same `(file, widget)` address, so `provenance_of` returns `Ambiguous` and `answering_file` selects neither.

+SURE: the current outcome is conservative—both commands run—so this is not a wrongly minted license. The repository also has a pre-existing same-file-redefinition refusal, reducing current user impact. It nevertheless means stage-i has not delivered the plan’s promised definition-level representation, and downstream work cannot rely on “chimera unrepresentable” as a type/storage property.

The same representational mismatch affects the plan’s custody commitment. `DefinitionId` stores only `file` and `span`; `custody()` is derived from the file rather than participating in identity. That happens to be equivalent under today’s custody rule, but it is weaker than the ratified `(SourceFileId, span, custody)` key and would silently cease to be equivalent if custody is re-keyed as anticipated in the type’s own documentation.

Confidence: +SURE on the mechanical discrepancy; ~SUSPECT that accepting the existing same-file refusal as an intentional scope boundary would require an explicit amendment to §1 rather than treating this implementation as complete.

---

### 2. The new differential battery validates frame identity, but not the license-bearing row selected through it

Severity: medium  
Location: `spike/crates/cli/tests/definition_frames.rs:662`, `spike/crates/cli/tests/definition_frames.rs:724`, `spike/crates/core/src/definition.rs:203`

Invariant/plan text:

> “true positional resolution over merged indices risks the chimera (identity through one author’s argparse, cells from another’s; pope-sin, invisible to goldens).”  
> — `Research/plans/28Q-context-kernel-unification.md:81`

> “Gate: syn-single-frame-byte-identical (full corpus, both legs) AND the differential cells agreeing with the frame answer.”  
> — `Research/plans/28Q-context-kernel-unification.md:521`

The principal new differential test calls:

```rust
let named = live.source_before(*site, FLOOR_ROLE);
```

and compares that source with the body observed from the shells. This verifies the function-environment answer, but it does not pass that answer through `command_effect`, `KindIndex::effect_of`, coordinate resolution, or license construction.

The four new `frame30-*` product fixtures exercise `__is_converged` bodies and auto-cell/verdict behavior. None gives two positionally live `__predict` definitions different argparse arms or different typed cell declarations. The small `answering_file` unit test likewise uses only abstract provenance rows.

~SUSPECT: a regression such as these would pass the distinctive new battery:

- Resolve argparse from the correct positional definition but read cells from another file.
- Read both from the same wrong source while `LiveDefinitions` itself remains correct.
- Select the correct verdict body but the wrong widening/effect row.
- Forget positional selection at one license-bearing consumer while keeping probe-body shipping correct.

A direct safety-oriented test should use two source definitions such as:

```sh
# a.oracle.sh
widget__predict() {
   item : sm.dorc.Package = "$2"
   widget query "$item" : sm.dorc.Package:"$item"@installed
}

# b.oracle.sh
widget__predict() {
   item : sm.dorc.Service = "$2"
   widget query "$item" : sm.dorc.Service:"$item"@active
}
```

with a book that makes each definition live in a different region and issues the same `widget sync x` spelling in both. The assertions should cover the selected `FactKey`, shipped body, and final disposition under deliberately conflicting probe facts. That would make an identity/cell chimera observable as either a wrong coordinate or, in the worst case, an incorrect elision.

+SURE: the landed tests provide substantial coverage of frame solving, source/body selection, helper-conflict withholding, undefined regions, nested subshells, and the book-definition wall. They do not directly test the exact license-plane failure that motivates §1.

Confidence: +SURE.

## Overall assessment

I found no demonstrated wrongly minted license in the reviewed implementation. The consumer wiring consistently resolves a positional source and reads argparse, effect cells, verdict bodies, widenings, and footprint bodies from that same source; unknown, ambiguous, and missing cases generally degrade toward execution.

The main shortfall is structural fidelity: this is source-factoring with reconstructed provenance, not the ratified definition-factored representation. The current discrepancy fails conservatively where it matters today, but the test suite does not directly prove the central identity-to-cell-to-license chain. I would therefore assess the landed stage as plausibly safe in its covered worlds, but incomplete against §1 and insufficiently pinned against its worst stated regression class.

Per the review constraint, I did not execute any gates or tests. Their runtime success and cross-shell behavior therefore remain unverified by this review.