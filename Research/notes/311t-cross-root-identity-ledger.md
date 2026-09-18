# 311t — Cross-root identity: the web, DNS, and sparing across roots (ledger)

> AI-authored (Fable, the `r31-prep-design-duck` sittings, from 2026-09-17). Notes-tier LIVING
> ledger for the dig `311q` § 18 opened: whether DNS can play the filesystem's role, what a
> physical identity foundation for web resources can and cannot be, and how `compare()` should
> answer when two chains reach two roots. Nothing here is ruled unless it cites a ruling by
> `docID:slug`; grades are +SURE / ~SUSPECT / -GUESS / --WONDER; **[HUMAN]** marks the human's
> framing, paraphrased from chat, never a ruling. Authority: root docs, `spike/CLAUDE.md`, the
> welds, `notes/311j`, and `311q` § 1 (the phase's conduct) outrank this. Successors append
> sections at the tail and edit nothing in the middle. § 3 is the working set of killers; read it
> first.

## § 1 — 2026-09-17: the question, and the framing that holds

The question (**[HUMAN]**): sound sparing between two mutually-unknowing tools that smoosh around
in their respective remote APIs, with no sparing when two mutually-unknowing tools splash in one
API, reached by physical tests that need no collaboration and conflict in physical reality; the
`hcloud`-creates-an-instance against `route53`-record-converged book, correct in both failure
directions, referentially agnostic.

What holds after four sittings, in order of how it was reached:

- `hold-dns-as-container-is-a-seat-error` (+SURE) — hanging remote things under the DNS name they
  were reached by (`sm.Host` in a zone; a bucket in `s3.amazonaws.com`) is a routing catalog
  declared as a store: the same error as path-as-primary, refused by `a-path-is-not-a-referent`
  and `fnd-two-epistemic-seats-mis-sited`. Its bites are `kill-route-declared-as-store` below. It
  says nothing about DNS being different from the filesystem.
- `hold-refag-correction` (**[HUMAN]**, acked by the conductor) — the four "edges" a container was
  said to give (mints identity, holds state, sole-route, coherent local routing) reduce inside the
  model to the two lookup warrants, the placement set, and the volatile bit, each claimable about
  any sort at any level. "Hands out" and "lives in" were world-talk. Nothing about filesystems is
  by derivation; every filesystem fact is a composed human claim.
- `hold-physical-means-enforced-plus-labeled` (+SURE) — the inode's strength is two properties: a
  running system whose invariants no speaker can override (ext4: two inodes, two disjoint
  write-sets), and that system labelling its own layering (`mountinfo`: ext4, overlay, fuse, loop
  and its backing file). The web has the first at endpoints (keypairs) and content (digests) and
  the second nowhere: no arbiter labels a terminator as store-or-view, or two terminators as
  one-backend-or-two.
- `hold-physical-gives-half-the-goal` (+SURE) — physical facts can COLLIDE two strangers who hit
  one terminator and one operator-minted token (modulo `kill-cloned-key-across-a-fleet` and
  `kill-emulator-presents-real-id`) and can never SPARE across stores, because "different store"
  is not a physical observable on the web. The second half of the goal needs an operator's word
  about how its terminators group (tabled, § 4), a consented policy that horizons standing bridges
  where they are rare, or guards.
- `hold-two-roots-stay-unknown-in-the-model` (~SUSPECT, conductor) — `311j` § 3.2's first bullet
  stands as the model's answer; any spare-across-roots is a policy beside the model, keyed on the
  three cells of `hold-cross-root-cells`, never a rule of `compare()`.
- `hold-cross-root-cells` (~SUSPECT on boundaries; +SURE the cells differ in frequency) —
  provider-versus-provider (undeclared standing bridges rare; spare-by-default defensible);
  web-versus-local (the box being configured is usually the origin; a CDN in front is standard;
  CI reactions land on it; collide is the only honest default); terminator-versus-terminator within
  one operator (spare is wrong on every provider with regional endpoints; grouping is the tabled
  social table). If a cross-root policy ships it is the second consumer of
  `--risk-faultless-skips`, restricted to the first cell.

Retracted along the way, so nothing rests on them: "the stdlib can provide a filesystem-tier
foundation for web resources" (true only with operator-declared grouping, tabled, or with guards);
"sole-route by decline discipline" (a valid mechanism, but it is the social canonical table);
"the registry tree is physical" (only where DNSSEC-signed); "a stdlib root-of-roots with
unique-name over shape tags" (`kill-stdlib-shape-tag-unique-name`).

Standing note (**[HUMAN]**, not to be dug yet): chains are potentially cyclic directed graphs
(ext4 under a VM under AWS under `.`; `.` served from files on a boot). The conductor's one-line
read: two roots are two descriptions, not two places, which supports UNKNOWN at two roots.

## § 2 — the strawmen this ledger walks

`book-provider-versus-provider`:

```sh
hcloud server create --name worker-3 --image debian-12 --type cx22            # 3  drifted, runs
aws route53 change-resource-record-sets --hosted-zone-id Z0DEADBEEF \
   --change-batch file://api-cname.json                                        # 4  converged
```

`book-strained` (runs on `web1`; standing facts nobody in the book typed: `web1` and `web2` share
the wildcard key; Cloudflare has proxied `blog.example.com` to `web1` since last year;
`api.example.com` is unproxied and points at `web2`; a GitHub Action deploys to `web1` over ssh
on every push; six authors who never met: Tessa files, Carla `cf-cache`, Hugo `curl`, Ravi `aws`,
Gitta `git`, Sven `systemctl`):

```sh
install -m 644 ./robots.txt /var/www/robots.txt                                       # 3  drifted: runs
cf-cache purge --zone example.com --url https://blog.example.com/robots.txt           # 4  converged: CDN serves current bytes
curl -fsS https://api.example.com/healthz                                             # 5  converged
aws s3api delete-bucket --bucket example-assets && aws s3 mb s3://example-assets      # 6  drifted: runs (global endpoint)
aws --endpoint-url https://s3.eu-west-1.amazonaws.com s3api put-bucket-policy \
   --bucket example-assets --policy file://p.json                                     # 7  converged: policy present (regional endpoint)
git push origin main                                                                  # 8  drifted: runs
systemctl is-active blog >/dev/null || systemctl restart blog                         # 9  converged: active
```

## § 3 — the killers (working set)

Each: the pair, the reading it kills, the wrong answer, the fact that would have caught it and who
could read it, status. "Physical" and "social" as in § 1.

- `kill-route-declared-as-store` — the four walks over the DNS-as-container reading, with a book
  that creates a box at Hetzner, points two names at it, pushes an image, converges over ssh by
  one name, health-checks by the other, and syncs backups. (a) two names, one box: `origin` and
  `blog` siblings under `example.com` with unique-name ⇒ DISJOINT ⇒ the health check survives a
  restart; wrong DISJOINT; the CDN/vanity/VPC-endpoint shape. (b) one name, two boxes: a rebuild
  gives a new address; the `until dig @1.1.1.1` line and the later `ssh` resolve through
  different catalogs (a public resolver against libc plus nscd, `a-cached-lookup-answers-for-the-past`); one placeholder ⇒
  SAME ⇒ the ssh lands on the reissued address; wrong SAME. (c) the catalog has other authors:
  TTL expiry, the registrar, certbot's TXT records, a colleague's terraform; no book line touches
  the record, so nothing perishes the resolution; an unperished resolution. (d) the container
  did not notice: `api.hetzner.cloud` and `example.com` are two routes to one box, both under the
  DNS root, unique names ⇒ DISJOINT ⇒ a service fact measured on the dead box licenses skipping
  the restart on the new one. Kills: any reading where a name is a store. Catches: names as
  secondary schemes yielding into provider ids; resolutions as volatile reads. Status: closed as
  a seat error; the walks stay as the reason.
- `kill-stdlib-shape-tag-unique-name` — a stdlib `sm.WorldRoot` whose shapes are our words
  (`hcloud-project:*`, `aws-account:*`) with unique-name across shapes. Two strangers describing
  one provider under two tags read DISJOINT at the one seat that can never detect it. Kills: any
  root-of-roots keyed on minted vocabulary (**[HUMAN]** nack, 2026-09-17: "hostile to
  collaboration; that is precisely why I keep naming DNS"). Catches: nothing; the shape is dead.
- `kill-cdn-over-local-origin` — `book-strained` 4 against 3. Carla's fact is keyed by
  Cloudflare's keypair and the URL; Tessa's footprint is an inode under `web1`'s boot; two roots.
  Under spare-across-roots the CDN keeps serving the old `robots.txt` until its cache expires;
  wrong DISJOINT. Physical facts on `web1` see nothing: Cloudflare's origin-pull is configured at
  Cloudflare. Readable by: the Cloudflare oracle (the proxied record and its origin), the local
  address list (is the origin me), the nginx oracle (`nginx -T`: which file serves that path).
  Status: OPEN; the § 5 sitting works it.
- `kill-cloned-key-across-a-fleet` — `book-strained` 5 against any local fact. One wildcard
  private key on every box makes `web2`'s API and `web1` one terminator by keypair equality;
  the "a terminator whose key is in the local filesystem is local" repair chains `web2` under
  `web1`'s boot; wrong SAME. Kills: SAME-by-keypair as a physical warrant. Catches: the
  operator's word that a key is on one listener (social). Status: the transparent-fronting
  subdivision is tabled (§ 4); the admin-side mechanism is tabled (§ 4).
- `kill-one-store-many-terminators` — `book-strained` 7 against 6. The global and regional S3
  endpoints present different keys; two roots; the bucket policy is never reapplied to the
  recreated bucket; wrong DISJOINT inside one provider. The only fact linking the two
  terminators is AWS's own answer (`sts`, `get-bucket-location`), which
  `kill-emulator-presents-real-id` shows is forgeable. Kills: keypair-as-root for DISJOINT.
  Catches: the operator's grouping (social, tabled). Status: OPEN under the tabled item.
- `kill-emulator-presents-real-id` — `an-emulated-authority-presents-the-real-id`. localstack answers `sts get-caller-identity` with
  the real account id; a corporate proxy serves the real account under a vanity domain. Kills:
  operator tokens as physical collision arbiters on their own. Catches: the presented chain
  (localstack fails it; a corp-CA proxy fails root pinning; a vanity front with a public cert
  passes and is horizon). Status: standing knife; the cert read is one of the physical facts that
  survive.
- `kill-vanity-front` — `s3.corp.example` CNAMEs to S3 with its own cert. Two authors, one using
  each name, both reaching one bucket. Under any "different measured apex ⇒ different store"
  reading: wrong DISJOINT. Catches: the provider's canonical endpoint list (social, tabled) or
  UNKNOWN. Status: folded into `kill-one-store-many-terminators`.
- `kill-remote-reaction-onto-local` — `book-strained` 9 against 8. The Action restarts `blog`
  asynchronously; if the deploy dies halfway, 9's plan-time `active` survives 8 and the service
  stays down. Readable by: a GitHub oracle that finds any push-triggered workflow in
  `.github/workflows/` and declares the push's footprint ⊤ (a total wall; arbitrary code runs
  somewhere). Status: horizon at parity with `a-plain-cp-can-trigger-a-watcher` absent that oracle; with it, a wall.
- `kill-reissued-address-to-a-stranger` — cloud addresses are reissued to other tenants within
  minutes of a delete; a name still pointing at the old address reaches a stranger; ssh's host-key
  check dies (`a-host-key-identifies-an-endpoint-not-a-machine`, barely). Kills: any unique-referent on address-shaped keys. Catches:
  instance ids carry unique-referent, addresses never; the standup `witness()` re-reads the id.
  Status: posture, no open question.
- `kill-cross-plane-precondition` — identity says DISJOINT (correctly) while the downstream line's
  argv was captured from the upstream line's output (`IP=$(hcloud server ip …)`); survival is
  wrong unless `275`'s patrol walls the capture (`an-single-shot-capture`, still O). Kills: any
  build of cross-root sparing without the value-plane patrol beside it. Status: an engine
  precondition to pin red beside any such build.

Physical facts that survive the killers, for the record: SAME at a terminator by keypair, modulo
cloned keys; SAME and DISJOINT for content by digest (both warrants, no collaboration, forever;
content only, never mutable state); the DNSSEC chain where a zone is signed (zone-owner
statements about routes, nothing about stores); the apply-standup `witness()` re-measuring the
keypair, which catches a changed route; the kernel's labels on the local side of any bridge (fuse,
nfs, loop), which catch `s3fs` and NFS and never a CDN.

