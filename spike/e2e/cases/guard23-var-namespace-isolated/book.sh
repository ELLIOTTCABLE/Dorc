# guard23-var-namespace-isolated (23C-fd1: guard-body variable capture — the material design
# finding; XFAIL until the guard tier lands). POSIX functions execute in the CALLER's variable
# namespace, and the shipped guard preamble is the oracle's check body VERBATIM (strip-only is
# law, rul-ternary-verdict) — whose body assigns `pkg` bare (`pkg="$1"`). This book uses `pkg`
# as its OWN variable, composing the hazard from individually-pinned pieces:
#   pkg=vim                   — a book variable, set early
#   hork wombat               — opaque wall (downstream loses its elide-license, plans/233 §0)
#   apt-get install -y curl   — vouched + converged-past-wall ⇒ GUARD; the predict's `pkg="$1"`
#                               would clobber the book's `pkg` to "curl" in the caller namespace
#   apt-get install -y "$pkg" — DIVERGED (vim absent) ⇒ runs with ORIGINAL BYTES (`"$pkg"`
#                               survives verbatim); at apply it must still install VIM
# Desired (mechanism-NEUTRAL per human ruling h3 — subshell-wrap OR check-body `local`/`${n:-}`
# hygiene, the engine's choice; the behaviour pins land either way): the guarded artifact is
# indistinguishable from the bare book w.r.t. book variables — the final line installs VIM, not
# a clobbered "curl". A naive whole-body ship clobbers `pkg` ⇒ vim never installs AND the
# suppressed curl re-installs (under-execute + unnecessary-execute in one stroke, silent, rc 0).
# The load-bearing pin is expected.ran's final `apt-get install -y vim`; the mechanism is open.
pkg=vim
hork wombat
apt-get install -y curl
apt-get install -y "$pkg"
