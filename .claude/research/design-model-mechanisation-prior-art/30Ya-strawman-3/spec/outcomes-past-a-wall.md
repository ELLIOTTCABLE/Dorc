# Outcomes past a wall

> STRAWMAN, non-normative. A conductor's exercise of the assay shape (`notes/30Y`) against a made-up cut of Dorc's outcome algebra, written to find where Alloy and assay fight an author. Nothing in this directory is Dorc's design; every design decision here was invented to reach a compiling model, binds nothing, and is superseded by any ruled document on the same topic (`plans/239`, `notes/23O` § 2, `KNOBS` named mechanisms, `USER_STORY` stages 2 and 5, `spike/CLAUDE.md`).

A converged line below a line that may run is guarded, unless the admin has typed the flag and the speech in force separates what the earlier line may write from what the later line's check reads, in which case it survives. This document holds the speech that decides it, the rules that consume the speech, and the books that pin the answers. The identity tier is a black box here: which keys are separate is a claim it makes, not a thing this document derives.

The stdlib describes apt.
Tessa writes the file describer.
Debsvc publishes the service library that describes systemctl and ufw.
Foob writes a certificate tool.
The identity tier answers which keys are separate, and speaks as one voice.
Nobody describes hork.
The host under test is the world; it speaks nothing, and its records show what each line wrote.

```alloy
one sig stdlib, tessa, debsvc, foob, identity extends Speaker {}
```

## § 1 Keys, and what a line really wrote

```alloy
sig MReferent {}
sig Key { w: one Shword, reaches: one MReferent }
fact { all disj a, b: Key | a.w != b.w }

sig Touched { line: one Line, wrote: set Key }
fact { all l: Line | lone line.l }
fun touches[l: Line]: set Key { some line.l implies (line.l).wrote else Key }
```

A key is a word that names a piece of the world; in this document a word is at most one key, and splitting a word into keys is the identity tier's work, not done here. What a line really wrote is the world's record about that line, not anybody's claim; a line with no record is taken to have written everything, so a book that says a survival was safe must say what each wall wrote.

## § 2 Speech about a verb

```alloy
sig Vouches extends MDecl { verb: one Shword }
sig Disturbs extends MDecl { verb: one Shword, sub: lone Shword, at: set Int, cells: set Shword }
sig Reads extends MDecl { verb: one Shword, sub: lone Shword, at: set Int, cells: set Shword }
sig Withholds extends MDecl { verb: one Shword }

pred matches[v: Shword, s: set Shword, l: Line] { v = l.cmd and (no s or s = l.argv[0]) }
fun keysAt[l: Line, at: set Int, cells: set Shword]: set Key { { k: Key | k.w in l.argv[at] + cells } }
fact { all d: Disturbs, l: Line | matches[d.verb, d.sub, l] implies d.at in (l.argv).Shword }
fact { all d: Reads, l: Line | matches[d.verb, d.sub, l] implies d.at in (l.argv).Shword }

fun footprint[S: set MDecl, l: Line]: set Key {
   (some d: Disturbs & S | matches[d.verb, d.sub, l])
      implies { k: Key | all d: Disturbs & S | matches[d.verb, d.sub, l] implies k in keysAt[l, d.at, d.cells] }
      else Key
}
fun backing[S: set MDecl, l: Line]: set Key {
   (some d: Reads & S | matches[d.verb, d.sub, l])
      implies { k: Key | all d: Reads & S | matches[d.verb, d.sub, l] implies k in keysAt[l, d.at, d.cells] }
      else Key
}

fact { all d: Disturbs | d in True iff all l: Line | matches[d.verb, d.sub, l] implies touches[l] in keysAt[l, d.at, d.cells] }
fact { all v: Verdict | some d: Vouches & True | d.verb = v.of.cmd and d.speaker = v.speaker }
```

A vouch is the existence of the verb's verdict function: whoever wrote it may say a line of that verb is converged, and that statement is the verdict claim. A footprint is an at-most claim about what a verb, under a matched first argument, writes: the keys at named argument positions and the named cells, and nothing else. A verb nobody has footprinted writes every key. Two true footprints for one line bound the same tool from above, so the footprint in force is their intersection. A backing is what the verb's check reads under the same shape; a verb whose check reads are unstated reads every key. A footprint is true exactly when every matched line really wrote inside it; a backing is what the check read, complete by construction, and is not computed against the world here. A withhold is the describer declining the elision half of the license while keeping the guard half. A claim that names an argument position names one every matched line has.

