# door1-and-form-runs (door-1 && POLE, charter 20V §4 / note 215): the `&&` companion-pole
# to door1-and-form. Here the `dpkg -s nginx` Query guard HOLDS (rc 0), so `&&` PROCEEDS ⇒
# the `{ … }` block is LIVE and runs. This pins the `&&` direction's OTHER pole (the dual of
# door1-cascade-diverged-runs for `||`): the identical `&& { … }` shape that folded whole in
# door1-and-form (guard failed) runs whole here (guard held), proving the `&&` deadness is
# PROBE-KEYED, never structural.
#
# The guard substitutes to `true` (rc 0, its probe-sourced value); the block is kept verbatim
# (its controller is live). Renders `true && { systemctl reload nginx; }`; the systemctl
# reload runs (the block's single command), the artifact exits 0 (the live block's last rc),
# so the exec gate accepts it (contrast door1-and-form, whose failing guard exits non-zero ⇒
# analysis-only). run-set: `systemctl reload nginx`. HOST: nginx installed (the guard holds).
set -e
dpkg -s nginx >/dev/null 2>&1 && { systemctl reload nginx; }