## § 4 — tabled by the human, 2026-09-17

Recorded as what was tabled and the one sentence that locates it; nothing here is owed.

- `tab-declared-canonical` — a community-blessed natural key (the ARN grammar; the documented
  endpoint) used for conflict detection between strangers at the aid plane, never for identity;
  possibly a performance optimisation later. The physical route was preferred for now because
  "every potential speaker collides here for social reasons, provably, forever" is rare in the
  world and "for physical reasons" is common.
- `tab-transparent-fronting-subdivision` (**[HUMAN]** nit, not a nack) — "one terminator fronts
  many stores" is, in a sane world, intentionally transparent; whether the round-robin or its
  participant is the thing under identity test, or the whole thing is disclaimed as noise ("I just
  want to hit EC2"), is admin intent the model can never decide universally and must carry both
  ways. Keys map to conceptual endpoint identity nearly perfectly (operators rotate with years of
  notice); admin intent and oracle authorship are the gap.
- `tab-uuid-and-digest-roots` (**[HUMAN]**) — UUID- and content-digest-shaped identifiers are
  their own kind of root that conflicts with nothing except by string match; deserves attention
  later.
- `tab-one-amazonaws` (**[HUMAN]**) — the intuition that, unless the book deploys
  `amazonaws.com`, there is one `amazonaws.com` whatever the keys, addresses, proxies, or
  regions, to be squared against identity needing to be granular about exactly those at the
  oracle's whim. The conductor reads this as the operator-grouping question in its honest form.
- `tab-admin-says-this-is-my-server` (**[HUMAN]**) — the cloned-wildcard case is only a killer if
  the user is offered no sane way to say "this is one of my servers, do not key it by its cert";
  wanted: the at-home spelling inside the model without breaking how ordinary DNS hosts helpfully
  ignore round-robin; suspected spellable, unergonomic, and a wrong-SAME footgun without care.
- `tab-cyclic-chains` (**[HUMAN]**) — see § 1's standing note.

## § 5 — 2026-09-17: the robots.txt sitting (conductor's analysis, unacked)

The case is `kill-cdn-over-local-origin`. Worked from scratch in chat; the conclusions:

- `fnd-cache-checks-must-read-the-source` (+SURE) — an honest convergence check for a cache is a
  comparison between the cache and its source, so its read set necessarily includes the source.
  Where the admin writes the check (`curl … | cmp -s - /var/www/robots.txt || cf-cache purge …`)
  the local file is in the guard's marked reads and the collision with line 3 is structural, with
  nobody declaring anything about Cloudflare: Half-B, USER_STORY stage 1. Where an oracle's
  verdict reads the edge alone, the vouch is inadequate (converged ≠ no-op, `an-adequacy-bite`):
  an oracle-quality defect, not identity's.
- `fnd-the-residue-is-the-indirect-source-read` (+SURE) — the hard case is an ADEQUATE verdict
  that reads the source THROUGH the front (an origin fetch via a cache-bypass query), so its
  backing is keyed at Cloudflare's terminator while the bytes it read are the local inode's. The
  world is right and the key is wrong.
- `fnd-three-hops-close-it` (~SUSPECT it is the whole set) — the indirect read resolves to the
  inode through three routing lookups, each one line in its owner's oracle, none needing the
  others: the front's oracle yields a proxied resource to (origin address, host header, path)
  by reading the zone's own record; the stdlib answers whether an address is this box
  (`ip -o addr`) and which local listener holds the port (`ss -ltnp`); the server's oracle
  yields (host, path) to a file by reading its own config (`nginx -T`). With all three the
  backing yields into Tessa's inode: SAME, collide, attributed to the three lines. With any
  missing: an unknown link, UNKNOWN, collide under collide-by-default; wrong DISJOINT under
  spare-by-default. This is the argument for collide-by-default in the web-versus-local cell,
  and the hops are how an ecosystem earns sparing there.
- `fnd-just-run-dissolves-purge-class-lines` (+SURE of the band; **[HUMAN]** unasked) — a cache
  purge's honest check costs about what the purge costs (two fetches against one API call) and
  the purge is harmless when redundant: `KNOBS:kPROBING`'s JUST-RUN band. A `cf-cache` oracle
  that declines to vouch purges (`return 2`) makes line 4 always run: one line of attention,
  never wrong, no hops needed. Same posture as reload-class lines
  (`a-reload-has-no-observable-converged-state`). The hops keep their value for other web facts
  and for `dorc why`.
- `fnd-horizon-line-is-readability` (~SUSPECT, the conductor's answer to the human's devil's
  advocate) — "writable routes from A to B not visibly in the book are horizon" is too coarse.
  The principled line: a route is horizon iff no party who could reasonably author an oracle can
  READ it. A coworker's forgotten fswatch daemon that deletes a file on seeing a write is
  horizon (its behaviour is arbitrary code, unreadable); Cloudflare's origin-pull is not (the
  Cloudflare oracle reads the proxied record and origin); nginx serving a directory is not
  (`nginx -T`); a push-triggered GitHub workflow is not (`.github/workflows/`, and it is a wall).
  The difference is readable configuration against invisible behaviour, not local against web.
