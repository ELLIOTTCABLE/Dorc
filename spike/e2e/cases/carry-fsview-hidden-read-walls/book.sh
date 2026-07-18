#!/bin/sh
# 27C §4(a) carry NEGATIVE (27O): ambient kp elides; the chroot-wrapped policyctl STRADDLER has (A)
# but fails (B) — its verdict reads an fs-view-dependent file unmarked — so the carry walls, site runs.
kp ip_forward 1
chroot /mnt policyctl swappiness 10
