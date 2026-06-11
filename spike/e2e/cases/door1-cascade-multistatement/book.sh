# door1-cascade-multistatement (door-1 NESTED REGION, charter 20V §4 / note 215): the
# guarded block has 3+ statements, one of them an inner `if`. When the `dpkg -s nginx`
# Query guard holds (rc 0), the `|| { … }` branch is DEAD ⇒ the fold's `kill_rec` walks
# the WHOLE brace group and kills every leaf in the nested region: the sed, BOTH leaves of
# the inner `if` (its `[ -f … ]` test condition AND its `cp` then-body), and the trailing
# systemctl restart. None of them needs rc-provenance of its own — they are all
# unreachable. The cascade does not stop at the block's top level; it descends into the
# inner construct (the `kill_rec` Group→If→Simple recursion).
#
# Renders `true || { :; if :; then :; fi; :; }`, dash-clean: the inner `if`'s arms become
# `:` (never empty — the span-edit substitutes each leaf, never deletes a clause). Run-set
# EMPTY. HOST: nginx installed (the guard holds).
set -e
dpkg -s nginx >/dev/null 2>&1 || {
   sed -i 's/^.*PermitRootLogin.*/PermitRootLogin no/' /etc/ssh/sshd_config
   if [ -f /etc/ssh/sshd_config.bak ]; then cp /etc/ssh/sshd_config /etc/ssh/sshd_config.bak; fi
   systemctl restart sshd
}
