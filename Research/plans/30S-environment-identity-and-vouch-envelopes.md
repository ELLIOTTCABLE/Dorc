# 30S — Environment identity: the retarget gap and the pin-or-sever envelope

> AI-authored (Fable; minted from the 2026-08-24 brainstorm ledger (session-local)'s
> sh-env sidequest, human-adjudicated in-chat; rulings marked `human-typed`).
> Ahistorical — rewrite in place. Scope: book-side environment identity ONLY; the
> sitting's sibling meta-orchestration topics stay in the 2026-08-24 brainstorm ledger
> (session-local), unbanked. This document is
> design-of-record for one correctness gap (bug-tier) and the ruled model that closes
> it — a pattern to carry into the stdlib and, later, hints/lints. It deliberately
> carries NO schedule: nothing here is owed-when; the §5 reds are the attention-calls.

## §1 — The gap (as-built, measured 2026-08-24 against `ai/r30-conduct`)

- **finding-prefix-stripped-at-dispatch** — the parser carries leading
  assignment-words faithfully (`syntax::ast` `Simple.assigns`, split at
  `parser.rs:901-916`) and the oracle-dispatch seam then reads only `words`
  (`analysis/src/value.rs:783-813` `site_argv`/`resolve_site_words`;
  `effect.rs:492-530` takes no env at all). `AWS_PROFILE=a aws …` and
  `AWS_PROFILE=b aws …` hand byte-identical argv to a family, mint one
  `core::FactKey` under `Context::HostDefault`, and SHARE fact state. Silent — no
  wall, no diagnostic.
- **finding-rho-fold-value-blind** — the `env`-utility spelling does reach the
  wrapper machinery (`oracle/src/entry.rs::peel_book_chain`) and can mint
  `Context::Wrapped`, but `RhoAccum::fold` composes the ContextKey from variable
  NAMES only: `env A=a …` and `env A=b …` share a key. This is `27K` §8's disclosed
  open debt — and `plans/24S` (the ρ-transforms bullet) had already named
  `$AWS_PROFILE` as "arguably the biggest coordinate-relevance risk in the whole
  context surface" and set the bar (`env VAR=x` is gold: value in argv). The intent
  existed; the carriage was never built. Only the ρ/env axis is name-blind — dimension
  shifts (`sudo -u alice` vs `-u bob`) already differ by resolved value.
- **finding-export-never-fences** — bare/`export` assignments are value-plane only
  (`value.rs:639-658`; the *scoping* is faithfully sh — bare persists, prefix
  evaporates): they never wall, never key facts, never fence transport.

The minimal reproducer, in the shape real multi-account conductor-books take:

```sh
export AWS_PROFILE=staging
aws_instance_exists web1 || aws ec2 run-instances --cli-input-json file://web1.json
export AWS_PROFILE=prod
aws_instance_exists web1 || aws ec2 run-instances --cli-input-json file://web1.json
```

Same bytes below each export; one shared cell; staging's measured "exists" can elide
prod's create — under-execution presenting as "your oracle said converged" while the
oracle told the truth about the wrong world (the mis-attributed cell of
`271:rul-sin-ordering`). Containment today is ACCIDENTAL: with zero stdlib, the
retarget-sensitive families are unmodeled and wall on their own. See §4.

## §2 — The model (ruled 2026-08-24)