- `fnd-plain-sh-comparison` (+SURE) — a book whose admin-written guard reads the CDN for a
  regeneration decision is broken by CDN staleness under plain sh exactly as under Dorc; Dorc
  replicates the admin's bug and is no worse. The purge case is different: plain sh runs the
  purge unconditionally and is right; a Dorc elision on an edge-keyed verdict is worse than plain
  sh. That is why it is a killer and the guard-shaped cases are not.
- `fnd-what-collide-by-default-costs` (+SURE) — every web fact below every local write becomes a
  guard (a drifted `apt-get install` re-checks the later `route53` record at apply): one API call,
  no attention saved for that line. Buying those lines back needs the remote store's sole-route,
  which is the tabled social table. Not this sitting's.

## § 6 — 2026-09-18: the emulated-authority exercise

Record: `312b-exercises/an-emulated-authority-presents-the-real-id.md` (a six-line book: two DNS
records in two zones through two providers' tools; one bucket reached through a corporate proxy
by `aws` and through the public endpoint by `rclone`; the same bucket name seeded into localstack
by flag; a stdlib of Dana's DNS tree and Hugo's URL origin; four tool authors who never met).
The non-ledger home for how a stdlib handling of DNS and rootness would function. Nothing ruled;
the model unedited. Findings, all unacked:

