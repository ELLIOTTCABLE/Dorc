```console
$ tree design/
design/
├── world/objects.json            # named things; q ∈ {forall, exists}; meaningless except as objects of speech
├── world/same.json               # declared coherence between two speakers' objects; never assumed by name
├── speech/tessa.json             # L0: one speaker, one claim, one object; stable across coordinates
├── speech/carl.json
├── books/siblings/book.sh        # immutable bytes
├── books/siblings/world.json     # exists-objects and their measured relations; stable
├── books/siblings/expect.json    # human intent, in L0 ids and product verdicts
├── books/two-trees/{book.sh,world.json,expect.json}
├── coord/311@fc56941e/vocab.json # the coordinate's own species, relations, answers, order, dangers; data, not tool fields
├── coord/311@fc56941e/rules.als  # hand-written beside the prose; implements the laws interface
├── coord/311@fc56941e/file.json  # L1: every L0 claim filed (or explicitly unfiled) under this coordinate
├── coord/311@fc56941e/measure.json  # each book's world read into this coordinate's species
├── laws/generic.als              # tool-owned; the only hardcoding: Decl, Speaker, Danger, Ans+weaker, Query, True, answer, wrong, restsOn
├── build/                        # generated; never edited
├── lock.json                     # committed; moves only by `dsn lock accept --by <human>`
└── proposals/                    # what an LLM hands to the gate
```

```console
$ cat design/world/objects.json
[{"id":"o.tool.chmod","q":"forall"},
 {"id":"o.file","q":"forall"},
 {"id":"o.file.charlie","q":"forall"},
 {"id":"o.path","q":"forall"},
 {"id":"o.inode-number","q":"forall"},
 {"id":"o.filesystem","q":"forall"},
 {"id":"o.filesystem.ext4","q":"forall","narrows":"o.filesystem"},
 {"id":"o.directory-entry","q":"forall"}]

$ cat design/world/same.json
[{"id":"same.1","objects":["o.file","o.file.charlie"],"by":"ec","at":"2026-09-27"}]

$ cat design/speech/tessa.json
{"speaker":"tessa","seat":"describes o.file",
 "claims":[
  {"id":"tessa-1","q":"forall","about":["o.inode-number","o.file","o.filesystem"],
   "text":"an inode number names exactly one file within one filesystem, and a file has exactly one inode number"},
  {"id":"tessa-2","q":"forall","about":["o.directory-entry","o.inode-number"],
   "text":"two directory entries may name one inode number"},
  {"id":"tessa-3","q":"forall","about":["o.filesystem.ext4","o.inode-number"],
   "text":"an ext4 filesystem exposes its inode numbers through no other filesystem"}]}

$ cat design/speech/carl.json
{"speaker":"carl","seat":"describes o.tool.chmod",
 "claims":[
  {"id":"carl-1","q":"forall","about":["o.tool.chmod","o.file","o.path"],
   "text":"chmod -R may change every file beneath its path argument, and nothing else"}]}
```

