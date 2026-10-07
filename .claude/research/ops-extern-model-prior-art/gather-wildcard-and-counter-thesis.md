# gather-wildcard-and-counter-thesis — unseeded hunt and counter-thesis

311 read at `63e49f29`. graded-by: subagent on every entry. Lane 10 of ten.

## Findings

- Counter-thesis, strongest statement found: Debian's accepted DEP-17 says "At its core, dpkg assumes that every filename uniquely refers to a file on disk", lists twelve failures when `/usr`-merge aliasing broke that across ownership, triggers, diversions, alternatives and statoverrides, and then REJECTS teaching dpkg about aliases in favour of eliminating the aliases. The most battle-tested package manager chose "the name is the thing" and paid for it in file loss [A-debian-dep17-usrmerge-aliasing-2023] +SURE
- Counter-thesis, second: the CMDB standards body says cross-repository identity is done by "any combination of automated analysis and manual input", is "seldom absolute and often must rely on heuristics", and may change later [A-dmtf-cmdbf-federation-dsp0252-2010] +SURE
- Counter-thesis, third: the whole-system provenance community (core authors of SPADE, CamFlow, PASS) says there is "little consensus" on how even a rename should be represented, and W3C PROV does not specify OS-level behaviour [A-chan-cheney-provmark-provenance-expressiveness-2019] +SURE
- Counter-thesis, fourth (a retreat): the IETF SACM endpoint information model delegated "same endpoint" to organisation-chosen attributes plus a domain-minted label, never became an RFC, and the WG concluded [B-ietf-sacm-information-model-draft-2017] ~SUSPECT on the causal link between the identity punt and the retreat
- Nearest unseeded analogue to 311's compare(): SNIA SMI-S Clause 7 "Correlatable and Durable Names" has a three-answer equality test keyed by name format (same format + same name = same; same format + different name = different; different formats = unknown), withholds identity from ambiguous schemes, and is enforced by Windows Server's storage service [A-snia-smis-correlatable-durable-names-2020] [B-microsoft-smis-requirements-windows-2017] +SURE
- Nearest unseeded analogue to 311's committee law and traversal: RETRO (intrusion recovery) has one "repair manager" per object type, each naming objects in its own scheme; a namei lookup records a dependency on every directory entry it crossed; finer "parts" are used only when the manager mediated all writes to the larger object, else coarse-but-safe OS-level dependencies [A-kim-retro-selective-reexecution-2010] +SURE
- Security dependency tracking independently split file (device+inode+version) from filename (the directory data mapping a name to a file) in 2003, and made every path lookup depend on all parent directories [A-king-chen-backtracking-intrusions-2003] +SURE
- Forward build systems (Riker) had to model paths, directory entries, links, pipes and absence ("anti-dependencies") to skip commands safely; correctness rests on one host and "Intercepting system calls is sufficient" [A-curtsinger-riker-incremental-builds-2022] +SURE
- The leading shell-effects tool (`try`, OSDI'26) keys effects by path and reports rename as delete+create: a battle-lite instance of "the name is the thing" [A-lamprou-try-semisolates-osdi-2026] +SURE
- OpenBSD `unveil(2)` decides identity per level: a directory is bound to its object at call time, a non-directory is remembered by name in its parent [A-openbsd-unveil-man-2026]; `pledge(2)` is a deployed at-most effect declaration by category, not by referent [A-openbsd-pledge-man-2026] +SURE
- Windows WOW64 registry documentation is a vendor-published per-key aliasing table (redirected vs shared, observer = process architecture), with a merged view and an indexical alias [A-microsoft-wow64-registry-keys-affected-2022] +SURE
- ITU-T X.720 (1992) is the ancestor of managed-object naming: single-superior containment, RDN unambiguous only within its superior, names reusable after delete, and an explicit punt: the resource-to-managed-object relation "is not modelled in a general way" [A-itu-x720-management-information-model-1992] +SURE
- TxOS solves conflict detection by kernel-object identity from inside the single authority, which 311 cannot occupy (counter-thesis case ii) [A-porter-txos-operating-system-transactions-2009] ~SUSPECT
- Counter-thesis verdict for this lane: no battle-tested project found that models identity across mutually-unknowing authors abstractly and broadly. Every hit falls into one of the four cases: (i) name is the thing (dpkg, try, pledge/unveil for files), (ii) single authority (TxOS, Riker, RETRO, Backtracker: all inside one kernel's view), (iii) delegated to humans/heuristics (CMDBf, SACM, X.720's per-class behaviour), (iv) one domain (SMI-S storage, WOW64 registry). SMI-S is the closest to a cross-author rule, and it is one domain with world-registered name formats ~SUSPECT (absence of evidence from ~25 searches, not proof)

## Candidate table

| [slug] | what it is | exhaustive / broad / abstract / battle-tested | 311 analogues | punts | altitude |
|---|---|---|---|---|---|
| [A-debian-dep17-usrmerge-aliasing-2023] | Debian DEP on `/usr`-merge aliasing | exhaustive for dpkg's path-keyed stores (P1-P12, M1-M23); narrow (files under dpkg); concrete; heavily battle-tested (bugs, archive-wide counts, consensus call) | `:guarantees-unique-name` failing; alias above the leaf; several stores keyed by one path (ownership, triggers, diversions, alternatives, statoverrides) each breaking separately | dpkg's identity = filename; the fix chosen is to remove aliases, not to model them (M1 rejected) | refuted-shape case study / mutation |
| [A-dmtf-cmdbf-federation-dsp0252-2010] | DMTF CMDB Federation standard | not exhaustive (framework); broad (any CI); abstract; moderately deployed (vendor implementations c. 2009-12) | instanceId = (mdrId, localId): MKey-Primary in MParent-Store; "shall never refer to anything except the original item": no-reuse warrant; several instanceIds per reconciled item: several MDerivations; conflicting records | reconciliation is heuristic + manual, may change | schema / read-only federation |
| [A-snia-smis-correlatable-durable-names-2020] | SMI-S Clause 7 | exhaustive within storage (LUNs, ports, fabrics, systems, OS device names); narrow; semi-abstract (NameFormat as a scheme tag); battle-tested (ISO standard, Windows enforces) | NameFormat/NameNamespace = MScheme; 7.7 three-answer compare; CIM key vs correlatable name = MKey-Primary vs cross-namespace identity; VPD80 withheld = no warrant for an ambiguous shape; OS names unique only in scoping system = MParent; durability vs correlatability | "no single storage system name format is in common use"; vendor names "not interoperably correlatable" | naming / read-only assessment |
| [A-kim-retro-selective-reexecution-2010] | RETRO intrusion recovery | exhaustive for its object types; narrow-ish (files, dirs, processes, ptys, sockets, some libc); abstract API; research-tested (honeypot attacks) | per-type managers = committee law; `<bootgen,pid,pidgen,execgen>` = boot-scoped lifecycle + recycled-key generation; dir entry `<dev,part,inode,name>` = MParent-Catalog entry; namei dependency on every entry crossed = emitted MTraversal; finer parts only with full mediation, else coarse = "finer buys sparing, coarse is safe" | kernel trusted; single machine; network via compensating actions | mutation (repair) |
| [A-king-chen-backtracking-intrusions-2003] | BackTracker | exhaustive for process/file/filename; narrow; semi-abstract; research-tested on real intrusions | file object (dev, inode, version) vs filename object (canonical path) = MReferent vs MParent-Catalog entry; parent-directory dependency = MTraversal; version numbers vs recycled keys | canonical absolute path as filename identity (no bind-mount story); low-control events untracked | read-only analysis |
| [A-chan-cheney-provmark-provenance-expressiveness-2019] | ProvMark benchmarking of SPADE/OPUS/CamFlow | 22 syscall families; narrow; empirical; validated by those systems' authors | shows three designs of path-vs-object identity for one rename | the community states no consensus / no OS model | meta / counter-thesis |
| [A-curtsinger-riker-incremental-builds-2022] | Riker forward build system | exhaustive for POSIX FS state in one build; narrow; concrete IR; evaluated on 14 projects | PathRef relative to base ref; Add/RemoveDirEntry; anti-dependencies (absence is a fact); artifact vs path | one host, syscalls assumed sufficient | mutation (elision of commands) |
| [A-lamprou-try-semisolates-osdi-2026] | `try` semisolates | FS effects of arbitrary commands; broad commands, narrow state; concrete; deployed (nixpkgs, AUR) and used by hS/Incr | observed footprint (vs 311's declared writeset); negative dependencies; containment ordering | effects keyed by path; rename = delete+create; submounts, users, devices out of scope | observation |
| [A-openbsd-unveil-man-2026] | `unveil(2)` | FS only; concrete; heavily deployed since 2018 | per-level identity choice (dir = object, file = name in parent); region given whole with more-specific override | no hardlink/bind-mount aliasing story | effect restriction |
| [A-openbsd-pledge-man-2026] | `pledge(2)` | ~3 dozen subsystems; broad; categorical; heavily deployed | at-most, author-declared effect set | categories, not referents | effect restriction |
| [A-microsoft-wow64-registry-keys-affected-2022] | WOW64 registry redirection table | exhaustive for affected keys; narrow (registry); concrete; deployed since 2005, revised at Win7 | observer-dependence (process architecture selects referent); shared keys = one physical copy in two views; HKCR merged view = store that aliases; HKCU symlink = indexical routing key; subkeys inherit unless listed | reflection (sync) retired | naming / aliasing table |
| [A-itu-x720-management-information-model-1992] | OSI management information model | framework; broad (any managed object); abstract; telecom-deployed | containment tree = MParent chain; RDN unambiguous within superior = per-parent warrant; name reuse after delete; per-instance name bindings; indirect effects on related objects | resource-to-managed-object relation not modelled generally; "exists" only if named | naming / schema |
| [A-porter-txos-operating-system-transactions-2009] | TxOS system transactions | 150 of 303 syscalls; narrow; concrete; research | inode split into separately versioned payloads = several cells of one MParent; directories as containers with disjoint-entry writers; parent dirs as read-only traversal | identity = kernel object address; mount, swapon, xattr, sockets unsupported | mutation (isolation) |
| [B-ietf-sacm-information-model-draft-2017] | IETF SACM information model draft | framework; broad for endpoints; abstract; never shipped | "same endpoint" test; identifying attributes graded by multiplicity/persistence/immutability/verifiability; domain label | identity chosen per organisation; draft expired, WG concluded | schema / read-only assessment |
| [B-microsoft-smis-requirements-windows-2017] | Windows consumption of SMI-S | corroboration only | — | — | battle-testing evidence |

## Citations

> [A-debian-dep17-usrmerge-aliasing-2023]:L44-46 (relevance: +1:SURE)
> In the presence of such links, two distinct filenames may refer to the same file on disk. We say that a filename aliases another when this happens.
> At its core, dpkg assumes that every filename uniquely refers to a file on disk. This assumption is violated when aliasing happens. As a result, we exercise undefined behavior in dpkg. This is known to cause problems such as unexpected file loss and is currently mitigated by a file move moratorium.

> [A-debian-dep17-usrmerge-aliasing-2023]:L86, L122 (relevance: +1:SURE)
> When a package uses dpkg-divert to displace a file from another package, the diverted location may have become aliased due to the /usr-merge.
> A statoverride may also be configured as an administrative change. As files are canonicalized, such overrides become ineffective without any warning.

> [A-debian-dep17-usrmerge-aliasing-2023]:L203, L891 (relevance: +1:SURE)
> this is a new feature in dpkg with a fairly involved implementation as it touches on core data structures. Since it adds to the API in non-trivial ways, it is not something we can remove anytime soon, so it adds to the permanent maintenance cost of dpkg.
> Discussion debian-devel@lists.debian.org indicates that the project prefers to finish the transition without relying on changes to dpkg as primary mechanism. This amounts to rejecting M1 and selecting M2.

> [A-dmtf-cmdbf-federation-dsp0252-2010]:p.18-19, spec lines 498-523 (relevance: +1:SURE)
> Managed resources are often identified in multiple ways, depending on the management perspective.
> The federating CMDB performs this identity mapping using any combination of automated analysis and manual input
> The determination of identity is seldom absolute and often must rely on heuristics because different MDRs typically know about different characteristics of an entity and thus establish different sets of identifying properties

> [A-dmtf-cmdbf-federation-dsp0252-2010]:p.19-20, spec lines 542-553 (relevance: -0:SUSPECT)
> Each item shall have at least one ID that is unique within the scope of the MDR that contains it and that serves as a key.
> After an ID has been assigned to an item, it shall never refer to anything except the original item.
> when two items are thought to be different but are later reconciled to the same item; or when an ID changes for any other reason.

> [A-chan-cheney-provmark-provenance-expressiveness-2019]:p.1 (relevance: +1:SURE)
> such standards do not specify how to record operating system-level behaviour, or even when such records are considered "accurate" or "complete". Indeed, as we shall see, in practice there is little consensus about how specific activities (e.g., renaming a file) should be represented in a provenance graph.
> there are as yet no complete formal models of mainstream operating systems such as Linux.

> [A-chan-cheney-provmark-provenance-expressiveness-2019]:§4.1 (relevance: -0:SUSPECT)
> SPADE represents a rename using two nodes for the new and old filenames ... CamFlow represents a rename as adding a new path associated with the file object; the old path does not appear in the benchmark result.

> [B-ietf-sacm-information-model-draft-2017]:L8311-8330 (relevance: -0:SUSPECT)
> Tell whether two endpoint attribute assertions concern the same endpoint
> Ideally, every endpoint would be identified by a unique identifier present on the endpoint, but, this is complicated due to different factors such as the variety of endpoints on a network, the ability of tools to reliably access such an identifer, and the ability of tools to correlate disparate identifiers.
> The set of attributes that uniquely identify an endpoint on a network will likely vary by organization

> [A-snia-smis-correlatable-durable-names-2020]:p.59 (relevance: +1:SURE)
> A management application understands when objects in different namespaces represent the same managed resource by the use of a unique common identifier, referred to as a "correlatable name".
> No name is permanently durable (e.g., even a name derived from hardware may change due to FRU replacement).
> CIM key-value combinations are unique across instances of a class, but CIM does not fully address cases where different types of identifiers are possible on different instances of an object.

> [A-snia-smis-correlatable-durable-names-2020]:p.68-69, §7.7 (relevance: +1:SURE)
> If the two objects have the same NameFormat and Name, then they refer to the same resource.
> If the two objects have the same NameFormat and different Names, then they refer to different resources.
> If the two objects have different NameFormats, whether the Names are the same or different, then it is unknown whether they refer to the same resource.

> [A-snia-smis-correlatable-durable-names-2020]:p.60, p.62, p.67 (relevance: +1:SURE)
> There's no mechanism to discover which approach the device is using. If a client is not coded to understand which products provide per-logical unit or per-target serial numbers, then it should not use the Unit Serial Number VPD page as a logical unit name.
> At this time, no single storage system name format is in common use.
> Operating system device names are unique within the namespace of the scoping system and are not unique between systems.

> [B-microsoft-smis-requirements-windows-2017]:L727 (relevance: -0:SUSPECT)
> Special attention must be paid to Clause 7: Correlatable and Durable Names. Failure to follow these requirements will prevent the SMI-S provider from working properly - particularly masking operations - and that provider will not be supported for use with Windows.

> [A-kim-retro-selective-reexecution-2010]:p.8, §4.4 (relevance: +1:SURE)
> The challenge in supporting refinement in the action history graph lies in dealing with multiple objects representing the same state.
> If the appropriate manager mediated all modifications to the larger object (such as a directory inode), and the manager was not compromised, RETRO can safely use finer-grained objects (such as individual directory entry objects). Otherwise, RETRO uses coarse-grained but safe OS-level dependencies.

> [A-kim-retro-selective-reexecution-2010]:p.9-10, §5.1-5.3 (relevance: +1:SURE)
> The manager names each process in the graph by bootgen, pid, pidgen, execgen . bootgen is a boot-up generation number to distinguish process IDs across reboots.
> The directory manager names each directory entry by device, part, inode, name
> such as name lookups in namei (which incur a dependency from every directory entry traversed)

> [A-kim-retro-selective-reexecution-2010]:p.11, §5.4 (relevance: -0:SUSPECT)
> unlike the directory manager, which mediates all accesses to a directory, a manager for a function in libc cannot guarantee that an attacker will not bypass it

> [A-king-chen-backtracking-intrusions-2003]:p.2, §2.1 (relevance: +1:SURE)
> A file is identified uniquely by a device, an inode number, and a version number. Because files are identified by inode number rather than by name, BackTracker tracks a file across rename operations.
> A filename object refers to the directory data that maps a name to a file object. A filename object is identified uniquely by a canonical name, which is an absolute pathname with all ./ and ../ links resolved.
> In Unix, a single file can appear in multiple places in the filesystem directory structure, so writing a file via one name will affect the data returned when reading the file via the different name.

> [A-king-chen-backtracking-intrusions-2003]:p.3, §2.2.3 (relevance: -0:SUSPECT)
> In addition, the process is affected by all parent directories of the filename (e.g., opening the file /a/b/c depends on the existence of /a and /a/b).

> [A-curtsinger-riker-incremental-builds-2022]:p.887 (relevance: -0:SUSPECT)
> correct builds must model not just files, but inode metadata, directories, symbolic links, hard links, pipes, and sockets. Correct build systems must also model the absence of such state.

> [A-curtsinger-riker-incremental-builds-2022]:p.889, p.893 (relevance: -0:SUSPECT)
> The observed sequence of failures is a build dependency, and any change in failures implies that a build must rerun. We call failing resolutions anti-dependencies.
> A3. Intercepting system calls is sufficient to determine all dependencies.

> [A-lamprou-try-semisolates-osdi-2026]:p.456 (relevance: -0:SUSPECT)
> Effects are made available to the semisolate caller as a combination of (1) filesystem paths or stream identifiers, and (2) optional metadata such as effect type and success.

> [A-lamprou-try-semisolates-osdi-2026]:p.462, p.464 (relevance: -1:GUESS)
> instead of reporting a -> b, try reports equivalent a (deleted) b (created) pairs.
> the effect of creating file f in directory d depends on the effect of creating directory d.

> [A-openbsd-unveil-man-2026]:L220-231 (relevance: +1:SURE)
> A path that is a directory will enable all filesystem access underneath path using permissions if and only if no more specific matching unveil() exists at a lower level. Directories are remembered at the time of a call to unveil(). This means that a directory that is removed and recreated after a call to unveil() will appear to not exist.
> Non-directory paths are remembered by name within their containing directory, and so may be created, removed, or re-created after a call to unveil() and still appear to exist.

> [A-openbsd-pledge-man-2026]:L171-173 (relevance: -1:GUESS)
> The pledge() system call separates the POSIX feature set into a group of approximately 3 dozen subsystems. By calling pledge() the program can declare which subsystems it will need in the future

> [A-microsoft-wow64-registry-keys-affected-2022]:(intro, table notes) (relevance: +1:SURE)
> Other registry keys are shared by applications of differing processor architectures on affected Windows installations. WOW application registry calls to shared keys are not redirected. Instead, one physical copy of the key is mapped into each logical view of the registry.
> Subkeys of the keys in this table inherit the parent key's behavior unless otherwise specified.
> HKEY_CURRENT_USER is a symbolic link to HKEY_USERS\[SID] where [SID] indicates a match for the current user's security ID (SID).
> HKEY_CLASSES_ROOT is a merged view of HKEY_LOCAL_MACHINE\SOFTWARE\Classes and HKEY_CURRENT_USER\SOFTWARE\Classes.

> [A-itu-x720-management-information-model-1992]:p.5, §5 (relevance: +1:SURE)
> The relationship that exists between the resource and the managed object as an abstraction of that resource is not modelled in a general way; that is, the precise properties abstracted and the specific effects of management operations on a resource must be specified as part of the managed object class specification.
> A managed object exists, from a management point of view, if it has a distinguished name (as defined in 6.3.2) and supports the operations and notifications defined for its class. Otherwise, it does not exist from a management point of view, even if a physical counterpart exists.

> [A-itu-x720-management-information-model-1992]:p.14, §5.3.3.1 (relevance: -0:SUSPECT)
> A management operation that is performed on one or more attributes in a managed object can result in other observable changes; these are called indirect effects. Indirect effects are the result of the relationships in the underlying resource.

> [A-itu-x720-management-information-model-1992]:p.24-25, §6.2-6.3.2 (relevance: +1:SURE)
> The name of an object that is unambiguous in a local naming context, may not be so in some larger naming context.
> Supported name bindings are, therefore, not a property of the object class as a whole, and individual instances of the same object class may use different name bindings.
> its semantics must permit its value to remain fixed for the lifetime of each managed object that uses it for naming.
> When a managed object is deleted, the value assigned to its naming attribute becomes available for re-use, to identify subsequent managed objects created within the same superior object.

> [A-porter-txos-operating-system-transactions-2009]:§3.1.1, §5.1 (relevance: -1:GUESS)
> the kernel enforces the invariant that a kernel object may only have one writer at a time, excepting containers, which allow multiple writers to disjoint entries.
> TxOS decomposes an object into multiple data payloads when it houses data that can be accessed disjointly.
> Many kernel objects are only read in a transaction, such as the parent directories in a path lookup.

## Seams this lane would REDRAW (one line each)

- Standards models (lane 1): read X.720's containment/RDN rules before CIM; CIM inherits them, and SMI-S Clause 7 is where CIM's own keys were found insufficient for cross-namespace identity ("CIM does not fully address cases...") [A-itu-x720-management-information-model-1992] [A-snia-smis-correlatable-durable-names-2020]
- Standards models / CMDB: DMTF CMDBf is the normative CMDB identity statement and its punt; ServiceNow IRE is a commercial descendant of the same problem [A-dmtf-cmdbf-federation-dsp0252-2010]
- Kernel/package specs (lane 7): DEP-17 is the single most useful dpkg document for 311; it is identity, not packaging [A-debian-dep17-usrmerge-aliasing-2023]
- Academic verification/theory (lanes 5, 6): add a security/provenance seam (BackTracker, RETRO, ProvMark, CamFlow) and a build-systems seam (Riker, Forward Build Systems Formally); both had to name OS objects for correctness and neither is in the nine [A-kim-retro-selective-reexecution-2010] [A-curtsinger-riker-incremental-builds-2022]
- Service/network specs (lane 8): SMI-S Clause 7 also covers port and fabric naming (WWN, iSCSI iqn/eui/naa, MAC), with "IP addresses ... not necessarily unique (e.g., NAT...)" as a stated reason not to use them [A-snia-smis-correlatable-durable-names-2020]
- Naming-theory classics (lane 10's sibling): the Windows WOW64 table and unveil are small, deployed, exact instances of per-observer and per-level naming decisions that the classics describe abstractly [A-microsoft-wow64-registry-keys-affected-2022] [A-openbsd-unveil-man-2026]

## Breadth against the 47 items

- Files and inodes (5, 6, 7, 8): BackTracker, RETRO, Riker, try, unveil, TxOS, DEP-17 (7 especially: hardlinks and aliases).
- Directory-entry and path identity (7, 30's file half, 36's link): RETRO, BackTracker, Riker, DEP-17 (P4 alternatives), unveil.
- dpkg status/version/alternatives (14, 15, 36): DEP-17 only (ownership, alternatives P4, statoverride P5).
- Process environment and pids (37): RETRO process naming only.
- Block devices and mounts (23, 46, 47): SMI-S (OS device names, LUN names), try (submount limitation).
- Interfaces/addresses (21, 22): SMI-S port names (MAC, WWN, IP caveats) only.
- Registry-like keyed config (9, 44, 45): WOW64 table by analogy only.
- Hosts/endpoints as wholes (30, and the vantage generally): SACM designation, SMI-S system names, CMDBf items.
- Not covered by any hit: sysctls (11-13), users/groups (17-20), services (1-4), containers/k8s (24-26), DNS (27), firewall rules (29, 31, 41), cron (28), certs (33), SELinux (34), timezone (35), SQL (42), pip (43), resolv.conf (40), swap (32). ~SUSPECT that this is a real gap of the unseeded seams, not of the search.

## Leads not pulled

- ServiceNow Identification and Reconciliation Engine docs · the commercial CMDB's identification rules (independent vs dependent CIs identified "in the context of their parent") are the practitioner form of CMDBf's punt · servicenow.com robots.txt returned 403 to the fetch tool, so not fetched (respecting it) · human can open https://www.servicenow.com/docs/r/servicenow-platform/configuration-management-database-cmdb/ire.html and paste; grade D at best, useful only as a problem statement
- 3GPP TS 32.300 "Name convention for Managed Objects" · confirms telecom inherits X.500 DNs (mirror text: "Network resources shall be named using the naming conventions in ITU-T Recommendation X.500 ... does not support multi-value RDN") · ETSI PDF is encrypted/403 to curl; only a thin mirror page read (itecspec.com) · try 3gpp.org archive zip by hand
- TM Forum SID · paywalled; not attempted beyond seed list · likely inherits the same DN model
- Chen et al., "Understanding and Discovering Software Configuration Dependencies in Cloud and Datacenter Systems", FSE 2020 · an exhaustive empirical taxonomy of intra- and cross-component config dependencies over 16 systems; would inform may-read / cross-MSort entailment rather than identity · PDF at https://www.cs.cornell.edu/~legunsen/pubs/ChenETAL20CDep.pdf, not read for budget
- CfgNet (TSE 2023) · tracks "equality-based" dependencies across artifacts (same port in Dockerfile and app config): a value-level cousin of `:corresponds` · computer.org landing only
- "Forward Build Systems, Formally" (Spall, Mitchell, Tobin-Hochstadt, 2022) · formal correctness of trace-based build systems; sibling of Riker · https://ndmitchell.com/downloads/paper-forward_build_systems_formally-17_jan_2022.pdf
- CamFlow "Practical Whole-System Provenance Capture" (SoCC 2017) · defines kernel-object state-versions (inode, path as separate entities) · https://tfjmp.org/publications/2017-socc.pdf
- Taser (SOSP 2005) · predecessor to RETRO; its NoIAN policy ignores "file name and attributes" (per RETRO's comparison table), a direct evidence line for identity being dropped for tractability · https://www.eecg.toronto.edu/~ashvin/publications/sosp-2005.pdf
- Puppet PUP-1073 "Package duplicate resource issue" and the `alias` metaparameter · where Puppet litigated same-name-different-provider uniqueness; lane 3 (CM big four) territory · https://groups.google.com/g/puppet-dev/c/LatVZFUkwEM
- Terraform plugin "Resource Identity" (2025) · "must correspond to at most one remote object per provider ... must not change during its lifecycle" (search snippet only) · lane 3 · https://developer.hashicorp.com/terraform/plugin/sdkv2/resources/identity
- Android package identity (package name + signing key; shared UID deprecated in Android 13) · AOSP app-signing page read, but it does not state the same-name/different-signer refusal rule itself · weak; not registered
- IETF SACM terminology draft (draft-ietf-sacm-terminology-16) · would give the WG's definitions of target endpoint identification · -17 URL 404; try -16

## Search log

- kagi · unix command effects formal model; OS config ontology; CMDB identification/reconciliation; GDMO naming · 1 kept
- kagi · "what commands touch"; syscall footprint corpus; package-manager semantics; config identity out-of-scope survey · 0 kept
- kagi · Backtracker; RETRO; Taser; provenance/ProvMark · 3 kept
- kagi · SMI-S correlatable; pledge; try; TxOS · 4 kept (SMI-S, Microsoft SMI-S, pledge, unveil via pledge), TxOS later
- kagi · ServiceNow IRE; 3GPP 32.300; WOW64 registry; Android signing · 1 kept (WOW64)
- kagi · CMDBf; identity reconciliation unsolved; Terraform import identity · 1 kept (CMDBf)
- kagi · HN shell command effects; Reddit sysadmin change tracking; CM survey naming; Puppet namevar · 0 kept
- github · binpash/try README · led to OSDI'26 paper, 1 kept
- kagi · Puppet alias uniqueness; "not a solved problem" CM identity; osquery identity; Linux audit watch semantics · 0 kept
- kagi · IETF SACM endpoint identification; SACM concluded; NIST SP 800-128 · 1 kept (SACM IM draft)
- datatracker API · SACM drafts/RFCs · status evidence only
- kagi · Riker; forward build systems · 1 kept
- kagi · DEP-17; dpkg aliasing · 1 kept
- kagi · CDep FSE'20; OS-state knowledge graph; config dependency taxonomy · 0 kept (leads)
- Total ≈ 26 Kagi queries in 13 calls; 17 sources read and graded (12 confirmed registered at hand-off, 5 pending; see Tooling problems).

## Tooling problems

- servicenow.com robots.txt returns 403 to mcp fetch; not bypassed.
- ETSI deliver PDFs return 403 to curl and are encrypted when fetched via mcp fetch; not decrypted.
- DUPLICATE: the TxOS paper (same URL) is registered twice, as my [A-porter-txos-operating-system-transactions-2009] and a sibling lane's [B-txos-system-transactions-2009]; both were queued concurrently, so neither pre-check saw the other. The two lanes graded it A vs B; the conductor should pick one.
- UNCONFIRMED at hand-off (all fetched and read; only registration is pending). Entry JSON files are in `$SC`:
  - [A-chan-cheney-provmark-provenance-expressiveness-2019]: timed out, `entry-A-chan-cheney-provmark-provenance-expressiveness-2019.json`
  - [A-dmtf-cmdbf-federation-dsp0252-2010]: timed out, `entry-A-dmtf-cmdbf-federation-dsp0252-2010.json`
  - [A-microsoft-wow64-registry-keys-affected-2022]: queued, `entry-A-microsoft-wow64-registry-keys-affected-2022.json`
  - [A-lamprou-try-semisolates-osdi-2026]: queued, `entry-A-lamprou-try-semisolates-osdi-2026.json`
  - [B-ietf-sacm-information-model-draft-2017]: queued, `entry-B-ietf-sacm-information-model-draft-2017.json`
  Citations for these use printed page/section locators (or the local scratch copies' line numbers for SACM), not archive lines.
- LOCK TIMEOUT: the wrapper reported `register: lock timeout after 600s` for [A-chan-cheney-provmark-provenance-expressiveness-2019] and [A-dmtf-cmdbf-federation-dsp0252-2010] (entry JSON for the latter at `$SC/entry-A-dmtf-cmdbf-federation-dsp0252-2010.json`). Per the brief I stopped launching new registrations after that. Its entry JSON is ready at `$SC/entry-A-chan-cheney-provmark-provenance-expressiveness-2019.json` for the conductor to register; until then validate.sh will flag that slug as unresolved. Final state (re-checked later): ProvMark, CMDBf, WOW64 and try have since landed in sources.json (manifest and sources/ both at 183, consistent); only [B-ietf-sacm-information-model-draft-2017] is still unregistered, its entry JSON is in the Pending section below.
- PDF sources are cited by printed page or section rather than archive line numbers (the archive copies are PDFs); SMI-S cites printed pages 59-70 of a 216-page book of which only Clause 7 was read.

## Pending entry JSON (verbatim; register with `sh register.sh $RD <slug> < entry.json`)

The scratchpad copies are in a temp dir and may not survive; these are the durable copies.

### A-chan-cheney-provmark-provenance-expressiveness-2019

```json
{"url":"https://arxiv.org/pdf/1909.11187","grading-certainty":"+1:SURE","grading-reasoning":"not B: peer-reviewed (Middleware 2019; the arXiv copy read in full is the authors' version), and the author list includes the core developers of the systems compared (Gehani and Irshad for SPADE, Pasquier for CamFlow, Seltzer for PASS), who validated the results. That first-party standing to state the gap is the deciding factor for A over B.","relevance-certainty":"+1:SURE","relevance-description":"Counter-thesis evidence with standing: the whole-system provenance community (which had to name OS objects to build causal graphs) states that W3C PROV does not specify how to record OS-level behaviour, that there is 'little consensus' on how even a rename should be represented, and that no complete formal model of mainstream OSes exists. Also shows three mature recorders modelling one rename three ways (path node vs file-object node vs path-attached-to-object), i.e. the file/filename identity split is re-decided per tool. Breadth: 22 syscall families on files, processes, permissions, pipes.","graded-by":"subagent","published":"2019-09","via":"mcp__kagi-ken__kagi_search_fetch(queries: ['whole-system provenance kernel object identity inode version CamFlow SPADE representation comparison ProvMark'])"}
```

### A-dmtf-cmdbf-federation-dsp0252-2010

```json
{"url":"https://www.dmtf.org/sites/default/files/standards/documents/DSP0252_1.0.1_0.pdf","grading-certainty":"-0:SUSPECT","grading-reasoning":"A not B: first-party normative DMTF standard written by the CMDB vendors' working group (BMC, CA, HP, IBM, Microsoft and others), so primary on what cross-repository identity the industry agreed to standardise. Certainty SUSPECT: clauses 1-5 (scope, terms, architecture, identity reconciliation, data elements) were read in full; the query/registration protocol schemas (clauses 6-9) were only scanned. Battle-testing is moderate: vendor implementations existed around 2009-2012, but adoption was limited.","relevance-certainty":"+1:SURE","relevance-description":"The ITIL/CMDB world's normative statement of the strangers problem and its punt. Analogues: an item's instance ID is (mdrId, localId), unique only within the assigning repository (an MKey-Primary scoped in its MParent-Store); an assigned ID 'shall never refer to anything except the original item' (a never-reused warrant); a reconciled item carries several instance IDs, one per repository (several MDerivations for one MTopic); records from different sources about one item may contradict. The punt: identity reconciliation is done by 'any combination of automated analysis and manual input', is 'seldom absolute and often must rely on heuristics', and may change later. This is counter-thesis case (iii), identity delegated to heuristics and humans, stated by the standard itself.","graded-by":"subagent","published":"2010-04","via":"mcp__kagi-ken__kagi_search_fetch(queries: ['DMTF CMDBf DSP0252 federation specification identity reconciliation instanceId alternateInstanceId'])"}
```

### A-microsoft-wow64-registry-keys-affected-2022

```json
{"url":"https://learn.microsoft.com/en-us/windows/win32/winprog64/shared-registry-keys","grading-certainty":"+1:SURE","grading-reasoning":"not B: first-party normative documentation of Microsoft's own registry behaviour, read in full, describing a mechanism deployed on every 64-bit Windows since XP x64 (2005) and revised across versions (the table records which keys changed from redirected/reflected to shared at Windows 7). Primacy about its own product plus two decades of deployment decides A.","relevance-certainty":"+1:SURE","relevance-description":"A battle-tested, vendor-published ALIASING TABLE for one keyed hierarchical store: for each key path it states whether a 32-bit observer's path is redirected to a different physical key or shares one physical copy mapped into each view, with subkeys inheriting the parent's rule unless listed. Maps to 311's observer-dependence (which MVantage asks decides which MReferent a path reaches), region given whole with more-specific override, :aliases-nothing-else failing for merged views (HKEY_CLASSES_ROOT 'is a merged view' of HKLM and HKCU Classes), and indexical routing keys (HKEY_CURRENT_USER 'is a symbolic link to HKEY_USERS\\[SID]', like /proc/self). Also shows the vendor retreating from a synchronisation mechanism (reflection removed at Windows 7). Domain: Windows registry only.","graded-by":"subagent","published":"2022-03-25","via":"mcp__kagi-ken__kagi_search_fetch(queries: ['Windows registry redirector WOW64 shared keys reflection documentation'])"}
```

### A-lamprou-try-semisolates-osdi-2026

```json
{"url":"https://www.usenix.org/system/files/osdi26-lamprou.pdf","grading-certainty":"+1:SURE","grading-reasoning":"not B: peer-reviewed OSDI'26 paper (best paper and artifact awards per the repo README), read in full, by the tool's own authors; the tool is deployed (AUR, nixpkgs) and used by other research systems (hS, Incr). Its deployment is two to three years old, which limits battle-testing, but source quality is A.","relevance-certainty":"-0:SUSPECT","relevance-description":"The most direct 'what did a shell command touch' tool in the literature, from the PaSh/shell-analysis group, used for dependency tracking between opaque commands and for mining partial command specs. Its effect vocabulary is punt (i), 'the name is the thing': effects are reported as filesystem paths or stream identifiers with a read/write type and success flag; a rename is reported as delete + create pairs, losing identity; overlayfs cannot see submounts; other users and devices are out of scope. It does record negative dependencies (a lookup of an absent path is an effect) and notes containment ordering (creating f in d depends on creating d). Relevant as the observed-footprint counterpart to 311's declared at-most writeset, and as evidence that the leading shell-effects system keys effects by path.","graded-by":"subagent","published":"2026-07","via":"mcp__github__get_file_contents(owner: 'binpash', repo: 'try', path: 'README.md')"}
```

### B-ietf-sacm-information-model-draft-2017

```json
{"url":"https://www.ietf.org/archive/id/draft-ietf-sacm-information-model-10.txt","grading-certainty":"+1:SURE","grading-reasoning":"B not A: a working-group Internet-Draft (first-party IETF SACM output, lead-authored from NIST) that expired in 2017 without becoming an RFC, so it never carried consensus approval; the datatracker lists the SACM group as Concluded and its only RFCs as use cases, requirements, SWIMA and CoSWID. Not C: primary WG text, not commentary. Identity-relevant sections (4.4, 4.6, 6.5, 6.6, 11.1) were read in full; the ~8,800-line element registry was only grepped.","relevance-certainty":"-0:SUSPECT","relevance-description":"Counter-thesis evidence with a retreat attached. The IETF's attempt at a vendor-neutral endpoint posture information model states that telling 'whether two endpoint attribute assertions concern the same endpoint' needs identifying attributes whose choice 'will likely vary by organization', and then lets a domain mint a target-endpoint label in place of them; references between elements need not use unique identifiers and may identify by content. Identity is delegated to each organisation (counter-thesis case iii). The model never shipped as an RFC; the group concluded after publishing only software-inventory formats. Scope: whole endpoints (hosts, devices), not the items on them.","graded-by":"subagent","published":"2017-04","via":"mcp__kagi-ken__kagi_search_fetch(queries: ['IETF SACM endpoint identification target endpoint characterization problem RFC 8248'])"}
```
