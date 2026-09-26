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

`:L<n>` is `grep -n -F` on the archived copy (a phrase split by inline HTML tags was located by a
shorter substring). Quotes were taken from this lane's local reading copies (tag-stripped HTML or
rfc-editor .txt) and are verbatim in wording. The AppArmor locators are `txtL<n>` of
`pdftotext -layout sources/A-apparmor-technical-documentation-2007.pdf -`.

> [A-solaris-fmri-man-2011]:L519 (relevance: +1:SURE)
> Every FMRI includes authority information, either explicitly with the authority nvlist if present in the FMRI or implicitly that of the local fault management domain if not present.

> [A-solaris-fmri-man-2011]:L530–532 (relevance: +1:SURE)
> A logical FMRI scheme defines FMRIs that can only meaningfully be interpreted within the fault management domain (typically an Oracle Solaris instance) in which they were generated. Identical FMRIs of a logical scheme that are native to distinct fault management domains do not necessarily identify the same actual resource.

> [A-solaris-fmri-man-2011]:L541–545 (relevance: +1:SURE)
> A universal FMRI scheme identifies resources in a universally unique manner, and two identical FMRIs in a universal scheme identify the same actual resource wherever they are interpreted. Such schemes are used when ambiguity must be avoided, such as in identifying hardware components that are faulted. FMRI schemes do not include a member indicating whether the scheme is logical or universal.

> [A-solaris-fmri-man-2011]:L656, L676 (relevance: +1:SURE)
> these members are invariant (such as platform serial number) and serve uniquely to identify some element, while others (such as hostname) are a soft identification which could change over time (albeit infrequently).
> The hostname (uname -n) string for the entity on which the fault manager is running. Not invariant.

> [A-solaris-fmri-man-2011]:L803 (relevance: +1:SURE)
> This scheme is universal if package name and package version conventions are adhered to.

> [A-solaris-fmri-man-2011]:L824 (relevance: -0:SUSPECT)
> FMRIs in the pkg scheme version 1 are universal: the same FMRI interpreted in two distinct contexts (such as in distinct Oracle Solaris instances) identify the same actual package (or copies thereof).

> [A-solaris-fmri-man-2011]:L882, L885 (relevance: -0:SUSPECT)
> The configuration of the local Oracle Solaris instance is called the localhost scope, and is the only currently supported scope.
> Furthermore, SMF permits further abbreviation if it identifies a unique service or instance. [...] Such abbreviations are a convention of the SMF subsystem and not part of the formal FMRI definition.

> [A-solaris-fmri-man-2011]:L899 (relevance: -0:SUSPECT)
> [root] string If present, real path to chroot root directory

> [A-solaris-smf-man-2011]:L581 (relevance: +1:SURE)
> If the same property is delivered by multiple files in any other layer, and is not set at a higher layer, the entire instance is tagged as in-conflict, and are not started by svc.startd(1M) until the conflicting definition is removed or the property is set at a higher layer. Other libscf consumers requesting a single value, including svccfg and svcprop, see a random property setting from amongst all appropriate values.

> [A-solaris-smf-man-2011]:L547 (relevance: -0:SUSPECT)
> Property lookups are composed. If a property group-property combination is not found on the service instance, most commands and the high-level interfaces of libscf(3LIB) search for the same property group-property combination on the service that contains that instance.

> [A-solaris-smf-man-2011]:L487 (relevance: +1:SURE)
> LEGACY-RUN This state represents a legacy instance that is not managed by the service management facility. Instances in this state have been started at some point, but might or might not be running.

> [A-solaris-smf-man-2011]:L601 (relevance: -1:GUESS)
> Some property groups are marked as non-persistent. These groups are not backed up in snapshots and their content is cleared during system boot.

> [A-rfc4007-ipv6-scoped-address-architecture-2005]:L235 (relevance: +1:SURE)
> The zone to which a particular non-global address pertains is not encoded in the address itself but determined by context, such as the interface from which it is sent or received. Thus, addresses of a given (non-global) scope may be re-used in different zones of that scope.

> [A-rfc4007-ipv6-scoped-address-architecture-2005]:L409 (relevance: +1:SURE)
> The zone indices are strictly local to the node. For example, the node on the other end of the point-to-point link may well use entirely different interface and link index values for that link.

