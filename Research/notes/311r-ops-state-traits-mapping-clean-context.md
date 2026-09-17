# 311r — Ops-state traits mapping: the per-item worksheet (clean-context desk study)

Method: written from prior knowledge of the tools only; no repository documents read, no web lookups, no state-changing commands. Every claim carries +SURE / ~SUSPECT / -GUESS / --WONDER. Where the same item has two reads (file vs. live), both are answered. "Thing" below means what the read reports on, per the brief.

Legend for Q1: "unset" = the tool has a slot for it and can say it is missing (often with a default); "absent" = the tool reports no such thing, nothing enumerates the gap. Several items are *both*, at different layers; those are called out.

The synthesis (sections B–E) is `notes/311s`; this file is the worksheet it rests on.

## A. Per-item answers

### 1. `systemctl is-active nginx.service` — the unit's ActiveState
- Q1: not deletable while the unit persists; it is a property of the loaded unit. Unit missing: `is-active` still prints `inactive` (rc 3) (~SUSPECT it says `inactive`, not `unknown`), while `systemctl show -p LoadState` says `not-found` (+SURE) → *unset-with-misleading-default* from one read, *absent* from the other. Unit file deleted while running: unit stays loaded and active until stopped (+SURE), LoadState becomes `not-found` after daemon-reload (~SUSPECT).
- Q2: yes. `nginx` (suffix defaulted, +SURE), any `Alias=` name (symlink in the unit search path, +SURE), `systemctl show -p ActiveState`, D-Bus `org.freedesktop.systemd1.Unit.ActiveState`, `service nginx status` (sysv shim), the cgroup dir `/sys/fs/cgroup/system.slice/nginx.service` (existence ≈ active, +SURE cgroup is removed on stop).
- Q3: the slot persists; the *activation* it describes is reissued on every start (see item 4). The unit definition behind the name can also be replaced wholesale (new file, same name).
- Q4: PID 1 memory only (+SURE). Survives `daemon-reexec` by serialization (+SURE), not reboot (rebuilt from enablement), a process restart of nginx is a new activation; re-provision rebuilds. `--user` units live in a per-user manager, a different store (+SURE).
- Q5: which manager you ask (`--user` vs system; `--machine`; `--root`/`SYSTEMD_OFFLINE`, +SURE); PID/mount namespace decides whether a systemd is reachable at all. Not user (reads are unprivileged), not cwd, not env beyond bus-address vars.
- Q6: sub-parts read-only: ActiveState, SubState, Result, timestamps, MainPID. Not separately writable. Name is flat `name.type` with template instances `name@inst.type`; not a path.
- Q7: writing (start/stop): spawns/kills processes, creates/removes the cgroup, pulls in Wants/Requires/Conflicts/PartOf/BindsTo peers (+SURE), triggers OnFailure/OnSuccess units, journal lines. Changed by others: main process exit (Restart= policy), OOM kill, watchdog, socket/path/timer activation, `Conflicts=` of another unit starting, package maintainer scripts (`deb-systemd-invoke`, +SURE), `BindsTo=` device disappearing.
- Q8: through the unit. Same running thing reachable via `ps`/`pgrep`, cgroup fs, `service`, D-Bus, `systemctl status <PID>` (+SURE PID lookup works).
- Q9: yes: admin, package scripts, cron/Ansible, other units' dependency edges. systemd serializes conflicting jobs in its queue (later job replaces or fails per `--job-mode`, +SURE); no locking beyond that; no convention.
- Q10: the unit has none beyond its canonical name (`Id=`, `Names=`); the *activation* has InvocationID (item 4); the process tree has MainPID and a cgroup path.
- Q11: persisted = enablement symlinks + unit file (what *will* be active at boot); live = ActiveState. Boot transaction moves persisted→live; `enable --now` writes both. They disagree freely (enabled-but-stopped, disabled-but-running), by design (+SURE).

### 2. `systemctl is-enabled nginx.service` — UnitFileState
- Q1: yes: `enable`/`disable` create/remove symlinks under `/etc/systemd/system/*.wants/` (+SURE); `mask` creates a `/dev/null` symlink (+SURE). Missing unit file: `is-enabled` errors "No such file or directory" rc 1 → *absent*. Unit present but without `[Install]`: prints `static` → *unset-with-default* (a slot exists, nothing to enable). Both kinds of missing exist for one read.
- Q2: yes. Alias names; `systemctl show -p UnitFileState`; the symlink path itself (`/etc/systemd/system/multi-user.target.wants/nginx.service`, +SURE); `systemctl list-unit-files`; Debian's `deb-systemd-helper is-enabled` (keeps its *own* record in `/var/lib/systemd/deb-systemd-helper-enabled/`, +SURE — a second store that can disagree). Runtime enablement (`--runtime`, `/run/systemd/system/*.wants/`) is a distinct slot reported as `enabled-runtime` (+SURE).
- Q3: disable→enable recreates a symlink at the same path: same thing. The link target can change (`linked` state when the unit file lives outside the search path, +SURE).
- Q4: on disk as symlinks (+SURE); `is-enabled` computes from the filesystem (works with `--root`, +SURE). Survives reboot and systemd restarts; `--runtime` variant does not survive reboot. Re-provision: only if re-enabled (presets do this on package install, +SURE).
- Q5: `--user` reads `~/.config/systemd/user/` (+SURE) → user-dependent; `--global` is a third tree; chroot/mount ns changes the tree; `SYSTEMD_UNIT_PATH` alters search path (~SUSPECT). Not cwd.
- Q6: sub-parts: one symlink per `WantedBy=`/`RequiredBy=` target, plus `Alias=` links, plus `Also=` units (+SURE), each a separate file you could `ln -s` by hand. Addressed by unit name; physically path-like.
- Q7: writing: symlinks change; implicit `daemon-reload` (+SURE); nothing starts or stops unless `--now`; next boot's transaction changes. `mask` additionally makes `start` fail (+SURE). Changed by others: package postinst via `deb-systemd-helper`/presets (+SURE), `systemctl preset-all`, `rm` of the unit file (→ absent), editing `[Install]` (no effect until `reenable`, +SURE), `daemon-reload` refreshes the loaded view.
- Q8: through the unit, or through the target (`.wants/` dir), or through the path, or through Debian's helper. Three tools, three views.
- Q9: admin `disable` vs package `enable` on upgrade; Debian's helper records "already enabled once" so upgrades do not re-enable an admin-disabled unit (+SURE that is its purpose). Otherwise atomic symlink ops, last writer wins, no lock.
- Q10: none (symlink inodes exist but nobody uses them); unit `Id`.
- Q11: persisted = symlinks; live = PID 1's loaded `Wants=` edges (visible via `systemctl list-dependencies`), updated only on daemon-reload. A hand-made `ln -s` without reload makes them disagree (+SURE). `is-enabled` reads disk; `list-dependencies` reads memory: two reads, two forms.

### 3. `Restart=` line of `/etc/systemd/system/foo.service`
- Q1: yes, a line can be added/removed. Missing line: systemd defaults to `no` and `systemctl show -p Restart` prints `no` (+SURE) → *unset* at the loaded layer, *absent* at the file layer (grep finds nothing). The same key may also live in drop-ins `foo.service.d/*.conf` and in `/run` or `/usr/lib` copies of the unit (+SURE layering).
- Q2: yes. Drop-in `override.conf` (`systemctl edit`), lower-priority copies under `/run/systemd/system` and `/usr/lib/systemd/system` (+SURE precedence /etc > /run > /usr/lib), `systemctl show -p Restart` (effective), `systemctl cat` (all layers), D-Bus property. If the unit file is a symlink (`Alias=`), one physical line serves two unit names. `set-property` cannot set it (limited to resource-control properties, +SURE).
- Q3: yes: line removed and re-added; file replaced by write-rename (new inode, same path). Last occurrence wins within the merged layers (+SURE).
- Q4: file on disk (survives reboot); effective value in PID 1 memory after load, until next `daemon-reload`. No owning process other than PID 1's cached parse. Re-provision: only if templated; `/etc/systemd/system` is admin-owned by convention, packages ship to `/usr/lib` (+SURE).
- Q5: mount namespace/chroot; `--user` tree is a different file. Reading the file needs no privilege. Two reads differ by tool: reading the file sees one layer; `systemctl cat`/`show` see the merge.
- Q6: value is an enum, no sub-parts. Addressing is hierarchical: unit → `[Service]` section → key, across layered files. The file has sections; the key is one line.
- Q7: writing: nothing until `daemon-reload` (systemd warns "changed on disk", `NeedDaemonReload=yes`, +SURE); after reload, applies to *future* exits of the service, not the running instance. Changed by others: `systemctl edit` drop-ins, package upgrade of the vendor copy when `/etc` has no override, `RestartPreventExitStatus=` and `RestartSec=` modulate its effect.
- Q8: unit name → file path(s) → section → key. Reachable by editing the path, by `systemctl edit`, by `show`, by D-Bus. Two tools disagree on *which layer* they show.
- Q9: admin file vs `systemctl edit` drop-in vs config-management template: drop-ins are the designed decollision (separate files, lexical order, +SURE). In-file, editors write-rename; no lock.
- Q10: file inode; the line and the value have none.
- Q11: persisted = file(s); live = loaded unit. `daemon-reload` moves persisted→live; `NeedDaemonReload` flags disagreement (+SURE). Never live→persisted (except `systemctl edit`, which writes then reloads).

### 4. `systemctl show -p InvocationID nginx.service`
- Q1: minted at each start, cleared at stop; the property is always enumerated, empty when inactive (+SURE) → *unset*. Not deletable independently. Unknown unit: `show` prints a minimal property set with an empty value (~SUSPECT).
- Q2: `$INVOCATION_ID` in the service's environment (+SURE, systemd ≥ 232), symlink `/run/systemd/units/invocation:nginx.service` whose target is the ID (+SURE), journal field `_SYSTEMD_INVOCATION_ID` (+SURE), D-Bus property, aliased unit names.
- Q3: yes by design: every start mints a fresh random 128-bit ID; never reused (+SURE).
- Q4: PID 1 memory + `/run` symlink + child env + journal. Lost at reboot; survives daemon-reexec (serialized, +SURE); nginx restart *is* a new one; re-provision: new.
- Q5: system vs user manager. Env var only inside the service's own processes. Not user/cwd.
- Q6: none; opaque. Addressed via the unit.
- Q7: not user-writable; changes only as a consequence of start. Journal lines carry it.
- Q8: unit; process env; journal; `/run` symlink.
- Q9: systemd only.
- Q10: it *is* an identity — of the activation, not of the unit.
- Q11: live only; the journal keeps history but no persisted "intended" form. A measurement of items 1 or 24-ish taken under one InvocationID is stale exactly when this changes.

### 5. `stat -c %a /etc/nginx/nginx.conf` — mode bits
- Q1: not deletable while the inode exists; every inode has `st_mode` (+SURE). File missing → `stat` fails ENOENT → *absent*, no slot, no default. `%a` shows 3 digits unless setuid/setgid/sticky set (+SURE).
- Q2: yes, many: any path to the inode (hard links, bind mounts, `/proc/<pid>/fd/N`, `..`-relative, doubled slashes); symlinks only with `stat -L` (+SURE plain `stat` stats the link itself); `ls -l`, `find -printf %m`, `getfacl` (which shows group bits as the ACL *mask* when an ACL exists, +SURE), `namei -m`.
- Q3: yes: the path can come to denote a new inode after write-rename (editors, `sed -i`, atomic writers; GNU `sed -i` copies mode, ~SUSPECT ownership only when root); freed inode numbers are reused (+SURE ext4/xfs).
- Q4: in the inode on disk (+SURE); survives reboot and every process; survives re-provision only if recreated the same way (dpkg sets from the .deb; hand-created files get umask, +SURE). Page-cache coherent, no separate live form.
- Q5: mount namespace (different root); user namespaces remap uid/gid, not mode bits (+SURE); cwd only for relative paths; the observing user only affects whether the parent dirs are searchable, not the value. FUSE/NFS may present synthetic modes (~SUSPECT for some FUSE fs). `umask` affects creation only.
- Q6: bits: u/g/o × rwx + setuid/setgid/sticky. `chmod u+x` is read-modify-write in the utility; `chmod(2)` writes the whole mode word (+SURE) → sub-parts are logically separable, physically not, so concurrent partial writes race. Not hierarchical itself; addressed by a hierarchical path.
- Q7: writing: ctime changes (+SURE); ACL mask entry rewritten if an ACL exists (+SURE); affects future `access(2)`. Changed by others: `chown` clears setuid/setgid on regular files (+SURE for non-root, ~SUSPECT also for root), `setfacl` rewrites the group triad, `cp --preserve`, file replacement (new inode with umask-derived mode), dpkg conffile handling, `dpkg-statoverride`, `chattr +i` blocks chmod (+SURE).
- Q8: through the path → inode. Two paths (item 7), bind mounts, `/proc/self/fd`. Several tools.
- Q9: dpkg (install-time mode), admin `chmod`, config management, editors on save, `setfacl`. `dpkg-statoverride` is the registered-intent mechanism so dpkg does not clobber admin modes (+SURE). Otherwise none.
- Q10: inode has `(st_dev, st_ino)` and a generation number (`FS_IOC_GETVERSION`, NFS handles, +SURE exists, rarely used). The mode itself has none.
- Q11: persisted only; the on-disk inode *is* the live form. Open fds keep the inode alive after unlink, so a process can hold an old identity while the path shows a new one (+SURE).

