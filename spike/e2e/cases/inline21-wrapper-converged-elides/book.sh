# inline21-wrapper-converged-elides (arch-2, brk-2 — the payoff; the 207 wrapper-pun shape):
# a one-line wrapper function defined once, called twice (`apt_install nginx`/`apt_install curl`).
# Each call INLINES — the body (`apt-get install -y "$1" >/dev/null 2>&1`) is spliced, its `$1`
# bound to the call's operand (nginx / curl), and the install is an EstablishMembers-like body
# sub-record OF THE CALL (`site 0.0` / `site 1.0`). BOTH packages are already installed
# (converged), so the all-or-nothing CALL license substitutes EACH call to `true` — run-set
# EMPTY. The body redirects to /dev/null (the tc-M2 devnull-exemption: a `>/dev/null` body
# write does NOT refuse the inline; a write to a real file would). This is the arch-2 payoff:
# the wrapper idiom real books and oracle libraries use, which used to kill every elision below.
apt_install() { apt-get install -y "$1" >/dev/null 2>&1; }
apt_install nginx
apt_install curl
