# Rare shapes worth copying

Companion to `SKILL.md`. Each entry is a shape that carries value an agent is unlikely to
reinvent under time pressure, and unlikely to find in whatever model it is about to open. Each
says when to reach for it and which failure it avoids. Vocabulary follows Practical Alloy's
file-system, file-sharing and leader-election models so the originals can be checked. Indentation
is three spaces by this repository's habit; Alloy does not care.

## The consistency run and the premise twin

When: every model, from the first signature onward. Avoids: inconsistent facts turning every check
green; an unsatisfiable premise doing the same for one check.

```alloy
abstract sig Object {}
sig Dir extends Object { entries : set Entry }
sig File extends Object {}
one sig Root extends Dir {}
sig Entry { object : one Object, name : one Name }
sig Name {}

fact one_directory_per_entry { all e : Entry | one entries.e }
fact unique_names { all d : Dir, n : Name | lone (d.entries & name.n) }

run consistent {} for 4 expect 1                                   -- keep for the life of the model
run consistent_nontrivial { some File and some Dir - Root } for 4 expect 1

pred no_self_containment { all d : Dir | d not in d.entries.object }
assert self_containment_suffices {
   no_self_containment implies all o : Object | o in Root + Root.^(entries.object)
}
check self_containment_suffices for 6 expect 1                     -- known red: a File no entry reaches, or two dirs containing each other
run self_containment_premise { no_self_containment and some Dir - Root } for 6 expect 1
```

The twin asks for the strongest world the law covers (a non-root directory exists), not merely
any world. A twin with no instance means the check above it has never been read.

## The labelled empty check

When: reviewing any file. Avoids: a green that checks `true`.

```alloy
assert no_partitions { all o : Object | o in Root + Root.^(entries.object) }
check no_partitions for 6          -- checks the assertion
check no_partitions {} for 6       -- checks the empty block; can never fail
```

## A concrete instance as a unit test

When: pinning a situation the prose described, positive or negative. Avoids: `one sig` atoms
that pollute every command's universe and break symmetry; a run that is unsatisfiable only
because the scope cannot seat it.

```alloy
run test_root_file_dir {
   some disj d0, d1 : Dir, disj f0, f1 : File, disj e0, e1, e2 : Entry, disj n0, n1, n2 : Name {
      Root    = d0
      Dir     = d0 + d1
      File    = f0 + f1
      Entry   = e0 + e1 + e2
      Name    = n0 + n1 + n2
      entries = d0->e0 + d0->e1 + d1->e2
      name    = e0->n0 + e1->n1 + e2->n2
      object  = e0->d1 + e1->f0 + e2->f1
   }
} for 4 Object, 3 Entry, 3 Name expect 1

run test_root_file_dir_bad_name {                                  -- same shape, one name shared; unique_names rejects it
   some disj d0, d1 : Dir, disj f0, f1 : File, disj e0, e1, e2 : Entry, disj n0, n2 : Name {
      Root    = d0
      Dir     = d0 + d1
      File    = f0 + f1
      Entry   = e0 + e1 + e2
      Name    = n0 + n2
      entries = d0->e0 + d0->e1 + d1->e2
      name    = e0->n0 + e1->n0 + e2->n2
      object  = e0->d1 + e1->f0 + e2->f1
   }
} for 4 Object, 3 Entry, 2 Name expect 0
```

`disj` is repeated per group. Pinning every signature to the union of its variables excludes
stray atoms. The Analyzer's "Export to Predicate" produces exactly this shape from any instance,
which is how a counterexample becomes a regression test after the fix.

The negative test above is `expect 0`, and a scope too small to seat it is also unsat, so it can
pass for the wrong reason; the positive twin beside it at the same scope is what keeps it honest.
With the rule as a predicate instead of the `unique_names` fact (an alternative to the module
above, not an addition to it), the rejection test cannot be faked that way:

```alloy
pred names_apart { all d : Dir, n : Name | lone (d.entries & name.n) }

run rejects_shared_name {                          -- sat means rejected; a starved scope is unsat and loud
   some d0 : Dir, disj e0, e1 : Entry, n0 : Name {
      Dir = d0  Entry = e0 + e1  Name = n0
      entries = d0->e0 + d0->e1
      name    = e0->n0 + e1->n0
   }
   not names_apart
} for 3 expect 1
```

The valuation is partial (`File` and `object` are left to the solver), which is allowed; a partial
valuation stands for every completion.

## A hand-written total order

When: comparable identifiers where smaller configurations must still be checked. Avoids:
`util/ordering` on the ring's own signature making the scope exact and non-empty, so a scope of
three checks only rings of exactly three.

