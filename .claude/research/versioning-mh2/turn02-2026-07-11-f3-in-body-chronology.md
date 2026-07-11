# turn02/f3 — in-body chronology (gatherer: Opus subagent, 2026-07-11)

Front: how ONE body of code honestly describes MANY versions of a described tool at once —
version-knowledge pushed DOWN into branching structure, not fanned out into per-version siblings.
Central filter recorded per source: does the prior art assume the versioner IS the author of the
thing versioned? (If yes → undershoots Dorc, whose describers are ~always third parties.) The
version-keyed vs capability-keyed tension is tracked on every finding.

## Findings

- f3-fd1 +SURE: The canonical counter-tradition is autoconf's "test for features, not for
  versions." The stated rationale is exactly the third-party-describer's dilemma: a *pure
  version-based approach* misses a capability the user installed out-of-band (an ISO-C compiler on
  a SunOS-4 box), and it forces you to "keep track of minute details of versions, patch numbers,
  etc." A feature *probe* sidesteps the version→behavior map entirely. [C-autoconf-doctrine-wikipedia-2026]
- f3-fd2 +SURE: Even autoconf keeps a version/vendor-keyed ESCAPE HATCH. For "features [that]
  can't be guessed automatically by running test programs" it falls back to canonicalizing a
  `cpu-company-system` triplet (via `config.guess`/`uname`) and dispatching with `case $host_os in`
  shell-wildcard arms. So the feature-not-version doctrine is not absolute — it concedes a residue
  of genuinely un-probeable facts that must be version/vendor-keyed. This is the exact shape Dorc
  faces. [A-autoconf-manual-system-type-1999]
- f3-fd3 +SURE: Feature-probing has its own honest failure modes, two of which autoconf documents
  structurally: (a) runtime probes cannot RUN when cross-compiling (you must pre-declare
  `--host`/`--build` and the cross-tools), so the probe degrades back to a version/type guess; and
  the config-run cost is real (each feature = compile+link a test program). [A-autoconf-manual-system-type-1999]
- f3-fd4 +SURE: The web is the four-decade natural experiment in third-party version-keying going
  wrong. Every browser eventually LIES in its UA string to defeat version sniffers — IE reported
  `Mozilla/2.0 (compatible; MSIE ...)`, WebKit/Chrome carry `(KHTML, like Gecko)`, all modern
  browsers say `Mozilla/5.0`. Zakas's one-line verdict: the history "is marked by browsers trying
  to convince user-agent sniffers that they are what they are not." Version-keying a third party's
  tool rotted into universal deception. [B-ua-string-history-zakas-2010]
