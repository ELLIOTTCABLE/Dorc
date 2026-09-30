# 312b-exercises/precise-writes-inside-an-opaque-carrier — a precise write below a shared carrier

Exercise record for `312f`, after the mechanization of 311. This is one concrete admin book,
read at several description levels. The human acked the problem statement, not a repair.
Claims are graded +SURE, ~SUSPECT, -GUESS, or --WONDER where that distinction matters.

The book uses hypothetical, specified tools. Their invocation shapes are ordinary sh. Their
contracts below are assumptions of this exercise, not claims about an installed program.
Nothing here was executed against `/srv`, and no setup should be run on an existing deployment.
Oracle syntax is illustrative, not a proposal for the 312 language. It follows the nearby
exercise records: `local` binds with trailers, typed report records, and explicit closures.
A record with `nothing-else` is an engineer's dangerous statement, not a theorem.

## § 1-one-file-two-languages

Alice runs a worker supervisor. Its configuration is `/srv/worker/capsule.json`. The supervisor
reads JSON, but passes the `worker_ini` string to a worker whose configuration language is INI.
The supervisor does not interpret the INI keys. The worker does not interpret the JSON envelope.
This exercise manages the persisted configuration only. Reloading either process is not in this
book, and no conclusion about live daemon state follows from a converged file.

A fresh test fixture could be created as follows. Setup precedes Dorc's probe pass and is not
part of the book being optimized.

```sh
mkdir -p /srv/worker
cat > /srv/worker/capsule.json <<'JSON'
{"label":"blue","worker_ini":"[worker]\nenabled=0\ntrace=0\n"}
JSON
```

The bytes between the JSON quotes contain `\n` escape pairs. The decoded inner document is:

```ini
[worker]
enabled=0
trace=0
```

There is one carrier inode, not a second live INI file. A temporary decoded copy used by a tool
is not a new source of configuration truth. On this morning `enabled=0` is drifted and
`trace=0` is already desired. In this exact fixture, the digits are at zero-based carrier byte
positions 48 and 57, and decoded positions 17 and 25. The fixture is 63 bytes including its
final LF. A static byte substitution at position 48 preserves valid JSON and changes only
`enabled=0` to `enabled=1` in the decoded document. This checks the fixture's geometry, not the
hypothetical tools' contracts.

## § 2-the-book-and-the-tool-contracts

Alice's complete book is:

```sh
#!/bin/sh
set -eu
jsonslot --file /srv/worker/capsule.json --member worker_ini --patch -- inictl set-bit worker.enabled 1
jsonslot --file /srv/worker/capsule.json --member worker_ini --patch -- inictl set-bit worker.trace 0
```

For this exercise, `jsonslot` and `inictl` have these precise contracts:

- `jsonslot --patch` exposes one decoded JSON string to its guest. The guest reads the decoded
  document on stdin and returns a replacement decoded document on stdout. The adapter consumes
  that stdout; it emits no successful output of its own.
- `inictl set-bit KEY BIT` handles only an existing, unique key with a one-character `0` or `1`
  value in the supported INI profile. It preserves every other decoded character. It emits the
  resulting document and returns zero. Unsupported input produces an error and no replacement.
- The adapter handles only a byte-for-byte recognized envelope profile with two admitted bit
  slots. It translates a one-digit change into a one-byte in-place patch. It neither reformats
  JSON nor replaces the inode. No change means no write, including no timestamp update.
- A failed guest causes no patch. Malformed output, several changes, a changed length, or a
  change outside an admitted bit slot causes no patch. An I/O failure after a patch begins is
  still a failure and makes no claim of transactional rollback.
- The file is an ordinary writable fixture with stable identity during the book. There are no
  concurrent writers or configured reactions. Reads use a no-atime fixture, so the read-only
  helper invocations do not introduce an atime write in this example.
- `jsonslot --read -- ...` supplies the decoded document to a read-only guest and relays its
  status without applying output. `inictl is-bit KEY BIT` answers 0 for a match, 1 for a
  mismatch, and at least 2 for a refusal. It emits no successful bytes.

The narrow patch mode is not a claim that a generic `jq` rewrite has these properties. A
serializer that replaces the file, reorders fields, or normalizes whitespace belongs to a
coarser case below. The narrow mode is plausible tool design and requires no engine knowledge
of either language. The tools themselves enforce their supported profile.

Both outer invocations have unused successful stdout and status compatible with `set -e`.
Their own verdicts can license their own elision. The inner guest's replacement stdout is
consumed by the adapter: no claim here licenses eliding that inner command while the adapter
still needs its output. The planned unit is the complete outer invocation.

## § 3-actors-and-the-shared-objects

