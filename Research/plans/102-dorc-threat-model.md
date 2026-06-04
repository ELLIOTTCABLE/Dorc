# 102 — Dorc system threat-model (round-10 deliverable (b))

The **actual threat-model** the security pass was for: STRIDE-per-element over the trust-boundary diagram
(`plans/101`), a premortem to catch what STRIDE misses, the per-oracle template extracted as a byproduct,
and the banked cheap-now/expensive-later list. Every threat grounds in a graded `[slug]` (`../sources.json`).
Methodology per `plans/101`: Shostack Four-Questions framing, **lightweight STRIDE run by everyone** (not
expert attack-trees), premortem [C-klein-premortem-author-page-2007] as kickoff. Responses use Shostack's
verbs: **eliminate / mitigate / transfer / accept**.

Scope note: this is *footgun-avoidance for the design*, not a product audit. "Did we do a good job?"
(Q4) is deferred to the calibration harness (`kVERIFY`), not claimed here.

## Soundness boundary (governs every response below — read first)
DESIGN is explicit: *"plan stage doesn't mutate, **as long as your handwritten oracles don't**"* (Priorities)
and *"if either end fails to follow the contracts we present, we're immediately unsound"* (Sensitivities).
Dorc's correctness is **best-effort and stops hard the moment it touches real-world oracle-library /
author-playbook code**: the CFG/effect analysis is sound about *propagation* but bottoms out in the author's
unverified *grounding* of what each command does (the symbol-grounding problem, DESIGN). Consequence for
this model: a response may **eliminate** a threat *only* when the fix lives in Dorc's **own trusted code**
(transport config, output sanitisation, registry-omission). Anything that depends on reasoning about
oracle/playbook behaviour is **transfer-to-contract + best-effort mitigate**, *never* eliminate — Dorc is
unsound *by thesis* if the author breaks the contract. This is the doctrine, not a defect; a model that
claimed otherwise would be selling the soundness Dorc explicitly disclaims.

## System model — the five elements (Q1: what are we working on?)
```
 [engineer]→authors→ [oracle (3rd-party sh)] —obtain via git→ [operator workstation = CONTROL NODE]
                                                                    │ holds fleet SSH keys (kAGENTLESS)
                                                          push over ssh (m→n→o hops) │ displays plan-as-shell to TTY
                                                                    ▼
                                                  [managed host] — probe (read-only) ─then─ apply (root)
```
Trust boundaries crossed: author→oracle (supply chain), oracle→control-node (obtain), control-node→host
(push/hops), probe↔host (the read-only contract), plan→operator-TTY (display).

## STRIDE per element (Q2/Q3: what can go wrong, what do we do)

### E1 · Operator workstation = control node (the crown-jewel)
- **E / S — workstation compromise = fleet RCE.** Holds keys to every host; this is Salt's master-RCE
  blast-radius [A-fsecure-saltstack-bypass-2020], welded to here by `kAGENTLESS`. **Accept** (welded push
  consequence) + **mitigate**: short-TTL certs over static keys; hardware-token auth; least-privilege
  per-host accounts so a single host-compromise isn't root-everywhere (Salt/Ansible `become`-minimization).
- **Design rule (eliminate by omission):** add *no* feature that concentrates fleet creds beyond this
  welded minimum — no credential cache, no daemonized key-holder, no agent-forward default (→ E3-transport).

### E2 · The push / hop chain (SSH transport)
- **T / I — a compromised intermediate bastion MITMs or eavesdrops the session.** With `ForwardAgent`, any
  hop can both impersonate the operator fleet-wide *and* read/modify the live session
  [B-heipei-ssh-agent-forwarding-harmful-2015]. **Eliminate**: default **ProxyJump/ProxyCommand**
  (session terminates on the operator workstation; hops are blind TCP relays), agent-forwarding **off**.
- **S — host-key TOFU: pushing config+secrets to an imposter host** (DNS/host-key spoof). CFEngine's
  lesson [A-cfengine-security-trust-model-2001]: *encryption ≠ trustworthiness*; verify host identity.
  **Mitigate**: verify host keys, never blind-accept; integrity-over-secrecy.

