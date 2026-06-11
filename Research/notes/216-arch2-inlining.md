# 216 — arch-2: budget-bounded function inlining (brk-2). Strain + decisions.

> Round-21 w-3, builder note (slug 216 reserved per 211 §4). Charter: 211 §1 arch-2 + the
> i-1…i-6 design in the round-21 priming prompt (restated by the orchestrator). Append-only;
> confidence-marked (+SURE/~SUSPECT/-GUESS/--WONDER). Engine edits: `analysis/src/cfg.rs`
> (the splice: `try_inline_call`/`splice_funcdef_body`, the funcdef registry, eligibility +
> budgets, `spliced_internal`/`call_body_sites`, `lower_funcdef` now marks its detached body
> non-leaf), `analysis/src/value.rs` (`inline_pass` positional binding + the positional `Frag`
> path), `analysis/src/effect.rs` (`SkipClass::InlineCall`/`InlineSite`, the call node gens
> Pure, spliced sites excluded from the leaf list), `plan/src/lib.rs`
> (`prove_inline_replaceable` + `inline_disposition` + `push_inline_checks`,
> `LicenseVia::InlineCall`). cli UNCHANGED (the `RecordKey { site, member }` N.M wire already
> existed from L2). Tests: `analysis/tests/cfg.rs` (+13), `analysis/src/value.rs` inline (+6),
> `plan/tests/observable_matrix.rs` (+8), 7 new e2e (`inline21-*`), `exec-detached-fn`
> re-homed. HEAD before task: `92162f1`. (Late addition after the hunt-6 self-crosscheck: an
> EXPLICIT in-loop floor in `inline_disposition` + the `inline21-in-loop-call-floored` case +
> a unit pin — see §1.4/hunt-6/§7.)

## §0 Headline (what shipped)

+SURE (traced + tested + e2e-exec-gated ×2): a call to a same-file-EARLIER funcdef is now a
**CFG-level subgraph splice** at the call site. The body's AST is freshly lowered into new CFG
nodes after the CALL node (un-detaching it — the find-7 fix), with `$1`..`$9`/`$#` bound from
the call's resolved argv; the CALL leaf is the render/substitution unit, the spliced body
commands are `spliced_internal` (non-leaf) and ship as `site N.M` probe sub-records OF THE
CALL. The all-or-nothing CALL license (the 20S Members precedent) substitutes the call's span
to `true` iff every effect-bearing body leaf licenses elision. The arch-2 payoff lands
end-to-end: `apt_install() { apt-get install -y "$1" >/dev/null 2>&1; }; apt_install nginx;
apt_install curl`, both converged ⇒ both CALL spans → `true`, run-set EMPTY
(`inline21-wrapper-converged-elides`). Gates green ×2: `cargo fmt --check` ·
`clippy --workspace --all-targets -D warnings` (no new expects) · `cargo test --workspace`
(388 tests) · `sh e2e/run.sh` ×2 (81 cases, ZERO xfail, all six gates) · `typos spike`.

## §1 Design-as-built

### §1.1 The splice mechanism (i-1) — `cfg.rs`

The keystone decision: **splice at CFG-build time, in `lower_simple`, with the body lowered
AFTER the CALL `Command` node** (not before, not via a post-pass). Rationale (each +SURE,
traced):
- `effect::classify` already walks CFG `Command` nodes and reaches over them; once the body
  is spliced + reachable, effect classification / reaching-defs / pristine-prefix / errexit
  all flow for free (i-5) — the body commands gen their own effects downstream EXACTLY as
  inline code would.
- The value plane runs over the same CFG; positional binding is a side-channel pass (§1.2).
- The CALL stays a real `Command` node, so the consumed-channel marking (`&&`/`||`, errexit,
  `$?`) attaches to it through the EXISTING machinery (the call node IS a fallible Command) —
  this is what makes i-5 "zero new special-casing" true.

`try_inline_call` resolves the command word against a funcdef registry (`collect_funcdefs`:
name ↦ each `(body AstId, def-start BytePos)`, source order), then enforces eligibility
in order (§2 inventory), then `splice_funcdef_body` lowers the body via `lower_node(body,
cmd)` between `inline_stack.push(body)`/`pop()` (the recursion guard) and marks the new arena
range `spliced_internal`, collecting the body's effect-bearing leaf nodes into
`call_body_sites[cmd]`. A transitively-inlined inner call's leaves are FLATTENED into the
outer call's site list (the inner call node has its own entry, skipped to avoid double count).