## § 3 The identity tier's answers

```alloy
sig Separate extends MDecl { a, b: one Key }
fact { Computed = Disturbs + Separate }
fact { all s: Separate | s in True iff s.a.reaches != s.b.reaches }
pred separated[S: set MDecl, x, y: Key] { some s: Separate & S | (s.a = x and s.b = y) or (s.a = y and s.b = x) }
```

Whether two keys reach two things is the identity tier's statement; it is true when they do. Nothing here says how it knows.

## § 4 The sparing question, the outcomes, and what a survival rested on

```alloy
fun answer[S: set MDecl, w, r: Line]: one Ans {
   (all x: footprint[S, w], y: backing[S, r] | separated[S, x, y]) implies DISJOINT else UNKNOWN
}

fun said[l: Line]: set MDecl { l.speech & MDecl }
sig Withheld in Line {}
fact { Withheld = { l: Line | some d: Withholds & l.speech | d.verb = l.cmd } }
fact { Elided = { l: Converged - Withheld | all w: l.above - Elided | some Typed and answer[said[l], w, l] in Spares } }

pred wrong[S: set MDecl, w, r: Line] { answer[S, w, r] = DISJOINT and some x: touches[w], y: backing[S, r] | x.reaches = y.reaches }
pred underExecuted[l: Line] { l in Elided and some w: l.above - Elided, x: touches[w], y: backing[said[l], l] | x.reaches = y.reaches }

fun support[S: set MDecl, w, r: Line]: set MDecl {
   { d: Disturbs & S | matches[d.verb, d.sub, w] }
   + { d: Reads & S | matches[d.verb, d.sub, r] }
   + { s: Separate & S | some x: footprint[S, w], y: backing[S, r] | (s.a = x and s.b = y) or (s.a = y and s.b = x) }
}
fun restsOn[S: set MDecl, w, r: Line]: set MDecl { { d: S | answer[S - d, w, r] != answer[S, w, r] } }
fun by[l: Line]: set MDecl { { d: said[l] | some w: l.above - Elided | d in support[said[l], w, l] } }
```

The lines that may run are the lines that did not elide: a line that runs, and a guarded line, whose guard falls through to the original bytes when the world drifted. A converged line elides only when nothing above it may run, or when the flag is typed and, for every line above it that may run, every pair of a key that line may write and a key this line's check reads is separated by a claim in force. The flag permits acting on separations and manufactures none. A converged line whose describer withholds elision guards even with nothing above it. What a survival rested on is the speech its derivation used: the footprints of the walls it survived, its own backing, and the separations it consumed. The counterfactual reading, the claims whose removal would change the answer, is kept beside it because it is the one the previous strawman used, and § 5 shows where the two part.

## § 5 The laws

