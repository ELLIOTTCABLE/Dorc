# gather-academic-verification-of-configuration-code — projects that had to model ops effects to verify, test, or repair real configuration code

Lane 5a. 311 read at `63e49f29` (`git rev-parse --short HEAD`). Every grade here is `graded-by: subagent`.
Line numbers below are `pdftotext -layout` lines of the archived PDF (checked byte-identical against my
reading copies for the five PDFs archived before writing), or raw source lines for the `.html` code copy.

## Findings

- The counter-thesis holds for every project that verified, tested, or repaired configuration code
  itself. CoLiS, Rehearsal, Tortoise, Citac, and Mancoosi all key the world by path (or by a flat
  name inside one registry), and each one either assumes aliasing away or says it is out of scope.
  The projects that did not punt all model one filesystem (SibylFS, Ntzik–Gardner, TxOS kernel
  objects). None of them models packages, services, or users.
  [A-colis-platform-maintainer-scripts-2022] [A-rehearsal-puppet-determinacy-2016] [B-tortoise-puppet-repair-2017]
  [A-citac-reliable-convergence-2016] [B-mancoosi-d21-system-configuration-metamodel-2009] [A-sibylfs-posix-oracle-2015]
  [A-ntzik-gardner-posix-fusion-logic-2015] [A-porter-txos-operating-system-transactions-2009] +SURE
- Headline punt, in the authors' own words: CoLiS's path resolution "simply ignore[s]" symbolic
  links, because the feature-tree logic cannot express them finitely. The model is a tree with
  structural equality, so it has no hard links either. The authors judged the gap "not noticeable in
  practice" over about 27k Debian scripts. That judgement stands behind 152 bug reports and 109
  fixes. [A-colis-platform-maintainer-scripts-2022] [B-colis-unix-utilities-specification-2019] +SURE
- Half-exception (FSMove): it says outright that "every file is associated with an inode rather than
  a path". It tracks inodes, fd tables, cwd sharing across clone, and symlinks, so that resolution is
  right. Its interference relation, however, is keyed by the (symlink-dereferenced) path string. The
  code mints a fresh inode for any path not yet seen in the trace, so hard links and bind mounts that
  existed before the run are invisible. This is 311u's "unequal names are different things", adopted
  trace-locally. [A-fsmove-puppet-fault-detection-2020] [B-fsmove-fstrace-domains-source-2020] +SURE
- The one battle-tested non-punt is SibylFS. Its state is a heap of directory and file references,
  and its API "permits arbitrary linking and unlinking". Its test generator has a class for "different
  paths to the same file (hard links)". It was checked against 21,070 traces on about 40
  configurations. It is filesystem-only, with no mounts in the model and no `*at` calls.
  [A-sibylfs-posix-oracle-2015] +SURE
- Closest academic twin of 311's traversal and routing-mutation story: Ntzik–Gardner's fusion logic.
  A path is a footprint over every directory it crosses, and "the local update has a global effect" on
  overlapping paths, even paths to disjoint entries. The logic separates a path footprint (shared) from
  an update footprint (owned). With it they found that GNU, BusyBox, and FreeBSD `rm -r` all mishandle
  `rm -r /tmp/a/b/../..`. [A-ntzik-gardner-posix-fusion-logic-2015] +SURE
- An implemented interference relation over real kernel state (TxOS). Conflict is per kernel object,
  not per path. A path lookup is a read of each parent directory, which is a traversal treated as a
  read set. Inode metadata and page data are versioned separately, much like 311's cells. Directory
  lists allow concurrent inserts and deletes of distinct entries, but iterating a list conflicts with
  any insert. That is 311's split between a catalog given whole and its member entries.
  [A-porter-txos-operating-system-transactions-2009] ~SUSPECT (partial read)
- Two projects that reach 311u's refuted shape "unequal paths ⇒ disjoint" as a deliberate choice or
  a proposed optimisation. Citac's future work proposes omitting preservation tests "for file
  resources with non-overlapping file paths". Rehearsal's commutativity check is a path-keyed R/W/D
  abstract interpretation and says "Puppet manifests have no aliasing".
  [A-citac-reliable-convergence-2016] [A-rehearsal-puppet-determinacy-2016] +SURE
