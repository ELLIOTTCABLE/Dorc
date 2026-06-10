# loop-analyzed-body-runs (task-L2 re-ground; brk-1(b)): the in-loop render floor STILL
# holds for a NON-Members in-loop establish. The body install does NOT reference the
# for-var (`nginx` is constant), so it is NOT a member-family — it takes the single-fact
# path: a self-establishing constant body becomes EstablishWritten via the back-edge
# (iteration 2 sees iteration 1's establish), so it is un-probeable AND the in-loop floor
# bars eliding one iteration. The body runs every iteration. (Contrast the loop-members-*
# cases, where a body referencing the for-var IS a member-family that elides per item-3.)
for x in a b; do apt-get install -y nginx; done