```alloy
pred allTrue[S: set MDecl] { S in True }
pred oneVoicePerLine[S: set MDecl] {
   all l: Line | lone { d: Disturbs & S | matches[d.verb, d.sub, l] } and lone { d: Reads & S | matches[d.verb, d.sub, l] }
   all disj s, t: Separate & S | not ((s.a = t.a and s.b = t.b) or (s.a = t.b and s.b = t.a))
}

check neverWrongWhenAllTrue { all S: set MDecl, w, r: Line | allTrue[S] implies not wrong[S, w, r] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword
run neverWrongWhenAllTrue_premise { some S: set MDecl, w, r: Line | allTrue[S] and answer[S, w, r] = DISJOINT and some touches[w] and some backing[S, r] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword

check neverUnderExecutedWhenAllTrue { allTrue[Line.speech & MDecl] implies no l: Elided | underExecuted[l] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword
run neverUnderExecutedWhenAllTrue_premise { allTrue[Line.speech & MDecl] and some Survived } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword

check monotoneInSpeech { all S, S2: set MDecl, w, r: Line | S in S2 and allTrue[S2] implies answer[S, w, r] in answer[S2, w, r].*weaker } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword
run monotoneInSpeech_premise { some S, S2: set MDecl, w, r: Line | S in S2 and S != S2 and allTrue[S2] and answer[S2, w, r] = DISJOINT and answer[S, w, r] = UNKNOWN } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword

check strangerSafe { all S: set MDecl, d: MDecl, w, r: Line | allTrue[S + d] and d.speaker not in S.speaker implies answer[S, w, r] in answer[S + d, w, r].*weaker } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword
run strangerSafe_premise { some S: set MDecl, d: MDecl, w, r: Line | some S and allTrue[S + d] and d.speaker not in S.speaker } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword

check attributionHonest { all S: set MDecl, w, r: Line | wrong[S, w, r] implies some d: support[S, w, r] | d not in True } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword
run attributionHonest_premise { some S: set MDecl, w, r: Line | wrong[S, w, r] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword

check attributionSufficient { all S: set MDecl, w, r: Line | answer[support[S, w, r], w, r] = answer[S, w, r] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword
run attributionSufficient_premise { some S: set MDecl, w, r: Line | some support[S, w, r] and answer[S, w, r] = DISJOINT } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword

check attributionByRemovalHonest { all S: set MDecl, w, r: Line | wrong[S, w, r] implies some d: restsOn[S, w, r] | d not in True } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword
run attributionByRemovalHonest_premise { some S: set MDecl, w, r: Line | wrong[S, w, r] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword

check attributionByRemovalHonestWithOneVoice { all S: set MDecl, w, r: Line | oneVoicePerLine[S] and wrong[S, w, r] implies some d: restsOn[S, w, r] | d not in True } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword
run attributionByRemovalHonestWithOneVoice_premise { some S: set MDecl, w, r: Line | oneVoicePerLine[S] and wrong[S, w, r] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword

check flagGatesSurvival { no Typed implies no Survived } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword
run flagGatesSurvival_premise { some Typed and some Survived } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword

check survivalRestsOnFootprints { all l: Survived, w: l.above - Elided | some backing[said[l], l] implies some d: Disturbs & l.speech | matches[d.verb, d.sub, w] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword
run survivalRestsOnFootprints_premise { some l: Survived | some backing[said[l], l] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword
```

The first four are the product laws: given true speech the sparing question never lies and no elided line needed to run; more speech never weakens an answer; a stranger's speech never weakens one either. Attribution by support is expected to hold. Attribution by removal is expected to fail on the free universe, with and without one voice per line: a false footprint that leaves a key out and a false separation of that key from the backing each cover the same collision on their own, so removing either leaves the answer standing and neither is named. Redundant speech is not the cause; two independent falsehoods masking each other is, and one voice per line does not exclude it. The one-voice premise stays in the corpus checks because it is what makes a book's attribution readable, not because it rescues the law.

## § 6 Who may say what

> A verdict is the word of whoever wrote the verb's verdict function.
> A footprint, a backing, and a withhold are the word of the verb's describer and nobody else.
> One describer speaks per verb.
> One footprint and one backing speak per shape.
> Which keys are separate is the identity tier's word.
> The world's records have no speaker.

```alloy
check oneDescriberPerVerb { all disj a, b: Vouches | a.verb != b.verb }
check describerSpeaksAlone { all d: Disturbs + Reads + Withholds | some v: Vouches | v.verb = d.verb and v.speaker = d.speaker }
check oneFootprintPerShape { all disj d, e: Disturbs | d.verb != e.verb or d.sub != e.sub }
check oneBackingPerShape { all disj d, e: Reads | d.verb != e.verb or d.sub != e.sub }
```

## § 7 The claims and the load files

