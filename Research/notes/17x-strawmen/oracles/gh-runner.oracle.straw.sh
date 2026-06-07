#!/bin/sh
# Oracle EXTRACTED from books/gh-runner.book.sh (fedorenkoivan/devops @b27bfac). [STRAWMAN]
# Kinds: tool/service:docker · group:docker (SHARED) · user:github-runner · file:config.sh
#
# (A) LIFTED — the human's guards:
docker_present() { command -v docker >/dev/null 2>&1; }        # book L46: if command -v docker
runner_user()    { id -u github-runner >/dev/null 2>&1; }      # book L65: id -u "$RUNNER_USER" && return 0
runner_config()  { [ -f /home/github-runner/actions-runner/config.sh ]; }   # book L71: [[ -f ...config.sh ]]
#
# (B) SUPPLIED — unguarded group-membership + service:
docker_group()     { getent group docker >/dev/null 2>&1; }                  # book L67 usermod -aG docker (assumes group)
runner_in_docker() { id -nG github-runner 2>/dev/null | grep -qw docker; }   # would elide the usermod
docker_svc()       { systemctl is-active --quiet docker 2>/dev/null; }       # book L61 enable --now docker
#
# Kind tags: group:docker=getent group · service:docker=systemctl is-active · user:github-runner=id/getent.
# group:docker is ESTABLISHED by the docker-install half (L41/L61) and CONSUMED by the usermod half (L67)
# — an intra-script (kind,establish)->(kind,require) pair; the same group:docker also correlates ACROSS to
# any separate docker-host script (see _correlate).
