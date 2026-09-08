A narrow, brief list of 'design forcing-functions': the real-world ops examples that kill simple designs. (See sibling `faleshoods-*.md` for longer, non-Dorc-specific domain-knowledge.)

Suggest new entries only when a particular case-under-study has repeated caused significant rework or damage; esp. when one has been *forgotten* during a later design-round and materially damaged the design. (Use numbers *only* to reference inside ephemera/chat; durables should reference by slug, as the item will be frequently reordered/trimmed.)

1. a-path-is-not-a-referent: "A remount mid-apply changes which inode a path denotes."
2. a-host-is-not-a-partition: "Two hosts mount one NFS export at two different paths."
3. same-name-different-referent-per-viewpoint: "`sudo crontab -l` reads root's crontab, not the caller's."
4. not-every-transit-changes-the-referent: "`sudo dpkg -s nginx` reads the same database as without sudo."
5. address-inequality-is-not-referent-inequality: "A default docker install serves alice and root from one socket."
6. distinct-names-alias-within-a-kind: "`nginx` and `nginx-full` can be one installed package via provides."
7. containment-by-path-prefix-lies: "A hardlink puts one inode both inside and outside a subtree."
8. namespace-composition-is-not-concatenation: "A symlink inside `/mnt` makes `chroot /mnt chroot /t` differ from `chroot /mnt/t`."
9. renaming-a-parent-moves-every-child-name: "`mv /etc/nginx /etc/nginx.bak` re-points every path beneath it, touching no inode."
10. a-name-resolves-from-a-vantage: "`10.0.0.5` names a different machine behind each NAT."
11. resolution-is-set-valued: "A round-robin hostname lands on a different member per connection."
12. identity-tokens-have-clone-horizons: "Cloned images share a machine-id; a restored snapshot boots twice with one boot_id."
13. a-name-is-not-a-target-over-time: "The VM behind `web1` is rebuilt under the same name between two lines."
14. a-store-is-not-one-inode: "An SQLite database becomes three files once its WAL and shm sidecars exist."
15. an-omitted-store-breaks-invariance: "pipx keeps per-user state under `~/.local` beside its system install."
16. nonzero-status-is-not-speech: "errexit exits with the failing command's status, and a false `[ ]` is 1."
