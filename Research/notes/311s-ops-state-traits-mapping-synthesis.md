# 311s — Ops-state traits mapping: the synthesis (clean-context desk study)

Sections B–E of the clean-context desk study whose per-item worksheet is `notes/311r`. Written by a Fable subagent with no Dorc context, 2026-09-17; grades are the author's. Nothing here is ruled.

## B. Clusters by behaviour (Q2–Q11, ignoring Q1)

Items with two halves (file side / kernel side) appear in two clusters; that is a finding, not sloppiness.

**B1. Kernel scalar slots loaded from a file at boot.** Value in a kernel table (per-netns or global), no sub-parts, no identity beyond the key, always present, persisted form is a `.d`-layered file replayed by a boot-time loader, live writes are last-wins with no lock, persisted↔live disagree after either side is written alone, aliases are spellings of one path (dots/slashes/procfs). Members: 11, 13, 34, 41 (policy slot), 30 (kernel side).
- **B1′. Kernel slot keyed by a foreign name.** As B1 but the slot's name contains another object's name, it is created/destroyed *with* that object, rename re-keys it, and a write to a sibling key (`all.*`) silently overwrites it. Members: 12; 22's rename behaviour drags 12 and every interface-named config with it.

**B2. Inode attributes.** Persisted-only, no live copy; identity is `(st_dev, st_ino)`, which is *reused*; unlimited path aliases including hard links and `/proc/*/fd`; each attribute written atomically and whole; only side-effect is ctime; write-rename by editors replaces the inode and silently drops or alters the attribute; decollision only by `dpkg-statoverride`. Members: 5, 7, 8. (6, the bytes, is *not* here: it has a live consumer copy.)

**B3. Daemon-loaded configuration text.** Persisted file (often a layered search chain plus drop-ins/includes) with exactly one live form: the parsed copy inside one process, moved by an explicit reload/restart or by the process's own watch; the live side is usually *unreadable* (no tool prints the daemon's parsed state); sub-structure is content the filesystem cannot address; identity is the inode plus, sometimes, a package fingerprint; decollision is file-per-writer via include dirs and marker comments. Members: 3, 6, 33 (file side), 44, 45; the file halves of 29, 31, 32, 40; the unit-file half of 2.

**B4a. Package/admin registries with a two-party protocol.** Persisted-only registry (status file, admin file, symlink farm), name-addressed with no identity, written by *both* package scripts and admins, decolliding by an explicit mode word (auto/manual, hold, helper-remembers-enablement) and, for dpkg, a real lock; read per invocation, never cached. Members: 2, 14, 15, 16, 36; 43 is this cluster crossed with B6 (registry per interpreter, three-level intent/installed/imported).

**B4b. Per-user text lists read per use.** One file per user or per purpose, entries identified by content (or position), read by the consumer at each use (per auth, per invocation, per mtime scan), effectiveness gated by *sibling* state (permissions, labels, allow-files, env), appended to by many tools that coexist by "add if not present" idioms and marker comments; no lock; no live copy beyond a minute-scale scan. Members: 9, 28, 39; 40's file half.

**B5. Account records with login-time snapshots.** Persisted records reachable through NSS (several backends, first match wins, caches), identified by a *numeric id that is reused*, written under the shadow-suite lock, but *copied into every process at login* (credentials, `$SHELL`, supplementary groups) so live effect lags until re-login; "locked" spread across several unrelated stores. Members: 17, 18, 19, 20.

**B6. Process-private environment.** Live-only, per process, inherited by copy at exec and never written back; no cross-process clobber, only divergence; persisted form is a matrix of profile files applied *once* per login path; `/proc/pid/environ` is a stale snapshot; consumers keep derived caches (bash hash). Members: 37, 38, 10 (derived from B6 plus B4b).

**B7. Manager-owned live network/host objects.** Kernel or file state that a daemon (NM, networkd, resolved, cloud-init, hostnamed, udisks) *owns*, re-asserts from its own profile, and may revert manual writes on its next event; netns-scoped; identity by ifindex at best; DHCP-derived values have no local persisted intent at all; the file, where there is one, is a *rendering*. Members: 21, 22, 40 (rendering side), 30 (transient side), 12 (networkd side), 46 (automount side).

