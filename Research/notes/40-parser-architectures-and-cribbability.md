# Parser architectures compared + what's cribbable (license-aware)

Three real, referenceable shell parsers span the design space. All three independently conclude shell needs context/mode-sensitive lexing, a lossless/concrete tree, and a small *enumerable* dynamic-construct boundary to exclude.

| | Morbig | Oils / OSH | mvdan/sh |
|---|---|---|---|
| Lang | OCaml | Python → C++ (mycpp) | Go |
| License | **GPL-3** ⚠ | Apache-2.0 | **BSD-3** ✅ |
| Size | ~5k LoC (.ml+.mli) | ~800k .py (incl. generated/vendored) | ~36k Go (parser+printer+interp) |
| Strategy | **Menhir incremental functional LR(1)** + OCamllex prelexer + **speculative parsing** + parser-state introspection; embeds POSIX BNF verbatim | **hand-written recursive descent + re2c regex lexer-modes**, single-pass up-front; **ASDL**-defined AST; "lossless invariant" | hand-written lexer+parser, production-grade, round-trips (print) |
| Output | concrete syntax tree (108 cases), auto-gen visitors (`visitors` ppx) | ASDL syntax tree | AST with positions |
| Trust basis | embed-spec + differential test vs dash/bash | huge spec-test suite + lossless round-trip | huge fuzz/round-trip test suite |

## The dynamic-construct boundary (Dorc's `unsafe`/⊤ set) — enumerated by Oils + Morbig
Both converge on the same exclusion list, which Dorc can adopt nearly verbatim:
- `eval`, `alias` (lexical macros), `trap`, `source`/`.` with dynamic filename — runtime code construction.
- `$PS1`-style prompt re-parsing.
- **Recursive arithmetic eval** (`a='1+2'; b='a+3'; echo $((b))` → 6) and `[[ x -eq x ]]` operands.
- LValue-taking builtins: `unset "$expr"`, `printf -v`, `${!ref}`, `test -v`.
- Morbig adds: aliases only resolvable at top-level (else lexing undecidable); subshell delimiting needs reentrant sub-parsing; here-doc non-locality.
GUESS: in a *strict superset we design*, most of these are simply not in the language (or are `unsafe` blocks); we never inherit POSIX's undecidable lexing because we choose the core.

## Cribbability verdict (answers "how much can we crib from Rust/OCaml projects")
- **Techniques are free; code reuse is license-gated.** Parsing *techniques* (incremental-LR + speculative parsing; lexer-modes; lossless invariant; ASDL-style schema→types+visitors) are described in papers/blogs and are not copyrightable — crib freely regardless of language.
- **GPL-3 cluster** (reuse forces Dorc → GPL-3): Morbig, morsmall, colis-language/constraints, shstats, **ShellCheck**. So the best-fit OCaml parser (Morbig) cannot be *linked* without GPL-3-ing Dorc. A clean-room OCaml reimplementation of Morbig's recipe is ~2–5k LoC (the paper is a near-complete spec).
- **Permissive, liftable-in-principle:** Goblint (MIT, OCaml AI framework — but C-specific; crib *structure* not code), Smoosh (MIT, OCaml semantics — reference), tree-sitter-bash (MIT, C GLR grammar — usable via bindings as a *parser* if we don't need Menhir-style introspection), mvdan/sh (BSD, Go — only liftable if Dorc is Go), Oils (Apache-2.0 — too entangled to lift; reference only).
- **Net:** there is no permissive, OCaml, incremental-LR shell parser to drop in. The realistic options are (a) clean-room reimplement Morbig's recipe in our impl language, (b) bind tree-sitter-bash (MIT, C) for a fast GLR parse if its tree suffices, or (c) if Dorc is Go, fork mvdan/sh (BSD). This is a **real input to the Phase-2 language decision**, not a footnote.

## Oils-usefulness verdict (the user asked explicitly)
SUSPECT: **reference, not dependency.** Oils is Python→C++ via a custom toolchain, ~MLoC, Apache-2.0 (legally reusable, practically not liftable into OCaml/Rust). Its value to Dorc: (1) the **best practitioner-grade education** on statically parsing shell (blog series + in-repo `doc/parser-architecture.md`); (2) the **lossless invariant** principle (tokens reconstruct the source — needed for our re-emit/annotate tooling) and the **ASDL** schema-driven IR idea (define the tree in a small schema, generate types+visitors — Morbig reaches the same place via the `visitors` ppx); (3) a clean **enumeration of the dynamic-parsing boundary** = our `unsafe` set; (4) a second independent existence-proof (alongside Morbig) that shell *can* be statically parsed. → Keep cloned for doc/reference; do not build on.

## CST/IR + visitors — convergent design signal
Both Morbig (`visitors` ppx over CST) and Oils (ASDL → generated node types) generate their tree-walking boilerplate from a schema. SUSPECT Dorc should do likewise: define the IR/CST in one schema, generate types + iter/map/reduce visitors, and write each analysis as a visitor/transfer-function over the stable IR (the modularity ShellCheck *lacks* — it analyzes on-the-fly during parse, which is why adding an analysis means editing its parser).