- f3-fd5 +SURE: MDN's current guidance states the capability-keyed thesis in Dorc-relevant terms:
  three assumptions a version/UA check makes that "will break the code" — (1) the version string
  is honest (it isn't), (2) the feature is present in every build of that version (an older build
  predates it, a newer build removed it — capability drifts WITHIN a version label), (3) NO other
  vendor has the feature (excludes tools you didn't enumerate). Feature detection (`"geolocation"
  in navigator`, CSS `@supports`) escapes all three. This is the strongest single statement that a
  third party should PROBE capability, not ENUMERATE version→behavior. [A-mdn-feature-detection-2026]
- f3-fd6 ~SUSPECT: Feature-detection is not free either — MDN concedes "in rare cases where
  behavior differs between browsers for a feature, you should test how browsers implement the
  API," i.e. a capability can be present-but-buggy (false positive). Probing answers "exists?" not
  "works correctly?"; the residual behavior-difference still needs handling. Polyfills are the
  capability-keyed sibling of a compat shim. [A-mdn-feature-detection-2026]
- f3-fd7 +SURE: The compat-shim spelling (python-six) is the archetype of in-body chronology:
  compute COARSE version flags ONCE at the top (`PY2 = sys.version_info[0] == 2`, `PY3 = ...`,
  `PY34 = sys.version_info[0:2] >= (3, 4)`), then push the chronology down into `if PY3: ... else:
  ...` branches around a single stable API surface. One file, both versions, no per-version
  siblings. This is precisely the "version-knowledge into branching structure" shape Dorc wants.
  [A-six-inbody-dispatch-2026]
- f3-fd8 ~SUSPECT: six is a partial-undershoot on the central filter. six's authors don't own
  CPython, so they ARE third-party describers — but the thing described (Python 2 vs 3) is a
  single, well-specified, sharply-bounded standard with exactly two coarse buckets. Dorc's
  described tools are messy, many-versioned, and unstandardized. six's clean `PY2/PY3` dichotomy is
  the easy case; the lesson caps out below Dorc's need. The retirement cost is real, though: whole
  tools exist to mechanically strip the chronology once a version dies. [A-six-inbody-dispatch-2026] [B-pyupgrade-retire-2026]
- f3-fd9 +SURE: Retiring a chronology is a first-class, tooled activity. pyupgrade is a pre-commit
  hook that "automatically upgrade[s] syntax for newer versions of the language" — mechanically
  rewriting old-version idioms (incl. six calls) to native modern forms once the floor rises. The
  chronology is machine-maintained on BOTH ends: added by hand, removed by tool. [B-pyupgrade-retire-2026]
- f3-fd10 +SURE: Real sh installers dispatch on the DESCRIBED system's version with `case`. Docker's
  `get.docker.com` script carries a chronology of every Debian release inline — `case
  "$dist_version" in 13|forky) dist_version="trixie";; 12) ...="bookworm";; ... 8) ...="jessie"`
  — a numeric→codename translation table that must be MAINTAINED (trixie/13 was added recently).
  One script body honestly handles Debian 8–13 + Ubuntu + CentOS/RHEL/Rocky at once. Pure
  version-keying, third-party (nobody at Docker owns Debian's numbering). [A-docker-install-case-2026]
- f3-fd11 +SURE: Portable sh version-COMPARISON is a horror show, and real scripts hand-roll it.
  nvm shells out to `awk`, splits both operands on `.`, strips a leading `v`, and loops i=1..3
  comparing numerically — with honest, visible limitations baked in: only 3 dot-components are
  compared, a non-numeric segment is treated as a magic signal (not parsed as pre-release), and
  there is no epoch/semver awareness. It then gates behavior on the result
  (`nvm_version_greater_than_or_equal_to "${NODE_VERSION}" 0.6.0 && ... NVM_IS_0_6=1`). This is the
  in-body chronology flag-then-branch pattern (cf. six's `PY3`) done in pure sh. [A-nvm-version-greater-2026]
- f3-fd12 +SURE: `sort -V` (GNU coreutils) is the tempting sh primitive, but coreutils itself
  warns "there is no standard for version sort ... no one correct way or universally agreed-upon
  way to order items." So the primitive is non-canonical: `sort -V` ≠ dpkg order ≠ semver ≠ PEP 440.
  Choosing "the" comparison is choosing an ecosystem. [A-coreutils-version-sort-2026]
- f3-fd13 +SURE: The authoritative comparison algorithms are ecosystem-OWNED and deeply quirky.
  Debian's `deb-version(7)` defines an EVR (`epoch:upstream-debian`) comparison where "a tilde
  sorts before anything, even the end of a part" (so `1.0~rc1` < `1.0`), letters sort before
  non-letters, and — critically — EPOCHS exist "to leave behind mistakes in version numbering ...
  where the version numbering scheme changes," while the algorithm explicitly "is not intended to
  cope with version numbers containing strings of letters ... (such as 'ALPHA' or 'pre-')." Read
  against Dorc's `vers1-fd-compare-is-kind-owned`: comparison semantics are per-KIND, and the
  epoch mechanism is literal prior-art for "upstream renumbered unpredictably" (the third-party
  rot dimension). [A-deb-version-evr-sorting-2025]
- f3-fd14 +SURE: Ansible confirms comparison-scheme is author-chosen-per-ecosystem, not universal.
  Its `version` test (né `version_compare`, renamed 2.5) takes `version_type` ∈ {loose, strict,
  semver, semantic, pep440}, warning that `loose` "may not always give the expected results." The
  idiom is a `when:` guard on the described host's fact: `when: ansible_facts['distribution_version']
  is version('12.04', '>=')`. Third-party version-keying, with the ecosystem's comparison scheme
  named explicitly. [A-ansible-version-test-2026]
- f3-fd15 +SURE: RPM spec `%if` is the packaging incarnation of one-body-many-versions, and its
  practitioner lore converges startlingly with Dorc's own design instincts. `%if %{suse_version}
  >= 1600` keys on the described distro version; the `0%{?suse_version}` guard idiom exists because
  a bare `%if 0%{?suse_version} < 1600` "would also be true on every non-suse distro" (the
  unguarded-negative failure mode of version-keying). `%{pkg_vcmp gimp >= 2.99}` version-compares a
  THIRD-PARTY tool inline. And the "responsible usage" advice — replace a repeated raw conditional
  with a descriptively-NAMED flag (`%global needs_newer_gcc_on_sle15 1`) so another packager can't
  accidentally cargo-cult or clean up an unrelated conditional — is the same move as Dorc's
  `vers1-position-promise-names` (encode the scope/promise into the name). [B-rpm-conditionals-nordisch-2024]
- f3-fd16 +SURE: Gentoo/portage is the explicit COUNTER-model: one file PER version
  (`name-version.ebuild`, e.g. `libfoo-1.2.5b_pre5-r2.ebuild`), i.e. exactly the "sixteen un-DRY
  siblings" Dorc rejects — but made tractable by factoring shared logic into eclasses and by an
  ecosystem-specific ordering (`_alpha < _beta < _pre < _rc < (none) < _p`, and `1.2b` > `1.2a` >
  `1.2`). The lesson for Dorc: if in-body branching is rejected, the fallback is N-siblings-plus-a-
  shared-library, and you still can't escape owning a comparison algorithm. [A-gentoo-ebuild-format-2026]
- f3-fd17 ~SUSPECT: Cross-cutting verdict on the front's core tension: for a THIRD-PARTY describer,
  capability-probing dominates version-enumeration wherever the capability is probeable
  (autoconf-fd1, MDN-fd5), because the version→behavior map is exactly what rots and what the
  described party can lie about (fd4) or drift within (fd5). Version-keying survives only where the
  fact is un-probeable (fd2), where probing is impossible/too costly (fd3, cross-compile), or where
  the described standard is clean and stable enough that the map barely rots (fd8, six). Dorc's
  oracle authors sit mostly in probeable territory — arguing the design should bias toward
  captured-value probes with in-body branching, reserving version-comparison gates for the
  un-probeable residue, with the comparison scheme owned per-KIND.

## Citations

> [C-autoconf-doctrine-wikipedia-2026]:§"portability" (relevance: +SURE)
> The Autoconf approach to portability is to test for features, not for versions. For example, the
> native C compiler on SunOS 4 did not support ISO C. However, it is possible for the user or
> administrator to have installed an ISO C-compliant compiler. A pure version-based approach would
> not detect the presence of the ISO C compiler, but a feature-testing approach would be able to
> discover the ISO C compiler the user had installed. The rationale of this approach is to gain the
> following advantages:
> * the configure script can get reasonable results on newer or unknown systems
> * it allows administrators to customize their machines and have the configure script take advantage of the customizations
> * there is no need to keep track of minute details of versions, patch numbers, etc., to figure out whether a particular feature is supported or not

> [A-autoconf-manual-system-type-1999]:§"Manual Configuration"/"Using the System Type" (relevance: +SURE)
> A few kinds of features can't be guessed automatically by running test programs. For example, the
> details of the object file format ... You can check for such features using ad-hoc means, such as
> having `configure` check the output of the `uname` program, or looking for libraries that are
> unique to particular systems.
> ...
> How do you use a canonical system type? Usually, you use it in one or more `case` statements in
> `configure.in' to select system-specific C files. ... The `case` statement patterns can use shell
> wildcards to group several [system types].
> ...
> Alternately, the user can specify the system type with command line arguments to `configure`.
> Doing so is necessary when cross-compiling. ... If you are cross-compiling, you still have to
> specify the names of the cross-tools you use, in particular the C compiler, on the `configure`
> command line