```console
$ cat design/books/siblings/book.sh
cp x.conf /srv/a/one.conf
cmp golden /srv/a/two.conf

$ cat design/books/siblings/world.json
{"objects":[
  {"id":"w.fs1","q":"exists","of":"o.filesystem.ext4"},
  {"id":"w.i77","q":"exists","of":"o.inode-number"},
  {"id":"w.i78","q":"exists","of":"o.inode-number"},
  {"id":"w.e-one","q":"exists","of":"o.directory-entry"},
  {"id":"w.e-two","q":"exists","of":"o.directory-entry"}],
 "measured":[
  {"id":"m1","speaker":"world","q":"exists","text":"/srv/a/one.conf is w.e-one, names w.i77, in w.fs1"},
  {"id":"m2","speaker":"world","q":"exists","text":"/srv/a/two.conf is w.e-two, names w.i78, in w.fs1"},
  {"id":"m3","speaker":"world","q":"exists","text":"line 1 runs; line 2 is converged before line 1"}],
 "loads":["tessa","carl"]}

$ cat design/books/siblings/expect.json
{"by":"ec","at":"2026-09-17",
 "lines":{"2":{"verdict":"elide","rests_on":["tessa-1","tessa-3","carl-1"],"mutants":{"tessa-3":"guard","carl-1":"guard"}}}}

$ cat design/books/two-trees/book.sh
chmod -R g-w /srv/a
chmod -R g+w /srv/b

$ cat design/books/two-trees/world.json
{"objects":[
  {"id":"w.fs1","q":"exists","of":"o.filesystem.ext4"},
  {"id":"w.i77","q":"exists","of":"o.inode-number"},
  {"id":"w.e-a","q":"exists","of":"o.directory-entry"},
  {"id":"w.e-b","q":"exists","of":"o.directory-entry"}],
 "measured":[
  {"id":"m1","speaker":"world","q":"exists","text":"/srv/a/shared is w.e-a, names w.i77, in w.fs1"},
  {"id":"m2","speaker":"world","q":"exists","text":"/srv/b/shared is w.e-b, names w.i77, in w.fs1"},
  {"id":"m3","speaker":"world","q":"exists","text":"line 1 runs; line 2 is converged before line 1"}],
 "loads":["tessa","carl"]}

$ cat design/books/two-trees/expect.json
{"by":"ec","at":"2026-09-24","found_by":"312cc",
 "lines":{"2":{"verdict":"guard","rests_on":[],"failed_before":"311@6b7108b4"}}}
```

```console
$ cat design/coord/311@fc56941e/vocab.json
{"coord":"311@fc56941e","prose":"Research/notes/311-identity-and-relation-model.md@fc56941e",
 "species":["Referent","Sort","Scheme","Shape","Key","Store","Traversal"],
 "relations":{
   "yields":                     {"slug":"2.1-yields-into-another-scheme","on":"Scheme","fields":["to","shape"]},
   "primary-of":                 {"slug":"2.2-primary-of-and-identified-in","on":"Scheme","fields":["sort"]},
   "identified-in":              {"slug":"2.2-primary-of-and-identified-in","on":"Shape","fields":["parent-sort"]},
   "guarantees-unique-referent": {"slug":"1.5-token-and-the-two-warrants","on":"Shape","fields":[]},
   "guarantees-unique-name":     {"slug":"1.5-token-and-the-two-warrants","on":"Shape","fields":[]},
   "aliases-nothing-else":       {"slug":"2.3-aliases-nothing-else-the-store-warrant","on":"Store","fields":[]},
   "may-write":                  {"slug":"2.6-may-write-the-writeset","on":"Verb","fields":["entries","whole","record"]}},
 "answers":["SAME","DISJOINT","KNOWN_UNSPOKEN","UNKNOWN"],
 "weaker":[["UNKNOWN","SAME"],["UNKNOWN","DISJOINT"],["KNOWN_UNSPOKEN","DISJOINT"],["UNKNOWN","KNOWN_UNSPOKEN"]],
 "safe":["UNKNOWN","KNOWN_UNSPOKEN"],
 "dangers":["wSAME","wDISJ","wSPARE","stale","none"],
 "consumer":{"DISJOINT":"elide","SAME":"guard","KNOWN_UNSPOKEN":"guard","UNKNOWN":"guard"},
 "product_vocab":"verdicts@239"}

$ cat design/coord/311@fc56941e/file.json
{"coord":"311@fc56941e","by":"llm/opus","reviewed":"ec@2026-09-24",
 "filed":[
  {"id":"d1a","from":"tessa-1","seat":"tessa","as":"guarantees-unique-referent","fields":{"scheme":"sm.Inode","shape":"inode","parent-sort":"sm.Filesystem"},"danger":"wSAME"},
  {"id":"d1b","from":"tessa-1","seat":"tessa","as":"guarantees-unique-name",    "fields":{"scheme":"sm.Inode","shape":"inode","parent-sort":"sm.Filesystem"},"danger":"wDISJ"},
  {"id":"d2", "from":"tessa-2","seat":"tessa","as":"guarantees-unique-name","withheld":true,"fields":{"scheme":"sm.Path","shape":"path"},"danger":"none"},
  {"id":"d3", "from":"tessa-3","seat":"tessa","as":"aliases-nothing-else","fields":{"store-sort":"sm.Filesystem"},"danger":"wDISJ"},
  {"id":"d4", "from":"carl-1", "seat":"carl", "as":"may-write","fields":{"verb":"chmod -R","entries":["argv[1]"],"whole":true,"record":true},"danger":"wSPARE"},
  {"id":"d5", "from":"carl-1", "seat":"carl", "as":"may-write","fields":{"verb":"cp","entries":["argv[2]"],"whole":false,"record":true},"danger":"wSPARE"}],
 "unfiled":[]}

$ cat design/coord/311@fc56941e/measure.json
{"coord":"311@fc56941e",
 "books":{
  "siblings":{
    "m1":{"key":{"scheme":"sm.Path","value":"/srv/a/one.conf"},"yields":{"scheme":"sm.Inode","value":"w.i77","parent":"w.fs1"},"traversal":["/","srv","a","w.e-one"],"closure_each_level":true},
    "m2":{"key":{"scheme":"sm.Path","value":"/srv/a/two.conf"},"yields":{"scheme":"sm.Inode","value":"w.i78","parent":"w.fs1"},"traversal":["/","srv","a","w.e-two"],"closure_each_level":true},
    "sites":{"1":{"writeset":["m1.key"],"decl":"d5"},"2":{"readset":["m2.key"]}}},
  "two-trees":{
    "m1":{"key":{"scheme":"sm.Path","value":"/srv/a","whole":true},"traversal":["/","srv","w.e-a"],"closure_each_level":true},
    "m2":{"key":{"scheme":"sm.Path","value":"/srv/b","whole":true},"traversal":["/","srv","w.e-b"],"closure_each_level":true},
    "sites":{"1":{"writeset":["m1.key"],"decl":"d4"},"2":{"readset":["m2.key"]}}}}}
```

