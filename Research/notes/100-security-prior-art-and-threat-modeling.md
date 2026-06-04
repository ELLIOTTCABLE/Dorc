# 100 — Security prior-art & threat-modeling (round 10, 2026-06-02)

Round-10 spike: **security prior-art**, threat-modeling-first, per the TODO item ("deep
security-dive on language/analysis + orchestrator; specifically distrust the seccomp-in-core
'secure-by-design' claim"). Practitioner/industry-weighted (the human's steer: trust high-effort
infosec/ops writing over theory here). 13 sources graded into `../sources.json`; map +
fronts in `../plans/101-security-threat-modeling-map.md`. This file = the raw findings + verbatim
citations. Source slugs carry grade+year; cite as `[slug]`.

## Findings (most load-bearing first)
- **The probe-non-mutation contract has a first-party prior-art refutation.** Chef's "why-run
  considered harmful" names Dorc's central hazard twice [A-chef-whyrun-harmful-2018]: (1) a dry-run
  can't evaluate a guard whose truth depends on a *prior leaf's real execution* (`only_if 'rpm -q
  httpd'` after an install) → mis-counts/mis-elides — this is the TODO elision-soundness hazard,
  generalised from `set -e` to *all* inter-leaf guard dependencies; (2) "no-op modes are **not**
  side-effect-free" — read-only `systemctl` interrogation locked up production hosts. ⇒ "non-mutative"
  ≠ "side-effect-free"; the `kFAIL-withhold` weld guarantees less than its name. Terraform's analog:
  data sources execute on *plan* [B-terraform-external-plan-exec-2017].
- **Control-node compromise = whole-fleet RCE** (SaltStack CVE-2020-11651/52: unauth → root on master
  AND every minion; 6000+ masters internet-exposed) [A-fsecure-saltstack-bypass-2020]. Push/agentless
  *structurally avoids the persistent listening master daemon* — a real, articulable win — but
  relocates the crown-jewel to the operator workstation (fleet-wide SSH keys; agent-hijack pivots
  [B-embracethered-ssh-agent-hijack-2022]). Counter-thesis: CFEngine pull/voluntary-cooperation bounds
  blast radius [A-cfengine-security-trust-model-2001].
- **seccomp is a syscall *filter*, not a *sandbox*** (Docker default blocks ~44/300+ syscalls, "compat
  not containment"; ptrace bypasses it) [B-docker-seccomp-docs-2026]. `plans/077`'s use (observe
  `socket(AF_INET)`) is defensible *because observe-only*; the TODO skepticism is warranted against any
  reading of it as containment for untrusted code. Hand-authored filters are incomplete by construction
  [B-jessfraz-seccomp-profiles-2016].
- **Dorc *is* a package manager and inherits all six supply-chain problems** ("if it has transitive
  execution, it's a package manager" [B-nesbitt-quacks-package-manager-2026]). Ansible Galaxy — the
  direct analog — shipped opt-in/off-by-default integrity, overwritable mutable versions, and
  become-escalating execution, and can't retrofit a lockfile (open since 2016).
- **Methodology answer: democratized STRIDE-per-element + Four-Questions, framed by a premortem — NOT
  expert attack-trees.** Shostack himself deprecates attack-trees [B-shostack-threat-modeling-guide-2022];
  the Manifesto's *Hero-Threat-Modeler* anti-pattern says everyone can/should TM
  [B-threat-modeling-manifesto-2020] — which maps onto Dorc's no-cliff/two-user constraint. Keep
  Schneier attack-tree *reuse* [A-schneier-attack-trees-1999] only as the oracle-composition lens;
  Klein premortem [C-klein-premortem-author-page-2007] as the kickoff.
- **Terminal-escape injection through Dorc's own plan-as-shell CLI output** (Wheeler: filenames carry
  "escape sequences that can execute commands when displayed") [A-wheeler-filenames-in-shell-2025] —
  a concrete footgun unique to the print-shell-to-terminal UX. Cheap to bake in (strip control chars),
  expensive to retrofit.
