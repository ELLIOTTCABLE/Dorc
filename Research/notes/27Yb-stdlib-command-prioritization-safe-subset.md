# 27Yb — stdlib prioritization: safe subset (C2 <= 2 rows)

## §5 — The prioritization table

Columns are 1..5, 5 = most of the named quality:

- **C1** complexity-to-model (easy..difficult)
- **C2** memetic hazard (low..high)
- **C3** prevalence (rare..common)
- **C4** punt-independence (ship-now value; low = value gated behind punted work..high = value immediately)
- **elide?** = yes / guard-only / no.

> The safe subset (`27Yb`) contains the **C2 <= 2** rows of this table (risk-2 and below).
> Rows with C2 >= 3 are, by construction, absent from 27Yb. The full table is quarantined in `27Ya`.

| command | domain | C1 | C2 | C3 | C4 | elide? | justification |
|---|---|---|---|---|---|---|---|
| apt-get | package | 5 | 2 | 5 | 5 | yes/no-update | broad index+package+files disturbance, provides-`resolve`; install/remove elide via dpkg-status, `update` is time-based (no convergence). |
| apt | package | 5 | 2 | 4 | 5 | yes/no-update | same dpkg machinery as apt-get; scripts prefer apt-get (apt CLI self-declared unstable), slightly lower prevalence. |
| dpkg | package | 4 | 2 | 3 | 5 | yes | named-`.deb` only (no repo graph) but still needs package->files disturbance; `dpkg -s` is the canonical rc-guard. |
| dpkg-query | package | 2 | 2 | 3 | 1 | no | pure read; dominant `-W -f` value-extraction = punted value-plane. |
| update-alternatives | package | 3 | 1 | 2 | 4 | yes | auto/manual+priority state-machine over one symlink-target cell; no infosec adjacency. |
| snap | package | 4 | 2 | 2 | 3 | yes/guard-refresh | separate Package kind + own resolve; channel/revision tracking = MH2-adjacent (docks C4). |
| dnf | package | 5 | 2 | 4 | 5 | yes/no-checkupd | structurally == apt-get; first-class `dnf provides` resolve; RPM-world default (Fedora/RHEL8+). |
| yum | package | 5 | 2 | 3 | 5 | yes/no-checkupd | == dnf (legacy shim on modern RHEL, primary on RHEL7/AL2); near-zero marginal cost once dnf modeled. |
| rpm | package | 4 | 2 | 4 | 5 | yes | named-rpm (no dep graph); dual kind (Package via -i/-q/-e, TrustedKey via --import); `rpm -q` = rc-guard. |
| brew | package | 4 | 1 | 3 | 5 | yes | multi-kind (tap + formula/cask + `brew services`); no sudo, no infosec adjacency. |
| pip3 | package | 5 | 1 | 4 | 2 | guard-only | version-RANGE default idiom = MH2 punt (docks C4 hard); venv/site-packages scoping wrinkle. |
| npm | package | 5 | 1 | 5 | 2 | guard/yes-ci/no-run | `install`=semver-range MH2 punt; `npm ci`=exact-lockfile (ships fully); `npm run`=arbitrary, no convergence. |
| gem | package | 3 | 1 | 2 | 4 | yes | simpler (exact-version pin; ranges live in `bundle`); Ruby ecosystem prevalence declined. |
| systemctl | service | 4 | 2 | 5 | 5 | yes | distinct `#enabled`/`#active` cells; broad verb argparse; daemon-reload/restart have no convergence sense. |
| service | service | 3 | 2 | 4 | 5 | yes | single `#active`-ish cell via status; narrower than systemctl (no enable on Debian). |
| journalctl | service | 1 | 2 | 3 | 1 | no | pure read-only log query; value = captured output (punted); never an elision target. |
| update-rc.d | service | 3 | 2 | 2 | 4 | yes | Debian legacy `#enabled` via rcN.d symlink presence; enable-slice only. |
| systemd-tmpfiles | service | 3 | 2 | 2 | 5 | yes | per-path existence/mode/owner cells from tmpfiles.d; rarely author-invoked (boot/postinst). |
| loginctl | service | 2 | 2 | 2 | 4 | guard-only | `enable-linger` = clean boolean cell; session actions have no convergence (heterogeneous). |
| invoke-rc.d | service | 3 | 2 | 2 | 4 | guard-only | same `#active` cell as service; `policy-rc.d` hook layer -> guard-only; mostly maintainer-scripts. |
| ip | firewall/net | 5 | 2 | 4 | 3 | guard-only | multi-kind (addr/link/route/rule/netns); `ip netns exec`=context-transiter (netns punted); `show`=value-capture. SPLIT candidate. |
| route | firewall/net | 3 | 2 | 2 | 3 | yes | legacy add/del; convergence via `route -n` parse; declining (superseded by `ip route`). |
| netplan | firewall/net | 4 | 2 | 2 | 2 | guard-only | whole-file YAML apply, broad disturbance; `netplan apply` rc is NOT a convergence signal; Ubuntu-only. |
| resolvectl | firewall/net | 3 | 2 | 2 | 3 | guard-only | per-link DNS cell via status; runtime settings ephemeral (networkd re-applies); systemd-resolved-specific. |
| ss | firewall/net | 1 | 2 | 3 | 1 | no | pure diagnostic read; value = captured output (punted). |
| netstat | firewall/net | 1 | 2 | 3 | 1 | no | == ss (legacy net-tools); still prevalent in scripts/docs. |
| ping | firewall/net | 1 | 2 | 4 | 1 | no | reachability is point-in-time, non-cacheable (true wall); rc-gate or parsed-stats, no Dorc value either way. |
| cp | filesystem | 3 | 1 | 5 | 5 | yes | content-match cell (src bytes == dest); archetypal ubiquitous elide. |
| mv | filesystem | 2 | 1 | 3 | 4 | no | non-idempotent (errors on absent source) so no "already achieved" state; clean rename footprint useful for survival. |
| rm | filesystem | 2 | 1 | 4 | 4 | guard-only | `rm -f` absent-path = no-op (mkdir -p-like) but bare `rm` errors -> flag-conditional elision. |
| mkdir | filesystem | 2 | 1 | 5 | 5 | yes | dir-exists cell under `-p`; textbook clean elide. |
| rmdir | filesystem | 2 | 1 | 2 | 4 | no | no `-p`-equivalent force; errors on missing/non-empty; rare (rm -rf supersedes). |
| ln | filesystem | 3 | 1 | 4 | 5 | yes | symlink-target cell under `-sf`; entity-cell compare, no cross-kind reach. |
| touch | filesystem | 2 | 1 | 3 | 5 | yes | existence-cell (sentinel idiom); mtime-refresh caveat needs a separate cell if a script depends on it. |
| cat | filesystem | 2 | 1 | 5 | 2 | no/guard | as heredoc/redirect config-writer -> content-cell (like tee); plain `cat file` reader = value-capture (punted); ubiquitous. |
| chmod | filesystem | 2 | 2 | 5 | 5 | yes | mode-bitmask cell; symbolic-mode resolution wrinkle; permissions = access-control valence. |
| chown | filesystem | 2 | 2 | 3 | 5 | yes | owner-cell + uid/gid name-resolution; ownership mildly security-flavored. |
| chgrp | filesystem | 2 | 2 | 2 | 5 | yes | group-cell subset of chown; rarer (chown user:group subsumes). |
| install | filesystem | 3 | 2 | 3 | 5 | yes | compound cell (content+mode+owner+dir via -D/-t/-m/-o/-g); real argparse; File kind. |
| rsync | filesystem | 4 | 2 | 3 | 3 | guard-only | full tree-diff convergence + `--delete` broad disturbance = CHECK-TAX; bulk-transfer mild adjacency. |
| tar | filesystem | 4 | 2 | 4 | 3 | no | create/extract = output/side-effect; extract footprint unbounded until members enumerated (`-t`); path-traversal adjacency. |
| umount | filesystem | 3 | 2 | 2 | 2 | guard-only | mirror of mount but needs the same punted mount-table truth; narrower argparse. |
| tee | filesystem | 1 | 1 | 2 | 2 | no | stdin-stream writer; content from upstream pipeline (no static convergence w/o running it). |
| sync | filesystem | 1 | 1 | 1 | 2 | no | global buffer-flush barrier; no Dorc cell (structurally thin). |
| mktemp | filesystem | 1 | 2 | 3 | 1 | no | value-capture (`X=$(mktemp)`); never converges (uniqueness); CWE-377 mild adjacency. |
| dpkg-reconfigure | auth/security | 4 | 2 | 3 | 2 | no | invokes arbitrary package-specific debconf scripts, no generic readback; broad/opaque disturbance; not itself a security tool. |
| env | wrapper | 4 | 2 | 3 | 4 | yes | context-transiting but FLAT (KV env overlay, no entity/kind resolution); mild LD_PRELOAD/PATH-hijack dual-use edge. |
| nice | wrapper | 2 | 1 | 2 | 4 | yes | identity-ish wrapper — scheduler priority isn't a tracked cell; verdict = inner command's; no infosec adjacency. |
| ionice | wrapper | 2 | 1 | 2 | 4 | yes | == nice class (I/O scheduling, not a tracked cell); rarer (backup/rsync scripts). |
| nohup | wrapper | 2 | 1 | 3 | 4 | guard-only | identity-ish wrapper; dominant `nohup cmd &` detaches rc from sync availability, so realistic elision rarely fires. |
| timeout | wrapper | 3 | 1 | 3 | 4 | yes | adds a new outcome class (rc 124 = deadline) distinct from inner divergence; not entity/kind heavy. |
| xargs | wrapper | 5 | 1 | 4 | 2 | guard-only | fan-out invocation over a (often dynamic, non-statically-enumerable) argument set; fan-out machinery NOT in this round's wrapper scope. |
| find | wrapper | 5 | 2 | 4 | 2 | guard-only | graded on `-exec`/`-delete` fan-out (dynamic-set, unshipped fan-out); bare read-only `find` is a trivial separate row. SPLIT candidate. |
| flock | wrapper | 3 | 1 | 2 | 1 | guard-only | execution-gating wrapper; lock-acquired/contended is a THIRD outcome class; value = punted concurrency/cross-run reasoning. |
| setsid | wrapper | 2 | 1 | 2 | 4 | guard-only | identity-ish wrapper (new session/pgroup, untracked); dominant `setsid cmd &` detaches rc like nohup. |
| uname | read/text/proc | 1 | 2 | 4 | 1 | no | pure read (arch/OS/kernel), ubiquitous in bootstrap arch-detection; value = captured stdout (punted); mild fingerprinting. |
| command -v | read/text/proc | 1 | 2 | 5 | 4 | guard-only | `command -v x >/dev/null ||` existence-guard (ships; Dorc elides the gated install); value-capture variant docks; POSIX-canonical. |
| which | read/text/proc | 1 | 2 | 4 | 3 | guard-only | == command -v shape but non-POSIX/less portable (busybox); varying exit semantics dock guard-reliability. |
| type | read/text/proc | 1 | 2 | 2 | 3 | guard-only | shell-builtin existence check; POSIX documents output as human-oriented (command -v is the scripting form) -> less used. |
| test / [ | read/text/proc | 2 | 1 | 5 | 5 | guard-only | THE canonical POSIX guard primitive — Dorc's is_converged/guard-recognition is built around parsing test-shaped conditions; ships now. |
| grep | read/text/proc | 2 | 1 | 5 | 4 | guard-only | `grep -q` guard-primitive (ships); non-guard filter/extract usage is value-capture (docks); ubiquitous. |
| sed | read/text/proc | 3 | 1 | 5 | 3 | yes | `sed -i` mutates a file; "already-replaced" is a genuine content-cell convergence via companion read; per-call authoring effort docks C4. |
| awk | read/text/proc | 1 | 1 | 3 | 1 | no | dominant portable use is field/value EXTRACTION, no convergence (GNU `awk -i inplace` would be sed-like but non-dominant). |
| cut | read/text/proc | 1 | 1 | 2 | 1 | no | pure field-splitter, no mutation, no rc to guard on — unambiguous value-producer (punted). |
| tr | read/text/proc | 1 | 1 | 2 | 1 | no | pure stdin->stdout transliteration, almost always inside `$(...|tr...)` capture chains (punted). |
| echo | read/text/proc | 1 | 1 | 4 | 1 | no | pure side-effect/output; `echo x >> file` append is a content-cell edge (see notes) but bucketed no-convergence. |
| printf | read/text/proc | 1 | 1 | 5 | 1 | no | == echo bucket, POSIX-preferred; census-top command (15) — highest raw prevalence in the read/text batch. |
| sleep | read/text/proc | 1 | 1 | 3 | 1 | no | temporal delay — no "already-slept" state (convergence is a category error); retry/backoff idioms. |
| date | read/text/proc | 1 | 2 | 3 | 1 | no | dominant `$(date +%F)` value-capture (punted); C2 up for `date -s` clock/log-timestamp tampering adjacency. |
| pgrep | read/text/proc | 1 | 2 | 4 | 4 | guard-only | pure READ; `pgrep -x svc >/dev/null || start` liveness-guard (ships); value-capture variant docks. |
| wait | read/text/proc | 1 | 1 | 2 | 1 | no | background-job sync primitive; "already waited" isn't a state (structurally no convergence). |
| cmp | read/text/proc | 1 | 1 | 3 | 4 | guard-only | byte-exact compare whose whole role IS the idempotent-update guard (`cmp -s new old || cp`); rc-consumed, ships now. |
| logger | read/text/proc | 1 | 2 | 2 | 1 | no | one-way syslog write, no idempotency; C2 mild for log-injection/anti-forensics adjacency. |
| mail | read/text/proc | 1 | 2 | 2 | 1 | no | one-way send, no convergence (twice = two emails); generic external-send channel (tier-2 like curl/wget). |
| docker | container/orch | 4 | 1 | 4 | 4 | yes | broad cross-kind cells (container/image/network/volume) + heavy `run` argparse; `docker inspect` local rc-convergence; no infosec adjacency. |
| docker-compose | container/orch | 4 | 1 | 3 | 4 | yes | multi-service desired-state; delegates to `compose ps`/config-diff (delegation-oracle); per-service resolution + correlated disturbance. |
| podman | container/orch | 4 | 2 | 2 | 4 | yes | docker-compatible surface; rootless/userns-by-default nudges C2 up one; narrower install base. |
| kubectl | container/orch | 4 | 2 | 3 | 2 | yes | many resource kinds + resolve/reaches (Deployment->RS->Pod); `kubectl diff` is trustworthy but diffs REMOTE state (multi-host dock). |
| helm | container/orch | 5 | 2 | 2 | 2 | guard-only | k8s-kind surface + opaque chart-templating (must render for footprint); trustworthy diff needs helm-diff plugin; remote — double dock. |
| terraform | container/orch | 2 | 1 | 2 | 2 | yes | near-pure delegation (`plan -detailed-exitcode` 0/2) — low modeling cost; diffed state is remote (multi-host dock). |
| ansible-playbook | container/orch | 3 | 2 | 3 | 4 | no | canonical DECLINE — `--check` untrustworthy (uneven module coverage), oracle declines, line always runs; broad wall correctly protects downstream (non-punted). |
| chezmoi | container/orch | 2 | 1 | 2 | 5 | yes | clean delegation to `chezmoi verify`/`status` (local whole-tree drift check); niche dotfiles audience. |
| git | container/orch | 4 | 1 | 4 | 3 | yes | broad VCS cells (refs/index/worktree/remotes); `git status`/`diff --quiet` local delegation; `pull`/`fetch`/`push` need REMOTE compare (dock). SPLIT candidate. |
| curl | container/orch | 2 | 2 | 4 | 2 | guard-only | no native convergence (always re-fetches); elides only via a wrapping file-exists guard; dominant API/health-check usage feeds value-plane (punted). |
| wget | container/orch | 2 | 2 | 3 | 3 | guard-only | == curl but narrower; dominant plain-download usage + `-nc`/`-N` quasi-native skip-if-present nudges C4 above curl. |
| make | container/orch | 3 | 1 | 4 | 4 | no | opaque recipe runner; native mtime tracking is real but `.PHONY` opts out and Dorc doesn't parse Makefiles -> declines; broad wall = non-punted value. |
| diff | read/text/proc | 1 | 1 | 4 | 4 | guard-only | `diff -q new old || cp` idempotent-update guard (like cmp); also value-capture usage; corpus-surfaced (dotfiles-frequent). |
| defaults | macos-config | 3 | 1 | 3 | 5 | yes | macOS per-(domain,key) value cell via `defaults read`; dotfiles-ubiquitous on macOS (USER_STORY's own unmodeled example); rc-convergence ships now. |
