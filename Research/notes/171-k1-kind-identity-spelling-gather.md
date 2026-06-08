# 171 — K1: cross-referent kind-identity spelling — prior-art gather (round 17, 2026-06-07)

> Firewalled **K1** thread (entity-identity / equivalence / ontology); charter `plans/170`. This is the **raw graded findings log**; the deliverable **map-with-a-lean**
> is `plans/175-cross-oracle-kind-channel-synthesis.md`. Sources graded into `../sources.json` via `new-source.sh`.
> AI-generated; confidence-marked; trust repo-root `DESIGN`/`KNOBS` over this. Per the brief I'm
> *upstream* of the soundness math (charter A3–A6) — this is about **spelling the anchor**.

## Findings (lifted, most-load-bearing first)

- **k1-shape-universal (+SURE).** Dorc's A1 shape — a named **kind** + a pluggable **provider** +
  an instance **name**, with the system resolving `(kind,provider,verb)→effect` — is the **mainstream
  convergent design**, not a novelty. Independently re-invented by Debian virtual-packages
  [A-debian-policy-relationships-2024], `update-alternatives` slots [B-update-alternatives-man-2024],
  RPM `Provides`, Ansible's generic `package:` module [B-ansible-package-module-2024], Puppet's
  Resource Abstraction Layer (types+providers) [B-puppet-ral-resource-2024], K8s `kind`+selectors,
  Terraform resource-type+provider. The *shape* is de-risked by overwhelming prior-art.

- **k1-two-regimes (+SURE) — the central result.** The world's kind-identity spellings split by
  **scope of authority**, and the split is sharp:
  1. **WITHIN one authority/ecosystem**, identity is declared **in-band, in the artifact you write
     anyway, and pays for itself** with *multiple independent consumers*: Debian `Provides:`
     [A-debian-policy-relationships-2024], systemd `Alias=`/`Wants=` [B-systemd-unit-man-2024],
     `update-alternatives --install … editor` [B-update-alternatives-man-2024]. These BEAT the
     comment-floor.
  2. **ACROSS authorities/ecosystems** ("apt's nginx ≡ brew's nginx"), in-band self-paying identity
     **does not exist anywhere** — it is reconstructed by a **central, curated index**: repology's
     rule-DB [B-repology-rules-2024], purl2cpe, CISA's "funded authority" [A-cisa-software-id-ecosystem-2023].
     The one in-band cross-referent primitive, `owl:sameAs`, is famously **misused**
     [A-halpin-owl-sameas-2010].

- **k1-grouping-is-the-unsolved-half (+SURE).** [A-cisa-software-id-ecosystem-2023] (US-gov ecosystem
  analysis) independently states Dorc's exact problem at ecosystem scale: identity needs **precision
  AND grouping**, *no* identifier delivers both, and "inherent identifiers that support grouping remain
  an **unsolved engineering challenge**." Per-token identity is cheap; **grouping tokens into a named
  kind is the hard part** — and the surveyed world's only working answer is **central curation /
  authority** (≡ `X3`, ≡ repology). Maps 1:1 onto `094 g4` (co-reference ≠ grounding).

- **k1-X3-confirmed-in-the-wild (+SURE).** No successful in-band system carries the 3-place relation in
  a **function-name** namespace (the `notes/151 X3` clobber). Every one uses a **declarative field/call
  that an external resolver lifts into an index**: `Provides:` → apt's solver; `--install` →
  alternatives DB; `package:`/`type` → Ansible/Puppet provider registry. This is direct prior-art for
  the **A2 `kOOB` analyzer-internal lifted index** — it is what the entire ecosystem actually does.

- **k1-floor-confirmed (+SURE).** ShellCheck [B-shellcheck-directive-2026] is the comment-floor
  validated in the wild: `# shellcheck disable=SC2059` (comment) and `.shellcheckrc` (sidecar) — exactly
  the `# dorc:` + YAML forms `kOOB` forbids. Decisive point: they have **zero value outside ShellCheck**.
  The config-mgmt sidecars (Ansible/Puppet/K8s/TF) "pay" only *inside their own tool* — same failure as
  the comment. The K1 bar "pays for itself **without Dorc**" really means **has independent consumers
  already**; only regime-1 declarations clear it.

- **k1-read-existing-metadata (+SURE) — the strongest independent-value answer.** The maximal "pays for
  itself" is **zero new annotation**: the kind-identity declarations *already exist* in the system for
  the OS's own reasons — systemd unit relationships [B-systemd-unit-man-2024], the dpkg/rpm
  `Provides` databases [A-debian-policy-relationships-2024], pkg-config `.pc`, `.desktop`/mailcap. An
  oracle **reads** these (its `dpkg -s nginx` fact-probe already does) rather than asking the user to
  write anything. The kind-NAME lives in the **oracle author's code** (lifted into the index), grounded
  in real metadata — not in a user-written tag. This is the cleanest beat-the-floor route.