> [A-rfc4007-ipv6-scoped-address-architecture-2005]:L423 (relevance: +1:SURE)
> At present, there is no way for a node to automatically determine which of its interfaces belong to the same zones; e.g., the same link or the same multicast scope zone larger than interface. In the future, protocols may be developed to determine that information.

> [A-rfc4007-ipv6-scoped-address-architecture-2005]:L864 (relevance: +1:SURE)
> When <zone_id> is the default, the delimiter characters "%" and <zone_id> can be omitted. Similarly, if a textual representation of an IPv6 address is given without a zone index, it should be interpreted as <address>%<default ID>, where <default ID> is the default zone index of the scope that <address> has.

> [A-rfc4007-ipv6-scoped-address-architecture-2005]:L889 (relevance: +1:SURE)
> It cannot be assumed that indices are common across all nodes in a zone (see Section 6). Hence, the format MUST be used only within a node and MUST NOT be sent on the wire unless every node that interprets the format agrees on the semantics.

> [A-rfc2181-dns-clarifications-1997]:L599 (relevance: +1:SURE)
> It has sometimes been inferred from some sections of the DNS specification [RFC1034, RFC1035] that a host, or perhaps an interface of a host, is permitted exactly one authoritative, or official, name, called the canonical name. There is no such requirement in the DNS.

> [A-rfc2181-dns-clarifications-1997]:L604 (relevance: +1:SURE)
> The DNS CNAME ("canonical name") record exists to provide the canonical name associated with an alias name. There may be only one such canonical name for any one alias.

> [A-rfc2181-dns-clarifications-1997]:L626 (relevance: +1:SURE)
> + one or more records exist, none being CNAME records,
> + the name exists, but has no associated RRs of any type,
> + the name does not exist at all.

> [A-rfc2181-dns-clarifications-1997]:L315 (relevance: -0:SUSPECT)
> Servers must never merge RRs from a response with RRs in their cache to form an RRSet. If a response contains data that would form an RRSet with data in a server's cache the server must either ignore the RRs in the response, or discard the entire RRSet currently in the cache, as appropriate.

> [A-rfc2181-dns-clarifications-1997]:L354 (relevance: -0:SUSPECT)
> The accuracy of data available is assumed from its source. Trustworthiness shall be, in order from most to least: + Data from a primary zone file, other than glue data, + Data from a zone transfer, other than glue, [...]

> [A-rfc2181-dns-clarifications-1997]:L466 (relevance: -0:SUSPECT)
> The NS records that indicate a zone cut are the property of the child zone created, as are any other records for the origin of that child zone, or any sub-domains of it.

> [A-rfc2181-dns-clarifications-1997]:L650 (relevance: +1:SURE)
> Note that while the value of a PTR record must not be an alias, there is no requirement that the process of resolving a PTR record not encounter any aliases.

> [A-rfc9499-dns-terminology-2024]:L968 (relevance: +1:SURE)
> Nevertheless, the effect of this is that a domain name that is notionally globally unique has different meanings for different network users. This can sometimes be the result of a "view" configuration, as described below.

> [A-rfc9499-dns-terminology-2024]:L982 (relevance: +1:SURE)
> Views are not a standardized part of the DNS, but they are widely implemented in server software.

> [A-rfc9499-dns-terminology-2024]:L318 (relevance: +1:SURE)
> Resolution of names through locally served zones may result in ambiguous results. For example, the same name may resolve to different results in different locally served DNS zone contexts.

> [A-rfc9499-dns-terminology-2024]:L171 (relevance: -0:SUSPECT)
> Note that this list is a small subset of facets that people have identified over time for naming systems, and the IETF has yet to agree on a good set of facets that can be used to compare naming systems.

> [A-rfc9499-dns-terminology-2024]:L1252 (relevance: -0:SUSPECT)
> Occluded name: "The addition of a delegation point via dynamic update will render all subordinate domain names to be in a limbo, still part of the zone but not available to the lookup process. The addition of a DNAME resource record has the same impact.

> [A-rfc9499-dns-terminology-2024]:L1233 (relevance: -1:GUESS)
> The dictionary definition of bailiwick has been observed to cause more confusion than meaning for this use. These terms should be considered historic in nature.

