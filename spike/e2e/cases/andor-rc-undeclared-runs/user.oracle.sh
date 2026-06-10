# A `user` oracle: `useradd <name>` establishes user:<name>#present. Verbless — the
# effect-map keys on the ε-verb, and the check() below binds the first operand as the
# entity (the find-3-era verb=deploy/Singleton reading is dead; task-W / 19I §2). The
# oracle declares no rc — under fork-mutator-rc none can exist — so a branch-consumed
# status carries ⊤ and the safe default is RUN (`19D`). The probe body is never
# executed by the harness; only the rendered apply runs.
oracle_kind=user
oracle_probe_user() { getent passwd "$1" >/dev/null 2>&1; }
oracle_effect useradd '' establish present
# command-keyed check(): `useradd <name>` binds NO verb (verbless — the effect-map keys
# on the ε-verb); annotate the first operand as `user`. The baked-verb wart (verb=deploy)
# dies: the entity is the operand, not a Singleton (task-W / 19I §2).
useradd__check() {
   user : user = "$1"
   getent passwd "$user" >/dev/null 2>&1
}