- An operator-minted identifier sits BELOW the endpoint it was read through, and the endpoint is
  a name with no warrants; SAME being "and" over levels, an emulator presenting the real id can
  never mint SAME, and nothing mints DISJOINT there. The exercised gotcha is carried by
  arrangement, with no certificate read and no endpoint list. The tempting first draft (a bucket
  identified in an `sts` account id, route-scoped, warranted) fires the wrong SAME and is
  `311m`'s parentless warrant.
- An origin is a name: the one warrant that would separate the localstack line from the sync
  (`:guarantees-unique-name` on origins) is refuted by the same book's two endpoints for one
  bucket. The ambient endpoint is a singleton spelling (`self`), the home for the env-selected
  and binary-read endpoint, one mPlaceholder per mEntryChain.
- DNS is a genuine store of zones and RRsets and of nothing else. The glue is a delegation check
  each provider author runs alone (their API's assigned name servers against the parent's
  delegation): it gave the book's one survival (two zones under two delegations), gives two
  strangers in one zone their collision, and declines on a hosted zone nobody delegates to.
  Hanging origins under the tree is the wrong DISJOINT of `kill-route-declared-as-store`.
- :rootness on `.` is the stdlib's line and its knife is a split horizon across two mVantages.
- The cost shown: a record's chain ends at a mRoot, a bucket's at the mRoute; one drifted DNS line
  guards every remote line below it. The vanity pair stays UNKNOWN; the proxy's operator is the
  one party who could say otherwise.
