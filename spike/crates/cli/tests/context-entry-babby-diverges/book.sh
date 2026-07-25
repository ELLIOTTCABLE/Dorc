#!/bin/sh
# 27C §8 babby-sudo (27N): an ambient `hork install` + a sudo-WRAPPED one, answered in two contexts.
# Unwrapped FIRST so its ambient elision is independent of the wrapped mutator's downstream wall
# (the context-blind Stage-1 wall is a disclosed 27N limitation). `hork`/`sudo` are inert mocks.
hork install wombat
sudo hork install frob
