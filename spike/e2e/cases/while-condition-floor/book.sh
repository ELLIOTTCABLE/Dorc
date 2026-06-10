# while-condition-floor (task-L1 item-4b): a `while` loop whose CONDITION's status is
# consumed at the render FLOOR. Under `set -e`:
#   * the condition `dpkg -s nginx` is errexit-EXEMPT (a failing while-condition does NOT
#     abort — so this terminates: the inert `dpkg` shim exits 1 ⇒ 0 iterations) and its
#     status is `StatusRenderFloor`-consumed (a loop condition is not in-situ substitutable,
#     like an `if`-guard) ⇒ it RUNS, never elided;
#   * the body `echo` is in-loop ⇒ floored to run (it is a pure builtin here; the
#     mutator-body errexit-region `StatusRelaxable` + failure-edge is pinned precisely in the
#     analysis unit test `while_condition_is_render_floor_and_errexit_exempt`).
# The post-loop `apt-get install -y curl` is poisoned to run: the in-loop Opaque condition
# propagates Reach::Top across the back-edge and out. Run-set: the condition (1×, false) +
# curl. Nothing elides. (set -e does NOT abort on the false condition — the L1 exemption.)
set -e
while dpkg -s nginx; do echo installing nginx; done
apt-get install -y curl
