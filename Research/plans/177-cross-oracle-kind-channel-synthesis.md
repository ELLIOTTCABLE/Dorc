# 177 — The cross-oracle kind-channel: synthesis + lean (rounds 171 + 172)

> **Status (round 17, 2026-06-07): the single K1 plans-synthesis.** Supersedes `plans/173` (the
> packaging-only map, now removed); integrates the raw gathers `notes/171` (packaging prior-art) and
> `notes/172` (adjacent-field prior-art). A **map with a lean**, not a decision — the human takes the lean
> into adversarial-crosscheck. A *third*, code-only round (`notes/173`, shell-spellings) is the last-ditch
> test of this synthesis's central negative; fold it back here when done. AI-generated; confidence-marked
> (+SURE/~SUSPECT/-GUESS/--WONDER); trust repo-root `DESIGN`/`KNOBS` over this.

## 0. The question
How do two oracles authored **independently** (A, B) converge on one **opaque kind-handle + its state
sub-data** so a fact B *probes* discharges a precondition A *declares* — with Dorc routing it and
understanding **none** of the semantics, and **nothing hardcoded**? Concretely: A = "non-mutative *provided
your wombat is defrocked*"; B = "I can check defrocked / frocked / wet." The floor to beat is the inert
`# dorc: kind=…` comment.

## 1. Inherited frame (true going in; do not relitigate)
A0 referent-agnostic by default · A1 the shape is **3-place** `(kind, provider, verb) → effect`, bound to a
**named kind** not a shared token · A2 the `kOOB` in-band lean (a lifted analyzer-index, not sidecar config)
· **X3** a 1-place sh *function-name* namespace cannot carry the 3-place relation (apt's then brew's oracle
*clobber*) — `notes/151` · the relational adjudication (`095`: Dorc keeps relational contracts over
meaningless symbols; grounding is the human's) · the **≥1-anchor floor** (`094 g4/g5`: co-reference ≠
grounding; something must be declared).

## 2. Result A — the shape is universal (+SURE; from `notes/171`)
`(kind, provider, verb)` is the **mainstream convergent design**, re-invented everywhere: Debian
virtual-packages [A-debian-policy-relationships-2024], `update-alternatives` slots
[B-update-alternatives-man-2024], RPM `Provides`, Ansible's generic `package:` over apt/yum
[B-ansible-package-module-2024], Puppet's RAL = *types + providers* [B-puppet-ral-resource-2024], K8s
`kind`, Terraform resource-type. **A1 is de-risked.** The novelty is never the shape — only the *spelling*
(in-band sh vs a config DSL).

## 3. Result B — two regimes, and where packaging poisoned the hunt (+SURE; from `notes/171`)
Kind-identity spellings split by **scope of authority**:
- **Within one authority** — in-band, self-paying, solver-lifted, *multiple independent consumers* (Debian
  `Provides:`; systemd `Alias=`/`Wants=` [B-systemd-unit-man-2024]; update-alternatives). These clear the
  floor.
- **Across authorities** ("apt's nginx ≡ brew's nginx") — **no in-band self-paying form exists**; the world
  uses a **central curated index** (repology; CISA's "funded authority" [A-cisa-software-id-ecosystem-2023];
  `owl:sameAs`, *famously misused* [A-halpin-owl-sameas-2010]).

**The poison, named:** *packages* drag **cross-*manager* real-world equivalence** (alias zoo, version
lattice) on top of the clean channel — that topping was round-171's rabbit-hole. The cross-*oracle* channel
underneath (A and B agreeing on one opaque handle) is a *different, smaller* problem and is the real target.
CISA's verdict transfers cleanly: per-token identity is cheap; **grouping into a named kind is the unsolved
part**, and the world answers it with a thin agreed anchor, not inference.

