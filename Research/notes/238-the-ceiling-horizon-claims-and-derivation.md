# 238 — the ceiling: horizon-bounded claims, derived footprints, and what stays dead

AI-authored stamp of the 2026-07-02 "how high can we fly" dialogue (human-participated and
human-acked for stamping; his corrections are load-bearing throughout). Status: SEED material for
the vouch-ceiling design round, which is PARKED — nothing here is committed; the whole frame is
~SUSPECT pending its own adversarial pass when that round un-parks. Confidence-marked. Trust
stamped `plans/233` + `notes/237` over this where they conflict.

## §1 The five claims (never blur them again)

"Elision in a book containing `apt-get install x`" is five different claims:

1. **Elide the apt site itself when converged** — alive, settled (its own oracle's probe; the
   payload doesn't execute).
2. **Elide other commands downstream of an apt site that itself elided** — alive, testimony-free
   (an elided command can't disturb anything; the fully-converged book elides entirely, apt lines
   included). The common re-run case.
3. **Elide downstream of a RETAINED apt, licensed by observation** — alive (guards; hoisted
   re-verification per the placement-spectrum round). Zero payload-knowledge needed: we look,
   we don't predict.
4. **Elide downstream of a retained apt, licensed by an AUTHORED-STATIC vouch** — DEAD,
   permanently, and — the human's leveling — dead for EVERY command, not just package-managers:
   any command hides a potential ocean of global state (`cp` can trip inotify handlers, FUSE,
   quotas, auditd). An un-horizoned authored universal ("running this disturbs nothing else")
   quantifies over third-party, per-version, per-host code the author cannot attend to — for apt
   the payload is chosen at run time by the archive; effort is spent at authoring time; no
   diligence reaches bytes that don't exist yet. Class-scoped framings of this death
   ("package-managers are special") are themselves the danger: they imply `cp` is safe.
5. **Elide downstream of a retained apt, licensed by a DERIVED, PER-RUN footprint** — alive at
   the ceiling (this note's subject). The crosscheck's "at any effort level, ever" wrongly
   lumped this with claim-4 (corrected in `237`).

## §2 The payload inventory (why claim-4 dies, concretely)

`apt-get install x` = dependency-closure resolution (archive+host-dependent) + per-package file
unpacking + maintainer scripts (arbitrary root shell, per version, third-party) + triggers
(deferred actions owned by OTHER installed packages — host-dependent) + apt hooks (host config) +
dpkg bookkeeping. The authored vouch fails per-channel: the maintainer script that will run may
not exist at authoring time; trigger effects belong to other packages on a host the author never
saw; the closure itself moves. This inventory is also the derivation target for claim-5.

## §3 The uniform sidestep (the human's toolkit, applied at the top level)

Three pieces, mechanism-uniform across every command (his hard requirement: "just as good a
solution for `cp` as for `apt-get`" — no class special-casing; no free lunch):

- **Horizon-bounded claims.** ALL effect-claims (establishes, footprints, vouches) cover
  first-order tool-contract effects ONLY, by one global contract. Host-configured *reactions*
  (watchers, others' triggers, hooks) are a named residue class no per-command claim covers —
  the same move as the hermeticity weld (name the exclusion as a precondition; don't model it).
  The horizon is what makes ANY authored claim honest-by-construction; without it, every oracle
  ever written was already signing the impossible sentence — cp's ocean is merely quieter.
- **Derivation as a gradient, one mechanism.** A claim's contents ("touches THESE") may be
  filled at authoring time (the tool contract binds effects to argv: cp — the engine's existing
  constant-propagation does the filling), at probe time (a read-only authored RECIPE computes the
  set: apt's `install -s` closure → payload file-lists → host trigger/hook registry →
  stereotyped-maintainer-script recognition, ⊤ on any unrecognized residue), or never (⊤ ⇒ wall
  or guard). One claim-language; effort/depth varies; laziness is priced by ⊤, not punished.
  New machinery implied: probes returning STRUCTURED SETS into the fact-plane, not just verdicts.
- **The residue handled once, globally.** Reaction machinery is itself probe-able host-state
  (registries exist); host-level oracles can declare it quiet or wall specific kinds — or the
  residue is ONE disclosed exclusion. Never per-command testimony.

**The composed boundary (human's framing, keep it):** the end-user receives a single set-union
boundary around Dorc's bounded promises + every loaded oracle's bounded promises, with an honest
"outside this line you are exactly where bare sh always left you" (his gloss: GFY — Go Figure
it out Yourself). Allocation principle for what lands outside: precisely the work it would be
extra-wasteful for anyone upstream to do (host-specific reactions; the weird package only this
admin uses) — which is why the residue landing admin-side doesn't violate the be-lazy contract.

**Honesty check (not a rug):** the horizon has a real wrong-elision corner — a downstream fact
disturbed by a reaction no first-order footprint covers. Accepted on the same grounds as the
existing tolerated exposures: identical blindness to a hand-written guard; same class as
probe→apply drift; and the residue machinery is admin-installed state (two-users doctrine puts
the disclosure on the right desk). It must be named loudly, once, like the hermeticity
precondition was.

## §4 Tabled: oracle-pins-into-artifact (metaprogramming)

The strong form of claim-5 had the oracle REWRITING the book line (`apt-get install x` →
`… x=1.24-1 libfoo=2.1-3`) to weld the analyzed bytes to the executed bytes. TABLED by the human
as too big a step: oracles-dictating-transformations-of-runbooks is metaprogramming — huge open
questions (how is a caller-transformation even spelled; what survives of your-runbook-is-the-bytes
trust). Also the third instance of the conductor's noted firewall-breaking tendency (memory'd).
**The non-breaching cousin stands instead:** the derived footprint is CONDITIONED on the
simulated package-set; at apply the guard/wave re-simulates and compares; match ⇒ elisions
stand, mismatch ⇒ footprint lapses, downstream degrades to verify, flagged
(proceed-and-flag). Nothing edits the user's bytes; drift costs elisions instead of being
pinned away. Weaker guarantee, zero metaprogramming.

## §5 What the mountain actually teaches

The claim-5 oracle at full depth is NOT a near-term artifact: much of it isn't even expressible
as oracle-sh (the stereotype recognizer, structured-derivation plumbing, payload fetching are
Dorc-side toolkit/analyzer surface). Its real near-term yield: (1) a requirements catalog for the
oracle-facing toolkit (the DX long-tail DESIGN promised); (2) proof the elide-goal is reachable
even for the package-manager class, so the architecture must not foreclose it. (Phrasing note,
same day: "ceiling, not critical path" is SUPERSEDED by the human's two-halves doctrine — full
elision is THE goal, never aspirational-tier; the guard-half earns equal attention as
sooner-value and permanent fallback; authoritative statement in `239` §1.) Imagining the whole thing in pure sh ("hah!") is the right
deflationary exercise.

## §6 The demo at the ceiling (kept for morale; not a plan)

Scene 1 (daily): converged fleet, sub-second plan, every line greyed — claim-2, no payload
knowledge. Scene 2 (the money shot): a diverged apt line shows its derived, version-pinned
footprint ("will touch: 214 paths, unit nginx.service, trigger man-db") and the other ninety
lines stay greyed EXCEPT the two that read a touched path — "it read the .deb before installing
it and proved the rest of my runbook didn't care." Scene 3 (the honesty shot that makes scene 2
credible): the cursed vendor installer stays un-annotated — `# unmodeled — runs as written` —
with visible verify-guards after it. The tool visibly doesn't pretend.

## §7 The carried sentence

The doc-rule replacing "package-managers can never vouch": **no claim without a horizon; no
closure authored where it can only be derived.** A rule an author can follow, a linter can
shape-check, and a gradient can price.
