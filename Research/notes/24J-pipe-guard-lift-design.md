# 24J — the pipe-guard lift: fork resolved (connected probes; per-line is dead)

AI-authored (Fable conductor), 2026-07-05, round 24. The design record for the pipe-guard FIX —
the build that lifts `A | F || M` (`otelcol --version | grep -q "0.155.0" || curl … | tar xz`),
the r25 trial-book shape the r25 arc is explicitly fenced off of (r24 owns the fix). The PINS
and gap-anatomy are `24C` §pipe-guard (fixtures `strawman24-pipe-guard-*`; the XFAIL
`strawman24-pipe-guard-oracle-converged` is the promotion tripwire). Human-resolved fork,
recorded so the build is dispatchable from empty context.

> **[CORRECTION — 2026-07-11, block-settle task-5 audit; full record `plans/271` seventh
> round.]** This note's shipped-form choice — "the host runs the real `A | F`", raw book
> bytes licensed by shape-match against oracle arms — is CONFIRMED STANDING-LAW DEBT: it
> contradicts the round-20 structural-vouch ruling (probe-inertness comes only from a
> command inside its own oracle's body; no engine-side matching makes a probe safe), the
> round-23 correction (the probe ships the stripped predict invoked per-site with the
> site's argv), and inv-one-observable (predict models per-channel — the rc-shaped arm
> bodies this note routed around should instead have been made stream-faithful). The
> CONNECTED shape, per-command licensing, and the phantom-dissolution of a separate
> filter-blessing TYPE all stand; only the shipped FORM is superseded. Repair drafted
> (`271:rul-only-oracle-bytes-ship`, awaiting typed ratification): compose predict
> invocations — `otelcol__predict '--version' | grep__predict '-q' '0.155.0'` — with
> delegation bodies producing the authentic stream by execution; author redirect-to-null
> = per-channel decline ⇒ ⊤; the engine discards unconsumed channels at the invocation
> site. "Nothing needs reassembling" bought this debt; the mechanism section below stands
> as history.

> **[RATIFIED — 2026-07-16, `271:rul-only-oracle-bytes-ship`: the deferral above closed
> by explicit typed ack; task-14 DISSOLVED (a fresh in-context re-derivation, not a
> clean-room pass — the human clarified "triple-check" never meant one). The
> composed-predict repair stands RULED; build riders bind `270:block-rebuild`'s
> probe-emission touch-point: per-channel coverage (rc-only bodies never upstream of a
> byte-consumer) · stream-fidelity of substituted bodies on consumed channels ·
> capture-ships-real-bytes (same rule, not an exception); plus
> `271:rul-argv-flows-bytes-do-not` — book argv flows through the oracle author's own
> argparse; book bytes never ship. This note's landed shape remains NEVER-imitate.]**

## The question

The `||` consumes ONE value: the check-pipeline's exit status (grep's — last stage). If the
plan knows that rc, everything downstream already exists: the StatusRelaxable fold substitutes
it, the fallback dies as a dead branch, the line collapses exactly as `dpkg -s nginx ||
apt-get install -y nginx` collapses today (USER_STORY line-6: the admin's own guard IS the
liftable material). The whole fork was: how does the plan learn one pipeline's rc?

## e-3 (per-line / AND-OR-list-level vouch) — steelmanned, then DEAD (human, viscerally + argued)

Steelman: the admin's intent is line-shaped; one vouch covering it is attribution-clean; a
verdict-function is arbitrary sh so one `otelcol.is_converged()` could do the whole check
without modeling grep; the guard emitter already wraps lines. Why it dies:
- **The container is syntactic, not semantic** (the human's newline test): the same intent
  spells as `A | F || M`, as `if ! A | F; then M; fi`, as a continuation-split — a
  container-keyed license changes behavior under beautification, the fragility the design
  refuses everywhere else (AST-keyed sites; atomic-command axiom; format-free).
- **A pseudo-argv violates the contract**: verdict-functions are invoked with the site's REAL
  argv everywhere; "stage argvs concatenated" is an invented composite no command ever
  received — a new contract-surface with no sh-native meaning (the timid middle ground).
- **Cross-tool vouch blast inside one line**: a line-vouch by otelcol's author silently
  licenses skipping `curl|tar` — another tool's execution — where the design never lets a
  vouch reach past its own tool's site.
- Keying heuristics multiply ("first vouched tool in the list"; both sides oracled; `A||M1||M2`).

## e-1 (per-command, connected probes) — CHOSEN, after a phantom dissolved

The objection that mis-ranked it — "someone must own *what grep's rc means*, and nobody wants
a grep oracle" — was wrong in both halves (conductor error, human-caught):
- **The engine never interprets rc meaning, here or anywhere** (rc is opaque, welded). Every
  existing lift MEASURES rc via a shipped probe and REPRODUCES it into the fold. The pipeline
  needs exactly that: measure the pipeline's rc on the host, substitute it. No filter-semantics
  table, no "rc 0 = match" in the engine, ever.
- **grep IS stdlib material** (USER_STORY stage 1: "dpkg/apt, coreutils, and friends"). Its
  oracle vouches what every oracle vouches: purity (read-only, Query-class) — about the safest
  claim in the library. The ownership vacuum was asserted, not real.

The mechanism — the Stage-4 philosophy (when static can't evaluate, ship it and let the host):
1. **stdlib grep oracle** — purity/Query vouch only (trivial).
2. **Connected probes (the MEDIUM core):** the probe-compiler ships a check-pipeline as ONE
   connected probe when EVERY stage is vouched read-only (otelcol's `--version` arm via its own
   oracle; grep via the stdlib) — the host runs the real `A | F`, the rc reads back keyed to
   the governing site like any probe record. Narrow first: simple all-vouched `A | F [| F…]`
   chains; anything else ⊤s to the wall floor (today's behavior). The expected-version literal
   stays in grep's argv where the admin wrote it — nothing needs reassembling when the host
   executes the real thing.
3. **rule-query-validity relax (SMALL):** a vouched-read-only pipe-predecessor does not
   invalidate the downstream Query (the pins' classify unit-tests — opacity-blocks /
   Query-doesn't — guide it).
4. **Downstream: ZERO new machinery.** Known rc → existing StatusRelaxable fold → fallback
   dead-branch-omits → the existing query-guard substitution. No new license type; the
   elide-weld untouched (queries substitute on probe-provenance; vouch machinery unmoved).

Beautification-proof by construction: the if-form variant rides the existing
StatusRelaxable-on-if-guards path; restructuring the book leaves the AST showing the same
vouched-read-only stages feeding a consumed status.

## Honest residuals
- Compound-probe topology is genuinely new probe-compiler surface — start narrow (above);
  redirections/nesting ⊤.
- A pure-but-UNCOVERED stage (some sed invocation) blocks its chain until the stdlib covers
  it — coverage grows filter-by-filter; the ordinary gradual-enhancement curve, not a cliff.
- Stdin-dependent stages only probe CONNECTED (a lone grep site has no independent fact —
  silence-is-wall, correct).

## Status + sequencing
Task #19. Dispatch AFTER the in-flight polish-apply merges; parallelizable with the e2e
migration (`24I`). Landing announces itself: the XFAIL flips XPASS (diff by eye before
promoting — the promotion discipline). The two 24C §pipe-guard human-forks
(flag-pipe-status-unit / flag-filter-blessing) are RESOLVED by this note: per-command; and
filter-blessing was a phantom (purity-vouch only). Stage 6 no longer carries them.
