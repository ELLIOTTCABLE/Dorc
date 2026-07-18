# LLM-GENERATED ORACLE SEED — intentionally quality-varied artificial testing
# corpus for a static-analysis project (Dorc). NOT real ops code; FROZEN EVIDENCE,
# NEVER EXECUTE. An artificial oracle cannot expose the truth of real-world ops-code.
# Validation is `dash -n` (parse-only) plus reading.
#
# Models the book's group creation (§1 L57-59):
#   for grp in sshusers suusers sudousers; do
#       getent group "$grp" >/dev/null || groupadd "$grp"
#   done
# `groupadd <name>` is verbless (the name is the operand, ε-verb) and establishes
# group:<name>#present, probed read-only by `getent group <name>`.
#
# THIS IS THE CLEANEST CHECK-THEN-ACT IN THE BOOK (and the foil for the hard cases):
#   - the entity is a single, unambiguous operand (a group NAME) — no facets, no
#     tuple, no value-by-content. Contrast user#password (un-probeable) and a ufw
#     rule (multi-word tuple).
#   - `getent group` is FILE-backed (/etc/group via nsswitch's `files`), so it is
#     hermetic — unlike `getent hosts`/`ahosts`, which route to live DNS and are a
#     buried network side-channel (F-GETENT-HOSTS: read-only ≠ hermetic). Group
#     PRESENCE is a genuine three-outcome probe (found / absent / getent-error).
#   - the book's guard IS exactly this probe (`getent group "$grp"`), so eliding a
#     converged groupadd is sound and matches the book 1:1.
# Note (um-group-1): this models group EXISTENCE only. Group MEMBERSHIP (who is in
# the group) is a DIFFERENT question the book answers via `useradd -G` (see
# user.oracle.sh), and its honest probe is `getent group <g> | cut -d: -f4`, NEVER
# `id -nG` (R2-IDCACHE: `id` reads a stale nsswitch/sssd cache ⇒ wrong elision after a
# membership change). Membership is not a facet of THIS presence-only `group` kind.
oracle_kind=group
oracle_probe_group() { getent group "$1" >/dev/null 2>&1; }
oracle_effect groupadd '' establish present

# command-keyed check(): `groupadd <name>` binds NO verb (verbless — ε-verb). Strip any
# leading flags defensively (the book passes none), then annotate the single operand
# `group`. Single-entity by construction; no multi-operand guard needed (groupadd takes
# exactly one group name).
groupadd__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   grp : group = "$1"
   getent group "$grp" >/dev/null 2>&1
}
