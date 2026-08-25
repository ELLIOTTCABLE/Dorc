#!/bin/sh
export VERBOSE=true
wombat ensure alpha
snipe ensure gamma
AWS_PROFILE=staging grebe ensure web1
AWS_PROFILE=prod grebe ensure web1
export PATH="/opt/fake:$PATH"
wombat ensure beta
