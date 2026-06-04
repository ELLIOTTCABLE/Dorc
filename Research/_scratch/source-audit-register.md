# Source-claim adversarial audit — register

**Snapshot:** `4c1d789` (branch `ai/snapshot`), pinned 2026-06-03. The corpus is moving
(round-12 DST research is live); re-pin and re-run the cheap passes if drift is suspected.
**Status:** `seq-resolve` ✓ complete · `seq-triage` ✓ complete · `seq-predetermine` ✓ first pass (tier-A headline atoms) · `seq-adjudicate` ⏳ batch-1 done (CoLiS/shell-front-end cluster: 4 works; 0 EGREGIOUS) · batch-2 done (055-spine + leaders: might-smaragdakis, burgess, lucassen-gifford, horwitz-reps-binkley, reps-horwitz-sagiv, reps-demand, heintze-tardieu; 0 EGREGIOUS, 0 MILD; reps-cfl OUT-OF-SCOPE) · batch-3 done (traugott · cfengine-security · salcianu-rinard · ramalingam · smoosh · jensen-moller-tajs; 6/6 PASS, 0 EGREGIOUS, 0 MILD; **zero dispatch** — precheck + arbiter-read of crux passages sufficed) · batch-4 done (engler · distefano · dozer · harnad · cousot[GAP] · arzt-bodden; 5/6 PASS + 1 GAP, 0 EGREGIOUS, 0 MILD; **zero dispatch** — both EGREGIOUS-class number-claims (distefano 0→70% fix-rate, arzt-bodden ~80% saved) verbatim-grounded by arbiter-read; **tier-A fully drained**). `grading-staging.jsonl` now holds 15 entries. · batch-5 done (tier-B drain: spall-rattle · scholz-souffle · mokhov · moller-schwartzbach[→B] · tobin-hochstadt · biabduction; 6/6 PASS, 0 EGREGIOUS, 0 MILD; **zero dispatch** — both EGREGIOUS-class candidates (souffle "35 s", rattle "CPU-pipeline RAW/WAR/WAW") verbatim-grounded by arbiter-read; rattle's provisional MILD retracted on the §6.4 full-read; **tier-B now down to `foster` alone**). `grading-staging.jsonl` now holds 20 entries. · batch-6 done (foster — sole tier-B remnant; **PASS**, 0 EGREGIOUS, 0 MILD; **zero dispatch** — all four factual atoms verbatim/near-verbatim by arbiter-read of clean born-digital text; full read **discharges** the entry's "abstract-read only" hedge; no staging line — already a full `sources/` entry). **tier-A + tier-B now BOTH fully drained; substantive audit target complete.** `grading-staging.jsonl` stays at 20 entries.

This is the durable, resumable artifact for the source→claim adversarial audit. The two cheap,
read-only passes are done (below). The expensive pass (2 subagents/work) drains the queue top-down
under the token budget and updates `status:` per work; the long tail rolls to the next budget cycle.

## The two caveats that ride every step (the top-level restates these at pin / arb / escalate; never to subagents)
- **caveat-COST** — expensive *and* invasive. Annotating a committed claim is history-adjacent; not done lightly.
- **caveat-NOTRUTH** — this process cannot find truth. A subagent (or the arbiter) becoming sure "this was badly
  wrong!" is *as* poisonable as the original "this is so right!". Pursue only egregious, human-adjudicable findings;
  log everything else and leave it alone. "I couldn't read it" must NEVER become "it doesn't support the claim."

## seq-resolve — cheap pass 1 (✓ COMPLETE)

**Headline: zero `gap-phantom` (no fabricated citations).** All 54 cited bracket-keys resolve to a graded
`sources.json` entry (no dangling keys); every author-name cue in prose resolves to a real, identifiable work.
*Bound (caveat-NOTRUTH):* this only rules out citing-nothing. Subtle mis-attribution — a real paper that says
something other than the claim — is invisible to the cheap pass and is exactly what `seq-adjudicate` exists to catch.

**`gap-access` — real works, cited in plans, NOT saved/readable locally → fetch-or-flag, do NOT adjudicate as-is:**
- Round-3/4 userbase+perf externals: `Opdebeeck` PhD (4 mentions), `GLITCH` (4), `Rahman` Gang-of-Eight (4),
  `Begoug` (1), `InfraFix` (1), `Mitogen` (2), `pyinfra` (3), `Murder`/lg (1).
- Attributed-from-recall (cited as a specific technical attribution but never saved — verify by fetch IF adjudicated):
  `Ammons` "Mining Specifications" POPL'02 (2), `Callahan` may-use/must-modify (1).
- Known-unreadable: `Heintze–McAllester` cubic-bottleneck LICS'97 (paywalled, every mirror dead per manifest);
  `Cousot'77` (file present but image-only scan — theory covered via the SPA textbook); `RCPSP` (a concept, no single source).
- Action: none is an alarm. Fetch the fetchable (MCP-fetch) only if its dependent claim reaches `seq-adjudicate`.

## seq-triage — cheap pass 2 (✓ COMPLETE)

Danger-rank = `pro·3 + fan + hum·5 + chk·2` (pro = prose-plan files citing it; fan = fan-out across plans+notes;
hum = reaches DESIGN/KNOBS/TODO; chk = claim-lines carrying a number/hard-verb = method has power). Weights are
taste; the *ordering* is what matters. **Generic-token caveat:** tool/project-name sources (`ansible`, `terraform`,
`docker`, `puppet`, `bash`, `chef`, `salt`, …) match the bare English/CLI word everywhere, so their fan/hum are
inflated and unreliable — re-count them by bracket-key or distinctive phrase before adjudicating, not by surname.