**B8. Declared objects with activation instances.** A name is *declared* in a persisted store (unit file + enablement, container hostconfig + restart policy, VG metadata, fstab, etcd spec) and *activated* into live kernel/daemon state by a controller or boot; declared-but-inactive and active-but-undeclared are both legal by design; every activation mints an instance id (InvocationID, container Pid/StartedAt, `dm-N`, mount id, pod uid) that is the only honest staleness key for measurements of the live half. Members: 1+4, 24 (+25), 47, 32, 23+46, 26.

**B9. Positional rule lists rendered into the kernel.** A persisted text ruleset loaded whole into a kernel table by a loader; entries identified by position or handle, renumbered on edit; several frameworks share the kernel by chain/table *naming convention*, a flush by one destroys all; live→persisted is a routine, tool-supported direction (`*-save`, `nft list`); which of the two forms the seed's read reports depends on the tool (ufw: file; nft/iptables: kernel). Members: 29, 31, 41.

**B10. Remote authority with local caches.** The value's home is off-machine; the machine holds caches that age by TTL or replication lag; answers depend on which server/source/session you ask; reboot and re-provision of this machine are irrelevant to the value; the substrate natively distinguishes unset from absent even where the seed's read discards it. Members: 27, 26, 42a (remote DB), 33 (CA/CT/revocation as a third form).

**B11. Transactional substrates with versioned identity.** Designed concurrency control (transactions/MVCC, `resourceVersion` + field managers, VG lock + seqno, migration version tables), ids that are never reused and distinct from names (`xmin`/`attnum`, `metadata.uid`, `lv_uuid`), deep hierarchical addressing, and a three-level intent → declared → observed pipeline. Members: 42a, 42b, 26, 47; 14 by the lock only.