- Practitioner consensus is *pro*-threat-modeling; the "TM is theater" counter-search returned only
  low-grade SEO — itself a finding (the real debate is lightweight-vs-heavy, already covered by the
  Manifesto anti-patterns).

## Citations
> [A-chef-whyrun-harmful-2018]:§"Why-Run Mode Doesn't Work" (relevance: +1:SURE)
> no-op modes by definition can only observe resources in isolation ... This is especially problematic
> when guards are used that cause inter-resource dependencies. [only_if 'rpm -q httpd' example] Why-run
> mode will infer that only a single resource is going to change ... Running this recipe for real
> changes two resources.
> despite the name, no-op modes are not side-effect-free against systems generally ... a buggy version
> of systemd that would occasionally lock up when interrogated about the state of running services even
> though "no changes were being made".

> [B-terraform-external-plan-exec-2017]:issue+comment (relevance: +1:SURE)
> Actual Behavior: The script is executing on plan. — [maintainer] a data source will, by default,
> happen during the plan phase, so Terraform's usual guarantees about not doing any actions during plan
> will not apply here.

> [A-fsecure-saltstack-bypass-2020]:§Description/§Remediation (relevance: +1:SURE)
> bypass all authentication and authorization controls ... full remote command execution as root on
> both the master and all minions that connect to it. ... the authentication and authorization controls
> provided by Salt are not currently robust enough to be exposed to hostile networks. [§Detection] over
> 6,000 instances of this service exposed to the public Internet.

> [A-cfengine-security-trust-model-2001]:§"Why trust cfengine?" (relevance: +1:SURE)
> Cfengine ... trusts the integrity of its input file and any data which it explicitly chooses to
> download ... the worst case scanario an outside attacker could spoof cfengine into configuring the
> host correctly. In short, no one except root@localhost can force cfengine to do anything.
> [§Trust] If you do not understand where you are placing your trust, your trust can be exploited ...
> encrypted connections ... improve the privacy of the data ... not their accuracy or trustworthiness.

> [B-docker-seccomp-docs-2026]:§"Pass a profile" (relevance: +1:SURE)
> The default seccomp profile ... disables around 44 system calls out of 300+. It is moderately
> protective while providing wide application compatibility. [ptrace] Blocked in Linux kernel versions
> before 4.8 to avoid seccomp bypass.

> [B-jessfraz-seccomp-profiles-2016]:body (relevance: -0:SUSPECT)
> [strace-derived profile] missed some ... it missed 6 ... unshare and setns were missing ... Obviously
> noone else is going to do this, debug for hours the syscalls that are missing.

> [B-nesbitt-quacks-package-manager-2026]:§"transitive execution"+§"Ansible Galaxy" (relevance: +1:SURE)
> Once a tool develops transitive dependencies, it inherits ... Reproducibility ... Supply chain
> amplification ... Override and exclusion ... Mutable references ... Full-tree pinning ... Integrity
> verification. If your tool has these problems, it's a package manager.
> [Galaxy] checksums ... both are opt-in and off by default ... versions ... can be overwritten by the
> publisher ... Roles execute with the full privileges of the Ansible process with become ... escalating.

> [B-shostack-threat-modeling-guide-2022]:§"4 Steps"/§Attack-Trees (relevance: +1:SURE)
> Shostack's Four Question Framework: What are we working on? What can go wrong? What are we going to do
> about it? Did we do a good job? [STRIDE→property: Spoofing/Authenticity ... Elevation/Authorization]
> [Attack Trees — Deprecated] Since creating attack trees requires substantial security expertise,
> modern threat modeling experts use newer techniques that include a broader range of participants.

> [B-threat-modeling-manifesto-2020]:§anti-patterns (relevance: +1:SURE)
> [Hero Threat Modeler] Threat modeling does not depend on one's innate ability or unique mindset;
> everyone can and should do it. [Tendency to Overfocus] Avoid exaggerating attention on adversaries,
> assets, or techniques.