> [A-rfc6672-dname-redirection-2012]:L248 (relevance: -0:SUSPECT)
> The effect of the DNAME RR is the substitution of the record's <target> for its owner name, as a suffix of a domain name. This substitution is to be applied for all names below the owner name of the DNAME RR.

> [A-rfc6672-dname-redirection-2012]:L374 (relevance: -0:SUSPECT)
> DNAME is a singleton type, meaning only one DNAME is allowed per name. The owner name of a DNAME can only have one DNAME RR, and no CNAME RRs can exist at that name.

> [A-rfc6672-dname-redirection-2012]:L719 (relevance: -0:SUSPECT)
> DNAME records can be added, changed, and removed in a zone using dynamic update transactions. Adding a DNAME RR to a zone occludes any domain names that may exist under the added DNAME.

> [A-rfc4592-dns-wildcards-2006]:L388 (relevance: -0:SUSPECT)
> # - When the query name or a name between the wildcard domain and
> #   the query name is know[n] to exist. . . .

> [A-rfc4592-dns-wildcards-2006]:L512 (relevance: -0:SUSPECT)
> Pedantically reading the above paragraph can lead to an interpretation that all possible domains exist--up to the suggested limit of 255 octets for a domain name [RFC1035].

> [A-rfc4592-dns-wildcards-2006]:L533 (relevance: -1:GUESS)
> A node with no descendants is a leaf node. Empty leaf nodes do not exist.

> [A-rfc4592-dns-wildcards-2006]:L666 (relevance: -0:SUSPECT)
> The important concept is that for any given lookup process, there is at most one place at which wildcard synthetic records can be obtained. If the source of synthesis does not exist, the lookup terminates, and the lookup does not look for other wildcard records.

> [A-linux-pid-namespaces-man-2026]:L155 (relevance: +1:SURE)
> A process has one process ID in each of the layers of the PID namespace hierarchy in which is visible, and walking back though each direct ancestor namespace through to the root PID namespace. System calls that operate on process IDs always operate using the process ID that is visible in the PID namespace of the caller.

> [A-linux-pid-namespaces-man-2026]:L242 (relevance: +1:SURE)
> A /proc filesystem shows (in the /proc/pid directories) only processes visible in the PID namespace of the process that performed the mount, even if the /proc filesystem is viewed from processes in other namespaces.

> [A-linux-pid-namespaces-man-2026]:L283 (relevance: +1:SURE)
> in a different PID namespace (see the description of SCM_CREDENTIALS in unix(7)), it is translated into the corresponding PID value in the receiving process's PID namespace.

> [A-linux-user-namespaces-man-2026]:L276 (relevance: +1:SURE)
> In other words, processes that are in different user namespaces will potentially see different values when reading from a particular uid_map file, depending on the user ID mappings for the user namespaces of the reading processes. Each line in the uid_map file specifies a 1-to-1 mapping of a range of contiguous user IDs between two user namespaces.

> [A-linux-user-namespaces-man-2026]:L345 (relevance: +1:SURE)
> After the creation of a new user namespace, the uid_map file of one of the processes in the namespace may be written to once to define the mapping of user IDs in the new user namespace. An attempt to write more than once to a uid_map file in a user namespace fails with the error EPERM.

> [A-linux-user-namespaces-man-2026]:L558 (relevance: +1:SURE)
> In most such cases, an unmapped user ID is converted to the overflow user ID (group ID); the default value for the overflow user ID (group ID) is 65534.

> [A-linux-user-namespaces-man-2026]:L583 (relevance: -0:SUSPECT)
> the process credentials (UID, GID) and the file credentials are in effect mapped back to what they would be in the initial user namespace and then compared to determine the permissions that the process has on the file.

> [A-linux-chroot-syscall-man-2026]:L99 (relevance: +1:SURE)
> This call changes an ingredient in the pathname resolution process and does nothing else. In particular, it is not intended to be used for any kind of security purpose, neither to fully sandbox a process nor to restrict filesystem system calls.

> [A-linux-chroot-syscall-man-2026]:L126 (relevance: +1:SURE)
> This call does not close open file descriptors, and such file descriptors may allow access to files outside the chroot tree.

> [A-linux-chroot-syscall-man-2026]:L105 (relevance: -1:GUESS)
> However, if a folder is moved out of the chroot directory, an attacker can exploit that to get out of the chroot directory as well.

