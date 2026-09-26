# gather-platform-specs-services-network-names — first-party specs for services, hosts, names, users, namespaces, wrappers

Lane 7 of ten. 311 read at `63e49f29` (`git rev-parse --short HEAD` at lane start). Every grade below is
`graded-by: subagent`. Citation line numbers are from the archived copies `sources/<slug>.<ext>`
(`grep -n -F` on the phrase), so an HTML archive's line number can sit one or two lines off the
visible sentence start.

## Findings

- The closest deployed analogue in this lane is Solaris FMA's FMRI. Every identifier carries an
  authority, "either explicitly ... or implicitly that of the local fault management domain". The
  spec declares each scheme logical or universal. A logical scheme's identical FMRIs "native to
  distinct fault management domains do not necessarily identify the same actual resource". A
  universal scheme's identical FMRIs "identify the same actual resource wherever they are
  interpreted". This is 311's mRoute-terminated shape vs `:root`, declared per scheme by the scheme
  owner. The declaration lives in the spec, not the data ("FMRI schemes do not include a member
  indicating whether the scheme is logical or universal"). pkg v0 is universal only "if package name
  and package version conventions are adhered to", a conditional warrant. Authority members are
  graded invariant vs "soft identification which could change over time" (hostname: "Not
  invariant"). Schemes embed other schemes' FMRIs (mod.mod-pkg). The sw scheme records the chroot
  root in the identity. Unlike 311, it has no DISJOINT, no write-reach, and no correspondence
  relation. [A-solaris-fmri-man-2011] +SURE on the text; ~SUSPECT that it is the nearest analogue
  anywhere, since only this lane's corpus was searched.
- RFC 4007 is the cleanest first-party statement of scoped identity. It also contains the refusal
  311 would predict:
  - "The zone to which a particular non-global address pertains is not encoded in the address
    itself but determined by context."
  - Zone indices are "strictly local to the node" (mRoute-scoped).
  - An unqualified address "should be interpreted as <address>%<default ID>" (an ambient mParent
    the node supplies).
  - "there is no way for a node to automatically determine which of its interfaces belong to the
    same zones ... an implementation must provide a means for manual assignment". Sameness of two
    parents is speech, never measured.
  - A zone-qualified literal "MUST NOT be sent on the wire unless every node that interprets the
    format agrees on the semantics". Nothing speaks across mWorlds.

  [A-rfc4007-ipv6-scoped-address-architecture-2005] +SURE.
- The DNS RFCs split 311's two warrants the way 311 does:
  - RFC 2181 §10 refuses unique naming: "There is no such requirement in the DNS" that a host have
    "exactly one authoritative, or official, name". So `:guarantees-unique-name` is absent.
  - It keeps the alias map a function: "There may be only one such canonical name for any one
    alias". So `:guarantees-unique-referent` holds on the CNAME lookup.
  - It enumerates four exclusive states of a name, which separate "exists, but has no associated
    RRs" (NODATA) from "does not exist at all" (NXDOMAIN).
  - NS, MX and PTR values "must not be an alias". These are canonical-only slots, i.e.
    primary-key-only positions.

  [A-rfc2181-dns-clarifications-1997] +SURE.
- Contradiction with a 311 example, not with a 311 rule. 311 §1.6 names "the DNS mRoot" as a mRoot
  example. The DNS community's own terminology says "a domain name that is notionally globally
  unique has different meanings for different network users" (split DNS). Views "are not a
  standardized part of the DNS, but they are widely implemented". Locally served zones resolve "the
  same name ... to different results" with a context "not known to the resolution client".
  [A-rfc9499-dns-terminology-2024] This coincides with GOTCHAS a-private-name-resolves-only-inside,
  and suggests that a DNS name's `:root` holds per view, not globally. +SURE on the quote; -GUESS on
  how 311's owners would want to read it.
- DNS specifies 311 §2.9's "alias above the leaf" and the negative-space traversal.
  - DNAME substitution "is to be applied for all names below the owner name". Adding one "occludes
    any domain names that may exist under" it. [A-rfc6672-dname-redirection-2012]
  - A wildcard does not apply once "a name between the wildcard domain and the query name is known
    to exist". Adding any record at an intermediate name therefore silently re-routes names nobody
    mentioned. [A-rfc4592-dns-wildcards-2006]
  - RFC 4592 had to redefine existence itself, because RFC 1034 read literally means "all possible
    domains exist". [A-rfc4592-dns-wildcards-2006]

  ~SUSPECT (partial reads of the algorithm chapters).
- The kernel documents `:corresponds` as transition-owner speech, and documents observer-dependent
  rendering.
  - PIDs: "A process has one process ID in each of the layers of the PID namespace hierarchy".
    PIDs passed over SCM_CREDENTIALS are "translated into the corresponding PID value" in the
    receiver's namespace. /proc shows the mounter's namespace "even if the /proc filesystem is
    viewed from processes in other namespaces". [A-linux-pid-namespaces-man-2026]
  - UIDs: uid_map is a write-once, range-to-range map authored by the namespace's creator. Its
    rendering differs per reader ("processes that are in different user namespaces will potentially
    see different values when reading from a particular uid_map file"). Unmapped ids collapse to
    overflow uid 65534, a many-to-one rendering that voids unique-referent on the rendered value.
    [A-linux-user-namespaces-man-2026]

  +SURE. This coincides with 311 §2.7 (the container manager knows guest pid 1 is host pid 4821)
  and §2.8.
- Wrappers write down their `:lends`, and the docs show three failure shapes 311 §3.4 anticipates:
  - A lend is exactly one ingredient. chroot "changes an ingredient in the pathname resolution
    process and does nothing else", not cwd and not open fds. [A-linux-chroot-syscall-man-2026]
  - A lend is wider than the wrapper's name. `ip netns exec` also creates "a mount namespace and
    bind mount[s] all of the per network namespace configure files into their traditional location
    in /etc". [B-iproute2-ip-netns-man-2026]
  - A lend depends on the guest. In sudoers, "command-specific Defaults settings are applied later,
    once the command's path is known", CWD=/CHROOT= are per command, and "the last match is used
    (which is not necessarily the most specific match)". This confirms GOTCHAS
    sudo-picks-the-context-by-the-command from the maintainer's text. [A-sudoers-manual-2026]
  - A catalog that looks path-shaped escapes the filesystem lends. Abstract unix sockets "are
    namespaced according to network namespaces rather than being part of the filesystem ...
    unaffected by ... chroot(2) and mount namespaces". [A-dbus-specification-2024] systemd's
    PrivateNetwork= corroborates this. [A-systemd-exec-man-2026]

  +SURE.
- Key recycling is stated first-party, as is its opposite, a scoped never-reuse warrant.
  - "UID/GIDs are recycled after a unit is terminated ... a different unit might get the same
    UID/GID". systemd closes the store rather than the name: RemoveIPC=, PrivateTmp=.
    [A-systemd-exec-man-2026]
  - useradd resets lastlog "to avoid reusing the entry from a previously deleted user", and says
    the uid, not the login name, "serves as key". [B-shadow-useradd-man-2026]
  - D-Bus unique names "are never reused for two different connections to the same bus". This is a
    warrant scoped to one bus instance. [A-dbus-specification-2024]
  - Clone horizons, and a lifecycle detector with two named modes:
    - machine-id must ship empty in images "used on multiple machines".
      [A-systemd-machine-id-man-2026]
    - cloud-init's check mode compares the cached instance-id with the runtime one. Its trust mode
      makes clones "detect their first boot as a subsequent boot" and skip SSH host-key rotation.
      [A-cloud-init-first-boot-determination-2026]

  +SURE.
- Absent vs inactive vs unknown (311s §C), now pinned.
  - systemd: on v252, `is-active` on a missing unit exited 3. PR #25689 (merged 2022-12-14) makes
    `is-*` exit 4 ("status is unknown") when no unit file exists. The diff still prints the
    active-state string, so stdout reads `inactive` and only the rc tells the cases apart.
    [B-systemd-issue-25680-is-active-nonexistent-2022] `is-enabled` prints `not-found` with exit 4.
    [A-systemctl-man-2026]
  - NSS separates notfound ("lookup succeeded, but the requested entry was not found") from unavail
    (source unreadable or unreachable). [B-glibc-nsswitch-conf-man-2026]
  - nscd keeps a negative cache with its own TTL. [B-nscd-man-2026]
  - SMF has a legacy_run state for instances that "might or might not be running".
    [A-solaris-smf-man-2011]

  +SURE. This refines GOTCHAS a-read-folds-absent-into-a-value: the fold is in the rendering, and
  since v253 it is no longer in the exit code (-GUESS on the exact first release that shipped it).
- Contradiction handling exists but mostly resolves by order. 311's "refuse both and attribute"
  appears once:
  - SMF: a property delivered by two files in one layer tags the instance "in-conflict", and
    svc.startd will not start it. Other readers "see a random property setting".
    [A-solaris-smf-man-2011] Two consumers of the same contradiction apply opposite policies.
  - Order-based resolution:
    - tmpfiles: earliest file wins and "All other conflicting entries will be logged as errors".
      [A-systemd-tmpfiles-d-man-2026]
    - setfiles: the last match wins, with a warning, for hard links matching different specs.
      [B-selinux-setfiles-man-2026]
    - sudoers: the last match wins. [A-sudoers-manual-2026]
    - RFC 2181: a trust ranking by provenance, and servers "must never merge RRs from a response
      with RRs in their cache". [A-rfc2181-dns-clarifications-1997]

  +SURE.
- Aliases inside one catalog, fully specified: systemd units.
  - An alias is a symlink inside the load path. A symlink outside the load path is a "linked unit",
    so which it is depends on catalog membership.
  - "service1.service will have four names".
  - Drop-ins are read for the canonical name and every alias.
  - "Aliases cannot be used with the preset command".
  - When an alias is introduced at reload, "the running state of the canonical unit ... is
    preserved" and the old state migrates to an orphan.

  [A-systemd-unit-man-2026] +SURE. This is a within-catalog `:yields` with a declared
  canonical/primary name.
- Path-as-identity, argued by its own authors, and the label camp's mirror punt.
  - AppArmor "identifies files by name rather than by label". Renaming `/etc/shadow` and replacing
    it moves the grant to the new file.
  - "Pathnames are meaningful only within a namespace".
  - "It is unclear at this point how AppArmor should support separate namespaces", an explicit
    punt.
  - Disconnected files and mount are denied outright.
  - Hard-link creation requires the new path to hold "a subset of the r, w, x, and m permissions of
    the old path", which litigates alias creation as a permission. [A-apparmor-technical-documentation-2007]
  - LWN's framing: "a given file name is not the file itself". [B-lwn-apparmor-debate-begins-2006]
  - SELinux's setfiles, the label camp, still keys policy by path regex and resolves a hard link
    matching several specs by "the last matching specification", with a warning.
    [B-selinux-setfiles-man-2026]

  +SURE.
- Counter-thesis verdict for this lane: mostly confirmed.
  - No platform in the lane has a general cross-scheme identity model, and none has a DISJOINT
    generator, a may-write entailment, or a region test. Each spec is one catalog's mechanism, and
    most punt identity to "the name is the key" or to the operator:
    - tmpfiles conflicts are detected by path equality.
    - AppArmor's namespace punt.
    - RFC 4007's manual zone assignment.
    - RFC 9499's "the IETF has yet to agree on a good set of facets that can be used to compare
      naming systems".
  - The explicit exceptions, where a platform does document a cross-scheme correspondence or a
    per-scheme scope, all come from the transition or scheme owner, as 311's committee law predicts:
    - FMRI authority and logical/universal. [A-solaris-fmri-man-2011]
    - RFC 4007 zone qualification. [A-rfc4007-ipv6-scoped-address-architecture-2005]
    - The kernel's pid and uid translation. [A-linux-pid-namespaces-man-2026]
      [A-linux-user-namespaces-man-2026]
    - D-Bus's semantic definition of machine-id ("if two processes see the same UUID, they should
      also see the same shared memory, UNIX domain sockets, process IDs"). [A-dbus-specification-2024]

  ~SUSPECT.

## Candidate table

| [slug] | what it is | exhaustive / broad / abstract / battle-tested | 311 analogues | punts | altitude |
|---|---|---|---|---|---|
| [A-solaris-fmri-man-2011] | Solaris FMRI scheme definitions | exhaustive for its ten schemes / broad (cpu, dev, hc, mem, mod, pkg, svc, sw, zfs) / abstract (typed member tuples, stabilities) / deployed since Solaris 10 in SMF, fmd, IPS | mScheme per scheme; authority = mParent-Store with implicit local default = mRoute; logical vs universal = mRoute-scoped vs `:root`; invariant vs soft authority members = warrant grading; embedded FMRIs = `:yields`; sw.root = lend in identity | logical/universal not in data; many members Private/undocumented; only localhost svc scope | naming theory + schema |
| [A-solaris-smf-man-2011] | SMF overview | exhaustive for SMF / one domain / moderately abstract / deployed 2005– | composed lookup = inherit from mParent; layers = precedence; in-conflict = refuse; snapshots; non-persistent groups = boot-scoped cells; legacy_run = UNKNOWN | readers get a "random" value under conflict | read + config model |
| [A-rfc4007-ipv6-scoped-address-architecture-2005] | IPv6 scoped addressing | exhaustive for scope / one domain / abstract (scope, zone, index) / universal deployment | mKey-Primary in mParent-Store (zone); node-local index = mRoute; default zone = ambient mParent; wire ban = no speech across mWorlds | zone membership manual; interface-name zone ids implementation-defined | naming theory |
| [A-rfc2181-dns-clarifications-1997] | DNS clarifications | exhaustive on RRsets, CNAME, TTL / one domain / normative / 29 years | warrants split (unique-referent kept, unique-name refused); four name states; canonical-only slots; provenance ranking; child owns its cut | "does not consider security" | naming + protocol |
| [A-rfc9499-dns-terminology-2024] | DNS terminology BCP | exhaustive glossary / one domain / abstract facets / consensus doc | alias, canonical name, owner, zone, cut, occlusion; split DNS vs `:root`; "context for resolving a name" facet | no agreed facet set; bailiwick "historic" | naming theory |
| [A-rfc6672-dname-redirection-2012] | DNAME | exhaustive for DNAME / narrow / normative / deployed | alias above the leaf; occlusion as a routing write; singleton redirection | loops left to administrators | naming |
| [A-rfc4592-dns-wildcards-2006] | DNS wildcards | narrow / normative / written from divergent implementations | many-to-one synthesis; traversal = negative space; existence redefined | implementations free; "no search for an alternate" | naming |
| [A-dbus-specification-2024] | D-Bus spec | exhaustive for bus naming / one domain / normative / 20 years | unique vs well-known names (primary vs natural); scoped never-reuse; owner queue; NameOwnerChanged = routing event; machine-id semantic definition; abstract socket lend gap | machine-id precedence between two files undefined | protocol + naming |
| [A-systemd-unit-man-2026] | unit files | exhaustive for units / one domain / precise / pervasive | alias set with canonical name; load-path precedence; drop-ins per alias; masking; conditions leave no state | "the condition failure may or may not show up in the state of the unit" | config model |
| [A-systemctl-man-2026] | systemctl | enumerations of is-enabled states and LSB codes | absent vs disabled vs masked rendering | "mapping ... is imperfect" | read interface |
| [B-systemd-issue-25680-is-active-nonexistent-2022] | bug + fix | narrow / litigated / merged PR | absent vs inactive: rc 4 since the fix, stdout still `inactive` | exit codes "not so ideally documented" | read interface |
| [A-systemd-exec-man-2026] | exec environment | per-directive lends | lends with stated gaps; uid recycling; store-lifetime bound to key-lifetime | lend "might be impossible" | wrapper spec |
| [A-systemd-machine-id-man-2026] | machine-id | narrow / precise | clone horizon; generator chain; first-boot states | none stated | token spec |
| [B-systemd-hostname-man-2026] | /etc/hostname | narrow | hostname priority chain; name derived from machine-id per boot | other programs may set it | slot spec |
| [B-systemd-hostnamectl-man-2026] | hostnamectl | narrow | three hostnames; -H/-M vantage flags | — | CLI |
| [B-systemd-net-naming-scheme-man-2026] | netdev naming | exhaustive for udev naming / versioned | name as a versioned function of attributes | versions may rename | naming |
| [B-systemd-predictable-interface-names-2026] | naming rationale | narrow | eth0 instability; two-writers race in one namespace | — | rationale |
| [A-systemd-tmpfiles-d-man-2026] | tmpfiles.d | exhaustive convergent-op language | create-if-absent vs replace modifiers; precedence; boot-only `!` | identity = path equality | mutation spec |
| [A-linux-pid-namespaces-man-2026] | pid namespaces | exhaustive for pid ns | per-level pid; kernel-owned translation = `:corresponds`; /proc = mounter's store; /proc/self indexical | — | kernel ABI |
| [A-linux-user-namespaces-man-2026] | user namespaces | exhaustive for user ns | uid_map = authored write-once correspondence; observer-dependent rendering; overflow collapse; initial ns as root store | — | kernel ABI |
| [A-linux-chroot-syscall-man-2026] | chroot(2) | narrow | a lend of exactly one ingredient | "not intended ... for any kind of security purpose" | kernel ABI |
| [B-iproute2-ip-netns-man-2026] | ip netns | narrow | lend wider than name; named netns = file | convention, not ABI | wrapper |
| [A-sudoers-manual-2026] | sudoers | exhaustive policy language | guest-dependent lends; env scrubbing; timestamp scope | last match, not most specific | wrapper policy |
| [B-sudo-manual-2026] | sudo -i | narrow | login shell inserted into the lend | — | wrapper |
| [B-glibc-nsswitch-conf-man-2026] | NSS | one mechanism, many DBs | ordered catalog chain; notfound vs unavail; `[NOTFOUND=return]` closure; merge | pre-2.33 frozen config | lookup chain |
| [B-nscd-man-2026] | nscd | narrow | positive and negative cache; open read set admitted | nonstandard modules not watched | cache |
| [B-shadow-useradd-man-2026] | useradd | narrow | uid is the key, login name aliases (`-o`); recycling admitted | — | admin tool |
| [B-git-gitrevisions-2023] | git revision syntax | narrow | first-match refname chain; abbreviated SHA-1 depends on the whole catalog | — | naming |
| [A-cloud-init-first-boot-determination-2026] | cloud-init first boot | narrow / bug-litigated | lifecycle detection by token comparison; check vs trust | IMDS failure makes it undecidable | lifecycle |
| [A-apparmor-technical-documentation-2007] | AppArmor design doc | broad for file mediation | path = mKey-Natural as policy key; namespace-relative paths; link = permissioned alias creation | namespaces "unclear"; disconnected files denied | policy model |
| [B-lwn-apparmor-debate-begins-2006] | LWN on the path vs label debate | secondary | "name is not the file" | — | debate record |
| [B-selinux-setfiles-man-2026] | setfiles | narrow | path-keyed spec onto inode; hard-link conflict warned | last match wins | policy tool |

Breadth against the 47 items: 1, 2, 3 (via unit files and drop-ins), 4 (InvocationID not covered;
invocation is only implicit in D-Bus unique names), 9/10 (git refname chain, adjacent only), 11–13
(namespacing of sysctls via ip-netns and systemd.exec, adjacency only), 17–20, 21–22, 24 (none
directly), 27, 28 (not pulled), 29/31/41 (not covered), 30, 37/38 (sudoers env, secure_path), 40
(ip-netns /etc/netns resolv.conf; NSS). The lane is strongest on 1, 2, 17–22, 27, 30 and wrappers.

## Citations

(Filled below after archive line numbers are computed.)

## Leads not pulled

- Oracle Solaris 11.4 `smf(7)` and `svc.configd`/`svccfg` snapshot docs · the successor to the 2011
  smf(5) page · the 2011 page covered the same concepts; not worth the budget · docs.oracle.com
  E37838_01.
- Sun FMA "Fault Management Architecture" PSARC 2002/412, 2003/089 and the FMA protocol spec · the
  original design documents behind fmri(5), likely deeper on authority · not located in open form
  within budget · search illumos-gate `usr/src/lib/fm/topo` and the OpenSolaris FMA community
  archive.
- SmartOS OS-6479 (FRU identity in the hc authority) · litigation of what goes in the authority · a
  bug entry, thin · smartos.org/bugview/OS-6479.
- SELinuxProject/selinux issue #43, hard-link conflict detection for restorecon · the (dev, ino)
  tracking debate · setfiles already records the outcome · github.com/SELinuxProject/selinux/issues/43.
- LWN 2007–2010 AppArmor/TOMOYO path-hooks threads (security_path_* LSM hooks) · the kernel-side
  settlement of path mediation · the 2006 article plus the techdoc carry the identity argument;
  later threads are implementation · lwn.net search "pathname-based".
- cloud-init module reference `frequency` (once / once-per-instance / always) and the semaphore
  directory · cache-keyed run-once semantics · the first-boot page covers the identity mechanism ·
  docs.cloud-init.io/en/latest/reference/modules.html.
- `sysusers.d(5)`, `sysctl.d(5)`, `file-hierarchy(7)`, `os-release(5)`, `sd_id128(3)` (404 at the
  guessed URL `sd_id128.html`; the real page is `sd-id128.html`) · each would add one catalog's
  precedence rules · budget went to FMRI and the RFCs instead · freedesktop.org man/latest.
- `proc(5)`, `credentials(7)`, `environ(7)`, `su(1)`, `hosts(5)`, `getaddrinfo(3)`, `unshare(1)` ·
  downloaded, partially grepped, not read in full, so not registered · man7.org.
- RFC 1034 §3.6.2 · downloaded, not read in full; RFC 2181 and RFC 9499 quote the load-bearing
  sentences · rfc-editor.org.
- SELinux refpolicy `policy/modules/*/*.fc` and `.if` interfaces as a may-read/may-write corpus ·
  potentially the largest hand-litigated footprint corpus for daemons · needs a dedicated pass,
  not a man-page read · github.com/SELinuxProject/refpolicy.

## Search log

- (seeds) freedesktop.org systemd man/latest: machine-id, systemd.unit, systemctl, tmpfiles.d,
  systemd.net-naming-scheme, hostname, hostnamectl, systemd.exec · 8 kept
- (seeds) man7.org: pid_namespaces, user_namespaces, nsswitch.conf, nscd, useradd, chroot(2),
  ip-netns · 7 kept (netdevice, ip-link, getent, proc, credentials, environ, su, hosts, unshare
  downloaded, not registered)
- (seeds) rfc-editor.org: 4007, 2181, 9499 (successor to 8499), 6672, 4592 · 5 kept (1034 not read)
- (seed) dbus-specification.html · 1 kept
- (seeds) sudo.ws sudoers, sudo; git-scm gitrevisions; cloud-init boot → first_boot · 4 kept
- kagi · smf(7) FMRI snapshots / FMRI scheme authority / FMA authority spec · 2 kept (fmri(5), smf(5))
- kagi · LWN AppArmor d_path debate / AppArmor techdoc / SELinux file_contexts hard links · 3 kept
- kagi · systemd is-active nonexistent / predictable interface names / cloud-init module frequency ·
  2 kept (issue #25680 + PR #25689 diff via `gh`, systemd.io page)

## Tooling problems

- `sd_id128.html` 404 (the page is `sd-id128.html`); not retried.
- The register lock was heavily contended by sibling lanes, so registrations took minutes each and
  ran in the background.
- Man pages were read through a local `sed 's/<[^>]*>//g'` strip (my reading copy only). The
  archived copies are the untouched HTML the wrapper downloaded.