```console
$ cat design/laws/generic.als
module laws
open rules                      -- the coordinate; must define Query, True, answer, wrong, restsOn, and the Ans order from vocab.json

sig Speaker {}
abstract sig Danger {}
sig Decl { speaker: one Speaker, danger: one Danger }
sig Ans { weaker: set Ans }
pred allTrue[S: set Decl] { S in True }

check neverWrongWhenAllTrue { all S: set Decl, q: Query | allTrue[S] implies not wrong[S, q] } for 5
check monotoneInSpeech       { all S, S2: set Decl, q: Query | S in S2 and allTrue[S2] implies answer[S, q] in answer[S2, q].*weaker } for 5
check strangerSafe           { all S: set Decl, d: Decl, q: Query | allTrue[S + d] and d.speaker not in S.speaker implies answer[S, q] in answer[S + d, q].*weaker } for 5
check attributionHonest      { all S: set Decl, q: Query | wrong[S, q] implies some d: restsOn[S, q] | d not in True } for 5
check attributionSufficient  { all S: set Decl, q: Query | answer[restsOn[S, q], q] = answer[S, q] } for 5
check attributionMinimal     { all S: set Decl, q: Query, d: restsOn[S, q] | answer[S - d, q] != answer[S, q] } for 5

$ sed -n '1,40p' design/coord/311@fc56941e/rules.als
module rules
open laws
sig Referent, Sort, Shape {}
sig Scheme { primaryOf: lone Sort, yields: Shape -> lone Scheme }
sig Key { scheme: one Scheme, shape: one Shape, value: one univ, parent: lone Key, reaches: one Referent }
sig Store in Key {}
sig UniqueName, UniqueReferent, AliasesNothingElse, MayWrite extends Decl {}     -- one Decl subsig per vocab relation
sig UniqueName     { onShape: one Shape }
sig AliasesNothingElse { onStore: one Store }
sig Query { write, read: one Key }
sig True in Decl {}
one sig SAME, DISJOINT, KNOWN_UNSPOKEN, UNKNOWN extends Ans {}
fact orderFromVocab { weaker = UNKNOWN->SAME + UNKNOWN->DISJOINT + KNOWN_UNSPOKEN->DISJOINT + UNKNOWN->KNOWN_UNSPOKEN }
fact worldMakesTrue {                                        -- what each relation means when true; the § 0 hypothesis
  all d: UniqueName & True, a, b: Key | a.shape = d.onShape and a.parent = b.parent and a.reaches = b.reaches implies a = b
  all d: AliasesNothingElse & True, k: parent.(d.onStore), k2: Key | k2.reaches = k.reaches implies k2.parent = d.onStore }
fun answer[S: set Decl, q: Query]: one Ans { disjointByTwoTops[S, q.write, q.read] implies DISJOINT else UNKNOWN }
pred wrong[S: set Decl, q: Query] { answer[S, q] = DISJOINT and q.write.reaches = q.read.reaches or answer[S, q] = SAME and q.write.reaches != q.read.reaches }
fun restsOn[S: set Decl, q: Query]: set Decl { { d: S | answer[S - d, q] != answer[S, q] } }
pred disjointByTwoTops[S: set Decl, x, y: Key] { ... }     -- 3.2-compare-one-chokepoint-four-answers, two-tops way
```