- Rehearsal's "D" value (a guarded mkdir that only ensures a directory exists) exists to stop
  shared-parent writes from colliding: "false sharing" over `/usr`, `/etc`. This coincides with the
  problem 311 §2.6 solves by excluding containers at or above the level two keys share.
  [A-rehearsal-puppet-determinacy-2016] ~SUSPECT (analogy, not identity)
- Breadth yardstick: Citac snapshots persistent state as a copy-on-write filesystem diff attributed
  by strace. Its transient-state list is interfaces, routes, listening sockets, mounted filesystems,
  and processes. The 2013 prototype tracked "network routes, OS services, open ports, mounted file
  systems, file contents and permissions, OS users and groups, cron jobs, installed packages, and
  consumed resources", as flat key→value properties. Interference is semantic: b preserves a iff
  a;b;a = a;b. [A-citac-reliable-convergence-2016] [B-hummer-testing-idempotence-iac-2013] +SURE
- A project the seed list lacked: Mancoosi (EU FP7, 2008–2011). To simulate and roll back Debian/RPM
  upgrades, it had to write a system-configuration metamodel: installed packages, running services,
  package settings, devices, filesystem, and an environment of modules, shared libraries, and
  processes. It derived that model empirically by clustering about 25k Lenny maintainer scripts
  (9,061 hand-written ones into 116 templates and 10 classes). Its answer to the static-analysis
  problem was to replace scripts with a non-Turing-complete DSL.
  [B-mancoosi-d21-system-configuration-metamodel-2009] [B-mancoosi-d32-maintainer-script-dsl-2009] +SURE
- Package identity in these models is always provider plus name. Tortoise models a package as the
  marker path `dpkg://vim`. Rehearsal models it as the file list from `apt-file`, which misses postinst
  effects. CoLiS uses static contents plus a `package-owns-file` oracle. None handles `provides` or
  virtual packages, which is GOTCHAS `distinct-names-alias-within-a-kind`.
  [B-tortoise-puppet-repair-2017] [A-rehearsal-puppet-determinacy-2016] [B-colis-unix-utilities-specification-2019] +SURE
- GRoot (DNS only, exhaustive) states two committee-law-shaped limits. It can only verify the zone
  files it is given, and "end-to-end correctness ... hinges on other organizations doing the same".
  It also does not model caches, server failures, or unreachability. It models resolution as
  set-valued, matching GOTCHAS `resolution-is-set-valued`. [A-groot-dns-verification-2020] +SURE
- Battle-testing ranking by independent checkability:
  - SibylFS: Austin Group POSIX bugs, FS defects [A-sibylfs-posix-oracle-2015]
  - CoLiS: 152 Debian BTS reports [A-colis-platform-maintainer-scripts-2022]
  - FSMove: 92 faults, 62 fixed upstream [A-fsmove-puppet-fault-detection-2020]
  - GRoot: production zone cleanup [A-groot-dns-verification-2020]
  - Citac: 263 non-idempotent Chef tasks, CHEF-4236 [B-hummer-testing-idempotence-iac-2013]
  - Rehearsal: 13 manifests [A-rehearsal-puppet-determinacy-2016]
  - Tortoise: 42 injected scenarios, self-judged [B-tortoise-puppet-repair-2017]

  ~SUSPECT
- How much bug-finding the effect models themselves did: in CoLiS only 4 of 151 bugs came from
  symbolic execution over the file model and 3 from formalising `dpkg-maintscript-helper`. The rest
  came from the parser, corpus mining, and translation. [A-colis-debian-installation-scenarios-2020] +SURE

## Candidate table