> [A-schneier-attack-trees-1999]:§"Creating Attack Trees" (relevance: +1:SURE)
> attack trees capture knowledge in a reusable form ... becomes part of a larger attack tree ... all you
> need to know are the values of the root node. [intro] "Secure from whom?" ... the areas people think
> of as vulnerable usually aren't.

> [A-wheeler-filenames-in-shell-2025]:§intro/§"Basic rules" (relevance: +1:SURE)
> filenames ... include almost any bytes ... the escape character (including escape sequences that can
> execute commands when displayed) ... Be careful about displaying or storing pathnames, since they can
> include newlines, tabs, terminal control escape sequences ... shellcheck can help ... but not all.

> [C-klein-premortem-author-page-2007]:body (relevance: -0:SUSPECT) [canonical primary = HBR 2007, paywalled]
> imagine that the proposed plan has failed ... everyone contributes ... potential threats and hurdles ...
> are generated. The goal ... is to increase plan success rate.

## Process note (round-10 tooling friction — for the human + future rounds)
Migrating these 13 from a scratch `.claude/research/` manifest into `../sources.json` burned
substantial time on a Windows-specific footgun: `for slug in $(mise exec -- jq -r 'keys[]' …)` yields
**`\r`-suffixed** slugs (jq emits CRLF on win32), so `.[$slug]` missed → produced the JSON value
`null` → `new-source.sh`'s `jq -e .` rejected it as "stdin is not valid JSON" (misleading; `null` *is*
valid JSON, just falsy). Fix applied to `new-source.sh`: the stdin check now distinguishes parse-error
from wrong-type and **echoes the payload** ("got JSON null (likely a wrong/empty slug lookup)").
Caller-side lesson: pipe `jq -r 'keys[]' | tr -d '\r'` on win32.

## Gate adjudication — human, 2026-06-02 (round-10 → b/c)
- **gap-2 is not a decision** (human): read-only is welded-forced (`kFAIL-withhold`); bounded-cost is
  inherently best-effort (can't statically bound a leaf's cost; read-only calls can still block — the
  systemd-lockup class [A-chef-whyrun-harmful-2018]). Honest probe contract = "read-only or withheld;
  cost/effect best-effort *flags*." Residual = a hazard to *name* + the existing `kPROBING`/`kPRECISION`
  proof-effort dial. (`../plans/101` front-2 rewritten.)
- **`kBLAST` → `kAGENTLESS`** (industry term), welded; ADDED to `../../KNOBS.md` welded section — named to
  keep the blast-radius cost of the welded push choice visible (operator-node crown-jewel; multi-hop
  bastion trust). Not a live knob.