```console
$ dsn lint design/ --coord 311@fc56941e
{"coord":"311@fc56941e","checks":[
  {"lint":"l0-filed",     "ok":true, "filed":["tessa-1","tessa-2","tessa-3","carl-1"],"unfiled":[]},
  {"lint":"seat-can-know","ok":true, "pairs":[["d3","tessa","describes o.file","aliases-nothing-else on o.filesystem.ext4"]],"warn":["d3: seat describes o.file; claim is about o.filesystem.ext4"]},
  {"lint":"world-coheres","ok":true, "same":["same.1"]},
  {"lint":"vocab-covers-file","ok":true},
  {"lint":"expect-ids-resolve","ok":true}]}

$ dsn render alloy design/ --coord 311@fc56941e --book siblings
wrote design/build/311@fc56941e/siblings.als

$ cat design/build/311@fc56941e/siblings.als
module build_siblings
open rules
one sig tessa, carl extends Speaker {}
one sig d1a extends UniqueReferent {} { speaker = tessa  danger = wSAME  onShape = inode }
one sig d1b extends UniqueName     {} { speaker = tessa  danger = wDISJ  onShape = inode }
one sig d3  extends AliasesNothingElse {} { speaker = tessa  danger = wDISJ  onStore = fs1 }
one sig d5  extends MayWrite       {} { speaker = carl   danger = wSPARE }
one sig i77, i78, fs1 extends Key {}
fact measured { i77.parent = fs1  i78.parent = fs1  i77.shape = inode  i78.shape = inode  i77.reaches != i78.reaches  fs1 in Store }
one sig q_siblings_2 extends Query {} { write = i77  read = i78 }
fact allTrue { True = Decl }
run siblings_2 { answer[Decl, q_siblings_2] = DISJOINT and restsOn[Decl, q_siblings_2] = d1b + d3 + d5 } for exactly 4 Decl, exactly 3 Key, 1 Query
run siblings_2_mutant_d3 { answer[Decl - d3, q_siblings_2] = UNKNOWN } for exactly 4 Decl, exactly 3 Key, 1 Query

$ dsn check design/ --coord 311@fc56941e
{"coord":"311@fc56941e","alloy":"6.2.0@6b8c1cb5",
 "laws":[
  {"check":"neverWrongWhenAllTrue","scope":5,"result":"no-counterexample"},
  {"check":"monotoneInSpeech",     "scope":5,"result":"no-counterexample"},
  {"check":"strangerSafe",         "scope":5,"result":"no-counterexample"},
  {"check":"attributionHonest",    "scope":5,"result":"no-counterexample"},
  {"check":"attributionSufficient","scope":5,"result":"no-counterexample"},
  {"check":"attributionMinimal",   "scope":5,"result":"no-counterexample"}],
 "vacuity":[{"law":"neverWrongWhenAllTrue","premise_satisfiable":true},{"law":"monotoneInSpeech","premise_satisfiable":true}],
 "books":[
  {"book":"siblings", "line":"2","answer":"DISJOINT","verdict":"elide","rests_on":["d1b","d3","d5"],"rests_on_l0":["tessa-1","tessa-3","carl-1"],"expect":"elide","ok":true,
   "mutants":{"d3":{"answer":"UNKNOWN","verdict":"guard","expect":"guard","ok":true},"d5":{"answer":"UNKNOWN","verdict":"guard","expect":"guard","ok":true}}},
  {"book":"two-trees","line":"2","answer":"UNKNOWN","verdict":"guard","rests_on":[],"expect":"guard","ok":true,
   "failed_before":{"coord":"311@6b7108b4","answer":"DISJOINT","ok":false}}],
 "equivalence":{"against":"311@c5e8f434","predicates":["answer"],"differs":[]},
 "status":"green"}
```