> [B-iproute2-ip-netns-man-2026]:L122 (relevance: +1:SURE)
> convention for network namespace unaware applications, by creating a mount namespace and bind mounting all of the per network namespace configure files into their traditional location in /etc.

> [B-iproute2-ip-netns-man-2026]:L108 (relevance: -0:SUSPECT)
> resulting from opening /var/run/netns/NAME refers to the specified network namespace. Holding that file descriptor open keeps the network namespace alive.

> [A-sudoers-manual-2026]:L4057 (relevance: +1:SURE)
> However, command-specific Defaults settings are applied later, once the command's path is known.

> [A-sudoers-manual-2026]:L3615 (relevance: +1:SURE)
> When multiple entries match for a user, they are applied in order. Where there are multiple matches, the last match is used (which is not necessarily the most specific match).

> [A-sudoers-manual-2026]:L3483 (relevance: -0:SUSPECT)
> By default, the env_reset flag is enabled. This causes commands to be executed with a new, minimal environment.

> [A-sudoers-manual-2026]:L6121 (relevance: -1:GUESS)
> A single time stamp record is used for all processes with the same parent process ID (usually the shell).

> [B-sudo-manual-2026]:L3585 (relevance: -1:GUESS)
> Run the shell specified by the target user's password database entry as a login shell. This means that login-specific resource files such as .profile, .bash_profile, or .login will be read by the shell.

> [A-dbus-specification-2024]:L2721 (relevance: +1:SURE)
> This automatically-assigned name is called the connection's unique name. Unique names are never reused for two different connections to the same bus.

> [A-dbus-specification-2024]:L2065 (relevance: +1:SURE)
> Implementors should note that on Linux, abstract sockets are namespaced according to network namespaces rather than being part of the filesystem. This means that abstract sockets are unaffected by mechanisms like chroot(2) and mount namespaces, which can lead to a sandbox escape if a sandboxing implementation alters the sandboxed process's view of the filesystem but shares the network namespace with the host.

> [A-dbus-specification-2024]:L2356 (relevance: +1:SURE)
> This UUID must be the same for all processes on a single system at least until that system next reboots. It should be the same across reboots if possible, but this is not always possible to implement and is not guaranteed.

> [A-dbus-specification-2024]:L2369 (relevance: -0:SUSPECT)
> If both exist, they are expected to have the same contents, and if they differ, the spec does not define which takes precedence (the reference implementation prefers /var/lib/dbus/machine-id, but sd-bus does not).

> [A-dbus-specification-2024]:L2380 (relevance: +1:SURE)
> Basically if two processes see the same UUID, they should also see the same shared memory, UNIX domain sockets, process IDs, and other features that require a running OS kernel in common between the processes.

> [A-systemd-exec-man-2026]:L509 (relevance: +1:SURE)
> However, UID/GIDs are recycled after a unit is terminated. Care should be taken that any processes running as part of a unit for which dynamic users/groups are enabled do not leave files or directories owned by these users/groups around, as a different unit might get the same UID/GID assigned later on, and thus gain access to these files or directories.

> [A-systemd-exec-man-2026]:L1121 (relevance: +1:SURE)
> And for AF_UNIX this has the effect that AF_UNIX sockets in the abstract socket namespace of the host will become unavailable to the unit's processes (however, those located in the file system will continue to be accessible). Note that the implementation of this setting might be impossible (for example if network namespaces are not available), and the unit should be written in a way that does not solely rely on this setting for security.

> [B-shadow-useradd-man-2026]:L250 (relevance: -0:SUSPECT)
> As a user identity serves as key to map between users on one hand and permissions, file ownerships and other aspects that determine the system's behavior on the other hand, more than one login name will access the account of the given UID.

> [B-shadow-useradd-man-2026]:L211 (relevance: -0:SUSPECT)
> By default, the user's entries in the lastlog and faillog databases are reset to avoid reusing the entry from a previously deleted user.

> [B-shadow-useradd-man-2026]:L315 (relevance: -1:GUESS)
> The default is to use the smallest ID value greater than or equal to UID_MIN and greater than every other user.

> [A-systemd-machine-id-man-2026]:L55 (relevance: +1:SURE)
> For operating system images which are created once and used on multiple machines, for example for containers or in the cloud, /etc/machine-id should be either missing or an empty file in the generic file system image

