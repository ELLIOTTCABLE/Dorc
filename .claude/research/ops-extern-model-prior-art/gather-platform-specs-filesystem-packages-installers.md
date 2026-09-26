# gather-platform-specs-filesystem-packages-installers — first-party fs / package / installer / block-device identity specs

Lane 6 of ten. 311 read at `63e49f29`. Every grade is `graded-by: subagent`.

> Conductor's note: drive C: filled before the lane could write more than a stub; the lane
> handed back its findings inline and the conductor banked them here verbatim. The candidate
> table and line-numbered Citations are held in the lane's own context and are owed; the lane's
> entry JSONs for the unregistered slugs are in the scratchpad. Everything below is the lane's
> text and the lane's grading.

## Findings (most important first)

1. **RFC 8881 spells out 311's two warrants as separate promises.** Equal filehandles "MUST refer to the same file" (unique referent). One-to-one is only SHOULD, and a separate per-filesystem attribute, `unique_handles`, is "TRUE, if two distinct filehandles are guaranteed to refer to two different file system objects" (unique name, declared by the server, absent by default). "Clients MUST use filehandle comparisons only to improve performance, not for correct behavior." Whether fileids and handles carry over a migration is declared by the server as "same fileid class" / "same handle class"; otherwise the client "is forced to assume that no object has been renamed". fsids are "only unique on a per-server basis". +SURE [A-ietf-rfc8881-nfs41-fileid-fsid-attributes-2020]
2. **POSIX pins (st_dev, st_ino) as unique only "At any given time".** Both halves are reusable: st_ino after the last unlink, st_dev on remount. The rule was re-litigated in Austin Defect 1314 ("uniqueness can only guaranteed at an instant of time"). This coincides with 311's recycled-key horizon and its refusal of `:root` for inode tokens. +SURE [A-posix-sys-stat-file-identity-2024] [A-austin-group-defect-1314-file-identity-2020]
3. **overlayfs's own documentation refutes stable inode identity.** st_ino and st_dev "can change over the lifetime of a non-directory object". Without `index`, a copy-up "will 'break' the link". Changes to the lower layer are "undefined". (The phrase "over-loads the high bits" about xino is Neil Brown's in the LWN solutions article, not the overlayfs document; corrected by the citation pass, see Citations.) This coincides directly with GOTCHAS identity-tokens-perish-on-write-not-only-on-rename and backing-is-not-presenting, and with the `311u` refuted-terminal-tokens shape. +SURE [A-linux-overlayfs-documentation-2025]
4. **A file handle's generation number sees a recycled inode that st_ino cannot.** The man-page example reissues the same inode number and the old handle goes ESTALE. Viro in the btrfs thread: "two different file handles [can] refer to the same file". mount_id "should not be treated as a persistent identifier". Brown on btrfs: "with its current framing the problem is unsolvable"; the adopted uniquifier "still is not guaranteed" unique. +SURE [A-linux-name-to-handle-at-man-2026] [B-lwn-btrfs-inode-number-epic-problem-2021] [B-lwn-btrfs-inode-number-epic-solutions-2021]
5. **dpkg triggers match by spelling, not by identity.** Interest is "matched against the path included in the triggering package, not against the truename … Only textually identical filenames … are guaranteed to match". Inode-only replacement is "not guaranteed to cause trigger activation". Processing is deferred to the end of the run in undefined order. This supports the counter-thesis ("the name is the thing") and GOTCHAS 24/25. +SURE [A-dpkg-triggers-specification-2025]
6. **Two-party file protocols between package system and admin:**
   - Debian: conffile vs configuration file are "not interchangeable". The MD5 rule: whichever single party changed the file wins; if both did, the admin is prompted; an admin-deleted conffile is never recreated.
   - Policy footnote 12 bans hard links to conffiles because editors and dpkg "break the link".
   - update-alternatives infers manual mode from observed admin divergence.
   - RPM: `.rpmnew` / `.rpmsave` / `.rpmorig` files, plus a per-file `%verify(not …)` list of cells the packager expects to drift.
   - Diversions and statoverride are the other two protocols.
   +SURE [A-debian-policy-files-conffiles-statoverride-2025] [B-debian-policy-conffile-handling-appendix-2025] [A-dpkg-update-alternatives-manual-2025] [A-rpm-spec-file-directives-2026] [B-debian-policy-diversions-appendix-2025]