```console
$ cat design/lock.json
{"coords":{"311@fc56941e":{"prose_sha":"fc56941e","vocab_sha":"9a1f…","file_sha":"3c77…","rules_sha":"e0b2…"}},
 "l0":{"tessa-1":{"311@fc56941e":["d1a","d1b"]},"tessa-2":{"311@fc56941e":["d2"]},"tessa-3":{"311@fc56941e":["d3"]},"carl-1":{"311@fc56941e":["d4","d5"]}},
 "books":{
  "siblings": {"2":{"expect":"elide","rests_on":["tessa-1","tessa-3","carl-1"],
               "by":{"alloy@311@fc56941e":{"verdict":"elide","rests_on":["d1b","d3","d5"],"mutants_ok":true}}}},
  "two-trees":{"2":{"expect":"guard","rests_on":[],
               "by":{"alloy@311@6b7108b4":{"verdict":"elide","ok":false},"alloy@311@fc56941e":{"verdict":"guard","ok":true}}}}},
 "rules":{
  "311:3.2-compare-one-chokepoint-four-answers":{"discharge":{"cased":["siblings","two-trees"],"model-checked":"5","kill-tested":["siblings.d3","siblings.d5"]},"depends":["311:1.5","311:2.3"]},
  "311:3.5-committee-law-and-attribution":{"discharge":{"model-checked":"attributionHonest,attributionSufficient,attributionMinimal","attributed":{"by":"ec","at":"2026-09-27","residue":"who can know"}}},
  "311:1.1-referent-state-and-value#engine-never-holds-a-referent":{"discharge":{"fenced":"spike/CLAUDE.md#no-world-facts","by":"ec"}}},
 "accepted_by":"ec","at":"2026-09-27T18:02:11Z"}
```

```console
$ cat design/proposals/0007.json
{"id":"0007","by":"llm/opus","at":"2026-09-26","kind":"coord-edit","coord":"311@fc56941e",
 "intent":"answer-preserving-reword",
 "edits":[
  {"file":"prose","slug":"2.5-may-read-the-readset","replace":"…","with":"An mSort that declares no may-read set … is affected by every write: every mKey of that mSort is in every line's writeset."},
  {"file":"rules.als","patch":"@@ pred writeset @@ + all m: Sort | no m.closedMayRead implies m.keys in ws"}],
 "claims":{"answers_unchanged":true}}

$ dsn gate design/proposals/0007.json
{"proposal":"0007","status":"red",
 "laws":[
  {"check":"monotoneInSpeech","result":"counterexample","scope":5,
   "witness":{"S":["d1a","d1b","d3","d5"],"S2":["d1a","d1b","d3","d5","x1"],"x1":{"speaker":"foob","as":"unfiled-sort-key","danger":"none"},"q":"q_siblings_2","answer_S":"DISJOINT","answer_S2":"UNKNOWN"},
   "reads_as":"adding a stranger's true statement removed a survival"},
  {"check":"strangerSafe","result":"counterexample","witness":"same"}],
 "books":[{"book":"siblings","line":"2","expect":"elide","got":"guard","ok":false,"cause":"writeset gained x1; compare(x1, i78) = UNKNOWN"}],
 "equivalence":{"against":"311@fc56941e","predicates":["answer"],"differs":[{"q":"q_siblings_2","before":"DISJOINT","after":"UNKNOWN"}],
                "contradicts_claim":"answers_unchanged"},
 "triage":"model","lock_diff":null}
```