Alice is the admin. Inez describes INI and `inictl`; Jules describes JSON and `jsonslot`; Tessa
owns the file-part vocabulary. Inez and Jules publish reusable helpers. Jules deliberately
sources Inez's supported helper entrypoint when describing this composition. That is explicit
custody of the composite tool answer, not the engine pooling strangers' judgments.

Their identity descriptions still have separate accountable owners. Each mapping names the
next shared vocabulary, not the other person's implementation. Another inner language can use
Jules's string-coordinate mapping without teaching Jules its grammar.

Use these ordinary objects, not new model species:

| name in this example | ordinary 311 representation | meaning |
| --- | --- | --- |
| F | an mReferent reached by a file mKey | the carrier inode |
| B | an mKey for a virtual document | the decoded `worker_ini` view, a catalog for locations |
| P | an mKey-Primary identified in F | the carrier byte holding the `enabled` digit |
| Q | another mKey-Primary identified in F | the carrier byte holding the `trace` digit |
| L | a finite set of mKeys-Primary identified in F | the remaining bytes needed to recognize and locate the supported layout |
| M, C | mKeys-Primary identified in F | the carrier's modification-time and change-time fields |

Tessa exposes payload-byte positions and the relevant metadata fields in one primary mScheme,
`sm.dorc.FilePartId`, over an ordinary mSort `sm.dorc.FilePart`. Its discriminated keys are
opaque to Dorc. Its owner warrants their separation. A byte and a timestamp are not separated
because the engine recognizes a number or a prefix.

The mKeys P and Q name storage bytes, not logical boolean settings. In this admitted encoding,
one decoded digit is one literal carrier byte. An mScheme named `org.inez.IniValueByte` denotes
that location. It may yield through `org.jules.JsonStringByte` into Tessa's byte mScheme because
all three names denote the same storage mReferent. This construction is not valid unchanged for
an escaped character, a derived value, or a value spanning several bytes. Those descriptions
need multiple read/write entries or another ordinary aggregate description; they must not use
`:yields` to assert false sameness.

A logical setting can instead be its own virtual mSort whose declared may-read entries name
carrier bytes. That is another available description. It is not necessary for this bit-slot
case, and this exercise introduces no new property, region, or range species.

## § 4-the-oracles-as-local-speech

The helpers below are fictional read-only queries with explicit result contracts. They are not
unexplained permission mints. Their job is to expose the evidence a knowledgeable author could
obtain. `DORC_REPORT` below means `${DREP_V1:-/dev/null}`. Instance arguments are shown explicitly
for readability; this is not a decision about the eventual callback ABI.

### § 4.1-inez-knows-the-inner-language

Inez's location helper accepts a supported virtual document and a key. It identifies the one
value byte and reports the syntax locations the lookup depends on. It does not know a host
filename or JSON escaping. Under the fixed profile, the value positions follow from layout,
not from the current values of the two digits.

```sh
# inez-ini.oracle.sh — illustrative declarations and helper API
org_inez_IniValueByte__resolve() {
   local key="$1"
   local document="$2"
   local position
   position=$(inictl locate-bit --document "$document" "$key") || return 2
   printf 'yields org.jules.JsonStringByte:%s identified-in %s\n' \
      "$position" "$document" >>"${DREP_V1:-/dev/null}"
   inictl report-location-reads --document "$document" "$key" >>"${DREP_V1:-/dev/null}" || return 2
   printf 'lookup-reads nothing-else\n' >>"${DREP_V1:-/dev/null}"
}
inez__set_bit_key() {
   [ "$#" -eq 3 ] && [ "$1" = set-bit ] || return 2
   case "$3" in 0|1) ;; *) return 2 ;; esac
   printf '%s\n' "$2"
}
inictl__may_write() {
   local key
   key=$(inez__set_bit_key "$@") || return 2
   local target="$key" : is "org.inez.IniValueByte"
   printf 'may-write org.inez.IniValueByte:%s\n' "$target" >>"${DREP_V1:-/dev/null}"
   printf 'may-write nothing-else\n' >>"${DREP_V1:-/dev/null}"
}
```

`lookup-reads nothing-else` is an illustrative name for the missing read-closure spelling,
not a shipped API. The reported read set must account for section names, duplicate-key rules,
and any parser-wide validation that can change the answer. A general INI parser may need much
more than this fixture's L. The tool's fixed-layout profile is what makes this example narrow.

Inez's guest-effect closure concerns the virtual document. It does not claim that patching a
host file updates no timestamps. That outer invocation has Jules's separate closure.

### § 4.2-jules-knows-the-envelope-and-the-writeback

Jules maps one unescaped decoded byte to its carrier position and supplies the carrier file as
its parent instance. The one-to-one arm declines other encodings. The same lookup is useful for
any guest language that names a byte in the decoded string.