**The detached-funcdef change (load-bearing, surprised me how clean it was).** `lower_funcdef`
ALSO lowers the body detached (the original, for arena consistency). Pre-arch-2 those detached
commands surfaced as `MustRun`/`skip-unresolvable` LEAVES of their own (unreachable, harmless,
but noisy AND double-counting the spliced copy). I made `lower_funcdef` mark its detached body
`spliced_internal` too: a funcdef DEFINITION's body commands are never independent Step leaves
(the body renders verbatim inside `name() { … }`, runs only via the per-call splices). Without
this, `exec-detached-fn` showed TWO `apt-get install` leaves (the detached + the spliced) —
the bug I hit first (§4 strain-1).

### §1.2 Positional binding (i-2) — `value.rs`

A spliced body's `$1`..`$9`/`$#` are RUNTIME input to the general lattice transfer (folded to
⊤, the sound default for an un-inlined funcdef), so — exactly like the 20S Members
side-channel — the binding rides a SEPARATE post-solve pass (`Prep::inline_pass`) that NEVER
flows through the lattice. For each inlined call it reads the call's resolved argv, builds a
`Positionals` overlay (`$N` = operand N, `$#` = operand count, a ⊤ operand ⇒ ⊤ positional),
and re-resolves each body site's words via a positional-aware recipe path (`collect_frags_pos`
+ two new `Frag` variants `PosLit`/`PosSplit` carrying the bound value directly). The result
populates `ValueFlow::positional_argv`, and `argv_values` returns it in place of the
⊤-positional general-transfer argv for a spliced site — so `effect::command_effect` resolves
`dpkg -s "$1"` to `dpkg -s nginx` at `apt_install nginx` (i-4 entity resolution).

POSIX scope (i-2): body variable ASSIGNMENTS leak to the caller (they ride the ordinary
lattice transfer — no scope node), so the site's incoming env already carries them; the ONE
scoped overlay is the positionals. Nesting (depth ≤ 2): the pass is BOUNDED-iterated
(`MAX_INLINE_PASSES = 3`) so an inner-of-inner positional settles once the outer binding lands
(monotone: a concrete binding never changes).

**Recorded imprecision (flag arch2-positional-via-assignment, ~SUSPECT benign).** The overlay
applies to the body command site's DIRECT word resolution, NOT to an intermediate ASSIGNMENT
that reads a positional: `w() { p="$1"; cmd "$p"; }` resolves `p="$1"`'s RHS via the LATTICE
(which sees `$1` as ⊤), so `p` is ⊤ and `cmd "$p"` ⇒ ⊤ (the safe degrade). The wrapper-pun
population uses `$1` DIRECTLY at command sites (`dpkg -s "$1"`, `apt-get install "$1"`), which
IS bound, so the imprecision does not bite the target idiom. Pinned
(`body_assignment_from_positional_is_top_recorded_imprecision`).

### §1.3 The effect aggregate (i-3/i-5) — `effect.rs`

Two changes. (a) An inlined CALL node gens `Pure` into reaching-defs — NOT the `Opaque` the
unmodeled command word `prov` would resolve to. The body (spliced after the call) carries the
real effects; classifying the call `Opaque` would poison its OWN spliced body (the install
reads `EstablishWritten` instead of `Ambient`) — the EXACT poison the splice exists to remove
(§4 strain-2, the second bug I hit). (b) The classify loop excludes `spliced_internal` sites
from the emitted leaf list (like `is_expansion_internal`), and an inlined CALL node emits
`SkipClass::InlineCall { sites: Vec<InlineSite> }` aggregating each body site's own
classification (resolved with the call's positionals bound). The per-site single-fact
classification is extracted into a `classify_site` closure shared by the ordinary leaf path
and the body-site aggregation.

**No mini-fold needed (the i-5 tc-flag, RESOLVED — did NOT need to stop).** The brief flagged:
"if the call-status-prediction turns out to need the fold to evaluate body terminal paths,
STOP and report scope before building a mini-fold." It does NOT. The CALL's own status is
ALWAYS ⊤ (a mutator-shaped aggregate's rc has no sanctioned source, fork-mutator-rc — like
Members). The all-or-nothing aggregate licenses on the body's ESTABLISH FACTS (each
EstablishAmbient body cell Converged), NOT on the body's internal control-flow deadness. The
wrapper-pun `dpkg -s "$1" || apt-get install "$1"` body, when converged: the only Establish is
the install (Converged) ⇒ the call elides; the body's internal `||` deadness of the install
is IRRELEVANT to the call's decision (whether the install's fact is converged is what matters,
not whether the guard short-circuits it). So the global AST fold is NOT consulted for body
deadness (it would be WRONG — the body's AstIds are shared across call sites, so a per-call
fold-observe is impossible; the InlineCall is excluded from `leaf_fact` so the body folds to ⊤
harmlessly). +SURE this is sound and is what kept the slice from growing a mini-fold.