- **k1-hindsight-verdicts (+SURE).** How the approaches aged: **CPE** (top-down global identity) =
  widely reviled, false-positive-ridden, curation-bottlenecked [A-cisa-software-id-ecosystem-2023]. **purl**
  (bottom-up, provider-qualified) = won (ECMA-427) precisely because it identifies *within* a provider
  and **refuses to assert cross-provider sameness** [B-purl-spec-intro-2025]. **owl:sameAs** = binary
  "same" is "only one point on a scale of similarity, often too strong" → use graded/contextual
  [A-halpin-owl-sameas-2010]. **systemd** = "use [explicit] dependencies only sparingly," prefer
  *implicit/inferred* [B-systemd-unit-man-2024]. **microformats2** = reuse-existing-markup (`class=`)
  beat heavier new-attribute schemes (RDFa/microdata) [B-mdn-microformats-2024]. **suffix→pattern
  rules** = structural recognition generalized [B-gnu-make-pattern-rules-2024].

- **k1-annotation-in-artifact (+SURE).** [B-mdn-microformats-2024] is the cleanest model of Dorc's own
  stated goal (AGENTS "annotation-by-narrowing; spelled in sh"): you add `class="h-card"` to the `<a>`
  you were already writing; it **renders normally** *and* yields machine semantics (SEO rich-snippets,
  Webmention, syndication) read by *many independent consumers*. Independent value via the artifact, not
  a sidecar. The direct HTML-analogue of "write the idempotency guard you'd write anyway; Dorc lifts the
  spec."

- **k1-too-strong-caution (~SUSPECT).** From [A-halpin-owl-sameas-2010]: do **not** make kind-identity a
  hard binary global `sameAs` (too strong, misused, philosophically the identity/reference problem).
  Dorc's relational 3-place `(kind,provider,verb)→effect` (A1), scoped to an authority/library, is the
  *right* weaker shape; raw token-`sameAs` is the trap `094 g4` already named. **Flag for K2 synthesis**
  (the named kind as nominal type): the type's equality should be authority-scoped, not global.

## Front-by-front (neutral; ranking + lean are in plans/175)

- **f1-provides** — STRONG. Debian virtual-packages, update-alternatives, RPM `Provides`, pkg-config.
  The decades-old, self-paying, in-band, solver-lifted capability declaration. Best regime-1 prior-art.
- **f2-crossmanager** — STRONG (as a *cautionary* + *curation* finding). purl/repology/CPE/CISA: the
  cross-provider equivalence is central-curated, an unsolved-in-band problem with clear verdicts.
- **f3-ontology** — MIXED. owl:sameAs = the explicit primitive + its misuse verdict; microformats =
  the in-artifact independent-value model. RDFa/microdata = heavier, lost to reuse-existing.
- **f4-sh-idiom** — STRONG (lean-bearing). systemd metadata + read-existing-metadata + oracle-as-real-
  library; the X3 namespacing constraint forces declarative-field-lifted-to-index, not function-names.
- **f5-recognition** — MODERATE. make pattern rules (structural kind w/o config), file(1)/libmagic
  (kind-from-content, central DB), ShellCheck `source=` (recognition-assist). Recognize-by-structure.

## Counter-thesis surveyed (A2 demand: report even if unwelcome)

The whole config-management world (Ansible [B-ansible-package-module-2024], Puppet
[B-puppet-ral-resource-2024], K8s, Terraform) shares **one declaration form**: a typed-resource +
pluggable-provider model spelled as **sidecar config** (YAML / Puppet-DSL / HCL). This both (a) validates
Dorc's A1 *shape* overwhelmingly and (b) confirms the in-band-sh *spelling* is the genuinely novel part —
**nobody** does the typed-resource model in-band; they all use a config DSL. Puppet's RAL is the closest
whole-model prior-art (types+providers + `puppet resource` reads current state into code — a bidirectional
probe↔apply). Reported as the brief requires; the `kOOB` lean weighs against adopting the sidecar form.

## Citations (verbatim; [slug]:loc, relevance = cite-specific certainty)

> [A-debian-policy-relationships-2024]:§7.5 (relevance: +1:SURE)
> A virtual package is one which appears in the Provides control field of another package. The effect is
> as if the package(s) which provide a particular virtual package name had been listed by name everywhere
> the virtual package name appears.
> … all mail transport agents (MTAs) would have the following fields in their control files:
> Provides: mail-transport-agent / Conflicts: mail-transport-agent / Replaces: mail-transport-agent

> [B-update-alternatives-man-2024]:DESCRIPTION (relevance: +1:SURE)
> A generic name in the filesystem is shared by all files providing interchangeable functionality. … if
> the text editors ed(1) and nvi(1) are both installed … the alternatives system will cause the generic
> name /usr/bin/editor to refer to /usr/bin/nvi by default. … Different packages providing the same file
> need to do so cooperatively. … the usage of update-alternatives is mandatory for all involved packages.

