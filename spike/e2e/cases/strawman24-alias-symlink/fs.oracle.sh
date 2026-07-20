#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
# minimal fs (config-file) oracle for a `writeconf <path>` tool — the 24F closure on the `fs` kind.
writeconf__predict() {
   path : sm.dorc.Fs = "$1"
   conf-exists "$path" : sm.dorc.Fs:"$path"@written
}

writeconf__disturbs() {
   printf '%s\n' "$1" : disturbs sm.dorc.Fs
}

writeconf__is_converged() {
   conf-exists "$1"
}

sm_dorc_Fs__resolve() {
   realpath -m -- "$1"
}