### §1.4 The all-or-nothing license + probe (i-3/i-4) — `plan.rs`

`prove_inline_replaceable` mints a `LicenseVia::InlineCall` `Replace` iff: every
`EstablishAmbient` body site's fact is Converged (a single non-Converged ⇒ refuse);
NO body site is a blocker (an `EstablishWritten`, a `MustRun` (Kill/Opaque/⊤), an in-loop
`EstablishMembers`, or a nested `InlineCall` — defensive); a body `QueryResolvable` does NOT
block (read-only); the call has ≥1 converged establish (a pure-builtin wrapper RUNS — refuse,
the run-it floor, no synthetic fact needed); and the CALL's own consumed channels pass
`consumption_ok` (status ⊤ ⇒ a consumed status blocks; door-3 `|| true` `StatusInvariant`
does not). `inline_disposition` (called from `build_plan` before `disposition_for`, like
`members_disposition`) observes each body Establish's verdict and mints/refuses. The CALL span
substitutes to `StandIn::True`.

`push_inline_checks` ships one `ProbeCheck` per effect-bearing/probeable body site, `member =
Some(body-site-index)` (the `site N.M` sub-record), with the entity bound at the call.
All-or-nothing on probe-ability: an un-probeable ESTABLISH ⇒ the WHOLE call unresolvable
(`can't-probe ⇒ can't-elide`); a Query without a probe body is omitted, not a blocker.

### §1.5 Render (i-3) — rides arch-1, UNCHANGED

The CALL leaf is an ordinary Step; arch-1's span-edit render (note 214) substitutes its span
to `true` (or `:` for a fold-dead Omit, never the call here). The spliced body leaves are NOT
Steps, so they are NEVER render-edited — their span belongs to the shared definition. No
render code changed. Verified: `apt_install nginx` (a call leaf) → `true` in-situ; two calls →
two independent span edits.

## §2 Eligibility-refusal inventory (i-1; every exclusion loud)