> [B-repology-rules-2024]:README (relevance: +1:SURE)
> Rename or merge rules. … merge differently-named packages into the same project. For example,
> `etracer`, `extremetuxracer`, `extreme-tuxracer` → `extreme-tuxracer`.
> Split rules. … split similarly-named packages of different projects. For example, `clementine` →
> `clementine-wm`, `clementine-player`.

> [A-cisa-software-id-ecosystem-2023]:Executive Summary (pp.2-3) (relevance: +1:SURE)
> The two key requirements … are: 1. Timely availability of software identifiers … 2. Software
> identifiers that support both precision and grouping. … no existing software identifier adequately
> meets both requirements.
> While inherent identifiers provide optimal availability, inherent identifiers that support grouping
> remain an unsolved engineering challenge.
> An authority that establishes common rules … would likely improve the overall accuracy and robustness
> … Without a global authority, the operational effectiveness of the ecosystem will depend on the
> individual efforts of the identifier generator community.
> [purl's] challenge … is adoption in software development communities besides package managers.

> [A-halpin-owl-sameas-2010]:Abstract (p.1) (relevance: +1:SURE)
> owl:sameAs can be viewed as encoding only one point on a scale of similarity, one that is often too
> strong for many of its current uses. … shed light upon how owl:sameAs is being used (and misused) on
> the Web of data.

> [B-systemd-unit-man-2024]:Automatic dependencies / Alias (relevance: +1:SURE)
> while systemd offers a flexible dependency system between units it is recommended to use this
> functionality only sparingly and instead rely on techniques such as bus-based or socket-based
> activation which make dependencies implicit …
> [service units with Type=dbus automatically acquire dependencies of type Requires= and After= on
> dbus.socket.]  ·  Alias names may be used … in all unit dependency directives, including Wants=,
> Requires=, Before=, After=.

> [B-mdn-microformats-2024]:How Microformats Work (relevance: +1:SURE)
> <a class="h-card" href="https://alice.example.com">Alice Blogger</a> … the parser … will know that
> this page contains a "card" which describes a person … All major search engines support reading and
> interpreting microformats … render special snippets such as star ratings.

> [B-purl-spec-intro-2025]:Introduction (relevance: +1:SURE)
> PURL (Package-URL): a standard URL-based syntax to identify software packages, independent from their
> ecosystem or distribution channel … PURL was approved by Ecma International as ECMA-427 in December 2025.
> └ note: purl identifies WITHIN an ecosystem (pkg:deb/debian/nginx); it does not assert cross-ecosystem
>   sameness — that layer is repology/purl2cpe (see k1-two-regimes).

> [B-ansible-package-module-2024]:Synopsis (relevance: +1:SURE)
> This modules manages packages on a target without specifying a package manager module (like
> ansible.builtin.dnf, ansible.builtin.apt, …). … ansible.builtin.package calls behind the module for
> [each system]. (generic kind=`package`; provider auto-selected from the `ansible_pkg_mgr` fact.)

> [B-puppet-ral-resource-2024]:DESCRIPTION (relevance: +1:SURE)
> puppet-resource - The resource abstraction layer shell … Uses the Puppet RAL to directly interact with
> the system. … provides simple facilities for converting current system state into Puppet code, along
> with some ability to modify the current state. (type + namevar + per-host provider; probe↔apply.)

> [B-gnu-make-pattern-rules-2024]:§10.5 (relevance: -0:SUSPECT)
> A pattern rule looks like an ordinary rule, except that its target contains the character `%` … Thus, a
> pattern rule `%.o : %.c` says how to make any file stem.o from another file stem.c. (structural
> kind-transformation; no per-file annotation. POSIX suffix rules `.c.o:` were the built-in precursor.)

> [B-shellcheck-directive-2026]:Supported directives (relevance: +1:SURE)
> Shellcheck directives … can be added: 1. As entries in a .shellcheckrc file … 2. and/or as comments
> inside individual script files: `# shellcheck disable=SC2059`. … Otherwise, they are scoped to the
> command that follows it (including compound commands like function definitions, loops, and case
> statements). (THE FLOOR: comment + sidecar, zero value outside ShellCheck.)

## Open / next
- Map + lean → `plans/175-cross-oracle-kind-channel-synthesis.md` (this round stops at the map; human takes the lean
  into adversarial-crosscheck per charter §Sequencing).
- Light/untaken leads (declared, not chased): RPM `Provides` primary (corroborates Debian, not read in
  full); pkg-config `.pc` primary; `.desktop`/mailcap MIME-handler kinds; tree-sitter/semgrep/CodeQL
  pattern-recognition (f5); file(1)/`magic` primary (man5 URL 404'd — referenced as common knowledge).
- Firewall held: no PLT/typing material pulled; entity-identity only.
