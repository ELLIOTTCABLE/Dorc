#!/bin/sh
# Recreate the read-only research-source checkouts under ./Vendor/.
# These are not submodules and are not committed; this file is the only thing
# git needs to track. Run on a new machine to rebuild the corpus, then let
# syncthing/git ignore ./Vendor/ itself. Idempotent: re-running only fetches
# what is missing or at the wrong commit.
#
# Usage:  ./clone-vendor.sh [name-filter ...]      # no args = everything
# Env:    VENDOR_DIR=path   override clone root (default ./Vendor)
#
# Requires git + a POSIX shell (Git Bash on Windows). For lighter clones of the
# big repos (codeql/flow/infer) add `--filter=blob:none` to the git clone below.
set -u
cd "$(dirname "$0")" || exit 1
VENDOR_DIR="${VENDOR_DIR:-Vendor}"
fails=0

clone_one() {
   dir="$VENDOR_DIR/$1"
   cur=$(git -C "$dir" rev-parse --verify -q HEAD 2>/dev/null || true)
   if [ "$cur" = "$3" ]; then echo "ok    $1"; return 0; fi
   echo "clone $1"
   rm -rf "$dir"
   if git clone --quiet "$2" "$dir" && git -C "$dir" checkout --quiet --detach "$3"; then
      return 0
   fi
   echo "FAIL  $1" >&2
   fails=$((fails + 1))
}

while read -r path url commit; do
   case "$path" in '' | '#'*) continue ;; esac
   if [ "$#" -gt 0 ]; then
      keep=0
      for f in "$@"; do case "$path" in *"$f"*) keep=1 ;; esac; done
      [ "$keep" -eq 1 ] || continue
   fi
   clone_one "$path" "$url" "$commit"
done <<'MANIFEST'
# path                          remote                                          commit
SVF                            https://github.com/SVF-tools/SVF                 5c45081f75d16afffc5fc9121e1f2f7a614e0bef
TAJS                           https://github.com/cs-au-dk/TAJS                 3bdf55a411d6fc278fce9a6b4198b104a07a7852
WALA                           https://github.com/wala/WALA                     3f73f62d11d966ebc50188e7a50dd72c954de46f
codeql                         https://github.com/github/codeql                 a16f1c555cea339ef5c8b4c7c9285b6e578c396c
doop                           https://github.com/plast-lab/doop                ad051399b7bffa7b503067fa568f0aae18281215
fabricate                      https://github.com/brushtechnology/fabricate.git 24a3acfc86129fda56c48a6ec1594e2cd5cd8e0c
flow                           https://github.com/facebook/flow                 6b6ebf0a75127dda99771edf3300b7e936bfdfe3
fsatrace                       https://github.com/jacereda/fsatrace.git         4d4a967293eed5bd2a0298c5be6858e3f7fccb28
goblint-analyzer               https://github.com/goblint/analyzer              4c8b277a4cfcbb5b0e65bff2e7a60b0c96bd60d6
infer                          https://github.com/facebook/infer                743771d504d3d0b8a2132399aed5c2929098a5c4
mvdan-sh                       https://github.com/mvdan/sh                      f5c6e2779117ed9e742709a7f717aab927bf47cd
oils                           https://github.com/oils-for-unix/oils            8df6fec7531b9f608f33950c5467b81753b9e9dd
rattle                         https://github.com/ndmitchell/rattle.git         3c935ca004ec8557ad6edb92d42f8bdf5bc78f46
salsa                          https://github.com/salsa-rs/salsa                7e77c49f27210dc85b49ba28606542d72836b5ab
shellcheck                     https://github.com/koalaman/shellcheck           764802b7c023e9fdc191c12f57c418f186ad76ae
smoosh                         https://github.com/mgree/smoosh                  cc67dbe6a4953e51431997eac025b5e3f46c3d2d
souffle                        https://github.com/souffle-lang/souffle          c3861e0d3b82dcbcfce14a7bc8f434e70868bef9
tree-sitter-bash               https://github.com/tree-sitter/tree-sitter-bash  a06c2e4415e9bc0346c6b86d401879ffb44058f7
tup                            https://github.com/gittup/tup.git                2867b66e7105d432dce2609538117c1e6910bc73
colis-anr/colis-batch          https://github.com/colis-anr/colis-batch         8f768a24063f44b1d46b2a52fcff94ff393fac88
colis-anr/colis-constraints    https://github.com/colis-anr/colis-constraints   470643707a8fdf445f4da7977b93ee1110522611
colis-anr/colis-language       https://github.com/colis-anr/colis-language      62e8d77b9e83d6ce691013662cbeea2a9cb0e349
colis-anr/ftwu                 https://github.com/colis-anr/ftwu                ce1534b9c3715eb56f6078578d6046f3e7c07ff3
colis-anr/lintshell            https://github.com/colis-anr/lintshell           f547ca4311b377583575c083e00ab33ae223f740
colis-anr/morbig               https://github.com/colis-anr/morbig              b97a5be622a7b5539f43617e74bf9cccad1d58b9
colis-anr/morsmall             https://github.com/colis-anr/morsmall            572d04bbbe0bd12848e2abae886e4b69402ce6e9
colis-anr/shstats              https://github.com/colis-anr/shstats             a21f0461a20aa25caf20f97f3fdfec2412411d61
colis-anr/sourcil-poc          https://github.com/colis-anr/sourcil-poc         a706d1bc9e226a6600a48aa28ea3d8fc3e432c61
MANIFEST

[ "$fails" -eq 0 ] || { echo "$fails repo(s) failed" >&2; exit 1; }
echo "done"
