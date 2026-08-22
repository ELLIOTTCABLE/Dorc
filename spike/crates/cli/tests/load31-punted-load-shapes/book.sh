#!/bin/sh
# TARGET (the three r31 load shapes in one book): a command-substitution script-relative load
# (`principle-load-operands-evaluate-over-controller-known-inputs`, held by the open ruling
# `ask-dollar-zero-command-substitution-path`), a set-valued glob load of two dorc-lang plugins
# (same principle, member-population half), and an unconditional inclusion of ordinary sh
# (`principle-book-code-source-is-inclusion`).
#
# The script-relative load is spelled through SCRIPT_DIR rather than inline, because an inline
# `$(...)` in a `.` operand is refused at PARSE tier today and takes the whole invocation's exit
# code with it, which would blind the other two shapes. The inline spelling is pinned at unit tier
# (`p-x-load-operand-dirname-of-dollar-zero`).
#
# TARGET RUN SET: `ran: wombat note done` and nothing else. The three described sites (`hork tune`,
# `wombat sync`, `zork prime`) all have vouching verdicts and probe fixtures saying their cells
# hold, so they elide; `plain_helper_step` is a function this book can only have because the plain
# sh file was included, and the `wombat note` inside it names no described verb, so it runs.
SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
. "$SCRIPT_DIR/hork.dorc.sh"

for plugin in ./*.plugin.dorc.sh; do
   . "$plugin"
done

. ./helpers.sh

hork tune web
wombat sync cache
zork prime cache
plain_helper_step