```alloy
sig Node { next : lone Node, succ : one Node }
one sig first, last in Node {}

fact ordering {
   no next.first and no last.next
   Node - first in first.^next
}
-- "i is greater than n" is then `i in n.^next`; no `gt` needed.
```

## Effects as equalities, frames for everything else

When: any event. Avoids: an inclusion effect that lets the rest of the set drift; an unmentioned
relation changing freely; a frame that contradicts the effect when two parameters coincide.

```alloy
pred initiate [n : Node] {
   historically n.id not in n.outbox              -- guard, using history instead of a phase field
   n.outbox' = n.outbox + n.id                    -- effect: equality, never `n.id in n.outbox'`
   all m : Node - n | m.outbox' = m.outbox        -- frame on the other outboxes
   inbox' = inbox                                 -- frame
   Elected' = Elected                             -- frame
}

-- Pointfree effect with a conditional addition: `i & n.^next` is `i` or empty.
pred process [n : Node, i : Node] {
   i in n.inbox
   inbox' = inbox - n->i + n.succ->(i & n.^next)
}

-- Pointwise, the frame must not contradict the effect when n.succ = n (a ring of one node).
pred process_pointwise [n : Node, i : Node] {
   i in n.inbox
   n.inbox' = n.inbox - i
   i in n.^next implies n.succ.inbox' = n.succ.inbox + i
                else    n.succ != n implies n.succ.inbox' = n.succ.inbox
   all m : Node - n - n.succ | m.inbox' = m.inbox
}
```

The one-node case is the shape the book's own authors missed: three checks stayed green because
the fairness premise, written from the guard, excused an event that could never fire. A `run`
demanding the event at `exactly 1 Node` caught it.

## The two sanctioned macros

When: many events, many frame conditions, several fairness assumptions. Avoids: a frame or a
fairness schema hand-expanded as text, which primes only the last factor or binds `always` to half
of an `or`.

```alloy
let unchanged[x] { x = (x)' }                     -- parenthesised as the book writes it
let fair[ev] { always (eventually (ev)) }         -- likewise

pred upload [f : File] {
   f not in uploaded
   uploaded' = uploaded + f
   unchanged[trashed]
   unchanged[shared]
}
```

The book calls macros textual expansion and says the parentheses are required; under the pinned
6.2.0 jar the bare forms `x = x'` and `always eventually ev` tested equivalent to these, so the
jar substitutes the parsed argument. Keep the parentheses as the portable spelling. Errors inside
macros are cryptic; these two are the whole recommended repertoire.

## Stuttering, transitions, and fairness as a premise

When: any behavioural model with a liveness property. Avoids: no infinite trace once actions run
out; a fairness fact taxing every safety check; an enabledness written from the guard alone.

```alloy
pred stutter { outbox' = outbox  inbox' = inbox  Elected' = Elected }

pred node_acts [n : Node] {
   initiate[n] or (some i : Id | send[n, i]) or (some i : Id | process[n, i])
}

fact events { always (stutter or some n : Node | node_acts[n]) }

pred node_enabled [n : Node] {
   (historically n.id not in n.outbox) or some n.inbox or some n.outbox
}

pred fairness {                                   -- weak fairness per node
   all n : Node | (eventually always node_enabled[n]) implies (always eventually node_acts[n])
}

assert at_most_one_leader { always lone Elected }
check at_most_one_leader for 4 but 20 steps

assert at_least_one_leader_fair { fairness implies eventually some Elected }
check at_least_one_leader_fair for 3 but 10 steps   -- 20 steps takes 100 s to translate and times out under the runner's caps
```

Unconditional fairness is `always eventually A`; strong fairness is
`(always eventually en) implies always eventually A`.

## The inductive-invariant refactor

When: an invariant check over unbounded traces is slow. Avoids: facts that hide the states
induction must consider; a premise twin that searches forever for an unreachable state.

```alloy
pred init  { no uploaded  no shared }
pred next  {
   (some f : File | upload[f] or delete[f] or restore[f]) or
   (some f : File, t : Token | share[f, t]) or
   (some t : Token | download[t]) or
   empty or stutter
}
pred traces { init  always next }                 -- the premise of every ordinary temporal check

pred inv_shared_are_accessible { shared.Token in uploaded - trashed }

assert init_inv { init implies inv_shared_are_accessible }
assert pres_inv { (inv_shared_are_accessible and next) implies after inv_shared_are_accessible }
check init_inv for 10 but 1 steps
check pres_inv for 10 but 2 steps                 -- 1.2 s, where a 20-step trace check at scope 10 times out

assert shared_are_uploaded { traces implies always shared.Token in uploaded }
check shared_are_uploaded for 4 but 20 steps      -- the non-inductive one stays a bounded trace check
```

A preservation counterexample may be an unreachable state; strengthen the invariant with one
known to be inductive rather than "confirming" reachability with a run. `for 1.. steps` needs a
complete model checker the pinned jar does not ship; it errors here.

## Derived state from history

When: a relation whose value is a function of what happened. Avoids: a stored `var` with an
effect and a frame in every event.

```alloy
fun Elected : set Node {
   { n : Node | once (before n in n.inbox and n not in n.inbox) }
}

fun Elected_by : Node -> Node {                   -- who each node believes is leader
   { n, i : Node |
      let inbox_elected = payload.i & ElectedMsg & n.inbox |
         once (before some inbox_elected and no inbox_elected) }
}
```

## Meta-signature init, stutter, and frames

When: a model whose set of mutable relations keeps growing. Avoids: an event that forgets the
newest relation; an init fact that misses one.

```alloy
fact init { all v : var$ | no v.value }
pred stutter { all v : var$ | v.value = v.value' }

pred upload [f : File] {
   f not in uploaded
   uploaded' = uploaded + f
   all v : var$ - uploaded$ | v.value = v.value'
}

run everything_happens { all v : var$ | eventually some v.value }
```

`sig$`, `field$`, `var$`, `static$`, and the `value`, `parent`, `fields`, `subfields` meta-fields
appear once any of them is mentioned.

## Event depiction as derived relations

When: reading traces, or asserting that events are exclusive. Avoids: guessing which event fired
from state diffs; an existentially quantified predicate call where a set would do.

```alloy
enum Event { Empty, Upload, Delete, Restore, Share, Download, Stutter }

-- One per event; delete, restore and download follow the same shape and are elided here.
fun empty_happens   : set Event        { { e : Empty | empty } }
fun stutter_happens : set Event        { { e : Stutter | stutter } }
fun upload_happens  : Event -> File    { { e : Upload, f : File | upload[f] } }
fun share_happens   : Event -> File -> Token { { e : Share, f : File, t : Token | share[f, t] } }

fun events : set Event {
   empty_happens + stutter_happens + upload_happens.File + share_happens.Token.File
}

fact transitions { always some events }                     -- every event listed, or the missing ones can never fire
check at_most_one_event { always lone events } for 3 expect 1   -- finds a step where two events coincide; the book's does too
run two_shares_in_a_row { eventually (some share_happens and after some share_happens) } for 3
```

Functions and predicates live in different namespaces, so `fun upload` beside `pred upload` is
legal and lets `some upload` mean "an upload happens".

## Trace scenarios with the sequence operator

When: pinning a specific execution, fully or as a family. Avoids: nested `after` or towers of
primes; a pinned prefix that still admits any continuation.

```alloy
pred two_tokens [f : File, t0, t1 : Token] { File = f  Token = t0 + t1 }

run scenario_state_wise {
   some f : File, disj t0, t1 : Token {
      two_tokens[f, t0, t1]
      no uploaded; uploaded = f; uploaded = f;   uploaded = f;           uploaded = f; always no uploaded
      no shared;   no shared;    shared = f->t0; shared = f->t0 + f->t1; no shared;    always no shared
   }
} for 1 File, 2 Token

run scenario_event_wise {
   some f : File, disj t0, t1 : Token {
      two_tokens[f, t0, t1]
      upload[f]; share[f, t0]; share[f, t1]; delete[f]; empty; always stutter
   }
} for 1 File, 2 Token
```

`p; q` is `p and after q`, lowest precedence of all. Leave a relation out to get a family of
scenarios rather than one; close with `always` on the last state to forbid continuations.

## Generator and uniqueness for record-like signatures

When: messages or records "created" during execution are signatures. Avoids: forks that fail
because no atom of the needed type and payload exists in the configuration; configuration
iteration that only permutes the available records.

```alloy
abstract sig Message { payload : one Node }
sig CandidateMsg, ElectedMsg extends Message {}

pred generator {
   all n : Node {
      some m : CandidateMsg | m.payload = n
      some m : ElectedMsg   | m.payload = n
   }
}
pred unique {
   all m1, m2 : CandidateMsg | m1.payload = m2.payload implies m1 = m2
   all m1, m2 : ElectedMsg   | m1.payload = m2.payload implies m1 = m2
}

pred initiate [n : Node] {
   historically no CandidateMsg & payload.n & n.succ.inbox
   some m : CandidateMsg & payload.n | inbox' = inbox + n.succ->m    -- "creation" is a choice
}

run example_unique_generator { generator  unique } for 3 Node, 10 Message
```

Keep them predicates for validation runs; promote to facts only once the message scope is known
to seat every combination. Too small a message scope makes runs unsatisfiable and checks vacuous.

## Messages as tuples

When: payloads are small and analysis is slow. Avoids: the generator and scope reasoning above;
an order of magnitude of solver time in the book's measurement.

```alloy
abstract sig Type {}
one sig Candidate, Elect extends Type {}

sig Node { succ : one Node, next : lone Node, var inbox : Type -> Node }

pred initiate [n : Node] {
   historically Candidate->n not in n.succ.inbox
   inbox' = inbox + n.succ->Candidate->n
}

pred processCandidate [n : Node, i : Node] {
   Candidate->i in n.inbox
   inbox' = inbox - n->Candidate->i + n.succ->Candidate->(i & n.^next) + n.succ->Elect->(n & i)
}

-- Payloads of unequal arity are padded to one arity with a singleton; `->` binds tighter than
-- `+`, so the union is parenthesised, and the field takes its own name beside `Node.inbox`.
sig Payload {}
one sig Empty {}
sig Node2 { var inbox2 : Type -> Node -> (Payload + Empty) }
```

Arity above three costs the solver on the order of two to the n-squared booleans; this idiom
does not scale to rich records.

## Memoization instead of recursion

When: a value that is naturally recursive (depth, rank). Avoids: recursion, which the Analyzer
refuses outright by default and, with the recursion-depth option on, silently returns `none` past
three unrollings, making the verdict option-dependent.

```alloy
open util/natural

abstract sig Object { depth : one Natural }

fact calculate_depth {
   all o : Object |
      o in Root implies o.depth = Zero
      else o.depth = inc[max[(entries.object.o).depth]]
}

run depth4 { some f : File | f.depth = inc[inc[inc[One]]] } for 5 but 3 Name
```

`Natural` is bounded by its own scope and `inc` past it is silently empty; the scope on `Natural`
must exceed the deepest value asked for. On Windows the pinned jar refuses to parse `util/natural`
at all (an alias clash inside the library file); run it under WSL.

## Higher-order quantification: what runs

When: a property over subsets or relations. Avoids: an error, or a solve that enumerates every
combination.

```alloy
run has_self_loop { some e : edge | e = ~e }                      -- outermost some in a run: solved
check no_self_loops { no e : edge | e = ~e }                      -- outermost no/all in a check: solved
check all_entries_same_name { all s : set Entry | lone s.name }   -- negated to an existential: solved
-- `all` over a set or relation inside a run, `some` inside a check, or either nested under a
-- quantifier of the other kind, is refused; nested under the same kind it solves. Rewrite the
-- refused shape with an explicit subset signature.
```

## Integer guards that stay sound

When: sizes or counts are unavoidable. Avoids: `sum` over a set deduplicating equal sizes; the
Prevent Overflows option hiding a false check.

```alloy
one sig Capacity in Int {}
abstract sig Object { size : one Int }

fact positive_sizes { all f : File | gt[f.size, 0] }
fact below_capacity { (sum f : File | f.size) <= Capacity }      -- not `sum File.size`
fact size_limits    { all f : File | f.size <= div[max, #File] } -- no instance can overflow

run full_root { #(Root.entries) = 3 } for 4 but 5 Int              -- bitwidth set explicitly
```

## Five one-line idioms

```alloy
some Root and Root in Dir            -- membership when Root is declared lone
(A <: iden) in R                     -- reflexivity on A; `iden in R` ranges over univ
no R                                 -- emptiness at any arity; `R = none` is an arity error
entries in Dir lone -> Entry         -- injective, stated on any expression, not only a field
object :> Dir in Entry lone -> Dir   -- injective on a restricted range
```

## The one/lone multi-variable witness

When: reviewing a counting claim. Avoids: reading `one x, y | P` as the nested form.

```alloy
sig A { r : set A }
-- With r = A0->A0 + A0->A1 + A1->A0:
--    one x, y : A | x->y in r             is false  (three pairs satisfy it)
--    one x : A | one y : A | x->y in r    is true   (A1 has exactly one partner)
-- Prefer some and all when several variables are bound.
```

## Bounding a subset signature

When: a `sig S in T {}` needs a size. Avoids: the error that subset signatures take no scope.

```alloy
sig Plant {}
sig Seedling in Plant {}
run two_seedlings { #Seedling = 2 } for 4 Plant
```