All refusals emit a `cfg-inline-refused` WARNING (proportional degradation — the call stays
`Opaque`, runs as an ordinary command; NOT an `error` ⊤-reject, per 211's "Opaque-with-
diagnostic, never a cliff"). Flag tc-inline-refusal-severity: warning vs error is a judgment
call — I chose warning (the call runs fine, just unoptimized; gate-3's error-floor does not
fail it). In order of check:

| # | exclusion | trigger | pinned |
|---|---|---|---|
| e-0 | not a funcdef / forward call | word is not a same-file-earlier funcdef name (def AFTER the call, or a non-funcdef word) | SILENT (might be a PATH binary) — `forward_call_before_definition_is_not_inlined` |
| e-1 | redefinition | name defined > 1 in the file | `redefined_function_call_refuses_with_diagnostic` |
| e-2 | recursion | resolved body already on the active inline stack (direct/transitive) | `direct_recursion_refuses_with_cycle_diagnostic` |
| e-3 | depth | inline_stack depth ≥ 2 | `depth_budget_refuses_a_fourth_level` |
| e-4 | body `$@`/`$*`/`shift`/`local` | span-contained scan of the body AST | `body_using_{shift,local,dollar_at}_refuses` |
| e-5 | tc-M2 body write-redirect | a `>`/`>>` redirect to a non-`/dev/null` target in the body | `body_write_redirect_to_real_file_refuses_but_devnull_is_exempt` |
| e-6 | per-call node budget | body AST-subtree node estimate > 64 | `at_budget_body_inlines_over_budget_refuses` |
| e-7 | per-book node budget | running spliced-node tally + estimate > 1024 | (no isolated test — covered by e-6's mechanism; ~SUSPECT a multi-call book hitting 1024 is hard to construct cheaply) |

**Recursion structural note (+SURE after tracing).** A TRUE body-call cycle is structurally
UNREACHABLE under the "definition strictly before the call" rule for body-internal calls: a
body call sits at its definition's textual position, so the resolvable-call relation is a
strict partial order with no cycle. The `inline_stack` guard is the belt-and-suspenders for
DIRECT recursion (`p() { p; }`, where `p`'s body is on the stack when the inner `p` resolves).
Transitive recursion is naturally broken by the textual-ordering forward-call refusal. Pinned
both: `direct_recursion_*` (the stack fires) and `mutual_recursion_terminates_no_infinite_splice`
(the chain terminates via ordering).

**Budget estimate (~SUSPECT, a deliberate conservatism).** The per-call budget is checked from
the body's AST-SUBTREE node count (a pre-estimate, checked BEFORE any CFG node is allocated so
refusal needs no rollback), NOT the actual spliced CFG node count. AST descendants ≥ the CFG
leaf nodes the body lowers to, so over-estimating refuses MORE (the safe direction). The exact
CFG-node count is only known post-splice, which would need a rollback the append-only arena
does not cheaply support — so the AST proxy is the honest engineering choice. A body of ~21+
simple commands crosses 64; the `inline21-overbudget-degrades` case uses 40 `echo`s (127
nodes).

## §3 Churn table (per case → bucket → justification)

Buckets: **re-home** (an existing case whose disposition legitimately changed under inlining,
re-derived with justification); **new** (the six `inline21-*`).

| case | bucket | one-line justification |
|---|---|---|
| exec-detached-fn | **re-home** | Pre-arch-2: `prov() { apt-get install -y nginx; }; prov` showed site:0 (detached body MustRun) + site:1 (`prov` Opaque), both skip-unresolvable, run-set `apt-get install -y nginx`. NOW: `prov` inlines, the body install is `site 0.0` (probed), the call runs when the body establish is DIVERGED. Re-pointed to the inline-diverged-runs pole (probe-results `site 0.0 effect=absent`, a new `dpkg-query` exit-1 shim) — run-set UNCHANGED (`apt-get install -y nginx`). The detached-poison PIN re-homed onto the recursion-refusal unit test (`recursive_call_refuses_inline_and_poisons_the_body`) + the `inline21-overbudget-degrades` e2e (an Opaque refused call poisons the curl below, identical to the old detached poison). |
| inline21-wrapper-converged-elides | **new** | The payoff: def + two calls, body `>/dev/null 2>&1`, both converged ⇒ both CALL spans → `true`, run-set EMPTY. |
| inline21-wrapper-diverged-runs | **new** | nginx converged ⇒ `apt_install nginx` elides; curl diverged ⇒ `apt_install curl` runs whole (calls independent). |
| inline21-recursion-rejects | **new** | self-calling helper ⇒ loud `cfg-inline-refused`; outer call runs; the recursion-Opaque poisons the curl below. Analysis-only (infinite-loop at runtime). |
| inline21-overbudget-degrades | **new** | 127-node body > 64 budget ⇒ Opaque-with-diagnostic; site:0 + site:1 skip-unresolvable (detached-fn-poison-identical). Analysis-only. |
| inline21-redirect-body-refuses | **new** | body `>> /etc/motd` ⇒ tc-M2 refuse-with-diagnostic; call runs. Analysis-only (a body writing /etc/motd must never execute). |
| inline21-errexit-call-composes | **new** | `set -e` book; bare `apt_install nginx` blocks (consumed ⊤ status ⇒ runs) AND `apt_install curl || true` (door-3 StatusInvariant unblocks ⇒ elides). The composition, errexit+door-3 riding the CALL node for free. Run-set `apt-get install -y nginx`. |

**Zero semantic golden churn outside {re-home, new}.** +SURE: only `exec-detached-fn` among
the pre-existing 75 cases changed (its expected.out + probe-results + a new dpkg-query shim);
the other 74 are byte-identical (verified — the e2e first reported exactly `1/75` changed, and
that one is the documented re-home). No BLESS was run (every golden hand-derived by running
dorc and inspecting).

## §4 What strained

- **strain-1 (the first bug) — the detached body double-counts the spliced body.**
  `lower_funcdef` STILL lowers the body detached, so `prov() { … }; prov` produced TWO
  `apt-get install` Command nodes: the detached copy (unreachable, MustRun, a Step leaf) AND
  the spliced copy. The detached copy surfaced as `argv 0 run apt-get install -y nginx` + a
  `site:0 skip-unresolvable`, on TOP of the call. FIX (+SURE): mark `lower_funcdef`'s detached
  body `spliced_internal` too — a definition's body commands are never independent leaves. This
  collapses the definition to its proper "no runnable leaf of its own" shape.
- **strain-2 (the second bug) — the call node poisons its own spliced body.** With the body
  spliced after the call node, the call node's effect (computed by `command_effect` on `prov`,
  an unmodeled word) was `Opaque` ⇒ it genned `Reach::Top` ⇒ the body's install (a successor)
  read `EstablishWritten` instead of `EstablishAmbient` ⇒ the call never elided. FIX (+SURE):
  an inlined CALL node gens `Pure` (the body carries the effects). Caught by the
  `called_function_body_inlines_to_a_single_call_leaf` assertion (the body site was
  Discriminant(2)=EstablishWritten before the fix, Discriminant(1)=EstablishAmbient after).
- **strain-3 — the pun's internal `||` Query guard needs oracle co-authoring (FLAG, deferred).**
  The literal 207 pun `dpkg -s "$1" >/dev/null 2>&1 || apt-get install -y "$1"` SPLICES and
  binds positionals correctly (`dpkg -s nginx`, `apt-get install nginx` per call — verified),
  but the `dpkg -s` guard misclassifies as MustRun (not Query) because the oracle's
  `dpkg__check` verb-binding and the effect-map's `oracle_effect dpkg -s query …` verb key did
  not match (an oracle-AUTHORING gap, orthogonal to arch-2's mechanism — "the oracle dialect is
  UNTOUCHED" per the brief). So the e2e `inline21-wrapper-converged-elides` uses the
  devnull-ESTABLISH wrapper (`apt-get install -y "$1" >/dev/null 2>&1`) instead — same payoff,
  same devnull-exemption, no internal-Query dependency. The `||`-pun-with-internal-Query is a
  recorded follow-up needing a correctly-authored dpkg query oracle; the inlining mechanism is
  proven (the splice + positional binding + the establish aggregate all work for it).
- **strain-4 — `set -e` shifts the call leaf-ids (a fixture authoring gotcha, not a bug).** In
  `inline21-errexit-call-composes`, `set -e` is leaf 0, so the calls are leaf 1/2 and the body
  records are `site 1.0`/`site 2.0` (not 0.0/1.0). My first probe-results used the wrong keys
  and nothing elided. The wire is correct; the fixture must key to the actual call leaf-ids
  (which the rendered probe discloses). Noted for any future inline-case author.

## §5 Flags (tc-*/doc-deltas — surfaced, not resolved)

- **doc-delta-1 (orchestrator/human to apply — I don't touch CLAUDE.md):**
  `crates/analysis/CLAUDE.md` describes the OLD detached-funcdef reality in three places, now
  superseded by arch-2:
  - line 11 (the `effect.rs` bullet): "a detached function body has a vacuous-⊥ in-state …
    fold to `MustRun`" — still TRUE for a genuinely-detached body (an UNCALLED funcdef, or a
    refused call), but a CALLED funcdef now splices (the body is reachable, classified live).
  - line 28 (`seam-interproc`): "Today `lower_funcdef` builds function bodies **detached**
    (`find-7`: pred-less `Entry`, seeded errexit-`Off`, forced `MustRun`) … Add intra-file
    call/return edges … to un-⊤ those bodies." — arch-2 IS that work (a CFG-splice, not
    IFDS summaries). The find-7 errexit residue is MOOT for runtime (the spliced copy gets its
    errexit inflow from the call; the detached copy is a non-leaf island). `. /path` source-
    following is still unbuilt.
  - line 52 (the "Tension to flag"): the `seam-interproc` ↔ `seam-finite` tension framing
    assumed un-⊤-ing via the recursive entity-algebra across call boundaries; arch-2's
    bounded INLINING sidesteps the unbounded-kind-nesting worry entirely (the body is a finite
    splice, depth-bounded), so the predicted finite-height pressure did NOT materialise for
    the shallow shapes — the IDE/summary path stays the labeled fire-escape (209 §0/§2).
  Proposed: update these to "a CALLED funcdef inlines (arch-2/brk-2); a genuinely-detached
  (uncalled/refused) body stays MustRun/non-leaf."
- **doc-delta-2:** `spike/CLAUDE.md` `inv-leaf-seam` ("a stable `LeafId → AstId` back-map")
  and `inv-site-keyed-results` are now NON-INJECTIVE in one direction (two call sites' body
  sub-records map to one shared body AstId) — but the LEAF/Step back-map stays injective (only
  calls + ordinary leaves are Steps, distinct AstIds; body sites are not Steps). Worth a
  one-line note that the back-map's injectivity holds at the STEP level, and the N.M sub-record
  keying (LeafId/call-site + member) is what keeps body records distinct. §6 hunt-1 documents
  the consumer audit.
- **tc-inline-refusal-severity (strain §2):** a refused inline is a `warning`, not an `error`.
  Judgment call (proportional degradation, not a correctness ⊤-reject). Flagged in case the
  human wants the louder error severity for the "every exclusion loud" reading.
- **flag arch2-positional-via-assignment (§1.2):** an intermediate `p=$1` inside a body does
  not propagate the positional (p stays ⊤). Safe degrade; does not bite the direct-`$1` idiom.
- **flag arch2-pun-internal-query (strain-3):** the `||`-pun-with-internal-Query-guard needs a
  correctly-authored dpkg query oracle to elide; the inlining mechanism is proven for it.

## §6 Adversarial hunt-list (WRITE-IT-YOURSELF — ranked; a hostile crosscheck must EXCEED this)

Hostile-identity framing: "a builder I distrust added CFG-level function-body SPLICING — fresh
CFG nodes per call site, sharing the definition's AstIds, with a positional side-channel and an
all-or-nothing aggregate license. The disaster class: a wrong-elision (a call elides when its
body needed to run), a poison-leak (a refused call's Opaque NOT poisoning downstream), or a
non-deterministic / corrupt splice." Construct every probe against dash (the semantic oracle),
not the engine.

- **hunt-1 (HIGHEST) — the back-map non-injectivity consumers (i-6).** Two calls' body
  sub-records map to ONE shared body AstId. I AUDITED every AstId-keyed map (grep
  `BTreeMap<AstId` / `cfg.node(*node).ast` across crates):
  - `cli::node_of_ast` (AST→CfgNode): NON-injective (last-write-wins among the detached + N
    spliced copies), but `emit_debug_argv` only queries it by `step.ast` = CALL AstIds (calls
    have distinct AstIds; body sites are not Steps) ⇒ benign.
  - `plan::leaf_fact` (AstId→FactKey): body AstIds + the InlineCall are EXCLUDED (InlineCall
    returns None), so no body AstId is a key ⇒ benign.
  - `plan::by_ast` (AstId→Disposition, the `is_neutralised` walk): built from `self.steps`
    (calls + ordinary leaves, distinct AstIds); the `subtree_leaves_all` walk over a
    controller's AST subtree never descends into a spliced body (the body is spliced in the
    CFG, not the AST) ⇒ benign.
  - `value::assigns` (AstId→recipe): one entry per shared AstId (identical recipe for all
    copies); the transfer reads it per copy with the same recipe ⇒ benign.
  - `fold::{dead,node_rc}` (AstId-keyed): the fold reaches over the AST (body leaves shared,
    once); the InlineCall is out of `leaf_fact` so body leaves fold to ⊤ (no per-call fold,
    which would be the wrong-shared-fold disaster) ⇒ benign.
  ATTACK: a shape where a body AstId IS queried for a per-call decision (does any future
  consumer assume `cfg.node(node).ast` uniquely identifies a call-site? — none do today). A
  funcdef called inside ANOTHER funcdef's body (depth-2 transitive) — do the inner body's
  sub-records key correctly under the outer call's LeafId? (The flatten in `splice_funcdef_body`
  appends inner leaves to the outer's site list; the inner's positionals come from the inner
  call's bound argv, itself bound from the outer — the bounded-iteration `inline_pass`.) Verify
  a depth-2 `a() { b "$1"; } b() { apt-get install "$1"; } a nginx` resolves the deepest install
  to `nginx` and ships ONE record under `a`'s call leaf.
- **hunt-2 — the call-status terminal-path question (i-5, I claim NO mini-fold).** I set the
  CALL status to ⊤ unconditionally. ATTACK: a body whose terminal status is KNOWN and a
  consumer reads it — e.g. `w() { command -v x; }; w foo && bar` where the body's LAST command
  is a Query with a known rc. Does the CALL deserve a known status (so `&& bar` folds)? I claim
  NO (the call is a mutator-shaped aggregate, ⊤; a body that is PURELY a Query is refused for
  elision anyway — no establish — so it runs and `&& bar` sees the real rc). But verify a body
  `{ apt-get install "$1"; command -v "$1"; }` (establish THEN query) — the call's terminal rc
  is the query's, KNOWN — does my ⊤ wrongly BLOCK a `&&` that should fold? (It blocks ⇒ the
  call runs ⇒ SAFE, never a wrong-elision; the cost is a lost fold, the deliberate
  fork-mutator-rc cost.) Confirm it is a lost-fold, never a wrong-elision, under dash.
