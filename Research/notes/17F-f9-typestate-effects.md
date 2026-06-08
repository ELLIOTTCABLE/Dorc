# 17F — f9 gather: typestate + effect systems (round 17, 2026-06-07)

> Charter `170` front f9. Read full + graded: [B-aldrich-typestate-oriented-2009] (modern typestate; the
> Plaid language). Strom-Yemini 1986 (the origin) is a scanned/image-only PDF (276 chars/15pp — no
> extractable text, would need OCR); covered via Aldrich's restatement, NOT graded. Effects: inc-7 rests on
> the in-corpus [A-lucassen-gifford-effect-systems-popl-1988]; Koka treated lightly (kill-4, framing-only).
> This front re-splits kill-5 and grounds fw-2 hard.

## Findings (lifted)
- **f9-1 · kill-5 RE-SPLIT CONFIRMED — keep the state-model, kill the enforcement.** Typestate = "an
  abstraction of the operations currently available on an object, which may change as the program executes"
  — each state has its own permitted operations *and its own representation* (open-file has a filePtr;
  closed-file doesn't); transitions change state. Two separable halves:
  - **the STATE-MODEL (states + transitions + state-specific ops/data) = axis-fidelity → KEEP.** This is
    exactly Dorc's multi-state entity (`service` ∈ {installed, enabled, active}; nit-1 ≥enum) and the
    structured pole of `dq-entity-algebra`. +SURE.
  - **the ENFORCEMENT (compiler rejects operations invalid in the current state) = axis-depth → KILL.**
    Dorc never rejects (optimizer-not-checker). The whole *point* of typestate-checkers is rejection
    ("ensuring that clients only call functions appropriate in a given state"); that's the part we drop.
- **f9-2 · fw-2 GROUNDED HARD — typestate-enforcement needs the uniqueness Dorc can't collect (= SF-1).**
  Aldrich §3.3: sound static state-tracking requires *aliasing control* — the `unique` (linear, no-alias)
  permission is precisely what licenses tracking a transition; with aliasing you "have to consider the
  possibility that [a call] might [change the state]". And the pushbutton static route "requires good
  aliasing information to be successful, which is notoriously challenging" (Fink et al.). This *is* SF-1:
  strong-update needs proven uniqueness, and Dorc holds opaque tokens whose uniqueness it disclaimed.
  ⇒ typestate-the-enforcement and the strong-update keystone are **one problem** (the human's earlier
  point, now grounded in the typestate prior-art). +SURE.
- **f9-3 · BONUS (load-bearing) — the PROBE is typestate's shared-object fallback, prior-art-validated.**
  Typestate's own resolution of the aliasing wall maps 1:1 onto Dorc's: `unique` ⇒ track statically
  (= strong-update / ambient elision); `shared` ⇒ either a *state guarantee* (an invariant all clients
  respect) or **"dynamically testing the object's state before performing a sensitive operation"** — which
  *is* Dorc's probe. So when static uniqueness can't be proven, the typestate literature independently
  prescribes exactly Dorc's move (observe at runtime). The probe isn't a Dorc hack; it's the blessed
  fallback. +SURE.
- **f9-4 · inc-7 CONFIRMED + UNIFIED with the state-model.** A `(provider, verb) → {establish, kill}`
  effect-map (grounded by [A-lucassen-gifford-effect-systems-popl-1988], in-corpus) *is a typestate
  transition table*: a verb's effect = the state transition it causes (`systemctl enable` :
  `disabled → enabled`). So inc-7 (the minimal effect system) and f9-1's kept state-model are one structure
  viewed two ways. Take the minimal 2-element effect; the transition-table reading gives it the multi-state
  shape for free. +SURE.
- **f9-5 · kill-4 (Koka row-poly effects) — framing-only, machinery killed.** Row-polymorphic effect rows
  (Koka) are more apparatus than a fixed 2-element `{establish, kill}` map needs; mine the framing (effects
  as a per-operation annotation), kill the row-inference machinery (HM-family, welded `kVERIFY`).
  Not deep-read this round (a kill; the framing is established via the pointers-file + Lucassen-Gifford). ~SUSPECT.
- **f9-6 · BONUS → dq-entity-algebra structured pole.** Typestate states carry *state-specific
  representation* (open-file has a filePtr field absent in closed-file). ⇒ a Dorc entity-state can carry
  state-specific facts (e.g. `service:active` has a PID; `inactive` doesn't), supporting the structured
  (not flat) pole — bounded, not a mandate (don't over-engineer; the flat ≥enum floor still holds).

## Citations
> [B-aldrich-typestate-oriented-2009]:p1 (relevance: +1:SURE)
> "typestate—an abstraction of the operations currently available on an object, which may change as the
> program executes [Strom and Yemini 1986]. A familiar example is files that may be open or closed. In the
> open state, one may read or write to a file, or one may close it… In the closed state, the only permitted
> operation is to (re-)open the file."

> [B-aldrich-typestate-oriented-2009]:p2 (relevance: +1:SURE)
> "Recent advances in typestate focus on specifying the interface of a class in terms of states and
> ensuring that clients only call functions that are appropriate in a given state." … "the analysis
> requires good aliasing information to be successful, which is notoriously challenging."

> [B-aldrich-typestate-oriented-2009]:p4 (relevance: +1:SURE)
> "What if we have stored an alias to f somewhere … the call to computeBase might close the file and make
> the call to f.read() illegal. We rule out this possibility using a system of permissions … unique,
> indicating that there are no aliases to the object." (the SF-1/fw-2 requirement.)

> [B-aldrich-typestate-oriented-2009]:p5 (relevance: +1:SURE)
> "It is more difficult to track the state of a shared File … we treat the state associated with each
> shared permission as a state guarantee … Another approach for dealing with shared objects is dynamically
> testing the object's state before performing a sensitive operation." (= Dorc's probe.)

## Carry-forward
- Strom-Yemini 1986 origin: scanned PDF (no OCR available; not a show-stopper — Aldrich restates it). Pull
  a clean copy only if the map needs the original formal definition (-GUESS: it won't).
- **Gradual Typestate** (Wolff/Garcia/Tanter/Aldrich, ECOOP'11) + **Foundations of TOP** (Garcia et al.,
  TOPLAS'14) surfaced: typestate made *gradual/forgiving* + the formal aliasing/permissions result. The
  former is genuinely in our forgiving lineage (typestate that doesn't always reject) — worth a *note* for
  the map (typestate CAN be made forgiving), but the kill-orientation means I don't deep-dive enforcement
  variants. Flag for the map / possible f10 touch.
- effect-map ≡ typestate-transition-table (f9-4) should be stated as one unified mechanism in the map, not
  two rows.