7. **Windows Installer component rules make unique naming a social contract across strangers.** "Never create two components that install a resource under the same name and target location … across applications, products, product versions, and companies." Breaking it "damages" reference counting, and removing either component deletes the shared file. This is a battle-tested case of a wrong DISJOINT causing real destruction. +SURE [A-msi-organizing-applications-into-components-2021] [A-msi-changing-the-component-code-2021] [A-msi-component-rules-broken-2021]
8. **OCI keeps several identifiers for one image.** ImageID (hash of the config), DiffID (explicitly "do not confuse" with the layer digest), and ChainID (an ordered compound hash, the analogue of 311's composite sort). A tag is "a pointer … zero, one, or many tags" per digest. Digest equality gives unique referent but not unique name (algorithms may vary). +SURE [A-oci-image-spec-config-identifiers-2025] [A-oci-image-spec-descriptor-digests-2025] [A-oci-distribution-spec-tags-digests-2025]
9. **Block and LVM naming.** Nine or more udev naming schemes point at one device. One rule comment admits an NVMe symlink "might get overridden". by-diskseq exists for race-free reopening, and by-loop-inode names a loop device by its backing file's outer inode. LVM: "different VGs with the same name can appear … after moving disks", resolved by `--select vg_uuid`; LV names are unique only within a VG. +SURE [A-systemd-persistent-storage-udev-rules-2025] [B-archwiki-persistent-block-device-naming-2026] [A-lvm-manual-unique-names-2025]
10. **Mount propagation and FUSE.** Under a shared mount, a mount or umount lands in every peer namespace, and systemd makes mounts shared by default. FUSE: "The filesystem does not have to guarantee uniqueness" of st_ino. +SURE [A-linux-mount-namespaces-man-2026] [B-libfuse-high-level-api-use-ino-2025]

**Counter-thesis verdict:** partly confirmed. No platform document models identity. Each specifies a mechanism and then refuses the identity promise exactly where 311 withholds warrants. Two bodies go further and litigate: NFS (unique_handles and fileid classes) and MSI (component rules). ~SUSPECT

## Candidate table

Rebuilt from the lane's entry JSONs (the lane's own context did not survive the hand-off); grades and
analogues are the entries' text, condensed.