- **hunt-3 — assignment-leak edge cases (i-2 POSIX scope).** Body assignments leak to the
  caller. ATTACK: `w() { x=set; }; w; echo "$x"` — does `x` leak to the post-call `echo "$x"`
  (it should, POSIX: no scope boundary)? Does a body assignment SHADOW a caller var across the
  call correctly (`x=caller; w` where `w() { x=body; }` ⇒ post-call `x`=body)? The spliced body
  has no ScopeEnter/Exit, so assignments flow through the lattice — verify the post-call `argv`
  sees the body's assignment, AND that a body PREFIX-env (`FOO=bar cmd` inside the body) does
  NOT leak (command-scoped). The trap: the value plane's `transfer_command` persists bare
  assignments but not prefix-envs — does that hold for a spliced body assignment?
- **hunt-4 — splice determinism across runs (i-1).** I verified byte-identical stdout+stderr
  across two runs of `inline21-wrapper-converged-elides`. ATTACK harder: a book with MANY calls
  to several funcdefs in non-source order, under a debug build (the arena is append-only and
  iteration is BTreeMap-ordered, but a `HashMap` leak would surface here). Run the whole
  `inline21-*` corpus twice and diff; run with `cargo test` twice and confirm no order-
  dependent flake. Construct a book where two funcdefs share a body-command AstId-NEIGHBOURHOOD
  and confirm the `call_body_sites` lists are stable.
