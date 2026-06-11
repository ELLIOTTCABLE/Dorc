# 218 — door-2 / door-4 / precedence seam: design-only (the path-not-taken record)

> Orchestrator note, round-21 close. The human ruled (2026-06-11, this session): round-21
> closes WITHOUT building task #5; this note records the design so a later round picks it
> up without re-derivation. Slug 218 honors the original 211 §4 reservation (the door-4/
> door-2/precedence BUILD note) — repurposed design-only, per the 212 precedent. NOTHING
> in this note is built; nothing here settles the human's open forks (dq-errexit-1/2/3,
> the declaration's ratified spelling — see §8). Inputs: 20V (the program), 212 (rulings),
> 222 (ecosystem evidence; cited as [222 c-N/m-N/p-N]), 21B (the dashboard decomposition),
> 211 §7 (mint forks), 21E (render scar tissue); plus, cited at point-of-use and
> belonging on any implementing round's read-list: 213/214/215/216 (the door-3 / span
> render / door-1 / inlining substrate §4 and §6 ride) and the C-1 full-argv ruling
> (round-19 lineage — resolve via STALENESS-AUDIT drift-grounding / 19A §5, NOT via
> 20V, which never names it). Strawman sh herein is FROZEN EVIDENCE, never executed.
> Confidence-marked per house rules.

## §1 The deferral record (why not built, and what an implementing round inherits)

Deferred on evidence, not doubt: 21B's headline decomposes H2SaLS's 0% into
oracle-coverage causes, with exactly ONE needs-declaration site (L58 groupadd under
`set -eu`) — which deeper analysis then showed the doors would NOT move alone (two
stacked blockers; §3(b)) — so the most expensive remaining build would have moved ZERO
sites of 172 on the measuring-stick corpus (21B recorded 195 sites; the close-out
re-derivation reproduces 172 at 21B's own commit AND at HEAD — decomposition stands,
the integer was a stale snapshot; IB-corrected in 21B), while its design inputs (222)
are harvested and don't rot. Two cautions against misreading this paragraph (they earn their place —
the neutral crosscheck flagged both as skimmer hazards): the one-site count measures the
CORPUS's oracle-coverage-boundness — a deliberately-unannotated book — never door-4's
value ceiling; and per §3, if L58 is hostile-rc as ~SUSPECTed, door-2's static-elision
value on this corpus is ZERO and the site is door-4-only territory. The human's mini-spike idea (build on a leave-behind branch) was
considered and replaced by this design-only note ("leave the complexity out of the spike
codebase at the end"). The 212 spike-disposition ("exploring exactly this sort of
dangerous state-space in a bounded, throwaway manner is perfectly reasonable — ideal,
even") still stands for whichever round builds this.

An implementing round inherits, besides this note: the 212 rulings verbatim (CLI-flag
opt-in for-sure; default `Never` PROVABLY zero transforms; precedence seam keeps ALL
THREE bare-middle ownership models live; build LAST; hostile pass mandatory; the product
hard-defers door-4 regardless of spike results) · the mandatory-crosscheck obligation
transfers intact to the build (§7 is its pre-registered attack list) · 222's mechanism
menu (m-1/m-2/m-3/m-4/m-5/m-6) and posture warnings (p-1..p-5) · the dashboard's
pre-reserved `guard-transform` / `static-declared` columns (21B §1, stable at 0).

## §2 door-2 — the declared converged-run (design)

**What it is.** The oracle author declares, as tool-knowledge, the observable a
converged re-run of a kind's mutator would produce. It is the system's FIRST
counterfactual claim-type (20V §4): every other claim describes the probeable present;
this predicts an unexecuted run. 222's taxonomy places it T-4 (never-verify-but-
attribute) — the claim-family ecosystems bled on (Chef's why-run assumptions API is its
exact prior shape, built carefully and still reputationally sunk [222 §2]) — so the two
fences 20V already names are LOAD-BEARING, not polish [222 §7]: the disclosure floor
(every declaration-elided site attributed in verdict lane + artifact comment) and
analytic pre-validation via door-1-on-wrappers (the T-3-ex-ante move).