```sh
# jules-json.oracle.sh — helper inclusion is explicit custody
. ./inez-ini.oracle.sh

org_jules_JsonStringByte__resolve() {
   local position="$1"
   local document="$2"
   local carrier offset
   carrier=$(jsonslot describe-carrier "$document") || return 2
   offset=$(jsonslot locate-literal-byte "$document" "$position") || return 2
   printf 'yields sm.dorc.FilePartId:data-byte/%s identified-in sm.dorc.Path:%s\n' \
      "$offset" "$carrier" >>"${DREP_V1:-/dev/null}"
   jsonslot report-location-reads "$document" "$position" >>"${DREP_V1:-/dev/null}" || return 2
   printf 'lookup-reads nothing-else\n' >>"${DREP_V1:-/dev/null}"
}

jsonslot__is_converged() {
   jules__admit_patch_invocation "$@" || return 2
   local document
   document=$(jsonslot describe-document "$2" "$4") || return 2
   local key="$9" : is "org.inez.IniValueByte" identified-in "$document"
   jsonslot --file "$2" --member "$4" --read -- inictl is-bit "$key" "${10}" \
      : asserts "org.inez.IniValueByte:$key"
}

jsonslot__may_write() {
   jules__admit_patch_invocation "$@" || return 2
   local carrier="$2" : is "sm.dorc.Path"
   local document key
   document=$(jsonslot describe-document "$2" "$4") || return 2
   key=$(inez__set_bit_key "$8" "$9" "${10}") || return 2
   local target="$key" : is "org.inez.IniValueByte" identified-in "$document"
   printf 'may-write org.inez.IniValueByte:%s\n' "$target" >>"${DREP_V1:-/dev/null}"
   printf 'may-write sm.dorc.FilePartId:mtime identified-in sm.dorc.Path:%s\n' "$carrier" >>"${DREP_V1:-/dev/null}"
   printf 'may-write sm.dorc.FilePartId:ctime identified-in sm.dorc.Path:%s\n' "$carrier" >>"${DREP_V1:-/dev/null}"
   printf 'may-write nothing-else\n' >>"${DREP_V1:-/dev/null}"
}
```

The admitted argv has ten words: `--file F --member worker_ini --patch -- inictl set-bit K V`.
`jules__admit_patch_invocation` admits exactly that grammar and supported values, checks the
adapter's version and profile, and otherwise declines. The bind supplies the virtual catalog
instance. These helpers are ordinary authored code, not engine features.

Jules calls Inez's plain-data helper, not her role member. Jules alone emits the outer
invocation's entries and one completion record, including M and C. This avoids pooling two
completion records or treating Inez's virtual-document closure as a complete description of
host effects. By sourcing and calling the helper, Jules takes custody of this composite
invocation claim. Inez's separately published role remains useful for a directly invoked inner
tool in a supported document context.

The verdict's marked reads are Q and the relevant L when it checks `trace`, or P and the
relevant L when it checks `enabled`. Its own vouch licenses avoiding the entire no-op adapter
invocation. Probe-time measurement does not apply the proposed patch.

### § 4.3-tessa-knows-the-shared-substrate

Tessa supplies one file identity and one common vocabulary for its parts. The byte-number
arithmetic stays in her body. Metadata discriminants belong to her definition too.

```sh
# tessa-file.oracle.sh — a primary lookup and the per-sort closures
sm_dorc_FilePart__declaration() {
   : : primary-scheme "sm.dorc.FilePartId"
}
sm_dorc_FilePartId__resolve() {
   case "$1" in
   data-byte/*|mtime|ctime)
      : : identified-in "sm.dorc.File" warrants "guarantees-unique-name,guarantees-unique-referent" ;;
   *) return 2 ;;
   esac
}
sm_dorc_FilePart__may_read() {
   printf 'may-read nothing-else\n' >>"${DREP_V1:-/dev/null}"
}
sm_dorc_FilePart__may_write_entailment() {
   printf 'may-write nothing-else\n' >>"${DREP_V1:-/dev/null}"
}
```

This is a declaration about independently addressed parts under one stable file identity, not
about arbitrary ranges. The file's own description accounts for its substrate and for the
ordinary whole-file case. An admitted file-part write names its collateral metadata at the
command seat. Nothing infers that a bare write to F changes only P.

The snippets omit the general file-identity library, entry transport, and helper implementations.
Those are explicit supplied premises, not knowledge derived from the strings above. Giving the
engine those premises is not by itself evidence that a deployed oracle establishes them.

## § 5-one-book-at-three-description-levels

### § 5.1-no-description-keeps-the-wall

Without the adapter oracle, line 1 runs and forms an opaque wall. The second invocation can
only use its own live guard if a suitable verdict exists. Neither a parsed key name nor a
recognizable filename grants separation.

### § 5.2-a-whole-carrier-bound-is-honestly-coarse

