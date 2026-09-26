# gather-assessment-languages-and-verifiers — read-only host-state assessment languages (OVAL, XCCDF, osquery, InSpec/Train, Testinfra, Goss, Facter, ohai)

311 read at `63e49f29`. Every grade below is `graded-by: subagent`.

## Findings

- OVAL has the battle-tested absence partition the lane was asked to look for, and it has it on three separate levels. A collected object carries a six-valued flag: `error`, `complete`, `incomplete`, `does not exist`, `not collected`, `not applicable`. Each item and each entity carries a four-valued status: `error`, `exists`, `does not exist`, `not collected`. A result is six-valued: `true`, `false`, `unknown`, `error`, `not evaluated`, `not applicable`. The spec gives MUST-level truth tables that carry each flag and status up into results. The strongest part is the `incomplete` flag, OVAL's closed-world marker: an existence test may conclude "none exist" or "only one exists" only when the collector claims `complete`. [A-oval-language-specification-5112-2016] [B-oval-issue-83-check-existence-2013] +SURE
- The partition breaks down in practice at `not applicable`. Interpreters disagree on whether an `rpm_object` on AIX or Ubuntu is `not applicable` or `does not exist`. In an AND, `not applicable` has the lowest priority, so it never reaches the top of a definition. `applicability_check` "was never completely thought out". Practitioners moved applicability up a layer, to XCCDF. [B-oval-issue-301-not-applicable-result-2018] [A-nist-ir7275r4-xccdf-12-2011] +SURE
- OVAL defines object identity as the name: the string tuple of the object's entities. Items are "differentiated by examining each OVAL Item's name and each of the OVAL Item's entity names and values". `file_state` has no inode or device entity. Two hardlinks are therefore two items, and nothing in OVAL can observe that they are one file. This is a clean instance of the "unequal names are different things" punt that 311 refuses. [A-oval-language-specification-5112-2016] [A-oval-unix-definitions-schema-2026] +SURE
- The same path string denotes different things in different tools. OVAL's `file_object` and Goss stat the link itself (lstat), so a symlinked `/etc/motd` reads mode 777. InSpec (through Train, `follow_symlink = true`) and osquery (lstat only for the `symlink` flag, then `stat()` for every other attribute) report the target. InSpec itself flipped between these readings in 2016 (#665). Its local and SSH backends once disagreed on the type of one path (#3099). [A-oval-unix-definitions-schema-2026] [B-oval-community-symlink-file-object-discussion-2025] [A-train-file-follow-symlink-default-2026] [B-inspec-issue-665-symlink-permissions-2016] [A-osquery-file-table-implementation-2026] [A-goss-gossfile-reference-2026] +SURE
- The strongest single 311 hit is a live 2025 OVAL board thread (discussion #261). Participants found that following a link hides an insecure intermediate directory or an intermediate link, and that the final canonical path skips the chain. The proposed fix is to emit each intermediate link as its own `chainlink` entity, which is 311's emitted mTraversal (§2.9) arrived at independently. The thread adds a sharp rule: an absent child entity means "not collected", so the end of the chain needs an explicit `status="does not exist"` entity. That rule is 311's "no emission means the whole catalog" floor, in OVAL terms. The idea is unratified. [B-oval-community-symlink-file-object-discussion-2025] [B-oval-issue-107-symlink-target-2013] +SURE
- osquery is the only tool in this family that exposes inode-level data (`inode`, `hard_links`, `symlink_target_path`). Its `device` column is `st_rdev`, not `st_dev`, so the row still gives no (dev, ino) identity pair. [A-osquery-table-schema-5231-2026] [A-osquery-file-table-implementation-2026] +SURE
- osquery collapses absence, error and incompleteness into "no row" or "empty cell". A failed `lstat` drops the row whether the cause is ENOENT, ELOOP or EACCES. The `file` table can return about 3,700 of 2 million files, with no completeness flag, and the report was closed WONTFIX. Maintainers concede that an empty column means "no data", a bug, or platform skew, visible only as a verbose-log cast warning. [A-osquery-file-table-implementation-2026] [B-osquery-issue-7306-file-table-incompleteness-2021] [B-osquery-issue-6319-empty-column-casting-2020] +SURE
- osquery's vantage mechanism is a close analogue of 311's mVantage (§1.10). A hidden `pid_with_namespace` input column (on 18 tables) makes the generator `setns()` into that pid's mount namespace. The returned rows are tagged with the pid and `mount_namespace_id`, and neither column is part of the row key. Four tables expose `net_namespace` as data. Only the mount namespace is ever entered. [A-osquery-table-schema-5231-2026] [B-osquery-issue-6209-container-table-access-2020] +SURE
- OVAL also has explicit observer dependence (311 §2.8). `environmentvariable58` keys a variable by pid, and a nil pid means "the scanner's own process". Facter's `identity` fact is the observer's own uid and gid. [A-oval-independent-definitions-schema-2026] [A-facter-core-facts-schema-2026] +SURE
- An exit status is not an error without author speech (GOTCHAS: nonzero-status-is-not-speech). OVAL 5.12's `shellcommand` reports every run as `exists`. It sets `error` only if the content author opts in with `error_if_exit_status_not_0` or `error_if_stderr_exists`. Implementers explicitly lack "enough knowledge of the intent of the command". [A-oval-independent-definitions-schema-2026] [B-oval-community-shellcommand-status-discussion-2025] +SURE
- For the 311s §C case (`systemctl is-active` folds absent into `inactive`), OVAL gets it right, but only in the reference interpreter, not in the schema. OpenSCAP's `systemdunitproperty` probe enumerates the unit-file catalog over D-Bus (`ListUnitFiles`) and collects items only for matching names. A nonexistent unit therefore yields no item, so the object flag is `does not exist`, not `ActiveState=inactive`. A D-Bus failure gives `error` online and `not collected` offline. The mechanism is enumerate the catalog, then match (a closed catalog). The schema itself is silent on this. The other tools fold:
  - Testinfra enumerates exit code 4, "no such unit", then returns `rc == 0`, so a missing unit reads as "not running".
  - InSpec separates `installed` (`LoadState == "loaded"`) from `running` (is-active exit 0), so the fold survives only in `running`.
  - osquery exposes `load_state`, so the information is there but not partitioned.

  [A-openscap-systemdunitproperty-probe-2026] [A-oval-linux-definitions-schema-2026] [A-testinfra-service-module-2026] [A-inspec-service-resource-source-2026] [A-osquery-table-schema-5231-2026] +SURE for the code paths. -GUESS whether `ListUnitFiles` omits transient or generated units, which would make those read "does not exist" while running.
- The tools pick "which provider answers a name" in four different ways:
  - InSpec: a hard-coded distro+release table for init systems, one package manager per OS family, and explicit per-manager resources as the author's override.
  - Testinfra: a runtime probe of `/run/systemd/system`.
  - Goss: a CLI flag (`--package`).
  - OVAL: pushes the choice into the test type (`dpkginfo` vs `rpminfo`).

  None handles `provides`, virtual packages, or one name under two init systems, except InSpec's is-enabled fallback to SysV. [A-inspec-service-resource-source-2026] [A-inspec-package-resource-source-2026] [A-testinfra-service-module-2026] [A-goss-gossfile-reference-2026] [A-oval-linux-definitions-schema-2026] +SURE
- InSpec folds absence into other verdicts:
  - `dpkg -s` with any nonzero exit becomes "not installed", covering missing, purged, locked and binary-missing alike.
  - The canonical custom-resource example raises `ResourceSkipped` when a config file does not exist, which turns absence into a skip, that is, not-applicable.
  - `resource_id` is an author-declared unique identifier. It is the one declared-identity hook in the family.

  [A-inspec-package-resource-source-2026] [A-inspec-custom-resources-docs-2026] +SURE
- Counter-thesis confirmed: no assessment language models what a write reaches, or interference between checks.
  - OVAL requires that "the same OVAL Items MUST be collected on a system for a given OVAL Object" whatever the methodology, which treats collection as a pure read.
  - OVAL overlapping objects are allowed and only de-duplicated.
  - OVAL's one nod to effects is the `shellcommand` "DO NO HARM" warning, a warning to authors, not a model.
  - XCCDF's `fix` carries a scalar `disruption`, `reboot` and `strategy`, not a set of things reached. Nothing relates one rule's fix to another rule's check.

  [A-oval-language-specification-5112-2016] [A-oval-independent-definitions-schema-2026] [A-nist-ir7275r4-xccdf-12-2011] +SURE
- Host identity is an unkeyed bag of facts. Facter's `uuid` comes from `/sys/class/dmi/id/product_uuid`, with no clone caveat and no machine-id fact. ohai's `shard_seed` is a host-identity recipe the admin declares: a configurable ordered list of sources, hashed, raising rather than defaulting when DMI is unreadable. ohai's `machine_id` falls back from `/etc/machine-id` to `/var/lib/dbus/machine-id`. [A-facter-core-facts-schema-2026] [B-ohai-shard-seed-plugin-2016] ~SUSPECT (relevance)

## Candidate table

| [slug] | what it is | exhaustive / broad / abstract / battle-tested | 311 analogues | punts | altitude |
|---|---|---|---|---|---|
| [A-oval-language-specification-5112-2016] | OVAL core spec 5.11.2 | Exhaustive in evaluation semantics; broad via component schemas; abstract (object/state/item/test); battle-tested since 2002 in SCAP validation, CIS, and Red Hat/Canonical/SUSE advisories | cell ≈ item entity; KNOWN_UNSPOKEN/UNKNOWN ≈ `unknown`/`not collected`; the closed at-most set ≈ `complete` vs `incomplete` | Identity is the entity-value tuple; no aliasing notion; no effects | read-only assessment |
| [A-oval-unix-definitions-schema-2026] | OVAL unix tests (file, symlink, sysctl, process58, interface, password/shadow...) | Deep on files; broad on unix; concrete; battle-tested | `recurse_file_system=defined` ≈ "stay inside one mParent-Store"; `symlink_object` ≈ a resolve() that yields only the terminus | No inode/device; symlink chain collapsed to its canonical end | assessment |
| [A-oval-linux-definitions-schema-2026] | OVAL linux tests (dpkginfo, rpminfo, systemdunit*, partition, inetlisteningservers, selinux, apparmor) | Broad; concrete; battle-tested | apparmorstatus_object = a cell with an empty key (a singleton under the host) | Package = name per manager; unit = its full name; no provides/alias | assessment |
| [A-oval-independent-definitions-schema-2026] | OVAL independent tests (shellcommand, environmentvariable58, textfilecontent54, sql512...) | Broad; the shell escape hatch | observer-dependence (pid in the key); author speech decides what an exit code means | Write effects left to "DO NO HARM" | assessment |
| [B-oval-issue-107-symlink-target-2013] | OVAL tracker: symlink representation debate | Deep on one aliasing case; core authors | The "illusion" vs "switching files" split is the SAME-vs-two-mReferents question | Outcome: a separate test yielding only the terminus | assessment |
| [B-oval-issue-301-not-applicable-result-2018] | OVAL tracker: making NA meaningful | Battle-test evidence (133 CIS definitions, interpreter disagreement) | the NA partition | NA pushed to XCCDF | assessment |
| [B-oval-issue-83-check-existence-2013] | OVAL tracker: all_exist vs closed world | Narrow but foundational | the at-most set closure (completeness) | "whatever the scanner found must be interpreted to be all there was" | assessment |
| [B-oval-community-symlink-file-object-discussion-2025] | OVAL board forum: follow_symlinks + chainlink | Deep on one case; live | the emitted mTraversal; the no-emission floor | Unresolved | assessment |
| [B-oval-community-shellcommand-status-discussion-2025] | OVAL board forum: shellcommand status | Narrow | nonzero-status-is-not-speech | — | assessment |
| [A-openscap-systemdunitproperty-probe-2026] | OpenSCAP's OVAL systemd probe | Narrow; reference implementation, shipped in RHEL | The closed catalog as the absence witness (§2.9 floor, inverted: enumerate, then match) | Transient units -GUESS | assessment |
| [A-nist-ir7275r4-xccdf-12-2011] | XCCDF 1.2 spec (the SCAP checklist layer) | Nine-valued result; fix metadata | Result partition; `disruption` as a coarse writeset | No reach/interference | assessment + remediation metadata |
| [A-osquery-table-schema-5231-2026] | osquery schema, 286 tables | Broadest catalogue; SQL-abstract; deployed on millions of endpoints | mVantage (`pid_with_namespace`, `net_namespace`); never-reused `upid`; two forms (`current_value`/`config_value`) | Row keys are names; no status column | read-only query |
| [A-osquery-file-table-implementation-2026] | osquery file table code | One table, precise | — | Absence, error and loop all mean no row; `device` is `st_rdev` | read-only |
| [B-osquery-issue-6209-container-table-access-2020] | osquery container-namespace PR | One mechanism | mVantage = input and output, not key | Mount namespace only | read-only |
| [B-osquery-issue-6319-empty-column-casting-2020] | osquery maintainers on empty cells | Narrow | — | Three states in one empty cell | read-only |
| [B-osquery-issue-7306-file-table-incompleteness-2021] | osquery file-table completeness | Narrow | completeness | WONTFIX; no flag | read-only |
| [A-train-file-follow-symlink-default-2026] | InSpec's transport file object | Precise | Which mReferent a path names | Follows by default; cache keyed by path string | assessment |
| [B-inspec-issue-665-symlink-permissions-2016] | InSpec tracker: symlink mode (CIS control) | Litigation record | as [B-oval-issue-107-symlink-target-2013] | — | assessment |
| [A-inspec-service-resource-source-2026] | InSpec service resolution | Broad over init systems | Provider seat ≈ which mScheme's resolve() answers | Distro table, not probe | assessment |
| [A-inspec-package-resource-source-2026] | InSpec package resolution | Broad over package managers | — | All errors fold into "not installed"; no provides | assessment |
| [A-inspec-custom-resources-docs-2026] | InSpec resource-author contract | Narrow | `resource_id` = declared identity | Absence becomes skipped | assessment |
| [A-testinfra-service-module-2026] | Testinfra service | Narrow, precise | the 311s §C case, enumerated then folded | Prefix-grep `exists` | assessment |
| [A-goss-gossfile-reference-2026] | Goss resource reference | Moderate breadth | tcp/tcp6 dual-stack alias (two keys, one socket) | Package manager by flag | assessment |
| [A-facter-core-facts-schema-2026] | Facter fact schema | Broad host facts | observer (`identity`) | Clone-blind uuid | fact collection |
| [B-ohai-shard-seed-plugin-2016] | ohai shard_seed and machine_id | Narrow | Identity recipe as admin speech; fallback lookup | Clone-blind | fact collection |

## Breadth: the 47 items mapped

Legend. O = OVAL test, Q = osquery table, I = InSpec core resource, G = Goss, T = Testinfra. Certainty is +SURE where I read the test or table spec, and -GUESS where I saw only the name in a listing. "Name only" means the tool keys the item by a string with no alias handling.

| # | item | coverage |
|---|---|---|
| 1 | is-active | O systemdunitproperty(ActiveState); Q systemd_units.active_state; I service be_running; G service running; T is_running +SURE. Absent folded into "not running" in I (running), T and G -GUESS |
| 2 | is-enabled | O systemdunitproperty(UnitFileState); Q unit_file_state; I be_enabled (SysV fallback); T is_enabled (raises on unknown) +SURE |
| 3 | `Restart=` line | O systemdunitproperty(Restart) or textfilecontent54; I service params (`systemctl show`) +SURE; Q none |
| 4 | InvocationID | O systemdunitproperty (any property) +SURE; Q none; I params -GUESS |
| 5 | mode bits | O file_state; Q file.mode; I file mode; G file mode +SURE. The symlink default splits them (see Findings) |
| 6 | bytes | O filehash58 / textfilecontent54; Q hash; I content/sha256sum; G sha256 +SURE |
| 7 | inode + hardlink | Q file.inode/hard_links only +SURE; O, I, G none +SURE (not in the schemas or code read) |
| 8 | xattr | O fileextendedattribute +SURE; Q extended_attributes -GUESS (macOS-focused); I none -GUESS |
| 9 | git config (one scope) | nobody natively; text-file tests only |
| 10 | git var (resolved chain) | nobody |
| 11 | sysctl `kernel.pid_max` | O sysctl; Q system_controls (current_value and config_value); I kernel_parameter; G kernel-param +SURE |
| 12 | `net.ipv4.conf.eth0.forwarding` | same as 11, keyed by name only; netns-blind except through the osquery vantage -GUESS |
| 13 | ip_forward vs conf.all | nobody relates the two names +SURE (name-keyed) |
| 14 | dpkg status | O dpkginfo; Q deb_packages.status; I package installed (config-files is not installed) +SURE |
| 15 | dpkg version | O dpkginfo evr; Q version; I version; G versions +SURE |
| 16 | apt-mark hold | I be_held (from the dpkg Status word) +SURE; Q status column -GUESS; O none -GUESS |
| 17 | login shell | O password (unix); Q users.shell; I user shell; G user +SURE / -GUESS for G |
| 18 | `id -u` | all +SURE |
| 19 | group members | Q user_groups; I group/etc_group; G group -GUESS |
| 20 | `passwd -S` lock | O shadow; Q shadow; I shadow -GUESS (names seen) |
| 21 | interface addresses | O interface; Q interface_addresses; G interface addrs +SURE |
| 22 | the netdev | same as 21, keyed by name (ifindex not exposed in what I read) -GUESS |
| 23 | mount options | O partition (mount options); Q mounts.flags; I mount; G mount opts/vfs-opts +SURE |
| 24 | docker `.State.Status` | Q docker_containers +SURE; I only via the separate inspec-docker pack (not in core) +SURE; O none |
| 25 | container health | Q docker_container_* -GUESS |
| 26 | kubectl jsonpath | none in core; OVAL has a stale kubernetes branch (board discussion #198, not pulled) |
| 27 | `dig +short A` | G dns (with a server attribute); I host -GUESS; Q none (events only) |
| 28 | crontab line | Q crontab; I crontab +SURE / -GUESS |
| 29 | ufw rule N | nobody (positional catalogs unmodelled) +SURE for O/Q/I name lists |
| 30 | hostname vs /etc/hostname | Q system_info; O uname; Facter hostname (gethostname) +SURE; the two forms are never related |
| 31 | nft rule and its file line | I nftables -GUESS; Q iptables (legacy only) +SURE; O none (networkfirewall_test is an open proposal) |
| 32 | swapon entry and its fstab line | I etc_fstab -GUESS; nobody relates the two |
| 33 | cert enddate/serial | I x509_certificate -GUESS; Q certificates/curl_certificate -GUESS |
| 34 | getenforce | O sestatus; Q selinux_settings; I selinux +SURE (names) |
| 35 | timezone | I timezone; Q time -GUESS |
| 36 | update-alternatives | G file linked-to, using `/etc/alternatives/mta` as its own example +SURE; nobody natively |
| 37 | printenv HOME in a process | O environmentvariable58(pid); Q process_envs(pid) +SURE |
| 38 | one PATH entry | O environmentvariable58 plus the split function -GUESS; I os_env -GUESS |
| 39 | authorized_keys line | Q authorized_keys (key_file, uid) +SURE; I none in core -GUESS |
| 40 | nameserver line | Q dns_resolvers (id = order) +SURE |
| 41 | iptables default policy | Q iptables.policy +SURE; I iptables -GUESS |
| 42 | SQL cell and its column | O sql512; I postgres_session/mysql_session -GUESS |
| 43 | pip show | Q python_packages; I pip -GUESS |
| 44 | INI key | I ini/parse_config; O textfilecontent54 -GUESS |
| 45 | JSON `.foo.bar` | I json; G matching/gjson; O yamlfilecontent/xmlfilecontent (JSON: -GUESS) |
| 46 | lsblk mountpoint | Q block_devices/mounts; O partition -GUESS |
| 47 | lvs lv_active + VG metadata | nobody seen -GUESS |

Nobody covers 9, 10, 13 (as a relation), 29, 30/31/32 (as persisted↔live pairs), or 47. Every tool keys every item by its natural name. The only exceptions: osquery exposes inodes, `upid` and namespaces as data, and OVAL exposes a pid for environment variables.

## Citations

OVAL spec line numbers (`L…`) are of the text conversion of the .docx (see Tooling problems); section names are given alongside.

> [A-oval-language-specification-5112-2016]:L2538-2553 §4.5.11 FlagEnumeration (relevance: +1:SURE)
> error — This value indicates that an error prevented the determination of the existence of OVAL Items on the system.
> complete — This value indicates that every matching OVAL Item on the system has been identified and represented in the OVAL System Characteristics. It can be assumed that no additional matching OVAL Items exist on the system.
> incomplete — ... It cannot be assumed that no additional matching OVAL Items exist on the system.
> does not exist — This value indicates that no matching OVAL Items were found on the system.
> not collected — This value indicates that no attempt was made to collect OVAL Items on the system.
> not applicable — This value indicates that the specified OVAL Object is not applicable to the system under test.

> [A-oval-language-specification-5112-2016]:L2554-2565 §4.5.12 StatusEnumeration (relevance: +1:SURE)
> error — ... there was an error collecting an OVAL Item or a property of an OVAL Item.
> exists / does not exist / not collected — This value indicates that no attempt was made to collect an OVAL Item or a property of an OVAL Item.

> [A-oval-language-specification-5112-2016]:L3147-3162 §4.6.15 ResultEnumeration (relevance: +1:SURE)
> unknown — This value indicates that it could not be determined if the conditions of the evaluation were satisfied.
> error — This value indicates that an error occurred during the evaluation.
> not evaluated — This value indicates that a choice was made not to perform the evaluation.
> not applicable — This value indicates that the evaluation being performed does not apply to the given platform.

> [A-oval-language-specification-5112-2016]:L3555-3568 §5.3.2 Determining the Final OVAL Test Evaluation Result (relevance: +1:SURE)
> If the OVAL Object, referenced by an OVAL Test, cannot be found in the Collected Objects section, the final result of the OVAL Test MUST be ‘unknown’.
> If the flag value is ‘error’, the final result of the OVAL Test MUST be ‘error’.
> If the flag value is ‘not collected’, the final result of the OVAL Test MUST be ‘unknown’.
> If the flag value is ‘not applicable’, the final result of the OVAL Test MUST be ‘not applicable’.
> If the check_existence property has a value of ‘none_exist’ and one or more OVAL Items, referenced by the OVAL Object, have a status of ‘exists’, the final result of the OVAL Test MUST be ‘false’. ... Otherwise, the final result of the OVAL Test MUST be ‘unknown’.

> [A-oval-language-specification-5112-2016]:L3249-3250 §5.2.4 Unique Items (relevance: +1:SURE)
> OVAL Items are differentiated by examining each OVAL Item’s name and each of the OVAL Item’s entity names and values. Each OVAL Item MUST represent a unique system data artifact. No two OVAL Items within an OVAL System Characteristics Model can be the same.

> [A-oval-language-specification-5112-2016]:L3239 §5.2.2 Item References (relevance: -0:SUSPECT)
> A given OVAL Item MAY be referenced by one or more objects. This situation will occur when two distinct OVAL Objects identify overlapping sets of OVAL Items.

> [A-oval-language-specification-5112-2016]:L3683 §5.3.3 Set Evaluation (relevance: +1:SURE)
> Two OVAL Items should be considered identical if all OVAL Item attributes match, except possibly the id, and all the attributes and values of every corresponding OVAL Item Entity also match.

> [A-oval-language-specification-5112-2016]:L3586 §5.3.3 OVAL Object Evaluation (relevance: +1:SURE)
> The methodology used to collect the system state information for the OVAL Items is strictly an implementation detail. Regardless of the chosen methodology, the same OVAL Items MUST be collected on a system for a given OVAL Object except when the flag for the collected OVAL Object has a value of ‘incomplete’.

> [A-oval-language-specification-5112-2016]:L3875 §5.3.4.1.5 Final Result of an OVAL State Entity Evaluation (relevance: +1:SURE)
> In the event that there is no corresponding entity appearing in the OVAL Item, it is equivalent to there being a single corresponding entity with a status property value of “not collected” (i.e., evaluation proceeds as though the item entity was not collected; non-existent OVAL Item entities should appear with a status property value of “does not exist”).

> [A-oval-language-specification-5112-2016]:L1108-1111 §4.3.12 TestType check_existence (relevance: -0:SUSPECT)
> Specifies how many OVAL Items must exist, on the system, in order for the OVAL Test to evaluate to true. When used in this context a value of ‘all_exist’ is equivalent to a value of ‘at_least_one_exists’.

> [A-oval-unix-definitions-schema-2026]:L169-172 file_object (relevance: +1:SURE)
> The file_object will collect all UNIX file types (directory, regular file, character device, block device, fifo, symbolic link, and socket). ... The set of files to be evaluated may be identified with either a complete filepath or a path and filename. Only one of these options may be selected.

> [A-oval-unix-definitions-schema-2026]:L253 file_state (relevance: +1:SURE)
> The file_state element defines the different metadata associate with a UNIX file. This includes the path, filename, type, group id, user id, size, etc.
(The entity list at L259-457 has no inode or device entity; a_time/c_time/m_time, size, suid..oexec, has_extended_acl only.)

> [A-oval-unix-definitions-schema-2026]:L559-562 FileBehaviors recurse_file_system (relevance: +1:SURE)
> The value of 'local' limits the search scope to local file systems (as opposed to file systems mounted from an external system). The value of 'defined' keeps any recursion within the file system that the file_object (path+filename or filepath) has specified. ... The default value is 'all' meaning to search all available file systems for data collection.

> [A-oval-unix-definitions-schema-2026]:L2466 symlink_object (relevance: +1:SURE)
> The resulting item identifies the canonical path of the link target (followed to its final destination, if there are intermediate links), an error if the link target does not exist or is a circular link (e.g., a link to itself). If the file located at filepath is not a symlink, or if there is no file located at the filepath, then any resulting item would itself have a status of does not exist.

> [A-oval-linux-definitions-schema-2026]:L157, L870 dpkginfo / rpminfo (relevance: +1:SURE)
> A dpkginfo object consists of a single name entity that identifies the package being checked.
> A rpm info object consists of a single name entity that identifies the package being checked.

> [A-oval-linux-definitions-schema-2026]:L2696 systemdunitproperty unit (relevance: -0:SUSPECT)
> The unit entity refers to the full systemd unit name, which has a form of "$name.$type". For example "cupsd.service". This name is usually also the filename of the unit configuration file located in the /etc/systemd/ and /usr/lib/systemd/ directories.

> [A-openscap-systemdunitproperty-probe-2026]:L226-230, L280-286, L301 (relevance: +1:SURE)
> if (probe_entobj_cmp(vars->unit_entity, se_unit) != OVAL_RESULT_TRUE) { /* Do nothing, continue with the next unit */
> "DBus connection failed, could not identify systemd units." ... probe_cobj_set_flag(..., ctx->offline_mode == PROBE_OFFLINE_NONE ? SYSCHAR_FLAG_ERROR : SYSCHAR_FLAG_NOT_COLLECTED);
> get_all_systemd_units(dbus_conn, unit_callback, &vars);   (systemdshared.h L119-129: method "ListUnitFiles")

> [A-oval-linux-definitions-schema-2026]:L62 apparmorstatus_object (relevance: -1:GUESS)
> There is actually only one object relating to AppArmor Status and this is the system as a whole. Therefore, there are no child entities defined.

> [A-oval-independent-definitions-schema-2026]:L634 environmentvariable58 pid (relevance: +1:SURE)
> The process ID of the process from which the environment variable should be retrieved. If the xsi:nil attribute is set to true, the process ID shall be the tool's running process; for scanners with no process ID (e.g., an agentless network scanner), no corresponding items will exist.

> [A-oval-independent-definitions-schema-2026]:L1003, L1157-1166 shellcommand (relevance: +1:SURE)
> IMPORTANT! - Since this test requires the running of code supplied by content and since OVAL interpreters commonly run with elevated privileges, significant responsibilty falls to the content author to DO NO HARM to the target system.
> The ShellCommandBehaviors complex type defines behaviors that allow content authors to determine when a shellcommand item may have a status set to 'error'. By default all shellcommands are set to 'exist', with any error data captured as part of the stderr_line elment and exit_status element.
> 'error_if_exit_status_not_0' enables the OVAL interpeter to set the corresponding shellcommand system-characteristicsobject with a flag of 'error' if exit_status is not set to 0. The default is false.

> [B-oval-issue-107-symlink-target-2013]:L{107a} (relevance: +1:SURE)
> Pretend the link _is_ the ultimate target, and assign item entities accordingly. This allows definitions to be totally independent of symlinking.
> if you are not preserving the "illusion" as described at the bottom, then in "following" the link to its target, you are switching files. For example, a link could point to a link owned by someone else.

> [B-oval-issue-83-check-existence-2013]:L{83a} (relevance: -0:SUSPECT)
> In general, you can't know what should have been collected, so you can't compare that against what was actually collected to determine whether you got everything. In the absence of errors, and ignoring bugs, whatever the scanner found must be interpreted to be all there was to find.

> [B-oval-issue-301-not-applicable-result-2018]:L{301a} (relevance: +1:SURE)
> There are entities with the applicability_check attribute, but at the moment the check is powerless to impact a result, because it was never completely thought out how it was supposed to work.
> Joval returns a status of "Not Applicable" for any linux:rpm_object on AIX when RPM is not installed. ... Other interpreters might set a flag of "does not exist" for rpm_objects on these platforms.
> because a "Not Applicable" result is overwhelmed in all cases when it's combined with any other result, it would be necessary to modify the truth tables for OperatorEnumeration

> [B-oval-community-symlink-file-object-discussion-2025]:L{261a} (relevance: +1:SURE)
> Being a symlink on linux though, it had wide open (777) permissions, and that is what Joval collected. This caused the test to fail, and by the OVAL spec, this appears to be correct behavior.
> If you blindly followed symlinks and treated them as files, you could hide an insecure sitation. ... you would also need to verify that the chain of parent directories was also sufficiently restricted
> what if the symlink item was modified to list any 'chain links' as new elements [0-n]? ... Just list out each link in the chain if a chain exists.
> if the file path represented a direct link to a canonical_path, then you'd still need a `<chainlink status="does not exist" />` to signify non-existence, otherwise the absence of the child entity simply means it wasn't collected.

> [B-oval-community-shellcommand-status-discussion-2025]:L{281a} (relevance: -0:SUSPECT)
> Internally we currently have the item created and it always has a status of 'exists' and any error information is captured in the item, but we do not have enough knowledge of the intent of the command to know if it constitutes marking the item as 'error'.

> [A-nist-ir7275r4-xccdf-12-2011]:p.43 §6.6.4.2 Table 26 (relevance: +1:SURE)
> error — The checking engine could not complete the evaluation, therefore the status of the target's compliance with the <xccdf:Rule> is not certain. This could happen, for example, if a testing tool was run with insufficient privileges
> unknown — The testing tool encountered some problem and the result is unknown.
> notapplicable — The <xccdf:Rule> was not applicable to the target of the test.
> notchecked — ... designed for <xccdf:Rule> elements that have no <xccdf:check> elements or that correspond to an unsupported checking system.

> [A-nist-ir7275r4-xccdf-12-2011]:p.28 §6.2.8 Tables 15-16 fix attributes (relevance: -0:SUSPECT)
> disruption — An estimate of the potential for disruption or operational degradation that the application of this fix will impose on the target. ... unknown (disruption not defined) (default) / low / medium / high
> reboot — Whether or not remediation will require a reboot or hard reset of the target.

> [A-osquery-table-schema-5231-2026]:L9807-9840 file (relevance: +1:SURE)
> "name":"hard_links", "description":"Number of hard links" ... "name":"symlink", "description":"1 if the path is a symlink, otherwise 0" ... "name":"symlink_target_path", "description":"Full path of the symlink target if any"

> [A-osquery-table-schema-5231-2026]:L10023-10028 file.pid_with_namespace (relevance: +1:SURE)
> "name":"pid_with_namespace", "description":"Pids that contain a namespace", "type":"integer", "hidden":true

> [A-osquery-table-schema-5231-2026]:L20288-20289 processes.upid (relevance: +1:SURE)
> "name":"upid", "description":"A 64bit pid that is never reused. Returns -1 if we couldn't gather them from the system."

> [A-osquery-table-schema-5231-2026]:L23913-23923 system_controls (relevance: -0:SUSPECT)
> "name":"current_value", "description":"Value of setting" ... "name":"config_value", "description":"The MIB value set in /etc/sysctl.conf"

> [A-osquery-table-schema-5231-2026]:L5446-5447 deb_packages.admindir (relevance: -1:GUESS)
> "name":"admindir", "description":"libdpkg admindir. Defaults to /var/lib/dpkg"

> [A-osquery-file-table-implementation-2026]:L389-407 (relevance: +1:SURE)
> if (lstat(path.string().c_str(), &link_stat) < 0) { // Path was not real, had too may links, or could not be accessed. return; }
> if (stat(path.string().c_str(), &file_stat)) { file_stat = link_stat; }
> r["device"] = BIGINT(file_stat.st_rdev);

> [B-osquery-issue-6209-container-table-access-2020]:L1105-1114 (relevance: +1:SURE)
> In practice it works by having a pid_with_namespace column, which should contain pids that are in the same mount namespace of the container one wants to query. ... the child receives the job, takes all the values given in the pid_with_namespace constraint, retrieves the fd of the mount namespace under "/proc/<constraint pid>/ns/mnt", then switches to it.

> [B-osquery-issue-6319-empty-column-casting-2020]:L834 (embedded JSON) (relevance: -0:SUSPECT)
> That happens when a table do not return any value for that column, which can be either "ok", in the sense that there's effectively no data to be returned (although, it would be better to explicitly set it), or can be a bug.
> The times I’ve looked at it, it’s been platform skew. Some columns don’t mean anything on a platform. And the platform table implementation returns nothing, which causes the cast error.

> [B-osquery-issue-7306-file-table-incompleteness-2021]:L{7306a} (relevance: -0:SUSPECT)
> Not returning a full data list (or not throwing a warning message?) about the data could lead to inaccurate results/decisions?
> I don't see this as a incompleteness in the file table, but as a feature request for a different sort of interaction.
> Meanwhile I'm going to close this as WONTFIX.

> [A-train-file-follow-symlink-default-2026]:L11, L50-56, L113-119 (relevance: +1:SURE)
> def initialize(backend, path, follow_symlink = true)
> def source; if @follow_symlink; self.class.new(@backend, @path, false) ...
> def path; if symlink? && @follow_symlink; link_path ...

> [B-inspec-issue-665-symlink-permissions-2016]:L{665a} (relevance: +1:SURE)
> the way that `file` currently works, is that it will check the item you are pointing to, without resolving any fancy links ... Users might be more interested in always testing the target, i.e. having it resolved. ... If we decide to change this behavior, let's do it in `train` and make it applicable everywhere.

> [A-inspec-service-resource-source-2026]:L119-125, L331-335, L356-357 (relevance: +1:SURE)
> when "ubuntu" ... if version < 15.04 Upstart.new(inspec, service_ctl) else Systemd.new(inspec, service_ctl)
> # Some systems may not have a `.service` file for a particular service which causes the `systemctl is-enabled` check to fail despite the service being enabled. In that event we fallback to `sysv_service`.
> # LoadState values eg. loaded, not-found / installed = params["LoadState"] == "loaded"

> [A-inspec-package-resource-source-2026]:L149-159 (relevance: +1:SURE)
> cmd = inspec.command("dpkg -s #{package_name}") / return {} if cmd.exit_status.to_i != 0
> # If the package is removed and not purged, Status is "deinstall ok config-files" with exit_status 0 / # If the package is purged cmd fails with non-zero exit status

> [A-inspec-custom-resources-docs-2026]:L76-83, L139-145 (relevance: -0:SUSPECT)
> `skip_resource` : A resource may call this method to indicate that requirements aren't met. All tests that use this resource will be marked as skipped.
> `resource_id` : An instance method. Place logic here to determine the unique identifier for a resource
> # If the file doesn't exist, skip all tests that use example_config / raise Inspec::Exceptions::ResourceSkipped, "Can't read config at #{@path}"

> [A-testinfra-service-module-2026]:L92, L179, L186-193, L207 (relevance: +1:SURE)
> if host.file("/run/systemd/system/").is_directory or ( ...
> cmd = self.run_test('systemctl list-unit-files | grep -q "^%s"', self.name)
> # 4: Unable to determine status (no such unit) / out = self.run_expect([0, 1, 3, 4], "systemctl is-active %s", self.name) ... return out.rc == 0
> f"Unable to determine state of {self.name}. Does this service exist?"

> [A-goss-gossfile-reference-2026]:L337-342, L585, L592-594 (relevance: -0:SUSPECT)
> /etc/alternatives/mta: ... filetype: symlink ... linked-to: /usr/sbin/sendmail.sendmail
> this check uses the `--package <format>` parameter passed on the command line.
> Goss might consider your port to be listening on `tcp6` rather than `tcp`, try running `goss add port ..` to see how goss detects it.

> [A-facter-core-facts-schema-2026]:L475, L1919-1924 (relevance: -1:GUESS)
> description: Return the identity information of the user running facter.
> uuid: ... Linux: parse the contents of `/sys/class/dmi/id/product_uuid` to retrieve the system product unique identifier.

> [B-ohai-shard-seed-plugin-2016]:L33-36, L66, L83 (relevance: -0:SUSPECT)
> def default_sources ... when "linux", "darwin", "windows" %i{machinename serial uuid}
> sources = Ohai.config[:plugin][:shard_seed][:sources] || default_sources
> shard_seed: Unable to generate seed! Either ensure 'dmidecode' is installed, or use 'Ohai.config[:plugin][:shard_seed][:sources]' to set different sources.

## Leads not pulled

- Serverspec / specinfra OS and backend detection. It is the ancestor of InSpec's resource model and would add a third provider-resolution strategy. Not pulled for budget. Find it at `github.com/mizzy/specinfra`, `lib/specinfra/helper/detect_os*`.
- The OVAL developer mailing-list threads cited in #107 (nabble "UNIX Symlink Capabilities", "Question about link traversal behavior for unix file_object", "The unix file test"). They are the pre-2013 litigation record. The nabble mirror is likely dead. Try the Wayback Machine.
- OVAL-Community discussion #198 (a kubernetes extension for OVAL 6.0) and #165 (usefulness of the existing tests, with a test-usage spreadsheet from the 2024-09-26 board meeting). The spreadsheet would give deployment-scale evidence per test. Find them via `gh api graphql` on OVAL-Community/OVAL discussions.
- OVAL 6.0 definition-structure vote (OVAL-Community/OVAL-Board `board_votes/2024-11-07_OVAL_6.0_Definition_Structure_Vote.md`). It may re-litigate object identity. Not read.
- The OVAL windows spec, for junction/reparse-point handling (the Windows counterpart of symlink_test that #107 mentions). Out of the Linux-centred remit.
- ohai `linux/virtualization.rb`. It is read but not registered: container and VM detection by heuristics (`which docker` means role "host"). Its vantage evidence is weak.
- Kolide, "Why You Can't Trust Your NULLs in Osquery" (kolide.com blog). Read in full. It gives practitioner evidence that TEXT columns render absence as `''` and numeric columns as NULL, and that one table emits the literal string "null". Not registered: it is a commercial vendor blog (grade C at best), undated on the page, and it duplicates [B-osquery-issue-6319-empty-column-casting-2020].
- InSpec issues #5209 (be_symlink and be_file both true for one path) and #3099 (local :symlink vs SSH :directory). Read. #3099 is folded into [B-inspec-issue-665-symlink-permissions-2016]'s grading. Neither is registered separately.

## Search log

- gh · `repo:OVALProject/Language` issues: symlink / hard link / inode / does not exist / not applicable / mount / container / namespace / file system / dpkginfo / systemd · kept #107, #301, #83 (read #105, #51, #82, #231)
- gh · `repo:OVAL-Community/OVAL` issues (same terms) plus the GraphQL list of the newest 60 discussions · kept #261, #281
- kagi · 'OVAL Language Specification 5.11.2 pdf', 'OVAL unix file_object symlink behaviors follow link item', 'osquery file table symlink hardlink issue' · 3 (discussion #261, osquery #7306/#7291)
- kagi · 'osquery pid_with_namespace container tables docs', 'Introducing osquery Facebook engineering 2014', 'osquery file table symlink follow stat lstat' · 2 (#6428 leading to #6209)
- kagi · 'osquery issue surface table generation errors ...', 'osquery table errors empty results ...' · 1 (Kolide lead leading to #6319)
- github semantic issue search · osquery "table returns empty results instead of an error" · 0
- gh · `repo:inspec/inspec` symlink mode / follow symlink / link_path / service not found / package provides · read #665, #5209, #2106, #3099
- gh · `repo:inspec/train` follow_symlink / symlink · 0
- curl · OpenSCAP `systemdunitproperty_probe.c` + `systemdshared.h` (to settle OVAL's schema silence on missing units) · 1
- direct reads (seeded): osquery specs/*.table, osquery-site schema JSON, InSpec docs and resources, Train file backends, Testinfra modules, Goss docs and system/file.go plus goss issue 149, facter schema, ohai plugins, NIST IR 7275r4

## Tooling problems

- The OVAL Language Specification exists only as `.docx`. `new-source.sh` archived it under a `.html` extension, as a binary zip. Its line cites (`L…`) point to my unzip plus tag-strip text conversion, not the archived bytes. The section name is given alongside every cite so a reader can find it in Word.
- `register.sh` lock contention: roughly 10 lanes compete for a non-FIFO mkdir lock, and single registrations waited several minutes. There was no timeout.
- GitHub code search hit its rate limit (HTTP 403) partway through locating osquery spec paths. I fell back to guessing raw paths, and every one resolved.