**Spelling (strawman; kTYANNOT-precedent acceptable-debt; ratification OPEN).** Mirror
the existing per-(kind,selector) probe-declaration idiom (st-2 lineage):

```sh
# existing idiom — the kind's vouched read-only probe body:
oracle_probe_package_installed() { dpkg -s "$1" >/dev/null 2>&1 ;}
# proposed — the (provider, verb)'s declared converged-run observable (counterfactual):
oracle_converged_run_apt_get_install() { return 0 ;}
```

- **Keying: per-(provider, verb), NOT per-(kind, selector)** — corrected from this
  note's first draft by the independent design (218a d2-2, adopted; the crosscheck
  verified the dialect mirror byte-for-byte): the claim's subject is a command family's
  re-run behavior, and two providers establishing the same cell (`apt-get install` /
  `dpkg -i` → `package#installed`) have independently-true-or-false re-run claims —
  kind-keying would let one provider's declaration license another provider's sites, a
  category error. The (kind, selector) coordinate is recovered through the effect-map at
  bind time, so the kind still anchors blame and the probe. m-1's tri-level support
  [222 §1] falls out STRUCTURALLY: declare only the (provider, verb)s you vouch;
  absence = none; check-argparse narrowing = partial; no new syntax.
- **Body shape, validated strictly:** first slice accepts ONLY `return <n>` (one
  statement). The engine vomits on anything else (`dq-door2-decl-shape` — the "dorc
  introduced this concept so we're allowed to vomit over it" license, per the human's
  kind-coordination rulings). Rationale: p-1 [222] — declared rc is a small checkable
  domain; declared stdout consumed by control flow re-imports Chef's assumptions
  problem, and apt's "already newest version" text is the most rot-prone claim-shape in
  the whole survey (locale- and version-sensitive). Printing bodies are DEFERRED, not
  refused-forever; the counterfactual TEXT lives in the plan comment instead (m-6),
  where the admin can read and veto it — never in the executed artifact.
- **Sanctioned channels (p-1 adopted for the spike):** Status ← the declared rc.
  Stdout/Stderr ← sanctioned only as unconsumed (site consumes either ⇒ door-2 REFUSES,
  catalogued diagnostic). Effect ← probed-converged covers it (door-2 static elision
  mints only on a converged Effect verdict — the declaration sanctions the rc weld-4
  reserved, nothing more).
- **The stand-in:** the existing value-preserving substitution vocabulary
  (`StandIn::{True, False, Exit(n)}` — `from_rc` maps 0⇒True, 1⇒False, n⇒Exit(n); 19C
  "emit `9`, not `1`").
  The declaration feeds the EXISTING replace machinery; no new artifact shapes. The
  provenance comment carries [m-6] the assumption text + [m-2] attribution: "elided per
  package-oracle's converged-run declaration (rc=0); wrong? report to the oracle".
- **Pinning (m-3, FOD/Flyway lesson [222 §3/§4]):** the disclosure records the
  declaring oracle's source-hash alongside the claim. In-spike (no persistence) that is
  disclosure-only; the durable-cache era must key any cached verdict to it (a changed
  oracle lapses its declarations to r-2 behavior pending re-vouch, validCheckSum-style)
  and any future `--check`-style re-verifier MUST NOT inherit the elision (the FOD
  suppression hole).
- **Author-side harness (m-4, the Hummer protocol narrowed):** out of engine scope, but
  the DX direction is pre-named — run the kind's mutator twice in a container, diff the
  declared converged-run observables against the MEASURED second run. This is what
  makes declarations testable by their authors before anyone trusts them.

## §3 The converged-rc trichotomy (the design insight 21B's one site exposes)

Tools split three ways on what a converged re-run's rc actually IS, and the doors'
value splits with them:

- **benign-rc tools** (`apt-get install -y` on installed ⇒ rc 0): an honest door-2
  declaration elides the site fully. Door-2's whole static-elision value lives here.
