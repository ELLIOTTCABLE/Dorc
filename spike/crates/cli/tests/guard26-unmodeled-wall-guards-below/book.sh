#!/bin/sh
# guard26-unmodeled-wall-guards-below — the control: an unmodeled `hork` is a total wall.
# Both converged, vouched drops re-check live below it, matching the modeled-wall siblings.
hork provision
wombat a.conf /etc/a.conf
wombat b.conf /etc/b.conf