| [slug] | what it is | exhaustive / broad / abstract / battle-tested | 311 analogues | punts | altitude |
|---|---|---|---|---|---|
| [B-colis-unix-utilities-specification-2019] | per-utility effect spec (cp, mv/rename, rm, rmdir, mkdir, touch, test/[ ×13, which, dpkg-maintscript-helper ×4) | exhaustive per argument shape with success/failure/unknown cases; narrow (fs + one dpkg helper); abstract (feature-tree logic); tested via the platform papers | writeset per matched shape; `similar(r,r',…,f)` frame ≈ "everything except entry f unchanged"; `resolve()` macro ≈ traversal of feature edges | symlinks never followed by `resolve`; attributes dropped; tree = no hard links | mutation modelling |
| [A-colis-debian-installation-scenarios-2020] | CoLiS on 28,814 Debian scripts / 113,328 scenarios | broad corpus; battle-tested (151 BTS bugs) | idempotency as self-composition | ownership, permissions, timestamps, symlinks, multiple hard links, concurrency | mutation modelling |
| [A-colis-platform-maintainer-scripts-2022] | journal version, bullseye rerun, spec self-checks and spec-vs-real testing | completeness/coherence/determinism checks per spec; 152 bugs, 109 fixed | — | "symbolic links is simply ignored"; `cp -R` overlap over-approximated | mutation modelling |
| [A-rehearsal-puppet-determinacy-2016] | Puppet → FS language → SMT determinacy/idempotence | abstract; resources (file, package, user, group, cron, ssh key, host) compiled to path ops; weakly tested (13 manifests) | R/W/D commutativity ≈ may-read/may-write collision; D ≈ container exclusion | "no aliasing"; no hard links; `exec` excluded; postinst effects missed | verification |
| [B-tortoise-puppet-repair-2017] | shell-trace-driven manifest repair | path → (state, contents, owner, mode); packages as `provider://name` | observed writes as assertions (measurement) | processes, services, /proc, background daemons out of scope | repair |
| [A-citac-reliable-convergence-2016] | model-based convergence testing of Puppet | state-agnostic, snapshot-based; 101 Forge modules, 250,805 steps, 5 new bugs | preservation a;b;a = a;b ≈ behavioural DISJOINT; state/input/output-dependency taxonomy | proposes skipping tests for non-overlapping paths; persistent/transient only | testing |
| [B-hummer-testing-idempotence-iac-2013] | Citac precursor on Chef | flat K→V state; 298 cookbooks, 263 non-idempotent tasks | "non-conflicting" relation needs per-domain knowledge (≈ owner speech) | per-domain knowledge left to the tester | testing |
| [A-fsmove-puppet-fault-detection-2020] | strace-based missing-dependency and missing-notifier detection | fs only; 354 modules, 92 faults, 62 fixed | consumed/produced/expunged per block ≈ read/write sets; inode-aware resolution | effect key is the path; pre-existing aliases invisible | dynamic analysis |
| [B-fsmove-fstrace-domains-source-2020] | FSMove's `domains.ml` | code-level proof of the key choice | — | `to_inode` mints fresh inodes per unseen path | code |
| [A-sibylfs-posix-oracle-2015] | executable POSIX/Linux/OS X/FreeBSD fs spec and trace oracle | exhaustive for ~25 fs calls; 21,070 tests; about 40 configurations | references ≈ MKey-Primary/MToken; path resolution a separate module ≈ `resolve()` | single namespace, no mounts, no `*at` calls, no FIFOs, no crash | read+write semantics |
| [A-ntzik-gardner-posix-fusion-logic-2015] | separation-style logic for POSIX with `..` and symlinks | abstract, sound (views framework); rm -r bugs in 3 implementations | path footprint vs update footprint ≈ traversal vs written key; effect frame ≈ routing-mutation invalidation | directory hard links excluded; sequential only | verification logic |
| [A-porter-txos-operating-system-transactions-2009] | transactional Linux (150/303 syscalls) | real kernel; conflicts per object | object conflict ≈ DISJOINT by identity; parent-directory reads ≈ traversal; split payloads ≈ cells; list entry vs iteration ≈ catalog given whole | mount, setxattr, sockets, swapon unsupported | implementation |
| [A-groot-dns-verification-2020] | formal DNS resolution semantics and verifier | exhaustive for DNS; production battle-tested | set-valued resolution; CNAME/DNAME aliasing; "local not global" ≈ committee law | caches, failures, reachability; other orgs' zones | verification |
| [B-mancoosi-d21-system-configuration-metamodel-2009] | FP7 metamodel for system configuration and maintainer scripts | broad (packages, services, settings, fs, modules, libs, processes); empirical clustering of the Debian corpus | sorts list; installation-specific inter-package config dependencies ≈ may-read across sorts | user edits between transactions; skeptical of static analysis | model / schema |
| [B-mancoosi-d32-maintainer-script-dsl-2009] | DSL templates with ATL semantics per packaging helper | ~15 helper subsystems (alternatives, init, users/groups, menu, mime, …) | per-verb effect on name-keyed registries | name identity only; precondition "service exists" | mutation modelling |

## Breadth against the 47 items (311r)

- CoLiS: 5–8 (existence and type only; no modes, xattrs, or link counts). The 36 alternatives link is
  expressible only as an opaque symlink leaf.
- Rehearsal and Tortoise: 6 (content), 5 (Tortoise mode and owner), and 14 as file sets or a marker
  path. Rehearsal also compiles 17–19 (users/groups), 28 (cron), 39 (authorized_keys), and host entries
  into file operations. Services (1–4) are not modelled.
- Citac (by snapshot, not by model): 1, 5, 6, 14, 17–19, 21, 23, 28, plus listening ports and
  processes. There is no identity beyond the snapshot key.
- FSMove: 6, 7 (resolution only), and 1 as "service consumes file".
- SibylFS: 5–7 in full, including hard links. No 23, since mounts are not modelled.
- Ntzik–Gardner: 6 and 7, with symlinks.
- TxOS: 5–7 and 18 (credentials) as conflict objects; 23 and 8 unsupported.
- GRoot: 27, deeply.
- Mancoosi: 1–2 (init), 14, 17–19, 36 (alternatives), 46–47 not, modules and libraries as extra
  sorts.
- Nothing in this lane touches 11–13 (sysctls), 24–26 (containers, k8s), 29/31/41 (firewall), 42–45
  (SQL, pip, INI/JSON keys), or 47 (LVM). ~SUSPECT, from the reads above.

## Citations

> [B-colis-unix-utilities-specification-2019]:L275-283 (relevance: +1:SURE)
> Our notion of equality on trees is structural equality, i.e. t1 = t2 iff … both t1 and t2 are the same kind of leaf, … or t1 = symlink(p1), t2 = symlink(p2), and p1 = p2

> [B-colis-unix-utilities-specification-2019]:L1523-1528 (relevance: +1:SURE)
> These are all POSIX or GNU test operators testing certain file permissions, non-zero size, or ownership attributes of files. Since we decided not to include file attributes in our model we obtain on our level of abstraction a non-deterministic semantics.

> [B-colis-unix-utilities-specification-2019]:L1177-1184 (relevance: -0:SUSPECT)
> bancestor is true iff qo is an ancestor of qn. … This test is an under-approximation of initial condition because normalize does not expand symbolic links and therefore the paths computed may not be fully normalized. A precise test for bancestor shall use a reachability predicate in the logic.

> [A-colis-platform-maintainer-scripts-2022]:L903-908 (relevance: +1:SURE)
> Notice a significant limitation of this constraint-based representation of path resolution: the possible presence of symbolic links is simply ignored. Supporting the potential presence of symbolic links is a challenge since there is no way to express finitely, at least in our feature tree logic, the infinite variety of symbolic links that may occur. Yet, this limitation was not noticeable in practice in the experiments reported in the next section.

> [A-colis-platform-maintainer-scripts-2022]:L1073-1081 (relevance: -0:SUSPECT)
> the command cp with recursive option and overlapping source and destination paths may produce a (potentially partial) interleaving of the two input file systems, … which our logic can simply not express. In that case, our specification over-approximates the behaviour of the command

> [A-colis-platform-maintainer-scripts-2022]:L1127-1131 (relevance: -0:SUSPECT)
> We did not find specification errors, but we found errors in the implementation in OCaml of the UNIX commands' specifications. These bugs concern the handling of input paths containing special file names like here ('.') and parent ('..'), or sequences of slashes (`/').

