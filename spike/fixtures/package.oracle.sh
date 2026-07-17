#!/bin/sh
# ── oracle: package (Debian dpkg/apt) ──────────────  [predict-is-the-oracle]
# Lifted statically by Dorc (never run as Dorc-code). Plain sh: `<provider>__predict`
# functions whose `case $verb` arms + inline annotations + trailing marks the analyzer
# derives the effect-map from (23D §1). See Research/notes/162 §2.

# COMMAND-KEYED predict() (19H §2 / task-W): the oracle's OWN argparse traces the book's
# resolved argv to the inline kind-annotation — the real entity-resolution (the engine
# parses nothing). Flag-strip pre- and post-verb, bind the verb, annotate the single
# operand as `package`; the `[ "$2" = "" ]` guard refuses a SECOND operand (a
# multi-operand `install a b` resolves no probe ⇒ runs, never a wrong single-entity
# elision). install/reinstall establish #installed; purge/remove invert it (the `!` mark).
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then
      case $verb in
         install|reinstall) dpkg-query -W -f='${Status}' "$pkg" >/dev/null 2>&1 : package:"$pkg"#installed ;;
         purge|remove) dpkg-query -W -f='${Status}' "$pkg" >/dev/null 2>&1 :! package:"$pkg"#installed ;;
      esac
   fi
}

# `dpkg -i <pkg>` establishes package:<pkg>#installed (jc-dpkg-i). `-i` is a flag the
# check strips, so `dpkg` is a verbless provider (the ε-verb): the operand annotates as
# `package` and the trailing mark carries the establish claim.
dpkg__predict() {
   case $1 in -i) shift ;; esac
   pkg : package = "$1"
   dpkg-query -W -f='${Status}' "$pkg" >/dev/null 2>&1 : package:"$pkg"#installed
}