**B12. Settings encoded as symlink targets.** The value is *inferred from where a link points*; a copied (non-link) file loses the name; a second persisted spelling exists and can disagree (`/etc/timezone`, the admin file, the helper's record); writes are atomic link replacement. Members: 35, 36, 2, 40 (the link itself).

**B13. Derived and sampled reads.** Not stores. Either resolved from a chain of contributors, so a disagreement with any contributor is the *signal* of an override (10; also `hostname -f`, `is-active` as a projection of 4), or a periodic *measurement* kept by a daemon that changes with zero writes (25). Members: 10, 25.

**Which questions did the separating.** Q4 (where it lives, what it survives) and Q11 (how many forms, what moves them, whether the live side is readable) produced almost the whole partition. Q10 (identity, and whether the substrate *reuses* it) split B8/B11 from everything else and split B2/B5 (reused ids) from B8/B11 (never reused). Q5 split by *scope of observer* (namespace / user / process / session / resolver) and separated B5, B6, B7 from B1. Q6 contributed two features only: "slot keyed by a foreign name" (B1′) and "positional" (B9, and arrays in 45). Q9 separated B4a and B11 (designed protocols) from the last-write-wins majority.

**Which did not matter.** Q2: nearly every item has aliases; the *kind* of alias (path resolution, name normalization, semantic sibling in another store) tracks Q4 and added no partition of its own. Q3 collapsed into Q10's reused/never-reused axis. Q8 was fully implied by Q4 + Q6. Q7 did not separate clusters; it flagged a handful of *item-level* hazards that cut across clusters: silent cross-key propagation (12/13), silent clobber of others' additions (19's `usermod -G`, 31's `flush ruleset`, 40's rewrites), re-keying by rename (22, 18, 47), and a read with a side effect (41).

## C. The Q1 cross-tab

- B1: 11, 13, 34, 30-kernel, 41 are *never absent, never unset* at the live layer (their persisted line may be absent → default). 41 adds "absent in the kernel until the read creates it". **Q1 splits B1′ off cleanly**: 12 is *absent* whenever its interface is.
- B2: 5 always present; 7 present/absent per link; 8 *absent* with no default plus a third state, "unsupported". **Q1 splits B2** (mode bits never unset vs. xattr absent).
- B3: uniformly *absent at the file layer, unset-with-default at the consumer layer* (3, 44, 45); 6 adds present-but-empty; 33 adds present-but-unparseable and has no unset at all. **Q1 joins B3.**
- B4a: heterogeneous. 2 has both kinds (`static` = unset; missing file = absent); 14's `config-files` is a *named* not-installed state; 15 retains a stale version in that state; 16 is unset-with-explicit-default; 36 is absent-or-auto-fallback. **Q1 does not describe B4a**: the registries chose different renderings for "known but not in effect".
- B4b: the *line* is always absent; the *file* missing yields a default (40: local host; 39: fallback files; 28: "no crontab"). **Q1 joins B4b.**
- B5: 18 is pure absent; 17, 19, 20 encode "unset" as a *value* (empty shell / `nologin`, empty list, `NP`). **Q1 splits B5** along "id" vs "attribute".
- B6: 37, 38 distinguish absent from empty; 10 is never missing. **Q1 splits 10 off**, which is why it sits in B13 too.
- B7: 21 present-but-empty set; 22 absent; 40 always defaulted. Mixed.
- B8: the messiest row and the most important. 1 reports `inactive` for a unit that does not exist (conflates not-declared with not-active); 24 separates them ("No such object" vs `exited`); 47 separates (rc 5 vs blank); 4 is empty-when-inactive; 25 prints `<no value>`; 32 *conflates* (`noauto` and "no fstab line" both print nothing); 23 spells its default out (`relatime`); 26 materializes the default on admission. **Q1 fractures B8**: the cluster behaves alike but its tools *report absence* in eight different ways, and two of them (1, 32) cannot tell "declared but inactive" from "not declared" with the seed's read.
- B9: 29 has a hidden-but-present state when ufw is inactive; 31 is plain absent; 41 is never absent. **Q1 splits B9.**
- B10: 27 distinguishes NODATA from NXDOMAIN but `+short` discards it; 26 defaults; 42a has NULL vs no-row. The *substrates* join (all three model both); the *reads* do not.
- B11: NULL/no-row, default, blank/rc 5. Joins at the substrate level.
- B12: 35 absent→UTC; 36 absent group; 2 `static`; 40 defaulted. Mixed.
- B13: never absent (10), `<no value>` (25).

**Where Q1 joins clusters that behave differently.** "Unset-with-default" appears in B1 (persisted line), B3 (consumer default), B4b (resolv.conf), B5 (shell), B10 (k8s admission), B13 (git var): six clusters with unrelated Q4–Q11 behaviour. Q1 is a property of *how a particular read renders missingness*, not of the state; grouping by it would mislead. The one place Q1 carries real information is B8, where it exposes which reads conflate "not declared" with "not active".

## D. Hard, ambiguous, read-dependent, and least-sure items

Read- or tool-dependent answers (both given in A): 1 (`is-active` vs `show -p LoadState`), 6 (file vs `nginx -T`), 9/10 (file scope vs resolved chain), 23 (`findmnt` vs `-T`; VFS vs FS `ro`), 27 (`+short` vs full; `dig` vs `getent`), 29 (`status` vs `show added` vs `iptables -S`), 30 (`hostname` vs `hostnamectl` vs `-f`), 35 (`timedatectl` vs `/etc/timezone` vs `TZ`), 41 (legacy vs nft binary select different kernel objects), 42a (which session/transaction/replica), 43 (`pip show` vs `import`), 46 (`MOUNTPOINT` vs `MOUNTPOINTS`), 47 (`lvs` vs `dmsetup`).

Claims I would want checked before anyone builds on them:
- 1: `is-active` printing `inactive` for a nonexistent unit (~SUSPECT).
- 5: root `chown` clearing setuid/setgid (~SUSPECT).
- 8: tmpfs `user.*` xattr support version; whether `sed -i` copies xattrs (~SUSPECT both).
- 11: `pid_max` per pid-namespace in 6.14 (~SUSPECT).
- 12/13: `forwarding` propagation is +SURE; which *other* `conf.*` keys propagate vs combine is ~SUSPECT.
- 16: dpkg/apt behaviour on explicit install of a held package; holds surviving remove-not-purge (~SUSPECT).
- 20: whether sshd honours the `!` lock (distro/PAM dependent).
- 21: NetworkManager's treatment of externally added addresses (~SUSPECT).
- 23: util-linux `remount` option-merge rules (~SUSPECT).
- 25: whether health state is flushed to the container's on-disk state (~SUSPECT).
- 29: what `ufw status` checks to decide active; whether ufw tags kernel rules (~SUSPECT).
- 31: nft handle reuse; listing canonicalization (~SUSPECT).
- 32: gpt-auto skipping fstab-listed devices (~SUSPECT).
- 34: default when `SELINUX=` is missing; the boolean that locks `setenforce` (--WONDER both).
- 35: whether timedated writes `/etc/timezone`; how eagerly glibc re-reads `/etc/localtime` (~SUSPECT both).
- 36: `--query` output on dangling links; the "no alternatives" rc (~SUSPECT).
- 37: duplicate env keys and `getenv` (~SUSPECT).

Least sure overall, ranked: 34, 16, 36, 29, 25, 23.

## E. Mis-posed items, and what the seed is missing

Mis-posed or under-posed:
- 10 is a derived view, not a store; valuable as the *category* "resolved from a chain", not as an item.
- 13 is one word with two names; the interesting behaviour is its propagation into 12, which the pairing hides.
- 25 is a sampled measurement; calling it state invites treating "healthy" as something a line can write.
- 29 names a *position*; the tuple is the thing. Same for arrays in 45 and entries in 38.
- 33's two fields cannot vary independently; the pair adds only the identity discussion.
- 46 "the mount point" is a set; the seed's read discards members.
- 32's "corresponding fstab line" is not computable from `swapon --show` without `blkid` (device-name aliases; `mkswap` rewrites the UUID).
- 44/45 are unanswerable generically: reload semantics, merge chains, and unset-vs-absent are all *per program*; the item needs the program named.
- 1 and 32, per section C, cannot distinguish not-declared from not-active with the read given.

Common ops state absent from the seed that behaves unlike everything in it:
- A *listening port* (`ss -ltnp`): an exclusive resource — the constraint is *who holds it*, not what value it has; nothing in the seed is mutually exclusive.
- *Disk usage or a quota* (`df`, `quota`): a metered quantity consumed by everything, not name-addressed, whose measurement decays continuously; no quantity appears in the seed.
- A *loaded kernel module* (`lsmod`, `modules-load.d`, `modprobe.d` blacklist): refcounted existence (cannot unload while in use), aliases through `modules.alias`, parameters writable live in `/sys/module/*/parameters`.
- A *git checkout* (`git rev-parse HEAD` in `/opt/app`, plus dirtiness): content-addressed identity with a working tree that is live-vs-persisted *inside one directory*, and a remote authority; the seed has content addressing only as a cert fingerprint.
- An *image tag* (`web:latest` → digest): a mutable name for immutable content, expected to drift, remote registry authority.
- A *secret* (private key, token): the analyzer must never read the value; only existence, hash, or fingerprint; no "forbidden read" exists in the seed.
- The *kernel command line* (`/etc/default/grub` → `update-grub` → `grub.cfg` → `/proc/cmdline`): a generated intermediate file that people edit by mistake, and a live form read-only until reboot.
- A *ZFS dataset property* (`zfs get` with its `source` column: local/inherited/default/received): the read reports *provenance*; only `git config --show-origin` and `sestatus` do that in the seed.
- *apt lists* (`apt update` into `/var/lib/apt/lists`): a cache that must be refreshed before other state (14/15) can be written; no gating cache is in the seed.
- A *firewalld* zone service (`--permanent` vs runtime, `--runtime-to-permanent`): the only tool that names the two-form seam in its CLI.
- An */etc/hosts* entry: NSS-only override of 27 and 40, cloud-init managed (`manage_etc_hosts`), consumed by `getent` and never by `dig`.
- A *tmpfiles.d*-managed path with an age (`d /tmp 1777 root root 10d`): state with a garbage-collection schedule.
- *Clock synchronization* (`NTPSynchronized`, offset): continuously varying, no write path, and the validity of 33's judgement and every timestamp comparison depends on it.