> [A-colis-debian-installation-scenarios-2020]:L163-168 (relevance: +1:SURE)
> A Unix file system implementation contains many features that are difficult to model, e.g., ownership, permissions, timestamps, symbolic links, and multiple hard links to regular files.

> [A-colis-debian-installation-scenarios-2020]:L953-961 (relevance: -0:SUSPECT)
> 4 symbolic execution [33,37,35] try to remove a directory with rm / 3 formalisation [32] bug in dpkg-maintscript-helper / 151

> [A-rehearsal-puppet-determinacy-2016]:L318-321 (relevance: +1:SURE)
> We model filesystems (σ) as maps from paths (p) to file contents. A file may be a regular file with some content (File(str)) or the value Dir that represents a directory.

> [A-rehearsal-puppet-determinacy-2016]:L895-900 (relevance: +1:SURE)
> Liquid Effects proves determinism for multi-threaded C programs with pointers, aliasing, and functions … In contrast, Puppet manifests have no aliasing, loops, or procedures.

> [A-rehearsal-puppet-determinacy-2016]:L853-856 (relevance: -0:SUSPECT)
> Rehearsal uses a straightforward model of the filesystem, partly because Puppet's model hides many platform-specific filesystem details for portability (e.g., Puppet doesn't support hard links).

> [A-rehearsal-puppet-determinacy-2016]:L537-546 (relevance: -0:SUSPECT)
> the obvious approach, based on calculating read- and write-sets is not effective because many resources may create overlapping directories (e.g., /usr and /etc). We observe that this is a form of false sharing and develop a commutativity check that accounts for idempotent directory creation.

