#!/bin/sh
# payload-composed-shape-walls (24T §1 acceptance shape; lane-payload-v1 rung-0): the composed
# `pipeline ∘ context ∘ payload` shape analyzes end-to-end WITHOUT crashing. With no wrapper or
# eval'er oracle loaded (MODELS-only lane), every leg walls: the pipe composes, the `sudo` context
# leg takes the honest wall, and the site runs verbatim (empty-world-byte-identical). When
# lane-context-entry + eval'er oracles land, this site upgrades transparently. (The brief's shape
# uses a `>> /etc/f` redirect; a read-only payload is used here so the exec gate stays sandbox-safe —
# the `sudo` wall never evaluates the payload anyway.)
echo data | sudo sh -c 'cat /etc/motd'