```alloy
one sig stdlib__apt_get_is_described extends Vouches {} { speaker = stdlib  verb = apt_get }
one sig stdlib__apt_get_update_disturbs_only_the_package_index extends Disturbs {} { speaker = stdlib  verb = apt_get  sub = update  no at  cells = pkg_index }
one sig stdlib__apt_get_update_is_converged_when_the_package_index_is_fresh extends Reads {} { speaker = stdlib  verb = apt_get  sub = update  no at  cells = pkg_index }
one sig stdlib__apt_get_install_is_converged_when_the_package_database_lists_it extends Reads {} { speaker = stdlib  verb = apt_get  sub = install  no at  cells = dpkg_db }

one sig tessa__cp_is_described extends Vouches {} { speaker = tessa  verb = cp }
one sig tessa__cp_disturbs_only_its_destination extends Disturbs {} { speaker = tessa  verb = cp  no sub  at = 1  no cells }
one sig tessa__cp_is_converged_when_its_destination_has_the_source_content extends Reads {} { speaker = tessa  verb = cp  no sub  at = 0 + 1  no cells }

one sig debsvc__systemctl_is_described extends Vouches {} { speaker = debsvc  verb = systemctl }
one sig debsvc__systemctl_enable_disturbs_only_the_unit_state extends Disturbs {} { speaker = debsvc  verb = systemctl  sub = enable  no at  cells = unit_state }
one sig debsvc__systemctl_enable_is_converged_when_the_unit_is_enabled_and_active extends Reads {} { speaker = debsvc  verb = systemctl  sub = enable  no at  cells = unit_state }
one sig debsvc__ufw_is_described extends Vouches {} { speaker = debsvc  verb = ufw }
one sig debsvc__ufw_allow_disturbs_only_the_rule_table extends Disturbs {} { speaker = debsvc  verb = ufw  sub = allow  no at  cells = ufw_rules }
one sig debsvc__ufw_allow_is_converged_when_the_rule_is_present extends Reads {} { speaker = debsvc  verb = ufw  sub = allow  no at  cells = ufw_rules }
one sig debsvc__ufw_withholds_elision extends Withholds {} { speaker = debsvc  verb = ufw }

one sig foob__foobar_is_described extends Vouches {} { speaker = foob  verb = foobar }
one sig foob__foobar_sync_certs_disturbs_only_the_certs extends Disturbs {} { speaker = foob  verb = foobar  sub = sync_certs  at = 1  no cells }
one sig foob__foobar_sync_certs_is_converged_when_the_certs_are_current extends Reads {} { speaker = foob  verb = foobar  sub = sync_certs  at = 1  no cells }
```

The stdlib footprints `apt-get update` and declines to footprint `apt-get install`, whose maintainer scripts are arbitrary root shell; an install that really runs walls everything below it. Tessa's `cp` writes its destination and its check reads both files. Debsvc's two verbs write one cell each and read the same cell. Foob's tool writes the directory it is handed. Nobody describes hork.

```sh
# stdlib_apt.sh
. ./stdlib__apt_get_is_described.sh
. ./stdlib__apt_get_update_disturbs_only_the_package_index.sh
. ./stdlib__apt_get_update_is_converged_when_the_package_index_is_fresh.sh
. ./stdlib__apt_get_install_is_converged_when_the_package_database_lists_it.sh
```

```sh
# tessa_fs.sh
. ./tessa__cp_is_described.sh
. ./tessa__cp_disturbs_only_its_destination.sh
. ./tessa__cp_is_converged_when_its_destination_has_the_source_content.sh
```

```sh
# debsvc_services.sh
. ./debsvc__systemctl_is_described.sh
. ./debsvc__systemctl_enable_disturbs_only_the_unit_state.sh
. ./debsvc__systemctl_enable_is_converged_when_the_unit_is_enabled_and_active.sh
. ./debsvc__ufw_is_described.sh
. ./debsvc__ufw_allow_disturbs_only_the_rule_table.sh
. ./debsvc__ufw_allow_is_converged_when_the_rule_is_present.sh
```

```sh
# debsvc_ufw_guard_only.sh
. ./debsvc__ufw_withholds_elision.sh
```

```sh
# foob_certs.sh
. ./foob__foobar_is_described.sh
. ./foob__foobar_sync_certs_disturbs_only_the_certs.sh
. ./foob__foobar_sync_certs_is_converged_when_the_certs_are_current.sh
```

## § 8 The books

The seven lines of the webhost book, under the speech and the world of each morning.

```sh
# pristine_host.sh
. ./stdlib_apt.sh
. ./tessa_fs.sh
. ./debsvc_services.sh
. ./foob_certs.sh

   apt-get update
#} apt_get update
#= no Typed
#= this in Ran

   apt-get install -y nginx
#} apt_get install dash_y nginx
#= this in Ran

   cp ./nginx.conf /etc/nginx/nginx.conf
#} cp local_nginx_conf nginx_conf
#= this in Ran

   foobar sync-certs /etc/nginx/certs
#} foobar sync_certs certs_dir
#= this in Ran

   systemctl enable --now nginx
#} systemctl enable dash_dash_now nginx
#= this in Ran

   hork tune --profile web >>/var/log/hork.log 2>&1
#} hork tune dash_dash_profile web append_hork_log stderr_to_stdout
#= this in Ran

   ufw allow 443/tcp
#} ufw allow port_443_tcp
#= this in Ran
```

Nothing is converged, so everything runs.