Jules can first describe the running invocation as may-writing F, with no part-level precision.
This is a useful honest bound and may be the best one a serializer can provide. A write to F
collides with P, Q, and L. Line 2 guards rather than keeping its probe-time elision. The flag
does not convert a coarse bound into a narrow one.

The same applies if a nested storage describer emits only its whole backing image. Once that
coarse claim enters the effect set, an origin tag does not silently exempt siblings from it.

### § 5.3-precise-speech-identifies-the-value-question

With the rich descriptions, the running first invocation has write entries P, M, and C. The
second invocation's fact reads Q and L. The written entries separate from those read entries
through Tessa's one key space. The two logical keys' names do not supply that distinction.

The concrete operation preserves the carrier identity, length, syntax, and the second bit.
Thus an ordinary execution leaves `trace=0` true. A complete description should have a route
to retaining line 2's elision. That is the value question the human acked, not a claim that all
required inference already exists in 311.

Two distinct limitations must not be hidden:

- A general parser may read a larger routing or validation set. If it genuinely depends on P,
  the comparison can correctly guard line 2. The fixed-profile case removes that particular
  dependency; it does not establish a rule that parser reads can be ignored.
- Even granting direct canonical keys and closed routing information, the present token rule
  compares P with the shared ancestor F. That pair is UNKNOWN, so it invalidates Q's token.
  This second obstacle survives ideal lookup precision.

## § 6-the-current-mechanical-obstacle

+SURE by direct reading of `311:3.3-invalidation-three-mutator-species`:

```text
P and Q are distinct members under F.
compare(P, Q) = DISJOINT.
compare(P, F) = UNKNOWN because one is the other's container.
tokenInvalidatedBy tests every written key against every ancestor of Q.
UNKNOWN is not DISJOINT, so a write entry P invalidates Q's token.
```

This is not evidence of a wrong elision. It is an over-invalidation that loses a potentially
valid elision. The ordinary conservative case, a write entry F, must continue to invalidate Q.
Removing the parent test unconditionally would therefore not be a justified repair.

The narrow mechanical study abstracts the upstream identity and format lookups into supplied
canonical keys. It tests this token rule under unchanged 311. It is not a translation of the
JSON parser, a proof of the hypothetical tools, or a new decision about `affects` transitivity.
Its independent inhabited witness must distinguish a genuinely admitted world from a check that
passes or fails only because the fixture contradicts itself.

## § 7-who-knows-and-who-pays

| knowledge | nearest reusable speaker | another possible speaker | difficult part |
| --- | --- | --- | --- |
| which INI occurrence the command changes | Inez | Alice or Jules after implementing INI knowledge | duplicates, defaults, parser errors, and accepted invocation shapes |
| which encoded carrier bytes implement an inner edit | Jules | Inez or Alice after implementing JSON knowledge | escaping, length changes, serializer normalization, and replacement versus patching |
| whether two location names reach one file part | Tessa with the supplied instances | either tool author after investigating the substrate | aliases, identity lifetime, and a changed route to the carrier |
| which complete outer-invocation effects are claimed | Jules for this composition | Alice through a replacement oracle | metadata, guest failure, and correctly scoping delegated knowledge |
| what the deployment presently contains | any adequate probe, interpreted by its describer | Alice, who installed it | instance discovery and freshness rather than format theory |

No unique omniscient speaker is forced by this case. Each author could investigate the whole
stack, but requiring that would multiply specialist work across every consumer. Local lookups
and explicit composition let one party's expertise be reused.

The closure's residue is still real. The engineer must pursue justified confidence as far as
the contract requires. The engineer's deliberate closure and the admin's risk flag then admit
the epistemically unknowable remainder. The exercise does not replace that arrangement with a
proof requirement, and the flag does not excuse a known omitted timestamp or parser dependency.

## § 8-contrasts-that-the-sitting-must-preserve

- A whole-carrier replacement must invalidate the inner fact, even if both setting names remain.
- Writing the `trace` byte must invalidate the `trace` fact. Its alias through another syntax
  must do the same when both resolve to Q.
- An inode-replacing serializer changes identity and routing obligations. It is not the
  layout-preserving patch tested here.
- A length-changing or escaped replacement can move other locations. The narrow mapping must
  decline or describe those effects and invalidations.
- A whole-file checksum reads changed content and must not survive the patch. That does not
  imply every independently described setting changed.
- A setter's successful status does not certify that a lookup remained valid. The author must
  describe the relevant effects or the engine must withhold the optimization.
- A backing declaration is directional. Reading an outer object does not imply that writing an
  inner cache writes that object. The rejected mirror rule remains rejected.

The unresolved design question is how to preserve independently described member identity across
an admitted sibling write without granting the same preservation across a write to the carrier
itself. The effect model, token lifetime, and routing lifetime must remain distinguishable.