| [slug] | what it is | exhaustive / broad / abstract / battle-tested | 311 analogues | punts | altitude |
|---|---|---|---|---|---|
| [A-ietf-rfc8881-nfs41-fileid-fsid-attributes-2020] | NFSv4.1 fsid, fileid, unique_handles, mounted_on_fileid, transitions | exhaustive for NFS object identity / one protocol / abstract attributes / 20 years of multi-vendor interop (3530 → 5661 → 8881) | unique-referent (MUST) vs unique-name (`unique_handles`, server-declared); fileid in fsid in server = mToken in mParent-Store; `:corresponds` declared by the transition owner (fileid / handle class); covering-mount two identities | handle comparison "only to improve performance"; fsids not to be shown to applications | protocol |
| [A-posix-sys-stat-file-identity-2024] | POSIX `<sys/stat.h>` file identity | exhaustive for (st_dev, st_ino) / universal / normative / every POSIX system | inode mToken in the st_dev mParent-Store; no `:root`; recycled-mKey horizon; hard links fail unique-name | uniqueness only "at any given time" | standard |
| [A-austin-group-defect-1314-file-identity-2020] | Austin Group defect behind the identity paragraph | narrow / litigation record / accepted text | the witness horizon: uniqueness only when the reference is produced; attach order changes st_dev | removable and network media argued, not solved | standards debate |
| [B-linux-inode-man-2026] | inode(7) | narrow overview | inode number unique only within a filesystem; hard links stay in one store | restates POSIX | kernel ABI |
| [A-linux-overlayfs-documentation-2025] | in-tree overlayfs documentation | exhaustive for overlay identity / one fs / concrete / in production (containers) | identity-tokens-perish-on-write (copy-up); backing-is-not-presenting; `311u` refuted terminal tokens; xino composite id | lower-layer changes "undefined"; xino overflow falls back silently | fs spec |
| [A-linux-name-to-handle-at-man-2026] | name_to_handle_at(2) / open_by_handle_at(2) | exhaustive for handles / Linux-only / concrete | handle generation sees a recycled mKey st_ino cannot; mount_id is a recycled token; move invalidates one handle species only | no handles for /proc, /sys, many network fs | kernel ABI |
| [B-lwn-btrfs-inode-number-epic-problem-2021] | LWN on btrfs subvolume st_dev / st_ino | one case / secondary / deployed failure | hidden mParent-Store (subvolume) without a user-visible token; nfsd collapses stores, so a client mount is never aliases-nothing-else | none; problem statement | kernel debate |
| [B-lwn-btrfs-inode-number-epic-solutions-2021] | LWN synthesis of four patch series | one case / litigated by Brown, Viro, Hellwig, Goldstein, Bacik | two warrants separate (handles lack unique-name); several primary keys for one referent; refuted terminal tokens | "unsolvable" in current framing; uniquifier not guaranteed unique | kernel debate |
| [A-dpkg-triggers-specification-2025] | dpkg triggers spec | exhaustive for triggers / one tool / precise / dpkg ≥ 1.14.17 | name-is-the-thing punt; may-write entailment across packages; effects land after the command; triggers-pending / triggers-awaited states | truename not matched; inode-only replacement may not activate; order undefined | tool spec |
| [B-dpkg-manual-package-states-2025] | dpkg(1) states, selections, flags | exhaustive enumeration / one tool | state vs selection as two mSorts over one record; one name, several instances (REFCOUNT); `--verify` is md5-only | self-declared "particularly inadequate" on install/remove | CLI |
| [A-debian-policy-package-relationships-2025] | Debian Policy ch. 7 | exhaustive for relationships / one distro family / normative / ~30 years | Provides = secondary mScheme without unique-name; Replaces re-assigns file ownership; file takeover demands real names | config-files state is not presence | policy |
| [A-debian-policy-files-conffiles-statoverride-2025] | Debian Policy §10.5, §10.7, §10.10 | exhaustive for conffiles / normative | two-party protocol (B4a); committee law (one owning package); hard-link ban as perish-on-write; statoverride as admin override | §10.10.1 self-declared non-policy | policy |
| [A-debian-policy-maintainer-scripts-2025] | Debian Policy ch. 6 | exhaustive state machine / normative | named not-installed states (Config-Files); a package "disappears" when fully overwritten; may-write entailment; arbitrary root shell | follows an existing directory symlink rather than replacing it | policy |
| [B-debian-policy-conffile-handling-appendix-2025] | conffile three-way rule | narrow / non-normative appendix | the MD5 decision rule; admin-deleted as a distinct state; content hash as change witness | carried over from the old Packaging Manual | policy appendix |
| [B-debian-policy-diversions-appendix-2025] | dpkg-divert idiom | narrow / non-normative | third-party routing write on the file-list catalog; per-owner vantage | diverting conffiles unsupported; a window where the path does not exist | policy appendix |
| [A-dpkg-update-alternatives-manual-2025] | update-alternatives(1) | exhaustive for alternatives / one tool / stable model | two-hop emitted traversal; mode as a separate mSort inferred from divergence; committee law; B4a, B12 | out-of-order script calls "flip-flop" the manual state | CLI |
| [A-rpm-spec-file-directives-2026] | rpm-spec(5) `%files`, Epoch, Provides | exhaustive for file attributes / one tool family | per-file cells as mSorts; `%verify(not …)` as the owner's may-write declaration; `.rpmnew` / `.rpmsave` / `.rpmorig`; B4a | Epoch is an "artificial" override | tool spec |
| [A-msi-organizing-applications-into-components-2021] | MSI component rules | exhaustive for components / Windows / normative / since 1999, ICE-enforced | global GUID primary vs natural (path, name) key; unique-name imposed across companies; key path as a presence probe | a social contract, not checked across vendors | installer spec |
| [A-msi-changing-the-component-code-2021] | when a component code must change | narrow / normative | new referent vs new version of one referent is authored, not measured; §3.3 recreation under an old mKey | compatibility, not content, decides | installer spec |
| [A-msi-component-rules-broken-2021] | consequences of breaking the rules | narrow / first-party failure catalogue | a wrong DISJOINT destroys shared files; one key-path file versions the whole component; ref-counting as an aliasing guard | none; it lists the damage | installer spec |
| [A-oci-image-spec-config-identifiers-2025] | OCI ImageID, DiffID, ChainID | exhaustive for image ids / normative / every runtime | composite sort (ChainID over ordered layers); several primary keys for one referent; unique-referent without unique-name | none stated | format spec |
| [A-oci-image-spec-descriptor-digests-2025] | OCI descriptor digests | exhaustive for descriptors / normative | digest equality licenses SAME, inequality licenses nothing; mixed algorithms | canonicalization is MAY | format spec |
| [A-oci-distribution-spec-tags-digests-2025] | OCI distribution: tags vs digests | normative / conformance-tested | tag = mutable secondary mScheme into a digest; registry aliases blobs across repositories | returned digest MAY differ | protocol |
| [A-systemd-persistent-storage-udev-rules-2025] | 60-persistent-storage.rules | exhaustive for /dev/disk/by-* / code-as-spec / universal on systemd | nine-plus secondary mSchemes into one device; diskseq as a recycle-proof witness; by-loop-inode as backing-is-not-presenting | an NVMe symlink "might get overridden" | code |
| [B-archwiki-persistent-block-device-naming-2026] | ArchWiki persistent naming | broad / community / heavily used | per-scheme warrant scope (clone breaks unique-referent; labels break unique-name); positional catalog (partition numbers) | blkid without root may show cached data | community guide |
| [A-lvm-manual-unique-names-2025] | lvm(8) VALID / UNIQUE NAMES | narrow / first-party | unique-name scoped in mParent (LV in VG) but not at VG level; natural name falls back to UUID; tombstone fences name reuse | /dev/mapper names "might change between releases" | CLI |
| [A-linux-mount-namespaces-man-2026] | mount_namespaces(7) | exhaustive for propagation / Linux | may-write entailment across namespaces; mount namespace as the mRoute for paths; unshare as a lend; recycled peer-group ids; a directory with two parents | sharedsubtree.rst not read | kernel ABI |
| [B-libfuse-high-level-api-use-ino-2025] | fuse.h `use_ino` and friends | narrow / header comments | inode mToken with no warrant; kernel nodeid as a second private primary key | uniqueness not required; locks inaccurate with hard links | library API |

