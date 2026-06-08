# 175 — The cross-oracle kind-channel: K1 synthesis + lean (round 17)

> **Status (round 17, 2026-06-07): the single K1 plans-synthesis.** Integrates four gather rounds —
> `notes/171` (packaging prior-art), `notes/172` (adjacent fields), `notes/173` (shell-spellings / env-vars
> / GitHub Actions), `notes/174` (command-execution / the getent pattern) — plus the real-code strawmen
> (`notes/17x-strawmen/`). Folds in the prior packaging map (since removed). **Deliberately lean:** this is
> the *semantic* map; noisy enumerations (the cross-OS getent catalog; the per-specimen oracle extractions)
> stay in their notes and are linked, not inlined. A **map with a lean**, *not a decision* — the lean is
> marked "for adversarial-crosscheck to earn," and that pass has **not** been run. The reunited K1+K2
> synthesis is a later step (`17Y`/`17Z`). AI-generated; confidence-marked (+SURE/~SUSPECT/-GUESS/--WONDER);
> trust repo-root `DESIGN`/`KNOBS` over this.

> **[REVISED→17N · UNSETTLED · 2026-06-08]** The reunited K1+K2 synthesis forecast above as "`17Y`/`17Z`" **landed as `plans/17N`** — treat 17N as current where it differs. Two refinements supersede leans here: (1) `094` g1's framing of co-reference as a *link* (inherited by P4 and §11's "only free link") is **downgraded to a may-grade *hint*** — a guard's operand need not be its body's operand; (2) the `dq-kOOB` lean now carries a provisional non-sh-native **inline-annotation escape-hatch** for the claims sh cannot spell.

## 0. The question
How do two oracles authored **independently** (A, B) converge on one **opaque kind-handle + its state
sub-data** so a fact B *probes* discharges a precondition A *declares* — Dorc routing it, understanding
**none** of the semantics, with **nothing hardcoded**? Concretely: A = "non-mutative *provided your wombat
is defrocked*"; B = "I can check defrocked / frocked / wet." The floor to beat is the inert `# dorc: kind=…`
comment.

## 1. Inherited frame (true going in; do not relitigate)
A0 referent-agnostic by default · A1 the shape is **3-place** `(kind, provider, verb) → effect`, bound to a
**named kind** not a shared token · A2 the `kOOB` in-band lean (a lifted analyzer-index, not sidecar config)
· **X3** a 1-place sh *function-name* namespace cannot carry the 3-place relation (apt's then brew's oracle
*clobber*) — `notes/151` · the relational adjudication (`095`: Dorc keeps relational contracts over
meaningless symbols; grounding is the human's) · the **≥1-anchor floor** (`094 g4/g5`: co-reference ≠
grounding; something must be declared).

## 2. Result A — the shape is universal (+SURE; `notes/171`)
`(kind, provider, verb)` is the **mainstream convergent design**, re-invented everywhere: Debian
virtual-packages [A-debian-policy-relationships-2024], `update-alternatives` slots
[B-update-alternatives-man-2024], RPM `Provides`, Ansible's generic `package:`
[B-ansible-package-module-2024], Puppet's RAL = *types + providers* [B-puppet-ral-resource-2024], K8s
`kind`, Terraform resource-type. **A1 is de-risked.** The novelty was never the shape — only the *spelling*
(in-band sh vs a config DSL).

## 3. Result B — two regimes, and where packaging poisoned the hunt (+SURE; `notes/171`)
- **Within one authority** — in-band, self-paying, solver-lifted, *many independent consumers*: Debian
  `Provides:`; systemd `Alias=`/`Wants=` [B-systemd-unit-man-2024]; update-alternatives. These clear the
  floor.
- **Across authorities** ("apt's nginx ≡ brew's nginx") — **no in-band self-paying form exists**; the world
  uses a **central curated index** (repology; CISA's "funded authority" [A-cisa-software-id-ecosystem-2023];
  `owl:sameAs`, famously *misused* [A-halpin-owl-sameas-2010]).