```sh
# converged_host_below_two_walls.sh
. ./stdlib_apt.sh
. ./tessa_fs.sh
. ./debsvc_services.sh

   apt-get update
#} apt_get update
#= no Typed
#= one sig stdlib__the_package_index_is_fresh extends Verdict {} { speaker = stdlib  of = this }
#= this in Elided

   apt-get install -y nginx
#} apt_get install dash_y nginx
#= one sig stdlib__nginx_is_installed extends Verdict {} { speaker = stdlib  of = this }
#= this in Elided

   cp ./nginx.conf /etc/nginx/nginx.conf
#} cp local_nginx_conf nginx_conf
#= one sig tessa__the_nginx_config_matches extends Verdict {} { speaker = tessa  of = this }
#= this in Elided and no by[this]

   foobar sync-certs /etc/nginx/certs
#} foobar sync_certs certs_dir
#= this in Ran

   systemctl enable --now nginx
#} systemctl enable dash_dash_now nginx
#= one sig debsvc__nginx_is_enabled_and_active extends Verdict {} { speaker = debsvc  of = this }
#= this in Guarded

   hork tune --profile web >>/var/log/hork.log 2>&1
#} hork tune dash_dash_profile web append_hork_log stderr_to_stdout
#= this in Ran

   ufw allow 443/tcp
#} ufw allow port_443_tcp
#= one sig debsvc__the_rule_for_443_is_present extends Verdict {} { speaker = debsvc  of = this }
#= this in Guarded
```

Stage 2: foobar is undescribed, so it runs and walls systemctl; hork walls ufw. The three lines above the first wall elide, and nothing rested on anything.

```sh
# stale_index_honest_walls.sh
. ./stdlib_apt.sh
. ./tessa_fs.sh
. ./debsvc_services.sh
. ./foob_certs.sh

   apt-get update
#} apt_get update
#= no Typed
#= this in Ran

   apt-get install -y nginx
#} apt_get install dash_y nginx
#= one sig stdlib__nginx_is_installed extends Verdict {} { speaker = stdlib  of = this }
#= this in Guarded

   cp ./nginx.conf /etc/nginx/nginx.conf
#} cp local_nginx_conf nginx_conf
#= one sig tessa__the_nginx_config_matches extends Verdict {} { speaker = tessa  of = this }
#= this in Guarded

   foobar sync-certs /etc/nginx/certs
#} foobar sync_certs certs_dir
#= one sig foob__the_certs_are_current extends Verdict {} { speaker = foob  of = this }
#= this in Guarded

   systemctl enable --now nginx
#} systemctl enable dash_dash_now nginx
#= one sig debsvc__nginx_is_enabled_and_active extends Verdict {} { speaker = debsvc  of = this }
#= this in Guarded

   hork tune --profile web >>/var/log/hork.log 2>&1
#} hork tune dash_dash_profile web append_hork_log stderr_to_stdout
#= this in Ran

   ufw allow 443/tcp
#} ufw allow port_443_tcp
#= one sig debsvc__the_rule_for_443_is_present extends Verdict {} { speaker = debsvc  of = this }
#= this in Guarded
```

Stage 5's opening: one stale index and every converged line below it guards, flag or no flag, because nobody has been asked to trust anything.

