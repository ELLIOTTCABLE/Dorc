# turn02/f4 — comparison-keys + rot-governance rider (gatherer: Opus subagent, 2026-07-11)

Scope: f4-comparison-keys (mid-narrow version-applicability window; comparison-keyed, purl/semver-shaped;
"which claim applies to version V") + light f5-rot-governance rider (freshness/expiry semantics only).
Central filter recorded per-source as `undershoots: yes/no` — does the prior art assume the versioner is
the AUTHOR of the thing versioned? (If yes, its lessons cap out below Dorc's third-party-describer need.)

## Findings

- f4-fd1 +SURE: Every system that PASSES the third-party filter refuses a universal cross-ecosystem version
  comparison and delegates the compare-relation to a per-ecosystem "type." purl states outright there is "no
  reliable and uniform way" to compare versions [B-purl-faq-no-uniform-compare-2025]; VERS makes the `type`
  (semver/npm/deb/…) define "how two versions are compared" [A-vers-range-specifier-2026]; OSV splits ranges
  into `SEMVER` (strict linear order), `ECOSYSTEM` (uninterpreted strings, "may not be able to be answered
  without reference to the package ecosystem's own logic"), and `GIT` (commit-graph reachability)
  [A-osv-schema-version-ranges-2026]. → external corroboration of no-join-across-managers and
  vers1-fd-compare-is-kind-owned: comparison is per-KIND, never global.
- f4-fd2 +SURE: The passing systems keep identity-key and version-window as SEPARATE axes, and one goes
  further and forbids mixing them: CVE's Quality Working Group mandates that purls carry no version —
  "Package URLs added via this new field will **not** be allowed to include versions. All version information
  should only be included within the existing `versions` field" [A-cve-qwg-expanding-software-ids-2025]. →
  direct precedent for making explicit-version-match a SEPARATE enablement axis from the coordinate/identity
  (vers1-position-comparison-keyed; the §4-relay "grounding/version qualifier segment").
- f4-fd3 +SURE: The third-party window grammar is a sorted event/SIGNPOST timeline, not an interval algebra.
  OSV events = sorted {introduced, fixed, last_affected, limit} with sentinels `"0"` (sorts-before-all) and
  `"*"` (infinity) [A-osv-schema-version-ranges-2026]; VERS calls constraints "signposts in the version
  timeline," canonically sorted, where the separators "do not mean 'and' or 'or'" and non-`!=` constraints
  must alternate `<`/`>` [A-vers-range-specifier-2026]. Both reduce a window to boundary-crossings over a
  per-type order.
- f4-fd4 +SURE: Third-party range claims demonstrably rot, and the machinery encodes it. OSV `last_affected`
  is "the hard ceiling of the vulnerability *at the time of publication*"; versions above are ASSUMED
  unaffected — an acknowledged false-negative that ages as new versions ship — so `fixed` is "overwhelmingly
  preferred" because it does not rot [A-osv-schema-version-ranges-2026]. OSV also warns range strings "are
  not guaranteed to exactly match versions of the package found in the upstream package repository"
  (describer normalization drift). → vers1-position-rot-asymmetry observed in the wild.
- f4-fd5 +SURE: The language-package-manager range grammars supply rich WINDOW SYNTAX but all UNDERSHOOT —
  they rest on the author versioning their own API. node-semver desugars caret/tilde/hyphen/X-ranges to
  comparator-set intersections `||`-unioned, and excludes prereleases from a range unless the range opts in
  on the same `[major,minor,patch]` tuple [B-node-semver-range-grammar-2020]; RubyGems `~> 2.2` ≡
  `>=2.2,<3.0` [B-rubygems-pessimistic-operator-2024]. Good spelling prior-art for the window shape; their
  *compatibility* meaning is inapplicable to Dorc's third-party oracle-authors.
- f4-fd6 ~SUSPECT: Tight upper-bounded windows are a KNOWN anti-pattern — a caution against a too-narrow
  sharp-knife gate. RubyGems' own guide now says "pessimistic versioning is often wrong in both directions"
  and that a pessimistic upper bound in a published gem causes transitive lock-in (over-exclusion)
  [B-rubygems-pessimistic-operator-2024]. → an over-tight explicit-version-match window would silently
  disable the sharp feature across churned-but-compatible versions — the same value-loss that got
  hash-keying rejected.
- f4-fd7 +SURE: Epoch escape-hatches and downstream-rebuild handling are universal in the sharpest passing
  schemes, and both target "same string, different meaning/bytes." dpkg EVR has `epoch:` to "leave behind"
  a changed numbering scheme and a tilde `~` that "sorts before anything, even the end of a part"
  [A-deb-version-evr-sorting-2025]; PEP 440 has epoch `N!` for scheme changes AND a local-version `+label`
  segment built explicitly for downstream integrators "backporting security and bug fixes," to differentiate
  "potentially altered rebuilds by downstream integrators" from upstream [A-pep440-python-versioning-2014].
  → the describer case (downstream ≠ upstream at the same upstream string) is designed INTO the schemes.
- f4-fd8 ~SUSPECT: rpmvercmp shows per-ecosystem comparators are NOT clean total orders. Behavior is
  "undefined" for labels not bounded by alphanumerics; uppercase sorts before lowercase (`FC5` < `fc4`),
  numeric beats alphabetic (`2a` < `2.0`), separators are ignored (`fc4` == `fc.4`)
  [B-rpmvercmp-fedora-quirks-2012]; a live bug has `rpmvercmp("5.3.0","5.3.0+") == 0` though the bytes
  differ. → delegating compare to a KIND inherits that KIND's pathologies; vers1-q-compare-home ("kinds with
  no total order") is a real cell, not hypothetical.
- f4-fd9 +SURE: CPE is the negative example the whole third-party world is fleeing, and the diagnosis is
  Dorc-shaped. CPE rots because it is an EXTRINSIC identifier bound to a central dictionary NIST must update
  per new product [C-tomalrich-cpe-on-way-out-2025]; it is "coarse-grained ... not components or materials
  within those software products," unable to express module/file-level applicability
  [A-cve-qwg-expanding-software-ids-2025]; <50% of CVEs since Feb 2024 even receive a CPE
  [C-tomalrich-cpe-on-way-out-2025]. purl (INTRINSIC — constructible from what you already know) is the
  chosen successor. → validates Dorc's no-registry weld and intrinsic-coordinate direction; the
  coarse-grained failure argues the version window must reach BELOW the tool into arms/paths
  (vers1-position-two-domains item 2).
- f4-fd10 ~SUSPECT: The counter-thesis holds the version-COMPARISON boundary itself is a lie. Hickey: "change
  is not a thing" — it is either growth (accretion) or breakage (removal); a major bump means only "you're
  screwed" and "does not tell you in what way," so SemVer "has got too broad a semantic"
  [B-hickey-spec-ulation-2016]. ZeroVer supplies the empirical tail: OpenSSL, Terraform/Vault, Elm,
  FastAPI, React-Native etc. sit at 0.x for a decade-plus, so the string encodes ~nothing
  [C-zerover-version-means-nothing-2018]. → a window like "applies to <2.0" smuggles a clean-fence assumption
  that often fails; Hickey's per-symbol growth/breakage framing independently ALIGNS with
  vers1-position-surfaced-contract (per-callsite version marks, not per-package numbers).
- f4-fd11 +SURE (undershoot ledger, the deliverable's meta-finding): the filter split is clean and
  load-bearing. PASSES (lessons transfer at full strength — third-party describers): OSV, VERS, purl,
  CVE-QWG, Tom-Alrich/CPE, dpkg-EVR, rpmvercmp, reproducible-builds/buildinfo. UNDERSHOOTS (author-versions-
  own-API, lessons cap out): semver.org, node-semver, RubyGems `~>`, Hickey, ZeroVer — plus PEP 440 EXCEPT
  its local-version segment (the one part authored FOR downstream describers). The transferable applicability
  core lives in the vuln-DB + distro-packaging worlds; the language-PM world contributes window SYNTAX only.

- f5-fd1 +SURE: TUF models freshness as a first-class, signed, EXPIRING property — the direct anti-rot
  primitive. A dedicated Timestamp role periodically re-signs a short-lived statement so clients detect
  "indefinite freeze" (stale-metadata replay), rollback, and fast-forward attacks; delegated roles sign
  SCOPED (partial-trust) claims and "any delegation can be revoked at any time" by signing new metadata that
  omits it [A-tuf-freshness-expiry-2024]. → working template for traveling-claim rot-governance: claims
  expire and must be re-vouched; scoped, revocable third-party delegation. (Transport/key-compromise
  machinery excluded per charter.)
- f5-fd2 ~SUSPECT: reproducible-builds/buildinfo is the "same version-string, different bytes" grounding
  record. buildinfo is a SEPARATE build-product recording the build environment so exact distributed bytes
  can be re-derived — Debian ships it as clearsigned plain-text distributed independently
  (buildinfo.debian.net); Arch embeds it in the signed package [B-reproducible-builds-buildinfo-2023]. →
  prior art for capture-at-authoring grounding (lockin-3): the version string is insufficient identity, a
  separate signed record makes "the binary I described" checkable. Contrast worth carrying: Debian's sidecar
  distribution is the OPPOSITE of Dorc's inline-pin lean (vers1-position-inline-pins).

## Citations

> [B-purl-faq-no-uniform-compare-2025]:faq.md "Version" section (relevance: +SURE)
> **QUESTION**: How do package **types** handle the comparison and sorting of versions? **ANSWER**: Some
> package **types** use versioning conventions such as SemVer for NPMs or NEVRA conventions for RPMs. A
> **type** may define a procedure to compare and sort versions, but there is no reliable and uniform way to
> do such comparison consistently.

> [A-vers-range-specifier-2026]:docs/standard/specification.md (relevance: +SURE)
> A **type** defines: - the specific notation and conventions used for a version string encoded according to
> this type - how two versions are compared to determine if a version is inside or outside a range ...
> The list of **constraints** strings for a range are like a set of signposts in the version timeline of a
> package. The separators do not mean "and" or "or". They are separators in a sequence of **constraints**.
> ... Ignoring all constraints with a '=' or '!=' **comparator**, the sequence of constraints must be an
> alternation of Greater-than and Lesser-than **comparators**
> (and, on prior art it is replacing:) For security advisories, the lack of a portable and compact notation
> for vulnerable package version ranges means that these ranges may be ambiguous or hard to compute and may
> be replaced by complete enumerations of all impacted versions, such as in the [NVD CPE Match feed].

> [A-osv-schema-version-ranges-2026]:ranges[].type + events + evaluation (relevance: +SURE)
> `SEMVER`: ... The relation `u < v` denotes the precedence order defined in section 11 of SemVer 2.0.
> Ranges listed with type `SEMVER` should not overlap ...
> `ECOSYSTEM`: The versions `introduced` and `fixed` are arbitrary, uninterpreted strings specific to the
> package ecosystem, which does not conform to SemVer 2.0's version ordering. ... `ECOSYSTEM` range inclusion
> queries may not be able to be answered without reference to the package ecosystem's own logic ...
> `GIT`: ... The relation `u < v` is true when commit `u` is a (perhaps distant) parent of commit `v`.
> Note that these version strings are not guaranteed to exactly match versions of the package found in the
> upstream package repository. For example, they may be normalized, or have build metadata stripped.
> `introduced` allows a version of the value `"0"` to represent a version that sorts before any other version.
> `limit` allows versions containing the string `"*"` to represent "infinity" ...
> `last_affected` should be thought of as the hard ceiling of the vulnerability *at the time of publication*
> in the absence of a fixed version. Versions above `last_affected` should be considered unaffected.
> Unfortunately this opens up the possibility for false negatives, which is why `fixed` is overwhelmingly
> preferred.

> [A-cve-qwg-expanding-software-ids-2025]:Problem Statement + Proposed Solution (relevance: +SURE)
> For CPE, the key challenges are its reliance on a central dictionary and the processes used to update that
> dictionary. ... the issuance of new CPEs for vendors or products not present in the dictionary requires
> NIST to update the dictionary to support them. ...
> Moreover, some vulnerability conditions cannot be expressed adequately using CPE. For example, sometimes a
> vulnerability is only present when certain modules or files are present, but CPEs do not capture software
> at the module or file level. ... CPE is a relatively coarse-grained software identifier ...
> Additionally, Package URLs added via this new field will __not__ be allowed to include versions. All
> version information should only be included within the existing `versions` field of the `product` object.

> [C-tomalrich-cpe-on-way-out-2025] (relevance: +SURE)
> the difference between "extrinsic" identifiers like CPE, which depend on an external dictionary, and
> intrinsic identifiers like purl, which don't require an external dictionary. ... the user can construct
> them using information they either have on hand or can easily look up. ...
> fewer than half of CVE records created since February 2024 contain a CPE name. This means that half of the
> vulnerabilities (CVEs) that have been identified since that month do not usually show up in an automated
> vulnerability search of the NVD.

> [A-deb-version-evr-sorting-2025] (relevance: +SURE)
> [epoch] It is provided to allow mistakes in the version numbers of older versions of a package, and also a
> package's previous version numbering schemes, to be left behind. ...
> The lexical comparison is a comparison of ASCII values modified so that all the letters sort earlier than
> all the non-letters and so that a tilde sorts before anything, even the end of a part. For example, the
> following parts are in sorted order: '~~', '~~a', '~', the empty part, 'a'. ...
> Usually this will be in the same format as that specified by the upstream author(s); however, it may need
> to be reformatted to fit into the package management system's format and comparison scheme.

> [A-pep440-python-versioning-2014]:Local version identifiers + Version epochs (relevance: +SURE)
> Local version identifiers are used to denote fully API (and, if applicable, ABI) compatible patched
> versions of upstream projects. ... The inclusion of the local version label makes it possible to
> differentiate upstream releases from potentially altered rebuilds by downstream integrators. ...
> An "upstream project" is a project that defines its own public versions. A "downstream project" is one
> which tracks and redistributes an upstream project, potentially backporting security and bug fixes ...
> Most version identifiers will not include an epoch, as an explicit epoch is only needed if a project
> *changes* the way it handles version numbering in a way that means the normal version ordering rules will
> give the wrong answer.

> [B-rpmvercmp-fedora-quirks-2012] (relevance: ~SUSPECT)
> the algorithm's actions is undefined in some cases, in a ways may make the resulting comparisons stop
> working sanely ... To avoid these, make sure that all your labels start and end with alphanumeric
> characters. ...
> 6. 'FC5' is older than 'fc4', because it uses uppercase letters. 7. '2a' is older than '2.0', because
> numbers are considered newer than letters. ... 5. 'fc4' is equal to 'fc.4' ...
> (live quirk, rpm discussions #2765) rpmvercmp("5.3.0","5.3.0+") == 0 even when the versions are not the same.

> [A-semver-2-precedence-2013]:spec items 1, 8, 11 (relevance: +SURE)
> Software using Semantic Versioning MUST declare a public API. ...
> Major version X (X.y.z | X > 0) MUST be incremented if any backward incompatible changes are introduced to
> the public API. ...
> Precedence is determined by the first difference when comparing each of these identifiers from left to
> right ... Numeric identifiers always have lower precedence than non-numeric identifiers.

> [B-node-semver-range-grammar-2020]:Ranges + Advanced Range Syntax (relevance: +SURE)
> Comparators can be joined by whitespace to form a `comparator set`, which is satisfied by the
> **intersection** of all of the comparators it includes. A range is composed of one or more comparator sets,
> joined by `||`. ...
> `~1.2.3` := `>=1.2.3 <1.3.0` ... `^1.2.3` := `>=1.2.3 <2.0.0` ... `^0.2.3` := `>=0.2.3 <0.3.0` ...
> If a version has a prerelease tag ... then it will only be allowed to satisfy comparator sets if at least
> one comparator with the same `[major, minor, patch]` tuple also has a prerelease tag.

> [B-rubygems-pessimistic-operator-2024]:Optimistic vs pessimistic (relevance: +SURE)
> RubyGems provides a shortcut ... the twiddle-wakka: `~> 2.2` ... Had we said `~> 2.2.0`, that would have
> been equivalent to `['>= 2.2.0', '< 2.3.0']`. ...
> Pessimistic versioning is often wrong in both directions. Patch or minor releases can still introduce
> incompatibilities ... a pessimistic constraint in a published gem can lock the entire dependency graph out
> of new releases ... This kind of transitive lock-in is a common and serious problem in practice.

> [B-hickey-spec-ulation-2016] (relevance: ~SUSPECT)
> change is _not_ a thing. ... We need to talk about one of two things. It either grew or it broke. There is
> growth and there is breakage. ... Adding stuff is growth. ... And removing stuff is always breakage. ...
> But what about the major component? What does it mean? It means you are screwed. ... It is an absolute
> catastrophe, because it does not tell you in what way. ...
> So it has got too broad a semantic.

> [C-zerover-version-means-nothing-2018] (relevance: ~SUSPECT)
> ZeroVer (AKA 0ver) is simple: Your software's major version should never exceed the first and most
> important number in computing: *zero*. ... [HN, quoted on-site] ZeroVer is when a project chooses not to
> try to communicate very much through version numbers.
> (evidence tail: OpenSSL, HashiCorp Vault/Terraform, Elm, FastAPI, React Native, Neovim etc. listed at 0.x
> for 8–29 "0ver years".)

> [A-tuf-freshness-expiry-2024]:§1.5.2 + §2.1.4 Timestamp role (relevance: +SURE)
> **Indefinite freeze attacks.** An attacker cannot respond to client requests with the same, outdated
> metadata without the client being aware of the problem. **Fast-forward attacks.** ... **Rollback
> attacks.** An attacker cannot trick clients into installing software that is older than that which the
> client previously knew to be available. ...
> To prevent an adversary from replaying an out-of-date signed metadata file whose signature has not yet
> expired, an automated process periodically signs a timestamped statement ... Any delegation can be revoked
> at any time: the delegating role needs only to sign new metadata that no longer contains that delegation.

> [B-reproducible-builds-buildinfo-2023] (relevance: ~SUSPECT)
> All relevant information about the build environment should either be defined as part of the development
> process or recorded during the build process. ... This product is called the 'buildinfo' ... Debian shares
> its buildinfo files as plain text files following the control file format, usually clearsigned with
> OpenPGP ... Unlike on Debian, [Arch's] file is not independently signed and distributed, but included into
> the package.

## Source registrations
(Registered into this directory's script-owned `sources.json` on 2026-07-11; the pending-JSON blocks formerly here were removed after registration — the manifest is the canonical record.)

## Residue

- Deliberately NOT re-fetched, to spend budget on unseen material (both previously project-graded per the
  charter): the CISA "Software Identification Ecosystem Option Analysis" PDF (cisa.gov, 2023) and the
  purl-intro/PURL-SPECIFICATION identity text. Both remain available and locatable; I confirmed the purl
  identity angle only through the FAQ's version-comparison clause (a fresh, narrower read). If the conductor
  wants an independent fresh grade of the CISA PDF specifically, that is a clean follow-up — I judged the
  CVE-QWG RFD (2025, unseen, more current) the better spend on the CPE-failure front.
- purl normative spec is NOT paywalled in practice: ECMA-427 is published free at
  ecma-tc54.github.io/ECMA-427 and the source `.rst` lives in package-url/purl-spec. No show-stopper.
- Hickey primary is a YouTube keynote; I used the community transcript (fidelity caveat, graded C). Did not
  fetch the video. If a load-bearing verbatim is ever contested, the transcript line-anchors to the talk.
- Angles touched but not spent as separate kept sources: (a) the "CPE semantics are insufficiently defined"
  CVE quality-workgroup issue #12 — folded into the CVE-QWG RFD, which is the stronger primary; (b) the
  seal.security >10k-Alpine same-string/different-hash finding — already in the corpus (r10 security per the
  charter), and covered here structurally by PEP-440 local-version + reproducible-builds buildinfo; (c)
  Hynek "Semantic Versioning Will Not Save You" and the jashkenas "SemVer isn't" gist — redundant with
  Hickey+ZeroVer for the counter-thesis, not separately kept.
- Not pursued (out of charter / standing exclusions): transport-verification mechanics, hash-algorithm
  choice, registry design, secrets, corpus-sizing/H2SaLS. TUF's key-compromise and transport machinery was
  read but deliberately excluded from the citation set; only its freshness/expiry/delegation semantics kept.
- One thin cell flagged for a future turn, not gathered here: ecosystems with NO total order and NO
  vuln-DB coverage (vers1-q-compare-home). rpmvercmp shows even "ordered" ecosystems have pathological
  regions; a kind whose tool has no comparator at all (e.g. content-hash-only or date-stamped tools) is
  unaddressed by every source found — all prior art assumes SOME orderable version string exists.