**The poison:** *packages* drag **cross-*manager* real-world equivalence** (alias zoo, version lattice) on
top of the clean channel — round-171's rabbit-hole. The cross-*oracle* channel underneath is a smaller,
different problem. CISA's verdict transfers: per-token identity is cheap; **grouping into a named kind is
the unsolved part**, answered by a thin agreed anchor, not inference.

## 4. Result C — the channel mechanism (+SURE; `notes/172`, adjacent fields, PLT-free)
- **C1 · identifier ⊥ matching, consumer-driven.** Durable schemes separate the *identifier* from the
  *match rule*; the **consumer** picks match-depth: BCP-47 tag vs RFC-4647 *range* [B-w3c-language-tags-2024];
  InChI full-id vs layer-match [C-inchi-wikipedia-2026]; Pact *contract-by-example* [B-pact-cdc-docs-2022].
  → A's precondition is a *pattern* matching B's handle at the depth A needs; only what A consumes is checked.
- **C2 · reverse-DNS = the X3 clobber solved ergonomically, the most sh-native handle.** Root the handle in
  *existing DNS*, not a new registry [C-reverse-dns-notation-wikipedia-2026]; universal convergence (Java,
  UTI, D-Bus, dconf, Flatpak, iSCSI, AT-proto). Already a plain string → `kind=net.example.wombat` as a
  lifted datum, zero Dorc registry.
- **C3 · dimensionality = minimal-but-extensible** [B-w3c-language-tags-2024][C-inchi-wikipedia-2026]: flat +
  optional layers, never a mandatory schema.
- **C4 · cross-party matching *forces* a thin coherence standard.** InChI is computed-not-assigned, yet
  independent groups' ids wouldn't match until a **"standard InChI" fixed the layer-set** — `095 f28`
  coherence proven in the field: the *compared* dimensions must be agreed; private extra sub-data may ride.
- **C5 · self-describing reduces but never removes the anchor** (multihash type-prefix
  [B-multihash-multiformats-2024]; Apple **dynamic UTIs** carry a discovered tag with no registry
  [B-houghton-utis-2012]) — but the anchor is irreducible (`094 g5`).
- **C6 · carry-vs-compare** (semver `+build` carried-but-ignored [B-semver-spec-2013]) → a handle may carry
  provenance it doesn't match on.

## 5. Result D — command-execution carries kind-signal, but only for a *blessed* vocabulary (+SURE; `notes/173`+`notes/174`)
The round's one genuine **positive**, and the enabling-condition shift. The earlier rounds found env-vars
(`KUBECONFIG`, read by independent tools [C-petergardfjall-helm-init-2024][B-twelve-factor-config-2017]) give
analyzable **co-reference but not kind**. Round 174, *once Dorc is allowed to bless a small set of
capabilities*, found that **a command execution can name the kind itself**:
- **`getent <database> <key>`** [B-getent-man-2024] — arg-1 *is* the kind, from a bounded system-blessed NSS
  vocabulary (`passwd`/`group`/`hosts`/`services`/`networks`/`protocols`/…), read-only, rc = 3-outcome fact.
  A ready-made, self-kind-describing, multi-kind **fact-probe** — the closest thing in the whole effort to
  "Dorc reads the kind *off the command*."
- **The pattern is universal** — every OS, every subsystem, in three shapes: positional kind-first
  (`getent`/`ip <object>`/`dig <TYPE>`/`Get-CimInstance -ClassName`/`dladm show-<class>`), a `-t`/`--type`
  flag (`dmidecode -t`/`lsns -t`/`sc query type=`), or the arg *is* a kind-namespace member
  (`sysctl`/`getconf`/`getsebool`/`prlimit --<resource>`). *Full cross-OS catalog + 8 graded anchors:
  `notes/174`.*
- **Bootstrap implication (`effort-allocation`):** blessing ~40 commands covers *hundreds* of kinds —
  `getent` = user/group/host/service/…; `Get-CimInstance` ≈ all of Windows; `semanage`/`seinfo` = SELinux.
  A real oracle-library bootstrap, not a toy.
- **A new, orthogonal axis — the command-*wrapper*.** `sudo`/`ssh host CMD`/`docker exec C CMD`/`flock`/
  `chroot` [B-flock-man-2024] run a *nested, recursively-analyzable* command in a named **context-kind**
  (user/host/container/lock); `ssh host`/`docker exec` *are Dorc's own execution model*. ~SUSPECT this
  deserves its own `dq`.