## 4. Result C — the channel mechanism (+SURE; from `notes/172`, adjacent fields, PLT-free)
- **C1 · identifier ⊥ matching, consumer-driven.** Durable schemes separate the *identifier* from the
  *match rule* and let the **consumer** pick match-depth: BCP-47 tag vs RFC-4647 *range*
  [B-w3c-language-tags-2024]; InChI full-id vs layer-match [C-inchi-wikipedia-2026]; Pact *contract-by-
  example* not schema [B-pact-cdc-docs-2022]. → A's precondition is a *pattern* matching B's handle at the
  depth A needs; only what A consumes is checked (B's other behaviour stays free).
- **C2 · reverse-DNS = the X3 clobber solved ergonomically, and the most sh-native handle.** Root the
  handle in *existing DNS* instead of a new registry [C-reverse-dns-notation-wikipedia-2026]; universal
  cross-field convergence (Java, UTI, D-Bus, dconf, Flatpak, freedesktop, iSCSI, AT-proto). It is *already
  a plain string* → an oracle spells `kind=net.example.wombat` as a lifted sh datum, **zero Dorc registry**.
- **C3 · dimensionality = minimal-but-extensible.** "Keep the tag as short as possible; add subtags only
  where they distinguish" [B-w3c-language-tags-2024]; InChI layers omitted when irrelevant
  [C-inchi-wikipedia-2026]. Start **flat + optional layers**, never a mandatory schema.
- **C4 · cross-party matching *forces* a thin coherence standard.** InChI is computed-not-assigned, yet
  independent groups' ids wouldn't match until a **"standard InChI" fixed the layer-set**
  [C-inchi-wikipedia-2026] — `095 f28` coherence, proven in the field: **the dimensions A and B *compare
  on* must be agreed; private extra sub-data may ride along.**
- **C5 · self-describing reduces but never removes the anchor.** Bake the kind in (multihash type-prefix
  [B-multihash-multiformats-2024]; Apple **dynamic UTIs** carry a discovered `frob` extension with no
  registry [B-houghton-utis-2012]) — but multihash's own critique ("still need OOB that it *is* a
  multihash") + InChIKey collisions reconfirm `094 g5`: **decentralize everything except the last bit.**
- **C6 · carry-vs-compare.** A handle may carry sub-data it does *not* match on (semver `+build` ignored
  for precedence [B-semver-spec-2013]; UTI tags) → provenance/version can flow A↔B without affecting the
  discharge.

## 5. The lean — the sh-spelling candidate (±, for adversarial-crosscheck to earn)
~SUSPECT the least-burdensome cross-oracle channel is a **reverse-DNS-rooted kind-handle**, author-declared
in the oracle and **lifted into the analyzer index** (per `dq-kOOB`, never a function-name — X3):

```sh
# oracle B (the prober), idiomatic sh; the handle is a plain string datum Dorc lifts:
wombat_kind=net.example.wombat            # C2 reverse-DNS root: decentralized, collision-free, no registry
wombat_state() { frobctl status "$1"; }   # B's real probe; rc/stdout -> {defrocked|frocked|wet}
# oracle A (the consumer) names the SAME handle + only the state it needs (C1 consumer-driven, C3 minimal):
#   requires net.example.wombat is defrocked
```

with: **state sub-data** as optional sub-tags (`net.example.wombat#defrocked`, C3) · **matching**
consumer-driven at A's depth (C1) · a **thin coherence standard** for the *compared* states (C4) · extra
**carried** provenance allowed (C6) · the irreducible **≥1 anchor** = A and B both literally writing
`net.example.wombat` (C5/`094 g5`). It is plain strings + sh → **off-ramp-clean**.

**Honest bound on the lean (the converging negative — now TRIPLE-confirmed):** *all three* rounds —
packaging (`notes/171`), adjacent-fields (`notes/172`), and code (`notes/173`) — failed to find a
self-paying, *inference-only* cross-referent spelling. Packaging's `Provides:` only "pays" because a whole
ecosystem already consumes it (a lone oracle cannot bootstrap that); every adjacent field bottoms out at a
shared anchor it cannot inference-away; and the real-code hunt found the **environment** to be a genuine
non-file/non-package coordination channel but **co-reference-only** (the var *name*'s meaning is convention,
ungrounded). So the channel is **irreducibly author-declared**; the win is only *which anchor is
least-burdensome* — a **convention-named env-var** like `KUBECONFIG` that independent tools *already* read
(`notes/173` k1c), or a reverse-DNS string — and **co-reference/dataflow is the only *free* link** (`094
g1`, sound within one script). This
modestly beats the comment-floor (the handle has decentralized independent value + is real sh), but it is
**not magic** — exactly as predicted.

## 6. The dimensionality menu (the `dq-entity-algebra` answer, with field verdicts)
| answer to "how much sub-data" | exemplar | verdict for Dorc |
|---|---|---|
| flat id + **external conformance DAG** | UTI [B-houghton-utis-2012] | keep handle flat; structure as a *declared* relation, not in the string |
| **optional progressive layers** | InChI [C-inchi-wikipedia-2026] | match at the layer A cares about; **fix a standard layer-set for compared dims (C4)** |
| **minimal-but-extensible** subtags | BCP-47 [B-w3c-language-tags-2024] | the default: shortest that distinguishes, extend on demand |
| fixed + a **carried-but-ignored** slot | semver [B-semver-spec-2013] | carry provenance without it affecting identity (C6) |
| self-describing **type-prefix** | multihash [B-multihash-multiformats-2024] | bake kind in so A doesn't hardcode B's repr; anchor still OOB |
| **precision = prefix length** | geohash | prefix-match = is-a; a cheap subtype encoding |

Lean: **flat reverse-DNS handle (C2) + optional InChI/BCP-47-style state layers (C3) + a UTI-style declared
conformance relation if subtyping is ever needed**, never a mandatory structured kind.

## 7. Hindsight verdicts (condensed)
CPE (top-down global id) **failed** → purl (provider-qualified, refuses cross-provider sameness) **won**
[A-cisa-software-id-ecosystem-2023] · `owl:sameAs` **misused** (binary "same" too strong)
[A-halpin-owl-sameas-2010] · systemd **self-deprecates** explicit deps (prefer inferred)
[B-systemd-unit-man-2024] · MIME registration-friction → UTI's own-a-domain **won** [B-houghton-utis-2012] ·
reverse-DNS **universally adopted** [C-reverse-dns-notation-wikipedia-2026] · InChI needed a **standard
layer-set** for cross-party matching [C-inchi-wikipedia-2026]. Throughline: **decentralized root + thin
shared standard + consumer-driven, minimal-but-extensible matching** is the repeatedly-validated ergonomic
shape; **global top-down identity repeatedly fails.**

## 8. Open questions / what the final code-round (`notes/173`) tests
- **q1 — ANSWERED (`notes/173`).** The **environment** *is* a real, analyzable, non-file/non-package
  coordination channel — convention-named cross-tool handles (the `KUBECONFIG` model, read by independently-
  authored `kubectl` + `helm` + `velero`; `export`+dataflow is statically visible; already core-modeled,
  `09A §3a`/`099 W5`). So the **free-link surface extends beyond filenames** to named env-handles (a real
  gain). **But it gives co-reference only — the KIND stays convention/author-declared** — so §5's
  "irreducibly author-declared" *holds*, now triple-confirmed. Residual **hazard:** ambient env-leakage
  (`099 W5`; the postgres entrypoint's defensive `PGHOST= PGHOSTADDR=` unset).
- **q2 — `dq-kOOB`:** does a reverse-DNS handle spelled as a *lifted assignment/marker* satisfy the in-band
  redline (config-as-data lifted, not parsed-as-config)?
- **q3 — `dq-entity-algebra`:** ratify flat-handle + optional-layers + declared-conformance (§6); reject
  mandatory structured kinds.
- **q4 — coherence governance (C4):** who owns the thin "standard layer-set" for a kind's *compared*
  states, given no central registry? (The one place a minimal shared authority seems unavoidable.)

## 9. What this feeds
`dq-kOOB` (the in-band spelling = a lifted reverse-DNS datum **or a convention-named env-var**, not a
function-name; X3-safe — and GitHub Actions shows the concrete sh-template: `echo "name=value" >>
"$GITHUB_OUTPUT"` ↔ `${{ steps.x.outputs.name }}` [B-github-actions-workflow-commands-2024], an
author-declared-yet-statically-analyzable handle emission) · `dq-entity-algebra` (flat + optional layers +
declared conformance) · the `094`/`099`/`09A` relational frame (confirmed from **three** outside angles —
packaging, adjacent-fields, and real ops/CI code: co-reference ≠ grounding is the same wall as CISA's
"grouping is unsolved", every adjacent field's ≥1-anchor floor, and GitHub Actions' author-declared output
names) · **`kCOMMS` (transport bonus, beyond K1)** — GitHub Actions independently arrived at the two-lane
split (freeform logs on stdout vs coordination/state in environment *files*) and was *forced* there by an
injection CVE [B-github-actions-setoutput-deprecation-2022]: real-world confirmation that signalling must
not share a lane with freeform output · `DESIGN` "Inference limitations" (the `wombat` chicken-and-egg: the
continuation is "an author-declared, reverse-DNS-or-env-var-named, analyzer-lifted handle — co-reference is
the only free part").

## 10. Candidate sh-spellings — *illustrative possibilities, not recommendations*

> **Read this as strawman, not lean.** Everything below is a *candidate* spelling, written concretely only
> so it can be argued with and thrown away — per AGENTS, inline strawmen to motivate a problem, never
> committed Dorc patterns. Confidence is **low** (-GUESS / --WONDER) throughout; the lone narrow exception
> is a *documented cost* (P6, GitHub in-band-signalling), flagged inline. The whole set sits downstream of
> §5's **uncertain** lean that the kind is author-declared — if that's wrong, most of this is moot. None of
> it decides `dq-kOOB` / `dq-entity-algebra`; it is grist for that decision, nothing more.
>
> Running example (so the sh coheres): a tool `frobctl` *might* manage "wombats" with states
> `defrocked`/`frocked`/`wet`; oracle **B** wraps `frobctl`; oracle **A** wraps `zonk`, which *might* be
> non-mutative only on a defrocked wombat. Intent: plain POSIX, dash-clean, inert if Dorc vanishes.

- **P1 (-GUESS) — the kind-handle *could* be a lifted datum rather than a function name.** *If* the kind
  must be declared at all (§5, uncertain), one option is a string assignment Dorc lifts — perhaps
  reverse-DNS-rooted so independent oracles needn't a registry to avoid collisions (the Java/D-Bus/UTI
  pattern [C-reverse-dns-notation-wikipedia-2026][B-houghton-utis-2012]). A *flat slug* or a *convention-named
  env-var* might serve equally; this is a menu, not a pick. (A function-*name* convention is separately ruled
  out by the inherited X3 constraint, not by this suggestion.)
  ```sh
  oracle_kind='net.frobnitz.wombat'      # one candidate datum-form; could as easily be a slug or an env-var name
  ```

- **P2 (-GUESS on the spelling; the hazard is real) — the probe *might* capture the tool's own rc.** The
  *cost* it dodges is documented: `cmd | grep -q` conflates "no match" with "tool failed" (the DP-3 finding,
  round-16 spike), which would mis-read as a converged skip. One possible shape:
  ```sh
  wombat_is() {                          # candidate convention: 0 holds · 1 absent · 2 unknown
     command -v frobctl >/dev/null 2>&1 || return 2
     _s=$(frobctl status -- "$2" 2>/dev/null) || return 2
     [ "$_s" = "$1" ]
  }
  ```

- **P3 (--WONDER) — states *might* hang off the kind as optional selectors.** How much sub-data is right is
  the open `dq-entity-algebra` question. The adjacent fields *hint* at minimal-but-extensible (BCP-47's
  "shortest that distinguishes" [B-w3c-language-tags-2024]; InChI's omit-irrelevant-layers
  [C-inchi-wikipedia-2026]) and consumer-named depth (Pact [B-pact-cdc-docs-2022]) — a possibility to test,
  not a shape to adopt.
  ```sh
  #  net.frobnitz.wombat            # bare kind     ·     net.frobnitz.wombat#defrocked   # a state selector
  ```

- **P4 (-GUESS) — effects *may* often be inferable from the idempotency-guard, so perhaps declare little.**
  The (provider, verb) is the command itself; polarity *might* fall out of guard structure (094 g1 — a probe
  + an establisher sharing an entity), leaving an explicit marker only for the un-inferable residue. Whether
  inference reaches far enough is unknown.
  ```sh
  wombat_is defrocked "$w" || frobctl defrock -- "$w"   # the guard *might* already imply defrock → #defrocked
  ```

- **P5 (-GUESS) — A's precondition *could* be a guard naming the same handle.** *If* A and B agree at all, it
  might be only by both referencing `net.frobnitz.wombat#defrocked`; a consumer-side guard is real, useful sh
  and could double as the lift-point.
  ```sh
  zonk_check() { oracle_requires 'net.frobnitz.wombat#defrocked' "$1"; }   # candidate marker
  ```

- **P6 (~SUSPECT, *narrowly* — a documented cost, not speculation) — the verdict should not share a lane
  with the wrapped tool's freeform output.** GitHub Actions mixed coordination-signalling into stdout
  (`echo "::set-output…"`) and paid for it with injection CVEs, then moved coordination onto dedicated
  environment *files* — *"to avoid untrusted logged data to use … commands without the intention of the
  workflow author"* [B-github-actions-setoutput-deprecation-2022] (`kCOMMS`). The firm part is *only* that
  narrow claim: keep the verdict off the tool's freeform stdout. The spelling stays a possibility:
  ```sh
  wombat_is "$state" "$entity"; rc=$?
  printf '%s\t%d\n' "net.frobnitz.wombat#$state" "$rc" >> "$DORC_VERDICT"   # cf. >> "$GITHUB_OUTPUT"
  ```

- **P7 (-GUESS) — the *link* (not the kind) *may* come for free from co-reference.** The one part with
  support from all three rounds: a shared variable or a conventional env-var threads A and B with no
  declaration (094 g1; the KUBECONFIG-read-by-both-kubectl-and-helm pattern [C-petergardfjall-helm-init-2024]).
  The kind would still need P1; only the *coupling* is free.
  ```sh
  w=$(frobctl create); zonk_check "$w" && zonk -- "$w"; frobctl defrock -- "$w"   # same $w → co-reference
  ```

- **P8 (-GUESS, but the highest-value direction) — where the kind is *already* grounded in system metadata,
  the probe *might* just read it.** No new handle if the OS already wrote one (round-1 rank-1).
  ```sh
  wombat_is() { systemctl is-enabled -- "$2" >/dev/null 2>&1; }   # rides metadata written for systemd
  #  other candidates: dpkg -s · pkg-config --exists · id -u · ip link show
  ```

**Anti-patterns — these carry *documented* costs (firm, but narrowly about the exact failure):**
- A **dotted function-name** handle (`frobctl.check()`) empirically fails `dash -n` (151 X4, run under
  dash/bash), breaking the off-ramp; and any 1-place name can't carry the 3-place relation (X3). *That* cost
  is clear; the datum-form fix (P1) is the speculative part.
- **Verdict mixed into freeform stdout** — the GitHub CVE (P6). Clear cost.
- **`cmd | grep -q` as the probe** — the DP-3 rc-conflation. Clear cost.
- *(Leanings, not cost-backed:)* inferring the kind from token shape or co-occurrence *appears* unsound
  (095 f27), and a full consumer-side schema *appears* heavier than consumer-driven naming — treat as
  ~SUSPECT, not established.

**Bound (restating §5, still uncertain):** *if* anything here holds, it is probably that the kind needs
*one* declared anchor both A and B write, with most of the rest free or inferred — but the survey makes that
a **converging lean, not a proof**, and this whole section is a menu to argue with, not a recommendation.