> [B-ua-string-history-zakas-2010]:§"Conclusion" + §"Netscape/IE 3" + §"WebKit" (relevance: +SURE)
> Since most browser sniffers at the time looked only at the product-name part of the user-agent
> string, IE successfully identified itself as Mozilla, the same as Netscape Navigator. ... The
> most interesting and controversial part of this user-agent string is the addition of the string
> "(KHTML, like Gecko)" ... Apple got a lot of pushback from developers who saw this as a blatant
> attempt to trick clients and servers into thinking Safari was actually Gecko
> ...
> The history of the user-agent string is marked by browsers trying to convince user-agent sniffers
> that they are what they are not. Internet Explorer wants to be identified as Netscape 4;
> Konqueror and WebKit want to be identified as Firefox; Chrome wants to be identified as Safari.

> [A-mdn-feature-detection-2026]:§"Why feature detection is better than browser detection" (relevance: +SURE)
> This code makes several assumptions that may be wrong ...
> 1. All user agent strings that include the substring `Chrome` indicate a Chrome browser. One of
>    the biggest problems with browser detection based on UA strings is that browsers and user
>    agents routinely pretend to be another browser ...
> 2. The lookbehind feature is always available if the browser is Chrome. In reality, the browser
>    might be an older version of Chrome before support was added, or it could be a later version of
>    Chrome that removes it.
> 3. Most importantly, it assumes no other browsers support the feature, when it could be added to
>    any other browser at any time. ...
> Feature detection is where you check to see if a specific feature is available to the client
> instead of figuring out which browser is rendering your page. ...
> In rare cases where behavior differs between browsers for a feature, you should test how browsers
> implement the API and determine how to use it based on that.