- Bears on `311p` thread 6 (the route vouch broader than its justification, unedited in `311j`):
  two flag-spelled lines naming one literal origin are SAME only by § 1.10's vouch as written;
  under that thread's narrowing the ambient `self` is the only same-endpoint sameness.

## § 7 — 2026-09-18: the unacceptable cost, the four first guesses, and their decomposition

**[HUMAN]**: § 6's cost (every remote-touching line walls every other) is unacceptable: half of
ops is remote, and the attention product is dead under it. No mechanism or finding is ruled
against. Narrow repair by explicit statement (an override making an unenforceable but reasonable
claim about a remote service) is available; the worry is the general default case most users
meet most days. The flag stays on the table only for what it already names: faultless skips,
where the issue is epistemology and no speaker can be blamed or expected to fix it because the
fix is each naming the other, M×N across the ecosystem.

The conductor's first guess at the default case was four sentences: a provider's describer says
which hosts enter their service, what kind of thing it is (plain store, view, driver), and that
their store lives at their registrable domain; the stdlib says DNS registrations form a tree.
Walked in three stages at the human's direction (shave to objects and relations under
referential agnosticism and test for redundancy; find the pattern elsewhere; rebuild the DNS
story from the shaved parts). Detail and the revised strawman (Petra's store keys under Dana's
tree) are in the exercise record, sections "The four first guesses" and "A store-sort under the
tree". In brief:

- All four decompose into existing `311j` relations: a secondary mScheme's `:yields` with
  declining arms; `:sole-route`, a closed mPlacement set, and a finished footprint as three
  independent declarations (view and driver are the silent defaults; plain store is earned);
  one `:identified-in` edge; an ordinary mSort that is a delegation tree.
- The one piece of new content is the registration edge: "this store belongs to that
  registration and to no other". Its local twin is a filesystem in a boot by device number,
  where the arbiter hands the edge back (`st_dev`); on the web it is recited. The certificate
  is re-sited: a candidate MEASUREMENT of this edge (the far end proving which registration it
  answers for), the web's `st_dev`, not hardening of the host lookup. **[HUMAN]**: unconvinced it
  is optional, it may be the foundation; the tension is that people talk in DNS names and the
  path-is-not-an-id hole must not be re-dug. Open.
- Two things are recited on the web that the filesystem measures: the name-to-store lookup and
  the store-to-registration edge.
- **[HUMAN]** nack, accepted: strangers on one provider are not stuck. The endpoint is not a
  unit of composition: a host fronting several stores is several mSchemes over one class of
  strings, each declining the rest, and strangers' resource mSorts identify into a thin
  store-sort's mKeys and separate there as siblings. **[HUMAN]** nit: a thin store-sort is still
  less than ideal for collaboration; it is the open composition corner (separate authors
  composing arms into one lookup; fall through to another author's handler on decline; hard to
  spell in sh), reduced to a small corner and punted.
- **[HUMAN]**: the four first guesses are not dead. A first guess that decomposes into the
  abstract model is what to reach for when the abstract model chafes; recomposing
  technically-reachable constructs into first-guess ones is how ergonomics is re-bought. Default
  lean: take the existing construct, remembering that such minimalism can quietly hurt usability
  in ways found only later.
- **[HUMAN]** acks: routers and proxies need speech (what one does, translation or mutation, is
  unknowable without an author, as with pivots and wrappers); a self-compiled or self-hosted
  copy of a known shape is the abstraction-and-sharing question, for later.