> [A-rehearsal-puppet-determinacy-2016]:L878-888 (relevance: -0:SUSPECT)
> to model packages, we need to know the files that a package creates. At present, we assume that packages only create the files returned by apt-file … However, many packages use "post-install scripts" to create additional files, which our approach will miss.

> [B-tortoise-puppet-repair-2017]:L684-695 (relevance: -0:SUSPECT)
> P only models a few key attributes of regular files and directories. If a shell command performs an update beyond the scope of the model, Tortoise will not detect it. For example, if a manifest is configured to start a service and the user terminates the service from the shell, Tortoise will not be able to repair the manifest.

> [B-tortoise-puppet-repair-2017]:L484-490 (relevance: -0:SUSPECT)
> We translate invocations of these programs to constraints that create and delete files in the dpkg:// path. For example, the command apt remove vim produces: assert(file?("dpkg://vim") == false)

> [A-citac-reliable-convergence-2016]:L172-177 (relevance: -0:SUSPECT)
> The configured system is modeled as a possibly infinite set of states S … Our formal approach is agnostic of the particular system on which it is implemented as long as it is possible to recognize equivalent states or differences in states, respectively.

> [A-citac-reliable-convergence-2016]:L361-370 (relevance: +1:SURE)
> Resource b preserves resource a iff for any state s ∈ S satisfying a (s |= a), the state s' after applying b … satisfies a as well … we can say that resource b preserves resource a iff a∘b∘a = a∘b.

> [A-citac-reliable-convergence-2016]:L693-701 (relevance: -0:SUSPECT)
> status of network interfaces · network route configuration · listening server sockets · mounted file systems · running processes

> [A-citac-reliable-convergence-2016]:L909-917 (relevance: +1:SURE)
> we plan to optimize this by incorporating knowledge about the specific resources which are executed. For instance, preservation test cases for file resources with non-overlapping file paths may be omitted.

> [B-hummer-testing-idempotence-iac-2013]:L295-298 (relevance: +1:SURE)
> Our prototype testing framework tracks the following pieces of state: network routes, OS services, open ports, mounted file systems, file contents and permissions, OS users and groups, cron jobs, installed packages, and consumed resources.

> [B-hummer-testing-idempotence-iac-2013]:L359-363 (relevance: -0:SUSPECT)
> In general, domain-specific knowledge is required to define concrete non-conflicting properties. By default, we consider state properties as non-conflicting if they are equal.

> [A-fsmove-puppet-fault-detection-2020]:L299-319 (relevance: +1:SURE)
> in a Unix-like file system, every file is associated with an inode rather than a path. Thus, the file-related OS structures (e.g., file descriptor table) have to refer to inodes instead of path names. Existing approaches … describe files through their paths [20]. The latter makes the corresponding file descriptor table hold the stale entry (3, /usr/lib/perl5), after the rename at line 2.

> [A-fsmove-puppet-fault-detection-2020]:L401-458 (relevance: +1:SURE)
> FSAcc = Path → P(Eff × BlockID) … maps path names to an element of the power set of blocks and effects … a block b1 producing a certain file p must precede a block b2 that consumes or expunges the same file p

> [A-fsmove-puppet-fault-detection-2020]:L131-147 (relevance: -0:SUSPECT)
> 92 previously unknown faults in 33 modules … More than a half of the issues (62 out of 92) were confirmed and fixed by the developers.

