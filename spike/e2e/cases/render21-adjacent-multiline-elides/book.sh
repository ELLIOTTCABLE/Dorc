# render21-adjacent-multiline-elides (P1 fix 21E, note 214 §9 hunt-7): two ADJACENT elidable
# MULTI-LINE leaves. Each `apt-get install` operand carries a LITERAL NEWLINE, and the two are
# `;`-separated so the SECOND install STARTS on the FIRST's closing line. Both converge ⇒ both
# substitute to `true`. The pre-fix render keyed edits by their lone START line and the line-walk
# jumped over the first edit's CONSUMED span — ORPHANING the second leaf (it survived
# half-spliced, `apt-get install -y "c`, with the provenance comment landing inside the open
# quote). dash-n-clean BY COINCIDENCE here (the quotes happened to balance), but the second
# install RAN under argv-echo mocks — a silent over-execute. The group-splice fix collapses both
# leaves onto one rendered line (`true; true`) with a combined disclosure, run-set EMPTY.
apt-get install -y "a
b"; apt-get install -y "c
d"
