# fold-oror-guard-omits (the canonical idempotency idiom): a read-only QUERY guard
# short-circuits the mutator. `command -v nginx` is now a first-class Query (task-D2);
# its OWN probed rc (0 when nginx is on PATH) is fold-usable as the guard's Status
# (rule-query-validity passes — nothing mutates upstream), so the fold proves the
# `|| apt-get install` branch DEAD and OMITS the install. The guard itself mutates
# nothing and its rc is known + `||`-consumed, so it is value-preservingly substituted
# by its stand-in `true` (rc 0). The whole line collapses to `true`; nothing runs
# (expected.ran is empty). The DESIGN `dpkg -s nginx || apt-get install nginx` idiom,
# with `command -v` as the guard. (An un-probed/⊤ guard rc would NOT fold — the
# kFAIL-perform floor; and an invalid guard, below a mutator, runs for real — see
# exec-query-after-mutator-runs.)
command -v nginx >/dev/null 2>&1 || apt-get install -y nginx