- **hostile-rc tools** (bare `mkdir` on existing dir ⇒ rc 1, +SURE; `groupadd` on
  existing group ⇒ rc 9, ~SUSPECT exact code, +SURE nonzero): an HONEST door-2
  declaration must declare the nonzero rc — whose faithful stand-in still kills the
  `set -e` book. Door-2 on a hostile-rc tool is faithfulness-without-value: it
  reproduces the crash the bare re-run would have had. (m-6's plan comment then does
  something genuinely useful: it SHOWS the admin their book's own latent
  converged-re-run crash — provenance-as-pedagogy, the dir-soundiness-ux direction.)
  The rescue here is door-4: the inserted guard short-circuits BEFORE the hostile-rc
  mutator, so the transformed book SURVIVES a converged re-run its bare form would have
  died in — door-4 as re-run bugfix, strictly more valuable than door-2 for this class.
- **flag-softened forms** (`mkdir -p`, `groupadd -f` ⇒ rc 0 on converged): benign-rc by
  spelling; the oracle's declaration may be conditioned on the flag's presence (the
  check() argparse already sees the full argv — C-1).

Consequences: (a) the dashboard's needs-declaration column should EVENTUALLY split by
rc-class — a hostile-rc site is door-4-only territory, mislabeling it "door-2 would
move this" overstates door-2; (b) the corpus's one needs-declaration site (L58)
is now FULLY characterized (close-out re-derivation; five isolating strawmen; +SURE on
mechanism): `getent group "$grp" >/dev/null || groupadd "$grp"` inside a three-literal
`for` loop under `set -eu`, Members-class. It is blocked by TWO stacked, independent
causes: blk-1, errexit-⊤ — the mutator's StatusRelaxable-consumed rc is ⊤
(fork-mutator-rc), which a door-2 declared rc WOULD clear; and blk-2, non-self-reach —
the un-oracled `getent` guard is an in-loop Opaque command breaking the Members
license's pristine conjunct, which NO declaration clears. The dashboard surfaces only
blk-1 (`in_loop_floor` is masked whenever StatusRelaxable is present — 21B seam-3's
predicted masking, now demonstrated). Worse for door-2 specifically: on a converged
host the `||` short-circuits and `groupadd` never executes — there is no converged-run
to declare — and bare `groupadd` (no `-f`) on an existing group exits nonzero by the
site's own spelling, so an honest declaration fails the conformance gate (218a d2-4a)
regardless. Net: the site is group-QUERY-oracle territory (rung r-2, zero new trust —
and once the guard is oracled, door-1 folds the dead `groupadd` free), and the doors'
demonstrated population on this corpus is ZERO sites. The dashboard's
needs-declaration label was the engine's coarse view, now refined; (c) the canary taxonomy is graded, not binary:
full canary (run the mutator) > PARTIAL canary (door-4's guard still touches the
tool's subsystem — a dpkg -s read notices db corruption/lock-readability, not
disk-full-on-write) > zero canary (door-2's static stand-in touches nothing). This
grading belongs in the dq-errexit-1 evidence ledger; it bounds what door-4 "removes"
more tightly than 20V §2's binary framing.

## §4 door-4 — guard-insertion (design)

**License vocabulary: a NEW disposition, not a Replace.** `Disposition::Transform`
(carrying the guard text + the untouched original) alongside Run/Replace/Omit — NOT a
`LicenseVia` variant on Replace. Keeping the category structurally separate is what
stops it eroding weld-5/observable-reproduction semantics (20V §4): a Transform
reproduces NOTHING — it re-measures; its correctness argument is the four-world trace,
not channel-reproduction. The coverage crate's `#[expect(unreachable_patterns)]` arms
fire on the new variant at compile time (21B's evolution-survival working as designed);
the build lights the pre-reserved `guard-transform` column, reported forever separate
from full-elision (north-star clause).

**Guard construction.** The inserted text is the kind's (kind,selector) `oracle_probe_*`
BODY, inlined with its positional bound to the site's value-plane-resolved operand, and
unconditionally silenced:

```sh
# before:                          # after (m-a, probed-converged site):
apt-get install -y nginx          dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx  # dorc: guard per package-oracle converged-run decl
```

- Provenance: the same structurally-vouched body that ships in probes — door-4 extends
  EXISTING trust to the apply lane (the dq-errexit-3 question), it mints no new
  vocabulary. The hostile pass asserts this byte-level (§7 attack-2).
- Silencing `>/dev/null 2>&1` is appended ALWAYS (idempotent if the body already
  redirects): the guard must add no observable channels to the book.
- Errexit composition, traced: the guard sits in `||`-left = errexit-exempt context
  (`lower_condition_region` precedent from door-3's d-4 checkpoint, 213) — a diverged
  guard's nonzero rc does NOT kill the book; it routes to the original mutator. +SURE
  this is the correct lowering reading; re-verify at build.
- **Self-recognizability (the re-analysis closure clause — new in this note):** a kind
  is door-4-eligible only if its own `check()` argparse resolves its probe body's
  spelling (the package oracle's check must recognize `dpkg -s nginx`). Then re-running
  Dorc over an already-transformed book sees an ordinary admin-style guard → Query →
  door-1 fold — NO second transform, no guard-stacking, and the transformed artifact's
  elision becomes provable-from-sh (rung r-4) instead of declaration-trusted (r-3).
  Transforms are thereby idempotent under re-analysis BY CONSTRUCTION, and the
  declaration is needed only for the FIRST transform. Lint-able; m-4's harness checks
  it. ~SUSPECT this clause is the design's strongest novel property; the build should
  pin it with a transform-then-reanalyze test.

**Mint conditions (all must hold; each refusal is a catalogued diagnostic):**
1. Site is a plain Simple-command leaf (no heredoc-bearing, no multi-line operand —
   render-refuse poles ride arch-1's existing refusal machinery; first slice also
   refuses leaves sharing a line with scaffolding keywords, relaxable later).
2. Kind+selector resolved (oracled), with a door-2 declaration present — the
   declaration IS the engineer's intent-answer to the canary question (20V §2); without
   it the canary-removal decision has no owner. Note the asymmetry vs §3: door-4 needs
   the declaration's EXISTENCE (intent), not its rc VALUE (door-2's payload).
3. Mint-policy seam (211 §7, all three live behind one enum, default m-a):
   m-a probed-converged-only · m-b declared-kind-always · m-c even-unprobed. All three
   are correctness-safe (the guard re-measures); they differ in wasted-read cost,
   artifact-diff size, and disclosure posture.
4. Non-Status consumed channels pass the EXISTING gates (consumed stdout/stderr ⇒
   refuse — the guard changes line-level output timing; stay conservative first slice).
5. Not inside a dead/folded region (Omit wins — never transform unreachable code); not
   already fully elidable (Replace wins — a converged `x || true` site door-3+Effect
   already elides is CHEAPER than a transform; arm-ordering: Omit > Replace >
   Transform > Run); not in a loop body (first slice floors it, like inlining — the
   Members composition is real design work, deferred explicitly).
6. The CLI flag is on (default `Never`): the policy module's Never arm returns identity
   and the build carries a corpus-wide test asserting BYTE-IDENTICAL artifacts under
   Never — the 212 "provably zero transforms" obligation as a standing gate, not a
   one-off proof.

Three more conditions, added at reconciliation (§9; all three from the adversarial
pass + the independent design, adopted):
7. **Conformance gate, BOTH doors** (218a d2-4a/d4-3): a declaration with `rc != 0` at
   an errexit-consumed site refuses BOTH doors — door-2 would mint a faithful-but-
   possibly-wrong abort (a wrong nonzero declaration injects a crash into a healthy
   book); door-4 would convert a crashing-on-converged book into a proceeding one (a
   behavior change beyond canary-removal). Non-conforming declarations are conformance
   DOCUMENTATION (and m-6 pedagogy: "your book cannot re-run converged"), never mints.
   This RETRACTS this note's §3 first-draft framing of door-4 as "the rescue" for
   hostile-rc tools — the rescue framing changed book behavior and is withdrawn.
8. **Already-guarded refusal, spelling-matched** (218a d4-6, sharpened by find-3): if
   the site's `||`-left sibling is check()-recognizable as this kind's probe SPELLING —
   matched syntactically, NOT by query-validity, because in a transformed multi-mutator
   artifact the later guards are st-3-invalidated and validity-matching would re-mint ⇒
   `guard || guard || mutator` stacking — the policy refuses (AlreadyGuarded). Without
   this the closure clause's "idempotent by construction" is FALSE outside
   single-transform books (find-3).
9. **The declared-rc fold fence** (find-2, adopted verbatim): a door-2 declared rc may
   satisfy ONLY the errexit-implicit consumer at the declaring site. It must NOT enter
   the fold's kill decisions (no sibling-branch deadness from a declared value — a
   rotted declaration must never under-execute a THIRD-PARTY command) and must NOT
   satisfy explicit readers (`$?`, `&&`/`||` operands, guards). First slice: the
   declared value lives inside `consumption_ok`'s check only, never in the site's
   `Observable` where `eval_and_or`/`eval_if` could consume it.

The composed precondition, stated ONCE so no builder has to assemble it from three
places: every mint-policy requires oracled ∧ declaration-exists ∧ conditions 1/4/5/6/7/8/9;
m-a ADDITIONALLY requires the site's probed-converged Effect verdict; m-b mints on any
declared kind's site regardless of probe verdict; m-c extends m-b to sites the probe
never covered. And the subtlest distinction in this whole design: probe-convergence is
door-2's correctness LICENSE but only door-4's m-a COST-GATE — the four-world trace
holds in every world regardless of the probe, which is exactly why door-4 is in the
surviving verify-eagerly family and door-2 is not. (Corollary, find-5 direction-2: the
`EstablishWritten` refusal in condition 1 is a DOOR-2 condition only; door-4 under
m-b/m-c legitimately reaches written-upstream sites — the guard re-measures live after
the upstream writes, which is exactly the four-world logic — so door-4's reachable
population is wider than door-2's on corpora with upstream-write poison, CONDITIONAL on
claim-noop being honestly declarable at all (hunt-A).)

**The four-world trace (the build's correctness argument, pre-stated):**
converged∧healthy ⇒ guard rc 0, LIVE provenance, mutator skipped — most of the win
(milliseconds vs apt's lock+resolver seconds) in-line and order-sacred ·
diverged-since-probe ⇒ guard fails, REAL mutator runs — kFAIL-perform BY CONSTRUCTION,
strictly better than static elision under TOCTOU drift (and [222 c-3]: this is what
puts door-4 in the surviving T-1 family — Terraform can only ERROR on divergence;
door-4 self-corrects) · converged∧env-sick ⇒ partial canary per §3's grading — the §2
trade, bounded · lying-check ⇒ under-execute, the PRE-EXISTING root trust every elision
already rests on, unwidened — with m-2 blame-routing and p-3 fleet-aggregation ("one
oracle's rot reads as one cause, not M incidents") as the attribution story.

**Sampled cross-check (m-5, c-9 — recorded, not first-slice):** where door-2 statically
elides, occasionally emit door-4's guard instead (apply-lane, no probe-purity issue);
disagreement = declaration-suspect, converting silent rot into early small signals.
Defer if it muddies plan-presentation determinism; the plan must disclose a sampled
site honestly.

## §5 The precedence seam (design)

One module (lean: `plan` crate, `policy.rs`), config-in-code, hot-swappable mid-round —
20V §8's seam obligation. Two layered decisions:

- **Layer A — admin-explicit, always wins, both directions — and it is REAL ENGINE
  WORK, not already-built semantics** (this note's first draft claimed the seam could
  just read HEAD's consumption marks; the adversarial crosscheck find-1 and the
  independent design's ps-2 independently refuted that): HEAD's `StatusRelaxable` is
  one undifferentiated mark written identically by all four consumer classes, and in a
  `set -e` book every command already carries it — an additional explicit reader is
  invisible to the union. The ladder needs a parallel per-node consumer-tag set
  (`StatusConsumer::{ErrexitImplicit, BranchOperand, DollarQ, IfGuard, LoopCond}`,
  218a ps-2), each existing mark-site adding one tag; doors 2/4 are eligible only when
  `status_consumers(site) ⊆ {ErrexitImplicit}`. Without this, `if apt-get install -y
  nginx; then …; fi` is a "bare" site to the engine and a transform flips the admin's
  branch — the find-1 disaster shape. An explicit `|| true` is door-3 (free relax); an
  explicit handler / `$?`-read / guard position marks the rc LIVE — doors 2/4 refuse,
  and no declaration overrides the admin's own sh.
- **Layer B — the bare middle:** `resolve(site) -> {Transform | StaticElide | Run}`
  parameterized over the THREE ownership models, ALL kept live in shape, naming, and
  tests (212; nothing may assume oracle-ownership):
  - oracle-default: the kind's declaration-existence decides (Ansible/Puppet's
    twenty-year stack — admin keyword > module declaration > engine default — is
    structurally this [222 §1/§7]; supports-but-does-not-settle dq-errexit-2, which
    the human holds open on kSILO grounds beyond prior art).
  - engine-global: one conservative global (the `Never` default IS this model's
    degenerate position; a future YOLO-mode global is its other pole, 207-adjacent).
  - admin-per-book: a book-level signal. Candidate sh-spellings, BOTH with named
    tensions, NEITHER settled (this is dq-errexit-2's territory): (a) an inert
    book-top assignment `DORC_DOORS=transform` — real sh, off-ramp-inert, but
    config-smell (20V §5 "smells like configuration-not-code") and a kOOB-redline
    judgment call; (b) CLI-side only (`dorc apply --doors=transform book.sh`) — no
    in-book spelling at all, cleanest redline story, but per-invocation not per-book,
    and it drifts toward engine-global in practice. The build implements the seam's
    THREE constructors with (b) as the admin-signal stand-in and leaves (a) unbuilt.
- **Breadcrumb disclosure (c-7's tolerated-WARN pattern):** when the flag is `Never`
  but declarations exist, the verdict lane notes per-site "transform available,
  flag off" — phased-enforcement calibration UX, and the dashboard's needs-declaration
  column gains its "available" sub-state for free.

## §6 Integration points at HEAD (for the implementing round; verify each, code moves)

- `crates/oracle`: parse `oracle_converged_run_<kind>[_<selector>]` alongside
  `oracle_probe_*`; body-shape validation (`return <n>` only) + `dq-door2-decl-shape`.
- `crates/plan`: `Disposition::Transform` variant; arm-ordering Omit > Replace >
  Transform > Run; `policy.rs` (the seam, §5); render emits the wrap-edit.
- Render mechanics (21E scar tissue applies): the transform is a PREFIX-INSERTION edit
  (guard text + ` || ` before the leaf's span, comment appended) — a new edit kind for
  `plan::render`'s region-grouping; the 21E adjacent-multi-line orphan class and the
  `comment_safe` quote-state machinery must be re-attacked with insertions in play
  (§7 attack-6). First slice may instead REPLACE the full leaf span with
  `guard || original-bytes` (replacement, not insertion — reuses the existing edit
  kind verbatim; ~SUSPECT the right first move).
- `crates/core/src/diag.rs`: all new codes catalog-registered (rq-1/2/3 discipline,
  21H precedent); per-code severity stays the round-22 retrofit's business (217 §6).
- `crates/coverage`: light the `guard-transform`/`static-declared` arms; add the §3
  rc-class split to needs-declaration ~when cheap; the door-3-vs-transform mark-union
  interaction (21B hunt-4's subtlety) gets a discriminator test.
- e2e: the env-sick four-world pole NEEDS the `EXIT_RC=` expected-nonzero-exit marker —
  task #12's harness work is a hard prerequisite for door-4's acceptance tests (the
  and-form precedent, 215). Sequencing dependency, recorded.
- `$DORC_VERDICT` lane: transform-site records (door, oracle, declaration hash, m-6
  counterfactual text) — shapes for p-3's fleet aggregation later.

## §7 Pre-registered hostile-pass attack list (transfers with the build; 212 mandates)

attack-1 four-world trace, each world driven end-to-end under mocks (env-sick via
EXIT_RC) · attack-2 boundary-3 audit: prove guard bytes ≡ the kind's vouched probe-body
bytes + silencing, NOTHING else, across every minted transform (the "novel apply-phase
action" line, 212) · attack-3 Never-arm: corpus-wide byte-identity, plus flag-on with
ZERO declarations loaded must also produce zero transforms · attack-4 correlated-failure
storyboard: one bad oracle → wrong plan AND wrong transform; grade the m-2 blame
template and p-3 aggregation against it · attack-5 composition cells (exclusion-check
discipline): transform×door-1-dead (must not mint), ×door-3-same-site (Replace wins),
×inlining (CALL sites never transform — body establishes aren't bare mutators),
×Members/loops (floored, pinned), ×door-2-on-same-kind (m-5's sampling seam) ·
attack-6 render: insertion-vs-21E region grouping, quote-state, CRLF, multi-edit lines,
heredoc-refusal poles · attack-7 re-analysis closure: transform → re-analyze → door-1
fold, NO guard-stacking, idempotent artifact · attack-8 disclosure floor: every minted
transform attributed (verdict lane + comment), every refusal catalogued — grade against
21B's honesty bar (Unattributed=0).

## §8 Exclusion-check + deliberately-not-designed + open forks (restated, all the human's)

Exclusion-check (4×2): reverse-propagation — re-analysis closure (§4) covers the
backward direction; the transform must also not confuse UPSTREAM st-3 validity (an
inserted guard is a Query, not a writer — gens nothing, ~SUSPECT verify) · other-phase —
door-4 is apply-only by construction; the probe NEVER ships transforms (probe compiles
from oracle bodies only; assert in attack-2) · other-user — the admin reads the
transform in the plan diff as sh they could have written (off-ramp holds); the lazy
admin gets §5's breadcrumbs, never silent behavior-change (flag default Never) ·
other-reliability — unreliable oracle IS the design's central case (four-world world-4,
m-2/m-3/p-3); unreliable DECLARATION rc handled by §3's trichotomy (an honestly-wrong
rc surfaces as a faithful crash, not an under-execute — the under-execute needs a lying
CHECK, pre-existing trust).

Deliberately NOT designed (build feedback required): the render edit-grammar's final
shape (replace-vs-insert, §6) · printing declaration bodies (deferred with p-1) · the
admin-per-book in-book spelling (§5's (a), dq-errexit-2's territory) · Members/loop
composition · m-5 sampling mechanics · the dashboard rc-class split's exact columns.

Open forks, unmoved by this note, cite-on-touch: dq-errexit-1 (canary the only
cost-species? — §3's partial-canary grading and run-evidence are LEDGER ENTRIES for it,
not answers) · dq-errexit-2 (bare-middle ownership; 222 §7 supports oracle-default,
the human is unconvinced on kSILO grounds — all three models stay live) · dq-errexit-3
(directionally ruled: flag, default-off, last; the line-crossed judgment stays his) ·
the declaration spelling's ratification (inline acceptable-debt sanctioned for spike
work only) · the door-4 mint-policy default beyond m-a.

## §9 Crosscheck reconciliation (the synthesis record; reconciled-by-source, fb-7)

This note was attacked by a neutral pass, a hostile pass, and synthesized against an
INDEPENDENT clean-context design (preserved verbatim as **218a** — an implementing
round reads BOTH notes; where they disagree and this section is silent, the
disagreement is OPEN). Dispositions:

**Adopted from 218a (the independent design), already folded above:** the
(provider, verb) declaration keying (§2) · the claim-noop/claim-rc split and hunt-A
(claim-noop is FALSE for the flagship apt oracle as naively written — installed-but-
outdated upgrades; DESIGN-GATING: if the canonical oracle cannot honestly declare,
the doors' reachable population collapses — 218a hunt-A is the implementing round's
first question) · both conformance gates (§4 cond-7) · the consumer-provenance tag
set (§5 Layer A) · the value-gate/trust-gate layering, the lift-gate diagnostics
inventory, the guard-preamble render shape, the `set -u` hazard, and the four-layer
zero-transform proof (218a d2-2/d4-2/ps-4 — not duplicated here; read 218a).
Convergent independently (confidence-raising): stand-in-never-prints (m-6 text to the
plan comment), the partial-canary grading (218a's world-3 narrowing).

**Adopted from the hostile pass (find-1..find-10):** find-1 → §5 Layer A rewritten
(consumer tags are real engine work; the if-guard transform-flips-the-branch disaster
shape is the motivating pole) · find-2 → §4 cond-9 (the declared-rc fold fence) ·
find-3 → §4 cond-8 (spelling-matched AlreadyGuarded; the closure claim is scoped:
idempotence holds GIVEN cond-8, not "by construction") · find-5 → §1/§3(b) (L58
re-characterized; door-4's m-b/m-c reach into EstablishWritten recorded as the §4
corollary; note 21B's Rung::NeedsDeclaration definition baked both errors — a
dashboard refinement for the implementing round) · find-9 → §2 stand-in fixed.

**Adopted as statements, not yet design (the implementing round owes these):**
- find-4: the arm-ordering is NOT fully welded — Replace-via-PROBE > Transform stays
  welded (probe-provenance beats deferral), but Replace-via-DECLARATION vs Transform
  is POLICY-KEYED in the §5 seam (218a's u-4 fork made mechanical: TOCTOU-immunity +
  partial-canary vs boundary-(3) purity — the human's, with dashboard numbers).
- find-6: the m-3 pin's honest coverage is AUTHORING-side rot only (oracle edited);
  HOST-side tool-version rot is uncovered in-spike at every mint policy — the
  product-era answers are the version-probe key (222 m-3's "where statable") and the
  m-4 author harness; and door-2 ships effectively SINGLE-fenced (the door-1-on-
  wrappers fence validates guard SHAPE analytically, never the declared rc VALUE).
  Stated so nobody believes the fences are stronger than they are.
- find-7: the declaration is a FACT claim; cond-7's "the declaration is the
  intent-answer" reading holds ONLY under the OracleDefault ownership model and is
  dq-errexit-2-CONTINGENT. Under AdminPerBook/EngineGlobal, consent comes from the
  seam, and an author documenting a hostile rc must not thereby auto-license
  transforms. If dq-errexit-2 lands away from oracle-default, the build needs a
  separate consent surface (a second declaration or the book/flag signal).
- find-8 omissions, appended to the deliberately-not-designed list: multi-host
  variance (per-host m-a verdicts ⇒ N divergent artifacts; plan presentation +
  p-3 fleet aggregation are single-host-designed) · oracle-side body-shape
  eligibility lint (pipeline probe bodies — firewall, pkgindex — fail guard-inlining;
  door-4's real population is narrower than oracled∧declared) · the kind-default vs
  per-selector resolution rule for declarations (the F-BLESSED analogue) · operand
  quoting in the guard text · door-2×Members surgery (members_disposition hardcodes
  status=⊤ — L58's class is unreachable without it) · `dorc bump`/partial-apply
  (kELISION) interaction with transforms.
- find-10: spell-env's "recognized env-name" is a pragma-in-sh-clothing risk under
  the kOOB redline — kept as candidate, posture-flagged.

**Independent-design items NOT adopted (left open in 218a):** its u-1..u-12 unsettled
list stands; its div-2 (door-2 also behind the flag) is this note's reading too but
flagged for the human; its EngineGlobal-default -GUESS for the spike's BareMiddleOwner
is plausible and unratified.