> [B-fsmove-fstrace-domains-source-2020]:L289-301 (relevance: +1:SURE)
> let rec to_inode path state = … match find_from_inodetable inode_p b state'.i with | None -> let inode = gen_inode state' in …

> [B-fsmove-fstrace-domains-source-2020]:L400-407 (relevance: -0:SUSPECT)
> let add_effect (lst, cache) (elem, sdesc) = match elem with | Create x -> … add_effect_to_cache cache x Syntax.Produce | Read x | Touch x | Write x -> … Syntax.Consume | Remove x -> … Syntax.Expunge

> [A-sibylfs-posix-oracle-2015]:L517-523 (relevance: +1:SURE)
> The interface to the state model is expressed in terms of references to files and directories (types dh dir ref and dh file ref). The state-model API permits arbitrary linking and unlinking, in particular, our model can handle directory links, and disconnected files and directories can also be modelled

> [A-sibylfs-posix-oracle-2015]:L518-523 (relevance: +1:SURE)
> For API calls involving two paths (such as rename) we consider … equivalence classes based on properties of two paths: whether they are equal or not; whether they are different paths to the same file (hard links); and whether one path is a proper prefix of the other.

> [A-sibylfs-posix-oracle-2015]:L150-159 (relevance: -0:SUSPECT)
> We do not model unusual file types (such as FIFO special files), or asynchronous I/O, signals … We do not currently model the *at forms of functions such as openat

> [A-sibylfs-posix-oracle-2015]:L691-701 (relevance: -0:SUSPECT)
> POSIX also mandates a strong invariant: a libc call which returns with an error should leave the underlying file system state unchanged. … as well as returning the ENOTDIR error, FreeBSD deletes the symlink and replaces it with a newly created file. This breaks the POSIX invariant.

> [A-ntzik-gardner-posix-fusion-logic-2015]:L41-53 (relevance: +1:SURE)
> POSIX pathnames use '..' to traverse up the directory structure and symbolic links to jump between directories. They thus cut across the underlying inductive definition of the directory tree and overlap with the directory subtree(s) being updated. … a directory in lib which is different (disjoint) from tex can be identified by a path using it, for example /usr/lib/tex/../latex. Removing tex, will invalidate this path as well; the local update has a global effect.

> [A-ntzik-gardner-posix-fusion-logic-2015]:L316-358 (relevance: -0:SUSPECT)
> The file system structure is a directed acyclic graph consisting of a directory tree and files. Files are uniquely identified by inodes … As most implementations we only allow hard links to files. Directory hard links introduce cycles which are not detectable during directory traversal.

> [A-ntzik-gardner-posix-fusion-logic-2015]:L905-917 (relevance: -0:SUSPECT)
> $> mkdir -p /tmp/a/b/c · $> mkdir -p /tmp/a/e · $> rm -r /tmp/a/b/../.. — which should remove the directory /tmp/a and its contents. Both implementations actually result in the directory not being removed but becoming empty instead.

> [A-porter-txos-operating-system-transactions-2009]:L440-442 (relevance: +1:SURE)
> For two concurrent transactions to successfully commit in TxOS, they must write disjoint objects.

> [A-porter-txos-operating-system-transactions-2009]:L474-476 (relevance: +1:SURE)
> Many kernel objects are only read in a transaction, such as the parent directories in a path lookup.

> [A-porter-txos-operating-system-transactions-2009]:L526-531 (relevance: -0:SUSPECT)
> write — Any number of insertions and deletions are allowed, provided they do not access the same entries. Reads (iterations) are not allowed.

> [A-porter-txos-operating-system-transactions-2009]:L464-470 (relevance: -0:SUSPECT)
> the inode_header contains both file metadata (owner, permissions, etc.) and the mapping of file blocks to cached pages in memory (i_data). … TxOS versions these objects separately, allowing metadata operations and data operations on the same file to execute concurrently

> [A-groot-dns-verification-2020]:L848-857 (relevance: +1:SURE)
> GRoot does not model dynamic phenomena that affect DNS results such as caching, server failures, and network unreachability. … Because GRoot can only analyze the zone files that it is given, it can only verify the correctness of the DNS configuration of the organization that owns those files. The end-to-end correctness of the DNS configuration (globally) hinges on other organizations doing the same.