Breadth: strongest on file identity (POSIX, NFS, overlay, handles, btrfs, FUSE), the package / admin
two-party file protocols (dpkg, Policy, alternatives, RPM, MSI) and block-device naming. Thin on
OCI runtime state (images only) and on Apple / Windows filesystems.

## Citations

Line numbers are `grep -n` against the archived copy under `sources/` unless marked otherwise. HTML
archives are numbered as raw HTML lines. Three sources have no archived copy yet (they are in the
conductor's registration batch); they are cited by section, and by the lane's scratchpad reading copy
where one exists.

- Finding 1 [A-ietf-rfc8881-nfs41-fileid-fsid-attributes-2020] (`.txt`): equal handles "MUST refer to
  the same file" L4766; "Clients MUST use filehandle comparisons only to improve performance" L4768–4769;
  §5.8.1.10 `unique_handles` heading L5490, "TRUE, if two distinct filehandles are guaranteed to refer
  to two different" L5492; "same fileid class" L12112, L12156; "forced to assume that no object has been
  renamed" L12121; "only unique on a per-server basis" L12129; §5.8.2.23 `mounted_on_fileid` L5642,
  L5668.
- Finding 2 [A-posix-sys-stat-file-identity-2024]: "At any given time" L75; reuse of st_ino "after the
  last" link L77; st_dev reusable on unmount / mount L79; "uniquely identified by the combination of
  st_ino and st_dev" (the rationale text) L414. [A-austin-group-defect-1314-file-identity-2020]: "the
  uniqueness can only guaranteed at an instant of time" L283; the preceding "instant of time" argument
  L257. [B-linux-inode-man-2026]: "guaranteed to be unique only within a" filesystem L94.
- Finding 3 [A-linux-overlayfs-documentation-2025]: st_dev / st_ino change "over the lifetime of a
  non-directory object" L29; xino composes a unique id L41, uses the high inode bits L43, falls back on
  overflow L45–46; index keyed by lower file handles L249–257; copy-up "will 'break' the link" L645;
  lower-layer changes "undefined" L555, L671, L678. The "over-loads the high bits" quote is Brown's, in
  [B-lwn-btrfs-inode-number-epic-solutions-2021] L129, not in the overlayfs document; Finding 3 blends
  the two.
- Finding 4 [A-linux-name-to-handle-at-man-2026]: handles "compare the identity of filesystem objects"
  L131; ESTALE L287, L319; mount_id "should not be treated as a persistent identifier" L341–342; the
  delete-and-recreate example with the same inode number L390–394. (Reading copy
  `open_by_handle_at.2.txt`: L56, L177, L203, L223.) [B-lwn-btrfs-inode-number-epic-problem-2021]: find(1)
  aborts on duplicate inode numbers across subvolumes L142–159.
  [B-lwn-btrfs-inode-number-epic-solutions-2021]: "unsolvable" L123; "over-loads the high bits" L129;
  "two different file handles to refer to the same" file L162; uniquifier "still is not guaranteed" L278.
- Finding 5 [A-dpkg-triggers-specification-2025] (`.txt`): "not a general mechanism for filesystem
  monitoring" L280; "not against the truename" L286; "Only textually identical filenames" L287; "solely
  the inode number" L294. [B-dpkg-manual-package-states-2025]: install/remove descriptions
  "inadequate" L924; `--verify` not "any kind of security verification" L1156, L1753;
  DPKG_MAINTSCRIPT_PACKAGE_REFCOUNT L1996–1998.
- Finding 6 [A-debian-policy-files-conffiles-statoverride-2025]: "configuration file" vs "conffile" not
  "interchangeable concepts" L429; survivor "will take over the conffile" L537; footnote 12, editors
  "break the link" L822, dpkg "might break the hard link" L824. [A-debian-policy-maintainer-scripts-2025]:
  disappear procedure L274, L307, L458; "point of no return" L443, L475; "lobotomize the file list"
  L471. [B-debian-policy-diversions-appendix-2025]: "can bypass the diversion" L125; the window when the
  file "does not exist" L165.
  [B-debian-policy-conffile-handling-appendix-2025]: not archived and no reading copy; cited by section
  (Debian Policy appendix "Configuration file handling", ap-pkg-conffiles.html, the MD5 three-way rule
  and "a missing file needs to be kept that way").
  [A-dpkg-update-alternatives-manual-2025]: not archived and no reading copy; cited by section
  (update-alternatives(1) DESCRIPTION, the auto → manual switch on admin edit, and the maintainer-script
  call rules that otherwise "flip-flop").
  [A-rpm-spec-file-directives-2026]: not archived; reading copy `rpmspec.md` in the lane scratchpad
  (rpm-spec.5.scd): Epoch "Artificial versioning override" L227; `%ghost` L621; `missingok` L631;
  `.rpmorig` L635, `.rpmsave` L636, `.rpmnew` L639; `%verify` L679, `%verify(not size filedigest mtime)`
  example L696.
- Finding 7 [A-msi-organizing-applications-into-components-2021]: "applied across applications,
  products, product versions, and companies" L743; shared key path, "unable to distinguish" L744.
  [A-msi-component-rules-broken-2021]: key path file version "determines the version of the component"
  L755; old products "damaged" L758; "removal of either component removes the common resource" L762;
  "reference-counting mechanism is damaged" L765. [A-msi-changing-the-component-code-2021]: not archived;
  cited by section (the page's two lists: when the code must change, and when it must not).
- Finding 8 [A-oci-image-spec-config-identifiers-2025] (`.md`): config immutable because it changes the
  ImageID L23; "Do not confuse DiffIDs with layer digests" L31; ChainID L33–42; ImageID L85–89.
  [A-oci-image-spec-descriptor-digests-2025]: "collision-resistant hash" L72; canonicalization MAY L112;
  must not "modify existing content identifiers" L184. [A-oci-distribution-spec-tags-digests-2025]:
  "zero, one, or many tags" L74; returned digest "MAY differ" L173, L474; "MAY cross-mount the blob" L477.
- Finding 9 [A-systemd-persistent-storage-udev-rules-2025]: partition ID_FS_* import would give
  "inconsistencies" L26; NVMe "obsolete symlink that might get overridden" L49; by-diskseq "race-free"
  L165; by-loop-inode L169. [B-archwiki-persistent-block-device-naming-2026]: kernel names "switching
  around on each boot" L647; "disk cloning creates two different disks with the same name" L661; blkid
  "stale/cached data" L678; WWNs "fully persistent" L852. [A-lvm-manual-unique-names-2025]: /dev/mapper
  format "might change between releases" L267–268; "different VGs with the same name" L278; `--select
  vg_uuid` L287; "LV names are unique within a VG" L293.
- Finding 10 [A-linux-mount-namespaces-man-2026]: not archived; reading copy `mount_namespaces.7.txt`:
  propagation to "other mounts in the peer group" L47; peer-group IDs "may be recycled" L111; systemd
  remounts "as MS_SHARED on system startup" L508; locked mounts "may not be separated" L543, L549.
  [B-libfuse-high-level-api-use-ino-2025]: not archived; reading copy `fuse.h`: "have to guarantee
  uniqueness" L198; `use_ino` L204; st_ino "ignored except if the 'use_ino'" L359; lock results "may not
  be accurate" with hard links L696–697.
- [A-debian-policy-package-relationships-2025]: not archived and no reading copy; cited by section
  (Debian Policy ch. 7: §7.5 virtual packages, §7.6 Replaces, and the rule that file takeover names real
  packages). Table only; no Finding rests on it alone.

## Leads not pulled
- MultiarchSpec (wiki.ubuntu.com 504 Gateway Timeout).
- T10 SPC VPD page 0x83 and multipath `uid_attribute`.
- Apple HFS+/APFS directory hard links.
- `Documentation/filesystems/sharedsubtree.rst`.
- LVM VG metadata seqno (`vgs` / `lvmcache`).
- Microsoft SharedDLLs reference counting.

## Search log

Rebuilt from the `via` fields of the lane's entry JSONs. Discarded results were not recorded, so only
kept counts are given.

- (seeds) pubs.opengroup.org `sys_stat.h` → Austin Group defect 1314 via its link · 2 kept
- (seeds) man7.org: open_by_handle_at(2), inode(7), lvm(8), mount_namespaces(7) · 4 kept
- (seed) kernel `Documentation/filesystems/overlayfs.rst` (raw GitHub) · 1 kept
- (seed) rfc-editor.org RFC 8881 (§4.2.1, §5.8.1.9–10, §5.8.2.7, §5.8.2.23, §11.11.2–11.11.4.1) · 1 kept
- (seeds) debian.org Debian Policy ch. 6, ch. 7, ch. 10 → appendices ap-pkg-conffiles, ap-pkg-diversions
  → manpages.debian.org dpkg(1), update-alternatives(1) via their links · 7 kept
- (seed) git.dpkg.org `doc/triggers.txt` blocked by Anubis → `gh api repos/guillemj/dpkg/contents/doc/spec`
  · 1 kept
- (seeds) learn.microsoft.com MSI component rules → changing-the-component-code, component-rules-broken
  via links · 3 kept
- (seeds) github.com opencontainers image-spec `config.md`, `descriptor.md`; distribution-spec `spec.md`
  · 3 kept
- (seed) systemd `rules.d/60-persistent-storage.rules.in` → ArchWiki Persistent block device naming · 2
  kept
- (brief) libfuse `include/fuse.h` for FUSE / sshfs `use_ino` · 1 kept
- kagi · "lwn btrfs inode numbers st_dev subvolumes NFS Neil Brown" / "lwn.net \"The inode-number
  problem\" btrfs" / "btrfs subvolume st_dev anonymous device number uniqueness find du" · 1 kept (LWN
  866582)
- kagi · "\"The Btrfs inode-number epic (part 2: solutions)\" lwn" · 1 kept (LWN 866709)
- kagi · "rpm %config(noreplace) .rpmnew .rpmsave rules rpm.org documentation" / "rpm-software-management
  rpm manual \"%config(noreplace)\" rpmnew rpmsave md5 three-way" · 1 kept (rpm-spec(5) source; the
  rpm.org HTML page was a redirect stub)
- wiki.ubuntu.com MultiarchSpec · 504, not kept

## Tooling problems
- register.sh hit a 600s lock timeout twice under cross-lane contention.
- git.dpkg.org sits behind an Anubis anti-bot wall; the maintainer's GitHub mirror (guillemj/dpkg) was used instead.
- wiki.ubuntu.com (MultiarchSpec) returned 504 Gateway Timeout; not pulled.
- C: is full: 0 bytes free on 931G. The lane's share is small: about 77M across the whole shared scratchpad and 36M in `sources/`.
- Registered with archived copy at hand-off (15, plus three OCI slugs whose job exited 0 but were unconfirmed): [A-posix-sys-stat-file-identity-2024], [A-austin-group-defect-1314-file-identity-2020], [A-linux-overlayfs-documentation-2025], [A-linux-name-to-handle-at-man-2026], [B-linux-inode-man-2026], [B-lwn-btrfs-inode-number-epic-problem-2021], [B-lwn-btrfs-inode-number-epic-solutions-2021], [A-dpkg-triggers-specification-2025], [B-dpkg-manual-package-states-2025], [A-msi-organizing-applications-into-components-2021], [A-systemd-persistent-storage-udev-rules-2025], [B-archwiki-persistent-block-device-naming-2026], [A-lvm-manual-unique-names-2025], [A-ietf-rfc8881-nfs41-fileid-fsid-attributes-2020]; OCI: [A-oci-image-spec-config-identifiers-2025], [A-oci-image-spec-descriptor-digests-2025], [A-oci-distribution-spec-tags-digests-2025].
- Read and graded, not registered at hand-off (entry JSONs in the scratchpad): [A-debian-policy-package-relationships-2025], [A-debian-policy-files-conffiles-statoverride-2025], [A-debian-policy-maintainer-scripts-2025], [B-debian-policy-conffile-handling-appendix-2025], [B-debian-policy-diversions-appendix-2025], [A-dpkg-update-alternatives-manual-2025], [A-msi-changing-the-component-code-2021], [A-msi-component-rules-broken-2021], [A-rpm-spec-file-directives-2026], [A-linux-mount-namespaces-man-2026], [B-libfuse-high-level-api-use-ino-2025].
