#!/bin/sh
# Self-test for ./commit-msg. Run via `mise run test:hooks`; rides `mise run gate`.
#
# Worth its keep because the gate is unusual on three counts: it is the only mechanical
# enforcement of a convention agents actively fight, it is untyped sh, and its failure
# direction is OPEN — a broken matcher stops refusing and says nothing. The subtle cases
# (generated merge/revert messages exempted, the editor's `#` block stripped, a HUMAN
# co-author left alone) are each here because getting one wrong is invisible in review.
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
hook="$here/commit-msg"
msg=$(mktemp) || exit 1
trap 'rm -f "$msg"' EXIT INT TERM

failures=0

# check NAME WANT(pass|fail) ENV... -- message-on-stdin
check() {
   name=$1 want=$2
   shift 2
   cat > "$msg"
   if env "$@" sh "$hook" "$msg" >/dev/null 2>&1; then got=pass; else got=fail; fi
   if [ "$got" = "$want" ]; then
      echo "ok   $name"
   else
      echo "FAIL $name (want $want, got $got)"
      failures=$((failures + 1))
   fi
}

check accepts-a-labelled-ai-commit pass CLAUDECODE=1 <<'EOF'
(AI fix) Move the index onto the hot path
EOF

check refuses-the-claude-coauthor-trailer fail CLAUDECODE=1 <<'EOF'
(AI fix) Move the index onto the hot path

Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
EOF

check refuses-the-session-trailer fail CLAUDECODE=1 <<'EOF'
(AI fix) Move the index onto the hot path

Claude-Session: https://claude.ai/code/session_01
EOF

check refuses-the-generation-footer fail CLAUDECODE=1 <<'EOF'
(AI new) Add the thing

Generated with [Claude Code](https://claude.com/claude-code)
EOF

check leaves-a-human-coauthor-alone pass CLAUDECODE=1 <<'EOF'
(AI fix) Move the index onto the hot path

Co-Authored-By: Jane Doe <jane@example.com>
EOF

check refuses-a-subject-with-no-labels fail CLAUDECODE=1 <<'EOF'
Fix the thing
EOF

check exempts-a-generated-merge pass CLAUDECODE=1 <<'EOF'
Merge branch 'ai/r28-unify' into ai/main
EOF

check exempts-a-generated-revert pass CLAUDECODE=1 <<'EOF'
Revert "(AI test) Prove the transcript-driven loop"
EOF

check refuses-an-agent-commit-without-ai fail CLAUDECODE=1 <<'EOF'
(fix) Move the index onto the hot path
EOF

check honours-the-human-escape-hatch pass CLAUDECODE=1 DORC_HUMAN_COMMIT=1 <<'EOF'
(fix) Move the index onto the hot path
EOF

check leaves-a-non-agent-commit-alone pass CLAUDECODE= CLAUDE_CODE_ENTRYPOINT= <<'EOF'
(fix) Move the index onto the hot path
EOF

check warns-but-admits-an-unknown-label pass CLAUDECODE=1 <<'EOF'
(AI fix loom) Reword the catalog register
EOF

check accepts-the-purpose-labels pass CLAUDECODE=1 <<'EOF'
(AI fix aid cli) Reword a register and the usage line
EOF

check strips-the-editor-comment-block pass CLAUDECODE=1 <<'EOF'
(AI doc) Explain the seam

# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
EOF

if [ "$failures" -ne 0 ]; then
   echo "$failures case(s) failed" >&2
   exit 1
fi
echo "commit-msg hook: all cases green"
