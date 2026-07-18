#!/bin/sh
# 27C §4(a) pure-predicate carry (27O): chroot shifts fs-view; KernelParam is invariant:fs-view and
# the verdict is read-set-closed, so the ambient measurement carries the wrapped check UNFLAGGED.
chroot /mnt kp ip_forward 1