> [A-groot-dns-verification-2020]:L28-32 (relevance: -0:SUSPECT)
> Applied to the configuration files from a campus network with over a hundred thousand records, GRoot revealed 109 bugs within seconds. When applied to internal zone files consisting of over 3.5 million records … GRoot revealed around 160k issues of blackholing

> [B-mancoosi-d21-system-configuration-metamodel-2009]:L2693-2699 (relevance: +1:SURE)
> class Configuration { reference installedPackages … reference runningServices … reference packageSettings … reference devices … reference fileSystem … reference environment … }

> [B-mancoosi-d21-system-configuration-metamodel-2009]:L1278-1290 (relevance: -0:SUSPECT)
> 9061 scripts … (we identified 116 templates) … The result of this step is the identification of 10 classes.

> [B-mancoosi-d21-system-configuration-metamodel-2009]:L248-268 (relevance: -0:SUSPECT)
> Both works are hence far even from the minimal requirement of determining a priori the set of files touched by script execution … we are skeptical that static analysis can fully solve the problem … File modifications performed by users between transactions are not supported by model-based rollback, since they cannot be captured in the model.

> [B-mancoosi-d32-maintainer-script-dsl-2009]:L1370-1373 (relevance: -0:SUSPECT)
> The aim of the simulation is to identify possible inconsistencies … For instance, the execution of a statement on a service which does not exist in the current configuration can stop the simulation.

> [B-mancoosi-d32-maintainer-script-dsl-2009]:L3172-3175 (relevance: -1:GUESS)
> add alternative(name, location) … If name already exists in the source configuration, a new reference from the existing name alternative to the location file is created

## Where these coincide with or contradict 311

- Contradicts 311 (311u refuted shape, "unequal names ⇒ different things"):
  - CoLiS's tree model makes unequal paths disjoint by construction [B-colis-unix-utilities-specification-2019].
  - FSMove does the same trace-locally [B-fsmove-fstrace-domains-source-2020].
  - Citac proposes it as an optimisation [A-citac-reliable-convergence-2016].
  - Rehearsal declares that no aliasing exists [A-rehearsal-puppet-determinacy-2016].

  In 311's terms, each assumes `:guarantees-unique-name` plus `:aliases-nothing-else` everywhere.
  +SURE on the assumption; ~SUSPECT on the mapping.
- Coincides with 311 §2.9 and §3.3 (routing mutation invalidates any resolution whose traversal it
  touches): Ntzik–Gardner's effect frame [A-ntzik-gardner-posix-fusion-logic-2015], and TxOS's
  parent-directory read sets [A-porter-txos-operating-system-transactions-2009]. ~SUSPECT
- Coincides with 311 §1.9 (cells) and §2.9 (a catalog given whole versus its members): TxOS's split
  inode payloads and its list-conflict states [A-porter-txos-operating-system-transactions-2009]. -GUESS on how far the
  analogy carries.
- Coincides with 311 §3.5 (committee law: each author speaks only about their own thing): GRoot's
  "local not global correctness" [A-groot-dns-verification-2020]. Hummer's per-domain
  "non-conflicting" knowledge is left to the tester [B-hummer-testing-idempotence-iac-2013]. ~SUSPECT
- A caution for any fact that treats an erroring call as a no-op: SibylFS found FreeBSD violating
  POSIX's "an error leaves state unchanged" [A-sibylfs-posix-oracle-2015]. +SURE on the finding.

## Leads not pulled

- Mancoosi SCP 2011 "Supporting software evolution in component-based FOSS systems" (Di Cosmo et al.)
  · the peer-reviewed condensation of the metamodel · budget; the deliverables were read instead ·
  doi 10.1016/j.scico.2010.11.001, cited by CoLiS TACAS.
- CoLiS "Revision 2 of CoLiS language" tech report (hal-02321743) and Jeannerod–Treinen IJCAR 2018
  (feature trees with updates, hal-01807474) · the logic behind the spec corpus · the spec corpus
  itself was prioritised · HAL.
- Ntzik et al. ECOOP 2018 "A concurrent specification of POSIX file systems" · adds concurrency to
  fusion logic · budget · LIPIcs.ECOOP.2018.4.