### tier-A — adjudicate first (high blast-radius × checkable × load-bearing; reaches-human gets priority)
| status | rank | pro/fan/hum/chk | work | why it load-bears |
|---|---|---|---|---|
| DONE-b1 | 77 | 7/18/2/14 | `A-colis-*` (spec-2019 · platform-sttt-2022 · installation-tacas-2020) | **installation-tacas-2020 = MILD** ("hundreds of commands"); **spec-2019 = PASS (grade→B)**; **platform-sttt-2022 = OUT-OF-SCOPE** (not cited in `plans/`). See batch-1 results. |
| DONE-b1 | 39 | 3/8/0/11 | `A-morbig-sle-2018` | **PASS** (M1/M2/M3 all verbatim-grounded; grade A). See batch-1 results. |
| DONE-b2 | 28 | 3/8/1/3 | `A-might-smaragdakis-vanhorn-kcfa-paradox-pldi-2010` | **PASS** (arbiter-read; EXPTIME-direction settled verbatim — functional/closures exponential, OO/flat polynomial; matches 076:16). hum=1 was the word "might". Grade A. See batch-2 results. |
| DONE-b1 | 13 | 1/8/0/1 | `A-verified-interpreter-shell-vstte-2017` | **MILD** (V3 "symbolic over-approximation" at 021:16 — top lead) + V1/V2 PASS; grade A. Pulled up from rank-13 for entanglement w/ the shell-front-end flag. See batch-1 results. |
| DONE-b3 | 19 | 1/6/0/5 | `A-salcianu-rinard-purity-vmcai-2005` | **PASS** (both atoms RESOLVED-DET: "mutating only new objects is pure" + "regex of touched locations" = :108-110 "regular expressions that … characterize the externally visible heap locations that a method mutates"). Grade A. Staging line emitted (cluster-1 ungraded resolved). See batch-3. |
| DONE-b3 | 18 | 2/4/0/4 | `A-ramalingam-undecidability-aliasing-1994` | **PASS** (all factual atoms RESOLVED-DET in seq-predetermine; W2 headline verbatim). Already in sources.json (registered 2026-06-02) — grade stands, no staging line. See batch-3. |
| DONE-b2 | 22 | 2/8/0/4 | `A-reps-horwitz-sagiv-ifds-popl-1995` | **PASS** (headline RESOLVED-DET; secondaries are Dorc *applying* IFDS — architectural, low-power). Grade A. See batch-2 results. |
| DONE-b2 | 22 | 2/8/0/4 | `A-reps-demand-interprocedural-cc-1994` · `A-reps-cfl-reachability-survey-1998` | reps-demand **PASS** (low-power; cited as prose "Reps cc94" 055:34,62). reps-cfl **OUT-OF-SCOPE** (no plan claim-site; "CFL-reachability" at 076:15 attributed to Heintze–McAllester, not Reps' survey). See batch-2. |
| DONE-b2 | 18 | 2/8/0/2 | `A-horwitz-reps-binkley-sdg-slicing-toplas-1990` | **PASS** ("slicing IS reachability" verbatim-grounded — "interprocedural slices ... each [pass] cast as a reachability"). Grade A. See batch-2 results. |
| DONE-b4 | 10 | 1/5/0/1 | `A-distefano-scaling-static-analyses-facebook-cacm-2019` | **PASS.** NARROWED residual resolved: "0%→70%" **is** the fix-rate (*"the fix rate … was near zero … rocketed to over 70%"* :479-487); scale figures (100M LOC/<30min/24-core :234,:889; ~1T LOC/day :245) verbatim. Grade A. Staging line emitted. See batch-4. |
| DONE-b2 | 26 | 2/5/1/5 | `B-burgess-cfengine-2010` | **PASS** (convergence "more than idempotence" + cfengine's basis grounded at slide :21,:8,:16,:28). Grade B (informal author slides). reaches-human. See batch-2 results. |
| DONE-b3 | 20 | 2/7/1/1 | `A-cfengine-security-trust-model-2001` | **PASS** · **reaches-human** (the three security lessons verbatim-grounded: encryption≠trust :230-233, verify-host-identity :223-224, integrity-over-secrecy :217). "pull/voluntary-cooperation" absent from doc → KEEP-not-finding (true-of-CFEngine framing, not the load-bearing checklist). Already in sources.json — no staging line. See batch-3. |
| DONE-b3 | 21 | 2/3/0/6 | `A-traugott-order-matters-2002` | **PASS** (trichotomy :371 + Turing-equivalence title :13 RESOLVED-DET; camp-assignment correct :428; unsolvability-spine framing has bearing text :2655/:2711/:2733/:555 — interpretive, doc self-hedges). Already in sources.json (`.html`) — no staging line. See batch-3. |
| DONE-b2 | 22 | 3/7/0/3 | `A-lucassen-gifford-effect-systems-popl-1988` | **PASS** (neutral+adversarial pair; "latent" strong-verbatim, "commutative" grounded via MAXEFF=least-upper-bound). Grade A. See batch-2 results. |
| DONE-b2 | 23 | 2/7/0/5 | `A-heintze-tardieu-demand-pointer-analysis-pldi-2001` | **PASS** (low-power; interpretive demand-lineage 055:34,62; title-level — body OCR-garbled → caveat-NOTRUTH, no non-support inferred; handoff grade A stands). See batch-2 results. |
| DONE-b3 | 17 | 2/7/0/2 | `A-smoosh-popl-2020` | **PASS** (executable-POSIX-semantics core RESOLVED-DET :13; "Smoosh (OCaml)" is Lem-cored per :73 — logged, immaterial). Grade A. Staging line emitted (confirms handoff). See batch-3. |
| DONE-b3 | 16 | 2/6/0/2 | `A-jensen-moller-tajs-type-analysis-javascript-sas-2009` | **PASS** (the batch's one genuine dispatch candidate — both attributed NUMBERS verbatim-confirmed by arbiter-read of §5: "<2% vs 87% before" recency-disabled :830-832, "512MB" OOM :819-820). Grade A. Staging line emitted. See batch-3. |
| DONE-b4 | 15 | 2/3/0/3 | `A-engler-deviant-behavior-2001` | **PASS.** MUST/MAY direction confirmed: MUST→contradiction-is-error (:14), MAY→statistical/coincidence (:196) — matches plan's declared=MUST / mined=MAY-grade. Already graded A/+1:SURE in sources.json — no staging line. See batch-4. |

### tier-B — adjudicate next (lower blast-radius or lower method-power)
`A-dozer-icse-seip-2022` (16, **DONE-b4 PASS→B**) · `A-harnad-symbol-grounding-1990` (14, **DONE-b4 PASS**; *plan downgraded the analogy's reach, source-grade stands*) ·
`A-cousot-abstract-interpretation-popl-1977` (10, **DONE-b4 GAP**: image-only; provenance-graded A for migration) · `A-arzt-bodden-reviser-incremental-ifds-icse-2014` (9, **DONE-b4 PASS**) ·
`A-spall-mitchell-rattle-perfect-dependencies-2020` (9, **DONE-b5 PASS**) · `A-scholz-souffle-datalog-cc-2016` (8, **DONE-b5 PASS**) · `A-mokhov-build-systems-a-la-carte-icfp-2018` (8, **DONE-b5 PASS**) ·
`A-moller-schwartzbach-static-program-analysis-2025` (8, **DONE-b5 PASS→B** rekey; *textbook, low power; section-numbers exact*) ·
`A-tobin-hochstadt-logical-types-2010` (8, **DONE-b5 PASS**; *already full in sources.json — deep-read discharges its "abstract-read only" hedge; no staging line*) · `A-biabduction-popl-2009` (8, **DONE-b5 PASS**) · `A-foster-flow-sensitive-qualifiers-2002` (7, **DONE-b6 PASS**; *headline RESOLVED-DET; all four factual atoms verbatim/near-verbatim; abstract-read hedge discharged; no staging line — already a full `sources/` entry*).

### generic-token / round-10–11 bracket set — lowest marginal value (per user: already got the sourcing-discipline)
`ansible-challenges` · `ansible-error-handling` · `terraform-{graph,external-plan-exec,native-tests}` · `test-kitchen` ·
`docker-seccomp` · `puppet-{litmus,transaction-report}` · `bash-in-the-wild` · `shellcheck-readme` · `chef-whyrun` (+ the other 45 bracket-keys).
Audit only if budget remains after tier-A/B.

## seq-adjudicate — expensive pass (✓ tier-A + tier-B DRAINED — batches 1–6 done; remainder = deprioritized gap-access/OCR + generic-token sets, audit-only-if-budget)

Per work, top-down from tier-A. Unit = the work with ALL its claim-sites bundled.
1. **pin** *(top-level; restate caveats)* — `grep` the work across `plans/`; quote each asserting sentence; decompose
   to atomic sub-propositions; tag factual-attributable vs interpretive-analogical (only factual gets full power —
   tension-POWER); carry inherited `+SURE`/`human-corrected` markers.
2. **read** *(2 subagents in parallel — the only fan-out; clean context, NO Dorc framing, NO meta-process, NO caveats)*
   — neutral: "does the source establish atom T? quote it." adversarial (`adversarial-crosscheck` skill): disown+invert,
   carry the "don't manufacture faults; say where the criticism fails" guard. **Each MUST return verbatim source quotes
   with locators, or "no bearing text" — no quote, no claim** (the anti-hallucination anchor).
3. **arb** *(top-level; restate caveats)* — adjudicate on the *quotes*, not the agents' rhetoric. EGREGIOUS only on a
   quotable mismatch a human verifies in seconds (wrong number · source about a different thing · claim inverts the
   source's conclusion · attribution to a work that lacks it). Interpretive/mild/divergent → log, no escalation.
4. **trace** *(on confirmed finding — first-class here, the whole point of targeting blast-radius)* — `grep` the work
   across all `Research/` + DESIGN/KNOBS/TODO; map the broken atom into named knobs/walls/verdicts; check later-round reliance.
5. **log** — update `status:` (PASS / MILD / EGREGIOUS / GAP) + the quotes + verdict, here.

### batch-1 results — CoLiS / shell-front-end cluster (2026-06-03; `4c1d789`; 0 EGREGIOUS)

Batch = the entangled shell-front-end neighborhood (the rank-77 + rank-39 queue head, plus rank-13 `verified-interpreter` **pulled up** because the one FLAGGED atom — plans/021:15 "differential testing vs dash/bash" — spans Morbig + verified-interpreter + colis-installation-tacas-2020 and splitting it would force a re-read). Dispatch deviated from the 2-subagent default for two works, justified by the precheck having already settled their factual atoms: `morbig` and `colis-spec` got a **single neutral pass** (grade + one confirmation read) since `morbig`'s factual atoms were all RESOLVED-DET and `colis-spec`'s plan-claims are purely interpretive (tension-POWER). `colis-installation` + `verified-interpreter` got the full neutral+adversarial pair (live contested atoms).

A deeper precheck during pinning resolved more than the headline pass: the plans/021:15 quote *"as the specification is informal, it is impossible to prove our code formally correct… we actually do not even claim the absence of bugs"* is **verbatim in Morbig** (lines 1217-1220), and colis-installation:435-437 is **verbatim** for the dash-comparison. Both crux passages were read by the top-level arbiter directly (not outsourced).

- **`A-colis-installation-scenarios-tacas-2020` → MILD.** Grade A (+1:SURE), supersedes handoff. claim-sites: plans/021:48, 084:27.
  - C1 (~28k scripts) PASS — *"28 814 maintainer scripts in 12 592 different packages"* (:78, :736, :795).
  - C3 (shell→IL not proven, trusted via review + auto-tests vs dash) PASS, strong — *"The conformance of the CoLiS script with the original shell script is not proven formally but tested by manual review and some automatic tests. For the latter, we developed a tool that automatically compares the results of the CoLiS interpreter … with the results of the Debian default shell (dash)"* (:433-437; also :376-377, :424-425).
  - C2 **MILD**: engine-generic-over-per-command-knowledge (*"The symbolic engine is generic with respect to the utilities: their specifications … are taken as parameters"* :681-682) and corpus-scale (28,814 scripts/~half-hour :736) are verbatim. But plans/021:48's appositive **"hundreds of commands"** has **no bearing text** — the paper says only *"most of the UNIX commands called by the maintainer scripts"* (:565-566), unquantified. Both passes flagged exactly this. An unsourced quantifier on an otherwise-accurate claim, not a contradicted number → MILD, log only.
- **`A-verified-interpreter-shell-vstte-2017` → MILD.** Grade A (+1:SURE), **upgrades** handoff's -0:SUSPECT grading+relevance (both passes agree A/SURE). claim-sites: plans/021:14-16, 102:68-69, 055:99.
  - V1 (verified only the IL interpreter, Why3+SMT, deliberately not Coq/Isabelle) PASS — :52, :139-141, :746-749 (*"most of the time done using … Coq or Isabelle. Yet … with automatic provers only, was already shown possible"*), :741-742 (Alt-Ergo/Z3/E).
  - V2 (parser + shell→IL **not** formally verified; trusted by review/testing) PASS, strong — *"Since the correctness of the translation from shell to CoLiS cannot be proven … one will have to trust it by reading or testing it"* (:114); parser is ref [14], translation is future work (:791-794).
  - V3 **MILD (batch top lead)**: plans/021:16 — *"The proof effort bought soundness of a symbolic over-approximation."* Both passes: **unsupported as written**. The paper proved soundness **and completeness** of a concrete *interpreter* w.r.t. the CoLiS semantics — an equivalence, not an over-approximation (*"proven both sound and complete"* :779; Theorem 1 :514; *"the file system as well as the built-ins … are left abstract … We focus only on the structure of the language"* :333). "Abstract interpretation / over-approximation" is **Abash**, cited as *others'* work (:772). **Not escalated**: defensible only if "the proof effort" denotes the broader CoLiS symbolic-execution project (whose engine *does* soundly over-approximate) rather than this interpreter proof — that referent ambiguity needs intent-adjudication, so it fails the "verifies in seconds" EGREGIOUS bar. Recommend (human's call, additive-only) clarifying 021:16 that VSTTE-2017's proof is interpreter soundness+completeness, while "symbolic over-approximation" belongs to the symbolic-execution engine (TACAS-2020), not the proof.
  - V4 (validate vs dash/bash) partial — VSTTE-2017 only *motivates* the comparison ("will allow us to compare … by real shell interpreters" :135); the *performed* comparison is in TACAS-2020 (C3). Immaterial: plans/021:15 cites TACAS for it.
- **`A-morbig-sle-2018` → PASS.** Grade A (+1:SURE). claim-sites: plans/021:15,26-27,38,51,54; 041:11,25.
  - M1 (Menhir incremental purely-functional LR + speculative/reentrant) PASS (:176-181, :885-888, :957-960, :1058-1059).
  - M2 (informal spec → cannot prove parser correct → no absence-of-bugs claim) PASS, **verbatim** = the plans/021:15 quote (:1217-1220).
  - M3 (validate by coincidence with Dash & Bash) PASS (:1255-1260) — nuance: paper frames it as *"to disambiguate several paragraphs of the standard"* (sanity-check), narrower than "validation methodology," but the substance holds. **Dissolves the register's "differential absent from Morbig" flag**: the technique is present; only the word "differential" is absent (immaterial — the plan never quotes "differential" as Morbig's term).
- **`B-colis-specification-of-unix-utilities-2019` → PASS.** Grade **B** (+1:SURE) — confirms handoff A→B. claim-sites: plans/102:68 (bracket-key), 055:61,98 (interpretive "ontology seed" / "canonical-fact set").
  - SP1 (formal per-command UNIX-utility filesystem-effect specs as feature-tree/FOL constraints) PASS (:308, :310, :319, :633). Self-labels *"[Technical Report] ANR. 2019"* (non-refereed) → B.

**Cross-cutting — plans/021:15 parenthetical "(Morbig paper; TACAS 2020)" → MILD (citation terseness, not the conflation the precheck feared).** With all three works adjudicated, the sentence resolves as a **dual** citation (semicolon-separated): *Morbig [SLE'18]* supplies the "can't prove parser correct / no absence-of-bugs" quote (verbatim M2); *TACAS'20 = colis-installation* supplies the "shell→IL trusted via review + dash-comparison" half (verbatim C3). Every sub-claim is grounded; the only defect is that "Morbig paper" is unlabelled as SLE'18 (it is **not** TACAS 2020) and the two halves trace to two papers. Additive-only clarification (human's call): e.g. "Morbig (SLE'18) for the parser; CoLiS (TACAS'20) for the dash-comparison."

**Carry-forward:** `A-colis-platform-sttt-2022` is **not cited in `plans/`** (zero hits; the only colis bracket-key in plans is `[A-colis-specification-of-unix-utilities-2019]`). It is therefore **out of audit scope** (plans are the target); its handoff grade (A, -0:SUSPECT) stands verbatim, un-superseded. Likewise `A-jeannerod-phd-thesis-2021` was not encountered as a plan claim-site here. Next batch resumes top-down at the next PENDING: `A-might-smaragdakis-vanhorn-kcfa-paradox-pldi-2010` (rank 28, NARROWED — confirm EXPTIME *direction*) · `B-burgess-cfengine-2010` (26, reaches-human) · `A-heintze-tardieu-demand-pointer-analysis-pldi-2001` (23) · the reps-* / lucassen-gifford / horwitz-reps-binkley 055-spine cluster (22-18).

### batch-2 results — 055-spine + leaders (2026-06-03; `4c1d789`; 0 EGREGIOUS, 0 MILD)

Batch = the rank-28→18 PENDING head: the Might–Smaragdakis k-CFA paradox, Burgess/cfengine, and the plans/055 analysis-architecture PLT spine (the reps-*, lucassen-gifford, horwitz-reps-binkley citations). Dispatch deviated from the 2-subagent default heavily, justified per work: only **lucassen-gifford** carried live, body-dependent factual atoms (the neutral+adversarial pair ran on it alone). The rest resolved by precheck-RESOLVED-DET, by arbiter-read of the determining passage, or as low-power interpretive-analogical lineage cites (tension-POWER). All seven works PASS; nothing rose to MILD.

- **`A-might-smaragdakis-vanhorn-kcfa-paradox-pldi-2010` → PASS.** Grade A (handoff stands). claim-sites: plans/076:16,52; 083:112,192; 088:127. The NARROWED residual (which side is EXPTIME) is **settled verbatim** by the abstract/intro, read directly by the arbiter: *"the exact same specification of k-CFA is polynomial-time for object-oriented languages yet exponential-time for functional ones"* (:15); *"the former create implicit closures when lambda expressions are created, while the latter require the programmer to explicitly 'close' … the data"* (:27). Exactly plans/076:16 (poly for OO/flat, exp for functional/closure; blowup needs closure-style variable recombination). The MS-mapping ("Dorc's flat fact-map → polynomial-amenable regime") is interpretive-analogical and the plan self-hedges it ("Verify the flat-domain assumption in the spike"). No dispatch needed; misalignment essentially impossible.
- **`B-burgess-cfengine-2010` → PASS.** Grade **B** (informal author slide-deck; reaches-human). claim-sites: plans/090:243-244, 099:30,127. The factual atoms — convergence is a *stronger* property than idempotence, and is cfengine's basis — are grounded: *"1999-2002 Formalized concept of 'convergence' and limits for system correctness (more than idempotence)"* (slide :21); *"convergent or self-healing semantics"* (:8); *"Convergence to end state"* (:16); *"(convergence + idempotence) … Run many times — system never gets worse"* (:27-28). caveat-NOTRUTH discharged: readable, and supports. The plan already hedges the cite as `[cand. B-burgess-*]` (provisional front) and the DESIGN "not convergence" link as "Burgess-adjacent" — appropriately tentative; no overstatement.
- **`A-lucassen-gifford-effect-systems-popl-1988` → PASS.** Grade A (+1:SURE; both passes agree), supersedes the cluster-1 ungraded slot. claim-sites: plans/090:223, 099:109,128. **Full neutral+adversarial pair** (the one live body-dependent read this batch).
  - LG-latent (099:109 "'latent' is borrowed from Lucassen–Gifford") **PASS, strong-verbatim** — *"a subroutine type incorporates a latent effect, which describes the side-effects that the subroutine may have when it is applied"* (p.2); also p.3, p.5. The most at-risk atom; it holds outright.
  - LG-commutative (090:223 "[L-G effect systems] is *commutative*") **PASS** — effect combination is `MAXEFF` = *"the least upper bound of its arguments"* over a lattice of *"unions of simple effects"* (p.1,3); a lub/union join is commutative, so the algebraic characterization (motivating Gordon's non-commutative effect quantale) is sound. Both passes independently raised and dismissed the red herring that the *operational* rules enforce left-to-right *evaluation* order (p.7) — governs evaluation, not effect-combination. Adversarial pass explicitly affirmed both attributions hold; no manufactured fault.
- **`A-horwitz-reps-binkley-sdg-slicing-toplas-1990` → PASS.** Grade A, supersedes cluster-1 ungraded. claim-sites: plans/055:32,47. "HRB: slicing IS reachability" + backward-slicing-as-graph-reachability are **verbatim-grounded** (arbiter grep): *"interprocedural slices to be computed in two passes, each of which is cast as a reachability"* (:1551); also :545,:1488,:1511. SDG/PDG provenance confirmed (abstract). The architectural reuse claim ("build once, query many ways") is Dorc's design move, low-power.
- **`A-reps-horwitz-sagiv-ifds-popl-1995` → PASS.** Grade A (+1:SURE relevance — directly load-bearing), supersedes cluster-1 ungraded. claim-sites: plans/055:31,38; 099:18,129. Headline RESOLVED-DET in seq-predetermine (verbatim "interprocedurally realizable paths" + "the Tabulation Algorithm"); secondary sites are Dorc *applying* IFDS as its fact-layer engine (architectural), not attributing a fact to the paper — low-power, no dispatch.
- **`A-reps-demand-interprocedural-cc-1994` → PASS** (low-power). Grade A, supersedes cluster-1 ungraded. Cited only as informal prose "Reps cc94" for the demand-analysis lineage (plans/055:34,62); title-level grounded (*"Solving Demand Versions of Interprocedural Analysis Problems"*, Reps, CC 1994 / LNCS 786). Interpretive-analogical.
- **`A-heintze-tardieu-demand-pointer-analysis-pldi-2001` → PASS** (low-power). Grade A stands (handoff; **not** superseded — body unreadable, so no adversarial read possible). Interpretive demand-lineage cite (plans/055:34,62); title legible (*"Demand-Driven Pointer Analysis"*, Heintze) but **body OCR-garbled** → per caveat-NOTRUTH the unreadable body is NOT taken as non-support; the title-thesis + handoff provenance (PLDI 2001, dblp-confirmed) carry the analogical cite.

**Out-of-scope (plans are the target):** `A-reps-cfl-reachability-survey-1998` has **zero** `plans/` claim-sites (the "CFL-reachability" cubic-floor at plans/076:15 is attributed to *Heintze–McAllester*, not Reps' survey); its handoff/`sources.json` grade stands un-superseded.

**Carry-forward (batch-2):** Next PENDING, top-down: `A-cfengine-security-trust-model-2001` (rank 20, **reaches-human** — threat model 102:51-52,170 + 101:27,76-77; factual trust claims "encryption ≠ trustworthiness / verify host identity", untouched by precheck — a genuine dispatch candidate) · `A-traugott-order-matters-2002` (21, headline RESOLVED-DET trichotomy + title-level Turing-equivalence — likely cheap PASS) · `A-salcianu-rinard-purity-vmcai-2005` (19, headline RESOLVED-DET) · `A-ramalingam-undecidability-aliasing-1994` (18, headline RESOLVED-DET, strongest) · `A-smoosh-popl-2020` (17) · `A-jensen-moller-tajs-type-analysis-javascript-sas-2009` (16, headline RESOLVED-DET "recency abstraction") · `A-engler-deviant-behavior-2001` (15, headline RESOLVED-DET) · `A-distefano-scaling-static-analyses-facebook-cacm-2019` (13, NARROWED — is plans' "0%→70%" the fix-rate or an adoption trajectory?). Most are RESOLVED-DET cheap-PASSes; the genuine dispatch candidates are **cfengine-security-trust-model** (security / reaches-human, untouched) and **distefano** (the 0%→70% NARROWED residual). After tier-A: tier-B, then the gap-access/OCR set and the deprioritized generic-token bracket set.

### batch-3 results — security/ops + 055 precision-levers + foundations (2026-06-03; `4c1d789`; 6/6 PASS, 0 EGREGIOUS, 0 MILD)

Batch = the rank-21→16 PENDING head + ramalingam(18). **Zero dispatch this batch** — every high-power factual atom
resolved either by seq-predetermine `RESOLVED-DET` or by arbiter-direct-read of the determining passage (the
sanctioned crux-read move from batch-1/2). The one genuine dispatch candidate going in was jensen-moller-tajs (two
attributed *numbers*); both dissolved on a direct read of the eval section. caveat-NOTRUTH discharged per work (all
readable; all support). Three low-power nuances logged, none status-changing. Self-check (tension-MYBIAS): the two
TAJS number-quotes were re-read skeptically — both are unambiguous, verifying in seconds in the *reverse* direction.

- **`A-traugott-order-matters-2002` → PASS.** Already in sources.json (`.html`) — grade stands, **no staging line.**
  claim-sites: plans/099:25,30,77,127; 090:205,237-240,342 (090 = research-plan, `[cand.]`). Trichotomy
  *"…convergent, and congruent"* (:371) + title *"Turing Equivalence"* (:13) RESOLVED-DET. Camp-assignment correct —
  *"is congruent according to our definition, not convergent"* (:428) ⇒ Traugott=congruent advocate, Burgess=convergent.
  The 099:30 "unsolvability spine" framing ("both name tracking-state + understanding-intentions as the unsolved core")
  has bearing text — :2655/:2711/:2713 "keep track of … internal state / what changes have already been [made]", :2733
  "who decides?", :555 "correctly understand the [intentions]" — interpretive (tension-POWER) but grounded; the plan
  self-hedges (`~SUSPECT` at 099:25). No surviving high-power atom.
- **`A-cfengine-security-trust-model-2001` → PASS.** **reaches-human.** Already in sources.json (`.html`) — **no staging
  line.** claim-sites: plans/101:27,77-78; 102:51-52,170. The three human-facing security lessons all verbatim-grounded:
  (a) *"encrypted connections do not change these trust [relationships] … not their accuracy or trustworthiness"*
  (:230-233) = "encryption ≠ trustworthiness" (102:52); (b) *"authenticate the identity of the host … once the host's
  identity is verified"* (:223-224) = verify-host-identity / host-key verification (102:53,170 = E2 mitigation); (c)
  *"The input file does not even have to be private as long as"* (:217) = integrity-over-secrecy (101:77-78). **Logged
  nuance:** "pull/voluntary-cooperation" (101:77) — voluntary / cooperat / pull / push are ALL absent from this doc; per
  caveat-NOTRUTH that only KEEPS the atom — it is a true-of-CFEngine architectural descriptor (corroborated by sibling
  `B-burgess-cfengine-2010`), not the load-bearing trust-checklist the cite actually grounds → fails "verifies-in-seconds",
  not escalated. The human-facing walls (E2 host-key verify, integrity-over-secrecy) are accurately sourced.
- **`A-salcianu-rinard-purity-vmcai-2005` → PASS.** Grade **A** (`-0:SUSPECT` — VMCAI mid-tier vs POPL/PLDI; a strict
  grader might high-B). papers/ migration target → **staging line emitted** (resolves a cluster-1 ungraded). claim-sites:
  plans/055:21,22,35,58. Both factual atoms RESOLVED-DET: "mutating only new objects is pure" (predetermine: *"they
  mutate only new objects"* / *"A method is pure if it does not mutate any location…"*) + "regex of touched locations" =
  *"generate regular expressions that completely characterize the externally visible heap locations that a method
  mutates"* (:108-110; also :23, :129-130, :634, :299). Per-method summaries (055:58) interpretive/architectural,
  low-power. **URL PROPOSED** (Springer DOI `10.1007/978-3-540-30579-8_14`) — not in corpus; curl-verify at migration.
- **`A-ramalingam-undecidability-aliasing-1994` → PASS.** Already in sources.json (registered 2026-06-02) — grade stands,
  **no staging line.** claim-sites: plans/099:19,39,129. W2 headline (099:39 "precise alias/footprint undecidable *even
  intraprocedurally*, PCP reduction") RESOLVED-DET in seq-predetermine (verbatim *"The Undecidability of Aliasing"* /
  *"reducing the Post's Correspondence"* / *"even the simpler intraprocedural"*). Other sites lineage/bibliographic. No
  surviving atom → cheap PASS.
- **`A-smoosh-popl-2020` → PASS.** Grade **A** (`+1:SURE`). papers/ migration target → **staging line** (confirms handoff
  A). claim-sites: plans/021:38,61; 041:13; 101:126; 102:68. Core RESOLVED-DET: *"executable small-step semantics for the
  POSIX shell, which we call Smoosh"* (:13; title :6). **Logged nuance:** :73 *"1 034 SLOC of Lem. (Lem is an OCaml-like
  language that can compile to … OCaml)"* — so "Smoosh (OCaml)" (021:61, 041:13) is ecosystem-accurate but Lem-cored;
  immaterial to the soundness-base use (102:68). "permissive license" (041:13) is a repo fact not in the PDF → absence ≠
  finding.
- **`A-jensen-moller-tajs-type-analysis-javascript-sas-2009` → PASS.** Grade **A** (grading `-0:SUSPECT` — SAS venue;
  relevance `+1:SURE`). papers/ migration target → **staging line** (confirms handoff A; was the batch's one genuine
  dispatch candidate, resolved by arbiter-read). claim-sites: plans/055:30,33,97; 076:17; 090:281. **Both attributed
  numbers verbatim-confirmed in §5** (the EGREGIOUS-class "wrong number" risk, dissolved): "87%→<2% precision when
  [recency] disabled" = *"With this technique disabled, the analysis of richards.js can only guarantee that a constant
  property is present in 2 of the 156 read-property nodes (i.e. less than 2%, compared to 87% before)"* (:830-832);
  "OOM at 512 MB" = *"cryptobench.js, presently causes our prototype to run out of memory (with a limit of 512MB)"*
  (:819-820). The precheck's "different 87%" suspicion (the :739 read-property figure) is the SAME baseline, not a
  misread. dead-code (055:30) RESOLVED-DET (:150 "can also detect dead code"); recency-naming RESOLVED-DET (origin is
  Balakrishnan–Reps [3], TAJS *adopts* it — the plan never claims TAJS invented it); context-sensitivity (055:97)
  grounded (:838 delta-blue.js). Nuance (non-finding): the numbers are per-benchmark (richards.js / cryptobench.js,
  512MB a *configured* limit), faithfully used by the plans as illustrations ("keystone lever" / "memory is the wall").

**Carry-forward (batch-3):** tier-A is nearly drained. Next PENDING, top-down: `A-engler-deviant-behavior-2001` (15,
headline RESOLVED-DET — MUST/MAY beliefs + "bugs as deviant behavior"; likely cheap PASS) · `A-distefano-scaling-static-
analyses-facebook-cacm-2019` (10/13, **NARROWED** — the one live residual: is plans' "0%→70%" the *fix-rate* or an
*adoption trajectory*? 076:18 + the handoff relevance-desc both read it as **fix-rate** at diff-time — a genuine
arbiter-read/dispatch candidate; resolve against the CACM article's diff-time-deployment figure). After those two:
**tier-B** (dozer · harnad[plan already downgraded] · cousot[**GAP-ACCESS**: image-only, needs Tesseract] · arzt-bodden ·
spall-mitchell-rattle · scholz-souffle · mokhov · moller-schwartzbach[textbook, low-power] · tobin-hochstadt ·
biabduction · foster), then the gap-access/OCR set, then the deprioritized generic-token bracket set.

### batch-4 results — tier-A drain (engler · distefano) + tier-B head (dozer · harnad · cousot · arzt-bodden) (2026-06-03; `4c1d789`; 5/6 PASS + 1 GAP, 0 EGREGIOUS, 0 MILD)

Batch = the last two tier-A PENDING (engler, distefano) + the tier-B head (dozer, harnad, cousot, arzt-bodden).
**Zero dispatch** — every surviving factual atom resolved by seq-predetermine `RESOLVED-DET` or arbiter-direct-read
of the determining passage (the sanctioned crux-read move). The two genuine EGREGIOUS-class "wrong number" candidates —
distefano "0%→70%" and arzt-bodden "~80%" — both dissolved on a direct read: verbatim in-source and in the *reverse*
(skeptical) direction. caveat-MYBIAS self-check: both number-quotes re-read against the source's own words; unambiguous,
verify in seconds. **State surprise reconciled:** the live `sources.json` already holds entries for all six — engler &
harnad as *full, audited grades* (url-acquired, `graded-by: top-level-agent` → no staging line); distefano, dozer, cousot,
arzt-bodden as *minimal `papers/` stubs* (`file: papers/…`, no grading fields → staging line emitted to grade them for
the deferred migration). **tier-A is now fully drained.**

- **`A-engler-deviant-behavior-2001` → PASS.** Already fully graded A/+1:SURE in sources.json (url-acquired) — **no staging
  line.** claim-sites: plans/099:16,99; 090:382 (099:132 = pointer to note 096). Headline RESOLVED-DET in seq-predetermine
  ("Bugs as Deviant Behavior" · "beliefs" · MUST×51/MAY×24). MUST/MAY **direction** confirmed by arbiter-read: *"For a set
  of MUST beliefs, we look for contradictions. Any contradiction implies the existence of an error in the code"* (:14) vs
  *"we can only infer that code may believe ! protects v. We call this type of belief a MAY belief"* (:196) + *"a
  statistical analysis to rank each error by the probability of its beliefs"* (:18). Exactly the plan's mapping — 099:16
  "*declared by an oracle* (Engler's MUST vs MAY)" = MUST (hard, contradiction-is-error); 099:99 "mining … produces
  **MAY-grade** beliefs … never a licence to elide" = MAY (statistical, needs a corpus). Interpretive-analogical onto
  Dorc's elision boundary, well-grounded; sources.json relevance-desc independently confirmed faithful. 090:382 "lineage:
  Engler 'bugs as deviant behaviour'" = title-verbatim, low-power.
- **`A-distefano-scaling-static-analyses-facebook-cacm-2019` → PASS.** Grade A (+1:SURE). `papers/` migration stub →
  **staging line emitted.** claim-sites (cited as "Facebook", not by surname — generic-token alias): plans/055:58,65,68;
  076:18,63; 083:176. **NARROWED residual resolved**: "0%→70%" is unambiguously the **fix-rate** — *"the fix rate—the
  proportion of reported issues that developers resolved—was near zero. Next, we switched Infer on at diff time … the fix
  rate rocketed to over 70%"* (:479-487); summary box *"the diff time deployment saw a 70% fix rate, where a more
  traditional 'offline' or 'batch' deployment … saw a 0% fix rate"* (:156-160). EGREGIOUS-class "wrong number" risk
  dissolved in the reverse direction. Secondary scale figures (076:18) also verbatim: *"over 100-million lines of Hack
  code, which Zoncolan can process in less than 30 minutes"* (:234-236) / *"in less than 30 minutes on a 24-core server"*
  (:889-890); *"one trillion lines of code (LOC) per day"* (:245-246); *"each procedure only needs to be visited a few
  times"* (:1019-1020 = 055:58). Compositional⇒incremental is the article's named thesis (grounded; interpretive).
- **`A-dozer-icse-seip-2022` → PASS → grade B.** `papers/` migration stub → **staging line emitted (A→B rekey, confirms
  handoff).** claim-sites: plans/076:77,80; 055:102 — all low-power/architectural ("future *Dozer-style* oracle/effect
  derivation" + coverage-list mentions). Syscall-altitude framing verbatim: *"shell commands and Ansible modules can only
  change the system state by communicating with the kernel via the system call (syscall) interface. This shared interface
  provides an opportunity to observe and compare the behavior"* (:68-71); *"Dozer works by profiling a program's behavior
  based on its interaction with the syscall interface"* (:198-200). **Logged nuance (non-finding):** Dozer itself uses
  *strace* (ptrace-based; :29-30,:73), while plans/076:77 advises *eBPF* altitude over ptrace for the live path — no
  conflict: the plan invokes "Dozer-style" only for *offline/fixture-side* derivation (Dozer is an offline migration tool)
  and does **not** attribute the eBPF/ptrace overhead numbers (<1-2% / 2-10× / 102×) to Dozer. Handoff's "task composition
  = open hard problem" (:206-210) is corroborated but is not itself a plan claim-site.
- **`A-harnad-symbol-grounding-1990` → PASS.** Already fully graded A/+1:SURE in sources.json (url-acquired) — **no staging
  line.** claim-sites: plans/090:373 (pull+grade list); 099:131 (table — *the plan already downgraded the **application** to
  ~-1:GUESS, human-adjudicated over-reach*). Title/provenance verbatim: *"The Symbol Grounding Problem. Physica D 42:
  335-346"* (:5); Chinese/Chinese-dictionary regress *"passing endlessly from one meaningless symbol … to another, never
  coming to a halt on what anything meant"* (:70) = the named impossibility the sources.json relevance-desc maps onto
  Dorc's probe/oracle escape. The plan's self-downgrade is of the *analogy's reach*, not the source's quality — correctly
  separated; source-grade stands.
- **`A-cousot-abstract-interpretation-popl-1977` → GAP.** claim-site: plans/055:25 *"The over-approximation guarantee
  (Cousot AI; TAJS): everything **unmodeled collapses to ⊤**"*. Local scan is **IMAGE-ONLY** (txt = page-markers only;
  *"image-only scan; no born-digital source … NOT the real paper"* :1), Tesseract not installed → **not dispatched, no
  claim-support inferred** (caveat-NOTRUTH: "couldn't read" ≠ "doesn't support"). Atom is **KEPT-as-GAP, not a finding**:
  the over-approximation / ⊤-on-unknown discipline is foundational AI, independently covered by the SPA textbook
  (moller-schwartzbach, tier-B) and the co-cited TAJS (PASSed batch-3) — the *claim* is low-risk though the *source* is
  unreadable. **Provenance-graded A** for the deferred migration (founding Cousot & Cousot POPL'77) → staging line emitted,
  read-depth=provenance-only, url added (ENS publications page). Installing Tesseract remains the only human-gated unblock,
  and only if 055:25 ever needs the primary rather than the SPA/TAJS cover.
- **`A-arzt-bodden-reviser-incremental-ifds-icse-2014` → PASS.** Grade A (+1:SURE). `papers/` migration stub → **staging
  line emitted.** claim-sites: plans/055:66 (+ term at 055:89); 076:65,80. The attributed **~80%** + mechanism + soundness
  all verbatim: *"Reviser produces the same results as a full recomputation, can save up to 80% of the time required"*
  (:143-145); *"saves, on average, about 80% of analysis time in comparison to a full recomputation"* (:153-156); *"Reviser
  follows a clear-and-propagate philosophy"* (:24) / *"a clear-and-propagate strategy: for each affected node it first
  clears … then re-propagates the information from all the node's predecessors"* (:316-319); *"This is an
  over-approximation"* (:315). Exactly plans/055:66 ("clear-and-propagate … over-approximate = safe … ~80% saved, identical
  results"). EGREGIOUS-class number dissolved — source says "up to 80%" / "about 80%"; the plan's "~80%" is faithful (if
  anything conservative vs the max).

**Carry-forward (batch-4):** **tier-A fully drained** (all DONE across b1–b4; 0 EGREGIOUS, 3 MILD total — all in batch-1's
shell-front-end cluster). Remaining = **tier-B tail** (rank-order): `A-spall-mitchell-rattle-perfect-dependencies-2020`
(9) · `A-scholz-souffle-datalog-cc-2016` (8) · `A-mokhov-build-systems-a-la-carte-icfp-2018` (8) ·
`A-moller-schwartzbach-static-program-analysis-2025` (8, **A→B rekey per handoff**; textbook, low-power, fuzzy "supports")
· `A-tobin-hochstadt-logical-types-2010` (8, headline RESOLVED-DET) · `A-biabduction-popl-2009` (8) ·
`A-foster-flow-sensitive-qualifiers-2002` (7, headline RESOLVED-DET). Most carry numeric/architectural cites checkable by
arbiter-read or already RESOLVED-DET (foster, tobin-hochstadt). After tier-B: the gap-access/OCR fetch set
(Opdebeeck/GLITCH/Rahman/Begoug/InfraFix/Mitogen/pyinfra/Murder · Ammons · Callahan · Heintze–McAllester) only where a
dependent claim is reached, then the deprioritized generic-token bracket set. New staging lines this batch: distefano(A) ·
arzt-bodden(A) · dozer(B) · cousot(A, provenance) → grading-staging.jsonl now 15.

### batch-5 results — tier-B drain (spall-rattle · scholz-souffle · mokhov · moller-schwartzbach · tobin-hochstadt · biabduction) (2026-06-03; `4c1d789`; 6/6 PASS, 0 EGREGIOUS, 0 MILD)

Batch = the rank-9→8 tier-B PENDING head (`foster`, rank-7, rolls to next batch — the ≤6 cap). **Zero dispatch** — every factual
atom resolved by precheck-`RESOLVED-DET` (mokhov vocab, tobin triple, souffle semi-naïve) or arbiter-direct-read of the determining
passage (souffle Table 4, rattle §6.4, the SPA TOC). Both genuine EGREGIOUS-class candidates dissolved on a full read, in the
skeptical direction. **State surprise (caveat-NOTRUTH, logged):** the rattle precheck saw only the §2.6/§4.1 hazard *definitions*
("read-write"/"write-write" hazards) and the CPU-pipeline/RAW-WAR-WAW framing looked like a plan overlay — a provisional MILD. The
§6.4 full-read **retracted** it: the paper itself draws the CPU analogy explicitly. "Couldn't find it in §2.6" was not "absent."
Five `papers/` stubs → staging lines; tobin-hochstadt (already a full `sources/` entry) → none.

- **`A-spall-mitchell-rattle-perfect-dependencies-2020` → PASS.** Grade A (+1:SURE; OOPSLA 2020, PACMPL Art.169). `papers/` stub →
  **staging line.** claim-sites: plans/090:207,268-277,322-323,363 (090:351 F5 = the *sequel* [cand. rattle-formally-2022], **not**
  this paper). The plan's "Rattle borrows the **CPU pipeline hazard taxonomy** (RAW/WAR/WAW + speculative write-before-read)"
  (090:268-270) is **verbatim-grounded in §6.4** (:689): *"CPUs use hazards to detect incorrect speculation, with similar types of
  read/write, write/write and write/read hazards [Patterson and Hennessy 2013] — our terminology is inspired by their approaches …
  Rattle also stalls the pipeline (stops speculating) if it detects potential hazards"* — the paper's three CPU hazard types
  (read/write=WAR, write/write=WAW, write/read=RAW) + the speculation hazard = the plan's four. Speculate-detect-fallback verbatim
  (§2.8 :176 *"aborting the build and restarting without any speculation"*; :178 speculated-write-to-input). The paper's *own*
  hazard terms (read-write / write-write, §2.6 :107-108, §4.1 :229) are the two Rattle actually detects; §6.4 supplies the CPU-
  pipeline lineage the plan attributes. **Provisional MILD retracted.**
- **`A-scholz-souffle-datalog-cc-2016` → PASS.** Grade A (+1:SURE; CC 2016). `papers/` stub → **staging line.** Prose-cited
  (tool-name "Soufflé", no bracket-key) but in-scope (the name denotes this work; carries real attributed numbers). claim-sites:
  plans/055:49,63,67,88; 076:17. **EGREGIOUS-class number dissolved:** "OpenJDK points-to in 35 s" (055:63) is **verbatim** — Table 4
  *"Total … 35 [s]"* (:1152) + prose *"the same result on the same dataset can be obtained … within 35s on a commodity desktop
  system"* (:1177-1179) for the context-insensitive points-to on OpenJDK7. Semi-naïve evaluation (:355+), compile-to-native
  relational-algebra (RAM, :273+), "billions of tuples" (:131) all verbatim; "34× faster than the best … solver" (:1042). **Logged
  nuance (non-finding):** "Soufflé why-trees / provenance" (055:49) is a real Soufflé-*tool* feature but from a *later* Soufflé paper
  — **absent from this CC2016 paper**; 055:49 prose-attributes it to the tool (no CC2016 bracket-key), so per caveat-NOTRUTH it is
  not a CC2016 mis-attribution, only a tool-capability reference whose primary source is elsewhere.
- **`A-mokhov-build-systems-a-la-carte-icfp-2018` → PASS** (the batch's strongest). Grade A (+1:SURE; ICFP 2018). `papers/` stub →
  **staging line.** Bracket-cited [A-mokhov-…] at 076:36. claim-sites: plans/076:36-42. Every attributed term **verbatim**:
  Minimality (Def 2.1 :84 *"executes tasks at most once per build and only if they transitively depend on inputs that changed"*);
  Scheduler × Rebuilder factoring (:476 *"a `scheduler' … can be cleanly separated from … a `rebuilder'"*); the 3 schedulers
  (topological/restarting :481 /suspending :488); the 4 rebuilders (dirty-bit §4.2.1 :492 / verifying-trace §4.2.2 :496 /
  constructive-trace §4.2.3 :510 *"additionally stores the resulting value … share them with other users, providing cloud-build"* /
  deep-constructive §4.2.4 :514); early cutoff (:153-158); self-tracking (:12). The cross-host-memoization mapping (constructive-
  trace keyed by input content-hash → reuse anywhere) is faithful.
- **`A-moller-schwartzbach-static-program-analysis-2025` → PASS → grade B.** `papers/` stub → **staging line (A→B rekey, confirms
  handoff).** Prose-cited ("SPA" / "Møller-Schwartzbach SPA"); in-scope. claim-sites: plans/021:28,29,32,46,52,62,79; 055:30,39,41.
  **Section-number cites exact** (TOC read): 2.5 Control Flow Graphs · ch.4 Lattice Theory · ch.5 Dataflow Analysis with Monotone
  Frameworks (5.8 *"Forward, Backward, May, and Must"*) · ch.8 Interprocedural Analysis · ch.9 Distributive Analysis Frameworks
  (9.4 The IFDS Framework, 9.6 The IDE Framework). Grade **B** (lecture-notes textbook, secondary, no peer-review venue). **Logged
  nuance (non-finding):** 021:32/79 bundle "slicing" into the "(SPA §8-9)" pointer, but SPA has **no slicing chapter** — the plan's
  real slicing source is Horwitz-Reps-Binkley (055:32, PASSed b2). Low-power textbook parenthetical, not escalated; additive-only
  fix (human's call): point 021's "slicing" to HRB, keep §8-9 for interprocedural/IFDS/IDE.
- **`A-tobin-hochstadt-logical-types-2010` → PASS.** Grade A (+1:SURE) **stands**; already a full `sources/` entry (url-acquired,
  graded-by top-level-agent) → **no staging line.** claim-sites: plans/099:106-109,128; 090:366. The triple **`(latent-proposition,
  object, substitution)`** (099:106) is fully verbatim-grounded: *latent proposition* (:183,:226,:281), *object* (§3.1 :175; :182
  *"derives an object for each expression … which part of the environment an expression accesses"*), *substitution* as the narrowing
  mechanism (:183 *"the substitution of the actual object for the formal parameter in the latent proposition"*) — exactly the plan's
  "narrowing = substituting the accessed object into it." "occurrence typing" reformulation `RESOLVED-DET` (:8). **Discharges the
  owed full-read** (090:366) and the entry's own *"abstract-read only … revisit if a deep read diverges"* hedge — the deep read
  **confirms, does not diverge** (recommend, human's call: lift that hedge in sources.json).
- **`A-biabduction-popl-2009` → PASS** (low-power interpretive). Grade A (+1:SURE; POPL 2009). `papers/` stub → **staging line.**
  Bracket-cited at 090:233,338. claim-sites: plans/090:195,232-233,338. Bi-abduction inferring frame + anti-frame is **strongly
  verbatim** (:23-26 *"infers anti-frames (missing portions of state) and … frames"*; :259-264 the entailment *A ∗ ?anti-frame ⊢ G ∗
  ?frame*); "footprint" present (:1441,:1505). "Infer infers footprints" (090:233) is interpretive (the Infer *tool* post-dates this
  2009 paper but is built on it; the substance holds). Correctly positioned as the *infer* side of the kBURDEN frame knob (self-
  hedged "the open knob").

**Carry-forward (batch-5):** **tier-B is down to one** — `A-foster-flow-sensitive-qualifiers-2002` (rank-7; headline RESOLVED-DET in
seq-predetermine: title *"Flow-Sensitive Type Qualifiers"* + strong/weak-update present; also a full `sources/` entry, abstract-graded,
carrying the same *"abstract-read only"* hedge as tobin-hochstadt → likely cheap PASS + hedge-discharge, no staging line). After
foster: the **gap-access/OCR fetch set** (Opdebeeck/GLITCH/Rahman/Begoug/InfraFix/Mitogen/pyinfra/Murder · Ammons · Callahan ·
Heintze–McAllester · cousot[GAP]/tip-survey OCR) only where a dependent claim is reached, then the deprioritized generic-token bracket
set. New staging lines this batch: biabduction(A) · mokhov(A) · moller-schwartzbach(B) · scholz-souffle(A) · spall-rattle(A) →
grading-staging.jsonl now 20.

### batch-6 results — tier-B tail drain (foster) (2026-06-03; `4c1d789`; 1/1 PASS, 0 EGREGIOUS, 0 MILD)

Batch = the sole tier-B remnant, `A-foster-flow-sensitive-qualifiers-2002` (rank-7). **Zero dispatch** — headline
RESOLVED-DET in seq-predetermine and the full born-digital text (clean, no OCR gap) let the arbiter ground every
factual atom directly (the sanctioned crux-read move from b1–b5). caveat-NOTRUTH discharged: readable, and supports.
tension-MYBIAS self-check: F2 (uniqueness) and F3 (effect-masking gloss) re-read skeptically against the source's own
words — both hold in the reverse direction.

- **`A-foster-flow-sensitive-qualifiers-2002` → PASS.** Grade A (+1:SURE) **stands**; already a full `sources/` entry
  (url-acquired, graded-by top-level-agent) → **no staging line.** claim-sites: plans/099:110-112 (the CQual synthesis
  line) + 099:128 (the 092 lineage-table bracket-key, bibliographic/low-power). The four factual atoms all
  verbatim/near-verbatim:
  - F1 (CQual = Foster et al.; state-facts as flow-sensitive qualifiers) — title *"Flow-Sensitive Type Qualifiers"*
    (:1) + *"we have built a tool Cqual that implements our inference algorithm"* (:513-514); authors Foster/Terauchi/
    Aiken (:3-11). RESOLVED-DET.
  - F2 (strong/weak update gated by uniqueness) — *"an abstract location is linear if … it corresponds to a single
    concrete location in every execution … We perform strong updates on locations that are linear and weak updates on
    locations that are non-linear"* (:41-43); *"1 for linear locations (these admit strong updates), and ⊤ for
    nonlinear locations (which admit only weak updates)"* (:176); *"linearity … corresponds to their singleness
    computation"* (:58). "uniqueness" = linearity/singleness, faithful.
  - F3 (frame falls out of the effect-set; *"functions don't join locations they don't use"*) — **near-verbatim**
    :345 *"this rule gives us some low-cost polymorphism, in which functions do not act as join points for locations
    they do not use"*; mechanism at :45 (*"If an expression e does not reference location ℓ … the analysis of ℓ can
    simply flow … without passing through e"*). The word *"frame"* is Dorc's separation-logic overlay on Foster's
    effect-masking — interpretive-analogical (tension-POWER), glossed with a near-verbatim quote → **non-finding**, not
    a misattribution.
  - F4 (declare+infer = `kBURDEN`; qualifier *lattice* = second anchor-locus) — declare+infer grounded (*"Users
    annotate their programs with type qualifiers, and inference checks that the annotations are correct"* :14, :26);
    qualifier lattice grounded (*"a natural subtyping relation … users can define a partial order on the qualifiers"*
    :28; *"a supplied partial order ⊑ among constant qualifiers"* :174). The `kBURDEN`/anchor-locus tags are
    Dorc-internal, low-power.
  - **Hedge discharged:** the sources.json entry's grading-reasoning carries *"Abstract-read only; A rests on
    venue+provenance, not full-text verification."* — the full read **confirms** (PLDI'02 full paper: type system +
    soundness-referenced [15] + 513-module Linux-kernel locking eval), does **not** diverge. Recommend (human's call,
    additive-only): lift that hedge in sources.json (parallels the tobin-hochstadt b5 recommendation).

**Audit status (post-b6) — substantive target complete.** Every tier-A and tier-B source (the high-blast-radius ×
checkable × load-bearing set) is adjudicated: **6 batches, 0 EGREGIOUS, 3 MILD** — all three MILD in batch-1's
shell-front-end cluster (colis-installation "hundreds of commands"; verified-interpreter "symbolic over-approximation";
the 021:15 dual-citation terseness), each additive-only and human-gated. The remainder is **deliberately out-of-budget**
per the standing prioritization and the user's "we're basically done": (a) the **gap-access/OCR set**
(Opdebeeck/GLITCH/Rahman/Begoug/InfraFix/Mitogen/pyinfra/Murder · Ammons · Callahan · Heintze–McAllester · cousot[GAP] ·
tip-survey OCR) — fetch-and-adjudicate *only if a dependent claim is reached* (none escalated through b1–b6); and (b) the
**generic-token bracket set** (~45 ansible/terraform/docker/puppet/… keys) whose sourcing-discipline the user has already
accepted. Neither is an open correctness risk; both are low marginal value. **Recommendation: close the audit here** unless
the human wants the generic-token tail swept for completeness.

**Annotation policy (fork-annot, locked):** default is fact-finding only — **zero edits to any file**. The only write
path is, *when the human requests it*, an additive **ahistorical** annotation placed local to the inaccuracy (a dated
audit tag beside the claim; historical text untouched). Substantive change / deletion is the human's alone.

**Residual weaknesses (stated, not hidden):** tension-MYBIAS (the arbiter is Dorc-pilled → minimize paraphrase, rest
arb on source quotes both agents independently surface) · tension-POWER (near-unfalsifiable framing claims —
Traugott "trichotomy", the SPA textbook, "maps to"/"analog" — labelled low-power, not audited symmetrically).

## seq-predetermine — deterministic no-LLM pre-filter (✓ first pass: tier-A headline atoms)

Shrinks the agent queue by settling purely-lexical claims for free. Method: `pdftotext` v4.00 → `.txt`
sidecars beside each `sources/*.pdf` (gitignored dir; `papers/*.txt` pre-existed), then `grep` each claim's
*attributed tokens* (quoted terms · named techniques · numbers) against its source text. **Reversed
conservativity:** a match ELIMINATES an atom only when the claim is purely "the source contains/uses/names X"
*and* X is present verbatim. Absence never proves a finding — it only *keeps* the atom (caveat-NOTRUTH); and
`pdftotext` line-splitting can only cost elimination-power, never mis-eliminate.

Greppable inventory: `papers/*.txt` (36) + `sources/*.txt` (23 sidecars). **OCR gap:** `cousot-1977` +
`tip-survey-1995` are image-only scans and **Tesseract is not installed** → deferred (low-priority; Cousot is
theory-covered by the SPA textbook). Installing Tesseract is the *only* thing that needs the human, and only if
those two matter.

**RESOLVED-DET — headline atom confirmed verbatim; nothing non-obvious left (evidence = source quote):**
- `ramalingam` / 099-W2 "precise alias undecidable *even intraprocedurally*, PCP reduction" → *"The Undecidability
  of Aliasing"* · *"reducing the Post's Correspondence"* · *"even the simpler intraprocedural"*. (Strongest.)
- `salcianu-rinard` / 055 "mutating only new objects is pure" → *"they mutate only new objects"* · *"A method is
  pure if it does not mutate any location…"*.
- `verified-interpreter` / 021 "Why3+SMT, deliberately not Coq, ghost-skeleton" → *"…syntax and semantics in
  Why3"* · *"Proving termination with ghosts and skeletons"* · *"proof assistants like Coq or Isabelle. Yet…"*.
- `reps-horwitz-sagiv` / 055 "realizable-path reachability + tabulation summaries" → *"interprocedurally
  realizable paths"* · *"the Tabulation Algorithm"*.
- `engler` / 099,096 "MUST vs MAY (beliefs); bugs as deviant behaviour" → *"Bugs as Deviant Behavior"* ·
  *"inferring programmer 'beliefs'"* · MUST ×51 / MAY ×24.
- `foster` / 092 "flow-sensitive qualifiers, strong/weak update" → title *"Flow-Sensitive Type Qualifiers"* + strong/weak-update present.
- `tobin-hochstadt` / 092 "occurrence typing, latent propositions" → *"reformulate occurrence typing"* · *"latent … proposition"*.
- `jensen-moller-tajs` / 055,054 "recency abstraction" → *"in particular recency abstraction"*.
- `traugott` / 099,091 "divergent/convergent/congruent trichotomy" → *"three categories: divergent, convergent,
  and congruent"* (only the *Dorc = congruent* mapping stays interpretive — tension-POWER).

**NARROWED — terms confirmed, a specific residual remains for agents (do NOT eliminate):**
- `might-smaragdakis` / 076 — EXPTIME · polynomial · OO · flat · closure all present + title *"Functional vs.
  Object-Oriented"*; residual = grep can't pin the *direction* (which side is EXPTIME). Near-certain; cheap confirm.
- `distefano` / 055 — *"70% fix rate"* · *"diff time"* · *"compositional"* present; residual = is the plan's
  "0%→70%" the *same* 70% (fix-rate vs adoption-trajectory)?
- `colis` / feasibility — *"Debian"* · *"maintainer scripts … Posix shell"* confirmed; the specific count ("27k")
  not located deterministically (only years matched) → agent or fuller grep.
- `burgess` / 091 — *"convergent / self-healing"* confirmed but extraction thin (85 lines); deeper claims need fuller text.

**KEEP / FLAGGED — deterministically un-eliminable, mild lead:**
- `morbig`+`verified-interpreter` / 021 "differential testing vs dash/bash" — *differential* is ABSENT from BOTH
  Morbig-SLE-2018 and VSTTE-2017 (dash/bash present in Morbig). The parenthetical "(Morbig paper; TACAS 2020)" also
  conflates Morbig-SLE-2018 with the colis-TACAS-2020 paper. Not a finding — agents check whether either paper
  *describes* output-comparison-vs-dash/bash under other words.

**Net:** ~9 tier-A headline atoms eliminated with evidence, ~4 narrowed, 1 flagged. The agent pass shrinks to:
NARROWED residuals · secondary (non-headline) claim-sites not yet enumerated · tier-B · gap-access/OCR · the
deprioritized bracket set.

## Handoff reconciliation — papers/ migration + grading (folded in)

A parallel session graded 28/36 legacy `papers/` sources (non-adversarial "grade-to-merge" mandate →
`_papers-handoff.md`) and built `_adopt-source.sh` (mutative migration tool). It was stopped mid-way;
`sources.json` left pristine (no migration applied). **Integrity caveat:** that session ran `git checkout`
×3 during tooling tests — working==index==131 entries, no detected loss, but the index is *not* a pristine
baseline; trust the live `sources.json`, not git history of it.

- **Grading folds into the adversarial audit (decision).** The 8 still-ungraded papers (cousot · lucassen-gifford ·
  tip-survey · horwitz-reps-binkley · reps-horwitz-sagiv · reps-cfl · reps-demand · salcianu-rinard) are ⊆ the
  tier-A/B audit targets — the pass reads them anyway. So: audit-target sources → grade from the adversarial pass
  (`graded-by: top-level-agent`, SUPERSEDES the handoff's non-adversarial grade, which is insufficient where we
  genuinely validate claim-support). Non-audit sources (doop · souffle · codeql · svf · sridharan · szabo×2 · abadi ·
  vanhorn-mairson · bash-in-wild · jeannerod-thesis · bravenboer) → handoff grade stands verbatim.
- **4 A→B rekeys** (rename `[A-…]`→`[B-…]` in plans/notes, do not change claim-support): `dozer` (tier-B) ·
  `colis-specification` (tier-B) · `moller-schwartzbach` (tier-B, already low-power) · `ansible-challenges`
  (generic-token). Deterministic RESOLVED-DET atoms are unaffected — **no re-pass.**
- **Migration DEFERRED, human-gated.** `_adopt-source.sh` moves `papers/*→sources/*` and rekeys the LIVE,
  concurrently-edited `sources.json` — mutative + collision-risky, and the audit reads those files in place. The loop
  writes grading to `_scratch/grading-staging.jsonl` (the `_adopt-source.sh` stdin schema); the human runs the
  migration + ref-fixes + `validate.sh` afterward. Source resolution is location-agnostic (try `sources/` then `papers/`).
- New source in the handoff, not yet in the queue: `A-jeannerod-phd-thesis-2021` (CoLiS PhD, ~195pp, graded A) —
  audit-target only if plan-cited (likely notes-only; check at pin time).