- **hunt-5 — door-3-composition edges (i-5).** I verified `apt_install nginx || true` elides,
  `apt_install curl && echo` runs, and `apt_install nginx || true; echo $?` ELIDES (the `$?`
  reads the invariant list-rc 0, not the call's rc — door-3 StatusInvariant correctly handles
  it). ATTACK the chain shapes: `apt_install x || apt_install y` (the left call is a `||` left
  StatusRelaxable ⇒ ⊤ ⇒ blocks ⇒ runs; the right call's disposition?); `(apt_install x) || true`
  (a subshell-wrapped call — is the call still door-3-marked through the `( )`?); `apt_install x
  | grep y` (the call's stdout consumed ⇒ blocks); `set -e; apt_install x || true` (door-3
  under errexit — the `||` left is errexit-exempt AND StatusInvariant, so it should elide —
  verify the mark-union does not double-count). The disaster: a door-3 call eliding when a
  DIFFERENT consumed channel (stdout) should block it.
- **hunt-6 — the all-or-nothing aggregate completeness.** ATTACK a body whose effect set is
  subtle: a body with TWO establishes of DIFFERENT cells (`w() { apt-get install "$1"; apt-get
  install other; }`) — does the call elide only when BOTH are converged? (It should — both are
  EstablishAmbient sites in the aggregate.) A body with an establish AND a Query of the SAME
  cell (the pun) — does the Query's validity interact with the establish's convergence? A body
  with a multi-cell verb (an establish resolving to ≥2 cells ⇒ MustRun per the single-fact
  limit) — does it block the call? (It should — MustRun blocks.) **The in-loop call (RESOLVED
  by the builder before shipping, +SURE):** `for pkg in nginx; do w "$pkg"; done` with `w`
  inlining — `inline_disposition` runs from `build_plan` BEFORE `disposition_for`, like Members,
  so it COULD bypass the in-loop floor. I added an EXPLICIT `in_loop_body` check to
  `inline_disposition` (it no longer relies on the back-edge self-poison that ALSO tends to
  make the in-loop body establish `EstablishWritten` / unresolvable). Pinned BOTH ways:
  `inline21-in-loop-call-floored` (e2e — the call runs verbatim even when converged; the body
  establish is ALSO EstablishWritten ⇒ unresolvable ⇒ empty probe, doubly safe) +
  `inline_call_inside_loop_is_floored_even_when_converged` (unit — the single-member loop makes
  the for-var concrete so the floor is the ISOLATED operative block). STILL worth the
  crosscheck's eye: a Members-bound MULTI-member loop calling `w` (the for-var is ⊤ ⇒ the
  call's positional is ⊤ ⇒ the body Opaque ⇒ runs — verified `w TOP` runs, but re-attack the
  composition of Members PRECISION with an inlined call, an unbuilt multi-leaf case).
