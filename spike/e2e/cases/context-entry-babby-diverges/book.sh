#!/bin/sh
# 27C §8 babby-sudo (27N end-to-end): an ambient `hork install` + a sudo-WRAPPED one. The wrapped
# site is answered by measurement IN the root context (sudo's entry form), the ambient one bare;
# two sites, two contexts, two independent answers. `hork`/`sudo` are inert mocks (never real
# mutators). The unwrapped site is FIRST so its ambient elision is independent of the wrapped
# mutator's downstream wall (the context-blind Stage-1 wall is a disclosed 27N limitation).
hork install wombat
sudo hork install frob
