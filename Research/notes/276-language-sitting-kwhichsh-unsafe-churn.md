# 276 — the language-item sitting: kWHICHSH scope · the `unsafe` hatch · the churn policy

AI-scribed (design-rubber-duck sitting, 2026-07-12), human-adjudicated. The durable record of
the TODO-ADDTL item-1 sitting ("the Dorc language proper" — human-selected as the
final-Fable-window design dive), run in parallel with block-settle under an explicit carve:
this sitting owns the **superset boundary**, the **kWHICHSH carve**, the **`unsafe` hatch**,
and the **epoch/churn policy**, plus two fold-ins (**`trap`**, **KNOBS registration**). It
does NOT own the analyzer stopping-point (`271` task 9), the mark/entity grammar (`271`
tasks 1/12), or the reentry token (`271` task 6 — closed there; its outputs are inputs here).

Authority: root docs and human-TYPED rulings outrank this; entries without "(typed)" are
conductor-drafted and awaiting ack (silence ≠ ack). Naming per `270` §1.

---

## Closed this sitting

### rul-kwhichsh-registered · rul-kwhichsh-oracle-scoped  (typed 2026-07-12)
`kWHICHSH` now exists in KNOBS (it had been cited for two rounds with no entry): poles
`kWHICHSH-minimum-lcd ↔ kWHICHSH-maximum-gcd` (pole names human-edited in place), human-voice
source = DESIGN's *"POSIX" sh* section, dialect lean = r23-h3 (`notes/23F`: dash-ish,
"POSIX2024-ish", `local` carved in). The scope ruling: **the knob as leaned is
ORACLE-scoped; book-acceptance is a separate, open question.** Far pole banked verbatim-ish:
even non-sh books (fish et al.) are imaginable someday, provided the thing-under-analysis
calls unix binaries, has positional arguments, and can itself invoke/consume stripped-POSIX
oracles. The public *why* of oracle-side strictness is shareability, as worded in the KNOBS
entry.