- **THE BOUND (+SURE):** all of this is **blessed/bounded**. An arbitrary/opaque kind (the `wombat`) still
  needs the declared handle (§7 P1). Command-execution beats co-reference **only via blessing** — which is
  the enabling condition the design now permits, narrowing the "no-magic" negative to the *un-blessed* case.

## 6. The empirical test — strawmen on real downloaded code (+SURE; `notes/17x-strawmen/`)
A viability check on **six real, commit-pinned provisioning scripts** (Debian/Ubuntu/RHEL × app/service/
cache/CI/hardening; honestly sampled, *not* cherry-picked), guards extracted by grep without reading the
mutative bodies.
- **The spine is real.** Every book's idempotent core *is* the getent-pattern kinds
  (`user`/`group`/`service`/`tool`/`file`), probed by exactly the Result-D commands; a careful author
  (`consul`) already writes the whole oracle for free, and the kinds are uniform → cross-script-correlatable.
- **The working example — a cross-script elision token-co-reference cannot reach.** `enginescript-redis`'s
  unguarded `usermod -aG redis www-data` has an *undeclared* dependency on `user:www-data` (created by a
  *separate* web-stack script). No shared token → co-reference (`094 g1`) is blind. But `getent passwd
  www-data` (a blessed kind) **discharges it across the script boundary**, and `id -nG www-data | grep -qw
  redis` **elides the usermod**. Real code; reachable *only* by kind-correlation. (Same shape: `group:docker`.)
- **The honest long tail (the coverage measure).** A real corpus leaves much **un-liftable**: config
  *content*, whole-script *sentinels*, *packages*, unguarded *group-membership* + *ufw ports*. Hardening
  scripts are the worst case (~2 of ~9 state-changes author-guarded). Port-probe-ability is *provider-
  dependent* — clean under `firewall-cmd --query-port`, fragile under `ufw status | grep` (re-introducing the
  15x `.`-as-regex wrong-skip).
- **Verdict:** the blessed-kind spine covers the *idempotent core* of real provisioning and uniquely enables
  cross-script elision; a meaningful fraction (the tail) still needs oracle-*supplied* probes or falls to the
  ⊤-run floor. *Specimens + extracted oracles + the elision strawman: `notes/17x-strawmen/`.*

## 7. The lean — the sh-spelling candidate (±, for adversarial-crosscheck to earn)
~SUSPECT the least-burdensome cross-oracle channel is a **handle author-declared in the oracle and lifted
into the analyzer index** (per `dq-kOOB`, never a function-name — X3): a **reverse-DNS string** (C2) *or* a
**convention-named env-var** (`KUBECONFIG`-model, `notes/173`) **or**, for kinds in a blessed vocabulary, the
kind read straight off a blessed probe (`getent <db>`, Result D — no handle at all). With: **state sub-data**
as optional selectors (C3); **consumer-driven matching** at A's depth (C1); a **thin coherence standard** for
the *compared* states (C4); **carried** provenance allowed (C6); the irreducible **≥1 anchor** = A and B both
naming the same handle (C5/`094 g5`). All plain strings + sh → off-ramp-clean.

**Honest bound (the converging result, across four rounds + the strawmen):** for the **arbitrary/opaque**
cross-referent case, *no* round found a self-paying, inference-only spelling — the kind is **irreducibly
author-declared**, and co-reference/dataflow (incl. named env-handles) is the only *free* link (`094 g1`).
Round 174 sharpens, not overturns, this: for a **blessed, bounded vocabulary** the kind *is* read off the
command (handle-free), and the strawmen show that blessed spine covers the idempotent core of real
provisioning — while the open-ended tail stays declared-or-⊤. So: **blessing buys a large, high-value slice;
the genuinely-open kind still costs one declaration. Not magic — exactly as predicted.**

**Weakest points (mark for the adversarial pass):** ~SUSPECT the lean quietly assumes the *engineer* (not
the lazy admin) carries kind-grounding; re-test under "the other user." ~SUSPECT `owl:sameAs` + CISA's
"grouping is unsolved" bite harder if a future feature *does* need cross-provider sameness.