- **hunt-7 — the budget boundary precision.** I used an AST-node estimate, not the CFG count.
  ATTACK: a body whose AST node count is JUST under 64 but whose CFG lowering is JUST over (a
  construct that lowers to more CFG nodes than AST nodes — a `case` with many arms, an `if`
  chain). Does the per-call CFG node count ever exceed 64 despite passing the AST estimate? (It
  should not blow up — the AST estimate over-counts for simple commands, but a control-heavy
  body might invert. Verify the actual spliced CFG node count stays bounded, or accept the
  estimate is a soft guard.) And the per-BOOK budget (1024): construct a book with enough calls
  to cross it and confirm the LATER calls refuse (proportional degradation) while the earlier
  ones inlined.
- **hunt-8 (lower) — eligibility scanner completeness (i-1).** The body scanners
  (`body_uses_unmodeled_positional`, `body_has_unmodeled_write_redirect`) are span-contained
  AST walks. ATTACK: a `$@` inside a NESTED `$()` in the body (is it span-contained + caught?);
  a write-redirect inside a nested funcdef-in-funcdef (does the scan over-reach into a nested
  definition that is NOT this body's runtime?); a `shift` inside an `if` arm of the body; a
  `>>` whose target is a DYNAMIC word (`>> "$x"`) — I refuse it (the `None` arm: "dynamic/
  unresolved target"), verify that is the safe call. A heredoc body (`<<EOF`) — is it a
  write-redirect? (No — `HereDoc` op is not `Write`/`Append`, so it does NOT fence; but the
  heredoc-carrying CALL leaf would hit arch-1's heredoc render-refusal if it tried to elide —
  verify a wrapper whose body has a heredoc still behaves.)

## §7 Confidence summary

- +SURE: the splice is behavior-preserving on run-sets except the documented {re-home, new}
  set; all 81 e2e ×2 green zero-xfail; 388 workspace tests; fmt/clippy-D/typos clean; splice
  determinism (stdout+stderr byte-identical ×2); the detached body + spliced body both
  non-leaf (only the CALL renders).
- +SURE: the two bugs (strain-1 detached-double-count, strain-2 call-self-poison) are fixed
  and pinned; the call-status needs NO mini-fold (the i-5 tc-flag resolved without stopping);
  the back-map non-injectivity is benign across every audited consumer (hunt-1).
- ~SUSPECT: the AST-node budget estimate (vs CFG count) is a soft guard (hunt-7); the
  positional-via-assignment imprecision (§1.2) is benign for the target idiom; the per-book
  budget (e-7) has no isolated test.
- RESOLVED (was --WONDER): an IN-LOOP inlined call correctly stays floored. I found
  `inline_disposition` ran before `disposition_for`'s floor (the Members pattern) and COULD
  bypass it; added an EXPLICIT `in_loop_body` check + an e2e (`inline21-in-loop-call-floored`)
  + a unit pin (`inline_call_inside_loop_is_floored_even_when_converged`). The in-loop call is
  doubly safe (the explicit floor AND the back-edge self-poison making the body establish
  EstablishWritten/unresolvable). The remaining unverified composition is Members PRECISION
  (multi-member elision) WITH an inlined call — an unbuilt multi-leaf case (hunt-6).
