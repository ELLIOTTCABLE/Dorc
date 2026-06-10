# LLM-GENERATED ORACLE SEED — intentionally quality-varied artificial testing
# corpus for a static-analysis project (Dorc). NOT real ops code; FROZEN EVIDENCE,
# NEVER EXECUTE. An artificial oracle cannot expose the truth of real-world ops-code.
# Validation is `dash -n` (parse-only) plus reading.
#
# Models the book's user creation (§1 L64-69):
#   useradd -m -s /bin/bash -G sshusers,sudousers,suusers \
#           -p "$(openssl passwd -6 "$USER_PW")" "$USER_NAME"
# guarded by `if ! getent passwd "$USER_NAME"` (L64). Verbless provider (`useradd
# <name>`; the name is the operand, ε-verb — exemplar shape). Establishes
# user:<name>#present, probed read-only by `getent passwd` (file-backed, hermetic for
# the passwd database; three-outcome: found / not-found / getent-error).
#
# THE ENTITY HAS SEVERAL FACETS, only ONE is honestly probed (um-user-1):
#   - #present  — `getent passwd <name>` rc. PROBED. This is also the ONLY facet the
#                 book's own guard checks, so eliding on presence MATCHES the book.
#   - #shell    — set by `-s /bin/bash`; readable via `getent passwd <name> | cut
#                 -d: -f7`, but value-COMPARISON, and the book never guards it.
#                 NOT probed here ⇒ a present user with the wrong shell still elides.
#   - #groups   — set by `-G …`; membership is `getent group <g> | cut -d: -f4`
#                 (R2-IDCACHE: NEVER `id -nG`, which reads a STALE nsswitch cache).
#                 Multi-valued and per-group; NOT a facet of the `user` kind's
#                 presence probe. See group.oracle.sh for the group side.
#   - #password — set by `-p <crypt>`. PASSWORD-BY-VALUE IS UN-PROBEABLE (the headline,
#                 um-user-1): the hash lives in root-only /etc/shadow, and `-p` takes
#                 an ALREADY-crypted value. `openssl passwd -6` mints a RANDOM salt per
#                 call, so re-running it yields a DIFFERENT hash every time — there is
#                 no read-only, deterministic "is the stored hash this password?" check
#                 without extracting the existing salt and re-crypting (not a clean
#                 idempotent read). So #password can NEVER gate an elision.
# CONSEQUENCE: this oracle elides a `useradd` ONLY on #present. A host where the user
# exists but has the wrong shell/groups/password reads converged and the useradd is
# elided — which is exactly the book's own presence-only `if ! getent passwd` logic,
# but the OTHER facets useradd would have set are silently not reconciled. Faithful,
# but recorded as a real limit.
#
# getent-as-query and the CONSUMED-STDOUT use (um-user-2): the book also does
# `getent passwd "$USER_NAME" | cut -d: -f6` (L109) to read the HOME DIR, whose value
# is consumed downstream (install -d "$user_home/.ssh"). A presence probe returns only
# rc; it CANNOT reproduce that consumed stdout. Modelling that would need getent as a
# provider with a `query` effect that PREDICTS the field-6 stdout value — a stdout-
# producing query, not a yes/no probe. Sketched in the note; not built here (the
# `user` kind's probe is presence-only).
oracle_kind=user
oracle_probe_user() { getent passwd "$1" >/dev/null 2>&1; }
oracle_effect useradd '' establish present

# command-keyed check(): `useradd <flags…> <name>` binds NO verb (verbless — the
# effect-map keys the ε-verb). The argparse MUST consume each VALUE-TAKING flag's
# argument (`-s SHELL`, `-G GROUPS`, `-p CRYPT`) so the operand bind lands on the
# username, NOT a flag value. The exemplar's bare `while [ "${1#-}" != "$1" ]; do
# shift; done` would STOP at `/bin/bash` (no leading dash) and wrongly bind it as the
# entity — so this check drives the same prefix-strip loop but consumes TWO words for
# value-taking flags (`case … shift 2`) and one for valueless flags (`-m`, the `*`
# arm). When `$1` is the username (no leading `-`) the loop exits; annotate it `user`.
# (Only `=`/`!=` tests and `shift`/`case` are in-dialect; a `[ "$#" -gt N ]` numeric
# guard is NOT modelled, so the prefix-strip condition is the portable spelling.)
useradd__check() {
   while [ "${1#-}" != "$1" ]; do
      case $1 in
         -s|-G|-p|-g|-d|-u|-c|-k) shift 2 ;;
         *) shift ;;
      esac
   done
   user : user = "$1"
   getent passwd "$user" >/dev/null 2>&1
}