### rul-unsafe-is-bare-sh  (typed 2026-07-12 — "I see no reason to grow warts and cruft for a second version of that"; closed)
The long-owed Rust-`unsafe`-equivalent escape hatch (STALENESS-AUDIT drift-language; `23O`
§5's "human-required hatch... in no doc") is **discharged by identification, not by
building**: the bare-real-shell-head escape of `271:rul-dorc-prefix-head-synthesis` IS the
hatch — an unmarked `sh` head walls its payload deliberately; analysis descends hint-lane
only and licenses nothing. No second construct will ever exist for this. Riding notes:
multi-line regions fall out with zero new syntax as `sh <<'EOF' … EOF` (already inside `24T`
pin1's stdin-shape scope; real sh; single-quoted delimiter, quote-hazard-free); and the
wished-for taint semantics (loses CFG-totality; subgraph permanently unskippable) is exactly
the standing unmodeled-wall machinery — there is nothing to build.

### rul-verdicts-never-stable  (typed 2026-07-12, emphatic) — the churn policy
Dorc does **not** promise cross-version verdict stability for `dorc plan`; the core mode
explicitly gets better without notice. The `# dorc-lang/vN` marker gates **language-warts
only** — it exists to keep the syntax parsable and agile through redesign — and is never a
promise to reproduce **semantic** warts to keep somebody's CI green. This discharges
`24O:dq-verdict-churn-policy` at design tier. The stability ledger, stated once:
**syntax = marker-gated · `__role` names = permanent (`24M`) · verdicts =
unstable-and-improving, disowned.**

Banked names (typed: "bank both, we'll roll with them"):
- **plan-as-API** — the named failure-mode: treating plan output as a cross-version-stable
  interface (canonical nightmare: admins CI-gating on plan shape).
- **verdict-pinning** — the named, *disowned* someday-feature ("someday we may offer
  `dorc <scaryfeature>`; we don't now"). Human sizing on record: genuinely useful, no
  visible route through the murk of ops; a Hard Problem "on the order of ~all of the rest
  of Dorc combined."
- Rider (conductor-drafted, un-nacked): `dorc plan --exit-code` (the `24R` §2d cheap-add)
  *inherits* verdict-churn — a smarter engine or one newly-installed oracle legitimately
  flips run→elide and changes the exit code — so its contract must be worded as gating
  **divergence-of-world**, never plan shape. The wording obligation travels with whoever
  builds the flag.

### The book-tolerance shape  (typed leans; the design itself stays tabled with zsh/bash)
- **lean-book-tolerance-binary** (typed): for artifact-modifying tiers there is no semantic
  middle-ground — either FULL dialect support (a real per-shell semantic model: rc/flow/
  expansion at errexit-door grade) or parse-warn-comment only. "Inject POSIX into the middle
  of a bash book and expect sanity" is rejected as a papered-over Hard Problem: license-
  bearing edits need dialect-accurate semantics of the *surrounding* code (`pipefail`
  pipeline-rc, `((0))`, `[[`, arrays-feeding-argv), not merely evaluator-compatibility of
  the inserted bytes.
- **direction-drift-report-tier** (conductor-drafted, awaiting ack): the one middle tier
  that survives the binary because it is license-free — parse + probe + *display only*: a
  drift-report plan that mutates no artifact and licenses nothing. Same pattern as the
  bare-head hint-lane row of `271:rul-dorc-prefix-head-synthesis`, applied at book scale;
  the `24R` gap-inhost-drift position needs nothing more.
- **lean-byte-honesty-is-the-priced-feature** (typed lean): POSIX books (and any future
  fully-supported dialect) keep the byte-honest plan — the on-screen bytes ARE the bytes
  that ship; never traded away. A future trampoline tier for foreign dialects would *pay*
  byte-honesty as the named cost of that shell choice (per-site trampoline ceremony cannot
  be honestly shown at full fidelity); the drift-report tier edits nothing, so the question
  never arises there. Two mechanisms accepted over one, on user-attention/honesty grounds.
- **banked — finding-guard-bytes-travel** (tabled): Dorc-inserted guard/oracle bytes execute
  under the *book's* shell. Mostly a zsh knife (word-splitting inversion), largely not a
  bash one (evaluator-compat holds for defensive POSIX bodies; bash's gap is
  license-analysis, above). Banked mitigation shape: the reentry trampoline
  (`( dorc-sh -c '…check…' ) || original-line`), which composes with `271` task 6's
  shim/binding machinery.
- **banked — strain-inverse-wall** (human): the sharpened wall "probes are book-cfg +
  oracle-bytes, never book-bytes + oracle-bytes" may want an inverse twin — oracle code
  never transports into book code; only synthesized cfg does. Test it whenever zsh/bash
  unparks.
- **zsh grading** (human): zsh-loss is not-a-cheap-loss. The parser half is bounded,
  LLM-responsive, differentially-testable work (two living binaries to test against); the
  full-support gate is the *semantic model*, never the grammar. Expect site-finding-grade
  zsh parsing to be cheap whenever wanted.
- **errand-render-kit-floor** (banked; human near-typed "almost want to try it"): a
  throwaway cost-sizing spike for a build round — one implementor, brief: "figure out how
  to support fish/zsh/ksh/csh site-finding + connective-render all at once with minimal
  effort; give up and throw your work away if the going gets rough." Natural slot: after
  block-rebuild's render churn settles. Sizes the trampoline tier without designing it.
- Prior-art note for any of the above: Vendor/ already carries mvdan-sh (interleaved
  POSIX+bash+mksh grammar) and tree-sitter-bash (error-tolerant by construction); an
  error-tolerant parser posture is also the substrate any future LSP needs (TODO.md:8–11).

---

## The base-dialect resolution — the floor weld (2026-07-12, second half of the sitting)

### The GCD research (evidence base)
One research errand: `.claude/research/kwhichsh-gcd/turn01-2026-07-12-report.md` (committed
`c1bf0f1`; graded sources inline; conductor spot-checked the load-bearing claims against
independent knowledge — citations not re-read, so the matrix is subagent-tier until someone
re-walks the A-graded sources). Headline: **for oracles, the two kWHICHSH poles nearly
collapse** — the consumer-paste requirement (a stripped oracle must survive being pasted
into a bash or zsh user's own file) forbids exactly the richness that admitting those
shells would notionally grant, so the delta above "dash + `local`" is *subtractive
discipline, not a feature set*. POSIX Issue 8 pin-downs: `local` genuinely absent (Austin
defect 767 stalled on scoping); `pipefail` genuinely standardized; so r23-h3's "carves out
`local`" parenthetical was exactly right, and its "POSIX2024-ish" gently over-generous
(pipefail is Issue-8 yet cannot ride bare in paste-floor bytes — see the pipefail thread).

### rul-base-dialect-ruling-list  (HUMAN full-ack, typed 2026-07-12 — his framing: "it
turns out our target is *write good portable shell*, which is what I wanted it to turn out
as all along")
Care-set rulings:
- ksh93 **OUT** (it has no `local` at all — `typeset` scopes only in the `function f{}`
  form — so ksh93-membership and the `local` keystone are literally one decision).
- zsh **IN via discipline** (macOS login shell, can't drop; quote-as-law covers the
  word-split inversion; honest residual = zsh NOMATCH glob-abort → one quality-bar line:
  avoid bare globs in oracle bodies).
- mksh **free-rider** (has `local`; not targeted; revisit only if an Android target
  surfaces).
- posh/yash: **CI differential-test targets, not members**.

Dialect rulings:
- `local` keystone reaffirmed (r23-h3) — the dialect is "POSIX + `local`".
- `local x=$(cmd)` **permitted**; analyzer treats it rc-opaque and hints under `set -e`
  (the SC2155 masking is real in every care-set shell; the only portable fix is
  declare-then-assign — `local -r` is itself a bashism).
- printf-doctrine: never `echo` with flags/escapes.
- quote-as-law: quoting is law, not style — the one rule that makes the dialect survive
  the consumer cohort.
- `f()` only; `function f{}` rejected.
- Ban the bash-family constructs (`${x/…}` `${x^^}` `${x:off:len}` `[[ ]]` `==` `<<<`
  `&>` `|&`) — the exact set a bash-habituated author reaches for.
- `test -a/-o`: accept-run / **emit-never** (POSIX-2024 removed them; Debian Policy still
  mandates shells support them — the contradiction resolves as: they run everywhere, we
  and the stdlib never write them).
- `$'…'` **OUT-for-now** (conductor push-back, acked in the blanket: the only
  permit-candidate resting on an unverified version floor; `printf` covers the use-cases;
  cheap to admit later, expensive to retract).

### The pipefail thread  (human: "very nearly a hard-requirement-floor… find a way that
isn't 'give up on pipefail'" — resolved without surrender by splitting lanes)
**rul-pipefail-four-lanes** (typed strong-ack, 2026-07-12):
- **-dialect: IN.** `set -o pipefail` is legal dorc-lang (POSIX-2024), and the analyzer
  models pipeline-rc first-class — it must regardless, because pipeline rc is
  VERDICT-load-bearing: without pipefail, a masked upstream crash in `foo | grep -q x`
  mints a clean licensing 0/1 (against the spirit of `271:rul-zero-one-inversion-pair`);
  with it, the SIGPIPE wart (early-exit consumer → rc 141) lands in the flat ≥2 sink ⇒
  can't-say ⇒ run. The failure directions: off ⇒ wrong verdicts (unsafe); on ⇒ lost
  elisions (safe). The verdict lane *wants* pipefail. Quality-bar rider: prefer full-read
  forms (`grep x >/dev/null`) over early-exit `-q` where the producer minds SIGPIPE.
- **-support-envelope (rul-envelope-no-pipefail-less-executors).** Typed correction folded
  here: the EXECUTOR story is **unruled** — no shipped-dash promise exists; the likely
  at-least-short-term posture is floating/yolo'd (the admin's book runs on the host's own
  shell; "ship a bare base executor" and "mandate an executor install" are unevaluated
  options, not spike material). The carve, typed: **non-pipefail executors are an
  explicitly unsupported class** — carved now precisely so no obligation to support
  {no shipped executor} ∧ {pipefail-less ancient host sh} can ever accrete.
- **-guard-handshake.** Apply-lane availability is a per-host *handshake fact*, never a
  version database: a session-start known-answer probe of the host's sh (the
  candidate-evaluator-handshake shape, `271` thread-delegation-head candidate-d). Absent ⇒
  the affected check body is unshippable there ⇒ guard declines ⇒ site runs (existing
  fail-toward-run law; no silent semantic downgrade, ever). Bonus, kWARN-rich: the same
  handshake powers a pre-flight book warning ("your own `set -o pipefail` will crash host
  Y's ancient ash") that no incumbent gives. **Handshake-scope bank (human nit, typed):**
  the handshake is plausibly NEITHER plan-time NOR apply-time but *both* — a small class of
  correctness-testing candidates may belong in *every* communication channel; wants its own
  much-later design round, probably riding the 26\* resumption.
- **-strip-idiom.** The blessed paste/stripped spelling is the self-gating
  `(set -o pipefail 2>/dev/null) && set -o pipefail` — pure floor-safe bytes; literally
  check-then-act, the founding shape the product lifts; annotation-by-idiom the analyzer
  recognizes as a *conditional* pipefail-active fact; errexit-safe (the left of `&&` is
  `set -e`-exempt — the classic `false && x` non-exit, checked). On ancient shells it
  degrades to the consumer's own ambient laxness — the no-worse-than-bare floor, verbatim.
  Forward-looking: pipefail is now POSIX; the incompatible tail only ages out; the gate
  decays into a harmless two-line fossil while the practice it spreads stays.
- The `--exit-code` wording rider (restated from the churn section) rode this thread to an
  ack-in-passing ("we're basically in accordance on all of this"): the flag's contract
  gates **divergence-of-world**, never plan shape.

### rul-spec-two-binary-floor  (typed 2026-07-12 — "It's a solid floor, and I like it";
kWHICHSH **WELDED** on it)
No language spec is written, mid-spike or ever for this tier. The specification is one
sentence, executable:

> **A valid dorc-lang v0.1 base-dialect text is a stripped file that parses and runs
> identically under `posh <vP>` and `dash <vD>`; where the two disagree, the construct is
> outside the dialect.**

Properties, each load-bearing: agent-runnable with zero spec-reading (two downloadable
binaries as the oracle) · strip-then-run-under-both IS the executable off-ramp test
(F-OFFRAMP as a command, at last) · differential-agreement-as-spec = kVERIFY-calibrate
applied to the language itself, and DESIGN's own dash-differential posture generalized ·
two shells' agreement cancels most single-shell bug-for-bug warts; genuine disagreement
resolves to "undefined — avoid," the correct answer for a portability dialect.
Inheritance: **Debian Policy §10.4 is a 25-year, institutionally-maintained definition of
"good portable shell with `local`"** — policy text as the citable human-readable spec,
`checkbashisms` as a free linter, posh as the enforcement binary, an ecosystem of
maintainer-scripts as proof the floor is livable. posh's Debian-mandated over-acceptance
(`test -a/-o`, `echo -n`) is covered by the lint-tier emit-nevers; posh's pipefail-lack
mechanically *enforces* the gate spelling (bare `set -o pipefail` fails the floor test;
the gate idiom passes).

Rider ledger (small, none spec-shaped): lint-tier emit-nevers (posh can't catch them) ·
pipefail semantics live ABOVE the floor by construction (analyzer model + handshake +
envelope carve — the sentence defines floor *text*, not the semantics ceiling) · zsh-paste
discipline items sit outside any binary (quality-bar) · **version pins RULED (conductor, 2026-07-12, under
delegated authority — "decide for us"): `posh 0.14.1` ∩ `dash 0.5.12`.** Real-binary
empirics (turn02 report, committed `cd777a5`: dash built from the official source
tarballs, posh extracted from official Debian `.deb`s, no installs; all 12 battery checks
landed exactly as expected on all four candidate binaries — the local-assign keystone
passes posh; the PE/`[[`/`function` rejections all hold; the gate idiom is errexit-safe on
both). Two corrections the empirics forced: **pipefail enters dash at 0.5.13, not 0.5.12**
(proved by `options.h` diff + built-binary behavior; falsifies the turn01 care-set note,
and Debian's patched 0.5.12 carries no backport) ⇒ newest-lacking = 0.5.12; and
current-Debian-stable posh = **0.14.1** (0.14.5 is testing-only). Pleasing coherence: the
floor pin (0.5.12, the last pre-pipefail dash) and DESIGN's executor lean (dash ≥ 0.5.13,
the first with pipefail) sit exactly astride the pipefail notch — "pipefail lives above
the floor" made literal in version numbers. dash-pin-tension, recorded for free veto:
0.5.11.5 is battery-identical on all 12 checks, so dropping older costs nothing if
"older is gently better" should outrank "newest-lacking"; 0.5.12 chosen as
principle-faithful, with the side-benefit that it matches the bookworm-era system dash
most widely deployed. **fence-rejection-rc** (turn02 observation, promoted to a fence):
the spec sentence's "parses and runs identically" is scoped to ACCEPTED constructs —
dash exits 2 where posh exits 1 on *rejected* ones, so no dialect rule may ever depend on
the exit code or error text of a rejected construct.

**The weld:** kWHICHSH → **welded to `kWHICHSH-minimum-lcd`** as this executable
two-binary floor (human, typed 2026-07-12: "annotate kWHICHSH as fully welded. It's a
solid floor, and I like it."). The scope carve stands intact: the weld binds ORACLE/marked
dialect text only; *book*-acceptance remains a separate open question (value-ladder,
tabled). Root `KNOBS.md` annotated same date, left uncommitted per the carve-out fence.
With this, the sitting's "superset-boundary statement" primary question is **closed in
shape**: dorc-lang v0.1 = *the posh∩dash floor (this weld) + the `271`-accreted authored
additions (marker · binds · trailing marks incl. `#selector`/`:?` · the `dorc:sh`/
`dorc-sh` reentry pair · `__role` name-semantics) + the stability ledger
(rul-verdicts-never-stable)*. Assembling that into one reference page is scribe-work,
unscheduled, cheap at any time.

---

## Still open in this sitting's carve

- **trap** — v1 disposition gentle-ACKED (typed 2026-07-12, the task-12 checkpoint
  sweep: "unmodeled trap is probably fine for now ... it annoys me but there's too
  much else to do"): t2 stands — trap is recognized-but-unmodeled, registration
  walls loudly with a named hint; t1 (the pin-what-tip-does fixture-errand) rides
  block-rebuild as a conductor-tier rider; deeper modeling stays a language-round
  item. (Original entry: human-DEFERRED same date; fold-in; genuinely undesigned —
  `plans/064`:13's one-liner is the entire corpus
  treatment; `set -e` earned round 20V, trap never did). Proposed close (conductor, cheap):
  (t1) fixture-errand — pin what tip actually does on a trap-registering book
  (silently-ordinary-command would be a soundness bug; wall is fine); (t2) v1 disposition —
  trap is recognized-but-unmodeled: registration walls loudly with a named hint; deeper
  modeling (handler body as implicit may-run edges at every subsequent point) banked as a
  language-round item. Softener, unverified: elision's StandIn rc-reproduction means
  trap-ERR firing behavior is ~preserved under elision, so the deep interaction is narrower
  than it first looks.
- **Awaiting-ack ledger:** direction-drift-report-tier (human-DEFERRED 2026-07-12,
  explicitly neither ack nor nack — value/effort unsure). The `--exit-code` wording rider
  graduated to acked-in-passing (see the pipefail thread). (The KNOBS entry itself was
  human-edited in place — counts as acked.)

## Pointers
KNOBS `kWHICHSH` (registered → WELDED 2026-07-12; human-edited pole names) · TODO-ADDTL
item-1 (this note is that sitting's durable record) · `plans/271` (the adjacent
block-settle ledger; carve respected) · `notes/24O` item-7 (churn — discharged at design
tier here) · `notes/23O` §5 language item (`unsafe` hatch discharged; superset statement
closed-in-shape via the floor weld) · `.claude/research/kwhichsh-gcd/` (turn01 GCD report,
`c1bf0f1`; turn02 version pins + battery matrix + SHA256s, `cd777a5`).
