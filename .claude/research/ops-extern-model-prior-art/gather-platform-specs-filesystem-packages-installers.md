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
3. **overlayfs's own documentation refutes stable inode identity.** st_ino and st_dev "can change over the lifetime of a non-directory object". Without `index`, a copy-up "will 'break' the link". xino "over-loads the high bits". Changes to the lower layer are "undefined". This coincides directly with GOTCHAS identity-tokens-perish-on-write-not-only-on-rename and backing-is-not-presenting, and with the `311u` refuted-terminal-tokens shape. +SURE [A-linux-overlayfs-documentation-2025]
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

(Owed by the lane; held in its context at hand-off.)

## Citations

(Owed by the lane: `grep -n` the archived copies for line numbers. The key quotes are inline in the Findings above.)

## Leads not pulled
- MultiarchSpec (wiki.ubuntu.com 504 Gateway Timeout).
- T10 SPC VPD page 0x83 and multipath `uid_attribute`.
- Apple HFS+/APFS directory hard links.
- `Documentation/filesystems/sharedsubtree.rst`.
- LVM VG metadata seqno (`vgs` / `lvmcache`).
- Microsoft SharedDLLs reference counting.

## Search log
(Owed by the lane.)

## Tooling problems
- register.sh hit a 600s lock timeout twice under cross-lane contention.
- git.dpkg.org sits behind an Anubis anti-bot wall; the maintainer's GitHub mirror (guillemj/dpkg) was used instead.
- wiki.ubuntu.com (MultiarchSpec) returned 504 Gateway Timeout; not pulled.
- C: is full: 0 bytes free on 931G. The lane's share is small: about 77M across the whole shared scratchpad and 36M in `sources/`.
- Registered with archived copy at hand-off (15, plus three OCI slugs whose job exited 0 but were unconfirmed): [A-posix-sys-stat-file-identity-2024], [A-austin-group-defect-1314-file-identity-2020], [A-linux-overlayfs-documentation-2025], [A-linux-name-to-handle-at-man-2026], [B-linux-inode-man-2026], [B-lwn-btrfs-inode-number-epic-problem-2021], [B-lwn-btrfs-inode-number-epic-solutions-2021], [A-dpkg-triggers-specification-2025], [B-dpkg-manual-package-states-2025], [A-msi-organizing-applications-into-components-2021], [A-systemd-persistent-storage-udev-rules-2025], [B-archwiki-persistent-block-device-naming-2026], [A-lvm-manual-unique-names-2025], [A-ietf-rfc8881-nfs41-fileid-fsid-attributes-2020]; OCI: [A-oci-image-spec-config-identifiers-2025], [A-oci-image-spec-descriptor-digests-2025], [A-oci-distribution-spec-tags-digests-2025].
- Read and graded, not registered at hand-off (entry JSONs in the scratchpad): [A-debian-policy-package-relationships-2025], [A-debian-policy-files-conffiles-statoverride-2025], [A-debian-policy-maintainer-scripts-2025], [B-debian-policy-conffile-handling-appendix-2025], [B-debian-policy-diversions-appendix-2025], [A-dpkg-update-alternatives-manual-2025], [A-msi-changing-the-component-code-2021], [A-msi-component-rules-broken-2021], [A-rpm-spec-file-directives-2026], [A-linux-mount-namespaces-man-2026], [B-libfuse-high-level-api-use-ino-2025].