### E3 · The probe phase (managed host, read-only pass — Dorc's signature surface AND its hardest soundness boundary)
- **T — a probe leaf mutates** (a probe-section shells out / has a side effect during the "read-only" plan).
  Same family as Chef/Terraform [A-chef-whyrun-harmful-2018] · [B-terraform-external-plan-exec-2017] — but
  **NOT eliminable by Dorc, by thesis** (see *Soundness boundary*): non-mutation is the *author's contract*
  (*"plan stage doesn't mutate, as long as your oracles don't"*) and the analysis stops at the grounding
  boundary. Response is **transfer + mitigate + accept**, never eliminate:
  - **Transfer** → the oracle-author contract (probe-sections must be non-mutative). Author mis-grounds or
    lies ⇒ Dorc *immediately unsound* (DESIGN) — the accepted residual.
  - **Mitigate, fail-safe direction** → `kFAIL-withhold` under-approximates: what Dorc can't *establish*
    inert (relative to the author's grounding) it won't ship as a probe — loses perf, stays safe. Covers the
    *un-analysable* part only.
  - **Mitigate, best-effort backstops** → `plans/077` seccomp socket-observe + deferred `plans/078`
    container/trace *detect some* non-inert leaves the author missed — detection, not proof.
  The CoLiS/smoosh base ([B-colis-specification-of-unix-utilities-2019] · [A-smoosh-popl-2020] ·
  [A-verified-interpreter-shell-vstte-2017]) bounds *how much propagation* can be sound; it does **not**
  extend soundness past the grounding boundary. Dorc's edge over Chef is the **contract + tooling +
  fail-safe direction**, not a guarantee Chef lacked.
- **D — a read-only leaf wedges the host** (interrogating a buggy systemd locked up production)
  [A-chef-whyrun-harmful-2018]. Unpromisable in general. **Accept + mitigate**: timeout/rlimit bounds probe
  *wallclock* (safe to kill a truly read-only probe); **document loudly that read-only ≠ non-blocking ≠
  side-effect-free** — never market the stronger claim (the exact thing Chef warns invites the failure).

### E4 · The oracle (3rd-party shell that runs as root on apply)
- **E — a malicious/buggy oracle runs with full privilege on apply.** Dorc-is-a-package-manager: it inherits
  Galaxy's become-escalating exec + mutable versions [B-nesbitt-quacks-package-manager-2026]. **Eliminate**
  the registry surface (no code-fetching — `plans/101` front-1; obtain via `git`). **Transfer**-to-user
  "review before use" is *unreliable* (real users don't read shell) → **mitigate** with **defensive linting**
  [B-shellcheck-readme-2026]: best-effort pattern-filter for the accidental footguns (unquoted vars, `eval`,
  `curl|sh`), *not* a malice-detector. **Accept** the residual: a `git clone`d-and-run malicious oracle is
  "you ran a shell script" — the same trust model as all shell; be honest, don't over-claim.
- **T — oracle integrity in transit** (force-pushed git tag → backdoored commit). **Mitigate**: pin oracle
  refs to **commit SHAs**, not mutable tags (the content-addressed integrity idiom that survives "cede to
  git" — the parked `kTRUST` seam; reserve the SHA-pin convention now, it's cheap).

### E5 · The plan output (sanitized shell displayed to the operator TTY)
- **T / I — terminal-escape injection via displayed plan-as-shell.** A hostile oracle or an adversarial
  remote *filename* carries terminal control sequences that execute/forge when printed to the operator's
  terminal [A-wheeler-filenames-in-shell-2025]. **Eliminate**: strip/escape control chars before display.
- **T — codegen quoting bugs** when Dorc emits sanitized probes/plans → injection. **Mitigate**: disciplined
  template-quoting; treat the codegen boundary as trusted and test it (`kVERIFY`). Cheap now, expensive to
  retrofit (front-5).

## Cross-cutting · Version drift breaks the grounding (flagged for a separate spike)
The hazard *under* E3/E4 the human asked to surface: an oracle's grounding ("`docker --dry-run` is inert")
is implicitly parameterised by **the exact binary it was authored against**, but the host's binary may
differ — and **"version" is a treacherous coordinate**. The *same* version string routinely names *different
bytes and behaviour* (distro backports, build divergence): [C-seal-versioning-ghost-2026] measured >10,000
Alpine package-string/hash collisions (musl `1.2.5-r9` with-vs-without the CVE-2025-26519 fix; Debian-vs-
Ubuntu `VLC 3.0.16` patch divergence). So pinning "docker 2.x" does **not** pin behaviour — `docker@3` adding
auto-install-on-`--dry-run` silently flips an inert probe into a mutator. This is the concrete mechanism by
which an *honest* author's grounding goes wrong (E3), breaking **both** load-bearing claims (read-only AND
oracle-soundness) without announcing itself.
- **Version strings can't fix it** [B-matrixai-content-addressed-vs-semver-2016]: "no direct relationship
  between a semantic version tag and the actual contents ... `1.2.3` only enforces that the package *says*
  it's `1.2.3`." Semver is *intent metadata*, not identity.
- **Candidate defence — content-hash identity, two lightweight no-registry moves:** (1) **gate the grounding
  on a binary content-hash** — oracle declares "grounded against `jq@<hash>`"; runner hashes `$PATH` at
  probe/apply; mismatch ⇒ grounding unverified ⇒ **fail-safe** (`kFAIL-withhold`/warn/re-probe). The whole
  distro-patch matrix collapses to one byte-comparison. (2) **invoke-by-content-hash, not `$PATH`** (Nix-
  store style) — jointly defensive against **injection** (a planted `$PATH` binary can't hijack) *and*
  **drift** (run the grounded bytes or fail). The human's "defensive against both version and injection"
  checks out.
- **No-registry tension, resolved toward the light end:** full content-identity *à la* Nix = owning a
  content-addressed store = the heavy pole the human doesn't want; but *record-host-hash-and-compare*
  (observe + gate, never store/serve binaries) needs **no registry**. The git-SHA idiom already covers the
  *oracle* side (lev-4; git-commit-hash is "a viable alternative for people not using NixOS"
  [B-matrixai-content-addressed-vs-semver-2016]); binary-hash gating is its host-observation analog.
- **Two distinct jobs, don't conflate:** the parked **version-lattice** (semver as *comparable intent* — "is
  3.x newer/compatible?") vs **content-hash identity** ("is THIS the byte-exact binary the oracle was
  grounded against?"). Complementary — lattice for hints/UX, hash for the soundness gate.
- **Status: parked for a dedicated spike** (human's call; sensitive to the registry tension). This model
  only *records* that version-drift is the concrete grounding-breaker and content-hash-gating the lightweight
  no-registry candidate. Spike-leads: Nix/Guix store + invoke-by-store-path; reproducible-builds.org; PURL
  distro-coordinate [C-seal-versioning-ghost-2026]; POLLUX (syscall-signature behavioural diff = "did the
  grounding break?"); ties to `plans/078` (trace) + parked `kTRUST`/version-lattice.

## Premortem (Q2, the other lens — "it's 2027, Dorc caused a breach; what happened?")
The narratives that STRIDE-per-cell tends to under-weight; each maps to a banked decision below.
- **PM-1 (E1):** operator laptop popped by unrelated malware; attacker found a live `ForwardAgent` socket /
  cached fleet key → pushed a malicious book to every host. → ProxyJump-only, no agent-forward, short-TTL certs.
- **PM-2 (E4-T):** a popular oracle's `@main` tag was force-pushed to a backdoor; everyone who re-pulled ran
  it as root. Defensive lint flagged neither (subtle, not a `curl|sh` pattern). → SHA-pinning; honest lint limits.
- **PM-3 (E3-D):** a probe ran `docker inspect` against a wedged dockerd and hung the fleet during a
  *read-only* plan. → read-only≠non-blocking; timeout/rlimit.
- **PM-4 (E3-T):** an oracle's probe-section shelled out to a vendor installer that apt-installed on
  `--dry-run` → mutation in the no-mutation phase, because the author's grounding was wrong and Dorc trusted
  it. → it's the author's contract, not Dorc's guarantee; fail-safe withhold + `077`/`078` trace catch *some*;
  document the boundary, don't market non-mutation as sound.
- **PM-5 (E2-S):** a managed host's name was DNS-spoofed; Dorc pushed a secrets-bearing book to the imposter.
  → host-key verification; integrity-over-secrecy.
- **PM-6 (version drift):** an oracle grounded `docker@2 --dry-run` as inert; the host ran `docker@3`, which
  auto-installs-on-missing under `--dry-run`. The "read-only" probe mutated — author honest, binary changed
  underneath. → content-hash-gate the grounding; pin binary identity, not the version string.

## Byproduct — the per-oracle threat-model template (front-6; no-cliff, fillable by a non-expert author)
A short checklist an oracle author answers **in sh-idiom terms**, not YAML (the `kBURDEN` gradient — a
deployer skips it; an engineer fills it):
1. *What does each leaf touch?* (state, files, network, other hosts) — the Dorc-flavored Q1.
2. *Does any **probe-phase** leaf shell out / open a socket / take a lock?* If yes it is **not inert** —
   guard it or mark it apply-only. **This is yours to get right:** Dorc's withhold only catches what it can
   ground; non-mutation is your contract, not Dorc's guarantee. [A-chef-whyrun-harmful-2018]
3. *Does the oracle run anything as root a guard should gate?* Write the guard in sh. [B-nesbitt-quacks-package-manager-2026]
4. *Does any output embed an attacker-influenceable filename/string?* Quote + strip control chars.
   [A-wheeler-filenames-in-shell-2025]
5. *Is the oracle pinned by SHA when shared?* [B-nesbitt-quacks-package-manager-2026]
6. *Is the grounding pinned to a **binary identity** (content-hash), not just a version string?* The same
   version string can name different bytes across distros/patch-levels. [C-seal-versioning-ghost-2026]

## Banked footgun-avoidance — cheap now, expensive to retrofit (the leverage)
1. **ProxyJump-only, agent-forwarding-off default** (E2). [B-heipei-ssh-agent-forwarding-harmful-2015]
2. **Strip terminal control chars before displaying plan-as-shell** (E5). [A-wheeler-filenames-in-shell-2025]
3. **Probe-runner leaf seam must support fail-safe withholding + the `077` socket-observe** (E3) — it already
   must be a wrappable seam (`plans/077`); withhold-on-unprovable-inert is the same hook. *Best-effort, not a
   guarantee* — probe-non-mutation is the author's contract; Dorc only helps honor it. [A-chef-whyrun-harmful-2018]
4. **Reserve the SHA-pin oracle-ref idiom** (E4) — the integrity convention that survives no-registry. [B-nesbitt-quacks-package-manager-2026]
5. **Host-key verification on push, never blind-accept** (E2). [A-cfengine-security-trust-model-2001]
6. **No feature concentrating fleet creds beyond the welded operator-node minimum** (E1). [A-fsecure-saltstack-bypass-2020]
7. **Reserve the content-hash-gate seam** (Cross-cutting) — let an oracle bind its grounding to a binary
   content-hash and the runner hash-check `$PATH` at probe/apply: lightweight, no registry, joint anti-drift
   + anti-injection. Deferred to a spike; the *seam* is cheap to reserve now. [B-matrixai-content-addressed-vs-semver-2016]

## What this pass deliberately did NOT do
- No product audit / pen-test (Q4 → `kVERIFY` harness, not here).
- Registry/identity/version-lattice: out-of-scope, parked (`plans/101` front-1).
- The privileged tracer's own attack surface: deferred with it (`plans/078`).
→ Next: (c) fold the banked list into implementation-shaped, per-component findings (TODO
"preparation-for-agentic-implementation").