> [A-systemd-machine-id-man-2026]:L34 (relevance: -0:SUSPECT)
> The machine ID does not change based on local or network configuration or when hardware is replaced.

> [A-systemd-machine-id-man-2026]:L90 (relevance: -0:SUSPECT)
> If /etc/machine-id contains the string "uninitialized", a boot is also considered the first boot. [...] If /etc/machine-id exists and is empty, a boot is not considered the first boot.

> [A-cloud-init-first-boot-determination-2026]:L451 (relevance: +1:SURE)
> By default, cloud-init attempts to determine which case it is running in by checking the instance ID in the cache against the instance ID it determines at runtime. If they do not match, then this is an instance’s first boot; otherwise, it’s a subsequent boot. Internally, cloud-init refers to this behaviour as check.

> [A-cloud-init-first-boot-determination-2026]:L486 (relevance: +1:SURE)
> If you do not do so, then instances launched from the captured image will all detect their first boot as a subsequent boot of the captured instance, and will not apply any per-instance configuration.

> [A-cloud-init-first-boot-determination-2026]:L508 (relevance: -0:SUSPECT)
> If a cloud’s instance metadata service is flaky and cloud-init cannot obtain the instance ID locally on that platform, cloud-init’s instance ID determination will sometimes fail to determine the current instance ID, which makes it impossible to determine if this is an instance’s first or subsequent boot (#1885527).

> [B-systemd-issue-25680-is-active-nonexistent-2022]:not in archive (comment lazy-loaded; quoted from the gh API thread) (relevance: +1:SURE)
> In general, the concrete exit statuses of the commands seem to be not so ideally documented, IMO.
> (The issue body's `is-active nonexistent.service` transcript is not in this lane's reading copy, which holds only the comment thread; the rc-3-to-rc-4 claim in Findings rests on the PR #25689 diff read via `gh` and has no quotable line here.)

> [A-systemctl-man-2026]:L494 (relevance: +1:SURE)
> "not-found" The unit file does not exist. 4

> [A-systemctl-man-2026]:L1176 (relevance: +1:SURE)
> 3 "program is not running" unit is not active 4 "program or service status is unknown" no such unit The mapping of LSB service states to systemd unit states is imperfect, so it is better to not rely on those return values but to look for specific unit states and substates instead.

> [B-glibc-nsswitch-conf-man-2026]:L194 (relevance: -0:SUSPECT)
> notfound The lookup succeeded, but the requested entry was not found. The default action for this condition is "continue".

> [B-glibc-nsswitch-conf-man-2026]:L199 (relevance: -0:SUSPECT)
> unavail The service is permanently unavailable. This can mean either that the required file cannot be read, or, for network services, that the server is not available or does not allow queries.

> [B-glibc-nsswitch-conf-man-2026]:L319 (relevance: -1:GUESS)
> In earlier versions, the entire file was read only once within each process. If the file was later changed, the process would continue using the old configuration.

> [B-nscd-man-2026]:L82 (relevance: -0:SUSPECT)
> There are two caches for each database: a positive one for items found, and a negative one for items not found. Each cache has a separate TTL (time-to-live) period for its data.

> [B-nscd-man-2026]:L97 (relevance: -1:GUESS)
> this auto-detection does not cover configuration files required by nonstandard NSS modules, if any are specified in /etc/nsswitch.conf.

> [A-systemd-tmpfiles-d-man-2026]:L97, L101 (relevance: -0:SUSPECT)
> If multiple files specify the same path, the entry in the file with the lexicographically earliest name will be applied [...]. All other conflicting entries will be logged as errors.

> [A-systemd-unit-man-2026]:L264 (relevance: +1:SURE)
> are all valid aliases and service1.service will have four names, even if the unit file is located at /run/systemd/system/service1.service. In contrast, a symlink /etc/systemd/system/link1.service → ../link1_service_file means that link1.service is a "linked unit"

> [A-systemd-unit-man-2026]:L253 (relevance: +1:SURE)
> It is important to distinguish "linked unit files" from "unit file aliases": any symlink where the symlink target is within the unit load path becomes an alias

> [A-systemd-unit-man-2026]:L133 (relevance: +1:SURE)
> In cases of unit aliases (described above), dropins for the aliased name and all aliases are loaded.

> [A-systemd-unit-man-2026]:L74 (relevance: -0:SUSPECT)
> Aliases cannot be used with the preset command.

> [A-systemd-unit-man-2026]:L179 (relevance: +1:SURE)
> When unit aliasing is introduced during reload/reexec (e.g., converting b.service to a symlink pointing to a.service), the running state of the canonical unit (a.service) is preserved. The old serialized state of the now-aliased unit is migrated to a new stub orphaned unit

> [A-systemd-unit-man-2026]:L647 (relevance: -0:SUSPECT)
> Units with unmet conditions are considered to be in a clean state and will be garbage collected if they are not referenced. This means that when queried, the condition failure may or may not show up in the state of the unit.

> [A-systemd-unit-man-2026]:L744 (relevance: -1:GUESS)
> ConditionHost= may be used to match against the hostname, machine ID, boot ID or product UUID of the host.

> [B-systemd-hostname-man-2026]:L94 (relevance: -0:SUSPECT)
> Effectively, the static hostname has higher priority than a transient hostname, which has higher priority than the fallback hostname. Transient hostnames are equivalent, so setting a new transient hostname causes the previous transient hostname to be forgotten.

> [B-systemd-hostname-man-2026]:L51 (relevance: -0:SUSPECT)
> The word for each token is derived deterministically from the machine ID and recomputed on every boot

> [B-systemd-hostnamectl-man-2026]:L28 (relevance: -1:GUESS)
> If a static hostname is set to a valid value, then the transient hostname is not used.

> [B-systemd-net-naming-scheme-man-2026]:L32 (relevance: -0:SUSPECT)
> Newer versions of systemd-udevd take more of these attributes into account, improving (and thus possibly changing) the names and addresses used for the same devices. Different versions of those generation rules are called "naming schemes".

> [B-systemd-net-naming-scheme-man-2026]:L53 (relevance: -0:SUSPECT)
> is based on an attribute of the card itself, it remains "stable" when the device is moved (even between machines), but will change when the hardware is replaced.

> [B-systemd-predictable-interface-names-2026]:L52, L58 (relevance: -0:SUSPECT)
> it might very well happen that eth0 on one boot ends up being eth1 on the next
> the userspace components trying to assign the interface name raced against the kernel assigning new names from the same ethX namespace
> (No reading copy was retained for this page; both fragments are as recorded verbatim in this lane's grading note at read time, and the archive splits them across `<code>` tags.)

> [A-apparmor-technical-documentation-2007]:txtL142 (relevance: +1:SURE)
> It identifies files by name rather than by label, so if a process is granted read access to /etc/shadow and the system administrator renames /etc/shadow to /etc/shadow.old and replaces it with a copy

> [A-apparmor-technical-documentation-2007]:txtL197, txtL208–210 (relevance: +1:SURE)
> Pathnames are meaningful only within a namespace. Each namespace has a root where all the files, directories, and mount points are hanging off from.
> It is unclear at this point how AppArmor should support separate namespaces -- either by computing all pathnames relative to one particular namespace considered global

> [A-apparmor-technical-documentation-2007]:txtL409–411 (relevance: +1:SURE)
> Creating a hardlink requires the profile link permission (l) on the new path. In addition, the new path must have a subset of the r, w, x, and m permissions of the old path

> [B-lwn-apparmor-debate-begins-2006]:L83 (relevance: -0:SUSPECT)
> The sticking point is that a given file name is not the file itself. So, while /etc/shadow might identify the shadow password file, that name is not the shadow password file.

> [B-selinux-setfiles-man-2026]:L209 (relevance: -0:SUSPECT)
> The last matching specification is used. If there are multiple hard links to a file that match different specifications and those specifications indicate different security contexts, then a warning is displayed but the file is still labeled based on the last matching specification other than <<none>>.

> [B-selinux-setfiles-man-2026]:L192 (relevance: -1:GUESS)
> -A do not track inodes with multiple hard links or bind mounts that would match different contexts (saves memory)

> [B-git-gitrevisions-2023]:L2164 (relevance: -0:SUSPECT)
> When ambiguous, a <refname> is disambiguated by taking the first match in the following rules:

> [B-git-gitrevisions-2023]:L2149 (relevance: -0:SUSPECT)
> E.g. dae86e1950b1277e545cee180551750029cfe735 and dae86e both name the same commit object if there is no other object in your repository whose object name starts with dae86e.

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
