# Learning path: building a static analyzer / CFG engine (for the human)

Curated, **graded**, human-authored only (AI-generated pages like Grokipedia were found and rejected). Goal: get from "experienced compiler/PLT dev" to "can design a sound-enough may-mutate dataflow analyzer over a CFG." You (ELLIOTTCABLE) are over-qualified on language/parsing and under-exposed on *abstract interpretation / dataflow frameworks* specifically — so the path front-loads that.

Recency is irrelevant here: this theory is 30–50 years stable. Prioritized by signal/effort.

## Tier 0 — the one anchor (read this, mostly skip the rest)
- **Møller & Schwartzbach, _Static Program Analysis_** — `Research/papers/moller-schwartzbach-…pdf` (saved). Free, ~200pp, continuously updated (2025), CC-licensed, used at Aarhus. It is *the* modern free text and covers the entire Dorc engine in order. Chapter → Dorc-need map:
  - **§2.5 Control Flow Graphs** — building the CFG (our IR backbone). §2.1-2.4 = a toy language to anchor on.
  - **§4 Lattice Theory** (lattices, construction, monotonicity, fixed points) — the math under ⊤/⊥ and "skip iff ⊥". *Read closely.*
  - **§5 Dataflow Analysis with Monotone Frameworks** — sign/constant/live-vars/reaching-defs, **fixed-point/worklist algorithms**, **forward/backward/may/must** (§5.8), **transfer functions** (§5.10). Our "may-mutate" analysis is a textbook *forward, may* analysis; each command's effect is a transfer function. *This is the core chapter for Dorc.*
  - **§6 Widening/Narrowing** — only if we ever do interval-style value analysis (probably not v1).
  - **§7 Path Sensitivity & Relational** — relevant to guard-position reasoning (`if cmd; then …`).
  - **§8 Interprocedural Analysis** (interprocedural CFG, context sensitivity, call-strings vs functional) — needed for functions / `source` / whole-program CFG.
  - **§9 Distributive Frameworks: IFDS / IDE** — *directly* the machinery for precise interprocedural facts and **program slicing** (the planning-log's "backward slice from the dirty set" for sub-host minimization). Read when you reach Tier-B.
  - **§10+ Control-Flow Analysis, Pointer Analysis, Abstract Interpretation, SMT** — reference as needed; CFA (§10) matters for dynamic dispatch / first-class-function-ish constructs (our dynamic command names live here conceptually).
  - Suggested order: §1 → §2.5 → §4 → §5 (whole) → §8 → §9. That's ~the whole job.

## Tier 1 — shell-specific parsing pedagogy (human-authored, practitioner-grade)
Andy Chu's Oils blog — the best writing anywhere on *statically* parsing shell (complements the academic Morbig paper). Read in this order:
- **"Parsing Bash is Undecidable"** — https://www.oilshell.org/blog/2016/10/20.html (parse-up-front vs parse-during-exec; why dynamic state breaks it).
- **"How to Parse Shell Like a Programming Language"** (2019) — https://www.oilshell.org/blog/2019/02/07.html (the architecture overview).
- **"How OSH Uses Lexer Modes"** + **"When Are Lexer Modes Useful?"** — https://www.oilshell.org/blog/2016/10/19.html , …/2017/12/17.html (the lexer-modes technique — the recursive-descent alternative to Morbig's LR approach).
- **"Why Lexing and Parsing Should Be Separate"** — oils wiki.
- In-repo (saved under `Vendor/oils/doc/`): `parser-architecture.md` (the "lossless invariant", the 4 re-parsing sites, the *enumerated* runtime-parsing/`unsafe` boundary), `architecture-notes.md`.
- The academic counterpart (saved): **Morbig paper** (`Research/papers/morbig-sle2018…`) — §2 "perils of POSIX shell" is the canonical difficulty catalogue; §3 the incremental-LR + speculative-parsing recipe.

## Tier 2 — dataflow framework engineering (saved + linked)
- `Research/learning-path/cmu-17355-dataflow-frameworks.pdf` (Aldrich, CMU 17-355) — dataflow over a 3-address IR; clean, implementable. (saved)
- `Research/learning-path/harvard-cs153-cfg-dataflow.pdf` (Harvard CS153 Lec17) — CFG + dataflow lecture. (saved)
- **Ed Yang, "Hoopl: Dataflow lattices"** blog series — https://blog.ezyang.com/2011/04/hoopl-dataflow-lattices/ — how GHC's *reusable* dataflow library (Hoopl) models lattices/transfer/rewrite in a functional language. Most relevant existing design if Dorc is OCaml/functional. (linked; A-grade)
- **Clang, "Data flow analysis: an informal introduction"** — https://clang.llvm.org/docs/DataFlowAnalysisIntro.html — practitioner framing of CFG-fact-propagation-to-fixpoint. (linked; B+/practitioner)
- **Wisconsin CS704 (Horwitz) dataflow notes** — https://pages.cs.wisc.edu/~horwitz/CS704-NOTES/2.DATAFLOW.html (Kildall framework). (linked; A)

## Tier 3 — recorded talks / video (nice-to-have; UNVERIFIED links — fetch on request)
GUESS (not yet verified — say the word and I'll confirm URLs and download slides):
- Patrick & Radhia Cousot's **Abstract Interpretation** lecture materials (MIT 6.S083 / 16.399) — the source of the whole field.
- Andy Chu has given Oils talks (recorded) — pairs with the blog series above.
- The Aarhus _Static Program Analysis_ course (Møller) has accompanying lecture videos in some years.
I deliberately did **not** auto-fetch video transcripts (low signal/token ratio, and link-rot risk); flag if you want them chased down and saved.

## What you can safely *skip* for Dorc
- Heavy abstract-interpretation Galois-connection formalism (Cousot-grade) — you rejected soundness-as-goal, so the lattice intuition from SPA §4 suffices; you don't need the categorical apparatus.
- SSA / optimizing-compiler dataflow (dominators, GVN) — Dorc analyzes for *effect/skip*, not optimization.
- Separation logic / shape analysis depth — relevant only as the *concept* behind compositional summaries (see `notes/40-…` Infer/bi-abduction), not as machinery to learn.