> [A-coreutils-version-sort-2026]:§30.1.3 "Variations in version sort order" (relevance: +SURE)
> Currently there is no standard for version sort.
> That is: there is no one correct way or universally agreed-upon way to order items. Each program
> and each programming language can decide its own ordering algorithm and call it "version sort",
> "natural sort", or other names.

> [A-deb-version-evr-sorting-2025]:§"Sorting algorithm" + §NOTES (relevance: +SURE)
> The lexical comparison is a comparison of ASCII values modified so that all the letters sort
> earlier than all the non-letters and so that a tilde sorts before anything, even the end of a
> part. For example, the following parts are in sorted order: '~~', '~~a', '~', the empty part, 'a'.
> ...
> Note that the purpose of epochs is to allow us to leave behind mistakes in version numbering, and
> to cope with situations where the version numbering scheme changes. It is not intended to cope
> with version numbers containing strings of letters which the package management system cannot
> interpret (such as 'ALPHA' or 'pre-'), or with silly orderings.

> [A-nvm-version-greater-2026]:nvm.sh L679-698 (relevance: +SURE)
> nvm_version_greater() {
>   command awk 'BEGIN {
>     if (ARGV[1] == "" || ARGV[2] == "") exit(1)
>     split(ARGV[1], a, /\./);
>     split(ARGV[2], b, /\./);
>     for (i=1; i<=3; i++) {
>       if (a[i] && a[i] !~ /^[0-9]+$/) exit(2);
>       if (b[i] && b[i] !~ /^[0-9]+$/) { exit(0); }
>       if (a[i] < b[i]) exit(3);
>       else if (a[i] > b[i]) exit(0);
>     }
>     exit(4)
>   }' "${1#v}" "${2#v}"
> }
> --- (in-body gate elsewhere in nvm.sh:) ---
>   NVM_IS_0_6=0
>   if nvm_version_greater_than_or_equal_to "${NODE_VERSION}" 0.6.0 && nvm_version_greater 0.7.0 "${NODE_VERSION}"; then
>     NVM_IS_0_6=1

> [A-docker-install-case-2026]:install.sh (do_install, case "$lsb_dist") (relevance: +SURE)
> 	case "$lsb_dist" in
> 		ubuntu)
> 			if command_exists lsb_release; then
> 				dist_version="$(lsb_release --codename | cut -f2)"
> 			fi
> 			...
> 		debian|raspbian)
> 			dist_version="$(sed 's/\/.*//' /etc/debian_version | sed 's/\..*//')"
> 			case "$dist_version" in
> 				13) dist_version="trixie" ;;
> 				12) dist_version="bookworm" ;;
> 				11) dist_version="bullseye" ;;
> 				10) dist_version="buster" ;;
> 				9)  dist_version="stretch" ;;
> 				8)  dist_version="jessie" ;;
> 			esac
> 		;;
> 		centos|rhel|rocky)
> 			if [ -z "$dist_version" ] && [ -r /etc/os-release ]; then
> 				dist_version="$(. /etc/os-release && echo "$VERSION_ID")"
> 			fi

> [A-six-inbody-dispatch-2026]:six.py L34-52 (relevance: +SURE)
> # Useful for very coarse version differentiation.
> PY2 = sys.version_info[0] == 2
> PY3 = sys.version_info[0] == 3
> PY34 = sys.version_info[0:2] >= (3, 4)
>
> if PY3:
>     string_types = str,
>     integer_types = int,
>     class_types = type,
>     text_type = str
>     binary_type = bytes
>     MAXSIZE = sys.maxsize
> else:
>     string_types = basestring,
>     integer_types = (int, long)
>     ...