- Strains the revised strawman exposed (exercise observations): § 3.2's walk asks `:sole-route`
  of a registration for the store hung there, and `311j` has no warrant for what is actually
  leaned on (a store has one registration, by its describer's word); one operator serves one
  tenancy under several registrations; an organisation's proxy needs an arm in a lookup only
  someone else owns; a multi-provider tool chafes at one mScheme, one mSort.

Tabled this sitting (**[HUMAN]**): the flag-shaped portion (two complete `:sole-route` chains to
two roots reading DISJOINT under the flag, as the multi-author no-fault residue), unread and
set aside so the critical path stays on the registration edge, which the next sitting digs.

## § 8 — 2026-09-18: authority, the registration edge dug, two holes; held unevaluated

Reached in chat, in order:

- The registration edge from first principles. One `stat` gives three things: a token handed
  back by the store's arbiter; route-invariance of that token; and knowing who answered. The
  third is free under an arbiter that is queryable, true, and cannot be proxied, which so far
  means the kernel running the interpreter, asked about filesystem-flavoured objects
  (**[HUMAN]**: that, and not local-against-remote, is the division). The web has the first two
  (operator-minted ids) and lacks the third (`an-emulated-authority-presents-the-real-id`). A
  verified certificate is not a token and not identity; it establishes who answered, and the
  tool already performs it against the host it selected. RETRACTED: "the certificate is the
  web's `st_dev`". The division: measure who answered; recite who is entitled to answer; yield
  only on agreement. What hangs on the tree is an id NAMESPACE under the authority that attests
  it, never a store by its hosts.
- **[HUMAN]**: authority is inherent to identity, treated epistemically and not adversarially: an
  authority is somebody everyone must know because the world said so, and Dorc piggybacks on the
  world's solution instead of demanding double social work of describers. The conductor's read of
  `311j` under it: a fully qualified mKey is a chain of deferrals to issuers; the warrants and
  `:sole-route` are testimony about an issuer's habits; :rootness is an authority needing no
  introduction; `:corresponds` is an authority over a transition; § 1.10's local-route vouch is
  the degenerate case of knowing who answered. One constraint falls out, no relation: a lookup
  that reads a token across a transit establishes who answered, or declines.
- DNS already models much of this (`NS` and `SOA` for delegation and the zone apex; the `AA` bit
  and non-recursive queries for authoritative against relayed; DNSSEC for attested delegation;
  `CAA`, `TLSA`, and challenge `TXT` records for "who is entitled"), and nothing about what sits
  behind a name. Relying-party validation of OpenID Connect tokens and SPIFFE trust domains are
  the same measure-recite-agree shape. A research round is likely; not started.
- **[HUMAN]** nack, accepted: no suffix lists. One step at a time: zone cuts are measured, and
  below the last cut a describer claims labels arm by arm; what is not written is not in play,
  which is composition, fail-safety, and the authority stop-point at once. "Registration" as a
  special level was a DNS trapping. `a-tenant-answers-under-the-operators-name` minted.
- **[HUMAN]**: a self-hosted service is the same oracle as a provider's; the interesting part is
  reuse, tabled. A private name failing across a change of mVantage is correct and is a litmus
  for the whole arc: Dorc as a typecheck of the admin's naming infrastructure.
  `a-private-name-resolves-only-inside` minted; the litmus paragraph and four gotchas now open
  `311j` § 4.

The two holes, attacked (conductor; the leaks and the candidate below are HELD, unevaluated by
the human, recorded so they are not lost):

- One hole, not two: § 3.2's DISJOINT-by-divergence assumes one referent has one chain. True
  under a single arbiter measuring every level; false wherever a thing sits in two authorities'
  trees. Murderers: a container named on a data-plane host and a management-plane host; a package
  reached through a registry host and an API host; a hosted-zone row described as the world's
  RRset by one tool's describer and as an operator's resource by another's; a registrar's API
  writing the delegation the tree itself measures; one repository namespace read through two
  registrations.
- A conflation found: who answered (attestation), who am I (credentials: an OBSERVER, never an
  mParent), and which namespace minted this id (the mParent) are three things. Hanging a bucket
  under the caller's account makes one bucket written by two accounts read DISJOINT;
  `312b:lead-identify-in-the-narrowest-primary-store` again. Visibility per caller is
  :observer-dependence.
- `leak-divergence-bypasses-the-placement-bound`: for a cross-sort pair the second bullet of
  § 3.2 answers DISJOINT on divergence and § 2.5's bound (overlapping or undeclared mPlacements
  collide) is never consulted.
- `leak-writes-do-not-disturb-placements`: § 2.4 says writes to an mPlacement affect K and never
  that a write to K disturbs K's mPlacements; one direction collides, the other leaks.
- `leak-the-sort-owner-cannot-place-a-federated-sort`: the owner of a world-wide mSort cannot
  enumerate who hosts its members; only the describer who glued knows; a yield may supply an
  mParent instance (§ 1.6) and nothing lets it supply an mPlacement instance.
- Candidate repair (~SUSPECT, unhunted): for a cross-sort pair, divergence yields DISJOINT only
  when both mSorts' mPlacement sets are closed and do not overlap, with footprints widened by the
  written mKey's mPlacements, and a yield allowed to supply the mPlacement instance. Same-sort
  pairs unchanged.
- Against the exercise record: its line 4 survived line 3 on identity alone while Dana's mSort
  declared no mPlacements, which § 2.10 reads as ⊤; the sysctl record had this right. Left until
  the hole settles.

**[HUMAN]** assertion, attacked and defended: identity-in-DNS against writable world-state is the
persisted-against-live division of `311q` § 13 (a config file's `[foo]` against the running
process's own answer; a registrar's row against what resolvers say): two authorities federating
behind a leaky abstraction to act as one, to be solved once and not by a DNS hack. Defended: a
hosted-zone row and the world's answer are a persisted form and a live form joined by movers
(the operator's own propagation; resolvers' TTLs), and the `until dig …` loop is the admin
writing the mover's wait by hand. It corrects the conductor's "one referent, two chains": they
are two referents, DISJOINT identity between them is right, and what is missing is interference
through the move, `312a:relation-requires-is-missing`, which mPlacement only approximates. The
one strain: under a single arbiter both forms are attested for free; in DNS the live side's "who
answered" is itself open, so the DNS case is the shared problem plus the no-arbiter constraint,
and the two compose.
