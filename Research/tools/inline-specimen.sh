#!/bin/sh
# inline-specimen.sh — pull a remote source file VERBATIM into a Research note,
# commit-pinned, with license + sha256, ready for literate annotation.
#
# Why this exists: a note that quotes real in-the-wild code must (1) reproduce it
# byte-for-byte and (2) attribute it to an immutable commit. This does the
# mechanical part, so an agent only ever EDITS AROUND the source afterwards
# (adding markdown fences + commentary) and can never corrupt the source or its
# attribution by retyping/paraphrasing. House style: see specimens/090.
#
# Usage:
#   Research/tools/inline-specimen.sh OWNER/REPO PATH NOTE.md [REF] [LANG]
#
#   OWNER/REPO  e.g. FlorentRevest/linux-kernel-vscode
#   PATH        path within the repo, e.g. tasks.sh
#   NOTE.md     output note (refuses to overwrite an existing file)
#   REF         optional commit/branch/tag to pin; default = last commit to touch PATH
#   FENCE_LANG  optional code-fence language; default inferred from the extension
#
# Example:
#   Research/tools/inline-specimen.sh FlorentRevest/linux-kernel-vscode tasks.sh \
#     Research/specimens/092-specimen-something.md
#
# Then: edit NOTE.md — set the title, add an intro, and split the single fenced
# block into ~4-6 chunks with commentary between (never touch the code lines).
#
# Requires: gh (authenticated), curl, sha256sum|shasum, date.

set -eu

die() { echo "inline-specimen: $*" >&2; exit 1; }

sha256_of() {
   if command -v sha256sum >/dev/null 2>&1; then
      sha256sum "$1" | cut -d' ' -f1
   elif command -v shasum >/dev/null 2>&1; then
      shasum -a 256 "$1" | cut -d' ' -f1
   else
      die "need sha256sum or shasum on PATH"
   fi
}

[ $# -ge 3 ] || die "usage: OWNER/REPO PATH NOTE.md [REF] [FENCE_LANG]"
REPO=$1; SRC_PATH=$2; NOTE=$3; REF=${4:-}; FENCE_LANG=${5:-}

case "$REPO" in
   */*) : ;;
   *)   die "OWNER/REPO must contain a slash: $REPO" ;;
esac
if [ -e "$NOTE" ]; then
   die "refusing to overwrite existing $NOTE (remove it first if intended)"
fi
command -v gh   >/dev/null 2>&1 || die "gh not found (check: gh auth status)"
command -v curl >/dev/null 2>&1 || die "curl not found"

# 1. Pin to an immutable commit: the caller's REF, else the last commit to touch PATH.
if [ -n "$REF" ]; then
   SHA=$(gh api "repos/$REPO/commits/$REF" --jq '.sha' 2>/dev/null || true)
else
   SHA=$(gh api "repos/$REPO/commits?path=$SRC_PATH&per_page=1" --jq '.[0].sha' 2>/dev/null || true)
fi
[ -n "$SHA" ] || die "could not resolve a commit SHA for $REPO :: $SRC_PATH"

URL="https://github.com/$REPO/blob/$SHA/$SRC_PATH"
RAW="https://raw.githubusercontent.com/$REPO/$SHA/$SRC_PATH"
DATE=$(date -u +%Y-%m-%d)
LICENSE=$(gh api "repos/$REPO/license" --jq '.license.spdx_id' 2>/dev/null || echo "NONE-DETECTED")

# 2. Fetch the bytes (curl, with gh's raw media type as a fallback) and hash them.
TMP=$(mktemp)
trap 'rm -f "$TMP"' EXIT
curl -fsSL "$RAW" -o "$TMP" \
   || gh api "repos/$REPO/contents/$SRC_PATH?ref=$SHA" -H "Accept: application/vnd.github.raw" >"$TMP" \
   || die "could not fetch raw source"
[ -s "$TMP" ] || die "fetched source is empty"
SUM=$(sha256_of "$TMP")

# 3. A source line that is itself a ``` fence would break the wrapping block.
if grep -q '^```' "$TMP"; then
   die "source contains a markdown code-fence line; inline it by hand with a longer fence"
fi

# 4. Infer a fence language from the extension when not given.
if [ -z "$FENCE_LANG" ]; then
   case "$SRC_PATH" in
      *.sh|*.bash|*.ksh|*.dash) FENCE_LANG=bash ;;
      *.py)                     FENCE_LANG=python ;;
      *.yml|*.yaml)             FENCE_LANG=yaml ;;
      *.tf|*.hcl)               FENCE_LANG=hcl ;;
      *.rb)                     FENCE_LANG=ruby ;;
      *.pp)                     FENCE_LANG=puppet ;;
      *.j2)                     FENCE_LANG=jinja ;;
      *)                        FENCE_LANG="" ;;
   esac
fi

# 5. Emit attribution header + verbatim, fenced source. Header values are
#    interpolated (mechanical); the source is cat'd, never retyped.
mkdir -p "$(dirname "$NOTE")"
{
   echo "# NNN — TITLE (edit me): specimen \`$SRC_PATH\` from $REPO"
   echo
   echo "> AI-generated literate specimen. The code below is reproduced VERBATIM"
   echo "> (commit-pinned + byte-checked by inline-specimen.sh); annotate it ONLY by"
   echo "> editing around the source — never retype a code line. Style: see specimens/090."
   echo
   echo "## Source & attribution"
   echo "- Repo: $REPO — file \`$SRC_PATH\`"
   echo "- Permalink (commit-pinned): $URL"
   echo "- Raw: $RAW"
   echo "- Commit: $SHA"
   echo "- License: $LICENSE"
   echo "- Retrieved: $DATE via inline-specimen.sh (gh + curl); sha256 \`$SUM\`"
   echo
   echo "---"
   echo
   printf '```%s\n' "$FENCE_LANG"
   cat "$TMP"
   if [ -n "$(tail -c1 "$TMP")" ]; then echo; fi
   printf '```\n'
} >"$NOTE"

echo "wrote $NOTE"
echo "  commit  $SHA"
echo "  license $LICENSE"
echo "  sha256  $SUM"
echo "  lang    ${FENCE_LANG:-(none)}"
echo "next: set the title, add an intro, and split the one code block into ~4-6 chunks with commentary."