## 8. The dimensionality menu (the `dq-entity-algebra` answer)
The "how much sub-data" answer is **minimal-but-extensible**, with a **carry-vs-compare** split. Field
verdicts: flat-id + external conformance-DAG (UTI) · optional progressive layers, but a fixed *standard*
layer-set for cross-party matching (InChI) · minimal-but-extensible subtags + separate graded matching
(BCP-47) · fixed + a carried-but-ignored slot (semver) · self-describing type-prefix (multihash) ·
precision-as-prefix-length (geohash). **Lean:** flat handle + optional state layers + a UTI-style *declared*
conformance relation if subtyping is ever needed; never a mandatory structured kind. *(Full table:
`notes/172`.)*

## 9. Hindsight verdicts (condensed)
CPE (top-down global id) **failed** → purl (provider-qualified, refuses cross-provider sameness) **won**
[A-cisa-software-id-ecosystem-2023] · `owl:sameAs` **misused** [A-halpin-owl-sameas-2010] · systemd
**self-deprecates** explicit deps (prefer inferred) [B-systemd-unit-man-2024] · MIME registration-friction →
UTI's own-a-domain **won** [B-houghton-utis-2012] · reverse-DNS **universally adopted**
[C-reverse-dns-notation-wikipedia-2026] · InChI needed a **standard layer-set** for cross-party matching
[C-inchi-wikipedia-2026] · GitHub Actions moved coordination off freeform stdout onto dedicated files **after
an injection CVE** [B-github-actions-setoutput-deprecation-2022]. Throughline: **decentralized root + thin
shared standard + consumer-driven minimal matching** wins; **global top-down identity repeatedly fails.**

## 10. Open questions
- **q1 — ANSWERED (`notes/173`/`174`).** Non-file/non-package coordination channels exist and are analyzable:
  env-vars give co-reference; **blessed command-execution (Result D) gives the kind itself** — but only for a
  bounded vocabulary; the open-ended kind stays declared.
- **q2 — `dq-kOOB`:** does a reverse-DNS handle (or env-var name, or blessed-probe) spelled as a *lifted
  datum* satisfy the in-band redline? (config-as-data lifted, not parsed-as-config.)
- **q3 — `dq-entity-algebra`:** ratify flat + optional layers + declared conformance; reject mandatory
  structured kinds.
- **q4 — new: the wrapper-context `dq`** (`ssh host`/`docker exec`/`sudo`/`flock` = execution-context kinds,
  on Dorc's own execution model — §5). And **coherence governance** (who owns a kind's "standard" compared
  states, given no central registry).

