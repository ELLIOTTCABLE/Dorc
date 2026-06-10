# 207 — The errexit incentive-tension (human reflection) + the inline-oracle direction

> Round-20, mid-round design conversation following the C-3/fork-mutator-rc cost arithmetic
> (206 §2). §1 is the human thinking out loud — NOT a ruling; recorded because it names a
> tension with no other durable home. §2–§4 are the orchestrator's assessment, delivered in
> conversation. Nothing here was built.

## §1 The human's reflection (2026-06-10, near-verbatim, marked not-a-ruling)

On the C-3 cost: "I *think* pursuing correctness here is probably the best route; but it's
also obviously costly — this belongs in an entire category of 'pursuing soundness is pushing
us down a user-unfriendly path' concerns I have, and probably belongs filed under 'YOLO
mode'." The two tensions, his words condensed:
- `errexit` is a perfect shell-spelling of user intent ("be total. be complete. be strict. it
  *should* do *what the user asked*") — but honoring it may mean "most scripts doing
  just-about-nothing under Dorc until you write oracles for the entire world."
- The opposite spelling also exists and is also legitimate intent (`set +e` plus `|| exit 1`
  on chosen lines = "I want *this* bit to depend on rc, not *all* of it") — "except that by
  saying that, we've just graduated into a tool that *encourages bad user-behaviour* …
  if you ever offboard, all your scripts are now *less by-default-safe* … that conflicts with
  an unstated but fundamental design-goal of all good software which is 'be a good fucking
  person.'"
- Solution-space he floated, explicitly unsettled: YOLO mode ("targeted unsoundness into
  controls where it's really annoying (`set -e` is definitely one), with the intent of
  helping the user onboard"); or more non-sh annotation ("I hate this in *different* ways");
  or — "maybe I'm just describing 'we need a way to write an inline oracle for extremely
  short/punned circumstances' … something like a way to *write `--dry-run` inline* into a
  random command nobody's written an oracle for … something that expands inline into a full,
  minimal oracle."

## §2 The incentive finding, sharpened (orchestrator; +SURE, traced against the gate)

The as-built incentive is WORSE than the reflection sketched. Mutator-elision requires the
conjunction: Converged ∧ ambient ∧ status-DEAD ∧ stdout/stderr-unconsumed-or-devnull. Under
fork-mutator-rc, ANY status reader blocks (⊤ rc): `set -e` (errexit-consumed), `&& / ||`
(including `|| exit 1` — so the human's "selective strictness" spelling does NOT unlock
elision either), `if`-guards, `$?`. The only book shape Dorc currently rewards with
mutator-elision is the *naked* mutator: `set +e` context, bare line, no error handling at
all, output discarded. The elision-incentive gradient points at maximal sloppiness.
Counterweight (~SUSPECT): lazy admins don't restructure scripts to chase a tool's elision
stats unless the UX rubs their nose in it — which the planned why-elided/why-probed per-line
disclosure (dir-soundiness-ux) would do. The disclosure surface and the incentive surface are
the same surface.

## §3 Naming + filing (orchestrator assessment)

- This is `kSILO`'s sibling, possibly a sub-case: kSILO tracks correctness-code *migrating*
  book→oracle; this tracks correctness-POSTURE being *suppressed* (books shedding `set -e`/
  guards to please the optimizer), with the same off-ramp-degradation endpoint ("the world's
  published shell gets less defensive" — kSILO's own text). Whether it folds into kSILO or
  wants its own entry (working name: the anti-defensive incentive) is the human's call per
  KNOBS protocol; flagged in-conversation.
- YOLO mode, precisely framed: it is the OLD conformance assumption ("converged ⇒ rc 0,"
  removed by C-3/19D as a silent default) re-admitted as an EXPLICIT, opt-in, per-run mode
  with per-line disclosure — bounded to apply-side (probe-side never; kFAIL-withhold is not
  modal), blast-radius = the known suppressed-abort/over-execute class. In KNOBS vocabulary
  that's a `mode`-status resolution (ceded to the user), and it converts the unsoundness into
  a teaching surface ("assumed rc-0 here; a strict run would have executed this") — which
  serves the frontload-the-unsoundness ruling rather than fighting it.

## §4 The inline-oracle observation (orchestrator; the promising one)

The heavyweight form ALREADY exists by design: DESIGN's Contract & DX mandates oracle-flavour
and book-flavour code "intermixable in the same file" — a `<provider>__check()` defined in
the book IS an inline oracle; zero new mechanism. What's genuinely missing is the
*punned/per-site* form, and there is a candidate spelling that stays 100% sh (--WONDER,
unexplored): **user-defined wrapper functions as providers-of-one**. Strawman:

    reload_nginx()        { systemctl reload nginx ;}
    reload_nginx__check() { systemctl reload nginx --dry-run >/dev/null ;}
    ...
    reload_nginx

The pun rides the existing command-keyed convention (the function name is the provider); the
human's "write `--dry-run` inline" example is exactly the check body; no kTYANNOT/kOOB cost,
no comment-parsing, no new syntax — and offboarding leaves a script that is BETTER-organized
than before (named functions), the be-a-good-person direction. Engineering cost: the engine
must treat book-defined functions as command-families — i.e. seam-interproc-lite (call-edges
for trivial wrapper fns), the seam 191 already names. Recorded as a direction worth a future
round's strawman; NOT built, NOT a commitment.