```console
$ cat design/proposals/0008.json
{"id":"0008","by":"llm/fable","at":"2026-09-17","kind":"file-edit","coord":"311@fc56941e",
 "intent":"re-seat","found_by":"311p:thr-aliases-nothing-else-is-the-parents-knowledge",
 "edits":[
  {"file":"speech/simon.json","add":{"speaker":"simon","seat":"describes o.filesystem","claims":[{"id":"simon-1","q":"forall","about":["o.filesystem.ext4","o.inode-number"],"text":"an ext4 filesystem exposes its inode numbers through no other filesystem"}]}},
  {"file":"speech/tessa.json","retire":"tessa-3","superseded_by":"simon-1"},
  {"file":"file.json","replace":{"id":"d3","from":"simon-1","seat":"simon","as":"aliases-nothing-else","fields":{"store-sort":"sm.Filesystem"},"danger":"wDISJ"}}]}

$ dsn gate design/proposals/0008.json
{"proposal":"0008","status":"needs-accept",
 "lint":[{"lint":"seat-can-know","ok":true,"warn":[]},{"lint":"l0-filed","ok":true,"retired":["tessa-3"],"filed":["simon-1"]}],
 "laws":"all no-counterexample",
 "books":[{"book":"siblings","line":"2","verdict":"elide","ok":true,"rests_on_l0":{"-":["tessa-3"],"+":["simon-1"]}},{"book":"two-trees","line":"2","verdict":"guard","ok":true}],
 "equivalence":{"differs":[]},
 "lock_diff":[{"path":"books.siblings.2.rests_on","-":["tessa-3"],"+":["simon-1"]},{"path":"l0.tessa-3","-":{"311@fc56941e":["d3"]},"+":"retired"},{"path":"l0.simon-1","+":{"311@fc56941e":["d3"]}}],
 "needs":"human"}

$ dsn lock accept design/proposals/0008.json --by ec
lock.json: 3 paths changed; accepted_by=ec at=2026-09-27T18:40:03Z
$ git commit -m '(dsn) Re-seat the store closure on the filesystem describer' design/
```

```console
$ dsn coord new 311@9d4e2b1 --from 311@fc56941e
{"coord":"311@9d4e2b1","carried":{"vocab":"unchanged","file":"re-key required","measure":"re-key required"},"l0":{"unfiled":["tessa-1","tessa-2","carl-1","simon-1"]}}
$ dsn lint design/ --coord 311@9d4e2b1
{"checks":[{"lint":"l0-filed","ok":false,"unfiled":["tessa-1","tessa-2","carl-1","simon-1"]}],"status":"red"}
```