### 6. bytes of `/etc/nginx/nginx.conf`
- Q1: file deletable while nginx keeps running on its parsed copy (+SURE nginx reads config at start/reload only). Empty file = present-but-empty (*unset-ish*); missing = ENOENT (*absent*). dpkg does not restore a deleted conffile on upgrade (admin removal is respected, +SURE; `--force-confmiss` restores).
- Q2: any path to the inode (as item 5); `nginx -T` dumps the *effective* config re-read from disk including `include`d files (+SURE it re-parses from disk, not from the master's memory); `/proc/<pid>/fd` while open.
- Q3: yes: write-rename replaces the inode under the same path; same bytes may live in a new inode.
- Q4: on disk; survives reboot and processes; re-provision only if packaged/templated. Live form: nginx master process memory (parsed tree), copied into workers on reload (+SURE).
- Q5: mount ns/chroot; cwd for relative paths; readability by the observing user (mode 600 → EACCES, so the *value* is unavailable rather than different). Not env.
- Q6: lines, directives, blocks — a hierarchy the filesystem does not know. Sub-parts are not separately writable at the fs level except by `pwrite` of fixed-length byte ranges (+SURE); any length-changing edit rewrites the tail or the file. nginx's `include conf.d/*.conf`, `sites-enabled/` is the idiom that makes sub-parts separately addressable files (+SURE).
- Q7: writing: mtime/ctime, possibly new inode; nothing in nginx until `nginx -s reload`/SIGHUP or restart (+SURE); dpkg conffile prompt on next upgrade if the md5 differs from the shipped one (+SURE). Changed by others: package upgrades (conffile merge), certbot's nginx plugin rewrites server blocks (+SURE, marks lines with `# managed by Certbot` — a convention, not a mechanism), config management, `sed -i`.
- Q8: path; package (`nginx-common` owns it on Debian, ~SUSPECT); nginx (`-c`, `-T`); `/proc/*/fd`.
- Q9: admin, certbot, dpkg, config management. Decollision in practice: file-per-writer via `include` (+SURE), dpkg's `.dpkg-dist`/`.dpkg-old` prompts, marker comments (Ansible `blockinfile`, certbot). No lock.
- Q10: inode; dpkg records the shipped md5sum in `/var/lib/dpkg/status` `Conffiles:` (+SURE) → the *package* gives the content a fingerprint identity.
- Q11: persisted = file; live = nginx master's parsed config. Reload moves persisted→live; disagree after any edit until reload. There is no tool read of the live form (`-T` re-reads disk) — the only evidence of the live form is behaviour or the reload timestamp (~SUSPECT nothing else). Central seam: an edit line and a reload line touch different things.

### 7. `stat -c %i /etc/nginx/nginx.conf` and a hard link under another path
Two items: (a) the inode number as seen via path A, (b) the same via path B. Relation: two directory entries naming one inode; everything in items 5/6 is shared; only the name, the parent directory, and the parent's mtime are per-path (+SURE).
- Q1: a link can be created/removed while the inode persists (link count ≥ 1); the inode dies when the last link *and* last open fd go (+SURE). Removing one path makes that name *absent* (ENOENT); nothing on the surviving path says an alias vanished except `%h` decrementing.
- Q2: this item *is* the alias. `%h` (link count) proves aliases exist but not where; `find -samefile` / `find -inum` enumerates them per filesystem (+SURE); `debugfs ncheck` on ext* (+SURE).
- Q3: yes: freed inode numbers are reused (+SURE ext4; xfs numbers encode disk location so reuse is common); a path can come to denote a different inode. `st_ino` alone is not a stable identity across delete/recreate; with generation it is (+SURE).
- Q4: on disk, both dentries. Survives reboot; re-provision usually *breaks* hard links: `rsync` needs `-H`, `tar` preserves, `cp -a` preserves only among files copied in one invocation (+SURE), config-management file modules write files independently (+SURE by construction).
- Q5: mount ns: a bind mount shows the same `(st_dev, st_ino)`; `st_dev` is stable across mount namespaces for block-backed fs (+SURE) but synthetic and non-unique on btrfs subvolumes (~SUSPECT the well-known wart) and assigned at mount time for NFS (~SUSPECT). Not user/cwd/env beyond path resolution.
- Q6: the number has none. The file's addressing is by path; the inode is flat within a device.
- Q7: `ln`: target's ctime and link count change (+SURE), parent dir mtime/ctime. `rm` of one path: link count decrements. An editor's write-rename through one path *silently breaks the aliasing* (+SURE: `sed -i`, most atomic writers; vim depends on `backupcopy`), so a content write to one name can change whether the two names alias. Cannot cross filesystems or link directories (+SURE).
- Q8: paths; `/proc/*/fd`; inode number via `find -inum`.
- Q9: two writers through two paths clobber at the content level if they write in place, or *diverge silently* if either write-renames. No mechanism.
- Q10: this *is* the substrate identity: `(st_dev, st_ino)`, optionally generation; `name_to_handle_at(2)` gives a persistent handle (+SURE).
- Q11: persisted only.

### 8. `getfattr -n user.comment /srv/x`
- Q1: yes: `setfattr -n` / `-x` while the inode persists (+SURE). Missing: `getfattr -n` exits 1 with "No such attribute" (ENODATA, +SURE) and `getfattr -d` enumerates only present ones → *absent*, no default. Third kind: filesystem without xattr support → ENOTSUP (+SURE; tmpfs gained `user.*` late, ~SUSPECT 6.6).
- Q2: any path to the inode; `attr -g comment` (prepends `user.`, +SURE); `os.getxattr`; `rsync -X`/`tar --xattrs` payloads. Value encoding differs by flag (`--only-values` for raw, +SURE).
- Q3: yes: remove/re-add; a write-rename of the file drops all xattrs unless the writer copies them (+SURE most editors do not; `cp` needs `--preserve=xattr`; `rsync` needs `-X`; GNU `sed -i` ~SUSPECT does not).
- Q4: on disk in the inode's xattr area; survives reboot/processes; re-provision almost never preserves `user.*` (dpkg does not carry them, ~SUSPECT; tar needs `--xattrs`). Over NFS only with v4.2 (+SURE RFC 8276), else ENOTSUP.
- Q5: mount ns; `user.*` get needs read permission on the file, set needs write permission, sticky dirs restrict (+SURE xattr(7)). `trusted.*` needs CAP_SYS_ADMIN (+SURE). Not cwd/env.
- Q6: value is an opaque blob; the name is two-level (`namespace.name`), flat beyond the prefix (+SURE). Each attribute is separately and atomically settable; the inode's attribute set is a small KV store with a shared size budget (ext4 about one block, +SURE-ish).
- Q7: writing: ctime changes (+SURE); nothing else. Changed by others: file replacement drops it; `cp` without preserve drops it; sibling xattrs (`system.posix_acl_access` from `setfacl`, `security.selinux` from relabel) share the size budget but do not touch it (+SURE).
- Q8: file; two paths; three tools with different encodings.
- Q9: rare; per-attribute atomic set; distinct names are the decollision; no lock.
- Q10: none of its own; the inode's.
- Q11: persisted only; no live form.

### 9. `git config --get core.editor` (scope `~/.gitconfig`)
- Q1: yes: `git config --global core.editor X` / `--unset` (+SURE). Missing: `--get` prints nothing, rc 1 → *absent* at the file layer; the *effective* editor is never missing (item 10) → *unset-with-fallback* at the resolved layer. Multi-valued occurrences possible (`--get-all`).
- Q2: yes. Section and key are case-insensitive (`Core.Editor`, +SURE; subsection names are case-sensitive). The "global scope" is *two files*: `~/.gitconfig` and `$XDG_CONFIG_HOME/git/config`; both are read, `~/.gitconfig` wins on read, `--global` writes to `~/.gitconfig` if it exists else XDG (+SURE). `GIT_CONFIG_GLOBAL` redirects (+SURE, since 2.32). `include`/`includeIf` can pull the key from another file (+SURE). `--show-origin`/`--show-scope` disambiguate.
- Q3: yes: unset/set; the "thing" is really "the last matching line across included files" (+SURE last wins). `git config` rewrites via `~/.gitconfig.lock` + rename (+SURE).
- Q4: `~/.gitconfig` on disk, per user, per machine unless home is shared/synced. No owning daemon (~SUSPECT fsmonitor/credential daemons do not cache config). Survives reboot trivially; re-provision via dotfile managers only.
- Q5: strongly: `HOME`, `XDG_CONFIG_HOME`, `GIT_CONFIG_GLOBAL`, `GIT_CONFIG_NOSYSTEM`, env-injected `GIT_CONFIG_COUNT`/`KEY_n`/`VALUE_n` (+SURE); `sudo` changes `HOME` or not per sudoers (+SURE classic); without `--global`, cwd layers in local/worktree config and `includeIf "gitdir:"` makes even the global *file's* effective content cwd-dependent (+SURE).
- Q6: `section.subsection.key`, three-level; each key separately writable; typed coercion on read (`--type=bool`). Path-like: yes.
- Q7: writing: file rewritten preserving comments/layout (+SURE); affects every git process started afterwards; nothing running changes. Changed by others: the human's editor, `gh`, IDEs, `git lfs install`, dotfile syncs (+SURE many tools write this file).
- Q8: through the file path; through git's scope abstraction; through the key. The *same key name* also exists in system/local/worktree scopes: same name, different stores; the effective value is the merge (+SURE).
- Q9: many tools; they coexist by using `git config` (locked, key-scoped RMW) rather than templating the file; `[include] path = ~/.gitconfig.local` is the idiom for machine-local vs synced (+SURE common).
- Q10: none.
- Q11: persisted only; every read re-parses. Disagreement only via the env overlay (item 10).

### 10. `git var GIT_EDITOR`
Resolution order: `GIT_EDITOR` env, then `core.editor` (merged scopes), then `VISUAL`, then `EDITOR`, then `vi` (+SURE documented; `VISUAL` skipped when the terminal is dumb, ~SUSPECT exact rule).
- Q1: never missing (default `vi`) → *unset-with-default* at worst. Cannot be created or deleted; only contributors can.
- Q2: no second spelling of the resolved value; every contributor is a partial alias. `git var -l` lists.
- Q3: n/a: computed per invocation.
- Q4: nowhere; computed from the caller's environment plus files at each call. The env part dies with the shell session; the file part persists as item 9.
- Q5: entirely: env (`GIT_EDITOR`, `VISUAL`, `EDITOR`, `HOME`, `GIT_CONFIG_*`, `TERM`), user, cwd (repo-local scope, `includeIf`).
- Q6: none; a shell command string git will `sh -c`. Not hierarchical.
- Q7: not writable; changes when any contributor changes.
- Q8: git only; relates to item 9 as "resolved-from". The two reads (9 vs 10) disagreeing is the *signal* that an env or other-scope override exists.
- Q9: not writable as a unit.
- Q10: none.
- Q11: no persisted form of its own; it is a live derivation. This item is a *view*, not a store; the analyzer needs that category (see E).

### 11. `sysctl kernel.pid_max`
- Q1: cannot be created/deleted; always present on Linux → no "unset" at the live layer. Missing only off-Linux/old kernels → *absent* (sysctl: cannot stat). The persisted *line* in `/etc/sysctl.d/*.conf` can be absent → then the live value is the kernel's compiled default (*unset* at the persisted layer).
- Q2: `kernel/pid_max` slash form (+SURE accepted), `/proc/sys/kernel/pid_max` path, `sysctl -n`, `cat`. Dots-vs-slashes ambiguity exists for keys whose components contain dots (+SURE documented gotcha).
- Q3: no; fixed name, value changes.
- Q4: kernel memory; historically global, per-pid-namespace in very recent kernels (~SUSPECT 6.14). Lost at reboot, re-applied by `systemd-sysctl` from `/etc/sysctl.d` at boot (+SURE). No owning process. Re-provision: the config file.
- Q5: pid namespace if per-ns; readable by all; `/proc/sys` may be read-only or masked in containers (+SURE runtimes do this). Not cwd/env.
- Q6: none (integer). Name is hierarchical (`kernel.pid_max` is a procfs path).
- Q7: writing: immediate effect on the PID allocator; bounds-checked (+SURE min about 301). Nothing else changes. Changed by others: `sysctl --system`, `systemd-sysctl`, container runtimes, direct `echo >`; `tuned` profiles.
- Q8: the key or the procfs path; persisted via `/etc/sysctl.conf`, `/etc/sysctl.d/`, `/run/sysctl.d/`, `/usr/lib/sysctl.d/` (+SURE layering, same-basename override). Ansible `sysctl` writes both forms.
- Q9: multiple `.d` files (lexical order, last wins, +SURE); wholesale overwrite; no lock. Practice: numeric-prefixed files per writer.
- Q10: none; the key is the identity.
- Q11: persisted (`sysctl.d`) vs live (`/proc/sys`); `sysctl --system`/boot moves persisted→live; disagree after `sysctl -w` without a file edit or a file edit without reapply. Two reads, two forms.

### 12. `sysctl net.ipv4.conf.eth0.forwarding`
- Q1: created/deleted *with the interface*: `/proc/sys/net/ipv4/conf/<ifname>/` appears on device creation and vanishes on removal (+SURE). Missing iface → *absent*. Persisted lines naming an interface that does not yet exist at boot fail silently; systemd added a udev-triggered `systemd-sysctl --prefix` for this (+SURE the ordering bug and fix exist).
- Q2: slash form `net/ipv4/conf/eth0/forwarding` (required if the ifname contains dots, +SURE); the procfs path; `ip netns exec <ns> sysctl`. Semantically aliased by `all.forwarding`/`ip_forward` (item 13). Interface altnames are not exposed here (~SUSPECT only the primary name).
- Q3: yes: interface deleted/recreated (veth, VLAN, bridge) or *renamed*; `eth0` may denote a different netdev (ifindex differs; ifindex is monotonic per netns, +SURE).
- Q4: kernel memory, per netdev, per netns; lost at reboot and at device removal; persisted only via `sysctl.d` (with the ordering caveat) or the network manager's config (`systemd-networkd` `IPForward=`, +SURE it writes this key).
- Q5: network namespace (+SURE the whole `net.*` tree is per-netns). Readable by all. Not cwd/env.
- Q6: none. Hierarchical name whose *middle component is another thing's name* (the interface): a slot keyed by a foreign key; the important seam.
- Q7: writing: immediate. Writing `conf.all.forwarding` overwrites every `conf.<iface>.forwarding` (+SURE: `inet_forward_change` walks all devices; this propagation is specific to `forwarding`; most other `conf.*` keys are combined with `all` at use time instead). `conf.default.*` is copied into newly created devices (+SURE). Device deletion deletes it.
- Q8: through the interface name, the netns, the sysctl tree; tools: `sysctl`, `cat`, `ip netns exec`, networkd.
- Q9: networkd, Docker (enables `ip_forward` at startup if off, +SURE), libvirt, CNI plugins, firewalld (masquerade), `sysctl.d`, admin. No lock; last write wins; an `all` write from any of them silently clobbers a per-iface value (+SURE).
- Q10: the netdev has an ifindex (and `ifalias`); the value has none.
- Q11: persisted vs live as item 11, plus the propagation rule means a persisted per-iface line can be overridden at runtime by a *different key's* write.

### 13. `net.ipv4.ip_forward` and `net.ipv4.conf.all.forwarding`
Relation: one kernel word, two sysctl entries; `ip_forward` is declared in `devinet.c` pointing at the `all` devconf's forwarding slot (+SURE same storage). Writing either propagates to every per-device entry (item 12).
- Q1: neither creatable/deletable; always present. Two names, one slot.
- Q2: this item *is* the alias; plus the procfs paths. IPv6 has only `net.ipv6.conf.all.forwarding`, no `ip6_forward` (+SURE).
- Q3: no.
- Q4: kernel, per-netns; not reboot-persistent; `sysctl.d`.
- Q5: netns.
- Q6: none; the two spellings sit at *different depths* of the hierarchy, so a tree-walker sees two leaves.
- Q7: writing either: all per-iface `forwarding` values overwritten (+SURE). Changed by others: Docker, libvirt, firewalld, CNI, `sysctl.d`; `sysctl -a` lists *both* keys (+SURE), so a naive enumeration double-counts.
- Q8: two names in one tool.
- Q9: many writers, clobbering; Docker's "set if 0" is idempotent-ish. Two persisted lines with the two spellings and opposite values can both exist in `sysctl.d`; the later-applied wins, order-dependent (+SURE a real footgun).
- Q10: none.
- Q11: persisted vs live; and the persisted layer can hold two contradictory spellings.

### 14. `dpkg-query -W -f '${db:Status-Status}' nginx`
The third word of the `Status:` triple (want / eflag / status), e.g. `installed`, `config-files`, `half-configured`, `not-installed` (+SURE the triple).
- Q1: the stanza is created by install; `apt remove` leaves `deinstall ok config-files` (*unset-with-known-state*: a slot with a meaningful non-installed value); `apt purge` drops the stanza (~SUSPECT a stub can linger as `unknown ok not-installed` when other references exist). Unknown name: `dpkg-query` rc 1 "no packages found" → *absent*.
- Q2: `dpkg -s nginx` `Status:` line, `dpkg -l` first two letters, the stanza in `/var/lib/dpkg/status`, `apt-cache policy` `Installed:`, `apt list --installed`, python-apt. Arch-qualified `nginx:amd64` (+SURE). Beware: on Debian `nginx` is a metapackage; the *daemon* comes from `nginx-core`/`nginx-full` etc. (+SURE), so "nginx installed?" has several package names and `Provides:` in play.
- Q3: remove/reinstall rewrites the stanza under the same name; the installed *instance* is new (files, version).
- Q4: `/var/lib/dpkg/status`, a text file rewritten wholesale with `status-old` (+SURE). Disk; survives reboot; no daemon; re-provision = reinstall. Live form: files listed in `/var/lib/dpkg/info/nginx.list` actually on disk.
- Q5: `--admindir`/`DPKG_ADMINDIR`/`--root` (+SURE), chroot/mount ns. Not user (world-readable), not cwd.
- Q6: three words, separately addressable (`${db:Status-Want}`, `-Eflag`, `-Status`, +SURE); the want-word is written by `dpkg --set-selections`/`apt-mark` (item 16), the status-word by dpkg's state machine. Name flat, with arch qualifier.
- Q7: writing (install/remove/purge): files unpacked or removed, maintainer scripts, triggers (`man-db`, `ldconfig`, `systemd`), `/var/lib/dpkg/info/*` created, alternatives registered (item 36), units enabled via presets (item 2), system users via `adduser --system`. Changed by others: only apt/dpkg frontends (+SURE nothing else writes status).
- Q8: through the package name. Tools: dpkg-query, apt-cache, apt list; apt keeps *additional* stores of its own (`/var/lib/apt/extended_states` for auto/manual, +SURE) that dpkg does not know.
- Q9: apt, dpkg, aptitude, unattended-upgrades: real locking via `/var/lib/dpkg/lock` and `lock-frontend` (+SURE); the only item in the seed with a genuine lock.
- Q10: none; name+version+arch is the closest thing.
- Q11: persisted = status file, which *is* the authority; live = files on disk and the running daemon; disagree when files are deleted by hand (`dpkg -V` compares md5sums, +SURE) or the state machine was interrupted (`half-installed`, `reinstreq` express "db and disk disagree", +SURE).

### 15. `dpkg-query -W -f '${Version}' nginx`
- Q1: same stanza as 14. In `config-files` state the `Version:` of the removed package is *retained* (+SURE, because conffiles are still tracked) → a stale value that looks set. Missing → *absent*.
- Q2: `dpkg -s` `Version:`, `dpkg -l` third column, `apt-cache policy` `Installed:`, `${binary:Version}`/`${source:Version}` (+SURE exist and can differ). The epoch (`1:1.18.0-…`) is shown by dpkg but routinely lost by human parsers and by `nginx -v` (+SURE the binary does not know its Debian epoch).
- Q3: upgrades/downgrades change the value under the same name.
- Q4: as 14.
- Q5: as 14.
- Q6: epoch : upstream - revision (+SURE grammar; compare with `dpkg --compare-versions`). Not separately writable; comes whole from the `.deb`.
- Q7: writing = installing another `.deb`: everything in 14 Q7. Cannot be written alone.
- Q8: as 14, plus `nginx -v` (the binary's self-report: usually agrees, disagrees when the binary came from elsewhere, +SURE).
- Q9: as 14.
- Q10: none for the installed instance (~SUSPECT dpkg records no hash of the installed `.deb`); the *archive* has SHA256 in apt lists.
- Q11: persisted (status) is the authority; "live" = what the binary reports; can disagree.

### 16. `apt-mark showhold` (whether nginx is held)
- Q1: hold set/cleared while the package persists (`apt-mark hold`, `echo nginx hold | dpkg --set-selections`, +SURE). Not held = want-word `install` → *unset-with-explicit-default*: `dpkg --get-selections` enumerates every package with its want-word (+SURE), `showhold` enumerates only holds. Holding a not-installed package: dpkg accepts a selection for an unknown name and creates a stub (~SUSPECT).
- Q2: `dpkg --get-selections`, `${db:Status-Want}`, `dpkg -l` first letter `h` (+SURE). *Semantic* alias with different storage: apt pinning in `/etc/apt/preferences.d/` (`Pin-Priority: -1` or `Pin: version …`) blocks upgrades by a different mechanism (+SURE) and does not show in `showhold`.
- Q3: n/a for the flag; the package under the name can be reissued (14 Q3) and the hold survives that (~SUSPECT the want-word persists across remove/reinstall; not across purge).
- Q4: the want-word in `/var/lib/dpkg/status` (+SURE `apt-mark hold` calls `dpkg --set-selections`). Disk; survives reboot; no daemon; re-provision only if replayed.
- Q5: as 14.
- Q6: one word of the Status triple; addressed by package name.
- Q7: writing: `apt upgrade`/`full-upgrade` keeps the package back (+SURE); explicit `apt install nginx` and `dpkg -i` behaviour around holds is version-dependent (~SUSPECT dpkg refuses without `--force-hold`; apt-get install of a held package upgrades it and prints a note). Changed by others: `apt-mark`, `dpkg --set-selections`/`--clear-selections`, `apt install`/`remove` rewriting want to `install`/`deinstall` (+SURE remove sets `deinstall`).
- Q8: through the package; two tools on one slot; a third mechanism (pinning) with separate storage that can contradict it.
- Q9: apt-mark and dpkg selections write the same word under the dpkg lock (+SURE). Pinning files are separate and unlocked.
- Q10: none.
- Q11: persisted only; apt reads it per run. Pinning is a second persisted form of overlapping intent.

### 17. `getent passwd alice | cut -d: -f7` — login shell
- Q1: the field always exists in a passwd entry; empty means `/bin/sh` to `login` (+SURE) → *unset-with-default*. User deleted → `getent` rc 2 → *absent*. `/usr/sbin/nologin` is a *value* that means "no login": tool-level unset spelled as a value (+SURE by convention).
- Q2: the `/etc/passwd` line; `getent passwd 1000` (by uid); NSS backends (`files`, `sss`, `ldap`, `systemd` for DynamicUser, +SURE) mean `grep alice /etc/passwd` and `getent passwd alice` can disagree (+SURE first match per `nsswitch.conf`). `$SHELL` in a live session is a *copy* taken at login (+SURE).
- Q3: `userdel`+`useradd alice` → same name, usually the same reused uid (`useradd` picks the lowest free uid ≥ UID_MIN, +SURE) → files owned by the old uid become the new alice's (+SURE the classic). Both name and uid are reused; the account is new.
- Q4: `/etc/passwd` on disk for local; remote directory plus `sssd` cache (`/var/lib/sss/db`, +SURE) for others. Survives reboot; no daemon for `files`. Re-provision: only if the provisioner recreates.
- Q5: NSS config and mount ns (a container sees its own `/etc/passwd`); user namespaces remap numbers, not names (+SURE). Readable by all.
- Q6: one of seven fields; each separately writable (`usermod -s/-d/-c/-u/-g`); `chsh` restricts non-root to `/etc/shells` (+SURE). Addressed by name or uid; flat.
- Q7: writing: `/etc/passwd` rewritten under the shadow-suite lock (`lckpwdf`, `passwd-` backup, +SURE); `nscd`/`sssd` caches may serve stale entries until invalidated (+SURE nscd caches passwd); running sessions keep their `$SHELL` copy. Changed by others: `usermod`, `chsh`, `useradd`, directory admins, `systemd-sysusers` (+SURE declarative creation), package postinst `adduser --system`.
- Q8: through the user (name or uid), the file, NSS, `finger`/`id`. Two tools (`getent` vs `grep`) can hit different stores.
- Q9: shadow-suite tools lock (+SURE); direct edits without `vipw` race; local vs directory both defining `alice`: `nsswitch` order decides (+SURE).
- Q10: uid is the kernel-level identity but is reused (Q3); no UUID locally; LDAP has `entryUUID` (+SURE).
- Q11: persisted only for `files`; `sssd` keeps a live cache that can disagree with the remote persisted form offline (+SURE); live sessions hold a stale `$SHELL`.

### 18. `id -u alice`
- Q1: exists iff the user exists → *absent* when missing (rc 1). Changeable (`usermod -u`), never unset.
- Q2: `getent passwd alice` field 3, `/etc/passwd`, `stat -c %u ~alice` (indirect). The reverse map `id -un 1000` is also many-to-one: duplicate uids are legal (`useradd -o`, +SURE).
- Q3: yes both directions: the name can get a new uid, the uid a new name; after delete/recreate both are typically reused (17 Q3).
- Q4: as 17.
- Q5: `id -u` *of the current process* is userns-mapped (+SURE `uid_map`); `id -u alice` is an NSS lookup, so the mount ns / nsswitch decide. Not cwd/env (`LD_PRELOAD`/`NSS_WRAPPER` aside).
- Q6: none; integer.
- Q7: `usermod -u`: chowns the home tree only (+SURE); files elsewhere keep the old number; running processes keep old credentials (+SURE); name-based `sudoers`/crontabs re-resolve to the new number. Changed by others: `userdel`/`useradd`.
- Q8: user; NSS; files' `st_uid` (the kernel only knows numbers, +SURE).
- Q9: as 17.
- Q10: it *is* the kernel's identity for ownership and credentials; the name is an NSS overlay; but it is reused, so it is identity-without-uniqueness-over-time.
- Q11: as 17; running processes carry a live copy of a resolution done at login.

### 19. `getent group wheel | cut -d: -f4` — member list
- Q1: group deletable (`groupdel` refuses if it is someone's primary group, +SURE); empty member list = present-but-empty (*unset-ish*); missing group → *absent*. Members added/removed individually (`gpasswd -a/-d`, `usermod -aG`).
- Q2: `/etc/group` line; `getent group 10`; the *inverse* view `id -Gn alice`/`groups alice`. Two stores for "is alice in wheel": the group's member field, and alice's *primary gid* in passwd, which `getent group` never lists (+SURE). `/etc/gshadow` duplicates the member list and can drift (`grpck`, +SURE). NSS backends as in 17.
- Q3: `groupdel`/`groupadd` → same name, likely reused gid → files with that gid re-attributed. Member list churns freely.
- Q4: `/etc/group` and `/etc/gshadow` (disk); directory + sssd cache. Survives reboot. Running processes hold a *copy* of supplementary groups taken by `initgroups` at login (+SURE).
- Q5: NSS; mount ns; `gshadow` root-only.
- Q6: comma-separated list; each member separately writable; four fields. Addressed by name or gid.
- Q7: writing: both files rewritten under lock; caches; *no running process changes until re-login* or `newgrp` (+SURE). Changed by others: `usermod -G` *without* `-a` replaces the user's whole supplementary set (+SURE classic clobber), `userdel` removes the user from all groups (+SURE), `useradd -G`, `sysusers`, directory admins.
- Q8: group; user (inverse); files; NSS.
- Q9: many; shadow-suite lock; the `-a` footgun means writers do clobber each other in practice.
- Q10: gid (reused); `entryUUID` remotely.
- Q11: persisted (files) vs live (each process's supplementary set); disagree until re-login; sssd cache.

### 20. `passwd -S alice` (the lock flag)
Second field of `passwd -S`: `P` usable, `L` locked, `NP` no password; derived from the shadow hash field (`!`/`*` prefix → L, empty → NP, +SURE).
- Q1: not deletable independently; user gone → *absent*. `NP` is its "unset". Important: "is the account locked" has at least four other stores this read does not see: account expiry (`usermod --expiredate 1`, `chage -l`, +SURE), shell `nologin` (item 17), `/etc/nologin`, `faillock`/`pam_tally2` counters in `/var/run/faillock/` (+SURE tmpfs). One read, one of several locks.
- Q2: `getent shadow alice` field 2 first char (root); `usermod -L` and `passwd -l` write the same prefix (+SURE); `chage -l` shows expiry, not this. `passwd -S` for other users is root-only (+SURE).
- Q3: lock/unlock toggles the prefix; `passwd -u` refuses to unlock a hash-less `!` without `-f` (+SURE). User delete/recreate.
- Q4: `/etc/shadow` (disk, root-only); directory (`userPassword`, ppolicy lock attributes); `sssd` caches hashes only with `cache_credentials` (+SURE). Survives reboot.
- Q5: root-only; NSS; mount ns.
- Q6: one *character* of one field; the prefix and the hash are separately writable (`-l`/`-u` vs `passwd`/`chpasswd`, +SURE). Addressed by user name.
- Q7: writing lock: password auth fails; SSH pubkey auth still succeeds (+SURE the classic gap; `sshd` only honours the lock via PAM account checks in some configurations, ~SUSPECT distro-dependent); cron jobs still run (+SURE unless PAM-restricted); `su alice` from root still works (+SURE). Changed by others: `passwd`, `chpasswd`, `usermod -p`, `cloud-init`, directory admins.
- Q8: user; shadow; NSS; `passwd -S`, `chage`, `pwck`.
- Q9: shadow-suite lock as 17.
- Q10: none; uid.
- Q11: persisted only; effect is per-authentication; `faillock` is a separate live store lost at reboot (+SURE).

### 21. `ip -4 addr show eth0` (the addresses)
The thing is the *set* of IPv4 addresses on one netdev; each element is keyed by (dev, addr/prefix).
- Q1: elements created/deleted (`ip addr add/del`) while the interface persists (+SURE); empty set = present-but-empty (*unset-ish*); interface missing → "Device does not exist" rc 1 → *absent*.
- Q2: `ip -j`/`-o` forms; `ifconfig eth0` shows only the *first* IPv4 address unless legacy labels `eth0:0` are used (+SURE, a real pitfall); `hostname -I` (all addresses, all interfaces, +SURE); `/proc/net/fib_trie`; `networkctl status`; `nmcli device show` (`IP4.ADDRESS[n]`); `getifaddrs(3)`; netlink `RTM_GETADDR`. Interface altnames (`ip link property add altname`, +SURE) give the *device* a second name. The same address may legally sit on two interfaces (+SURE), so the address alone is not a key.
- Q3: yes: DHCP renewal, manager reconfigure, interface recreation; a re-added address is a new element with new lifetimes/flags; no per-element id.
- Q4: kernel memory, per-netns, per-netdev (+SURE); lost at reboot and at device removal. Persisted form is *whichever manager owns the interface*: netplan YAML, `/etc/network/interfaces`, networkd `.network`, NM profiles under `/etc/NetworkManager/system-connections/`, plus DHCP lease files (`/var/lib/dhcp/*.leases`, `/var/lib/NetworkManager/`, +SURE). DHCP-assigned addresses have no local persisted intent at all beyond the lease.
- Q5: network namespace (+SURE). Readable by all. Not user/cwd/env.
- Q6: per element: address, prefixlen, broadcast, scope, label, flags (`dynamic`, `secondary`, `noprefixroute`, `deprecated`), `valid_lft`/`preferred_lft` (+SURE); some writable via `ip addr change`. Addressed by (dev, addr/prefix); the interface is the hierarchical parent.
- Q7: writing: kernel installs the connected route in `main` and `local` tables unless `noprefixroute` (+SURE); netlink event fans out to daemons (NM, networkd, `avahi`, `resolved`, `keepalived`, firewalld); deleting the last address in a subnet removes the route (+SURE). Managers may *revert* addresses they did not configure on their next reconfigure (networkd `KeepConfiguration=`, +SURE-ish; NM tolerates "external" addresses until the profile is re-applied, ~SUSPECT). Changed by others: DHCP lease events, `nmcli`, `netplan apply`, `ifup/ifdown`, cloud-init, IPv4LL, VRRP/keepalived VIPs (+SURE), Docker bridge creation, carrier loss handling by the manager (~SUSPECT NM removes DHCP addresses on carrier loss).
- Q8: through the interface (name, ifindex, altname); the netns; the manager's profile; the lease file; `hostname -I`.
- Q9: DHCP client, NM, networkd, keepalived, admin, Docker, cloud-init. Coexistence by *ownership convention*: NM `managed`/`unmanaged` per device (+SURE), networkd `KeepConfiguration=`, keepalived VIPs by convention. No lock; last netlink write wins; managers may undo manual writes (+SURE observed behaviour).
- Q10: the interface has an ifindex (+SURE); the address has no id.
- Q11: persisted (manager profile or lease) vs live (kernel); manager moves persisted→live at boot/apply/lease; manual `ip addr add` creates disagreement the manager may later resolve *in its own favour*.

### 22. `ip link show eth0` — the netdev
- Q1: the device itself is created/deleted (virtual: `ip link add/del`; physical: driver bind, hotplug); missing → "Device does not exist" → *absent*. Its attributes (MTU, MAC, flags) always have values, never unset; `altname`s are add/removable.
- Q2: `dev eth0`; altnames (+SURE); sysfs `/sys/class/net/eth0/` (+SURE); `ifconfig`; `networkctl status`; `nmcli device`; `ethtool`; `/proc/net/dev`; PCI path `/sys/bus/pci/devices/…/net/`; udev `ID_NET_NAME_*`. `ifalias` is a free-text label, not a lookup name (+SURE).
- Q3: yes: `eth0` can be re-bound to a different device by udev rename or `ip link set name`; ifindex is monotonic per netns (+SURE) so a recreated virtual device has a new ifindex; a replaced NIC has a new MAC and possibly a new predictable name.
- Q4: kernel `net_device`, in exactly one netns, movable with `ip link set netns` (+SURE). Lost at reboot: physical devices re-enumerated by drivers, virtual ones rebuilt by managers. Persisted forms: udev `.rules`/`.link` files for name/MAC/MTU (+SURE), `.netdev`/netplan/NM profiles for virtual devices, hardware EEPROM for the MAC (runtime overrides are lost).
- Q5: netns (+SURE). Readable by all. Not cwd/env.
- Q6: many named sub-parts, most separately writable via `ip link set`: admin `up`/`down`, MTU, `address`, `master`, `qdisc`, `txqueuelen`, `promisc`, `alias`, `altname`, `group`, `netns` (+SURE). Read-only: operstate/carrier, ifindex. Addressed by name/ifindex; sysfs is a path-like view.
- Q7: writing `down`: routes over the device are flushed and IPv6 addresses removed, while IPv4 addresses persist (+SURE the asymmetry); managers and DHCP clients react. MTU change caps VLAN children and alters PMTU. MAC change invalidates peers' ARP caches and DHCP client identity. `netns` move takes addresses and routes with it. *Rename* silently re-keys the sysctl tree (item 12) and every firewall/manager config that names the interface (+SURE). Changed by others: udev at boot, NM/networkd activation (re-asserting MTU from profile, +SURE), `ethtool`, carrier events, bridge/bond enslavement, Docker/CNI creating veths, `wpa_supplicant`.
- Q8: name, ifindex, altname, sysfs path, PCI address, MAC; many tools on one kernel object.
- Q9: NM vs networkd vs ifupdown vs admin: coexistence by NM `unmanaged-devices` and networkd match rules (+SURE); netlink is last-write-wins; managers re-assert their profile on activation.
- Q10: ifindex (per netns), MAC (hardware but spoofable), PCI slot, udev `ID_PATH`: several identities of differing stability (+SURE).
- Q11: persisted (link files, udev, profiles) vs live (kernel); boot/udev/manager moves; renames are the worst case: a persisted rule and a live name can point at different hardware.

### 23. `findmnt -no OPTIONS /srv` (`ro`, and `noatime`)
Two sub-items on one mount. Note `findmnt /srv` matches only an exact mountpoint; `findmnt -T /srv` walks up to the containing mount (+SURE) — different things.
- Q1: the mount is created/removed (`mount`/`umount`) while device and filesystem persist. `rw`/`ro` is always printed, one or the other (+SURE) → never unset. `noatime` missing means the default `relatime`, which `findmnt` prints explicitly (+SURE, kernel default since 2.6.30) → *unset spelled as default*. No mount at the path → nothing printed, rc 1 → *absent*.
- Q2: `/proc/self/mountinfo` (authoritative, +SURE), `/proc/mounts`, `/etc/mtab` (a symlink to it on modern systems, +SURE), `mount`, `systemctl show srv.mount -p Options` (+SURE systemd mirrors every mount as a unit), `statvfs` `f_flag` bits `ST_RDONLY`/`ST_NOATIME` (+SURE), `df`. **`ro` is two different things with one spelling**: per-mount `MNT_READONLY` (bind mounts, `ReadOnlyPaths=`) and per-superblock `SB_RDONLY` (+SURE); `findmnt -o VFS-OPTIONS,FS-OPTIONS` separates them (+SURE). Two bind mounts of one superblock share the superblock's `ro` but not the per-mount one.
- Q3: yes: umount/mount gives a new mount id (~SUSPECT ids are monotonic, not reused); `/srv` can become a *different* mount by over-mounting, and `findmnt /srv` shows the topmost only while `mountinfo` shows both (+SURE).
- Q4: kernel mount table, per mount namespace (+SURE); lost at reboot; persisted in `/etc/fstab` or `.mount`/`.automount` units, converted by `systemd-fstab-generator` (+SURE). Re-provision: fstab.
- Q5: mount namespace, strongly (+SURE): containers and systemd services with `ProtectSystem=`/`ReadOnlyPaths=` see their own read-only *views*; `mountinfo` lists only mounts reachable from the process's root (+SURE); propagation (`shared`/`private`/`slave`) decides whether a mount made elsewhere is visible (+SURE). cwd for relative targets. Readable by all.
- Q6: comma-separated options; each logically separate, but `mount -o remount,…` re-specifies a set with util-linux merge semantics that differ depending on whether device and mountpoint are both given (~SUSPECT the exact rule; it bites). Addressed by mountpoint, by source device, by mount id, by unit name.
- Q7: `remount,ro`: EBUSY if files are open for write (+SURE), else writers get EROFS; journaled fs flush. `noatime` only stops atime updates. Bind mounts sharing the superblock change together (+SURE). Changed by others: kernel `errors=remount-ro` on fs errors (+SURE), `systemd-remount-fs` at boot applying fstab options (+SURE), `fsck`/emergency, automounters, container runtimes and `snapd` over-mounting.
- Q8: mountpoint path, source device, mount id, `srv.mount` unit, the fstab line, the superblock (fs UUID). One superblock behind several mounts.
- Q9: fstab (admin) vs generated units vs manual `mount` vs runtimes; no lock; `remount` is last-wins; `findmnt --verify` compares fstab to the live table (+SURE).
- Q10: mount id and parent id (mountinfo), superblock `major:minor`, filesystem UUID/label via `blkid` (identity of the *filesystem*, not the mount, +SURE), `statfs` `f_fsid`. The option has none.
- Q11: persisted (fstab/units) vs live (table); `mount -a`/boot moves; disagree after a manual remount, or after the kernel's own `remount-ro`.

### 24. `docker inspect -f '{{.State.Status}}' web`
- Q1: container created/removed (`docker create`/`rm`; `--rm` auto-removes on exit, +SURE) while image and daemon persist. Status always present for an existing container → never unset. Missing → "No such object", rc 1 → *absent*.
- Q2: name `web` vs 64-hex ID or unique prefix (+SURE); `docker ps --format '{{.State}}'` (`.Status` in `ps` is the human string "Up 3 hours", a different rendering, +SURE); the API `/containers/web/json`; `docker events`; containerd's view via `ctr -n moby` (same ID, +SURE); the cgroup `docker-<id>.scope` (+SURE); `ps` for the process. Compose names the same container `<project>-web-1` (+SURE). Names can be changed with `docker rename` (+SURE).
- Q3: yes: `rm` + `run --name web` → same name, new ID (+SURE); `docker compose up` recreates on config change (+SURE). `restart` keeps the ID but the process is new (`Pid`, `StartedAt` change): the InvocationID analogue.
- Q4: dockerd memory plus `/var/lib/docker/containers/<id>/config.v2.json` and `hostconfig.json` (+SURE); containerd runtime state under `/run/containerd`. Daemon restart stops containers unless `live-restore` (+SURE). Reboot: stopped, then restart policy (`always`/`unless-stopped`) re-starts at daemon start (+SURE; `unless-stopped` remembers a manual stop in hostconfig). Re-provision: image + create.
- Q5: *which daemon*: `DOCKER_HOST`, `DOCKER_CONTEXT`, socket permissions, rootless Docker per user under `$XDG_RUNTIME_DIR` (+SURE) → user- and env-dependent. cwd matters for compose (project name defaults to the directory name, +SURE). Machine namespaces: irrelevant unless the socket is unreachable.
- Q6: `State` sub-fields: Status, Running, Paused, Restarting, OOMKilled, Dead, Pid, ExitCode, Error, StartedAt, FinishedAt, Health (+SURE); read-only, changed only through transitions. Addressed by name/ID; the template path is hierarchical.
- Q7: start/stop: process tree, cgroup, veth + bridge port, iptables/nft NAT rules for published ports (+SURE), embedded-DNS entries on user-defined networks, volume mounts, healthcheck timers, `docker events`. Changed by others: process exit, OOM, restart policy, dockerd restart, compose, host reboot, `docker system prune` (removes *exited* containers, +SURE), watchtower recreating on new image (+SURE).
- Q8: name, ID, compose project+service, cgroup, PID, containerd namespace `moby`. CRI's `k8s.io` namespace is a different world (+SURE).
- Q9: admin, compose, Portainer, watchtower, restart policy, systemd wrapper units. Name uniqueness is a *creation* lock (+SURE create fails on conflict); compose labels (`com.docker.compose.project`) mark ownership so compose touches only its own (+SURE). No other lock.
- Q10: container ID (random, never reused, +SURE); the process has a PID; the image a digest.
- Q11: persisted intent = compose file / unit / the restart policy stored *inside* Docker's own state (+SURE); live = status. `compose up` moves file→live; the restart policy moves hostconfig→live at daemon start. Disagree: `unless-stopped` after a manual stop; compose file changed but not applied; container outliving its compose file.

### 25. `docker inspect -f '{{.State.Health.Status}}' web` — `starting`/`healthy`/`unhealthy`
- Q1: exists only when a `HEALTHCHECK` is defined (image or run flags, +SURE); otherwise `.State.Health` is nil and the template prints `<no value>` (+SURE) → *unset rendered as a string*. Container missing → *absent*.
- Q2: `docker ps` STATUS suffix `(healthy)` (+SURE); `{{json .State.Health}}` (adds `FailingStreak`, `Log[]`); `docker events` `health_status:` (+SURE); compose `depends_on: condition: service_healthy` consumes it.
- Q3: yes: transitions; reset to `starting` with `FailingStreak` 0 on every container restart (+SURE). Container reissue as 24.
- Q4: dockerd memory (~SUSPECT also flushed to the container's on-disk state JSON); reset on container restart, daemon restart (without live-restore), reboot. Re-provision: new.
- Q5: as 24 (daemon selection).
- Q6: Status, FailingStreak, Log[] with Start/End/ExitCode/Output (+SURE); read-only. The *definition* (Test, Interval, Timeout, Retries, StartPeriod) lives in `Config.Healthcheck` (+SURE) and is writable only at create time.
- Q7: not directly writable. Changes cause `health_status` events; compose gating; Swarm reschedules unhealthy tasks, plain Docker does *not* restart them (+SURE the gap that `autoheal` fills). Changed by: the probe's exit code (0/1, +SURE), the application, the network, a container restart.
- Q8: container (as 24); image `HEALTHCHECK`; compose `healthcheck:` override.
- Q9: only dockerd writes the value. The *definition* has two authors (image author, deployer) layered by precedence run-flags > image (+SURE); no clobber, by design.
- Q10: none; container ID.
- Q11: live only: a measurement the daemon keeps; the persisted form is the *definition*, not the value. Time-varying with zero writes: a "sampled state" category (see E).

### 26. `kubectl get deploy web -o jsonpath='{.spec.replicas}'`
- Q1: field optional in the manifest; the API server defaults it to 1 on admission (+SURE) so a stored object never lacks it → *unset-materialized-as-default*. Deployment missing → NotFound, rc 1 → *absent*. Deleting a Deployment cascades to ReplicaSets/Pods unless `--cascade=orphan` (+SURE). Removing `replicas` from a manifest and re-`apply`ing resets it (client-side apply three-way merge, +SURE the HPA-vs-apply fight).
- Q2: `deployment.apps/web`, `deploy`, `deployments`; namespace-qualified, so `web` is not unique cluster-wide (+SURE); raw API path; `kubectl describe` ("N desired"); the `/scale` subresource (+SURE, what HPA uses). Distinct but related fields: `.status.replicas` (observed), `.status.readyReplicas`, the child ReplicaSet's `.spec.replicas` (derived, +SURE).
- Q3: yes: delete/recreate → same name, new `metadata.uid` (+SURE UUID); every write bumps `resourceVersion`; spec changes bump `generation` (+SURE).
- Q4: etcd via the API server (+SURE): remote; survives node reboots, apiserver restarts, node re-provision; lost only with cluster re-provision unless GitOps re-applies. Local persisted forms: the manifest in git/disk, Helm release Secrets in-cluster (+SURE Helm 3), Kustomize, ArgoCD Application.
- Q5: `KUBECONFIG`, `~/.kube/config`, `--context`, current-context namespace (+SURE env+user+file dependent); RBAC forbids rather than alters (+SURE). Not machine namespaces; cwd only via relative `--kubeconfig`.
- Q6: deep JSON tree; `spec.replicas` is a leaf; every leaf is separately writable via `kubectl patch` (JSON/merge/strategic, +SURE) or server-side apply, and the substrate records *which manager owns which field* in `managedFields` (+SURE). Hierarchical at two levels: group/version → namespace → kind → name → JSON path.
- Q7: writing: Deployment controller → ReplicaSet → Pods → scheduler → kubelet → EndpointSlices → Service routing; `.status` converges later; events. Changed by others: HPA overwrites it on its next reconcile (+SURE), `kubectl scale`, KEDA, ArgoCD self-heal reverting drift (+SURE), Helm upgrade, mutating admission (Kyverno) at write time.
- Q8: Deployment (ns+name); `/scale` subresource; Helm values; ReplicaSet (derived); manifest file; ArgoCD Application. Many tools, one store, *different field managers*.
- Q9: HPA vs apply vs Helm vs Argo. Substrate mechanisms: `resourceVersion` optimistic concurrency (+SURE), server-side-apply field-manager conflicts (+SURE, `--force-conflicts` to override). Practice: omit `replicas` from the manifest when HPA manages it (+SURE documented advice), Argo `ignoreDifferences`. The best-decolliding substrate in the seed.
- Q10: `metadata.uid` (per object instance), `resourceVersion` (per write), `generation` (per spec change) (+SURE all three). The field has none.
- Q11: *three* levels: manifest at rest (git) → `.spec` (declared intent, itself persisted in etcd) → `.status` (observed). apply/Argo moves manifest→spec; controllers move spec→status. `kubectl diff` compares the first two (+SURE); spec/status disagreement is normal mid-rollout.

### 27. `dig +short A web.example.com`
- Q1: the RRset is created/deleted at the authority while the zone persists. DNS natively distinguishes NODATA (name exists, no A records → NOERROR with empty answer) from NXDOMAIN (name absent) (+SURE) — *unset* vs *absent* both exist — and `+short` prints nothing for either (+SURE the distinction is lost by the read). SERVFAIL/timeout is a third, "unknown".
- Q2: many: trailing dot; case-insensitivity (+SURE); CNAME chains (`+short` prints the CNAME target then the A, +SURE); `search`-list completion applies to `getent hosts web` but not to `dig` unless `+search` (+SURE); `/etc/hosts` affects NSS reads, never `dig` (+SURE); `resolvectl query`, `host`, `nslookup`, `getent ahostsv4`. Different resolvers are different aliases that can answer differently (split-horizon).
- Q3: yes, and *simultaneously per observer* because of TTL caches (+SURE). The name is the only identity.
- Q4: authority: a remote zone (file, DB, provider). Caches: the recursive resolver, `systemd-resolved` (+SURE), nscd, application caches (the JVM's, +SURE infamous), negative caches (per SOA minimum, +SURE). Survives everything on *this* machine; local caches clear on reboot/resolver restart; this machine typically holds *no persisted form at all*.
- Q5: strongly: which resolver (`/etc/resolv.conf`, item 40; resolved's per-link DNS; `@server`), source IP (split-horizon/GeoDNS, +SURE), netns, `search`/`ndots` for short names, EDNS client subnet, DNSSEC state, *time* (TTL), env `LOCALDOMAIN`/`RES_OPTIONS`/`HOSTALIASES` for NSS-path reads (+SURE all three are honoured by the glibc resolver). Not the observing user as such.
- Q6: an RRset is a set of records (name, type, class, TTL, rdata); RFC 2136 updates add/delete individual RRs (+SURE); TTLs within an RRset must match (RFC 2181, +SURE). Addressed hierarchically by labels + type + class.
- Q7: writing at the authority: serial bump, NOTIFY, zone transfer; the *visible* value changes per observer as TTLs expire (+SURE). Nothing on this machine changes. Changed by others: DDNS, external-dns, Route53 health-check failover (+SURE), CDN steering; locally, `/etc/hosts` edits change NSS answers but not `dig`.
- Q8: the name; the resolver; NSS vs direct; the zone; the delegation. `dig` and `getent hosts` can disagree on one name (+SURE).
- Q9: at the authority: many writers; provider APIs use change batches/optimistic locking (+SURE Route53), BIND dynamic zones conflict with hand edits (`rndc freeze`, +SURE). On the reading side nobody writes the answer.
- Q10: none in the protocol; some providers expose record IDs (+SURE Cloudflare).
- Q11: persisted = zone (remote); live = each cache's copy; TTL expiry moves; disagreement lasts up to TTL (+ negative TTL). The only item with a built-in validity clock on measurements.

### 28. one line of `crontab -l`
- Q1: lines added/removed while the crontab persists; `crontab -r` removes the whole file → "no crontab for alice", rc 1 → *absent*; empty crontab is present-but-empty. A line has no native "unset": commenting it out is the convention (+SURE). Duplicates are legal and both run (+SURE). Identity of "a line" = its text or its position.
- Q2: `crontab -l -u alice` (root); the spool file `/var/spool/cron/crontabs/alice` (Debian) or `/var/spool/cron/alice` (RHEL) (+SURE both layouts) — same store; Debian's `crontab -e` prepends a header comment (+SURE). Schedule spellings: `@daily` ≡ `0 0 * * *`, `7` ≡ `0` for Sunday, `*/5` ≡ a list, day/month names (+SURE). *Sibling* homes with the same intent but different stores: `/etc/crontab`, `/etc/cron.d/*` (both with a user field), `/etc/cron.{hourly,…}` via `run-parts`/anacron, systemd timers, `at` (+SURE).
- Q3: yes: delete/re-add (same text = same thing by content); the file is replaced whole by `crontab -e`/`crontab file` (+SURE); position shifts.
- Q4: the spool file (disk); survives reboot; cron holds a parsed copy refreshed on spool mtime change (+SURE, scanned each minute); re-provision only if replayed.
- Q5: *the invoking user's* crontab (+SURE; `sudo crontab -l` is root's, the classic mistake); `cron.allow`/`cron.deny` gate who has one (+SURE); mount ns for the spool. The *job's* runtime env is minimal (`PATH=/usr/bin:/bin`, `SHELL=/bin/sh`, +SURE) and set by preceding `VAR=value` lines in the same file (+SURE) → a line's meaning depends on earlier lines: lines are not independent.
- Q6: fields minute/hour/dom/month/dow/command (+SURE), env-assignment lines, `MAILTO=` (+SURE); the line is written whole and the file is written whole. Addressed by user → line; not hierarchical.
- Q7: writing: cron reloads within a minute (+SURE); the job runs at the next match; mail on output. Changed by others: `crontab -r` typo for `-e` (+SURE famous), Ansible `cron` module rewriting its `#Ansible: name`-marked lines (+SURE), template pipes (`crontab -`), `userdel` (~SUSPECT leaves the spool unless the distro wrapper removes it).
- Q8: user; spool path; `crontab`; the sibling stores for the same intent.
- Q9: admin `-e` vs Ansible vs scripts: whole-file replace clobbers unless read-modify-write (`(crontab -l; echo …) | crontab -` idiom, +SURE); Ansible's marker comment is the decollision (+SURE); crontab(1) writes via temp file + rename (~SUSPECT). Tools prefer `/etc/cron.d/<file>` precisely because a *file* has identity.
- Q10: none; Ansible's `name:` marker *invents* one (+SURE); cron.d gives file-level identity.
- Q11: persisted = spool; live = cron's parsed copy (≤ 1 minute lag, +SURE). "Has it run" is a further, different state (syslog; anacron's `/var/spool/anacron/*` stamps, +SURE).

### 29. rule 3 of `ufw status numbered`
- Q1: rules added/deleted while ufw persists; numbering is *positional* and renumbers on every insert/delete above (+SURE, hence the confirmation prompt on `ufw delete N`). Fewer than 3 rules → nothing → *absent by number*; a rule missing by content → not listed → *absent*. When ufw is inactive, `ufw status` prints no rules at all although they persist in `user.rules` (+SURE); `ufw show added` lists them regardless (+SURE) → a "hidden but present" state.
- Q2: `ufw show added` (command syntax); `/etc/ufw/user.rules` and `user6.rules` (iptables-restore format with `### tuple ###` comments ufw parses, +SURE); the live iptables rule in chain `ufw-user-input` (+SURE); the same rule rendered by `nft list ruleset` under iptables-nft (+SURE on modern Ubuntu). Spellings: `allow 80/tcp` ≡ `allow http` (via `/etc/services`, +SURE) ≡ the long `allow in on eth0 to any port 80 proto tcp`; app profiles (`allow 'Nginx Full'`) take their ports from `/etc/ufw/applications.d/` (+SURE) so the rule's content depends on another file.
- Q3: yes: the number is reissued on any change above it (+SURE); a same-tuple rule re-added lands at a new position; `ufw reset` wipes and re-creates defaults (+SURE, with dated backups).
- Q4: persisted: `/etc/ufw/user.rules`, `user6.rules`, `ufw.conf` (`ENABLED=`), `/etc/default/ufw` (+SURE all); live: kernel netfilter, loaded by `ufw enable`/`ufw.service` at boot (+SURE). No daemon. The live rule dies with any `iptables -F`/`nft flush ruleset` while ufw still believes it exists (+SURE).
- Q5: netns for the kernel side; root-only (`ufw status` needs root, +SURE); mount ns for files. Not cwd/env.
- Q6: tuple fields: action, direction, interface, from, to, port, proto, log, comment (+SURE); not separately writable (ufw deletes and re-adds); position is writable via `insert`. Addressed by position (unstable) or by tuple (what `ufw delete allow 80` uses). v4 and v6 rules are listed separately with a `(v6)` suffix (+SURE).
- Q7: writing: `user.rules` rewritten and the specific iptables rule inserted/deleted incrementally when enabled (+SURE; `ufw reload` reloads all); packet path changes at once; established connections survive via `before.rules` conntrack accept (+SURE). Changed by others: Docker publishes ports through its own chains *ahead of* ufw's, so the ufw rule stays present but ineffective (+SURE the famous hole); `iptables -F`; firewalld (must not coexist, +SURE); fail2ban inserts its own chains and coexists (+SURE); `ufw reset`; ufw package upgrades touching `before.rules` conffiles.
- Q8: ufw (position or tuple), the file, the iptables chain, the nft ruleset. The kernel rule and the ufw rule are one thing only while in sync (~SUSPECT ufw adds no `-m comment` to tie them).
- Q9: ufw vs iptables vs Docker vs fail2ban vs firewalld: the xtables lock (`-w`) serializes *commands*, not ownership (+SURE); ownership is by chain-name convention (`ufw-*`, `DOCKER*`, `f2b-*`, +SURE). Docker's conflict is structural (chain order), not textual.
- Q10: none for the intent; the rendered kernel rule has a chain position and, under nft, a *handle* stable until flush (+SURE `nft -a list`). The substrate gives the rendering an id, ufw gives the intent none.
- Q11: persisted (`user.rules`) vs live (kernel); `enable`/`reload`/boot moves; disagree after `iptables -F`, Docker restarts, or edits to `user.rules` without `reload`. `ufw status` reports the *persisted* form plus an active check (~SUSPECT the exact check), so the seed's read is of the file, not the kernel.

### 30. `hostname`, and `/etc/hostname`
Two items: the kernel UTS nodename (live) and the static-hostname file (persisted). Relation: early boot copies file→kernel (+SURE); `hostnamectl set-hostname` writes both (+SURE); DHCP/cloud-init set a *transient* name that overrides the static one in the kernel by design (+SURE `hostnamed` static/transient/pretty model).
- Q1: kernel: always has a value (initial `(none)`, +SURE), never absent, never unset. File: can be missing → *absent*; `hostnamectl` then reports an empty static hostname → *unset* at the hostnamed layer, with fallback `localhost`.
- Q2: `uname -n`, `/proc/sys/kernel/hostname` (`sysctl kernel.hostname`, +SURE), `hostnamectl` (three names: static, transient, pretty, +SURE), `$HOSTNAME` (a copy bash takes at startup, +SURE), `hostname -f`/`-s` (derived via NSS/`/etc/hosts`/DNS, +SURE), legacy `/etc/HOSTNAME`, `/etc/sysconfig/network`, `/etc/machine-info` for pretty (+SURE), cloud metadata, DHCP option 12.
- Q3: n/a for the kernel slot (value changes); the file is rewritten.
- Q4: kernel `utsname`, per UTS namespace (+SURE); lost at reboot, restored from the file by systemd, then possibly overridden transiently. File: disk. Re-provision: cloud-init sets from metadata unless `preserve_hostname: true` (+SURE).
- Q5: UTS namespace, strongly (`docker run -h`, +SURE); `$HOSTNAME` is a stale copy; `-f` depends on `/etc/hosts`, NSS, DNS. Not user/cwd.
- Q6: kernel: one string (short or FQDN by convention only). hostnamed: static/transient/pretty separately writable (+SURE `--static`/`--transient`/`--pretty`). File: one line. Not hierarchical.
- Q7: writing the kernel name: new shells' `$HOSTNAME`, prompts, syslog tags, `sudo` warns "unable to resolve host" when `/etc/hosts` lacks it (+SURE), Avahi re-announces `<name>.local`, LLMNR, Postfix `myhostname` default, D-Bus `PropertiesChanged`. Writing the file alone: nothing until reboot. Changed by others: cloud-init (file and kernel), DHCP via NM `hostname-mode` (transient, +SURE), `hostnamectl`, config management.
- Q8: kernel, file, D-Bus `org.freedesktop.hostname1`, sysctl, env, cloud metadata, DHCP.
- Q9: cloud-init vs DHCP vs admin: opt-outs are `preserve_hostname` and NM `hostname-mode=none` (+SURE); hostnamed's design assigns *transient* to DHCP-like writers and *static* to admins, kernel = transient if set else static (+SURE) — decollision by *kind of writer*.
- Q10: none; `/etc/machine-id` is the machine's identity, distinct from its name (+SURE).
- Q11: persisted file vs live kernel; boot moves; `set-hostname` writes both; they disagree after `hostname foo` alone or after a transient override, *by design*.

### 31. one rule of `nft list ruleset`, and the corresponding line of `/etc/nftables.conf`
- Q1: rules added/deleted while table and chain persist (`nft add rule`, `nft delete rule … handle N`, +SURE); deleting the table deletes everything under it; `nft flush ruleset` removes all tables (+SURE). Missing rule → not listed → *absent*, no slot. File line: *absent*.
- Q2: `nft -a list ruleset` (with handles, +SURE), `nft -j` JSON, `nft list chain <family> <table> <chain>`; `iptables-nft -S` renders rules in the tables iptables-nft created (+SURE) and cannot see native tables of other names (+SURE); `iptables-legacy` is a separate kernel subsystem that can be active simultaneously (+SURE). Listing prints a *canonical* form that may differ textually from what the file says (~SUSPECT the exact normalizations; sets, service names, address forms).
- Q3: yes: a re-added rule gets a new handle; positional indexes shift (+SURE); handles are per-table monotonic (~SUSPECT not reused within the table's life).
- Q4: live: kernel tables, per netns (+SURE); lost at reboot; persisted: `/etc/nftables.conf` and its `include`s, loaded by `nftables.service` with `nft -f` (+SURE); firewalld keeps its own persisted XML and its own `firewalld` table (+SURE). No daemon for plain nft. Re-provision: the file.
- Q5: netns (+SURE); listing needs CAP_NET_ADMIN (+SURE); mount ns for the file. Not cwd/env.
- Q6: a rule = expressions + verdict, optional `comment` and `counter` (+SURE); not separately writable (`replace rule … handle N` swaps the whole rule, +SURE). Addressed hierarchically: family → table → chain → rule (handle or index) (+SURE genuinely path-like); the file nests with braces the same way.
- Q7: writing: atomic per transaction (+SURE); counters reset on replace. Loading a file that begins with `flush ruleset` (the Debian default) *replaces every table*, including Docker's, firewalld's, libvirt's (+SURE). Changed by others: Docker via iptables-nft (`ip filter`/`ip nat`, +SURE), firewalld, libvirt, fail2ban's nft backend, ufw via iptables-nft, kube-proxy nftables mode (own table, +SURE), `wg-quick` (~SUSPECT own table).
- Q8: kernel path (family/table/chain/handle), the file, iptables-nft (compatible tables only), firewalld, each tool's own table. nft's convention is one table per application (+SURE).
- Q9: coexistence by table ownership (+SURE); the kernel has no lock but atomic transactions; the *file* is the clobber point (`flush ruleset` vs the polite `flush table` form, +SURE).
- Q10: rule handle (per-table, +SURE); tables/chains have names and handles; the file line has none. Good live identity, no persisted identity.
- Q11: persisted vs live; `nft -f`/boot moves; `nft list ruleset > /etc/nftables.conf` moves *live→persisted* (+SURE routine practice, drops comments/includes) — the only seed item where that direction is tool-supported and common. Disagree whenever another tool touched the kernel or the file changed unloaded.

### 32. one entry of `swapon --show`, and the corresponding `/etc/fstab` line
- Q1: activated/deactivated (`swapon`/`swapoff`) while device and fstab persist; missing from `--show` → *absent* (no header, no line, +SURE). fstab line with `noauto` = persisted-but-not-live (the classic). An active swapfile can be unlinked and stays active until swapoff (+SURE unlink semantics).
- Q2: `/proc/swaps` (+SURE same data); `lsblk` `[SWAP]` mountpoint (+SURE); `blkid` `TYPE="swap"`; `systemctl list-units -t swap` (`dev-sda2.swap`, generated from fstab, +SURE); `free`/`/proc/meminfo` totals. Device spellings: `/dev/sda2`, `UUID=`, `LABEL=`, `PARTUUID=`, `/dev/disk/by-id/…` — one device, five names (+SURE), and `swapon --show` prints the resolved `/dev/…` form only. `zram0` has no persistent backing (+SURE). `systemd-gpt-auto-generator` activates GPT swap partitions with no fstab line at all (+SURE).
- Q3: yes: off/on; `/dev/sdX` names reshuffle across boots (+SURE, the reason UUIDs exist); `mkswap` writes a *new* UUID unless `-U` (+SURE), so re-signing breaks a UUID-based line.
- Q4: live: kernel swap table; lost at reboot. Persisted: fstab (+SURE), `.swap` units, `/etc/crypttab` for encrypted swap (+SURE), the device's own `mkswap` signature (self-description), `resume=` cmdline / initramfs config for hibernation (+SURE) — four persisted places naming one swap. Re-provision: partition + mkswap + fstab.
- Q5: not namespaced (+SURE no swap namespace); containers see host swap totals via `/proc/meminfo` unless lxcfs (+SURE). Readable by all. Not user/cwd/env.
- Q6: priority (settable at activation), discard flags, size, used (live); fstab has six fields. Addressed by device path/UUID; flat.
- Q7: `swapon`: memory reclaim behaviour, hibernation feasibility, `.swap` unit becomes active. `swapoff`: pages pulled back, may fail ENOMEM (+SURE). Editing fstab alone: nothing until `swapon -a`/reboot/`daemon-reload` (+SURE the generator re-runs on reload). Changed by others: gpt-auto (+SURE), zram-generator, cloud-init `swap:` (creates file and fstab line, +SURE), `dphys-swapfile`, `mkswap` re-signing.
- Q8: device path, UUID/LABEL, fstab line, `.swap` unit, `/proc/swaps`, `lsblk`, the on-device signature, crypttab, `resume=`.
- Q9: cloud-init vs admin vs gpt-auto vs zram-generator; fstab has no lock (Ansible `mount` matches by mountpoint/device, +SURE); gpt-auto is opt-out via `systemd.gpt_auto=0` (~SUSPECT it also skips devices listed in fstab).
- Q10: swap UUID from the `mkswap` header (+SURE, distinct from `PARTUUID`), rewritten by `mkswap`; live `major:minor`.
- Q11: persisted (fstab/unit/signature) vs live (`/proc/swaps`); boot/`swapon -a` moves; disagree with `noauto`, a manual `swapoff`, or a UUID mismatch (which without `nofail` delays boot 90 s, +SURE the classic).

### 33. `openssl x509 -enddate` and `-serial` of `/etc/ssl/certs/x.pem`
Two derived reads of one file's bytes; both fields are mandatory X.509 members (+SURE), so they co-vary and never exist separately.
- Q1: neither field can be absent from a certificate → never unset; file missing → *absent*; file present but not a certificate → "unable to load certificate" → *present-but-unparseable*, a third state. A PEM bundle: `x509` reads the *first* cert only (+SURE). `/etc/ssl/certs/*` are often `update-ca-certificates` hash symlinks (+SURE), so the path may be a link.
- Q2: `-dates`, `-text`, `-fingerprint`; `certtool -i`, `keytool`, `step certificate inspect`; the same bytes in DER, in PKCS#12, in a `fullchain.pem`; certbot's `live/<name>/cert.pem` symlink into `archive/<name>/certN.pem` (+SURE); deployed copies elsewhere; and the cert *as served* (`openssl s_client`), the live form. Serial formatting varies by tool: uppercase hex, colon-separated, decimal (+SURE the mess).
- Q3: *renewal* = a new certificate (new serial, new dates, same subject/SANs, maybe same key) at the same path (+SURE certbot flips the symlink). CA serials are never reused per issuer (+SURE CA/B rules).
- Q4: file on disk; live copy in the server process (nginx loads at start/reload, +SURE); remotely: the CA's issuance record, CT logs (+SURE for public CAs), and *revocation state* (CRL/OCSP), which the file never contains. Survives reboot; re-provision: re-issue or restore from secrets.
- Q5: mount ns/chroot; readability. The *validity judgement* depends on the observer's clock; the field does not (+SURE).
- Q6: many named fields (subject, issuer, SANs, serial, notBefore/notAfter, KU/EKU, SKI/AKI); *not* separately writable: the signature welds them (+SURE) — the only seed item whose sub-parts are cryptographically bound. Addressed by path; bundles are ordered.
- Q7: replacing the file: nothing until the consumer reloads (+SURE); for CA certs `update-ca-certificates`/`update-ca-trust` rebuild bundles and hash links (+SURE). Changed by others: certbot's timer with `--deploy-hook` (+SURE the only handshake to consumers), `acme.sh`, `ca-certificates` upgrades, cert-manager (k8s Secrets), config management.
- Q8: path; certbot lineage; the server's `ssl_certificate` directive; the served endpoint; the CA/CT; the trust store.
- Q9: certbot owns `/etc/letsencrypt`; deploy hooks; no lock. The CA never writes your disk.
- Q10: strong and layered: issuer+serial (per-cert), SHA-256 fingerprint (per-bytes), SPKI hash (per-*key*, survives renewal on the same key, +SURE), Subject Key Identifier — the richest identity in the seed.
- Q11: persisted (file) vs live (served); reload moves; disagree after renewal until reload (+SURE the common incident); a third form (CA record, revocation) is remote and never reflected in the file.

### 34. `getenforce`
- Q1: always answers, printing `Disabled` when SELinux is off or absent (+SURE) → never absent, never unset. The persisted `SELINUX=` line can be missing from `/etc/selinux/config` (--WONDER what the init code then assumes). Kernel cmdline `selinux=0`/`enforcing=0` override (+SURE both).
- Q2: `sestatus` (prints *both* "Current mode" and "Mode from config file", +SURE), `/sys/fs/selinux/enforce` (+SURE), the config file, `selinuxenabled` (rc), the cmdline. Finer: per-domain permissive (`semanage permissive -a httpd_t`, +SURE) makes "permissive for httpd" true while `getenforce` says Enforcing.
- Q3: n/a; a mode that toggles.
- Q4: kernel LSM state, global, not namespaced (+SURE containers cannot `setenforce`); lost at reboot, restored from config or cmdline early (+SURE). Disabled↔enabled needs a reboot plus relabel (`/.autorelabel`, +SURE); runtime disable was removed from recent kernels in favour of `selinux=0` (+SURE).
- Q5: not namespaced; readable by all; not cwd/env.
- Q6: mode plus `SELINUXTYPE=` (+SURE); per-domain permissive is a separate store (policy modules). Not hierarchical.
- Q7: `setenforce 0`: denials logged, not enforced (+SURE); `1`: immediate. Writing the config: nothing until reboot. Changed by others: Ansible `selinux` module writes both (+SURE), installers, grub edits. A policy can lock out `setenforce` (--WONDER the exact boolean name).
- Q8: kernel `/sys/fs/selinux`, config file, cmdline, `sestatus`; the *policy* is a separate thing.
- Q9: admin, config management, installer; whole-file rewrite; no lock.
- Q10: none.
- Q11: persisted (config, cmdline) vs live; boot moves; `sestatus` shows both side by side (+SURE the only seed read that reports both forms in one output); disagree after `setenforce` alone or a cmdline override.

### 35. `timedatectl show -p Timezone`, and `/etc/localtime`
- Q1: `/etc/localtime` can be missing → glibc uses UTC (+SURE) → *absent* at the file layer, reported as empty/`n/a` by timedatectl (~SUSPECT the exact rendering). Debian's `/etc/timezone` is a second persisted spelling (+SURE) that can disagree. Created by `timedatectl set-timezone`; not deleted by it.
- Q2: `/etc/localtime` is a symlink to `/usr/share/zoneinfo/<Area>/<City>` (+SURE) and timedatectl *derives the name from the link target*; a *copied* file yields no name (+SURE). `/etc/timezone`; `TZ` env per process (+SURE); `date +%Z` (abbreviations are ambiguous, `CST` is three zones, +SURE); D-Bus `timedate1.Timezone`; the TZif footer's POSIX string (+SURE v2+). Name aliases in tzdata: `Asia/Calcutta` ≡ `Asia/Kolkata`, `US/Pacific` ≡ `America/Los_Angeles`, `Etc/UTC` ≡ `UTC` ≡ `Zulu` (+SURE backward links).
- Q3: n/a; the link is replaced.
- Q4: the symlink (disk); no kernel form (the kernel keeps UTC, +SURE); each *process* holds its own cached zone (glibc re-checks on `tzset`, ~SUSPECT how eagerly; cron, JVMs, PostgreSQL need restarts, +SURE for several). `/etc/adjtime` (RTC local vs UTC) is a sibling (+SURE). Survives reboot; cloud images default UTC.
- Q5: `TZ` env overrides per process (+SURE); containers usually lack the link → UTC (+SURE); not user except via env; timedatectl talks to the host's `timedated`.
- Q6: hierarchical name `Area/City`; the zone file holds rules; not separately writable. Sibling properties `LocalRTC`, `NTP`, `NTPSynchronized` (+SURE).
- Q7: `set-timezone`: link replaced, D-Bus signal, RTC adjusted if local-RTC (+SURE); `/etc/timezone` update is distro-dependent (~SUSPECT). Running processes: new results only for those that re-read; Vixie cron does not (+SURE). Changed by others: `dpkg-reconfigure tzdata` (writes both, +SURE), cloud-init `timezone:`, and *tzdata package upgrades change the rules behind an unchanged name* (+SURE DST changes) — the value changes with no write to the link.
- Q8: symlink, `/etc/timezone`, D-Bus, env, the tzdata package.
- Q9: timedated vs `dpkg-reconfigure` vs cloud-init vs `ln -sf`; no lock; two persisted spellings; timedatectl reports the symlink (+SURE).
- Q10: none for the setting; the rules carry a tzdata version (`tzdata.zi` header, +SURE).
- Q11: persisted (link, `/etc/timezone`) vs live (each process's cached zone); no single live form; disagree per process until restart; plus semantic drift under a fixed name via tzdata.

### 36. `update-alternatives --query editor` (the current link)
Mechanism: `/usr/bin/editor` → `/etc/alternatives/editor` → `/usr/bin/vim.basic`; admin file `/var/lib/dpkg/alternatives/editor` (+SURE; RHEL uses `/var/lib/alternatives`).
- Q1: the group exists from the first `--install` to the last `--remove` (+SURE, done by package scripts); missing group → "no alternatives for editor" → *absent* (~SUSPECT rc 2). Removing the currently selected alternative flips the group back to auto (+SURE) — the closest thing to "unset".
- Q2: `readlink /etc/alternatives/editor`, `readlink -f /usr/bin/editor`, `--display`, `--get-selections`, the admin file; RHEL `alternatives` (same design, +SURE). Debian's `sensible-editor` chain (`SELECTED_EDITOR` → `VISUAL` → `EDITOR` → `editor`, +SURE) ends here, linking to items 9/10/37.
- Q3: yes: `--set`/`--auto`/`--config`; package churn recreates; slave links move with the master (+SURE).
- Q4: admin file (mode auto/manual, candidates, priorities, +SURE) plus symlinks; disk; survives reboot; no daemon; packages re-register on re-provision, *manual selections do not survive* unless replayed.
- Q5: `--altdir`/`--admindir` (+SURE), chroot; readable by all. *Use* of `editor` goes through `$PATH` (item 38).
- Q6: mode, current value, candidate list with priorities, slaves (+SURE); `--set` writes mode+value; `--install` adds candidates. Master → slaves is a small hierarchy; addressed by group name or link path.
- Q7: `--set`: master and slave symlinks re-pointed, mode → manual (+SURE). Changed by others: a package `--install` with higher priority changes the value only in auto mode (+SURE the design); `--remove` of the current → auto; package removal.
- Q8: group name, link path, `/etc/alternatives` link, admin file, the registering package.
- Q9: dpkg scripts vs admin: the auto/manual flag *is* the decollision (manual = packages keep off, +SURE). Ansible's module sets manual. No lock.
- Q10: none; priorities are not ids.
- Q11: two on-disk forms: admin file vs actual symlinks; they disagree after a hand `ln -sf` (+SURE), and `--query`/`--display` warn about dangling or non-symlink states (~SUSPECT exact reporting). No live process form.

### 37. `printenv HOME` inside a running process
- Q1: set/unset per process (`export`, `unset`, `env -u`, +SURE); missing → rc 1, nothing printed → *absent* (the env is a list, no slot). Empty and unset are distinct (`${HOME:-}` vs `${HOME-}`, +SURE). Many programs fall back to passwd field 6 when unset (bash's `~` does, +SURE); glibc itself does not.
- Q2: `$HOME`, `~` (uses `HOME` if set, else passwd, +SURE); `/proc/<pid>/environ` shows the *exec-time* environment, not later `setenv` changes (+SURE, unless the process rewrites that region) — a key trap; passwd field 6 (item 17's sibling, usually equal, not the same thing); `systemctl show-environment` for the user manager (+SURE); `sudo` keeps or resets it per `always_set_home`/`env_keep` (+SURE classic).
- Q3: yes: unset/re-export; each new process starts from a fresh copy.
- Q4: the process's own memory (+SURE); copied to children at fork/exec, never back-propagated (+SURE); dies with the process. Persisted sources are many: passwd, `/etc/environment` (`pam_env`), `/etc/profile.d`, `~/.profile`/`~/.bashrc`, systemd `Environment=`/`EnvironmentFile=`, `environment.d` (+SURE all), composed differently per login path (login/non-login/interactive/`ssh cmd`/cron/sudo, +SURE the matrix).
- Q5: it *is* observer state: which process, which login path, which user. Not namespaces; cwd only via `$PWD`.
- Q6: a string; the env set is a flat list, duplicates possible at `execve` (~SUSPECT `getenv` returns the first). Not hierarchical.
- Q7: writing: tilde expansion, `XDG_*` defaults (`$HOME/.config`, +SURE), git/ssh config lookup, children inherit. Nothing on disk. Changed by others: nothing can alter another process's env without ptrace (+SURE); new processes get new envs.
- Q8: the process (pid); `/proc/pid/environ` (snapshot); the shell's table; the login path's files; passwd.
- Q9: no cross-process clobbering is possible, only *divergence* (+SURE); within one shell, script vs user. On the persisted side `profile.d` drop-ins are the decollision (+SURE).
- Q10: none; the process has pid + start time (`/proc/pid/stat` field 22, +SURE the stable pair; pids are reused).
- Q11: persisted (profile files etc.) vs live (process env); moved *once* at process start; diverge freely afterwards; `/proc/pid/environ` is a third, snapshot form. No tool reads another process's live env (+SURE, gdb aside).

### 38. one entry of `PATH` in a running process
A *component* of a list-valued variable; everything in item 37 applies to the variable, this block covers what the entry adds.
- Q1: entries come and go only by rewriting the whole string; a missing entry is *absent* (nothing enumerates gaps); duplicates are common and harmless (+SURE re-sourced profiles); an *empty* entry (`::`, leading/trailing `:`) means cwd (+SURE POSIX). `PATH` wholly unset → `execvp` uses `confstr(_CS_PATH)` (+SURE glibc default `/bin:/usr/bin`) → *unset-with-default* at the variable level.
- Q2: `/usr/bin` vs `/usr/bin/` vs `/bin` (a symlink to `/usr/bin` on merged-usr, +SURE Debian ≥ 12); literal `~` is *not* expanded by `execvp`, only by the shell at assignment (+SURE); symlinked dirs; relative entries. Consumers are derived views: `command -v`, `type`, `which`, and bash's *hash table*, a per-shell cache that goes stale until `hash -r` (+SURE). Persisted spellings: `login.defs` `ENV_PATH`/`ENV_SUPATH` (+SURE), `/etc/environment`, sudoers `secure_path` (+SURE sudo replaces PATH), cron's hard default (item 28), systemd's compiled default for services (+SURE, profiles are not consulted).
- Q3: yes: reorder/duplicate; the same entry string can denote a different directory after a mount or symlink change.
- Q4: as 37, plus the bash lookup cache. The persisted matrix is larger: profiles, `login.defs`, sudoers, systemd `DefaultEnvironment=`, `/etc/environment`.
- Q5: as 37, and *order* is part of the value; `sudo`, cron, systemd, and non-interactive ssh each substitute their own (+SURE); `direnv` makes it cwd-dependent (+SURE); mount ns decides what an entry points at.
- Q6: ordered entries separated by `:`; separately meaningful, written whole; no native add/remove (shell idioms; `pathmunge` on RHEL, +SURE). The entry has no name: addressed by content or position. Values are paths; the variable is flat.
- Q7: writing: subsequent lookups in this process and children change; the hash cache may still hit old locations (+SURE). Changed by others: every tool's installer appends to `.bashrc`/`.profile` (cargo, nvm, pyenv, conda, +SURE), `direnv`, `mise` shims, `sudo`, `env -i`.
- Q8: the process; consumers; profile files; sudoers; `login.defs`; systemd config.
- Q9: many appenders; the idiom "add if not already present" (`case ":$PATH:" in …`) is the decollision (+SURE common); order conflicts remain unresolved; `profile.d` on the system side.
- Q10: none; the directory it names has an inode and a mount.
- Q11: as 37, plus the hash cache as a *derived live* form that can disagree with both (+SURE).

### 39. one line of `~/.ssh/authorized_keys`
- Q1: lines appended/removed while the file persists (`ssh-copy-id` appends, +SURE); file deletable (sshd then consults the other `AuthorizedKeysFile` entries, default also `authorized_keys2`, +SURE); missing line → *absent*; a `#` comment is the disable convention. Per-line options (`from=`, `command=`, `restrict`, +SURE) are prefixes.
- Q2: the key's identity is the blob (type + base64); the trailing comment is ignored by sshd (+SURE), so `user@host` is a hint, not a name. Same key as a fingerprint (`ssh-keygen -lf`, SHA256 vs MD5 renderings, +SURE), in the client's `.pub`, in `ssh-add -L`, in GitHub's `/<user>.keys`, in cloud metadata (cloud-init `ssh_authorized_keys`, +SURE), in `AuthorizedKeysCommand` output (`sss_ssh_authorizedkeys`, +SURE), and *the same access* via a CA certificate (`TrustedUserCAKeys`, a different mechanism, +SURE). The same line under another user's home is a different thing.
- Q3: yes: re-added same blob = same key by content; *rotation* is a new blob with no linkage except the comment (+SURE); `ssh-import-id` appends with a marker comment (+SURE).
- Q4: the file in the user's home (disk, per user, per machine unless NFS); sshd reads it *per authentication attempt* with no cache, as the target user, after `StrictModes` permission checks (+SURE). Re-provision: cloud-init from metadata (+SURE), config management, key import.
- Q5: per user; sshd uses the passwd home, not `$HOME` (+SURE); *effectiveness* is gated by sibling state: permissions of home/`.ssh`/file (item 5), SELinux `ssh_home_t` on RHEL (+SURE), sshd config (`PubkeyAuthentication`, `AuthorizedKeysFile`, `PermitRootLogin`). Mount ns. Not cwd/env.
- Q6: options list, key type, blob, comment: separable fields written as one line (+SURE). The file is a list; addressed by user → file → line (by content). Not hierarchical.
- Q7: writing: the next auth attempt sees it (+SURE, no reload); nothing else. Changed by others: `ssh-copy-id`, cloud-init first boot, `ssh-import-id`, Ansible `authorized_key` (with `exclusive: yes` it clobbers everything else, +SURE), home restores, `userdel -r`, `chmod`/`restorecon` on the gates.
- Q8: user → home → file → line; alternate `AuthorizedKeysFile` paths; `AuthorizedKeysCommand`; CA certs; cloud metadata. Same *grant* through several mechanisms; same *key* through several paths.
- Q9: appenders coexist by "add if blob not present" (+SURE Ansible's module does this); `exclusive` and cloud-init rewrites are the clobber risks; marker comments claim ownership; no lock.
- Q10: the key blob / SHA256 fingerprint (+SURE canonical); the *grant line* has none beyond content.
- Q11: persisted only; read per auth; but a line can be present-and-inert because of sibling gates.

### 40. one `nameserver` line of `/etc/resolv.conf`
- Q1: lines added/removed; glibc uses at most three (`MAXNS`, +SURE), extras are *present-but-inert*; no `nameserver` lines, or no file at all → glibc defaults to the local host (+SURE resolv.conf(5)) → *unset-with-default* and *absent-with-default* both resolve to something.
- Q2: the file is usually a *symlink* (+SURE): to `/run/systemd/resolve/stub-resolv.conf` (`127.0.0.53`, the stub) or `…/resolve/resolv.conf` (real upstreams), or NM-generated, or openresolv's `/run/resolvconf/resolv.conf` (+SURE). So the line is often an *indirection*; the real servers live in `resolvectl status` per link (+SURE), NM `ipv4.dns`, DHCP option 6, `resolved.conf` `DNS=`. Each consumer parses the file itself: glibc, musl (queries all servers in parallel, +SURE), Go's pure resolver, `dig` (+SURE own parser), browsers doing DoH ignore it (+SURE) → one line, several semantics. `RES_OPTIONS`/`LOCALDOMAIN` env override sibling lines (+SURE).
- Q3: yes, constantly: the manager rewrites the file on DHCP renew, link changes, VPN up/down (+SURE); manual edits are clobbered (the famous complaint).
- Q4: the file (disk, or tmpfs via the symlink into `/run`, +SURE, then regenerated at boot). Persisted intent: NM profiles, netplan, `resolved.conf`, dhclient `supersede`/`prepend` (+SURE), openresolv `head`/`base`/`tail` (+SURE). glibc ≥ 2.26 re-reads on mtime change (+SURE); older glibc cached for the process lifetime. Re-provision: DHCP.
- Q5: netns (`/etc/netns/<name>/resolv.conf` bind-mounted by `ip netns exec`, +SURE); containers get a Docker-managed copy pointing at `127.0.0.11` on user-defined networks (+SURE); env (`RES_OPTIONS`, `LOCALDOMAIN`); the consumer's resolver library. Not user/cwd.
- Q6: keyword + address; siblings `search`, `domain`, `options`, `sortlist`; order matters (first preferred unless `rotate`, +SURE). Written whole; not hierarchical.
- Q7: writing: glibc processes pick it up on next lookup (+SURE, modern glibc), `dig` at once, resolved *ignores* it (it produces it, +SURE); NM overwrites it on the next event unless `dns=none`/`rc-manager=unmanaged` (+SURE) or `chattr +i` (the folk veto, +SURE). Changed by others: NM, resolved, resolvconf/openresolv, dhclient hooks, `wg-quick` (+SURE pushes DNS via resolvconf), VPN scripts, Docker, cloud-init, `netplan apply`.
- Q8: path and symlink target; the manager's config; per-link DNS; the lease; consumers (glibc, musl, Go, dig, browsers).
- Q9: NM vs resolved vs resolvconf vs dhclient vs admin: the ecosystem's answer is a single owner, with openresolv as the *merger* of per-interface deposits under `/run/resolvconf/interface/` (+SURE its design). No lock.
- Q10: none; the inode is regenerated.
- Q11: three levels: manager intent (persisted) → the file (a *rendering* that reads as authoritative) → each process's parsed copy and resolved's per-link state. `resolvectl status` is the real live read when resolved is in use (+SURE).

### 41. `iptables -S INPUT` (the chain's default policy)
- Q1: built-in chains always have a policy, default ACCEPT (+SURE); cannot be deleted; user chains have none (`-P` errors, +SURE). Under iptables-nft the `filter` table is created lazily by the *first* `iptables` invocation (+SURE), and iptables-legacy autoloads modules the same way → *the read creates the thing*. Never absent from the tool's view; absent in the kernel until first touch.
- Q2: `iptables -L INPUT` header, `iptables-save` (`:INPUT ACCEPT [0:0]`, +SURE), `nft list chain ip filter INPUT` (`policy accept` on the base chain under iptables-nft, +SURE). Deepest trap: `iptables` is itself an *alternative* (item 36) between `iptables-nft` and `iptables-legacy` (+SURE on Debian), and the two address *different kernel objects* that can both be active with packets traversing both (+SURE); `iptables -V` tells which. `ip6tables` is a separate table. ufw sets this policy from `/etc/default/ufw` (+SURE); Docker sets FORWARD's, not INPUT's (+SURE).
- Q3: n/a for the slot; the table can vanish (`nft flush ruleset`) and be recreated with ACCEPT by the next `iptables` call (+SURE).
- Q4: kernel netfilter, per netns (+SURE); lost at reboot; persisted by `netfilter-persistent` (`/etc/iptables/rules.v4`, +SURE), ufw's config, firewalld XML, RHEL `/etc/sysconfig/iptables` (+SURE legacy), or `nftables.conf`. No daemon.
- Q5: netns; root to read (+SURE); *which binary* the observer runs (legacy vs nft) selects the kernel object (+SURE). Not cwd/env.
- Q6: none (a verdict); addressed table → chain → policy (+SURE `-t filter` default); `filter/INPUT` ≡ `ip filter INPUT` in nft.
- Q7: `-P INPUT DROP` takes effect immediately for unmatched packets, including the current SSH session without an ESTABLISHED rule (+SURE the classic lockout). Changed by others: `ufw enable`/`disable` (+SURE `disable` restores ACCEPT), `iptables-restore`, firewalld start, `nft flush ruleset` under iptables-nft (table gone, then ACCEPT on recreation, +SURE).
- Q8: table+chain via either iptables binary; the nft path; the persisted file; ufw's config. Two binaries, possibly two kernel objects, one name.
- Q9: xtables lock (`-w`) serializes *commands* only (+SURE); no ownership of the policy; last `-P` wins; convention: firewall frameworks own policies, others own chains.
- Q10: none; nft base chains have handles (+SURE).
- Q11: persisted vs live; `iptables-restore` moves; `iptables-save` moves live→persisted (+SURE routine, as with nft). Disagree after any manual `-P` or another framework starting.

### 42. one column's value in one SQL row; and that column in the table's schema
Two items.
**(a) the cell**
- Q1: rows are inserted/deleted while the table persists; a cell cannot be deleted, only set to `NULL` (+SURE) — SQL natively has *absent* (no row), *unset* (NULL, with `DEFAULT` applied at insert when omitted, +SURE), and present. Without a primary key a row is identifiable only by content (+SURE; Postgres `ctid` is physical and unstable).
- Q2: row by PK, by any unique key, by `ctid`/`rowid` (SQLite `rowid` aliases `INTEGER PRIMARY KEY`, +SURE), through updatable views (+SURE), through `search_path` resolution of an unqualified name (session-dependent, +SURE), case folding of unquoted identifiers (+SURE Postgres), replicas and FDWs (a lagging copy of the same logical row, +SURE), and *across time by observer*: two transactions see different versions of one row under MVCC (+SURE).
- Q3: yes: delete/insert under the same natural key; `SERIAL` surrogates are not reused (sequences do not roll back, +SURE); `TRUNCATE`; in-place `UPDATE` (physically a new tuple version, +SURE).
- Q4: table pages + WAL on disk, buffer cache in memory, durable at COMMIT (+SURE; `synchronous_commit=off` can lose recent commits on crash, +SURE); survives reboot and DB restart; often *remote*; re-provision only via backups/migrations/seeds.
- Q5: the observer's transaction/snapshot (+SURE), role (row-level security filters rows per role, +SURE), session settings that alter rendering (`timezone` for `timestamptz`, `DateStyle`, `client_encoding`, +SURE), primary vs replica (+SURE), env (`PGHOST`, `PGOPTIONS`, `.pgpass`, +SURE); cwd only for SQLite's file path (+SURE). Not machine namespaces.
- Q6: cells may hold structured values (JSON/JSONB, arrays, composites, +SURE) with paths that are separately addressable and updatable at the SQL level (`jsonb_set`), though the tuple is rewritten physically (+SURE). Addressed db → schema → table → row-by-key → column → path: the most hierarchical item in the seed.
- Q7: writing: triggers (+SURE), constraint checks with FK cascades into *other* tables (+SURE), index maintenance, WAL, replication (later), CDC, materialized views go stale until `REFRESH` (+SURE), generated columns recompute (+SURE), visibility to others at commit. Changed by others: any writer, cascades, triggers, replication, migrations, restores.
- Q8: table+key; view; replica; FDW; ORM object; the application's own cache (Redis etc.), a live copy that disagrees (+SURE).
- Q9: the substrate provides *real* concurrency control: transactions, row locks (`FOR UPDATE`), MVCC, serializable isolation, advisory locks (+SURE) — the strongest decollision in the seed, by design. Lost updates still happen with app-level read-modify-write at READ COMMITTED (+SURE).
- Q10: row: PK (logical), `ctid` (physical, unstable), `xmin` (creating transaction id, +SURE Postgres) — natively *versioned* identity. Cell: none.
- Q11: persisted and live are managed *as one* by the DBMS (+SURE, its purpose): disagreement is between observers (isolation) and between primary and replicas (lag), not between disk and memory. Application caches are the disagreeing live form.
**(b) the column definition**
- Q1: added/dropped while the table persists (+SURE); missing → "column does not exist" → *absent*, yet `information_schema.columns` enumerates present ones (+SURE); sub-parts can be unset (`DROP DEFAULT`, `DROP NOT NULL`, +SURE). Postgres leaves a hidden tombstone after `DROP COLUMN` (`attisdropped`, +SURE) until a rewrite.
- Q2: `information_schema.columns` (standard) vs `pg_attribute` (catalog) vs `\d` vs `DESCRIBE` vs `pg_dump --schema-only` vs the ORM model vs migration files: several renderings of one definition. `RENAME COLUMN` keeps the same `attnum` (+SURE Postgres), so the substrate identifies the column apart from its name; MySQL has no such id (+SURE renames are by name).
- Q3: drop then add the same name → new `attnum` (+SURE); `ALTER TYPE` in place.
- Q4: the catalog, which is itself rows in the DB (+SURE), durable; migration files and ORM models are persisted *intent*, outside the DB. Re-provision via migrations.
- Q5: `search_path`, privileges (`information_schema` shows only columns the role can access, +SURE), case folding, server version. Not namespaces/cwd.
- Q6: type, nullability, default, collation, generated expression, identity, constraints, comment (+SURE), each separately `ALTER`able. Addressed db → schema → table → column.
- Q7: `ADD COLUMN` with a constant default avoids a rewrite in PG ≥ 11 (+SURE), otherwise rewrite under exclusive lock; `ALTER TYPE` rewrites; dependent views block the change (+SURE PG refuses); cached plans invalidate; `SELECT *` consumers shift; logical replication does not carry DDL (+SURE). Changed by others: migration frameworks (+SURE), DBAs, restores, ORM auto-migrate drift.
- Q8: catalog; `information_schema`; DDL dump; migrations; ORM models.
- Q9: migration tools serialize through a version table (`schema_migrations`, `alembic_version`, +SURE) and advisory locks (+SURE Flyway/Rails); DDL is transactional in PG, not in MySQL (implicit commit, +SURE). Hand DDL vs migrations → drift, detected by schema-diff tooling (+SURE).
- Q10: `attnum` and table `oid` in PG (+SURE); none in MySQL.
- Q11: three-way like k8s: ORM model ↔ migration files ↔ catalog; migrations move; drift when hand-edited.

### 43. `pip show requests` (the Version line)
- Q1: installed/uninstalled while the *environment* persists (+SURE); missing → "Package(s) not found", rc 1 → *absent*. `Version:` is mandatory metadata (+SURE). Several installs of one distribution in different site-packages are different things; `pip show` reports the first on `sys.path` (+SURE), which can differ from what `import requests` loads when `.pth` files, editable installs, or shadowing directories intervene (+SURE).
- Q2: `pip list`, `pip freeze` (`requests==2.31.0`, +SURE), `importlib.metadata.version`, `requests.__version__` (self-report, can differ, +SURE), the `*.dist-info/METADATA` directory itself (+SURE the physical form), `pip inspect`, `uv pip show`, `conda list` (conda's *own* metadata in `conda-meta/*.json`, +SURE, a second store over the same files), the Debian package `python3-requests` (ships the `dist-info`, so both dpkg and pip see one install, +SURE). Name normalization: `Requests` ≡ `requests`, `-`/`_`/`.` equivalent (+SURE PEP 503). Distribution name ≠ import name in general (`PyYAML` → `yaml`, +SURE).
- Q3: yes: uninstall/reinstall; upgrade is uninstall-then-install, not atomic (+SURE).
- Q4: `site-packages/<dist>.dist-info/` in *some* site-packages: `/usr/lib/python3/dist-packages` (apt), `/usr/local/lib/python3.X/dist-packages` (pip on Debian), `~/.local/…` (`--user`), a venv, `--target` (+SURE all). Survives reboot; running Python processes keep the *imported* module after uninstall until restart (+SURE). Re-provision: `requirements.txt`, `pyproject.toml`, lockfiles (`uv.lock`, `poetry.lock`, +SURE).
- Q5: strongly: which interpreter (`pip` vs `pip3` vs `python -m pip`, `PATH`, +SURE), `VIRTUAL_ENV`, `PYTHONPATH`, `PYTHONUSERBASE`, `PYTHONNOUSERSITE` (+SURE), the user (`~/.local`), `.pth` files, editable finders (+SURE), `PIP_*` env and `pip.conf`; cwd shadows *imports* (a local `requests/`), not `pip show` (+SURE). The most observer-dependent file-backed item.
- Q6: PEP 440 parts (epoch, release, pre/post/dev, local, +SURE); not separately writable. Addressed by (environment, normalized name); flat within an env.
- Q7: install/upgrade: files replaced, `RECORD` rewritten (+SURE), dependency resolution may up/downgrade *other* distributions (+SURE), entry points in `bin/`; running processes unaffected; dependents may break (`pip check`, +SURE). Changed by others: `poetry`/`uv`/`pipenv`/`conda`/`pipx`, `apt upgrade python3-requests` for the apt-owned copy (+SURE), another package's install pulling a different pin.
- Q8: environment → distribution; the import system (module name); apt; conda; lockfiles; `RECORD`.
- Q9: pip vs apt on system site-packages: PEP 668 `EXTERNALLY-MANAGED` makes pip *refuse* by default (+SURE) — a designed decollision by refusal; pip vs conda: none, conda metadata goes stale (+SURE); pip has no lock (+SURE). Practice: one venv per app plus lockfiles.
- Q10: none for the install; the wheel's SHA256 in the lockfile and per-file hashes in `RECORD` identify *contents* (+SURE).
- Q11: three levels: intent (requirements/lockfile) → installed (`dist-info`) → imported (process memory) (+SURE). `pip install -r` moves intent→installed; `pip freeze` moves installed→intent (routine and lossy, +SURE); process restart moves installed→imported.

### 44. `[section]` / `key = value` in an INI file, read by the owning program
- Q1: key added/removed while the section persists, section while the file persists. INI has no schema: whether a missing key is *unset* (program default, `configparser` `fallback=`/`[DEFAULT]`, +SURE) or *absent* is the consumer's call (+SURE). Duplicate keys: last wins in `configparser` unless `strict` (+SURE), multi-valued in git-style and php `foo[]` — parser-dependent.
- Q2: keys case-insensitive in `configparser` (`optionxform`, +SURE), case-sensitive in git; `=` vs `:` (+SURE configparser accepts both); `;` vs `#` comments; `[DEFAULT]` injects keys into every section (+SURE) so a key can be present with no line in its section; interpolation makes a value depend on another key (+SURE); `.d` includes (`php … conf.d/*.ini`, mysql `!includedir`, +SURE); env overrides and CLI flags. *Which file*: chains merged per program (mysql reads `/etc/my.cnf`, `/etc/mysql/my.cnf`, `~/.my.cnf`, +SURE); php CLI and FPM read *different* `php.ini` files (+SURE the classic).
- Q3: yes; programs that write back (NetworkManager keyfiles, desktop apps, `git config`) rewrite the file; `configparser` round-trips drop comments (+SURE).
- Q4: the file (disk); the program's parsed copy (loaded at start, on SIGHUP, or on inotify — program-specific, +SURE). Re-provision: templates.
- Q5: mount ns; the chain depends on user (`~/.config`), cwd (`./app.ini`), env (`APP_CONFIG`, `XDG_CONFIG_HOME`), and which binary/SAPI (+SURE php). Readability.
- Q6: sections → keys (two levels); values may carry program-defined structure (comma lists, php arrays, +SURE); each key separately writable by key-aware tools (`crudini`, `git config`, Ansible `ini_file`, +SURE), else whole-file. Addressed (file, section, key).
- Q7: writing: nothing until the program reloads (program-specific). Changed by others: package upgrades (conffiles), the program writing back, config management, `.d` drop-ins from other packages, `sed -i`.
- Q8: file → section → key; the program's effective view when it offers one (`mysql --print-defaults`, `php -i`, +SURE both exist); env; `.d`.
- Q9: `.d` drop-ins are the decollision (+SURE); key-aware editors coexist; whole-file templaters and write-back programs clobber (+SURE). No lock.
- Q10: none.
- Q11: persisted (file chain) vs live (program memory); reload moves; the *effective* merged value is readable only if the program exposes it, and many do not (+SURE).

### 45. `.foo.bar` in a JSON config file, read by the owning program
- Q1: as 44, plus JSON's own `null`: key present with `null` (*unset*, natively) vs key missing (*absent*) (+SURE both exist; JS `undefined` vs `null`, Go `omitempty` treat them differently, +SURE). Duplicate keys: parsers take the last (+SURE common). Strict JSON has no comments (+SURE), so "disabled" is by deletion or renaming (`_foo`), a convention.
- Q2: `.foo.bar` ≡ JSON Pointer `/foo/bar` (RFC 6901, +SURE) ≡ JSONPath `$.foo.bar`; keys case-sensitive (+SURE); `foo` ≡ `foo` after parsing (+SURE); `1` vs `1.0` equal in some parsers, distinct in Python (+SURE); `"1"` vs `1` distinct, often coerced by programs. Merge chains: defaults ← file ← env (`APP__FOO__BAR`, the `__` convention in .NET/viper, +SURE) ← flags; directory-cascading configs (eslint) are cwd-dependent (+SURE); `tsconfig` `extends` (+SURE).
- Q3: as 44.
- Q4: as 44; JSON is often *written by the program* (`~/.docker/config.json` on `docker login`, VS Code settings, lockfiles, +SURE) and round-trips reformat (+SURE).
- Q5: as 44, plus directory cascades → cwd.
- Q6: arbitrarily nested; arrays are positional (unstable under insertion, like ufw rule numbers, +SURE); each path separately writable by JSON-aware tools (`jq`, JSON Patch RFC 6902, Merge Patch RFC 7386 where `null` means *delete* — so `null` is "unset" in a document and "delete" in a patch, +SURE the seam). Fully hierarchical.
- Q7: as 44; schema validation at load; program rewrites can drop human edits and unknown keys (+SURE some do).
- Q8: file → path; the program's chain; `jq`.
- Q9: JSON-aware read-modify-write (`jq … | sponge`) coexists; whole-file clobbers; owning programs rewrite. No lock.
- Q10: none.
- Q11: as 44; no comments, so intent annotations live outside the file.

### 46. `lsblk -o NAME,MOUNTPOINT` (a block device's mount point)
- Q1: the mapping exists while mounted; unmounted → device listed with a blank cell → *unset* (+SURE); device missing → "not a block device" → *absent*. A device mounted in several places (bind mounts, multiple mounts of one fs, +SURE legal) shows *one* value in `MOUNTPOINT`; the plural `MOUNTPOINTS` column (util-linux ≥ 2.37, +SURE) lists all → the singular read loses aliases; "the mountpoint" is a set (see E).
- Q2: `findmnt -S /dev/sda1` (+SURE by source), `/proc/self/mountinfo`, `mount`, `df`, `systemctl list-units -t mount`. Device names: `/dev/sda1`, `by-uuid`, `by-id`, `by-path`, `by-label`, `PARTUUID`; for LVM `/dev/mapper/vg-lv` ≡ `/dev/vg/lv` ≡ `/dev/dm-0` (+SURE three names). mountinfo reports the device as the kernel saw it at mount time, while systemd's `What=` keeps the `UUID=` spelling (+SURE-ish). Non-block sources (tmpfs, nfs, overlay) never appear here.
- Q3: yes: `/dev/sdX` letters reshuffle (+SURE); different device at the same path; same device elsewhere.
- Q4: mount table as item 23 (per mount ns); the device: kernel device model + udev db (`/run/udev/data`, +SURE), re-enumerated at boot; persisted intent in fstab/units.
- Q5: mount ns, with a twist: in a container `lsblk` shows the *host's* devices from sysfs but mountpoints from the *container's* mount ns → a mismatched view (+SURE known confusion). Readable by all. Not user/cwd/env.
- Q6: the device tree has children (partitions), holders, slaves (+SURE `lsblk` tree); `MOUNTPOINT` is one attribute, writable only via mount/umount. Addressed by any device alias; hierarchical device tree; mountpoint is a path.
- Q7: as item 23. Changed by others: udisks2 automounting under `/media/$USER/<label>` (+SURE), `.automount` units, Docker volumes, snapd loop mounts (+SURE), LVM activation (item 47), mdadm assembly, hotplug.
- Q8: device (many names), mountpoint, fstab, unit, UUID, sysfs, udev.
- Q9: as 23; udisks2 skips fstab-listed devices for automount (+SURE-ish).
- Q10: `major:minor` (live, +SURE), fs UUID (persisted, from mkfs, +SURE), `PARTUUID`, hardware WWN/serial via `by-id`, `DM_UUID` for LVM (+SURE) — several layers of strong device identity.
- Q11: as 23; the *device name* has a persisted alias (UUID) and a live alias (`/dev/sdX`), and this read reports the live one.

### 47. `lvs -o lv_active` for one LV, and that LV's entry in the VG metadata
- Q1: LVs created/removed while the VG persists (+SURE); `lv_active` prints `active` or blank (+SURE) → *unset* rendered as blank; LV missing → "Failed to find logical volume", rc 5 → *absent*. Existence (metadata) and activation (a dm device) are separate: present-but-inactive is the `noauto` analogue. `lvs -a` reveals hidden internal LVs (`[lv_rimage_0]`, +SURE), a hidden class.
- Q2: `lvdisplay`, `lvs -o +lv_dm_path`, `/dev/vg/lv` (udev symlink, +SURE), `/dev/mapper/vg-lv` (with `--` escaping of hyphens, +SURE), `/dev/dm-N`, `dmsetup info` (the device-mapper view; `lv_active` is derived from dm presence, +SURE), `lsblk` (item 46), `/etc/lvm/backup/<vg>` (metadata as text, +SURE), `pvck --dump metadata` (on-PV copy, +SURE), `lv_uuid`; `lvrename` changes the name, not the UUID (+SURE); tags group.
- Q3: yes: remove/create same name → new `lv_uuid` (+SURE); `dm-N` minors reassigned on re-activation (+SURE); `vgrename` moves every LV path; cloned disks with duplicate VG identity need `vgimportclone` (+SURE).
- Q4: live: dm tables in the kernel (+SURE), lost at reboot, re-activated at boot by the lvm2 generator or udev-triggered `pvscan --cache -aay` (+SURE; `lvmetad` is gone in 2.03). Persisted: VG metadata *on the PVs themselves*, replicated across the VG's PVs with a seqno (+SURE), plus `/etc/lvm/backup` and `/etc/lvm/archive` text copies (+SURE), plus `lvm.conf` gates (`volume_list`, `auto_activation_volume_list`, `filter`, +SURE). OS re-provision with data disks kept retains the VG (+SURE `vgscan`); `activationskip` (`-k`) persists a do-not-auto-activate flag *in the metadata* (+SURE).
- Q5: not namespaced (dm is global, +SURE); but `lvm.conf` `filter` and the devices file (`/etc/lvm/devices/system.devices`, +SURE ≥ 2.03.12) decide which PVs lvm may *see*, so the same disk can yield "no VG" under a different config; `LVM_SYSTEM_DIR` env (+SURE); root required. Not cwd.
- Q6: `lv_attr` is a 10-character flag string (type, perms, alloc, state, open, target, zero, health, skip, +SURE); size, segments, tags, `activationskip`; separately writable via `lvchange -a/-p/-k/--addtag`, `lvextend`. Addressed VG → LV (→ segments); the dm layer is flat.
- Q7: `lvchange -ay`: dm device created, udev links, `dev-vg-lv.device` unit appears (+SURE), dependent mounts can proceed; `-an` fails while open (+SURE). Activation is *not* recorded in metadata (+SURE, only `lv_attr` state bits change); `lvcreate`: seqno++, written to all PVs, backup rewritten, archive entry (+SURE). Changed by others: boot auto-activation, `vgchange -ay`, udev `pvscan`, `dmeventd` auto-extending thin pools (+SURE), `lvconvert`, snapshot merges, cluster locking (`lvmlockd`), installers, Ansible `lvol`.
- Q8: `vg/lv`; `lv_uuid`; dm name; `dm-N`; `/dev/mapper`; the PVs' metadata; the backup file; tags; the mount on top (item 46).
- Q9: lvm commands take a per-VG file lock (`/run/lock/lvm/V_<vg>`, +SURE) and use metadata seqno (+SURE); shared VGs require `lvmlockd` (+SURE); `dmsetup` bypasses all of it (+SURE). Designed locking, second only to SQL in the seed.
- Q10: `lv_uuid`, `vg_uuid`, `pv_uuid` persisted in metadata and stable across rename (+SURE); live `DM_UUID` = `LVM-<vguuid><lvuuid>` (+SURE), `dm-N` unstable. Strong persisted identity distinct from name, like k8s `uid`.
- Q11: persisted (on-PV metadata + text backups) vs live (dm tables); activation moves persisted→live; `vgcfgbackup`/`vgcfgrestore` move metadata↔text in both directions (+SURE). Disagree: inactive LV, `dmsetup` hand edits, stale metadata on a detached PV (`vgreduce --removemissing`, +SURE), duplicate VGs, a backup file lagging after `--nobackup` or a crash (+SURE possible).