- **Registry / `kTRUST`: out-of-scope, parked.** Human aware of the workload + security-responsibility
  dangers. Code-distribution cedes to `git clone` (do-one-thing-well). Distinct from the separately-planned
  **version-lattice**: first-class "version" as a comparable lattice over foreign registry version-strings
  (a meta-registry for canonical comparison, not a code source — e.g. an `npm` oracle declares `semver` as
  its domain's correct parser).
- **Defensive linting is the realistic supply-chain backstop** (human: real users don't read shell). The
  front-1 "you ran a script you read" framing is too optimistic; shellcheck-grade + Dorc-specific security
  lints (back-of-mind, "about the best we can do"). Ties front-1 ↔ front-5 (Wheeler injection
  [A-wheeler-filenames-in-shell-2025]).

## (b) threat-model gather — 2 new primaries (2026-06-03)
Front-3 and front-1 were the only genuinely-uncovered points (front-2's static-non-inert prior-art is
already in-base: CoLiS×3, smoosh, morbig, verified-interp). Two primaries close them; deliverable =
`../plans/102` (Dorc-system threat-model).
- **Secure multi-hop = ProxyJump, never ForwardAgent** [B-heipei-ssh-agent-forwarding-harmful-2015]: a
  compromised intermediate hop with ForwardAgent can both *impersonate* you fleet-wide (agent socket) and
  *eavesdrop/modify the live session*; ProxyCommand/ProxyJump terminates the session on the operator
  workstation, so intermediate hops are blind TCP relays. ⇒ Dorc's m→n→o default (`kAGENTLESS`) must be
  ProxyJump-style, agent-forwarding off.
- **Defensive linting is a pattern-filter, not a malice-detector** [B-shellcheck-readme-2026]: shellcheck
  (AST/pattern) catches the accidental injection/quoting footguns (Gallery: Quoting, Robustness — the
  Wheeler class) but cannot certify an unread third-party oracle safe (no cross-command taint; evolving
  check-set). Confirms front-1's "about the best we can do."

## Citations (b)
> [B-heipei-ssh-agent-forwarding-harmful-2015]:§"The problem"/§"Updates" (relevance: +1:SURE)
> anyone with sufficient permission on host A will be able to use that socket to connect to and use your
> local ssh-agent ... impersonate you to any host as long as you're connected to host A.
> If an attacker compromises Host A he ... is actually able to eavesdrop on your ongoing session ... the
> attacker could read and modify everything about your session. ProxyCommand will prevent this ... the SSH
> session to Host B terminates on your workstation!

> [B-shellcheck-readme-2026]:§goals/§Gallery (relevance: -0:SUSPECT)
> To point out subtle caveats, corner cases and pitfalls that may cause an advanced user's otherwise
> working script to fail under future circumstances. [Gallery of bad code: Quoting · Robustness ·
> Frequently misused commands] ... mostly intended for interactive use.

## (b+) version-matching survey — Exa/Kagi, for the parked versioning spike (2026-06-03)
Human flagged version-matching (oracle@hash ↔ the-binary-it-covers) as possibly-novel + security-relevant.
Quick survey: the *components* are well-grounded prior art; the *composition* — a third-party oracle's
grounding gated by the host-binary's content-hash, on an UNMANAGED host — is the novel Dorc angle. Feeds a
parked spike; deliverable = `../plans/102` "Cross-cutting · Version drift" + PM-6 + lev-7.
- **Version strings are not identity** [B-matrixai-content-addressed-vs-semver-2016]: semver = intent
  metadata; content-hash = identity; git-submodule commit-hash = the no-NixOS content-address route (= the
  Dorc SHA-pin idiom). Content-hash gates soundness; semver-lattice gives comparable intent — two jobs.
- **Same version string → different bytes/behaviour is measured & systemic** [C-seal-versioning-ghost-2026]:
  >10k Alpine string/hash collisions; Debian-vs-Ubuntu backport divergence; PURL distro-coordinate as partial
  fix. (Commercial vendor; the *measurement* is the value, the framing sells SCA.)
- Spike-leads (surfaced, not yet full-read/graded): Nix/Guix content-addressed store + invoke-by-store-path
  (Tweag 2024); reproducible-builds.org (bit-for-bit + abort-on-mismatch); POLLUX (syscall-signature
  behavioural diff across binary versions — "did the grounding break?"); jmmv Bazel-glibc (version-skew
  invisible to cache keys — the exact "grounding invisible to the version" analog).

## Citations (b+)
> [B-matrixai-content-addressed-vs-semver-2016]:body (relevance: +1:SURE)
> there's no direct relationship between a semantic version tag and the actual contents of the dependency ...
> The only constraint that 1.2.3 is enforcing, is that the package must say it's 1.2.3. However the package
> could have changed in the meantime, while spoofing the package version.
> ... for proper content addressable dependencies, you should prefer the commit hash ... Semantic versioning
> is still very useful as a metatag ... But it's not something we should be relying on when building
> production software.

> [C-seal-versioning-ghost-2026]:body (relevance: +1:SURE)
> a single package version can represent entirely different security states depending on the underlying
> operating system ... In Alpine 3.21, version 1.2.5-r9 contains security fixes for CVE-2025-26519. Yet, in
> the early builds of 3.22, the same version string was used for a build that lacked those fixes ... over
> 10,000 instances where the same version string yielded different hashes ... over 1000 "version collisions"
> remained where the actual patches differed.
