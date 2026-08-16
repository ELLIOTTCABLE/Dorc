# dorc-lang/v0.2
# The two-operand drop: converged when the destination already holds the source's bytes. The bind
# names the entity, the trailing verdict mark names the cell the exit status witnesses, so the two
# sites key `sm.dorc.File:/etc/a.conf@content` and `…:/etc/b.conf@content` — distinct cells.
wombat__is_converged() {
   dst : sm.dorc.File = "$2"
   wombat cmp -- "$1" "$dst"   : sm.dorc.File:"$dst"@content
}