> [B-pyupgrade-retire-2026]:README §header (relevance: +SURE)
> pyupgrade
> A tool (and pre-commit hook) to automatically upgrade syntax for newer versions of the language.

> [A-ansible-version-test-2026]:§"Comparing versions" (relevance: +SURE)
> Note: In 2.5 `version_compare` was renamed to `version`
> "{{ ansible_facts['distribution_version'] is version('12.04', '>=') }}"
> ... As of Ansible 2.11 the `version` test accepts a `version_type` parameter ... loose, strict,
> semver, semantic, pep440
> `loose` ... The rules for comparison are simple and predictable, but may not always give the
> expected results.
> `pep440` ... "{{ '2.14.0rc1' is version('2.14.0', 'lt', version_type='pep440') }}"

> [B-rpm-conditionals-nordisch-2024]:§"basics" + §"Responsible usage" (relevance: +SURE)
> %if %{suse_version} >= 1600
> ...
> Though accessing values like this can lead to errors when trying to build for multiple
> distributions. ... we can expand a variable only when it is actually defined. The syntax for this
> is `%{?suse_version}`. We can combine this with a default value `0%{?suse_version}`.
> ...
> %if 0%{?suse_version} < 1600
> # This would also be true on every non suse distro
> ...
> %if %{pkg_vcmp gimp >= 2.99}
> ...
> [responsible usage refactor:]
> %if 0%{?sle_version} && 0%{?sle_version} < 160000
> %global needs_newer_gcc_on_sle15 1
> %endif
> %if 0%{?needs_newer_gcc_on_sle15}
> BuildRequires:  gcc13
> ... Our conditional is descriptive and we have the conditional only once.

> [A-gentoo-ebuild-format-2026]:§"File naming rules" (relevance: +SURE)
> An ebuild should be named in the form `name-version.ebuild`.
> ... In the following table, what Portage considers to be the 'lowest' version comes first.
> _alpha (earliest) / _beta / _pre / _rc / (no suffix) / _p
> ... Portage treats `1.2b` as being a later version than `1.2` or `1.2a`.
> ... Overall, this gives us a filename like `libfoo-1.2.5b_pre5-r2.ebuild`.

## Source registrations
(Registered into this directory's script-owned `sources.json` on 2026-07-11; the pending-JSON blocks formerly here were removed after registration — the manifest is the canonical record.)

## Residue

- Show-stoppers: NONE. No paywalled/dead critical source.
- The pithy autoconf doctrine sentence ("test for features, not versions") is a community/Wikipedia
  distillation; the CURRENT GNU manual states the philosophy dispersed (the "Writing Tests" chapter
  opens on checking "whether various kinds of features are available" rather than as one quotable
  line). I anchored the doctrine on Wikipedia (C, faithful) + the primary MIT manual for the
  escape-hatch/cross-compile nuance (A). Repeated attempts to grep the modern `autoconf.texi`
  (autotools-mirror + Thyre mirrors) for the exact phrase returned nothing — the mirror that
  resolved was an old 2.13 branch; a fully-current verbatim doctrine passage was not located and is
  a minor gap, not a show-stopper.
- get.docker.com serves the installer but returns 403 on robots.txt to automated fetchers; I read
  the identical artifact from the docker/docker-install GitHub source instead (canonical, same file).
- Angles deliberately NOT deepened (breadth spent, keep-bar reached): (a) six's formal EOL/removal
  campaign threads (requests #6023 etc.) — the retirement COST is already carried by pyupgrade
  + fd8/fd9; (b) Modernizr internals — MDN covers feature-detection's failure modes primary-source,
  and Modernizr's own docs are largely marketing; (c) rpmvercmp C source and the full dpkg
  `--compare-versions` man section — deb-version(7) already gives the authoritative algorithm; (d)
  kernel-version parsing scripts — nvm+docker already specimen the sh idioms. None change the
  findings; all are re-openable if the front needs more depth on a specific spelling.
- Cross-check on the central filter: 10/13 sources pass it squarely (describer ≠ authored-tool);
  six is the one flagged PARTIAL-undershoot (clean two-bucket standard = the easy case); coreutils
  and pyupgrade are filter-N/A (a primitive and a tool). The version-keyed vs capability-keyed split
  is evidenced on BOTH sides with failure modes for each, per the front's charter.