```sh
# stale_index_survives_on_footprints.sh
. ./stdlib_apt.sh
. ./tessa_fs.sh
. ./debsvc_services.sh
. ./foob_certs.sh

   apt-get update
#} apt_get update
#= all disj a, b: Key | a.reaches != b.reaches
#= Typed = risk_faultless_skips
#= one sig identity__the_package_index_is_not_the_package_database extends Separate {} { speaker = identity  a = w.pkg_index  b = w.dpkg_db }
#= one sig identity__the_package_index_is_not_the_local_config extends Separate {} { speaker = identity  a = w.pkg_index  b = w.local_nginx_conf }
#= one sig identity__the_package_index_is_not_the_nginx_config extends Separate {} { speaker = identity  a = w.pkg_index  b = w.nginx_conf }
#= one sig identity__the_package_index_is_not_the_certs extends Separate {} { speaker = identity  a = w.pkg_index  b = w.certs_dir }
#= one sig identity__the_package_index_is_not_the_unit_state extends Separate {} { speaker = identity  a = w.pkg_index  b = w.unit_state }
#= one sig world__update_wrote_the_package_index extends Touched {} { line = this  wrote = w.pkg_index }
#= this in Ran

   apt-get install -y nginx
#} apt_get install dash_y nginx
#= one sig stdlib__nginx_is_installed extends Verdict {} { speaker = stdlib  of = this }
#= this in Survived and not underExecuted[this] and stdlib__apt_get_update_disturbs_only_the_package_index in by[this]

   cp ./nginx.conf /etc/nginx/nginx.conf
#} cp local_nginx_conf nginx_conf
#= one sig tessa__the_nginx_config_matches extends Verdict {} { speaker = tessa  of = this }
#= this in Survived and not underExecuted[this]

   foobar sync-certs /etc/nginx/certs
#} foobar sync_certs certs_dir
#= one sig foob__the_certs_are_current extends Verdict {} { speaker = foob  of = this }
#= this in Survived and not underExecuted[this]

   systemctl enable --now nginx
#} systemctl enable dash_dash_now nginx
#= one sig debsvc__nginx_is_enabled_and_active extends Verdict {} { speaker = debsvc  of = this }
#= this in Survived and not underExecuted[this] and no (speaker.foob & by[this])

   hork tune --profile web >>/var/log/hork.log 2>&1
#} hork tune dash_dash_profile web append_hork_log stderr_to_stdout
#= this in Ran

   ufw allow 443/tcp
#} ufw allow port_443_tcp
#= one sig debsvc__the_rule_for_443_is_present extends Verdict {} { speaker = debsvc  of = this }
#= this in Guarded
```

Stage 5 with the trust typed: the index refresh really runs and writes the index and nothing else; the identity tier separates the index from every backing below; four lines survive it, each naming the stdlib's footprint, and foob's speech carries none of them. Hork has no author and ufw guards.

```sh
# stale_index_flag_not_typed.sh
. ./stdlib_apt.sh
. ./tessa_fs.sh
. ./debsvc_services.sh
. ./foob_certs.sh

   apt-get update
#} apt_get update
#= all disj a, b: Key | a.reaches != b.reaches
#= no Typed
#= one sig identity__the_package_index_is_not_the_package_database extends Separate {} { speaker = identity  a = w.pkg_index  b = w.dpkg_db }
#= one sig identity__the_package_index_is_not_the_local_config extends Separate {} { speaker = identity  a = w.pkg_index  b = w.local_nginx_conf }
#= one sig identity__the_package_index_is_not_the_nginx_config extends Separate {} { speaker = identity  a = w.pkg_index  b = w.nginx_conf }
#= one sig identity__the_package_index_is_not_the_certs extends Separate {} { speaker = identity  a = w.pkg_index  b = w.certs_dir }
#= one sig identity__the_package_index_is_not_the_unit_state extends Separate {} { speaker = identity  a = w.pkg_index  b = w.unit_state }
#= one sig world__update_wrote_the_package_index extends Touched {} { line = this  wrote = w.pkg_index }
#= this in Ran

   apt-get install -y nginx
#} apt_get install dash_y nginx
#= one sig stdlib__nginx_is_installed extends Verdict {} { speaker = stdlib  of = this }
#= this in Guarded

   cp ./nginx.conf /etc/nginx/nginx.conf
#} cp local_nginx_conf nginx_conf
#= one sig tessa__the_nginx_config_matches extends Verdict {} { speaker = tessa  of = this }
#= this in Guarded

   foobar sync-certs /etc/nginx/certs
#} foobar sync_certs certs_dir
#= one sig foob__the_certs_are_current extends Verdict {} { speaker = foob  of = this }
#= this in Guarded

   systemctl enable --now nginx
#} systemctl enable dash_dash_now nginx
#= one sig debsvc__nginx_is_enabled_and_active extends Verdict {} { speaker = debsvc  of = this }
#= this in Guarded

   hork tune --profile web >>/var/log/hork.log 2>&1
#} hork tune dash_dash_profile web append_hork_log stderr_to_stdout
#= this in Ran

   ufw allow 443/tcp
#} ufw allow port_443_tcp
#= one sig debsvc__the_rule_for_443_is_present extends Verdict {} { speaker = debsvc  of = this }
#= this in Guarded
```

The same speech and the same world with the flag untyped: honest walls. The separations are in force and answer DISJOINT, and nothing consumes them.

