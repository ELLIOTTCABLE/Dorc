# loop-nested-converges (task-L1 item-4c: nested-loop convergence smoke). Two nested
for p in a b; do for q in c d; do apt-get install -y "$p$q"; done; done
echo all-done