```console
$ cat design/coord/312@a1b2c3/spell.json
{"coord":"312@a1b2c3","over":"311@9d4e2b1","by":"llm/opus","reviewed":"ec@2026-10-08",
 "spellings":{
  "d1a":{"file":"stdlib/fs.oracle.sh","span":[41,44],"count":1},
  "d1b":{"file":"stdlib/fs.oracle.sh","span":[41,44],"count":1},
  "d3": {"file":"stdlib/fs.oracle.sh","span":[120,131],"count":1},
  "d5": {"file":"stdlib/coreutils.oracle.sh","span":[212,219],"count":1}},
 "one_syllable":{"ok":true,"max_count":1}}

$ sed -n '120,131p' stdlib/fs.oracle.sh
sm_Filesystem__resolve() {
   case "$1" in
   ext4:*) ... ;;
   esac
}

$ dsn render sh design/ --coord 312@a1b2c3 --book siblings
wrote design/build/312@a1b2c3/siblings/{oracles/fs.oracle.sh,oracles/coreutils.oracle.sh,book.sh,fixture.sh,map.json}
$ cat design/build/312@a1b2c3/siblings/map.json
{"d1a":"oracles/fs.oracle.sh:41-44","d3":"oracles/fs.oracle.sh:120-131","d5":"oracles/coreutils.oracle.sh:212-219","m1":"fixture.sh:3","m2":"fixture.sh:4"}
$ cat design/build/312@a1b2c3/siblings/fixture.sh
mkfs.ext4 /dev/loop0 && mount /dev/loop0 /srv
touch /srv/a/one.conf /srv/a/two.conf
```

```console
$ dsn recover design/ --loader dorc@b81f2 --coord 312@a1b2c3 --book siblings
{"loader":"dorc@b81f2","book":"siblings",
 "emitted":[
  {"as":"guarantees-unique-referent","fields":{"scheme":"sm.Inode","shape":"inode","parent-sort":"sm.Filesystem"},"source":"oracles/fs.oracle.sh:41-44"},
  {"as":"guarantees-unique-name","fields":{"scheme":"sm.Inode","shape":"inode","parent-sort":"sm.Filesystem"},"source":"oracles/fs.oracle.sh:41-44"},
  {"as":"aliases-nothing-else","fields":{"store-sort":"sm.Filesystem"},"source":"oracles/fs.oracle.sh:120-131"},
  {"as":"may-write","fields":{"verb":"cp","entries":["argv[2]"],"whole":false,"record":true},"source":"oracles/coreutils.oracle.sh:212-219"}],
 "matched":{"d1a":"by-content","d1b":"by-content","d3":"by-content","d5":"by-content"},
 "unmatched_emitted":[],"unmatched_filed":[],
 "status":"green"}

$ dsn render kani design/ --coord 311@9d4e2b1 --book siblings
wrote spike/verify/kani/src/harness/design_books.rs
$ sed -n '1,14p' spike/verify/kani/src/harness/design_books.rs
#[kani::proof]
fn design_book_siblings_line_2() {
    let decls = design::load("311@9d4e2b1", "siblings");            // d1a d1b d3 d5, by id, from file.json
    let q = design::query("siblings", 2);
    assert_eq!(core::compare(&decls, &q), Answer::Disjoint);
    assert_eq!(core::rests_on(&decls, &q), ids!["d1b", "d3", "d5"]);
    for d in ["d3", "d5"] { assert_ne!(core::compare(&decls.without(d), &q), Answer::Disjoint); }
}

$ dsn run hostsim design/ --coord 312@a1b2c3 --book siblings
{"book":"siblings","host":"hostsim@seed=7","line":"2","verdict":"elide","rests_on":["d1b","d3","d5"],"ok":true}
$ dsn run e2e design/ --coord 312@a1b2c3 --book siblings --host vm-ext4-01
{"book":"siblings","host":"vm-ext4-01","fixture":"fixture.sh","line":"2","verdict":"elide","ok":true,"world_measured":{"m1":"/srv/a/one.conf inode 262151","m2":"/srv/a/two.conf inode 262152"}}
$ dsn lock diff
[{"path":"books.siblings.2.by","+":{"kani@311@9d4e2b1":{"ok":true},"hostsim@312@a1b2c3":{"ok":true},"e2e@312@a1b2c3@vm-ext4-01":{"ok":true}}},
 {"path":"rules.311:3.2-compare-one-chokepoint-four-answers.discharge","+":{"pinned":"design_book_siblings_line_2","demonstrated":["hostsim","e2e"]}},
 {"path":"l0.tessa-3","note":"retired 0008; no instrument cites it"}]
```