```sh
# false_footprint_under_executes.sh
. ./stdlib_apt.sh
. ./tessa_fs.sh
. ./debsvc_services.sh
. ./foob_certs.sh

   apt-get update
#} apt_get update
#= all disj a, b: Key | a.reaches != b.reaches
#= Typed = risk_faultless_skips
#= one sig identity__the_package_index_is_not_the_package_database extends Separate {} { speaker = identity  a = w.pkg_index  b = w.dpkg_db }
#= one sig identity__the_package_index_is_not_the_local_config extends Separate {} { speaker = identity  a = w.pkg_index  b = w.local_nginx_conf }
#= one sig identity__the_package_index_is_not_the_nginx_config extends Separate {} { speaker = identity  a = w.pkg_index  b = w.nginx_conf }
#= one sig identity__the_package_index_is_not_the_certs extends Separate {} { speaker = identity  a = w.pkg_index  b = w.certs_dir }
#= one sig identity__the_package_index_is_not_the_unit_state extends Separate {} { speaker = identity  a = w.pkg_index  b = w.unit_state }
#= one sig world__update_wrote_the_index_and_installed_packages extends Touched {} { line = this  wrote = w.pkg_index + w.dpkg_db }
#= this in Ran

   apt-get install -y nginx
#} apt_get install dash_y nginx
#= one sig stdlib__nginx_is_installed extends Verdict {} { speaker = stdlib  of = this }
#= this in Survived and underExecuted[this] and stdlib__apt_get_update_disturbs_only_the_package_index in by[this] and stdlib__apt_get_update_disturbs_only_the_package_index not in True

   cp ./nginx.conf /etc/nginx/nginx.conf
#} cp local_nginx_conf nginx_conf
#= one sig tessa__the_nginx_config_matches extends Verdict {} { speaker = tessa  of = this }
#= this in Survived and not underExecuted[this]

   foobar sync-certs /etc/nginx/certs
#} foobar sync_certs certs_dir
#= one sig foob__the_certs_are_current extends Verdict {} { speaker = foob  of = this }
#= this in Survived and not underExecuted[this]

   systemctl enable --now nginx
#} systemctl enable dash_dash_now nginx
#= one sig debsvc__nginx_is_enabled_and_active extends Verdict {} { speaker = debsvc  of = this }
#= this in Survived and not underExecuted[this]

   hork tune --profile web >>/var/log/hork.log 2>&1
#} hork tune dash_dash_profile web append_hork_log stderr_to_stdout
#= this in Ran

   ufw allow 443/tcp
#} ufw allow port_443_tcp
#= one sig debsvc__the_rule_for_443_is_present extends Verdict {} { speaker = debsvc  of = this }
#= this in Guarded
```

`GOTCHAS:apt-get-update-can-install-packages`: an APT hook installed packages during the refresh, the stdlib's footprint was false, the install line survived on it and needed to run. The sin is committed, and it is attributed: the survival names the false claim.

```sh
# guarded_writer_casts_a_wall.sh
. ./stdlib_apt.sh
. ./tessa_fs.sh
. ./debsvc_services.sh

   apt-get update
#} apt_get update
#= all disj a, b: Key | a.reaches != b.reaches
#= some w.local_nginx_conf
#= Typed = risk_faultless_skips
#= one sig identity__the_package_index_is_not_the_nginx_config extends Separate {} { speaker = identity  a = w.pkg_index  b = w.nginx_conf }
#= one sig identity__the_package_index_is_not_the_backup extends Separate {} { speaker = identity  a = w.pkg_index  b = w.conf_backup }
#= one sig identity__the_package_index_is_not_the_unit_state extends Separate {} { speaker = identity  a = w.pkg_index  b = w.unit_state }
#= one sig identity__the_nginx_config_is_not_the_unit_state extends Separate {} { speaker = identity  a = w.nginx_conf  b = w.unit_state }
#= one sig identity__the_backup_is_not_the_unit_state extends Separate {} { speaker = identity  a = w.conf_backup  b = w.unit_state }
#= one sig world__update_wrote_the_package_index extends Touched {} { line = this  wrote = w.pkg_index }
#= this in Ran

   cp ./nginx.conf /etc/nginx/nginx.conf
#} cp local_nginx_conf nginx_conf
#= one sig tessa__the_nginx_config_matches extends Verdict {} { speaker = tessa  of = this }
#= one sig world__the_copy_wrote_the_nginx_config extends Touched {} { line = this  wrote = w.nginx_conf }
#= this in Guarded

   cp /etc/nginx/nginx.conf /var/backups/nginx.conf.bak
#} cp nginx_conf conf_backup
#= one sig tessa__the_backup_matches extends Verdict {} { speaker = tessa  of = this }
#= one sig world__the_backup_wrote_the_backup extends Touched {} { line = this  wrote = w.conf_backup }
#= this in Guarded

   systemctl enable --now nginx
#} systemctl enable dash_dash_now nginx
#= one sig debsvc__nginx_is_enabled_and_active extends Verdict {} { speaker = debsvc  of = this }
#= this in Survived and not underExecuted[this] and tessa__cp_disturbs_only_its_destination in by[this]
```

