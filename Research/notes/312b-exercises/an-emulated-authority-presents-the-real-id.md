# 312b-exercises/an-emulated-authority-presents-the-real-id — one bucket behind two names; one name in front of two buckets; two zones under one root

Exercise record; ahistorical; the 2026-09-18 round over `notes/311j`, carrying the dig of
`notes/311t`. Book lines are real sh. Oracle lines are strawman spellings and carry no weight;
the `#` glosses are prose in `311j` vocabulary. Grades on the conductor's claims: +SURE /
~SUSPECT / -GUESS / --WONDER. Exercises `an-emulated-authority-presents-the-real-id` and, beside
it, `one-state-reached-through-two-kinds`, `an-env-variable-selects-the-referent`,
`a-binary-reads-environment-you-cannot-see`, `distinct-names-alias-within-a-kind`, and
`a-name-resolves-from-a-vantage`. A rich stdlib is assumed present.

Spelling rules in force for the strawmen are those of
`net-sysctls-are-per-namespace` (a bind is a `local` with a trailer; a typed runtime value is one
record line to the report lane; a trailer's `$name` refers to an earlier `local`). Scheme
prefixes mark who owns the spelling (`sm.` the stdlib; two letters a tool author) and carry no
weight.

<!-- /* superseded: `sole-route` throughout is `311j`'s `:aliases-nothing-else`, in its self-knowledge reading (`311j` § 2.2, folded 2026-09-19); where this record reads it as "reachable only through it", as in `obs-the-registration-edge-is-the-one-new-sentence`, it reads the older form. */ -->

## The world

An S3 bucket is reached by name through an endpoint: a URL the client picks from a flag, an
environment variable, a profile, or a regional default. Several endpoints can serve one bucket
(a corporate reverse proxy at `s3.corp.acme.example`; the public `s3.eu-west-1.amazonaws.com`),
and one bucket name can exist behind several unrelated endpoints (the real service; a
developer's localstack on `localhost:4566`). localstack answers `sts get-caller-identity` with
the real account's id, so the operator's own token cannot tell the two apart. Nothing a client
can read says whether two endpoints reach one store: the proxy presents the corporation's
certificate, not Amazon's. A bucket also answers to access-point aliases (`…-s3alias`) wherever
a bucket name is accepted.

A DNS record lives in a zone; a zone exists in the world's DNS only as a delegation from its
parent zone, up to the root. A provider's hosted zone is the world's zone exactly when the
parent delegates to the name servers the provider assigned it; a hosted zone nobody delegates
to keeps its records and answers nobody. Owner names compare case-insensitively (+SURE all).

## The book, the catastrophes, the truths

```sh
#!/bin/sh
set -eu
flarectl dns create-or-update --zone acme.example --name assets --type CNAME \
   --content acme-site-assets.s3-website.eu-west-1.amazonaws.com             # 3   the public name for the site
cli53 rrcreate --replace acme.org 'status 300 A 203.0.113.7'                 # 4   the status page, another zone, another provider
aws s3 mb s3://acme-site-assets                                              # 5   the bucket (the profile's endpoint is the corporate proxy)
rclone sync ./public s3remote:acme-site-assets                               # 6   the site (the remote's endpoint is the public one)
aws s3 cp ./robots.txt s3://acme-site-assets/robots.txt                      # 7   one file the sync does not carry
aws --endpoint-url http://localhost:4566 s3 mb s3://acme-site-assets         # 8   the same name, seeded into localstack for the smoke test
```

Catastrophes (`311b:rul-ground-identity-in-final-outcomes`): C1, wrong SAME across the emulator,
line 5's converged fact stands in for line 8 (or 8's for 5) and one of the two buckets is never
made; C2, wrong DISJOINT across the two endpoints, line 6's write is taken to be about another
bucket than line 7's, line 7's elision survives the sync, and `robots.txt` is gone from the
site. Truths: lines 5, 6, and 7 touch one bucket through two endpoints and two authors; line 8
touches another bucket with the same name; lines 3 and 4 write records in two zones; line 3's
value names a route to the bucket and is not the bucket.

## The actors

Alice writes the book. Dana and Hugo own stdlib vocabularies, present from the start:
`dana-dns.oracle.sh` (`sm.AuthoritativeZone`, `sm.DnsRRset`) and `hugo-http.oracle.sh`
(`sm.HttpEndpoint`). Four tool authors who have never met: Carla (`carla-flarectl.oracle.sh`),
Bram (`bram-cli53.oracle.sh`), Ravi (`ravi-aws.oracle.sh`), Quinn (`quinn-rclone.oracle.sh`).

## The floor: four tool oracles, no yields

Each author binds under mSchemes of their own with no `:yields`: the floor of `311j` § 1.3.
Ravi's argparse is strict, as taught (USER_STORY stage 3): a shape he did not write for answers
2.

```sh
# dorc-lang/v0.2   ravi-aws.oracle.sh
aws__is_converged() {
   [ "$1 $2" = "s3 mb" ] && [ $# -eq 3 ] || return 2          # a leading --endpoint-url is a shape I never considered
   local b="${3#s3://}"   : is "rv.S3BucketName"
   aws s3api head-bucket --bucket "$b" 2>/dev/null   : asserts "rv.S3BucketName:$b"
}
```

Render, steady state:

```
 3  # flarectl dns create-or-update --zone acme.example …      # converged
 4  # cli53 rrcreate --replace acme.org 'status 300 A …'       # converged
 5  # aws s3 mb s3://acme-site-assets                          # converged: bucket exists
 6  # rclone sync ./public s3remote:acme-site-assets           # converged: nothing to transfer
 7  # aws s3 cp ./robots.txt s3://acme-site-assets/robots.txt  # converged: object matches
 8  aws --endpoint-url http://localhost:4566 s3 mb s3://…      # runs: a shape ravi's oracle declines
plan: 1 to run (5 skipped)
```

Every identity comparison reads UNKNOWN or KNOWN_UNSPOKEN (four floor mSchemes of four unnamed
mSorts, all scoped in the mRoute, no warrants), which is safe for both consumers. C1 cannot
fire: line 8 is a declined shape, so it runs. C2 cannot fire: on a day the sync runs, line 7
guards. On any drifted day every line below the first running one verifies. Had line 8 selected
localstack by `AWS_ENDPOINT_URL=…` instead of the flag, the floor's guard would have been
`plans/30S`'s: a prefix assignment nothing in the verdict body pins or severs withholds the
site. Either way the floor's safety is not identity's.

## The tempting glue, and C1

A more capable Ravi accepts `--endpoint-url`, and reaches for the operator's own identifier to
say which bucket is which. First draft:

```sh
rv_S3BucketName__resolve() {                     # FIRST DRAFT: "a bucket is in an account, and sts tells me which"
   local acct; acct=$(aws sts get-caller-identity --query Account --output text) || return 2
   printf 'identified-in rv.AwsAccountId:%s warrants guarantees-unique-name,guarantees-unique-referent\n' "$acct" >>"${DREP_V1:-/dev/null}"
}
rv_AwsAccountId__resolve() {                     # no store declared, so scoped in the route
   printf 'warrants guarantees-unique-referent\n' >>"${DREP_V1:-/dev/null}"
}
```

Line 5 and line 8 both measure `rv.AwsAccountId:123456789012`, because localstack presents the
real id. The account shape is scoped in the mRoute and carries `:guarantees-unique-referent`;
the two mKeys are equal; SAME. The bucket names are equal under `:guarantees-unique-referent`;
SAME. One probe answers both lines, and C1 fires. Two faults, both already named in the corpus:
the operator's mToken was trusted above the place it was read from (the exercised gotcha), and a warrant
sat on a shape with no `:identified-in` (`311m`'s parentless warrant: a true local sentence read
as a claim about every route). A third, of arrangement: the primary's arm read the world, where
`311j` § 1.6 makes a shape's declarations a function of the mKey's own bytes.

## The stdlib, and the glue lines

Hugo holds the one forced spelling a URL has: its origin. He declares nothing else about it.
Dana holds the DNS tree. The tool authors each add lookups that yield into those.

```sh
# dorc-lang/v0.2   hugo-http.oracle.sh
sm_HttpEndpoint__declaration() { : : primary-scheme "sm.HttpOrigin" }
sm_HttpOrigin__resolve() {                       # "scheme://host:port"; no store, so scoped in the route; no warrants, on purpose: an origin is a name
   case "$1" in http://*:*|https://*:*) : ;; *) return 2 ;; esac
}
sm_Url__resolve() {                              # any URL, into its origin; pure text, no reads
   local scheme="${1%%://*}" rest="${1#*://}"
   local hostport="${rest%%/*}"
   case "$hostport" in *:*) : ;; *) [ "$scheme" = https ] && hostport="$hostport:443" || hostport="$hostport:80" ;; esac
   printf 'yields sm.HttpOrigin:%s://%s\n' "$scheme" "$hostport" >>"${DREP_V1:-/dev/null}"
}
```

```sh
# ravi-aws.oracle.sh, second draft
aws__is_converged() {
   local where=self
   [ "${1-}" = --endpoint-url ] && { where="$2"; shift 2; }
   [ "$1 $2" = "s3 mb" ] && [ $# -eq 3 ] || return 2
   local ep="$where"        : is "rv.S3Endpoint"
   local b="${3#s3://}"     : is "rv.S3BucketName" identified-in "rv.S3Endpoint:$ep"
   set --; [ "$where" = self ] || set -- --endpoint-url "$where"          # the check lands where the line would
   aws "$@" s3api head-bucket --bucket "$b" 2>/dev/null   : asserts "rv.S3BucketName:$b"
}
rv_S3Endpoint__resolve() {                       # a spelling of hugo's sort. "self": wherever a bare `aws s3` lands from here, read as the CLI reads it; else the URL the line named
   local url="$1"
   [ "$1" = self ] && url="${AWS_ENDPOINT_URL_S3:-${AWS_ENDPOINT_URL:-$(aws configure get endpoint_url)}}"
   [ -n "$url" ] || url="https://s3.$(aws configure get region).amazonaws.com"
   printf 'yields sm.Url:%s\n' "$url" >>"${DREP_V1:-/dev/null}"
}
rv_S3Bucket__declaration() { : : primary-scheme "rv.S3BucketName" }
rv_S3BucketName__resolve() {                     # the store is an endpoint, and the bind says which; what I know is true only among one endpoint's buckets
   case "$1" in
   *-s3alias) : ;;                               # an access-point alias is a second name for some bucket: no warrants
   *)         printf 'warrants guarantees-unique-name,guarantees-unique-referent\n' >>"${DREP_V1:-/dev/null}" ;;
   esac
}
```

```sh
# added to quinn-rclone.oracle.sh
qn_RcloneRemote__resolve() {                     # a remote's name, into the URL its config points at
   local ep; ep=$(rclone config show "$1" | sed -n 's/^endpoint *= *//p') || return 2
   [ -n "$ep" ] || return 2                      # a provider default I have not surveyed: cannot say
   case "$ep" in *://*) : ;; *) ep="https://$ep" ;; esac
   printf 'yields sm.Url:%s\n' "$ep" >>"${DREP_V1:-/dev/null}"
}                                                # her qn.S3BucketName is identified-in "qn.RcloneRemote:$remote", warranted as ravi's
```

```sh
# dorc-lang/v0.2   dana-dns.oracle.sh
sm_AuthoritativeZone__declaration() { : : primary-scheme "sm.ZoneApex" }
sm_ZoneApex__resolve() {                         # identity on the apex; whoever yielded it supplied its store
   case "$1" in
   .)   printf 'warrants rootness\n' >>"${DREP_V1:-/dev/null}" ;;      # the one global registry; a split horizon resolving SAME is this line's fault
   *.)  printf 'warrants guarantees-unique-name,guarantees-unique-referent,sole-route\n' >>"${DREP_V1:-/dev/null}" ;;
   *)   return 2 ;;
   esac
}
sm_ZoneCut__resolve() {                          # any name, into the apex of the zone that holds it; one read per label climbed
   local n="$1"
   while [ "$n" != . ] && [ -z "$(dig +short NS "$n")" ]; do n="${n#*.}"; [ -n "$n" ] || n=.; done
   printf 'yields sm.ZoneApex:%s\n' "$n" >>"${DREP_V1:-/dev/null}"
   local up="${n#*.}"; [ -n "$up" ] || up=.
   [ "$n" = . ] || printf 'identified-in sm.ZoneCut:%s\n' "$up" >>"${DREP_V1:-/dev/null}"
}
sm_DnsRRset__declaration() { : : primary-scheme "sm.RRsetKey" }
sm_RRsetKey__resolve() {                         # "owner. TYPE", lowercased by whoever yields it; stored in the zone that holds the owner
   printf 'identified-in sm.ZoneCut:%s warrants guarantees-unique-name,guarantees-unique-referent\n' "${1%% *}" >>"${DREP_V1:-/dev/null}"
}
```

```sh
# added to bram-cli53.oracle.sh                  (carla's is the same shape against cloudflare's api)
bm_R53Record__resolve() {                        # "ZONE NAME TYPE", into the world's DNS iff the parent delegates to the servers route53 gave this zone
   local zone="${1%% *}" rest="${1#* }"
   local mine world
   mine=$(aws route53 get-hosted-zone --id "$(cli53 list | awk -v z="$zone." '$2==z{print $1}')" \
            --query 'DelegationSet.NameServers[]' --output text | tr '\t' '\n' | sort)   || return 2
   world=$(dig +short NS "$zone." | sed 's/\.$//' | sort)                                 || return 2
   [ -n "$mine" ] && [ "$mine" = "$world" ] || return 2       # a hosted zone nobody delegates to: its records are route53's alone; cannot say
   local owner; owner=$(printf '%s.%s.' "${rest%% *}" "$zone" | tr A-Z a-z)
   printf 'yields sm.RRsetKey:%s %s\n' "$owner" "${rest##* }" >>"${DREP_V1:-/dev/null}"
}
```

The walks:

- Line 8 against line 5 (the C1 guard). Line 5's bucket: Ravi's bind, stored in
  `rv.S3Endpoint:self`, which resolves where the site runs (once per mEntryChain, an ambient
  instance like the sysctl record's `sm.NetnsSelf:self`) through Hugo's `sm.Url` to
  `sm.HttpOrigin:https://s3.corp.acme.example:443`, scoped in the mRoute. Line 8's bucket: the
  same bind, stored in `rv.S3Endpoint:http://localhost:4566`, to
  `sm.HttpOrigin:http://localhost:4566`. From the top: the mRoute is one inherited instance; at
  the origin level the two mKeys differ and the shape carries no warrant, so they are not SAME
  and not DISJOINT. UNKNOWN. Line 8 gets its own probe and line 5 never stands in. The forged
  account id never enters: Ravi did not need the account in the chain, and had he kept it, it
  would sit BELOW the origin and compare only inside one.
- Line 7 against line 5. Both bare; both stored in `rv.S3Endpoint:self`, one mPlaceholder; SAME at
  the origin by instance, no warrant consulted; bucket names equal under
  `:guarantees-unique-referent`; SAME. Line 7's object is in line 5's bucket, attributed to
  Ravi's warrant.
- Line 7 against line 6 (the C2 guard). Quinn's footprint is a `qn.S3BucketName` stored in
  `https://s3.eu-west-1.amazonaws.com:443`; Ravi's fact is stored in
  `https://s3.corp.acme.example:443`. The deepest SAME level is the mRoute; the tops are two
  `sm.HttpOrigin` mKeys, one mScheme, no `:guarantees-unique-name`. UNKNOWN. Line 7 guards,
  re-checks after the sync, and copies. Were the two origins equal the pair would still be two
  strangers' mSorts over one bucket: KNOWN_UNSPOKEN at the leaf, no mPlacements declared on
  either side, collide.
- Line 4 against line 3. Carla's record yields `sm.RRsetKey:"assets.acme.example. CNAME"`,
  stored in `acme.example.`, in `example.`, in `.`; Bram's yields
  `sm.RRsetKey:"status.acme.org. A"`, in `acme.org.`, in `org.`, in `.`. The root is one
  mWorld by :rootness. The tops `example.` and `org.` are mKeys of one mScheme carrying
  `:guarantees-unique-name`, with differing values, and every store below down to the leaves'
  mParents is `:sole-route`. DISJOINT. On a day the CNAME has drifted, line 4 survives line 3:
  two provider authors who never met, separated by Dana's tree and one `dig` apiece.
  <!-- /* superseded: this survival answers only the address question; under `311j` § 2.5 as folded 2026-09-19 the write-path question is also asked, Dana's mSorts declare no mPlacements, an undeclared mPlacement collides, and line 4 verifies in both renders (`311t` § 8). */ -->
- Two strangers in one zone (no book line; a colleague's `dnscontrol` beside Carla's
  `flarectl`). Both yield into `sm.RRsetKey` under `acme.example.`. The same owner and type:
  one mKey, SAME, one mCell, each tool's write a write to the other's fact. Different owners
  (certbot's `_acme-challenge` TXT churn beside the CNAME): siblings under one zone,
  `:guarantees-unique-name`, DISJOINT. Collision and sparing both, from the glue alone.
- Line 5 against line 3. A record's chain ends at a mRoot; a bucket's ends at the mRoute.
  `311j` § 3.2's first bullet: UNKNOWN. Line 5 guards on a day line 3 runs.

Render, a day the CNAME and the site have both drifted, with the flag:

```
 3  flarectl dns create-or-update --zone acme.example …        # runs: diverged (the CNAME points elsewhere)
 4  # cli53 rrcreate --replace acme.org 'status 300 A …'       # converged; survives line 3 (dana: two zones, each delegated)
 5  ( … ) || aws s3 mb s3://acme-site-assets                   # verify: converged, but past line 3 (a record and a bucket share no measured ancestor)
 6  rclone sync ./public s3remote:acme-site-assets             # runs: diverged
 7  ( … ) || aws s3 cp ./robots.txt s3://…/robots.txt          # verify: converged, but past line 6 (not known to be another bucket)
 8  ( … ) || aws --endpoint-url http://localhost:4566 s3 mb …  # verify: converged, but past line 6 (not known to be another bucket)
plan: 2 to run, 3 to verify (1 skipped)
```

## The other tempting glue, and C2

The DNS tree separates so well that Hugo is tempted to hang origins under it:
`sm.HttpOrigin` `:identified-in` `sm.ZoneCut:$host`. Then line 7's origin sits in
`acme.example.` and line 6's in `amazonaws.com.`; the tops `example.` and `com.` differ under
Dana's `:guarantees-unique-name`; the zones are `:sole-route`; DISJOINT; line 7 survives the
sync; C2 fires. The sentence Hugo would have made false is Dana's: `:sole-route` on a zone says
what is identified in it is reachable only through it, and a store behind an origin is
reachable through as many names as anyone cares to point at it (this book has two). A name is a
route to what answers, and a zone is a store of names. Records live in zones; nothing else
does.

## A hosted zone nobody delegates to

Acme moved `acme.org` to another provider last year and the Route53 hosted zone lingers. Line 4
still converges against Route53's API and still writes there. Bram's lookup declines (the
delegation names other servers), so his record has no place in Dana's tree: unknown from that
level, UNKNOWN against everything, never standing for the world's `status.acme.org`, and on the
drifted day it verifies instead of surviving. The decline's breadcrumb is the most useful line
in the plan that morning: the book has been maintaining a record nobody can resolve.

## The four first guesses, and what each is made of

A later sitting proposed four sentences a provider's describer and the stdlib might say, as the
way to buy back the lines the render above leaves as guards. Each was shaved to what it says
about objects and relations, checked against `311j` for redundancy, and paired with the same
pattern elsewhere. All four decompose into existing relations. They stay on the shelf as
first-guess constructs: where the abstract spellings chafe, recomposing them is how ergonomics
is bought back, and taking the existing construct can quietly cost usability that is only
discovered later (**[HUMAN]**, 2026-09-18).

- "My service is entered through these hosts." A lookup from typed route-names to a store's
  mKey that declines what it does not know: a secondary mScheme's `:yields` with declining arms
  (§ 2.1). It needs no closure (nothing consumes "these and no others") and is no part of
  `:sole-route` (many routes into a store all enter it). Pair: `sm.Path` into `sm.Inode`; a
  namespace label into its nsfs inode. The difference the pair shows: `stat` asks the store's
  own arbiter and is handed the mKey; a host table is its author reciting. Both are `resolve()`
  bodies, and the value plane already grades a table below a world read. A host that fronts
  several unrelated stores by path is several mSchemes over one class of strings, each owned by
  whoever knows that store, each declining the rest; the endpoint is not a unit of composition.
- "I am a plain store, a view, or a driver." Three independent declarations, not a category:
  `:sole-route` (§ 2.2); a closed mPlacement set, as against one pointing outward or undeclared,
  which is ⊤ (§ 2.4); a finished at-most footprint, as against none, which is a wall (§ 2.5).
  View and driver are the silent defaults; plain store is what is earned. Pairs: ext4 against
  an overlay's lower layer; `cp` against `apt-get install` and its postinst; a FUSE mount
  against a CDN.
- "My store lives at my registrable domain." `:identified-in`, with a node of somebody else's
  tree as the mParent, so that a store acquires a common ancestor with stores its describer
  never heard of. What it asserts of the world: this store belongs to that registration and to
  no other. The one piece of new content among the four. Pair: a filesystem identified in a
  boot by device number. The difference: `st_dev` is handed back by the arbiter when the file
  is touched, a measured edge; "the store at `amazonaws.com.`" is recited. A far end proving
  which registration it answers for (the names in its certificate) is the candidate
  measurement; open.
- The registry tree. A delegation tree: each node's owner alone assigns child labels, so a
  label path from the root is unique with no two owners coordinating. An ordinary mSort
  identified in itself level by level, :rootness at the top, its warrants true of
  registrations. Pairs: nested pid and user namespaces; the OID arcs; ISBN prefixes; address
  delegation; Dorc's own reverse-DNS names for mSorts, which already lean on this tree for the
  uniqueness of vocabulary. Dana's file above is this.

## A store-sort under the tree: the revised glue

<!-- /* superseded: this glue hangs a store under a registration as `:identified-in`; `311t` § 11 judged that a conflation of identification with who answers for a namespace, and § 10 withdrew the registration edge as `:identified-in`. The strawman stands as the record of the attempt. */ -->

Built from the shaved parts, with no relation this record had not already used. Petra knows AWS
and owns nobody's resources: she publishes the store keys once, hangs them under one
registration, and publishes the lookup that enters them.

```sh
# dorc-lang/v0.2   petra-aws-stores.oracle.sh
pt_AwsStore__declaration() { : : primary-scheme "pt.AwsStoreKey" }
pt_AwsStoreKey__resolve() {                      # "s3", "route53", "ec2/eu-west-1": the services, at the grain their identifiers are scoped
   case "$1" in
   s3|route53|ec2/*)  printf 'identified-in sm.ZoneCut:amazonaws.com. warrants guarantees-unique-name,guarantees-unique-referent,sole-route\n' >>"${DREP_V1:-/dev/null}" ;;
   *)                 return 2 ;;
   esac
}
pt_AwsEndpointUrl__resolve() {                   # a URL someone typed or configured, into the store it enters; what I do not recognise, I decline
   local host="${1#*://}"; host="${host%%[:/]*}"
   case "$host" in
   s3.amazonaws.com|s3.*.amazonaws.com|*.s3.*.amazonaws.com)  printf 'yields pt.AwsStoreKey:s3\n'      >>"${DREP_V1:-/dev/null}" ;;
   route53.amazonaws.com)                                      printf 'yields pt.AwsStoreKey:route53\n' >>"${DREP_V1:-/dev/null}" ;;
   ec2.*.amazonaws.com)  local r="${host#ec2.}"
                         printf 'yields pt.AwsStoreKey:ec2/%s\n' "${r%.amazonaws.com}"                   >>"${DREP_V1:-/dev/null}" ;;
   *)                    return 2 ;;             # localhost:4566; s3.corp.acme.example; anything else
   esac
}
```

Ravi changes one line: `rv_S3Endpoint__resolve` yields `pt.AwsEndpointUrl:$url` where it yielded
`sm.Url:$url`. Quinn's remote, configured `provider = AWS`, does the same (another provider's
remote is a second mScheme of hers, into Hugo's mSort as before). Nobody edits Petra's file, and
Petra has read nobody's.

The walks that change:

- Line 6 against line 3. Quinn's bucket: her remote's URL, through Petra's lookup to
  `pt.AwsStoreKey:s3`, stored by Petra's arm in `sm.ZoneCut:amazonaws.com.`, which Dana's lookup
  resolves to the apex `amazonaws.com.`, in `com.`, in `.`. Carla's record sits in
  `acme.example.`, in `example.`, in `.`. The tops `com.` and `example.` differ under Dana's
  `:guarantees-unique-name`; every store below is `:sole-route` by Dana's arm and Petra's.
  DISJOINT. On a morning only the CNAME has drifted, the sync's converged fact survives line 3.
- Lines 5 and 7 against line 3. Alice's profile points at `s3.corp.acme.example`; Petra's lookup
  declines it; the bucket's store is unknown from that level; UNKNOWN; both verify. Line 8's
  `localhost:4566` likewise. The emulator and the proxy are both names Petra never heard of.
- Line 7 against line 6 (the C2 guard). One side unknown; UNKNOWN; line 7 guards. Were Alice's
  profile the public endpoint, both buckets would sit under `pt.AwsStoreKey:s3`, SAME at the
  store by one yielded mKey and Petra's warrant, and the two bucket mKeys would be tops of two
  strangers' mSchemes: UNKNOWN still, and rightly, since they are one bucket.
- Strangers on one provider (no book line). Had Bram identified his hosted zones in
  `pt.AwsStoreKey:route53`, his records and Quinn's buckets would meet at Petra's registration
  with tops `route53` and `s3`: one mScheme, `:guarantees-unique-name`, DISJOINT. What strangers
  share is the thin store-sort, never a resource vocabulary.

Render, a morning only the CNAME has drifted, with the flag:

```
 3  flarectl dns create-or-update --zone acme.example …        # runs: diverged (the CNAME points elsewhere)
 4  # cli53 rrcreate --replace acme.org 'status 300 A …'       # converged; survives line 3 (dana: two zones, each delegated)
 5  ( … ) || aws s3 mb s3://acme-site-assets                   # verify: converged, but past line 3 (petra's lookup does not know s3.corp.acme.example)
 6  # rclone sync ./public s3remote:acme-site-assets           # converged; survives line 3 (petra: the s3 store, at amazonaws.com.; dana: another delegation)
 7  ( … ) || aws s3 cp ./robots.txt s3://…/robots.txt          # verify: converged, but past line 3 (petra's lookup does not know s3.corp.acme.example)
 8  ( … ) || aws --endpoint-url http://localhost:4566 s3 mb …  # verify: converged, but past line 3 (petra's lookup does not know localhost:4566)
plan: 1 to run, 3 to verify (2 skipped)
```

## Cost, briefly

Per distinct name, a few `dig`s up the labels, memoised within an unwalled span; per hosted
zone, one provider call and one `dig` for the delegation check; per mEntryChain, one resolution
of `self`; per rclone remote, one config read. Each is smaller than the API call its line
already makes. The zone climb and the delegation check read through the mVantage's resolver; a
careful Dana asks each parent's own servers without recursion, which costs the same reads and
stops depending on whose resolver answers.

## Observations

- `obs-operator-tokens-sit-below-the-place-they-were-read` (+SURE) — an identifier the far end
  hands out (an account id, a project id, a caller identity) compares only inside the endpoint
  it was read through, and that endpoint is a name with no warrants. An emulator presenting
  the real id then cannot mint SAME, because SAME is "and" over levels and the level above
  differs; and nothing can mint DISJOINT there either. The exercised gotcha is carried by arrangement,
  with no certificate read and no list of endpoints. It is `312b`'s
  `lead-identify-in-the-narrowest-primary-store`, at the web's grain.
- `obs-an-origin-is-a-name-and-the-book-proves-it` (+SURE) — the one warrant that would spare
  line 8 from line 6 is `:guarantees-unique-name` on origins (one store, one origin), and lines
  5 and 6 of the same book refute it. Whoever is tempted to write it can be shown their own
  profile.
- `obs-the-ambient-endpoint-is-a-singleton-spelling` (+SURE) — `rv.S3Endpoint:self` does for an
  endpoint what `sm.NetnsSelf:self` does for a namespace: one mPlaceholder per mEntryChain, SAME
  across bare lines by instance, and the home for `an-env-variable-selects-the-referent` and
  `a-binary-reads-environment-you-cannot-see` (Ravi's body reads what the binary reads, so the
  dependence is a read of his, marked or ⊤). An `export AWS_ENDPOINT_URL=…` between two bare
  lines is `30S`'s fence, and in this model a routing write that perishes `self`; § 3.3's
  routing species names only `PATH` today.
- `obs-equal-literal-origins-rest-on-the-route-vouch` (~SUSPECT it matters) — two lines that
  both say `--endpoint-url http://localhost:4566` are SAME at the origin only as "one mKey in
  one transit-free unwalled span" (`311j` § 1.10), which for an origin is the reading "whatever
  answers there is one thing; a round-robin behind it is noise". Under `311p` thread 6's
  narrowing they would be two mPlaceholders, UNKNOWN, and the ambient `self` would be the only
  same-endpoint sameness. Both are safe; they differ in what a flag-spelled book gets for free.
- `obs-dns-is-a-store-of-records-and-of-nothing-else` (+SURE) — the tree is a genuine container
  for zones and RRsets: forced spellings, one registry, delegation as the only way in. It gave
  this book its one survival and would give two strangers in one zone their collision. Hanging
  anything else under it is C2.
- `obs-the-delegation-check-is-the-glue` (+SURE of the walk; ~SUSPECT of the read's quality) — a
  provider's record belongs to the world's DNS exactly when the parent delegates to that
  provider's servers, which each provider author can test alone with their own API and one
  `dig`. It nails mutually-unknowing DNS tools together with no shared vocabulary beyond Dana's,
  and it declines on the lingering zone, the commonest real mistake in the domain.
- `obs-rootness-is-danas-line` (~SUSPECT of frequency) — :rootness on `.` claims there is one
  DNS. A split horizon makes two zones with one apex under one parent, and a pivot book that
  compares a record read inside the office with one read outside would find them SAME.
  Within one mVantage only one provider's delegation check passes, so the knife needs two
  mVantages in one comparison; asking the parents' own servers without recursion removes the
  dependence on the mVantage's resolver and leaves interception of port 53, a horizon. The
  stdlib owns it, as it owns cloned boots.
- `obs-names-inside-values-are-not-keys` (+SURE) — line 3's CNAME content spells the bucket's
  website endpoint. It is a value; no author binds it; the record and the bucket are unrelated
  in the model, which is right in both directions.
- `obs-a-drifted-record-guards-every-bucket-line` (+SURE; the cost) — a chain that ends at a
  mRoot and one that ends at the mRoute read UNKNOWN, so the book's one drifted DNS line turns
  every remote line below it into a guard. Nothing physical separates "what answers at an
  origin" from "what a zone holds"; buying those lines back is an operator's word or a
  consented policy (`311t` § 1 `hold-cross-root-cells`), and this exercise does not.
- `obs-the-vanity-stays-unknown-and-that-is-the-price` (+SURE) — lines 5 and 6 reach one bucket
  and the model never learns it. The one party who could say so is whoever runs the proxy, as a
  sentence about two origins reaching one store, true only of a proxy that keeps no state of
  its own (`311j` § 2.6: a view of a thing is not it). Until then: two probes, a guard, no harm.
- `obs-served-by-this-box-is-the-companion-case` — an origin whose address is this box's and
  whose port a local listener holds chains under the boot by three lookups, none of them
  Hugo's above; worked in `311t` § 5 (the `robots.txt` sitting), not here.
- `obs-the-four-guesses-are-existing-relations` (+SURE) — a lookup with declining arms, three
  declarations a store already has, one `:identified-in` edge, and an ordinary mSort that is a
  delegation tree. Nothing in the revised glue asked `311j` for a relation it lacks.
- `obs-the-registration-edge-is-the-one-new-sentence` (+SURE of the strain; --WONDER of the
  repair) — § 3.2's walk asks `:sole-route` of `amazonaws.com.` for what Petra hung there, and
  Dana's blanket arm supplies the letter. Read as reachability it is false: the corporate proxy
  reaches the s3 store through no name in that zone. Petra's arrangement is safe where Hugo's
  was not because she hangs only stores she recognises and declines the rest, so no one store
  is ever hung twice by her; what the separation leans on is that a store has one registration
  as its mParent, by its describer's word, and `311j` has no warrant that says that.
- `obs-one-operator-several-registrations` (~SUSPECT of frequency) — large operators serve one
  tenancy under several registrations (a storage host under one, a management host under
  another). A recited edge absorbs it: the describer picks one mParent and lists the other
  hosts as routes in her lookup. A measured edge would measure two registrations for one store
  and need the recital anyway.
- `obs-the-corporate-route-needs-an-arm-petra-cannot-write` (+SURE) — the organisation's one
  true sentence ("this URL is a stateless route into the s3 store") is an arm of Petra's lookup
  that only the organisation can author. A bind names one mScheme, and a lookup that declines
  falls through to nobody. This is the open composition corner, separate authors composing arms
  into one lookup, reduced here to one line of one file; punted (**[HUMAN]**, 2026-09-18). The
  thin store-sort is the same corner seen from the other side: better than a shared resource
  vocabulary, still less than ideal for collaboration.
- `obs-one-scheme-one-sort-chafes-at-multi-provider-tools` (~SUSPECT) — Quinn's remote is AWS or
  not according to its configuration, and one mScheme cannot yield into Petra's mSort on one arm
  and Hugo's on another; she needs two mSchemes and a bind that chooses between them by a read.
  Spellable, verbose, and a second place where recomposed first-guess constructs might earn
  their keep.