- Ntzik–Gardner tech report · has the full symlink rules · budget · linked from Ntzik2015Reasoning.html.
- Shseer (Narsipur, Brown MS report) and Lazarek et al. HotOS 2025 "From Ahead-of- to Just-in-Time and
  Back Again: Static Analysis for Unix Shell Programs" · a symbolic-tree fs model with per-command
  specs, from the Vasilakis group · the report is unrefereed, and a grep found no symlink or hard-link
  handling (-GUESS that it has none), so it was not registered · the report is on cs.brown.edu, the
  paper at nikos.vasilak.is/p/sash:hotos:2025.pdf.
- BuildFS (Sotiropoulos et al. OOPSLA 2020) · FSMove's model generalised to Make/Gradle · same
  identity choices expected (-GUESS) · github.com/theosotr/buildfs.
- PaSh annotations (binpash/annotations) · per-command I/O specs for about 60 commands · they classify
  arguments as input/output files for dataflow and say nothing about identity; the README was read,
  not registered · github.
- binnacle (Henkel et al. ICSE 2020), Acto (SOSP 2023), Sieve (OSDI 2022), SCALE/Ferret (NSDI 2022),
  Brown & Patterson "Undo for operators" (USENIX 2003) · all in brief item 6 · not reached within
  budget · arXiv 2002.03064 for binnacle; the rest via Kagi.
- Lepiller et al. TACAS 2021 "Analyzing Infrastructure as Code to Prevent Intra-update Sniping
  Vulnerabilities" · surfaced in the first Rehearsal search · not pulled · PMC7984555.

## Search log

- kagi · CoLiS spec / TACAS 2020 / Jeannerod symbolic execution (3 queries) · kept 3
- kagi · Rehearsal / Tortoise / Hanappi OOPSLA / Hummer Middleware (4 queries) · kept 4
- kagi · FSMove ICSE / BuildFS / Ansible trace faults / SibylFS (4 queries) · kept 3 (+ FSMove source via `gh api`)
- kagi · Ntzik–Gardner / formal model of config effects / IaC semantics Ansible (3 queries) · kept 1
- kagi · GRoot / PaSh annotations / binnacle / TxOS (4 queries) · kept 2
- exa · "research paper that formally models the effects of system administration commands…" · kept 2 (Mancoosi D2.1, D3.2; Shseer as a lead)
- gh · `repos/colis-anr/colis-language` listing; `repos/AUEB-BALab/fsmove` interpreter/domains; `repos/binpash/annotations` README

## Tooling problems

- The vendored `Research/Vendor/colis-anr/colis-batch` directory is empty in this worktree, and no
  `colis-language` copy is vendored. I read the published HAL artifacts only, which is what the brief
  asks for when grading.
- The two briefs disagree on ownership of the CoLiS utility-specification corpus. The shared brief
  says "owned by lane 4"; this lane's brief says "is yours". I registered it here as
  [B-colis-unix-utilities-specification-2019]. The conductor should dedupe if lane 4 registered it
  too.
- Register-lock timeouts. The wrapper returned `lock timeout after 600s` for
  [A-rehearsal-puppet-determinacy-2016] and [B-mancoosi-d21-system-configuration-metamodel-2009].
  As the brief instructs, I then stopped registering, and killed my queued batch while it was still
  waiting and did not hold the lock. That leaves five sources fully read and graded but not in
  `sources.json`: [A-rehearsal-puppet-determinacy-2016], [B-tortoise-puppet-repair-2017],
  [A-citac-reliable-convergence-2016], [B-hummer-testing-idempotence-iac-2013],
  [B-mancoosi-d21-system-configuration-metamodel-2009]. `validate.sh` will flag these five slugs until
  they are registered. Their complete entry JSONs are ready in the scratchpad as `entry-<slug>.json`.
  To finish, run `sh $SC/register.sh $RD <slug> < $SC/entry-<slug>.json` for each, once the lock is
  quiet. My reading copies (`pdftotext -layout`) are in `$SC/l5a/`.
- Duplicate registration: another lane registered the same TxOS paper as
  [A-porter-txos-operating-system-transactions-2009] while my entry was queued. My pre-registration
  URL grep found nothing at the time. My [A-porter-txos-operating-system-transactions-2009] stays in the manifest,
  graded B on my partial read. The conductor should pick one slug, and I did not touch the manifest.
- `mancoosi-d21` and `d32` render their metamodel figures as images. The metaclass lists come from the
  KM3 listing and the prose only.
