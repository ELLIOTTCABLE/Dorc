# 24T — Payload decomposition: carriers, the quote-stage map, and the render ladder (PROPOSAL)

AI-authored (Fable, research/planning session), 2026-07-10. Proposal-tier. The PAYLOAD
half of the wrapper/bounce round — the complement to `plans/24S` (the context/transform
half); the two documents share one evidence base and compose by design (a real site is a
point in their product: `sudo sh -c "$cmd"` = 24S's context machinery wrapped around this
document's payload machinery). Research-phase CLOSED by the human 2026-07-10 ("durable
all of this; call the payload-round done for the research-phase"); firming into an
implementation plan is DEFERRED — the pre-implementation conductor guides firming-up;
overall sequencing deliberately unspecified. Prime target for adversarial analysis.

Evidence base: `.claude/research/opaque-string-analysis-ceiling/` — turn notes
`turn01`–`turn08` (payload spine: turn03 Table 1 as recalibrated by
ruling-payloads-realistic; turn08 whole); 21 graded sources in its `sources.json`
(bracketed `[X-slug-year]` citations resolve there). Finding-slugs (`opaquesN-…`,
`t2-fdN`, `payN-…`) resolve to the turn notes; cited for traceability, not required
reading. Governing scope ruling (human, turn 4): "we're not trying to break any new
analyzer ground here, just trod old ground in a novel domain."

Reading rules: root docs and human rulings outrank this. Every sh spelling is STRAWMAN
(shapes proposed, no syllable). Certainty markers throughout.

---

## §0 — Problem, scope, and the impossibility ledger (read first)

**The problem.** A code-carrying head hands a STRING (or a stream) to a child evaluator:
`sh -c 'STR'`, `bash -c 'STR'`, `su -c 'STR'`'s operand, `xargs sh -c`,
`find -exec sh -c`, `sh <<'EOF'`, `sh file`. At HEAD the payload is one opaque word:
the site is Opaque, runs every apply, and casts a total poison-wall. The idiom cannot be
ruled away: the lint ecosystem itself MANUFACTURES it — ShellCheck's own blessed fixes
are `sudo sh -c '…'` and `sh -c` via `find -exec` [B-shellcheck-sc2232-wiki-2026]
[A-shellcheck-sc2032-2033-wrappers-2021] (t2-fd8) — and the r25 field trial's permanent
wall (`su - postgres -c '…'`) is the standard Debian idiom. Practice never parses these:
the best shipping tool opens `-c` payloads with a FLAT tokenizer (shlex.split,
first-word-only) [A-resholve-source-invocations-2025] (t2-fd27); the literature has the
full mechanism but only for JS eval [A-unevalizer-2012]. Composing them for sh is the
unoccupied territory this design claims — deliberately stopping at the
literature-proven rungs.

**Impossibilities and hard fences, frontloaded** (the ledger wins over any later
section):

- imp-P1 **General syntax-position holes stay walls.** A ⊤-valued splice whose bytes
  reach the child's parse can rewrite the grammar around itself; hole-parsing is
  UNBUILT in all literature for any language — Arceri's group reports 47% of their
  eval-sites opaqued by unknown inputs and sketches the fix as future work
  [A-arceri-dynamic-code-2021] (opaques3-finding12). Only the two ruled basic forms
  (§5b) are explorable; the general case is fenced.
- imp-P2 **Loop-assembled payloads are ⊤ forever.** The cross-literature identical
  cliff ([A-unevalizer-2012] 6/11 failures; [A-tarsis-string-automata-2024] cycles ⇒
  ⊤; opaques3-finding12's Σ*-loop) — and per the standing atomic-command axiom,
  per-iteration verdicts are out anyway.
- imp-P3 **Runtime-data argv caps at bounded poison.** `xargs`/`find -exec` feed argv
  from live stdin/fs: head-known/operands-⊤, kind-level bounds, never entity facts
  (turn03 pay-runtimedata).
- imp-P4 **Network/remote-fed code is ⊤ forever.** `curl | sh` cannot be decomposed;
  it arrives admin-guarded in practice (USER_STORY's own machine.sh) and the guard
  lane already carries it. The LOCAL stream cell (heredoc; constant-path `sh file`,
  freshness-caveated) is in-scope.
- imp-P5 **R2 rendering is refused** (§4c): the engine never re-serializes
  RECONSTRUCTED payload text back into an apply artifact — engine-authored sh,
  quoting re-embedding, razor-fail attribution. v1-refused; permanent-weld hovering
  (human, turn 8). R0/R1 below are the licensed rungs.
- imp-P6 **Payloads are plain sh, permanently** (HARD-ACKED, opaques8-ack1): no
  dorcisms recognized inside payload text, no dialect-marker inheritance. Forced by
  the off-ramp weld: `dorc strip` erases dorcisms from syntax and never rewrites
  string interiors, so a payload dorcism would survive strip and break under real sh.
  Rider lint: dorcism-shaped text in a declared payload ⇒ warn. (Human rider: plain,
  flat, simple sh is an oracle-authoring virtue on multiple axes; watch-item = whether
  forbidding code-piping spellings proves painful in practice.)
- imp-P7 **HEAD is the floor; decomposition is licensed, never inferred.** No carrier
  declaration ⇒ the payload stays one opaque word, byte-identical to HEAD.

---

## §1 — The feature at a glance

One sentence: **payload sites become decomposable — the value plane resolves the
payload string, the engine re-parses it as plain sh under the site's evaluation
environment (ρ, from 24S), the inner commands join the same analysis as
verdict-participating nodes, and the LINE keeps a single disposition.**

The proposed pins (turn08; ratification status in §7):

- **pin1 licensed-code-carriers**: the head's oracle declares which operand is code
  and how the rest bind (`sh -c 'STR' name args` ⇒ payload=STR, `$0`=name, `$@`=args;
  stdin shapes: `sh <<EOF`, `sh -s <<EOF` + argvbind, `sh file`). Which-arg-is-code is
  argparse knowledge the head's author owns (practice precedent: resholve's
  INVOKABLE_SHELL + per-command rules [A-resholve-manpage-2023]). The stdlib ships
  sh/bash/dash carriers.
- **pin2 plain-sh-payloads**: imp-P6. RATIFIED.
- **pin3 parse-failure-degrades**: an unparseable payload (bashisms under `bash -c`,
  exotica) ⇒ payload-⊤ ⇒ SITE-LOCAL wall + dq-payload-unparseable naming the first
  offending token — never a book-level error (the per-site-refuse posture, third-
  confirmed across the literature: opaques3-finding11).
- **pin4 whole-line-unit** (§4): fine-grained analysis and probing, coarse-grained
  disposition and render. Discussed at length, NOT yet typed-acked (§7 P-A1).
- **pin5 single-mechanism-analysis-time**: ONE decomposition path, at analysis time,
  consuming RESOLVED values from the value plane — a syntactic literal is the
  trivially-resolved case; a const-prop-resolved template (`CMD="…"; sh -c "$CMD"`)
  rides the same lane. In-fixpoint splice, nesting-bounded (the Unevalizer's
  circularity answer [A-unevalizer-2012]; smoosh's read/eval-loop grounding: re-parse
  IS reentry parameterized by {parse-context, environment} [A-greenberg-smoosh-2020],
  opaques3-finding14 — a special string-level trick would be a category error).
- **pin6 ladder-and-cliffs** (§5): syntactic-literal ⊑ resolved-literal ⊑
  concat-of-constants, then walls — as amended by the basic-forms ruling (§5b).

Composition with 24S: context peel and payload decomposition interleave by argv
structure under ONE shared recursion bound; ρ flows outer → wrapper-transform →
carrier-reset → inner nodes (opaques8-finding7). Payload decomposition REQUIRES ρ:
even a fully-literal payload is unsound to analyze without evaluating it under the
installed environment (opaques7-finding11's `~`-expansion pair). The composed case
(`echo data | sudo sh -c 'cat >> /etc/f'` = pipeline ∘ context ∘ payload) is v1's
acceptance shape, not a stretch goal.

---

## §2 — The quote-stage map (the model that makes sh tractable here)

**The stage-rule** (opaques8-finding10, the round's cleanest formulation): *a splice is
a hole iff its ⊤-valued bytes reach an evaluation stage that treats them as syntax —
and quoting only ever protects the stage it belongs to.* Payload analysis is therefore
a bookkeeping problem about WHICH evaluator last touches which bytes — the same
honesty-about-environments discipline that made ρ well-trodden ground, applied one
stage down. Two independent killers decide every splice: is its VALUE ⊤ (else the
resolved-literal rung absorbs it), and do its bytes land in SYNTAX position at any
later stage (else it is an ordinary operand).

The canonical hierarchy (one dynamic `$SVC`, three spellings):

```sh
sudo sh -c "systemctl restart $SVC"            # (a) HOLE. Outer double-quotes protect
                                               #     only the outer parse; the child
                                               #     re-lexes the VALUE as code.
sudo sh -c "systemctl restart '$SVC'"          # (b) bounded hole: child-stage quoting
                                               #     contains it UNLESS the value can
                                               #     break out (a ' in the value).
sudo sh -c 'systemctl restart "$1"' _ "$SVC"   # (c) NO hole: literal payload; the
                                               #     value travels out-of-band and
                                               #     expands at the child stage, in
                                               #     word position — an ordinary ⊤
                                               #     operand, bounded by the verb's
                                               #     oracle.
```

(a) is the imp-P1 cell regardless of its tidy outer quoting. (c) is the lint ecosystem's
own taught fix shape [A-shellcheck-sc2032-2033-wrappers-2021], fully analyzable at v1:
the structure parses perfectly and the ⊤ lives where ordinary Opaque-operand handling
already works. **The hint-nudge corollary**: the repair for a hole-walled (a)-site is
the one-line rewrite to (c) — the same advice the user's linter already gives
([A-shellcheck-sc2029-ssh-2026]'s escaping-layer warning is the ecosystem admitting
(b) is fragile). Dorc's walls and the ecosystem's style pressure point at the identical
transformation; the diagnostic should say so.

Fragment mechanics: a payload word's fragments partition by expansion stage —
double-quoted/outer fragments are resolved by the OUTER value plane before the payload
exists (constants after const-prop, or ⊤ ⇒ cliff); single-quoted fragments are TEXT
whose `$`-references re-expand inside the child under ρ (opaques8-finding1). Heredocs
are the tame half of the map: a quoted heredoc (`<<'EOF'`) is pure inner text; an
unquoted one is an outer-expanded template that never field-splits.

**The focus specimen** (human nit, adopted): heredoc-script-piped-to-sh
(`sudo sh <<'EOF' … EOF`) — an embedded BOOK, not a one-liner. It exercises the stdin
carrier shapes, the quote-stage map, and — the property that matters — VERBATIM body
bytes with real source spans (no quoting transform; provenance nearly free), while
making pin4's value-cap visible (§4).

**Engine dependency, named**: the fragment-preserving / cause-tagged ValueOf reshape
that `notes/219` (tc-fork ii) flagged and deferred — the current `Recipe::Top`
collapse is cause-erased and fragment-destroying. Payload work is the reshape's first
REQUIRED consumer (§7 P-A5 relay).

---

## §3 — Value anatomy, and each chair

Where decomposition actually pays, in order of realism (opaques8-finding3):

1. **Bounded walls** (the common win): an inner command with a `touches()` bounds the
   line's poison kind-granularly — downstream elisions survive a running payload line.
   An un-oracled inner command still total-walls from its position: decomposition
   without inner-tool coverage buys hints, not value.
2. **Whole-line elision**: realistic exactly for the 1–2-command lint-taught idioms
   (`sudo sh -c 'cat >> file'`, `sudo sh -c 'cd /root && pwd'`) when every inner node
   vouches.
3. **Surgical hints**: "line N blocked only by: inner `foo` unmodeled" — the
   first-wall-hint pattern applied inside the line.

Honesty row: the r25 `su - postgres -c 'psql …'` line decomposes, but its inner psql
is credential-gated (24S's class-3) — its cap is run-with-guard FOREVER; what payloads
buy there is the honest reason string and the su/ρ modeling, not elision.

By chair: the **admin** never learns the feature exists (books analyzed as-written;
opaque payloads lose value with named hints, never execution fidelity). The **carrier
author** is a tiny pay-once class (sh/bash/dash + su's `-c`; the stdlib ships them):
which-arg-is-code arity rules, stdin shapes, argv-binding, plus the carrier's own
context record (§6 ledger: fresh positionals, fresh shell-options, scope-isolation —
the existing subshell/scope-clobber semantics are exactly right). The **everyday tool
author** owes NOTHING new — the 24S no-wrapper-awareness referendum extends: no
payload-awareness in tool oracles, ever; if build contact forces one, the kBURDEN
story re-audits.

---

## §4 — The whole-line unit and the render ladder

### §4a — Fine-grained analysis, coarse-grained disposition (pin4)

Inner payload commands become verdict-PARTICIPATING nodes — oracle dispatch, effect
classes, `touches()`, vouches, effects flowing through the ordinary dataflow (what
un-walls downstream lines and bounds the poison). This is a strict upgrade from the
`$()` treatment (`notes/219`: effect-bearing NON-leaves, invisible to disposition).
But elide/guard/run is decided ONCE, for the outer leaf, by folding:

- **Elide(line)** ⟺ every effect-bearing inner node has a reached converged-vouch,
  AND the carrier + any wrapper vouch their own self-observables (24S's
  self-effects rule), AND the line-level consumption gate passes.
- **Guard(line)** ⟺ not elidable, but every diverged-relevant inner fact has a
  licensed check: the guard is a CONJUNCTION of inner checks placed OUTSIDE the line
  (`( chk1 && chk2 ) || sudo sh -c '…'`). Licensing composes from 24S unchanged (a
  bare carrier moves no coordinate axes; under sudo, invariant-kind checks ride
  probe-outside; checks close over bound positionals and §6b ρ-replication). Failure
  semantics: any check fails ⇒ the WHOLE line runs, re-running individually-converged
  inners — the lowest-tier sin (unnecessary-execution), identical to the bare script's
  every-day behavior.
- **Run(line)** otherwise — still with bounded poison and hints.

Probing is per-inner (checks are Dorc-constructed probe-lane invocations, shipped
individually, closed under ρ); only the USER's bytes are never subdivided.

### §4b — Why coarse (the three welds) and what it costs

Per-inner disposition requires rewriting payload strings. Three welds strain: the
authorship/attribution weld (a rewritten payload is engine-authored sh — when it
misbehaves, no human line said the false thing; the razor); quoting fidelity
(re-serialization through the outer quoting context is strip-grade correctness-critical
machinery where a bug executes bytes nobody wrote); and the leaf-seam (inner commands
execute inside the child's shell — un-wrappable individually without that same
rewrite). Precedent: pipelines already parse as one span-covering unit with
per-segment status parked (24E §14 / flag-pipe-status-unit) — and payload-per-inner is
STRICTLY harder than pipeline-per-segment (string interiors vs real spans).

Costs, honestly: one unmodeled inner command holds an N-line payload hostage forever
(all-or-nothing); guard conjunctions get long; the check-tax multiplies. The
mitigation doubles as style pressure the human independently endorses: the hint for a
value-capped payload line is "split it into separate lines" — off-ramp-friendly, and
it unlocks per-line disposition by construction. The empirical bet: lint-taught
payloads are 1–2 commands, where whole-line ≡ per-inner.

### §4c — The render ladder R0/R1/R2 (opaques8-finding8; born from the heredoc nit)

"Render-into-payload" is not one cost — it is a ladder keyed on whether the payload
bytes are HUMAN-WRITTEN-AND-VERBATIM in source:

- **R0** (v1 = pin4): no payload editing, ever.
- **R1** (the principled follow-on): span-edits WITHIN verbatim payloads —
  quoted-heredoc bodies first (bytes verbatim, spans 1:1, line-oriented), plausibly
  single-quoted `-c` literals. The edit vocabulary is the OUTER render's own
  (comment-out; quote-inert standins), so the authorship weld SURVIVES: human-written
  bytes, the same moves the outer render already makes. Hazards bounded: standin
  quote/delimiter inertness; unquoted-heredoc outer-stage expansion runs on COMMENTED
  lines too (harmless for `$var`; cmdsub splices are already pin6 cliffs — the cliff
  protects the render). Guard-INSERTS inside payloads stay OUT of R1 (in-child/
  in-context execution — the 23J-parked cell; guards remain outside).
- **R2** (the weld-shaped cell): re-serialization of RECONSTRUCTED/derived payload
  text. Refused (imp-P5); the weld, if taken, should target R2 specifically —
  welding ALL payload rendering would be relitigated by the heredoc-book cell, where
  per-inner value is real and R1 is cheap (this is the resolution of the human's
  "can't put my finger on why we can't weld against it").

R1 is what redeems the heredoc specimen's value-cap: an embedded twenty-line book
under R0 is wall-adjacent; under R1 its converged lines comment out individually,
inside the body, exactly like book lines.

> *(Annotation 2026-07-10, post-close — R1 scope EXTENDED by human direction,
> conductor-acked with one carve: R1 firms as "the verbatim body is a separate book" —
> per-line disposition INSIDE verbatim payload bodies, INCLUDING guard-insertion at
> UNWRAPPED carriers, since 24S §6d's in-sequence/same-stream argument licenses an
> in-body guard identically to a top-level one, and it dissolves §4b's
> conjunction-ergonomics cost for heredoc books. The carve: in-body guard-insertion
> under an ELEVATED wrapper (`sudo sh <<'EOF'`) is Dorc-authored code in the elevated
> lane = the 23J cell, parked — wrapped bodies get elision-edits only until then. R0
> stays the v1 floor. Record: turn08 opaques8-ack4-r1-reach.)*

---

## §5 — The resolution ladder, the cliffs, and the two ruled-in basic forms

### §5a — The ladder (pin6)

`syntactic-literal ⊑ resolved-literal ⊑ concat-of-constants` — all three
literature-proven and cheap ([A-unevalizer-2012]; [A-eval-men-do-2011]'s base rates:
66–82% of real dynamic code-strings constant/composite, 98.6% of sites monomorphic —
JS-measured, shell-unmeasured; the shell re-measurement is the human's own quarantined
corpus pass). Cliffs (⇒ payload-⊤ + a dq naming the exact blocker): unknown-var
splice, cmdsub splice, loop-assembly, any splice reaching syntax position.

### §5b — The basic-forms ruling (human, 2026-07-10 — opaques8-ruling1)

The MOST-BASIC form of each fenced Hard Thing is IN-BOUNDS for exploration by the
implementation-planning agent, puntable there if it proves research-grade:

- **basic-hole-form** ("single-quoted-inner-value-tracked"; human: "I *suspect* [it]
  is very doable"): holes bounded by CHILD-STAGE quoting — the (b)-cell of the §2
  hierarchy. The residual obligation shrinks from "metachar-free single word" to
  "cannot break the inner quoting" (for a single-quoted embedding: value contains no
  `'`). Word-position only; syntax-position holes stay imp-P1. The sh-restricted
  version is ~SUSPECT more engineering than research (lex known fragments, one opaque
  word-token); the research-grade half is where the value-bound FACT comes from — the
  Dorc-shaped source is read-value oracle claims ("this tool's output is one
  metachar-free word"), which is the 219 capture/Query lane's vocabulary, not string
  cleverness.
- **basic-set-form** (the "degenerate-case, idiot's-attempt automata"): bounded
  literal-SET carriage — joins keep ≤k literal candidates before collapsing to ⊤
  (finite height k, near-flat, kCONTEXT-compatible; opaques8-finding11). Rescues the
  branch-built template (`INSTALL="apt-get install -y"` / `"yum install -y"` then
  `sh -c "$INSTALL curl"`): each candidate rides the existing resolved-literal lane,
  parsed k times, verdict folded across candidates. This is the cheap end of the
  spectrum whose expensive end (full FA carriage, [A-tarsis-string-automata-2024]) 
  stays out: non-flat domain, hand-tuned widening, no scale evidence
  (opaques3-finding13).

What stays fenced even so: general hole-parsing (unbuilt anywhere, for any language),
full automata carriage, and — above ALL variants — loop-assembly and true runtime
data. The Hard Things raise the ceiling; they never remove it.

### §5c — The stopping-point note (gently acked, unsettled)

The old "one and only punt: eval" weld is currently FULLY-UNWELDED BY NECESSITY (this
work is the necessity); a principled announced stopping point is owed at
current-work close. Candidate on record (conductor analysis, human gentle-ack): the
boundary needs no new weld — it is DERIVABLE as the composition of three standing
lines: the const-resolvability cliff (imp-P1/P2) × no-escalation (24S imp-1) ×
no-cross-host (24S imp-5). Privilege enters via measurability, not as its own axis:
`sudo sh -c 'literal'` is fully in-scope while dynamic eval is out EVERYWHERE,
privileged or not, because resolvability kills it first. The human's composite
("round-trip, thru a privileged container on a remote host, dynamic exploratory
evaluation of opaque runtime values, then insert into further static analysis") lands
past all three lines independently. `eval` itself: the transparent-context cousin
(same shell, effects DO escape, no scope-clobber) — its unpark is "spelling"
(human ruling); it inherits whatever the stopping point announces.

---

## §6 — The asserted-semantics ledger (discharge before build-trust)

Semantic facts this design asserts at ~SUSPECT, each with its discharge route — the
house method is differential tests against real `dash` in the spike
(kVERIFY-calibrate), POSIX XCU §2 citations only where the plan wants paper to stand
on:

| # | assertion | route |
|---|---|---|
| L1 | a child `sh -c`/stdin-fed sh does NOT inherit the parent's `set -e` (fresh shell-options; 20V door analysis restarts clean per shell) | differential |
| L2 | child var-inheritance is export-only; IFS in the child = default unless exported | differential |
| L3 | positional binding: `sh -c 'STR' name args` ⇒ `$0`=name, `$@`=args; `sh -s args <<EOF` binds `$@` | differential + POSIX cite |
| L4 | unquoted-heredoc expansion performs parameter/cmdsub expansion but NEVER field-splitting | POSIX cite + differential |
| L5 | in an unquoted heredoc, COMMENTED lines still outer-expand (expansion precedes the child's parse) — harmless for `$var`, already-cliffed for `$()` | differential (R1 gate) |
| L6 | bash exported functions (`BASH_FUNC_*`) can leak function definitions bash-to-bash — treated as ⊤-risk edge, not modeled | note-only; lint someday |
| L7 | quoting-reconstruction fidelity: the engine's resolved payload value ≡ the argv word dash hands the child | the reconstruction differential (parse → run under dash → compare argv), a permanent sweep axis |

### §6b — Failure catalog (v1 modes + plugs, compressed from turn08)

Wrong carrier declaration (operand declared code, is a filename) ⇒ pointable oracle
line; lying-carrier sweep axis; the peel cross-check generalizes (the "code" operand
must PARSE and its head must resolve — disagreement ⇒ demote-to-wall + diagnostic).
Accept-but-misparse dialect divergence ⇒ same class as book-level parser fidelity,
same differential harness. Unquoted-splice word-splitting ⇒ cliff, never parsed
around. Inner-quote breakout ⇒ the basic-hole-form's residual bound and its test
(§5b). Outer-residue on the carrier line (`sh -c 'echo x' > f`) ⇒ ctx-outer machinery
unchanged; whole-line granularity absorbs the per-inner consumption gap 219 q-1.d
found for `$()`.

---

## §7 — Adjudication list (owed the keeper; typed acks wanted)

- **P-A1 — pin4 whole-line-unit**: discussed at length (the explainer + heredoc nit +
  render ladder), NOT yet typed-acked. Ratify R0-as-v1 with R1 as the principled
  follow-on and the R2 decision below. (Conductor rec: ratify.)
  *(Annotated 2026-07-10 post-close: the SHAPE is now settled by the human's typed
  takeaway + conductor ack — R0 as the v1 floor, R1-as-separate-book (incl. in-body
  guards at unwrapped carriers) as the reached-for follow-on, the 23J carve for
  elevated wrappers. Formal pin-ratification remains with the keeper at
  implementation-planning; see the §4c annotation + turn08 opaques8-ack4-r1-reach.)*
- **P-A2 — the R2 weld**: refuse-forever vs refuse-for-v1. The human's
  "permanent-weld hovering" intuition targets R2 specifically once the ladder splits
  the cost (welding all rendering would be relitigated by the heredoc-book cell).
  Decide at implementation-planning.
- **P-A3 — the basic-forms exploration charter** (RULED, recorded here): the
  implementation-planner explores basic-hole-form + basic-set-form and owns the punt
  decision (opaques8-ruling1).
- **P-A4 — the carrier quality bar** (fork6; ~~EXPLANATION STILL OWED to the human~~
  *soft-ACKED 2026-07-10 — turn08 opaques8-ack5-quality-bar, with the human's rider
  correction: in-body guards do NOT obsolete the outside-conjunction in general; it stays
  the mechanism for wrapped bodies (the 23J carve), non-verbatim payloads, and anywhere R1
  edits are unlicensed*): extend 24S §7-A6's wrapper checklist with carrier items —
  which-arg-is-code arity gates; the parse-and-resolve cross-check; the L7 reconstruction
  differential; the dorcism-in-payload lint.
- **P-A5 — the reshape relay** (fork4): the fragment-preserving ValueOf reshape
  (219 tc-fork ii) is now load-bearing (first required consumer = payloads); it
  touches value.rs + every consumer while other work churns the same surfaces.
  Sequencing deliberately unspecified (human); coordinate at close-out alongside
  24S §7-A7's queue-3b seam reservation.
- **P-A6 — the announced stopping point** (§5c): owed at current-work close; the
  union-of-standing-boundaries candidate is on record with a gentle ack only.

## §8 — Broad implementation sketch (NOT a build brief; firming deferred)

Dependency-shaped, composing with 24S §8's staging (payload needs ρ — a payload spike
before the region/ρ machinery exists would rediscover opaques7-finding11 as a bug):

carrier role surface (which-arg-is-code + stdin shapes + argv-binding; lift + lints) →
fragment-preserving ValueOf/Recipe reshape (P-A5) → nested parse at analysis time
(plain-sh mode; site-local failure; nesting bound shared with context peel) →
inner-node classification under ρ (verdict-participating; touches()-union footprints;
carrier context record per §6 L1–L3) → whole-line fold (elide/guard-conjunction/run;
consumption gate) → derived-text locators (per-fragment spans, the 111 DAG shape;
quoted-heredoc bodies get verbatim spans free) → diagnostics
(dq-payload-unparseable, dq-payload-splice-⊤ naming the variable, the
move-splice-to-positional hint of §2) → sweep axes (lying-carrier, reconstruction
fidelity, L1–L5 differentials) → THEN, gated on P-A1/P-A2: R1 span-edits within
verbatim payloads; and the P-A3 exploration cells (basic-set-form first — it is
value-plane-only; basic-hole-form second — it needs the bound-fact vocabulary).
Fixture seed: the heredoc specimen book + the three-spelling hierarchy of §2 + the
lint-taught one-liners. Rung-0 regression: carrier-free corpus goldens byte-stable.

## §9 — Explicitly out of scope (fenced, not forgotten)

General syntax-position holes and full automata carriage (§5b's fences); loop-assembly
and runtime-data precision (imp-P2/P3); remote streams (imp-P4); R2 rendering
(imp-P5); guard-insertion inside payloads/wrappers (23J-parked); the 219 capture lane
(`$(…)` value-carriage — opposite dataflow direction, own design); host (24S imp-5);
privilege semantics and the become/doas ecosystem collation (the round-2 topic, still
HARD-DEFERRED pending typed ack); corpus frequency/provenance measurement
(defer-explore4, human-owned); the deferred-to-human vendor dives (CoLiS, ShellCheck
source interior, mvdan-sh/oils — defer-explore1/2/3, quarantine-gated).

---

*Corrections log (annotate in place per plan-tier convention):*

- 2026-07-10 (r24 close-out; `plans/270`): **P-A5's sequencing is RESOLVED** — the
  value-recipe-reshape (né tc-fork ii) slots into `270:block-rebuild`, in the same
  fact-domain churn window as the entity-algebra rebuild (one merge-pain instead of two);
  **P-A6 is SCHEDULED** as `270:adj-stopping-point` on the block-settle design-pass agenda
  (the union-of-standing-boundaries candidate still awaits a TYPED ratification); **P-A4
  soft-acked** (annotated in place above); P-A1/P-A2 remain with the block-context
  implementation-planner as `270:adj-payload-pins`. The §8 sketch's ρ-dependency is honored
  by the block ordering (payload-v1 follows the wrapper stages).
