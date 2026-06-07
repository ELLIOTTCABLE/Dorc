#!/bin/sh
# Step D — "things correlated by KIND," spelled by executable guards (the getent-pattern). [STRAWMAN]
# Across the six extracted oracles, every kind collapses onto a SMALL set of blessed, uniformly-probe-able
# types. The SAME executable guard probes that kind regardless of which script (or oracle author) touched it:
#
#   user    X   ->  getent passwd X              plik · consul · deploy · github-runner · termidar · www-data
#   group   X   ->  getent group  X              plik · redis · docker · sudo
#   member X@G  ->  id -nG X | grep -qw G        www-data@redis · github-runner@docker · deploy@sudo
#   service X   ->  systemctl is-active X        consul · plikd · redis-server · docker · firewalld · termidar-ssh · fail2ban
#   tool    X   ->  command -v X                 docker · go · systemctl
#   file    P   ->  [ -e P ]                     /usr/local/bin/consul · /swapfile · config.sh
#   port  N/p   ->  firewall-cmd --query-port=N/p   (clean)   |   ufw status | grep   (FRAGILE)   2222 · 22 · 80 · 443
#
# === THE SOLID EXAMPLE (priority 2) — a cross-script elision token-co-reference CANNOT reach ===
#
#   books/enginescript-redis.book.sh  L77:   usermod -aG redis www-data        # UNGUARDED
#
#   This line has an UNDECLARED precondition: user:www-data must already exist. The redis book contains NO
#   token tying `www-data` to whoever created it — in a real web stack www-data is created by a SEPARATE
#   script (the apache/php-fpm install, or the distro package). Intra-script dataflow / shared-arg
#   co-reference (094 g1) sees nothing: `www-data` is a bare literal this script merely assumes.
#
#   With the getent-pattern KIND probe, Dorc CAN:
prereq_wwwdata() { getent passwd www-data >/dev/null 2>&1; }         # (1) recognize+probe user:www-data
#       -> www-data is a BLESSED NSS kind (getent passwd) — so it is the SAME analyzable kind that the
#          web-stack script established, even with zero shared token between the two scripts;
#       -> on a host where the web stack already ran, the precondition is CONVERGED (discharged);
already_member() { id -nG www-data 2>/dev/null | grep -qw redis; }   # (2) and the usermod itself ELIDES:
#       -> if www-data is already in group:redis, `usermod -aG redis www-data` is a no-op -> replace w/ `true`.
#
#   Net: an elision (skip the usermod; don't re-run the web-stack user-create) reachable ONLY because
#   user:www-data and group:redis are BLESSED, uniformly-probe-able kinds shared across INDEPENDENTLY
#   authored scripts. The same shape recurs for group:docker (gh-runner consumes a docker group a separate
#   docker-host script establishes). That is the round-17 thesis, on real downloaded code.
#
# === HONEST coverage (priority 1) — what the getent-pattern does NOT reach ===
#   - config CONTENT (sed'd redis.conf / sshd_config / php.ini): not a getent-pattern kind; per-line grep
#     is fragile + oracle-specific.
#   - whole-script SENTINELS (enginescript install-state.conf REDIS=1): Dorc-opaque; not kind-correlatable.
#   - PACKAGES (apt/apk/dnf install): mostly unguarded AND the deliberately-excluded domain.
#   - PORTS under ufw: probe-able only via fragile `ufw status` grep (carries the 15x `.`-as-regex bug);
#     CLEAN only under firewalld (`firewall-cmd --query-port`). Provider-dependent — same kind, two answers.
#   - group-MEMBERSHIP + ufw ports are mutated UNGUARDED in 4/6 books — the probe must be SUPPLIED by an
#     oracle; it is NOT liftable from the book.
#
# Verdict: the blessed-kind core (user/group/service/tool/file) covers the IDEMPOTENT SPINE of real
# provisioning — every service-install creates a user+service; every script checks tools+files — and it
# UNIQUELY enables cross-script elision. But a real hardening/messy book leaves a long tail (ports,
# content, sentinels, membership) un-liftable; those need oracle-supplied probes or fall to the ⊤-run floor.