## 11. What this feeds
`dq-kOOB` (the in-band spelling = a lifted reverse-DNS datum / env-var / blessed-probe, not a function-name;
X3-safe) · `dq-entity-algebra` (flat + optional layers + declared conformance) · **`effort-allocation`**
(bless ~40 getent-pattern commands → a large oracle-library bootstrap; the spine the strawmen confirmed) ·
the new **wrapper-context `dq`** · the `094`/`099`/`09A` relational frame (confirmed from four outside angles:
co-reference ≠ grounding is the same wall as CISA's "grouping is unsolved", every adjacent field's ≥1-anchor
floor, env-var name-convention, and the blessed-vs-arbitrary split) · **`kCOMMS`** (GitHub Actions
independently arrived at the two-lane split — freeform logs vs coordination/state in environment *files* —
forced by an injection CVE [B-github-actions-setoutput-deprecation-2022][B-github-actions-workflow-commands-2024]) ·
`DESIGN` "Inference limitations" (the `wombat` chicken-and-egg: the continuation is "an author-declared,
reverse-DNS-or-env-var-named-or-blessed-probed, analyzer-lifted handle — co-reference is the only free part,
and *blessing* buys the rest for a bounded vocabulary").

## 12. Candidate sh-spellings — *illustrative possibilities, not recommendations*

> **Strawman, not lean.** Each is a *candidate* spelling, concrete only so it can be argued with and thrown
> away (per AGENTS, inline strawmen to motivate). Confidence **low** (-GUESS/--WONDER) except where a
> *documented cost* is flagged. All downstream of §7's uncertain lean. Running example: tool `frobctl`
> manages "wombats" (states defrocked/frocked/wet); oracle **B** wraps it; oracle **A** wraps `zonk`,
> non-mutative only on a defrocked wombat. Intent: plain POSIX, dash-clean, inert if Dorc vanishes.

- **P1 (-GUESS) — handle as a lifted datum, not a function name.** `oracle_kind='net.frobnitz.wombat'` —
  reverse-DNS, collision-free, no registry (C2); a flat slug or convention-named env-var could serve. (The
  function-*name* form is ruled out by X3, not by this suggestion.)
- **P2 (-GUESS; the hazard is documented) — probe captures the tool's own rc.** `cmd | grep -q` conflates
  "no match" with "tool failed" (DP-3). `wombat_is() { command -v frobctl >/dev/null 2>&1 || return 2; _s=$(frobctl status -- "$2" 2>/dev/null) || return 2; [ "$_s" = "$1" ]; }`
- **P3 (--WONDER) — states as optional selectors** (`net.frobnitz.wombat#defrocked`); minimal-but-extensible
  (C3), consumer-named depth (Pact). Open `dq-entity-algebra`.
- **P4 (-GUESS) — let polarity be inferred from the idempotency-guard** (`094 g1`), declaring only the
  un-inferable residue: `wombat_is defrocked "$w" || frobctl defrock -- "$w"`.
- **P5 (-GUESS) — A's precondition is a guard naming the same handle.** `zonk_check() { oracle_requires 'net.frobnitz.wombat#defrocked' "$1"; }`
- **P6 (~SUSPECT, *narrow* — documented cost) — the verdict must not share a lane with freeform output.**
  GitHub Actions paid an injection CVE for stdout-marker coordination, then moved to environment files
  [B-github-actions-setoutput-deprecation-2022] (`kCOMMS`). Keep the verdict off the tool's stdout; the
  spelling stays a possibility: `printf '%s\t%d\n' "net.frobnitz.wombat#$state" "$rc" >> "$DORC_VERDICT"   # cf. >> "$GITHUB_OUTPUT"`
- **P7 (-GUESS) — the *link* (not the kind) comes free from co-reference** (a shared var/env-var threads A↔B;
  `094 g1`; the KUBECONFIG pattern [C-petergardfjall-helm-init-2024]). The kind still needs P1.
- **P8 (-GUESS, highest-value) — read existing system metadata where the kind is already grounded**
  (`systemctl is-enabled`/`dpkg -s`/`pkg-config --exists`/`id -u`); no new handle if the OS wrote one.
- **P9 (firm on the prior-art, hedged on the use; Result D) — for a *blessed, bounded* vocabulary, read the
  kind off the command.** Documented fact: `getent <database> <key>` names the kind as arg-1, read-only,
  3-outcome rc [B-getent-man-2024]; the wrapper category runs a nested command in a named context
  [B-flock-man-2024]. Hedged: Dorc *could* bless those vocabularies (`getent <db>`'s `<db>` token = the
  kind-name) — handle-free for the ~16 NSS kinds — but it does **not** generalize to an opaque kind (§5
  bound). `getent passwd "$u"  # arg-1 'passwd' = the kind; rc = found/absent/unknown; read-only`

**Anti-patterns (documented costs, firm but narrow):** a dotted function-name handle (`frobctl.check()`)
fails `dash -n` (`notes/151` X4) + can't carry the 3-place relation (X3) · verdict mixed into freeform stdout
(the GitHub CVE, P6) · `cmd | grep -q` (DP-3). *(Leanings, not costs:)* inferring the kind from token shape /
co-occurrence appears unsound (`095 f27`); a full consumer-side schema appears heavier than consumer-driven
naming.

**Bound (restating §7, still uncertain):** the open-ended kind needs *one* declared anchor both A and B
write; blessing buys a bounded vocabulary for free; co-reference buys the link. The survey makes this a
**converging lean, not a proof** — grist for `dq-kOOB`/`dq-entity-algebra`, earned only by the
(still-pending) adversarial pass.