- **rul-prefix-joins-site-identity** — a leading assignment-prefix participates in
  the site's fact identity, VALUE-carried: finish `24S`'s stated intent (the ρ-fold
  carries resolved values into the ContextKey) and route the bare prefix through the
  same peel/claim path the `env` spelling takes. Derived keying only — it blocks
  transport and never licenses separation (`272`'s never-derive-separation stands).
- **rul-export-is-an-index-fence** — an ambient exported-env mutation between sites
  is a fence in index-space: facts established above it never Must-transport below.
  The poison wall's dual, one plane over: unexplained world-mutation walls facts;
  unexplained index-mutation walls fact-transport.
- **rul-transport-wall-alone-is-banned** — the fence may never ship as the whole
  design. Under ⊤-env-read every binary is a potential reader of every variable, so a
  bare fence lets `export VERBOSE=true` poison every family below it, and
  un-poisoning by per-family teaching is at-most-claim economics (refused). The fence
  ships only together with:
- **model-pin-or-sever-composition** — a site below a book env-delta is probeable iff
  EVERY delta variable is witnessed at the verdict body, one of two ways: **pinned**
  (consumed faithfully — the probe replays the book's value for exactly that
  variable) or **severed** (provably unable to reach the check — `env -i`-class
  hygiene in the body's own sh, engine-recognized falsification-first, never a
  completeness gate). Any unwitnessed delta variable ⇒ withhold the probe ⇒ the site
  runs or guards, with a two-remedy hint ("sever ambient, or hoist the export").
  Guards stay right-world by construction (live, in-sequence, post-shift).
- **rul-positive-speech-only** — authors owe POSITIVE speech only: pins name what a
  body consumes (small, closed, knowable). The at-most complement — "and nothing else
  reaches me" — is performed by the CONSTRUCT (`env -i` computes it mechanically),
  never enumerated by a person. Open-world sensitivity lists are refused permanently:
  the one enumeration nobody can complete is done by the shell instead.
- **rul-engine-owns-shell-resolution-vars** [human-typed 2026-08-24] — variables with
  SHELL semantics (PATH, IFS, the ENV/BASH_ENV class — the ones no tool-author should
  ever reason about in a tool-specific way) are ENGINE-owned under the parity law.
- **rul-resolution-delta-invalidates-dispatch** — a book delta in that resolution
  class invalidates the site→family binding itself: the admin's line is no longer
  running the tool any oracle describes. Such deltas wall the sites below, body
  hygiene notwithstanding — this is about the admin's own line, not the probe.
- **rul-platform-describers-own-loader-locale** — the residual genuinely-open
  classes (dynamic-linker `LD_*`; libc locale `LC_*`/`LANG`) are authored
  platform-oracle speech, minted once by the few who care about ld.so and libc —
  never an engine denylist (`271:rul-net-quality-u-curve` governs the refusal).
- **rul-vouch-holds-under-witnessed-env** — the vouch-contract clause: a verdict
  body's answer stands under any environment its pins-and-severance witness admits.
  Pre-user, this edits the oracle-quality bars in place; no compat machinery.
- **rul-idiomatic-plus-offramp** [human-typed 2026-08-24] — `env -i` hygiene is
  mildly unidiomatic *because* it is over-defensive, and that is exactly Dorc's
  off-ramp flavour: not "idiomatic" but "idiomatic+" — sh patterns, extra-defensive,
  shareable. Expect raw `env -i` spellings as the common carrier, not helper-wrappers.
- **model-dead-store-export-elision** — an env assignment may elide exactly as
  dead-store elimination over the env plane: elidable iff no SURVIVING reader (any
  line that still runs or guards counts — a guard's check executes) sits between it
  and the next assignment of the same variable. Elisions license elisions; the
  straight-line induction extends to the env plane.
- **rul-env-delta-rides-provenance** — a fact measured under a replayed delta carries
  it in the why-chain ("measured under `{AWS_PROFILE=prod}`"). Without this, a body
  bent by book env mis-attributes to its author — the worst cell of the sin-ladder.
- **open-env-epoch-seat-unification** (deliberately unresolved; tc-tier) — whether
  ambient deltas ride the existing coordinate context slot (a book-scoped context
  entry) or the `26K` §0b scope/generation slot (host × epoch). The constraint that
  IS ruled: one slot family serves transits, local-exec scopes, and env-epochs —
  never three bespoke keyings. Settle at whichever sitting builds first.

## §3 — Considered and refused (recorded so nobody re-derives them)

- **Blanket replay** (probe under reconstructed book env, no witnesses): the pincer —
  a replayed delta can route vouched bodies to unvouched code (resolution class) or
  exceed the vouch's meaning-envelope (behavior class); faithful-but-unvouched versus
  vouched-but-unfaithful has no free middle. Witnesses (pins/severance) are the middle.
- **Bare transport-fence**: see rul-transport-wall-alone-is-banned.
- **Engine denylist of dangerous variables**: U-shaped net; ownership routes to the
  engine (sh semantics, by parity) or to platform describers (authored speech) instead.
- **Author-enumerated sensitivity sets** ("the vars I can't accept variability in"):
  open-world negative — never complete, never bothered-with; polarity flipped to
  positive pins plus the constructed complement.
- Noted, not a hazard: child processes cannot mint into the parent environment
  (process isolation); the apparent counterexamples (`eval "$(ssh-agent)"`,
  `export X=$(cmd)`, sourced env-files) all route through eval/capture/load
  constructs already priced ⊤.

## §4 — The one sequencing constraint (not a schedule)

**seq-stdlib-gates-on-env-identity** — value-carried context keying plus the
pin-or-sever floor are a prerequisite-or-rider of any stdlib arc shipping
retarget-sensitive families (the aws/kubectl/psql class). Today's safety is the empty
stdlib; shipping such families onto name-blind keys is how wrong-world elision goes
live in the field.

## §5 — Reds

Rust-native pins (via `internal_tooling::xfail`, horizon r31 as an attention-call):
`p-x-prefix-assignment-splits-fact-identity` ·
`p-x-env-wrapper-context-carries-values` ·
`p-x-ambient-export-fences-fact-transport` ·
`p-x-unwitnessed-env-delta-withholds-probe`.
E2e (XFAIL, target-tense goldens): `env30-book-exports-meet-oracle-envelopes` — one
book exercising, together: a severing (`env -i`-class) verdict body still eliding
below an unrelated `export` · a scrappy (witness-less) body withheld below the same
export · a prefix-retargeted same-argv pair yielding distinct outcomes · a PATH
export walling everything below it.

## Cross-refs

`plans/24S` (the ρ/context design this completes) · `notes/27K` §8 (the value-blind
debt, disclosed at landing) · `plans/271` rul-env-claim-inversion (the ladder — now
read body-side too) · `plans/27C` (context-entry; this is its ambient sibling) ·
the 2026-08-24 brainstorm ledger (session-local) (the minting sitting) ·
`ANALYZER-NEEDS:an-env-identity-carriage` (the
row) · `spike/CLAUDE.md` rho-claim-ladder (a steering entry lands when a build firms).