The identity tier says nothing about the local file, so the first copy guards below the refresh. A guarded line may run, and if it runs it rewrites the config the backup line's check reads; the identity tier can separate the config from the index, and cannot separate it from itself, so the backup guards too. The service line's backing is separate from what all three may write, so it survives all three, and names the copy's footprint among what it rested on.

```sh
# converged_host_everything_described.sh
. ./stdlib_apt.sh
. ./tessa_fs.sh
. ./debsvc_services.sh
. ./foob_certs.sh

   apt-get update
#} apt_get update
#= no Typed
#= one sig stdlib__the_package_index_is_fresh extends Verdict {} { speaker = stdlib  of = this }
#= this in Elided and no by[this]

   apt-get install -y nginx
#} apt_get install dash_y nginx
#= one sig stdlib__nginx_is_installed extends Verdict {} { speaker = stdlib  of = this }
#= this in Elided and no by[this]

   cp ./nginx.conf /etc/nginx/nginx.conf
#} cp local_nginx_conf nginx_conf
#= one sig tessa__the_nginx_config_matches extends Verdict {} { speaker = tessa  of = this }
#= this in Elided and no by[this]

   foobar sync-certs /etc/nginx/certs
#} foobar sync_certs certs_dir
#= one sig foob__the_certs_are_current extends Verdict {} { speaker = foob  of = this }
#= this in Elided and no by[this]

   systemctl enable --now nginx
#} systemctl enable dash_dash_now nginx
#= one sig debsvc__nginx_is_enabled_and_active extends Verdict {} { speaker = debsvc  of = this }
#= this in Elided and no by[this]

   ufw allow 443/tcp
#} ufw allow port_443_tcp
#= one sig debsvc__the_rule_for_443_is_present extends Verdict {} { speaker = debsvc  of = this }
#= this in Elided and no by[this]
```

Stage 3's steady state without hork: everything described, everything converged, nothing runs, nothing rested on anything.

```sh
# withheld_elision_guards_in_position.sh
. ./stdlib_apt.sh
. ./tessa_fs.sh
. ./debsvc_services.sh
. ./debsvc_ufw_guard_only.sh
. ./foob_certs.sh

   apt-get update
#} apt_get update
#= no Typed
#= one sig stdlib__the_package_index_is_fresh extends Verdict {} { speaker = stdlib  of = this }
#= this in Elided

   apt-get install -y nginx
#} apt_get install dash_y nginx
#= one sig stdlib__nginx_is_installed extends Verdict {} { speaker = stdlib  of = this }
#= this in Elided

   cp ./nginx.conf /etc/nginx/nginx.conf
#} cp local_nginx_conf nginx_conf
#= one sig tessa__the_nginx_config_matches extends Verdict {} { speaker = tessa  of = this }
#= this in Elided

   foobar sync-certs /etc/nginx/certs
#} foobar sync_certs certs_dir
#= one sig foob__the_certs_are_current extends Verdict {} { speaker = foob  of = this }
#= this in Elided

   systemctl enable --now nginx
#} systemctl enable dash_dash_now nginx
#= one sig debsvc__nginx_is_enabled_and_active extends Verdict {} { speaker = debsvc  of = this }
#= this in Elided

   ufw allow 443/tcp
#} ufw allow port_443_tcp
#= one sig debsvc__the_rule_for_443_is_present extends Verdict {} { speaker = debsvc  of = this }
#= this in Guarded and no by[this]
```

The same book with one more true statement loaded, debsvc withholding ufw's elision: no answer to any sparing question changed, and the ufw line moved from elided to guarded. Adding speech is monotone in answers and not in outcomes; a withhold is admissible because it only withholds.
