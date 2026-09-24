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
  said to give (mints identity, holds state, aliases-nothing-else, coherent local routing) reduce inside the
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
"aliases-nothing-else by decline discipline" (a valid mechanism, but it is the social canonical table);
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
  no attention saved for that line. Buying those lines back needs the remote store's aliases-nothing-else,
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
  declining arms; `:aliases-nothing-else`, a closed mPlacement set, and a finished footprint as three
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
- Strains the revised strawman exposed (exercise observations): § 3.2's walk asks `:aliases-nothing-else`
  of a registration for the store hung there, and `311j` has no warrant for what is actually
  leaned on (a store has one registration, by its describer's word); one operator serves one
  tenancy under several registrations; an organisation's proxy needs an arm in a lookup only
  someone else owns; a multi-provider tool chafes at one mScheme, one mSort.

Tabled this sitting (**[HUMAN]**): the flag-shaped portion (two complete `:aliases-nothing-else` chains to
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
  `:aliases-nothing-else` are testimony about an issuer's habits; :rootness is an authority needing no
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

## § 9 — 2026-09-18: several forms of one thing; the line around mReferent; `:aliases-nothing-else` split and attacked

**Whether to model the class at all** (argued both ways at the human's direction, each given its
best world). The class: several co-existing things that can disagree, joined by a step that
copies one into another (a reload, a boot, a login, a TTL). Not incarnations, which succeed one
another and never coexist. For: it is the dominant structure of ops state (`311s`, six
clusters); the commoner bug is the MISSING mover; the knowledge is per tool and amortises; only
a stateless prober can heal a lost mover, and only where the live side is readable; it is the
remote story's interference half. Against: correctness is already met (distinct things, DISJOINT
identity, a write to the file perishes what reads it); sh spells the changed-guard and Half-B
lifts it; a vouch on a read that is not really live (`nginx -T`, `sshd -T`) is a new knife;
movers are a bottomless per-daemon pit. The honest delta between the two best worlds is small:
elision of movers where the live side is readable, a vocabulary for "this verdict read the wrong
form", and the remote interference half, against one relation, describer burden, and that knife.
Conductor's lean (~SUSPECT): not now; additive later PROVIDED forms stay distinct things,
DISJOINT, related only by mPlacement; a merged-forms cell would make the later edge a re-keying.

Framing asked and answered: "two things, same identity, disagree, neither singularly
authoritative" is in-class, the general case. How authority flows is a property of the edges
(one-way: a config file into a daemon; two-way: a live ruleset saved back to its file; none: two
persisted spellings someone must keep agreeing). The fleet of cloned hosts is the boundary and
already lives in the per-host dimension.

**[HUMAN]** framing, this sitting:
- The concept was named "identity locus" for discussion: a referent may have zero or more; the
  case where Dorc holds one identity across a class of things the book treats as one. It is NOT
  minted as a model participant and must never appear as a tagged term. If the corner later
  proves most valuable with a mandated source of truth, "replica" or "fork" would lead instead.
- "Things the admin thinks of as one thing" is a bridge too far as a scope driver: intent is
  unknowable and the model will PUSH how users model. Scope is decided by the world-behaviour
  Dorc models or encourages. Much of "two things disagree and can then agree" is ordinary sh
  flow and tool behaviour; the only interest at this stage is whether the abstract model needs
  a concept of referent separate from the current one.
- Nack: not modelling it does not make it unmodelable. No hole has been shown; the discussion is
  ergonomics and fidelity, as it would be for any arbitrary thing lifted into the model.
- A sibling conductor's note: the relation is maybe always directional, but the direction is not
  fixed (Redis persists memory to disk, the inverse of the config case). Pointers: NMDA
  (RFC 8342) and earlier YANG datastore models; drift categories (intended, remnant); abstract
  interpretation as the closest semantic model (referents as summary locations; propagation as
  weak and strong updates).
- Aid is fully set aside: identity at the 311 stage is exclusively `compare()`, SAME and
  DISJOINT, collaborative mutually-unaware speech, and collision or sparing.
- The outcome may simply be a very precise line around mReferent and mKey; the concern is
  whether that line is a horizon beyond which meaningful behaviour is unmodelable.

**The synthesis** (conductor; the human acked most of it, nothing ruled):
- Identity loci are expressible today: two things that can disagree are two state-holders, two
  mCells with their own mPlacements, DISJOINT identity between them correct; a mover is a book
  line with a footprint on one and a backing in the other; propagation with no book line is
  `:reaches` from the writer's footprint; direction never enters identity.
- The hosted-zone murderer of § 8 was a describer conflating two loci and identifying an
  operator's row in the world's tree: a false sentence with an author, not honest strangers.
  Keyed in the store its state lives in, it collides with the other tool's key as strangers
  under one store.
- Proposed line: an mReferent is a single state-holder. If two reads through one mKey can
  disagree with no write between them, the mKey names a summary, not an mReferent.
  `:guarantees-unique-referent` is claimable only of single state-holders, hence never of a
  bearer-with-loci as a whole, with or without an authoritative locus; authority among loci is a
  fact about movers, which identity does not read. The test is operational and referentially
  agnostic. It creates no horizon (~SUSPECT): what lies outside "one referent" is "several", which
  the model holds; what stays outside is state with no read at all, the verdict plane's horizon.
  Consequence owed: § 1.9 gives an mCell its own mPlacement (`311p` thread 2).
- The concrete question left: the genuine case is ONE state-holder reachable through two honest
  id systems hung under two ancestors (a data-plane name and a management-plane id; one namespace
  under two registrations). Chains diverge high, DISJOINT follows, and whether a false sentence
  lies behind it depends on what `:aliases-nothing-else` says and who says it.

**`:aliases-nothing-else` as two sentences** (**[HUMAN]**: acked as a true narrowing of the problem-space,
no logical hole seen, not ruled; attack before any second item is called necessary):
- The two halves are distinguished by which kind of second chain a party can enumerate: a second
  CONTAINER presenting the same contents (known to whoever knows the container), and a second
  HOME for the children themselves (known to whoever knows the children). On any edge one half
  is usually trivial, and which one varies (inode in filesystem: the container half is live;
  namespace in registration: the children half), which is why every single-seat version fails.
- Single seats, tried: child-only makes a blanket inode claim cover both legs of an overlay pair
  and one file reads DISJOINT from itself (a correctness failure). Parent-only (the current text)
  leaves the two-registrations case with Dana's blanket arm as the only sentence to call false,
  about a thing she never heard of. Forbidding foreign children under a store kills the
  registration edge.
- The both-legs rule does most of the work: a second door is always a second store on the OTHER
  leg, and a view stays silent there, so the pair reads UNKNOWN whatever the viewed store's
  describer believes. Hence the container half need only say "I am not a view of something
  else" (self-knowledge), never "nothing views me" (unknowable). FUSE re-exports, encrypted
  views, bind mounts, `/proc/<pid>/root`, and a btrfs file keyed by subvolume against the same
  file keyed by device all land UNKNOWN or SAME; the last is saved by the rule that the tops
  share an mScheme, which is quietly load-bearing.
- The children half cannot be weakened the same way: the two-home cases are symmetric. It is a
  genuine negative existential about the world, the same species as `disturbs nothing-else`.
- What the children half is: `:guarantees-unique-name` lifted one level. The model has
  injectivity of mKeys within an mParent and nothing for injectivity of the mParent assignment:
  one thing, one home. § 4.2's pid case gets the right seat: a process has a key in every
  ancestor pid namespace, the half is false, the pid describer declines.
- It blocks nothing owed: siblings consult neither half; two files in two filesystems need "not
  a view" per type and "inodes live only in their filesystem", true by construction; the sysctl
  record's unequal-depth case NEEDS the children half ("a `net/*` knob lives only in its
  namespace"), which puts Rachel's line back beside Simon's; two providers need one sentence
  each about their own namespace. Movable children are true at each instant and the move is a
  routing write; things multi-homed by design (a process in several cgroup-v1 hierarchies; a
  clustered filesystem under two boots) decline, UNKNOWN, correct.
- Worries: the world adds doors (an operator opening a second registration for one namespace:
  safe while nobody hangs anything there, a wrong DISJOINT attributable to both hangers once
  someone does; **[HUMAN]**: chronology, MH2-flavoured, mildly out of scope for now); cargo-cult
  of a value-unlocking negative existential; whether this is two relations or one `:aliases-nothing-else`
  needing consent from both ends of the edge, a native describer giving both at once.
- **[HUMAN]**: if a negative existential is necessary it gets boxed in, attacked, and reduced,
  and that is to be talked through before any tuning. A long name alone is neither enough nor
  interesting; the work is the "rare" part, usually MOVING RESPONSIBILITY OUT of the negative
  existential, which is gradual-enhancement work, spelling-adjacent but not only spelling. It is
  one of the best motivations for keeping two things in the model that look similar: one carries
  the negative existential and is made much rarer by a second thing that is epistemically pure.

**Home-injectivity against mPlacements** (**[HUMAN]** side-eye, not a nack: "my two homes" spoken
by a describer is an old pattern; are these truly two tools, or is this (A) a finding about the
parent's statement plus (B) a finding that mPlacements are poorly designed and should be changed,
not supplemented). The conductor's exploration, no lean adopted:
- The container half is equivalent to the store's OWN closed mPlacement set: a store whose state
  lives only in itself or its backing device is thereby not a view, and a view either lists what
  it views (overlap, collide) or stays silent (⊤, collide). The sentence already exists; § 3.2
  reads a separately seated warrant instead of it.
- Over CONTAINMENT edges (the mParent holds the child's state) the children half is equivalent to
  the child mSort's closed mPlacement set: a thing in two containers has two mPlacements. With
  each mSort closing its own mPlacements one level, the seats of `311p` thread 4 and of Dana's
  blanket arm both fall out. Under this reading `311p` thread 3 (the identifying store inside the
  sibling overlap test) and the three held leaks of § 8 are all defects of how mPlacements are
  consulted, and the human's (B) holds for containment.
- The registration edge is NOT containment: a registration neither holds nor mints what is hung
  under it, so § 2.4's "exactly one mPlacement sits on the identifying chain" is false of it, and
  closing mPlacements says nothing about one registration against another. Exclusivity of
  ATTESTATION has no home in mPlacements. That residue is about a kind of edge `311j` does not
  distinguish from containment, more than about the parent's statement.
- Against changing mPlacements to carry identity: the sentinel's blast radius doubles; identity
  would read a relation `311j` deliberately reserves for interference; siblings still need the
  walk; it smells of the killed store-sets shape (§ 4.2), though a DISJOINT-only use escapes that
  kill (same sets, different roles reads "overlap", which is safe); and reading "writes to these
  mKeys" as including a stranger's aliases makes the sentinel unsayable. Against supplementing:
  two knife sentences about one world-fact that can disagree, with no rule for what disagreement
  means; the container half is a plain duplicate.
- A non-merging cousin: keep identity's own warrant, and cross-check it against mPlacement
  closure (a store claimed `:aliases-nothing-else` whose mPlacements are not closed at itself is a static
  contradiction, refused, attributed) rather than deriving one from the other.
- The positive, epistemically pure twin exists for both: an mPlacement entry ("my state also
  lives there") and, for attestation, "also attested under that registration" are collide-adding
  and safe from partial knowledge; only the closures are knives.

## § 10 — 2026-09-18: the equivalence of § 9 attacked, and withdrawn

**[HUMAN]**: "the store's own mPlacement is `:aliases-nothing-else`" is attractive and suspiciously tidy; is
the mPlacement of a store doing two jobs (a placement named for another reason that must not
ruin the privilege; a wish to disclaim the privilege without a second store to name; a store's
selfness and physicality conflated with how it holds children)? Expect a comparison of upsides,
not a kill. The conductor's attack; § 9's first two exploration findings are WITHDRAWN as stated:

- The two are different statements that coincide for plain disk filesystems. The mPlacement of a
  store says where the store's own state lives: arity the store, about the store as an object.
  The container half says whether the things identified in a store coincide with things
  identified in another: arity the store with respect to one child mSort.
- Closure true, yet a view: an overlay's state does live in its upper and lower directories, and
  each of its files is also a file of the lower filesystem; mPlacement overlap does not catch it
  at directory grain, since a footprint on a file and an mPlacement naming a directory are
  siblings, DISJOINT. Likewise a pid namespace (its state in the boot; its pids also the parent
  namespace's), an NSS view of LDAP, a chroot, an organisation's consolidated view of accounts.
- Outward mPlacement, yet a compartment: a loop-mounted filesystem, an LVM volume, a qcow2 image
  must name their backing (`dd` over it rewrites every inner fact) and are compartments for their
  children. § 9's "closed at itself or its backing device" smuggled in the distinction between
  BACKING (my state is bytes inside one foreign child) and VIEWING (my children are foreign
  children), a role mPlacements do not have.
- Cannot close, yet a compartment: a database whose describer cannot enumerate tablespaces, WAL,
  and replicas still knows its rows are reachable only through it; tying the two forfeits the
  sparing of others around its writes.
- Arity: a git repository is one object with one mPlacement, a compartment for its refs and not
  for its content-addressed commits.
- The equivalence is rescuable only by giving mPlacement entries a role (backs me; I present its
  contents), which relocates the container half rather than removing it. As a role on entries:
  names the viewed store when known, natural pure twin; strains on arity, puts identity's data in
  the interference relation, gives the closure a third consumer. As its own statement: right
  arity, sayable without knowing where one's bytes live, keeps the consumers apart, silence
  disables; duplicates when the viewed store is also an mPlacement, admits a contradiction.
- WITHDRAWN too: the cross-check "`:aliases-nothing-else` claimed but mPlacements not closed at itself is a
  contradiction"; it would refuse every loop-mounted filesystem.
- The sibling claim fails the same way: a process's state lives in one place and it is identified
  in two pid namespaces, so the child's mPlacement closure is true and it has two homes. The axis
  is MINTING against HOLDING. `:identified-in` has always been about who mints the address
  (`311j` § 1.6: a user namespace for a uid, whose state sits in a passwd file); mPlacements are
  about who holds the state; they coincide for disks and come apart for namespaces, kernel and
  web alike. The registration edge is one more minter that holds nothing, not a special kind.
  § 2.4's "exactly one mPlacement sits on the identifying chain" over-claims even locally.
- Standing: both halves of the split are statements about minters; mPlacements are about
  holders; two tools for two tasks. What survives of the human's (B) is the specific defects:
  § 2.4's over-claim, the identifying store inside the sibling overlap test (`311p` thread 3),
  and the three held leaks of § 8. The conductor's lean of § 9 is gone.

## § 11 — 2026-09-18: phrasing corrected; the table by meaning and by end; attestation opened

Every item in this section needs deep investigation; nothing is ruled. A successor starts here.

**Phrasing** (**[HUMAN]** gentle nacks, accepted):
- "Hands out the address" and "mints" pierce referential agnosticism, a hole fallen into three
  times. What Dorc can hold: T's mKey means something only relative to P, with an authored claim
  standing behind the "only". The gloss made "I mint from scratch" sound free. It is not: "I
  re-present nobody's things" is cheap only where an arbiter labels the INSTANCE (mount types).
  A `kubectl` endpoint may be a virtual cluster whose objects are a host cluster's; a registry URL
  a proxy; a database endpoint a pooler; `DOCKER_HOST` a socket proxy. The mSort's describer
  cannot know per instance; it needs a read or whoever stood the instance up. On the web it is
  what "who answered" tries to establish.
- "One mParent by design" holds per mKey. The arc showed a THING may have several mKeys under
  several mParents; the near closure below is the claim that this one does not.
- "Holds" and "lives in" pierce too. `311j` § 2.4 already has the clean form: "K's state is
  affected by writes to these mKeys". **[HUMAN]** offered it as a union: T resolves in P, or T can
  be surprisingly overwritten by others' writes to P. Conductor's tightening (phrasing only): the
  first member is already the walk's (an mKey against its own ancestor collides), so an
  mPlacement need only name things NOT on T's chain whose writes can change T (a uid's passwd
  file; a loop filesystem's image; a database's write-ahead file). Read so, `311p` thread 3
  disappears by definition.
- `backing-is-not-presenting` minted in GOTCHAS.

**The table** (rows named by what is true in the world; the two ends kept apart):

| what is true in the world | end | who can know it | positive entry | closure (the knife) | in `311j` today |
|---|---|---|---|---|---|
| T's mKey means something only relative to P | T's | T's describer | "this thing is also k' relative to Q" | "it has no other home" | entry: `:identified-in`, `:corresponds`; closure: ABSENT |
| | P's | P's describer, or whoever stood the instance up | "mKeys relative to me are Q's things, thus" | "I re-present nobody's things" | entry: `:corresponds`; closure: `:aliases-nothing-else` |
| a write to P can change T, P not on T's chain | T's | T's describer | "writes to P can change me" | "nothing else off my chain can" | `:lives-in` and its sentinel |
| | P's | P's describer | "writing me also changes T" | "writing me changes nothing else" | `:reaches` and the finished record |
| answers about T are A's word (OPEN) | T's | T's describer | "answers about me are vouched by A" | "by nobody else" | nothing |
| | A's | A's holder | "I also vouch for that" | "what I vouch for is this alone" | nothing |

Observations from building it:
- `:lives-in` and `:reaches` are ONE fact spoken from two ends (a package says "installing me
  disturbs that unit"; a loop filesystem says "writes to that image change me"); which end speaks
  depends on who knows.
- The second row already demands consent from both ends: sparing an unspoken pair needs the
  writer's "nothing else" bounded by the fact's closed mPlacements (§ 2.5). The split of
  `:aliases-nothing-else` brings the first row to the shape the second has had all along.
- The first row's positive entry is itself a knife today (`:corresponds` licenses SAME); a
  collide-only form ("may also be k' under Q") does not exist; silence already reads UNKNOWN, so
  it would matter only for contradicting someone else's closure.
- **[HUMAN]**: the rhyme is useful and is not proof. Of course an abstract model can be made to
  rhyme; the interesting question is which cells SHOULD be missing, because the world modelled
  has a global truth. The strawman being fought: "ops is chaotic and has some insane identity
  relationships; the whole table demands population to describe the world safely and with the
  epistemics right, without artificial backflips by the users doing the modelling."
- **[HUMAN]** current lean: re-cut 311 ENTIRELY according to this table; make it the mental model;
  give the pairs rhyming names; establish clear rules for how they compose and when both ends
  must be present to license something. First it must defend itself against "suspiciously tidy"
  and complexity creep: it must be MORE CORRECT than the current model on concrete ops strawmen
  with reasonably behaving, non-omniscient users.
- **[HUMAN]** on the closure's blast radius: nearly free and can be SPELLED. The model may keep one
  set of entities with two spellings at different levels of claim, as with emitting entries while
  omitting "and that's all", where the latter is both the opt-in to danger and the sentinel. The
  line between model and UX is fuzzy; not a nack of either option.
- Both-legs, defined: `compare()` walks down to the deepest level A where two chains are the same
  thing; each mKey's remaining chain below A is its leg; the qualifying warrant is needed on every
  store of BOTH legs, so a second door (always a store on the other leg) blocks DISJOINT by its
  own silence.

**Attestation, opened.** The conductor first held that "who answered" is a discipline on lookups
and the registration an ordinary `:identified-in` mParent. The hidden assumption: the answerer is
always an ancestor of what it answers for. Against it:
- apt: the mKey `nginx` means something relative to an archive's suite; its bytes sit on any
  mirror; what makes an answer trustworthy is the archive's SIGNING KEY, on nobody's chain and
  not a DNS name. Hung under the host's registration the namespace is wrong twice: two mirrors of
  one archive read DISJOINT; two archives on one proxy host are not separated. The signer gets
  both right and a mirror cannot forge it.
- Plural attestors are everyday (one operator's ids vouched under two or three registrations).
  Identification is single-valued per mKey; attestation is not. Forcing attestation into the
  chain made describers pick "the" registration, which is the two-registrations hole of § 8: a
  symptom of the conflation.
- Against: not strictly needed (one mParent plus an alias list in the lookup expresses it); a
  third pair of closures; buying cross-provider DISJOINT from it needs a new generator (closed,
  pairwise-disjoint attestor sets), which smells of the killed store-sets and might work here
  only because attestors compare in vocabularies the world forces (registrations; key
  fingerprints). `312b:lead-warrantless-tokens-are-witness-only` pulls the other way.
- **[HUMAN]**: attestation living in global namespaces through cryptographic reality plausibly
  suits a different kind of comparison (ack-ish). The `312b` rule likely concerns REPLACING a
  licence with attestation; what is being built may be a new CONSTRAINT that strictly buys
  collisions over the same model without it, the two standing relationships covering what `312b`
  feared. Unsure.
- Three candidate ways to license cross-provider separation, none chosen: the chain with a
  registration abused as an mParent; attestor-set disjointness as a new generator; the tabled
  flag-shaped clause of § 7.

## § 12 — 2026-09-18: the attestation flagship (conductor; unacked; built at the human's direction)

Task (**[HUMAN]**): give "attestation is not needed" its best foundation with every tool of § 11,
then build a counterexample that HURTS without a separate, non-store, non-identifying, n-ary edge;
abstract the pain fully, then lower it to several ops cases.

**The world without it, at its best.** Who-answered lives inside lookup bodies as ordinary sh: the
body checks that the tool verified TLS against a host its describer recites, or that a signing
key's fingerprint is one it recites, and declines otherwise; plural attestors are a longer
`case`. Strengthened by custody: only the namespace owner's mSchemes may yield its primary, the
owner publishes the ONLY reader of the operator's ids (an indexical singleton), and strangers
identify their things under that reader's mKey. Every honest outcome of the other world is
reproducible; apt is done right (the lookup yields the archive by its signer, whatever the
mirror). A wrong outcome is a false yield with an author, so
`lit`-style attribution holds. "Not strictly needed" is defended: for correctness with attribution.

**Where it hurts.** A reasonable, non-omniscient describer (Ravi) accepts `--endpoint-url` as just
a flag, passes it to his check, and files the bucket under the owner's reader, which reads the
ambient endpoint and never saw his flag:

```sh
aws --endpoint-url http://localhost:4566 s3 mb s3://acme-site-assets    # seeds the emulator
aws s3 mb s3://acme-site-assets                                          # the real bucket
```

Both mKeys get one fully qualified key; SAME; one probe answers both; the emulator has the
bucket, the real account does not, and the real line is elided. Ravi uttered nothing about
endpoints. The defect in the model's own terms: this is the one place where SILENCE LICENSES. A
filing under an mParent silently asserts "whoever answered my read speaks for this mParent", made
by every describer who touches the namespace, including those who never thought about it.

**The world with it, minimal** (a gate, never a generator):
1. A namespace's describer may list who answers for it: mKeys of answerer mSorts (a verified TLS
   name under a registration; a signing key's fingerprint; an ssh host key; implicitly, the kernel
   reached by this entry chain). Many-valued; others may ADD entries, attributed to them.
2. Required exactly where dangerous: a namespace may carry `:guarantees-unique-referent` across
   reads performed inside binaries only if it lists its answerers. Floor mSorts are untouched.
3. Every read (a lookup or a verdict) of an mKey at or under such a namespace reports who
   answered; inherited down the chain until a descendant lists its own.
4. Admission: a report comparing SAME with some entry files the mKey; anything else, no report
   included, reads unknown from that level.
5. What a report rests on (trust store, known_hosts, keyrings) is a marked read, so a book line
   writing them perishes admissions below it.
6. Nothing here yields SAME or DISJOINT by itself.
Under it lazy Ravi's facts are not admitted: UNKNOWN, both lines probe. Diligent Ravi reports per
invocation: the emulator's answerer is in nobody's list, the real line is admitted and elides on
its own probe. The party who knows a proxy is faithful (its operator) gains a seat, an added
entry, without forking anyone's lookup: a special-case answer to the arm-composition corner. It
strictly buys refusals over the same model without it (**[HUMAN]**'s suspicion holds for the
gate); an added entry admits more than the gate alone and still less than the ungated world;
`312b:lead-warrantless-tokens-are-witness-only` is respected, since a token only gates.

**Why a separate n-ary edge.** apt: the answerer (a signing key) is on nobody's chain, holds
nothing, is plural during key rotation, and one host answers for many archives. Neither the
relative-to row nor the writes-can-change row has a cell for it.

**The pain, abstracted.** A fact about mKey k is produced by a read r; r was answered by some
a(r) the engine cannot see unless told; filing the fact under k's mParent asserts that a(r)
speaks for that mParent. Without the edge the assertion is implicit and universal; with it, one
seat enumerates who speaks, reads say who answered, mismatch or silence is unknown. The model
ALREADY relies on this where it is free: placeholders are keyed by entry chain, which is "this
read was answered by the kernel reached through this entry". Attestation generalises the entry
chain to answerers reached inside binaries.

**Lowered.** Every mature tool has a verification-off switch or a silent route rewrite that a
reasonable describer will not enumerate: `--endpoint-url`, `--no-verify-ssl`, a swapped CA bundle;
apt's `[trusted=yes]` and a proxy repository under one host; a kubeconfig's
`insecure-skip-tls-verify` and a context NAME standing for a cluster; git's `url.*.insteadOf`,
`GIT_SSL_NO_VERIFY`, `StrictHostKeyChecking=no`; the docker daemon's `registry-mirrors` (content
is safe by digest, a root; the tag-to-digest answer is not); `dig` through a resolver against a
non-recursive query carrying the authoritative bit, or DNSSEC's validated bit, DNS's native
who-answered; and the degenerate case, `sudo`, `chroot`, `docker exec`, answered by the entered
kernel and already carried by the entry chain.

**Costs and open.** The report is author-produced: the knife moves from "must remember to
decline" (forgetting is wrong) to "must remember to claim" (forgetting is safe), but a describer
can still report falsely, and pasted report lines are a cargo-cult risk. A report is only honest
for the connection the tool itself verified; a separate probe is another connection. Whether the
engine could ever measure it (the trace backstop of `plans/077`) is unexamined. It does not by
itself give cross-provider DISJOINT, which stays with § 11's three candidates.

## § 13 — 2026-09-18: the human's read of the flagship, and the state at the rewind

**[HUMAN]** on § 12: "silence licenses" is fairly damning; it is exactly the pattern where a
thinner model fully represents the world and collapsing concepts introduces footguns. On balance
attestation is not dead on arrival and needs more investigation: a stay of execution for a
system that, by the conductor's own walk, is not strictly necessary. Not an ack, not a weld.
Noted, not for deep investigation now: consumption of attestation could be made opt-out (a flag
in the family of `--risk-faultless-skips`). That is dangerous in a specific way: authors in the
middle of the gradual-enhancement curve might stop authoring early, never writing the descriptors
that let Dorc elide their work for consumers who did not opt out: a coverage footgun for the
ecosystem, though it may flatten the curve.

State at the rewind, for a successor (description, not a worklist):
- Read `311t` whole, §§ 3, 11, 12 first; then the exercise record
  `312b-exercises/an-emulated-authority-presents-the-real-id.md`, knowing that its survival of
  line 4 past line 3 skipped mPlacements (§ 8) and that its revised glue predates §§ 10–12; then
  the litmus paragraph now opening `311j` § 4; GOTCHAS entries from
  `an-emulated-authority-presents-the-real-id` to `backing-is-not-presenting`.
- Standing, none ruled: the table of § 11 as the candidate re-cut of 311 (the human's lean, which
  must first beat "suspiciously tidy" and complexity creep on concrete ops strawmen); `:aliases-nothing-else`
  as two ends of one row (acked as a true narrowing); the line around mReferent of § 9 (a single
  state-holder; the identity-locus concept turned down as a participant); attestation as a gate
  (§ 12); the phrasing corrections of § 11; the three leaks and the candidate repair of § 8, held
  and unevaluated.
- Withdrawn, so nothing rests on them: the equivalence of § 9 (a store's own mPlacement closure as
  the container half) and its cross-check; "the certificate is the web's `st_dev`"; suffix lists;
  a stdlib root keyed on minted shape tags; "one referent, two chains" as the reading of the
  hosted-zone case; the registration edge as `:identified-in`.
- Tabled by the human: the flag-shaped cross-root clause; the declared canonical; modelling
  several forms of one thing; UUID and digest roots; cyclic chains; the admin saying "this is my
  server"; separate authors composing arms into one lookup; abstraction and sharing of oracle
  shapes; opt-out consumption of attestation; a research round on trust stores, relying-party
  validation, and DNS's own authority records, not started.
- **[HUMAN]**, 2026-09-18: a few ledger-only, fully acked changes are to be folded into `311j`
  when things quiesce; they have not, and `311j` carries only the § 4 litmus from this arc. The
  copy into the 312 series waits on this arc.

## § 14 — 2026-09-18: a successor's sitting: the class behind attestation, sparing as two questions, what the closures owe

Conductor: a rewound Fable successor. Nothing ruled. Words used in chat only and never to be
tagged: "landing" (where a contact ended), "pin", "forced route".

**Standup findings** (conductor):
- `disc-placement-pairs-read-two-ways` (+SURE of the texts; ~SUSPECT unreconciled) — `311q` § 8
  pins "KNOWN_UNSPOKEN mPlacement pairs do not collide" as the reading USER_STORY stage 5 and the
  facet collapse need; `311q` § 18 records the human's correctness choice as "they collide". Under
  § 18 every cross-vocabulary survival rests on the partition of `311q` § 18.
- `obs-table-covers-five-of-eleven-relations` (+SURE of the count) — § 11's table maps
  `:identified-in`, `:corresponds`, `:aliases-nothing-else`, `:lives-in`, and `:reaches` out of `311j`
  § 2.10's eleven. All eleven sort under four facts about the world: address (what an mKey is
  relative to), write path (whose writes change what), route (what a lookup passes through:
  `:yields`, the mTraversal, `:hierarchical`, `:lends`), and asker (`:observer-independence`).
  Route is the model's existing third relation (`311j` § 4.2, the kill of stored-in as one
  relation) and § 11's table has no row for it.

**The class behind attestation.** **[HUMAN]**: the flagship of § 12 is too narrow; a gut sense
that the class is large in ops, with real doubt whether licensing in the usual sense,
verification, attestation, and certificate checking interact with elision at all; a guessed
shape, explicitly not a requirement: cheaply recorded by a tool's describer where relevant;
protecting others' lazier work from themselves; already an ops pattern whose verification step is
painful and helps. Conductor:
- `fnd-most-verification-never-meets-elision` (~SUSPECT) — payload integrity (signatures,
  checksums) protects bytes a running line installs, and elision only removes runs. The conductor
  could not build an elision that apt's signing key protects (apt copies the remote namespace into
  local lists through a book-visible line, and every verdict reads local state), which leaves
  § 11's motivating case showing the edge's shape and none of its value. Authorization is
  :observer-dependence or a missing precondition; validity windows are the never-settled regime.
- `fnd-reuse-across-contacts-is-the-interaction` (+SURE of the logic) — plain sh never reuses an
  answer; Dorc reuses one across contacts (probe to apply; site to site; a fact past a running
  line), and reuse is sound only if both contacts reached the same other party, which can change
  with no write the book shows.
- The class in ops words: a recorded expectation of who is on the other side, checked at each
  contact, refusing loudly, never granting. `known_hosts`; borg refusing a relocated repository;
  Terraform's state lineage; Postgres refusing a standby whose system identifier differs;
  `start-stop-daemon --exec` against a pidfile; `mountpoint -q`; git's `safe.directory`; pip's
  `EXTERNALLY-MANAGED`; `allowed_account_ids`; molly-guard; a shell prompt carrying host, user,
  cwd, branch, context, and profile. Walks kept in chat: a `kubectl config use-context` line
  between two applies (the diligent footprint of the context line removes the wall protecting a
  lazy `kubectl` describer; pain 9 of `311q` § 16); `iptables` reaching the nft or the legacy
  backend by an alternatives link; a name failing over between plan and apply; an unmounted
  mountpoint as the free local twin (the device number rides the same `stat`).
- `fnd-attestation-decomposes-into-existing-seats` (~SUSPECT, unhunted) — "who is entitled to
  answer" is a lookup with declining arms (the exercise's Petra) fed where the contact ended
  instead of the typed name, so a signing key is a secondary mKey yielding the archive, and the
  added entry of § 12 is the punted arm-composition corner of § 7; the gate is `311j` § 1.6's
  refusal when two seats disagree; the one new piece is a per-shape demand that the mParent
  instance come from a seat that measured it.
- A contact's far end can be plural four ways, each with an existing plural home: a set
  (`terraform plan` across providers; the universal meet), a sequence (proxy, CDN, origin; the
  mTraversal), roles (reads on a replica, writes on a primary;
  `composite-identity-is-structure-not-a-bag`), and several tokens for one far end (mDerivations
  by coherence, whose disagreement is an emulator detector).
- **[HUMAN]**: "landing" reads as bog-standard work of this arc; renaming and cohering is welcome;
  inventing a parallel concept and keeping both is not, absent an eyes-open motivation.
  Conductor's mapping: a `resolve()` already returns where it ended (the mKey-Primary and a
  supplied mParent instance from one read); mPlaceholders keyed by mEntryChain are "answered by
  this kernel"; the standup `witness()` is equality of far ends across probe and apply; the
  ambient `self` spelling is a PREDICTED far end (the describer re-implements the tool's
  precedence); `plans/30S` pin-or-sever is the constructive form for environment.
- **[HUMAN]** nack, accepted: "who the tool connected to, never who it said it was" is impossible
  for large classes; `stat` reports only what the kernel said; an epistemic point apart from
  security and apart from user speech against Dorc measurement. Floated: two tiers of claim, one
  chaining through cryptography, perhaps first-class handling of one. Fence (**[HUMAN]**):
  anything where Dorc makes its own claims about cryptographic parties goes to an opaque reviewer
  first. Conductor (~SUSPECT): tier by CONSUMPTION, since origin is unknowable: a token consumed
  only through inequality (two contacts whose tokens differ share no answer; agreement licenses
  nothing; `312b:lead-warrantless-tokens-are-witness-only` as a rule), and an expected identity
  handed to the tool, which refuses inside its own contact while Dorc reads an exit status as
  today. RETRACTED: that a refute-only token catches § 12's emulator; the flag differing between
  the two lines caught it, which is declared routing input.
- **[HUMAN]**: the pinned form is interesting though surely a small class, because it is
  spelling-narrow AND improves the off-ramp, which nothing else this round does; what would
  modelling it buy. Conductor (~SUSPECT): model cost near zero (a bound mKey of a pin mScheme
  supplying the mParent instance at the bind seat and flowing into argv); it buys a plan-time
  literal for a remote thing, the tool re-checking at apply through the author's own bytes,
  indifference to routing the book cannot see, and the admin's seat of
  `tab-admin-says-this-is-my-server` in native sh; it never buys DISJOINT (one machine, many keys;
  content digests excepted, `tab-uuid-and-digest-roots`); issuer pins carry no functional warrant;
  pins held in ambient config are invisible again.
- **[HUMAN]**: MH2 never had a round; an old idea that has brushed against Dorc repeatedly and
  never sat well in it.

**Sparing is two questions.** Conductor:
- `fnd-sparing-needs-address-and-write-path` (+SURE of the logic; ~SUSPECT as the model's cut) —
  "not one thing" (the walk of `311j` § 3.2) and "no write path between them" (`:lives-in` and
  `:reaches` with both closures) are independent; a package and its configuration file are two
  things and a write to one changes the other. § 8's
  `leak-divergence-bypasses-the-placement-bound` is an address answer standing in for a write-path
  answer. In the other direction `30U` § 2 defers address aliasing to "the identity tier" while
  `311j` § 3.2 sends a pair of two mSorts back to the finished definition (+SURE of both texts): a
  write-path sentence standing in for an address answer.
- WITHDRAWN the same sitting: "transport lacks a route conjunct" as the reading of `311p`
  thread 6. **[HUMAN]** suspicion: the engine can never look inside a binary; a possible break of
  referential agnosticism. Conductor: a conjunct the engine must evaluate would be one; the honest
  form is `311p`'s own repair (the engine vouches ambient and inherited instances only; sameness
  of a leaf across two contacts is its lookup owner's `:guarantees-unique-referent`, which already
  prices routing the owner cannot see). Route stays a fact with one consumer, perishing.
- Corrections to the conductor's first rebuild of the table: the pairs `compare()` meets are four
  (two mKeys of one primary mScheme under one mParent instance; two mKeys of two mSorts under one
  mParent instance; one candidate mReferent under two mParent instances; two parentless mKeys),
  and both ends of § 11's first row sit in the third; "across mSchemes" at a divergence always
  means across mSorts, since an mFullyQualifiedKey holds only mKey-Primaries; for the second pair
  the "one thing" sentence exists as the human merge of `311j` § 1.2 and the "two things" sentence
  is the partition of `311q` § 18, absent, on which USER_STORY stage 5 rests (~SUSPECT of how a
  stdlib chains a filesystem and a service manager).
- **[HUMAN]**: the `tee /proc/sys` case (`311p` thread 5) is a poor driver: that path is not an
  `sm.File`, it is the knob, and the answer has the vague shape of the path's lookup handing it
  over; the case assumes a badly written oracle; the problem it stands for is real, and the fear
  inside it is about collaboration and composition. Conductor agrees: `311j` § 2.1 already has the
  net (a `resolve()` declines on mReferents its mSort does not describe) and `311m`'s walk needs
  Tessa to skip it; the hand-over itself is inexpressible today (an mScheme belongs to one mSort,
  § 1.3), which is the arm-composition corner of § 7. A driver with nobody badly written, offered:
  an `iptables` describer and an `nft` describer over one kernel ruleset.

**What the closures owe.** **[HUMAN]** (a principle, typed): unknowable does not entail
should-not-exist. Handling the unknowable is Dorc's routine job: spread knowledge as far as it
goes, and place the genuinely unknowable residue that value still needs between a double-ended
acknowledgment by contract and the flag. Conductor, re-done under it:
- "Nothing re-presents me" is unknowable (a store is overlaid, exported, or bind-mounted later
  with no mark on the viewed side) and, separately, not needed for value: any second chain to a
  thing in P passes through the presenting store, whose own silence blocks DISJOINT under
  both-legs (overlay, NFS client, and nested pid namespace walked). Two properties, not one; the
  redundancy holds only where every chain lists every store it passes through
  (`312b:lead-identify-in-the-narrowest-primary-store`). ~SUSPECT complete.
- `fnd-the-binarys-route-rides-the-vouch-unnamed` (~SUSPECT) — an mTraversal covers what an
  authored `resolve()` read; a fact produced by running a binary also carries "and the binary
  went where the mKey says", a negative existential about the binary's inputs that no describer
  can close (tools gain routing inputs long after their describers wrote). It is needed for value
  wherever a describer supplies a complete chain for an ambient contact, and it already sits where
  the principle puts it, inside the vouch under the flag, but nothing names it (USER_STORY's
  receipt does not). The model's safe silence exists: an unsupplied instance is an unknown link,
  which reads UNKNOWN and guards on drifted days. The epistemically pure twin (§ 9) is forcing the
  route in the book line itself (`--context prod`; `git -C`; an `ssh` line naming its
  `known_hosts`), bound by the describer at the bind seat and threaded into the check; legal in
  `311j` today. No structural repair proposed; the two shapes on the table are saying the unnamed
  conjunct in `311j` § 1.7, and the per-shape demand above, under which a borrowed ambient
  singleton is refused where a namespace's describer asked for a measuring seat.

**Corrections and positions, later the same sitting.**
- **[HUMAN]** position (stated as open to pushback; the human adds it to the edits owed to
  `311j`): an mScheme may yield into several mSorts. An mSort has exactly one primary mScheme,
  which must cohere by yielding at least some of the time into the mSort that claims it; beyond
  that nothing constrains where an mScheme yields. `311j` § 1.3, § 2.1, and the parenthetical of
  § 3.2's fourth bullet say otherwise; their source is `312b` § 8's
  `fnd-schemes-belong-to-one-sort`, from a sitting headed nothing-ruled, and no typed ack of it
  was found. Consequences (conductor, ~SUSPECT): the path's owner can hand a `/proc/sys` path to
  the knob's mSort, so the line above calling that hand-over inexpressible describes `311j`'s text
  only, and what remains of the arm-composition corner is the stranger's side; the exercise's
  `obs-one-scheme-one-sort-chafes-at-multi-provider-tools` dissolves; a footprint's mSort, and so
  its entailment, is whatever the lookup reached. Costs: the static refusal `312b` § 7 counted as
  a benefit of two species is forfeited; `311j` § 1.4's "never changes the structure of the
  mFullyQualifiedKey" weakens to choosing among declared structures.
- **[HUMAN]** lean: warrants and like annotations belong per lexical path of a body, on a line
  they are about, which also gives aid a line to point at; a body may then claim different levels
  of dangerous truth for different argv and world-state; a win, and what the engine (lattices,
  value-flow placeholders) is best at. `311j` § 2.2 already does this for a primary mScheme, per
  matched shape.
- WITHDRAWN as a finding: `fnd-the-binarys-route-rides-the-vouch-unnamed`. **[HUMAN]**:
  "unknowable", in the sense relevant to the flag, is epistemic and set-logical, never "hidden" or
  "laborious"; what a tool consults is visible to Dorc as input values, or exposable by the
  describer's authored probing, or seekable by the describer (the manpage, at the floor); the one
  unknowable in the framing is future change of the tool. Conductor agrees: which cluster a
  `kubectl` line reaches is the mParent instance of its mKey
  (`same-name-different-referent-per-viewpoint`, selected by configuration instead of a wrapper);
  supplying it is ordinary describer work through the three seats; the reads of that lookup are
  marked reads like any other, and their incompleteness is the adequacy already priced at the
  vouch (`ANALYZER-NEEDS:an-backing-selfframing`). What stands: `311m`'s text item that `311j`
  § 3.3's routing species names only `PATH`; and "forcing" as a plain observation, never a
  mechanism: a book line that states its selection in argv (`--context prod`) lets the describer
  supply the mParent instance at the bind seat from an input value, through an arm that owes
  nothing to the tool's precedence rules, the part most exposed to future change; the ambient arm
  can be left unknown or declined until its describer has bothered.

**The small-edits sitting.** **[HUMAN]** plan: loose ends; then the edits owed to `311j`; then,
only on the human's explicit ack, a new adversarial panel over those changes. The model keeps the
name 311, and revisiting any larger item becomes 312; this supersedes `311q` § 12's line about the
312 series (two fundamental rewrites were killed, properties and the attestation edges, which is
why). **[HUMAN]** on phrasing: referentially agnostic grammar is primary; a non-agnostic example
may stay as a parenthetical; where the ledgers show repeated breaches prefer a less pulling
example; about five added words per site at most; a specification, never a teaching document.
- Typed acks for `311j`, each re-walked by the conductor first: § 1.10, the engine vouches ambient
  and inherited instances only, same-spelled floor leaf mKeys being separate mPlaceholders (full
  ack; the defect is a SAME with no author behind it, against § 3.5); § 3.3's routing species
  widened past `PATH` to any environment, cwd, or configuration a lookup reads (acked in spirit,
  the mechanism unexamined; **[HUMAN]**: an engine decision deserves either authored speech or
  differential-tested proof that shell always behaves so, and whatever is not shell behaviour
  demands the former); § 1.9's reboot derivation struck; § 1.2's parenthetical pointed at § 3.2;
  warrants per lexical path for every lookup, § 1.5; the sentinel's danger said briefly, left to
  the conductor.
- Punted to 312 by the conductor's re-walk: rewording `:aliases-nothing-else` to the self-knowledge reading
  (`311p` thread 1's soundness argument for unequal depth uses the strong reading; the weak one
  needs the child's "no other home"); the single-state-holder line of § 9 (a bearer with two loci
  could then carry no `:guarantees-unique-referent`, so a wrapped `systemctl` read could share no
  fact with the bare line unless mCells get their own stores).
- **[HUMAN]** nack of "mPlacements are off-chain only": as an authoring rule it is a no-op and a
  burden; chain members are implicitly mPlacements, a redundant declaration is harmless, and a
  list copied from a tool's documentation should not move when the chain does. The conductor's
  substitute, "a shared chain member is never an overlap in § 2.5's test", WITHDRAWN after a walk
  (+SURE a hole): two vocabularies for one slot, both identified in one mParent instance, each
  closed at just the parent; today the shared mParent instance is the SAME mPlacement on both
  sides and the pair collides, which is the net `311j` § 1.2 names ("except where both owners'
  mPlacements land on one place"); the clause removes that net and the second line is spared.
  `311p` thread 3 closes without an edit: mCells that should separate take separate stores at the
  substrate's granularity, and mCells sharing a store collide (`311q` § 8). Noted: counting EVERY
  ancestor in § 2.5's test would overlap every pair on a host at the boot and end USER_STORY
  stage 5; the test counts the mParent instance only, a pragmatic cut whose principled form
  (address first, then write path; the partition of `311q` § 18) is 312's.
- CORRECTED, on the human's prompt that reuse being available does not make it right: "`311p`
  thread 3 closes without an edit" is overstated. Separating sibling mCells by giving them
  separate stores works only under the reading of § 2.5 in which SAME mPlacements alone overlap
  (`disc-placement-pairs-read-two-ways`), since the two minted stores are themselves two mSorts
  under one mParent instance that nobody has called distinct. Under that reading the net of
  `311j` § 1.2 is evaded by the same idiom: a stranger who mints a store of their own under the
  shared mParent instance (a vendor's table in a network namespace, against a knob identified in
  the namespace directly) presents an mPlacement that is the other's descendant, never SAME, and a
  finished definition spares one slot from itself (+SURE of the walk under that reading; it holds
  in `311j` today, and is `311p` thread 5's second consequence met from thread 3's side). Under
  the other reading (whatever is not DISJOINT collides; the correctness choice of `311q` § 18)
  the net holds at any depth and no sibling mCells ever spare without the partition sentence. The
  SAME-only reading is the disclosed-weak name floor of `311j` § 4.2 moved onto mPlacements:
  unequal names, nobody's speech, sparing. Threads 3 and 5, the two readings, and the partition
  are one knot, judged by the conductor not small.
- **[HUMAN]**: separation resting on silence is a dealbreaker wherever it licenses something
  dangerous; the case as walked is unacceptable (mutually unaware speakers doing ordinary things
  must never yield a wrong elision), so it is forced to default-collide and speech is forced on
  whoever climbs out. A spelled negative existential is strictly better than an implicit one: a
  specification sentence "no A ever coincides with B" is the same existential, acked by nobody,
  with no escape route. A parent-spoken partition is sound and value-expensive, and makes
  third-party children vestigial for sparing. Find the existing rhyme before minting.
- `fnd-difference-between-strangers-is-earned-one-way` (conductor; ~SUSPECT, unhunted) — wherever
  this corpus separates two mutually unaware authors' things, both have identified into one key
  space whose owner warrants `:guarantees-unique-name` from self-knowledge: canonical package
  names (USER_STORY stage 6); Dana's zones (§ 6); the thin store-sort of § 7, in the words of the
  human's own nack there ("strangers' resource mSorts identify into a thin store-sort's mKeys and
  separate there as siblings"); inodes, for every file-backed store
  (`312b:lead-identify-in-the-narrowest-primary-store`). Never pairwise naming, never a parent
  enumerating its children, never silence. The partition sentence of the conductor's counterpoint
  is WITHDRAWN: the pair "two mKeys of two mSorts under one mParent instance" keeps no "two
  things" sentence, and the way out is to become the first pair (two mKeys of one primary
  mScheme) by identifying into a shared key space. Third parties spare by filing themselves; a
  false filing is a false `:identified-in` with an author. Mode against contents is Tessa's two
  mKeys of one mScheme under the inode, under her warrant.
- `fnd-address-first-dissolves-the-two-readings` (conductor; ~SUSPECT) — if no pair spares unless
  the walk of `311j` § 3.2 answers DISJOINT (KNOWN_UNSPOKEN never spares; § 3.2's fourth bullet
  and `30U`'s cross-kind licensor change), then: the vendor-against-knob hole closes at any depth
  (tops of two mSorts, or a minted store that cannot honestly say it presents nobody's things);
  the mParent instance leaves § 2.5's test, because the walk already covers writes at or above
  it; the test runs over declared mPlacements only, overlap meaning not-DISJOINT; and both
  closures reach only what can be compared, the human's strike of the implicit universal
  (`311q` § 18) applied to both ends. Cost: USER_STORY stage 5 keeps its file-backed survivals
  through inodes and loses `active` against files until a stdlib key space exists for what sits
  directly in a boot. Not small; touches `30U`.
- **[HUMAN]**, 2026-09-19: acks the logic of the two findings above as read and takes it for the
  correct route; § 2.5 is never to be left ambiguous, and is to take its narrowest meaning unless
  a reason is found that it needs the wide one. The conductor's last hunt over address-first
  (each ~SUSPECT, none hunted further):
  - the wide meaning IS needed once the mParent instance has left the test: a declared mPlacement
    spelled in a private vocabulary is incomparable with a stranger's write to that very thing,
    and under the narrow meaning that silence spares; the ancestor problem that made the wide
    meaning value-dead came only from the mParent instance sitting in the test;
  - across two mSorts, only the shared-key-space clause of § 3.2 may separate (two tops of one
    mScheme under `:guarantees-unique-name`); the one-top clause rests on `:aliases-nothing-else` alone, a
    universal about every route including strangers', and with it a vendor's minted store that
    honestly believes itself primary separates a slot from the stranger's knob naming it; for two
    mKeys of one mSort the clause stays as acked on 2026-09-17;
  - the write-path question is asked of every pair, the same mSort included, and over the
    declared mPlacements of every member of the fact's chain: `dd` over a loop image against a
    file inside the loop filesystem is two inodes of one mSort, DISJOINT by the walk, and `311j`
    as written never reaches the filesystem's mPlacement from the file's fact;
  - a footprint naming a whole mSort and no mKey has no chain, reads UNKNOWN, and never spares
    across mSorts (USER_STORY stage 5's own apt strawman is one);
  - the curve: an author at USER_STORY stage 4 must identify their thing in a shared key space
    (one `:identified-in` at the bind) before anything of theirs spares or is spared.
  Why the conductor called it large: the text change to `311j` is a handful of sentences; the
  weight is that `30U`'s ruled sentence (a finished definition generates cross-kind disjointness)
  becomes necessary-not-sufficient, `ANALYZER-NEEDS` rows and USER_STORY stages 5 and 7 follow,
  every oracle's shape is constrained by it, and one more turn of hunting moved the rule twice.
- **[HUMAN]** acks: § 2.5 spares narrowly and collides widely (the earlier "narrowest" meant
  least value yielded); the write-path question asked of every pair, at first gloss. Memory
  check, confirmed by the conductor: strangers' routes are why § 9 split `:aliases-nothing-else` into two
  ends, heading toward pairs that need both present. It carries a front over another's store (the
  front's describer is silent on its own leg) and does not carry the vendor's store against a
  knob sitting directly in the shared mParent, where the stranger's leg holds no store to be
  silent and both closures are one party's.
- CORRECTED (conductor): "it constrains every oracle's shape" overstated what address-first adds.
  Under collide-widely alone, sparing across two mSorts already needs both sides' mParent
  instances and declared mPlacements to `compare()` DISJOINT, which already needs a shared key
  space. What address-first adds over § 2.5 pinned wide: it asks the walk about the two things
  themselves, so `:aliases-nothing-else` is demanded of every store on both legs, where the mPlacement test
  never asks; and it takes the mParent instance out of that test, which gives back the pairs the
  wide meaning kills (a store that sits inside a filesystem against any file of that filesystem:
  most of USER_STORY stage 5). A lower bound with nobody badly written, wrong under either
  reading of `311j` today: `ctr task kill` of a container's task against a converged
  `docker start` of that container, two daemons comparable and distinct as processes, each
  describer's mPlacements DISJOINT from the other's, the containerd describer's definition
  finished, and dockerd a front over containerd's things whose describer would withhold
  `:aliases-nothing-else` if anything asked.
- **[HUMAN]** hard ack, as written: an author who mints an mSort and hangs it on nothing
  comparable stays guard-only under the flag. The reason is referential agnosticism: that author
  could as easily have typed a name for an entire duplicated filesystem type and hung it on
  nothing; a name typed with no warrant and no correlation visible to Dorc can never be
  meaningfully compared with anything. **[HUMAN]** on the conductor's restriction of § 3.2's
  one-top clause to one mSort: reads as a narrow patch; wants a more fundamental fix or a
  co-example outside it.
- `fnd-a-closure-reaches-its-speakers-vocabulary-and-no-further` (conductor; ~SUSPECT) — the
  restriction is one instance of a rule the model already follows in one place. `311j`
  § 2.7-observer-dependence is written so: independence is declared per NAMED mSort O, and an O
  its speaker never named stays dependent. Three other closures are written as universals over
  strangers' names and each yields a wrong elision there: the finished record (`ctr` against
  `docker`, above); the `:lives-in` sentinel (the same walk from the other end); and the one-top
  clause, which rests on "the thing in the store has no other home", a sentence only that thing's
  describer can say and only about names that describer knows, which is the child's-end closure
  of § 11's first row doing its job at its honest reach. The wrapper's sentinel has the same form
  (`311p` thread 7) and is already harmless because § 2.7's default catches what it would carry.
  Across vocabularies, difference comes only from a shared key space whose owner warrants it.
- **[HUMAN]** ack, 2026-09-19, after repeated attack: separation is only ever concluded from a
  single definition's own distinctions. Two tops: one mScheme's body warrants that its distinct
  mKeys are distinct things. One top: one primary mScheme's `resolve()` body files different
  shapes at different levels, and an omission is a distinction only inside the body that made it;
  the test is read off authored text (that body has some arm whose `:identified-in` names the
  store's mSort), and the same mSort is that same body trivially. This is `311j`
  § 3.5-committee-law-satisfied applied to `compare()`; the one-top clause as written drew a
  distinction from two definitions at once. **[HUMAN]**: "stranger" is no term of the engine's; no
  mechanic interprets reverse-DNS names; absent `30J`'s speaker closures the largest unit
  guaranteed to be one speaker is the single definition. The human's formulation of the wrinkle,
  confirmed: the claims place B precisely (in Y, only in Y, Y in X) and place A only as finely as
  A's definition speaks; nothing says A is outside Y, and nobody who could be asked holds both
  words. Depth changes nothing: the walk re-localises every pair to the deepest shared level, the
  test reads the one store under it, and deeper stores only add closures that can each be false.
  Surveyed at the human's prompt: a child overlapping its parent is among the commonest shapes in
  ops (managers' inventories, nested namespaces, groupings, partitions, peer tools carving one
  subsystem), the dangerous configuration of it is rare, a loop filesystem is backing and never
  this shape, and the restated clause costs only a privately minted sub-store its separation from
  another definition's direct child.
- **[HUMAN]** acks toward the fold: `311j` § 2.4's "exactly one mPlacement sits on its identifying
  mFullyQualifiedKey" becomes "an mKey's mParent instance is implicitly one of its mPlacements;
  declaring it again is harmless"; the emulated-authority exercise record takes adjacent
  `<!-- /* superseded: … */ -->` notes, never a rewrite; § 4.2 gains brief lines for the shapes
  this sitting killed (the human may later drop that section for tightness); the model grows a
  very terse list of what it is known to supersede as currently written (an exception to normal
  practice, for a churn-heavy round; root docs excluded, known stale). The panel may fall to a
  successor; no predisposition on its brief. **[HUMAN]** on naming: `:relation` is only a gloss
  for an abstract relationship and constrains no sh spelling; a warrant's likely spelling home is
  a record to the report lane; "view" is mildly unsold. Conductor (~SUSPECT): the additive entry
  relation floated beside the closure is NOT owed and stays out of 311; and the rename carries a
  change of reading, from § 2.2's "reachable only through it" to the self-knowledge form of § 9,
  which the restated one-top clause no longer blocks (if A were B, A's definition files the
  thing outside every store of that mSort while B's files it inside one, and both hold only if
  that store gives its own mKeys to its parent's things, which the closure denies); the
  thing's-end gap stays 312's. Candidate names live in the root `_tmp-` naming file.
- STAMPED (**[HUMAN]**, 2026-09-19, "stamp it all"): the store's closure is
  `:aliases-nothing-else` in 311; many names will churn if the table shape is taken, the
  "guarantees" being better spelled as closure statements when spelling lands. **[HUMAN]**: Dorc
  as an engine for code motion modulo aliasing across unknown externs is quite exact, and a good
  model to fall back to by default. Folded into `311j` the same day, one commit each
  (`c300446f` to `1c65920e` and the rewrap after): § 1.10 the narrowed vouch; § 3.3 routing
  mutations widened past `PATH`; § 1.9's reboot derivation struck; § 1.2 pointed at § 3.2; § 1.5
  warrants per matched shape; an mScheme yields into any mSort per matched shape (§ 1.3, § 1.4,
  § 2.1, § 3.1, § 2.10); the rename with the self-knowledge reading (§ 2.2 and every use); § 3.2's
  separation from one definition's own distinctions; address first and then write path (§ 3.2's
  fourth bullet, § 2.5, § 2.4 with the implicit mParent instance, § 2.10, § 3.6) with the
  sentinel's danger in its table cell; the phrasing pass at four sites; five lines in § 4.2; a
  new § 3.7 naming what the model supersedes as written (`30U`'s generator sentence; two
  `ANALYZER-NEEDS` rows). The emulated-authority exercise record took three adjacent superseded
  notes.

## § 15 — 2026-09-19: a successor's standup: grain, the parent that is no placement, the write side of placements

Conductor: a rewound Fable successor. Nothing ruled beyond the typed acks named. The panel of § 14's
plan is on HOLD (**[HUMAN]**, typed); the conductor's standup read was that nothing substantive
blocks it, that about a quarter of the model's lines turned over since the last panel's copy, and
that `311p` is burned down except the sibling-cell residue punted to 312.

- WITHDRAWN (conductor's own, the same sitting): a clause skipping, in § 2.5's test, any mPlacement
  that is a shared ancestor of both mKeys. Counterexample with every sentence true: a describer of
  database rows identified in the main file's inode, unable to enumerate sidecar files across
  journal modes, declares that the rows live in the filesystem and closes the set; a write to a
  sidecar is a sibling inode, DISJOINT by the walk, and the clause spares the row fact. The engine
  cannot tell "my mParent, redundantly" from "somewhere relative to my mParent, I cannot say
  which"; the second must collide.
- `ack-a-parent-is-no-placement` (**[HUMAN]**, typed ack; folded, `5e5ef9ed`) — § 2.4's "implicitly
  one of its mPlacements; declaring it again is harmless" was false twice over: an implicit
  mPlacement would collide every sibling pair, and a declared one walls every sibling. It
  supersedes the wording acked in § 14. The intent behind that wording (a list copied from
  documentation costs nothing) cannot be met.
- `ack-grain-of-a-pair-is-its-coarser-key` (**[HUMAN]**, typed ack) — an address edge and an
  mPlacement are both dependence (of what the mKey means; of the state). Grain lives in mKeys,
  never in edges: `compare()` separates only below the deepest shared level and only where both
  chains continue below it. A sentence (mPlacement, footprint, or fact) naming a store has an
  empty leg there. An mPlacement naming an mKey relative to a store is exactly as fine as two
  siblings. "A write inside p counts" rests on silence (nobody has said a write to an mKey
  relative to p is no write to p), never on a store's state being an aggregate, which the
  conductor first wrote and which pierces referential agnosticism.
- `fnd-a-store-named-placement-is-the-cheapest-honest-closure` (conductor; read and not contested)
  — coarse and closed beats fine and open: a describer who cannot enumerate can still say "this
  store and that one, nothing else", where an open list collides with the whole book. The ladder:
  undeclared; a store; a sub-store carved by shape; enumerated or probe-emitted mKeys; closed.
- CORRECTED by the human: a pattern over mKeys is meaningful. It is a shape arm in a lookup's own
  body yielding into a sub-mSort (`/proc` under the path mScheme; § 2.2 already matches shapes of
  an mKey-Primary's value). What § 4.2 refuted was a consumer-side prefix predicate used for
  containment. Its reach is bounded by § 3.2's one-definition rule; a third party's arm is the
  arm-composition corner of § 7, still punted.
- `fnd-writes-do-not-answer-to-their-stores-placements` (conductor; § 8's held
  `leak-writes-do-not-disturb-placements` dug at the human's direction; **[HUMAN]**: an omission,
  not a complexity; the precise rule UNACKED) — a fact answers to the mPlacements of every member
  of its chain and a write answers to none of its own, so a write to an mKey relative to a store
  spares a fact about whatever that store's describer named (a converged loop mount; a write
  inside it; a checksum fact on the image: DISJOINT by the walk, the store's `:lives-in` on the
  wrong leg, the writer's finished record false of a thing its speaker cannot see per instance).
  The population today is the File family (loop mounts, image files), because elsewhere the walk
  already answers KNOWN_UNSPOKEN; it grows with every shared key space. Candidate rule: a write to
  an mKey may also be a write to any mPlacement declared by a member of that mKey's
  mFullyQualifiedKey, nobody having said otherwise; each such mPlacement strictly below the
  deepest level the footprint mKey shares with the fact joins the footprint (tested against the
  fact and against the fact's mPlacements), and a member with no closed set joins as ⊤. The fact
  side stays as written. It is the constructive half of bounding the finished record (`311q`
  § 18's struck universal; § 14's
  `fnd-a-closure-reaches-its-speakers-vocabulary-and-no-further`): the finisher answers at the
  level spoken at, each store above answers for itself. Why shared stores stay out: a path from
  one sibling through the store's mPlacement to another is two whole-store sentences composed,
  over a pair the store's own key space already separated at finer grain; including them ends
  USER_STORY stage 5 (two files, one backing, SAME). Why the fact side is not mirrored down to
  "below the shared level" too: a store whose mPlacement is an mKey relative to itself (a zone and
  its in-bailiwick glue; a filesystem and its quota file) keeps its net there.
- `fnd-coarse-bounds-are-two-way-by-nature` (conductor; the human asked for the damage in detail) —
  a drifted `docker pull` above three `cp` lines on a single-root host brings all three back as
  guards. Three layers cause it and the candidate rule is the thinnest: an incomparable vocabulary
  never spares (§ 14's hard ack); an honest describer who cannot list what a write touches must
  bound the write set as coarsely as the read set, since "the image, nothing else" is false of a
  pull; the candidate rule bites only where both were dodged. Monotone against having no oracle
  (a total wall); confined to drifted days and to the one filesystem named; a guard, never a run.
  TUNE, unacked: the pickup reads members ABOVE the written mKey only. At the written mKey's own
  level the speaker can know where that thing's state is recorded (the ordinary finished-record
  knife, at its designed seat); what can change a thing legitimately exceeds what writing it
  touches (a resolver cache bounded coarsely over its upstream files, whose flush writes none);
  and USER_STORY stage 5's price stays one `disturbs()` arm per verb. Given up: the automatic
  collision of `usermod` with a fact on the passwd file, which returns to the sort owner's
  authored `:reaches` (USER_STORY stage 7's seat). Residue: the level a coarse bound is hung at
  changes outcomes for a sloppy describer, never for an honest one.
- `fnd-a-prefix-is-a-naming-convention-and-a-mount-is-a-boundary` (conductor) — "it touches only
  that directory and below" is a sentence about paths and the fact is held by inode; a bind-backed
  volume, a relocated data root, a hardlink, and a rename each cross a directory boundary, and the
  crossings are labelled on the thing crossed (link count; the mount table), so membership is
  measurable, never a warrant. A directory relabels things of its filesystem and can never say
  `:aliases-nothing-else`; `/proc` carves cleanly because it is a mount. Sources for "no general
  reverse index": [B-unix-se-files-by-inode-2012] [B-stackoverflow-same-inode-2009]
  [A-btrfs-inspect-internal-manpage-2024] [A-lwn-xfs-parent-pointers-2024].
- `fnd-the-route-row-wants-its-closure` (conductor; ~SUSPECT, unhunted by anyone else; **[HUMAN]**
  asked for the upward check generalised past inodes, on three taste objections: narrow, cruft,
  and not referentially agnostic) — the shape is a grouping structure over a flat identity space,
  where membership is either single-valued with a label readable from the member (a file with one
  name; a process's one cgroup; a controller owner reference; a subnet's one network; a file's one
  owning package) or many-valued with none (tags; groups; label selectors; aliases pointing at a
  record). Model form: a sentence may name a SUBTREE of a `:hierarchical` mScheme (every mKey at
  or beneath R in one mParent-Catalog instance), a rung between a store and an mKey; an mKey
  beneath R reads UNKNOWN against it; an mKey not beneath R reads DISJOINT only where its matched
  shape carries `:guarantees-unique-name` (its one mKey is not beneath R, so it is none of the
  things beneath R), which is that warrant's existing licence quantified over a prefix, drawn
  from one definition's own distinctions; otherwise UNKNOWN. No new relation and no new warrant;
  one new term. Two subtrees separate only where one of them is closed from its own end (nothing
  beneath it has an mKey outside it), the downward walk, paid only for bound against bound. A
  `:hierarchical` primary mScheme needs none of this: its prefixes are mParents by a recursive
  `:identified-in`, expressible today. It is the route row § 14 found missing from § 11's table,
  with an entry (the mTraversal) and a closure at either end. It also gives an honest reading to
  a sentence that leaks today: an mPlacement naming a directory compares as a sibling inode with
  every file beneath it, DISJOINT. Near § 4.2's refuted region predicate, and distinct from it:
  nothing is authored by a sort owner, retargeting stays with mTraversal perishing, and a second
  name withdraws the warrant. Leans on § 14's lean that a warrant may sit on a lexical path
  conditioned on measured state. Conductor's read: it rides the re-cut as that row.
- `ack-a-routing-key-named-whole-is-no-sibling` (**[HUMAN]**, typed ack that the hole is real and
  needs a fix) — an mPlacement naming a directory compared its inode with a file's and read
  DISJOINT, which says only that the file is not the directory. The collide half is folded
  (`5ed426a2`, § 2.8): a routing mKey named whole stands for whatever its mScheme reaches beneath
  it and reads UNKNOWN in § 2.5's test against every mKey that mScheme can yield in that
  mParent-Catalog instance. The buy-back (an mKey not beneath it, under `:guarantees-unique-name`
  for the whole route) is NOT folded. **[HUMAN]** acks toward it: several names kill it; it is a
  carve-out rescuing the commonest case and the general hole must never be papered over; bound
  against bound is the harder problem.
- `rul-warrants-are-path-granular-and-built-at-evaluation` (**[HUMAN]**, typed, "ruled for 311";
  folded `37e87ad6`, § 1.5) — a warrant must be constructable by runtime evaluation, which is
  non-optional for many shapes; it is granular to the control-flow graph of the owner's body, and
  any path the dialect admits may decline it at any point. This promotes § 14's lean.
- CORRECTED (conductor): "a directory has exactly one parent" was stated too flatly. It is true of
  an on-disk tree on Linux (`link(2)` refuses a directory), false of the mounted tree (a bind
  mount gives one directory two paths), and false on HFS+, which hardlinks directories for Time
  Machine [B-eclecticlight-time-machine-filesystems-2024]. It bears on how cheap the measurement
  is and never on soundness: the warrant is given only on a path that measured the whole route
  alias-free, and declines otherwise.
- `fnd-a-hierarchy-is-a-chain-of-lookups-built-in-userspace` (**[HUMAN]** proposal; conductor's
  attack; nothing folded) — "hierarchical" need not mean a string the engine splits. All the engine
  needs is that a lookup sometimes supplies, as the mParent-Catalog of what it yields, another mKey
  of its own mScheme or of a set of mSchemes that feed one another (a path yielding a directory
  entry looked up in a shorter path; a mounted level between two directory levels, each owner
  speaking one level and any level free to withhold its warrant). The mTraversal is then that
  explicit chain, the splitting is `dirname` in the stdlib's body, and § 2.8's reading of mKey
  syntax by the engine can go; perishing and `namespace-composition-is-not-concatenation` are
  served better, since nothing is concatenated or decomposed outside a lookup. Identity stays
  flat (an inode in a device): the routing chain feeds only a sentence naming a routing mKey whole
  (§ 2.8 as folded today) and perishing, never SAME or DISJOINT between two things. What it forces
  into the open (conductor, +SURE of the logic): "no new warrant" was an artefact of packing a
  whole path into one mKey in one catalog. Split into levels, each level owes two sentences: one
  mKey for the thing in this catalog (`:guarantees-unique-name`, existing) and looked up in no
  other catalog, which is the thing's-end closure of § 11's first row, § 9's "unique-name lifted
  one level", left OPEN for 312; here it is measured per mKey (a link count of one) under
  `rul-warrants-are-path-granular-and-built-at-evaluation`, and rhymes as a sentinel
  (`looked-up-in nothing-else`). Rule shape: a thing is outside a routing mKey named whole iff
  every level of its own chain is so closed and every member compares DISJOINT from that mKey;
  inside iff a member is SAME; else UNKNOWN. Only the thing's leg is asked, unlike § 3.2. Priced
  limits: the chain exists only after resolution; a grouping the thing was not named through (a
  process and its cgroup; a file and its owning package) needs an upward read the model has no
  operation for; a bound is stated in one catalog instance, and a daemon that writes from another
  mount namespace is outside it (`daemon-effects-escape-the-traced-process`); a lookup that never
  shortens needs a depth cap.
- Typed this turn (**[HUMAN]**): ack that a stranger cannot make another's `resolve()` yield into
  them; insertion into an existing recursion is punted, as designed. A gut note, not a ruling: the
  hard part of arm composition is correctness, never spelling; it would make it critical that
  soundness stay hyper-local (one link, one step, one speaker; flagged negative existentials
  excepted) and that no author be encouraged to build soundness out of the constructed
  co-properties of several of their own mSorts. Ack that an upward read is owed and enters this
  version of the model; its siting is open (an emission in an existing body is suspected; a new
  entry point is acceptable if more ergonomic). Conditional: the engine reading an mKey's bytes
  should never have been in 311 and dies once the chain form is confirmed better. Sequence
  agreed: settle these small changes, fold, record the whole arc as an exercise under
  `312b-exercises/` (dead ends, dangers, the planned repair), then the adversarial panel, then
  the re-cut on that foundation.
- CORRECTED (conductor): "identity stays flat" was wrong wording. Identity is a chain, and it may
  nest (a zone in a zone; an object in a namespace; a rule in a chain in a table), in which case a
  bound naming a level is a store and § 3.2 already answers. Files are the odd case: their nesting
  sits in the lookup chain while the primary mKey (an inode in a device) does not nest. The
  conductor checked the origin of § 2.8's syntax reading: `30T` § 5 hands the engine PATH syntax
  as POSIX language, for the load plane; § 2.8 extended it to hostnames and unit names, which no
  specification hands the engine. `30T`'s own use is a different consumer and is untouched.
- `fnd-the-upward-read-is-an-ordinary-lookup` (conductor; ~SUSPECT; siting OPEN by the human's
  word) — a secondary mScheme owned by the grouping's describer, keyed on the thing's own primary
  token, yielding that same thing and supplying the grouping as its catalog plus the closure; run
  only where a sentence names a key of that grouping whole and a comparable thing has no chain in
  it; detectable statically because the body yields its own argument. The one new engine step:
  constructing a key of that mScheme from a primary token at comparison time, where keys are
  minted today only at binds and declared emission points.
- `fnd-no-hierarchy-declaration-is-needed` (the human's proposal, checked; ~SUSPECT) — perishing,
  "inside", and "outside" each need only the chain the lookups produced and the per-level
  closures; the engine needs statically only whether an mSort is ever supplied as a catalog by a
  loaded lookup (read off record shapes at lift), and only to read a bare whole-named key. The
  proposed detector corrected: upward reads cannot define a hierarchy (files have none), and
  cycles are not needed (one level supports a bound).
- Vocabulary (**[HUMAN]**, typed 2026-09-23; recorded in the root `_tmp-` naming file, unapplied):
  `may-read` / `may-write`, with the sets Readset / Writeset, replace placement, backing,
  footprint, and the entailment; the test reads "a line is transparent to a fact iff
  Writeset(line) ∩ Readset(fact) = ∅". The alias closure replaces "reference count one"
  (**[HUMAN]** direction): at each level the lookup emits the other names it knows for the thing,
  then `alias nothing-else`; a listed alias is checked like the first name; only the closure is
  the knife; it is the thing's-end closure of § 11's first row.
- `nack-a-primary-key-implies-no-descendants` (**[HUMAN]**; conductor's counterexamples: a zone
  identified in a zone; objects in a namespace; child cgroups; tables in a database; addresses in
  an interface) — the split "a routing name given whole is a region; a primary key is the thing"
  was a filesystem artefact and is dropped. Stands: "item" (the thing's own state) and "children"
  (everything beneath it, by identity chain or access path) are disjoint sets; both exclusive
  directions are inhabited (`chmod D`; a write inside D) with unequal precision (item-only buys
  chmod-class lines; children-only buys almost nothing; DNS entails item from beneath, the zone
  owner's two entries). Conductor's recommendation, UNACKED (the human leans to keeping an
  explicit item/children mark; an asterisk floated): a bare key denotes the wide set, its
  mReferent and everything beneath, which § 3.2 and the folded § 2.8 sentence already do; `@cell`
  denotes one part; fail-safety puts the mark on the narrow claim, so an asterisk on the children
  would leave the dangerous reading bare; "every item-only write is a cell write" ~SUSPECT
  (filesystems, DNS, Kubernetes, cgroups, interfaces checked; databases and git not); cell
  precision on identity-chain containers waits on the sibling-cell residue (312). Under the
  recommendation `-beneath` is unneeded.
- `fnd-a-referent-is-an-aggregate-and-a-may-set` (**[HUMAN]** challenge; conductor's retraction)
  — the stipulation "one concrete mReferent, one value at one instant" was wrong as the term's
  meaning: philosophy of language admits plural, abstract, and absent referents; a weak
  reference's referent is one aggregate object; alias analysis's referents are may-points-to sets
  of summary objects; a Rust or C++ referent is a place, possibly unsized. The model's usage is
  already the aggregate one (§ 1.1: a machine, a mount table; cells as fields) and a key without
  `:guarantees-unique-referent` reaches a may-set (§ 1.5); a bare container key denotes a summary
  object; a shape-carved sub-mSort is a summary over its members. The deep (reachability) reading
  is the conservative summary for a callee the analysis cannot see, hence the bare form; "a cell
  is a part, never a thing reached through" is the sort owner's layout, ~SUSPECT. **[HUMAN]**:
  mReferent keeps its name and is made extremely explicit at the freeze (the naming file carries
  the text owed).
- Pending on the human, none typed: the mirror rule's tuned wording; the outside rule with its
  alias closure; deleting § 2.8's key-syntax sentence; the upward read's siting; the item/children
  mark.
- `fnd-a-cell-fuses-a-field-and-a-named-read` (conductor, 2026-09-23; **[HUMAN]** gentle ack the
  next turn, and the terminology FROZEN: mReferent divided from mField, "even if only defensively,
  to stop people referring to the latter as a referent"; the role of each in the model not yet
  fixed) — § 1.9's "the unit that has a value: what a probe measures and a mutator writes" fuses
  two things. A FIELD: a part of a referent at the grain its store's keys expose, one value at one
  instant, written whole or not at all at that grain, what a Readset or Writeset entry reaches (a
  compiler's memory location). A NAMED READ: an author's question over one or more fields, its
  identity the bearer plus a name (`sm.Service:nginx@active`; the pre-r30 selector), what a
  verdict measures (a compiler's derived value). Two named reads may read one field: `test -x`
  reads the mode field, the owner fields, and the caller's credentials, so a `chmod` declared as
  writing `@mode` alone must collide with an `[ -x ]` guard's fact, and treating the two as two
  fields with two homes elides the guard wrongly. One named read may read many fields
  (`@installed` over the status stanza). The one-to-one case (`@mode`) is the cornerstone example
  that hid the fusion. Consequence, ~SUSPECT and unfolded: a named read's identity answers only
  whether two facts ask one question and never enters the interference test, only its Readset
  does; then `311p` thread 3 (`chmod` past a contents fact: the mode field against the data
  blocks, DISJOINT at the store's grain, the bearer never appearing) and thread 2 (`@active` reads
  a field in the manager's memory, scoped in the boot, and perishes at reboot; `@enabled` reads a
  symlink inode in a filesystem and survives) dissolve at the definition, with no per-cell
  identity store. The named read's word stays open in the naming file.
- `fnd-the-freeze-glossary-by-world` (conductor, 2026-09-23; **[HUMAN]**: "still cohering", no
  ack) — a specification-tier glossary in four worlds (Outside: state a command can observe or
  change, the executing shell's own variables included; Shell: events, bodies running, strings
  bound, records emitted, verdicts returned; Text: authored source; Model: plan-time objects),
  no term in two worlds, the crossings named (Text executes as Shell; Shell reads and writes
  Outside; Shell is ingested into Model; Model names Outside and never holds it). Its
  differences from `notes/311`: terminological (the two uses of "value"; mCell into a stored
  half and a question half; the acked set names; mParent-Store and mParent-Catalog as keys
  naming what prose calls a store; grain; transparent for spared; the four worlds);
  functional, each ~SUSPECT: (1) § 2.5 compares "a footprint mKey and a fact's mCell", a key
  against a question, where the glossary compares Writeset entries against Readset entries
  only, the question never entering, which is the earlier dissolution of `311p` threads 2 and
  3; (2) the observer's referents as Readset entries, where § 2.7 makes the observer a topic
  qualifier only (wider, safe). A third, reserving stored subdivisions to the sort owner, was
  WITHDRAWN under the nack below. Correction to the glossary's own text: Readset inheritance
  runs along the identity chain (§ 2.4), never the access path.
- `fnd-the-standard-split-is-field-against-property` (conductor) — the object languages' field
  (stored member) against property (accessor computed over fields) is disjoint by construction;
  for a compiler engineer "field-sensitive" analysis fixes field as a stored member, and
  "property" in PL means a predicate over states; a property whose name matches a field has it
  as its backing field; false friends: JavaScript's property covers both, Swift says stored and
  computed property, Kotlin's property wraps a field. Offered: mField as the stored member,
  mProperty as the named read.
- **[HUMAN]** NACK (2026-09-23) on any reading of "field" that means the sorted,
  themselves-referents entries in a referent that is a store: a first-class, sorted,
  Dorc-tracked referent is never doubled over by a novel term, whatever tree, chain, or graph
  of sort-inhabitants it sits in, so that no one graph is privileged over the others. The
  human's framing, acked by the conductor as coherent: a referent has a type (an mSort) and
  names (mSchemes); this arc names the things in Outside or Model that are NOT world-values
  under an mSort, so they are never confused with referent, value, or cell. Consequences
  (conductor, unacked): mField dies in both senses (a stored subdivision is an ordinary referent
  of an ordinary sort under its bearer, § 1.9's "one difference of shape"; a site is a may-read
  entry at a lower store, a tracked referent or nothing); mProperty is the only new tag; mCell
  retires whole rather than narrowing to a half; "part", "field", "site", "home", and "beneath"
  as nouns retire; the whole-entry rule restates on relations (a key given whole denotes its
  referent and every referent whose identity chain or access path passes through it, already
  § 3.2 plus the folded § 2.8 sentence); the stranger restriction withdrawn (a stranger mints a
  sort under my bearer like any sort; known-unspoken). Hazard named: `@x` names a referent's
  key in an entry and a property in a mark; a property's Readset is declared, defaults to
  unbounded, and is never inferred from a shared name. The naming file still records mField in
  the site sense, untouched pending the human's word.
- ACKED (**[HUMAN]**, typed 2026-09-23): mReferent, mState, mValue as three terms (the naming
  file carries the definitions: a referent is a persisting thing that has state; a state is its
  condition at an instant, never held by Dorc; a value is bytes a shell holds, what a read
  yields from a state, the standing use of § 1.4 and the value plane; "string" withdrawn);
  every selector is full reverse-DNS like every key (`sm.Unit:nginx@sm.Active`); no sort-local
  name table exists, every sort name being global by design; "bearer" retires.
- `fnd-the-marked-line-is-a-read-and-the-species-dissolves` (conductor; **[HUMAN]** ack) — with
  `scheme:key@sm.Sort` read as a KEY (the singleton of the global sort, identified in the
  referent the left key names; the left scheme never yields into it; the sort declares its own
  `:identified-in`; strangers mint freely; state possibly diffuse; the sort's may-read says
  where it is held), the marked shell line is a read of that key and the fact's identity is
  § 1.8's mTopic, the key read plus the observer instance. No question-species is needed: the
  engine cannot tell a stored singleton from a derived one and need not. Withdrawn on the way:
  mExpression (LCM's available expression was the precise compiler term, killed by the human as
  a second meaning beside sh syntax); mProperty (JavaScript's stored-member reading); mClaim
  (tried on by the human, nacked by the conductor: § 1.11's speech kinds and the spike's
  `Claim<Tier,_>` make a fact a claim by observation, so the word names the output). The
  inhabitant proving a singleton sort needs sort-level machinery while identity-keyed by
  another's referent: `sm.Path:F@sm.Writable` (own may-read: mode and owner; own
  `:observer-independence`; own perishing under `chmod` and none under `truncate`; no key of its
  own). Freeze text owed to § 1.9: a cell is a singleton sort under its parent, keyed
  `parent-key@sort`; § 1.8 stands.
- RETRACTED (conductor, 2026-09-23): the claim two sittings earlier that a question-species
  dissolved `311p` threads 2 and 3. Thread 3 stands one level down: two singletons of two sorts
  under one parent read known-unspoken under § 3.2, and only the parent's owner keying them in
  one scheme with `:guarantees-unique-name` separates them (§ 14's
  `fnd-difference-between-strangers-is-earned-one-way`). Thread 2 resolves by parenting, never
  by a species: `@sm.Enabled` hangs under the unit file in a filesystem and `@sm.Active` under
  the unit in the manager (`312b:lead-identify-in-the-narrowest-primary-store`; § 8's ruling
  that a persisted form and a live form are two referents).
- OPEN: "receiver" for the referent left of `@` (**[HUMAN]** holds it) against the conductor's
  reading that it is the mParent under the confirmed semantics; the human asked why "parent"
  works and whether a second kind of parent hides in `sm.SchemeA:key@sm.SortB`.
- `fnd-one-kind-of-parent` (conductor; **[HUMAN]** "Okay; acked") — sorts never have parents,
  keys do; a sort's `:identified-in` declaration constrains the parent's sort per shape and
  names no instance. `sm.SchemeA:key@sm.SortB` is one link of `:identified-in`, B's own, whose
  instance the left key supplies after resolution through A's yields, so A is irrelevant beyond
  being how the author wrote the referent's name, and a left key resolving to the wrong sort is
  a shape mismatch, refused. The two flavours 311 has (catalog through a secondary scheme,
  store through the primary) are one edge per key, and a singleton's edge is always the
  identity kind. What looks like a second parent, where the state is held, is an entry in the
  sort's may-read set, never a parent (`311t` § 10's minting against holding, and the folded
  "a parent is no placement" read from the other side). Extension, chat only: for a plural sort
  under a parent (`sm.Unit:nginx@sm.DropIn`) the same spelling names no key but the set of that
  sort's referents identified in the parent, a legal entry.
- CORRECTED (conductor, on the human's objection): "a singleton's scheme" was wrong; nothing
  yields into a singleton sort and no author types its key. Its key is minted at the mark as
  (parent instance, sort). 311's letter gives it a degenerate primary scheme (one value, identity
  lookup) so that "every sort has a primary scheme" holds; that is ceremony and may go at the
  freeze, with § 1.3's "a bind or a mark always names an mScheme" clarified: a mark of the form
  `scheme:key@sort` names the parent's scheme and the singleton's sort (~SUSPECT).
- CORRECTED (conductor; the human's wobbliness strawman): "thread 2 resolves by parenting" was
  the weaker route, superseded. A file's wobbliness perishes at every boot however the file was
  named: its parent is the file (any scheme yielding an `sm.File` may sit left of the `@`; the
  sort declares `:identified-in sm.File`), and its freshness is its sort's may-read set, declared
  once by the wobbliness describer, naming a referent scoped in the boot; a reboot's footprint
  names the boot whole, the deep reading collides everything beneath, the fact goes stale, and
  the topic "wobbliness of F" is the same topic afterward with a new state to measure. So
  `@sm.UnitActiveness` (held in the manager's memory, in the boot) and `@sm.UnitEnabledness`
  (held in a symlink, in a filesystem) hang under one unit and a reboot kills one fact and
  spares the other by their may-read sets alone; no choice of parent is forced on the author.
  Identity is minting; freshness is holding; one kind of parent. **[HUMAN]**: "Okay; acked."
- `fnd-the-left-of-at-is-the-parent-instance` (conductor; **[HUMAN]** "Okay", not typed as an
  ack) — the referent left of `@` is the singleton's mParent (its mParent-Store view); the key
  written there is the parent instance as § 1.6 defines it, supplied at the bind seat (the mark)
  as a key of one of the parent's sort's schemes. "Receiver" would name the same thing with the
  lookup connotation the semantics ruled out; "bearer" and mSort-Bearer retire. User-facing: a
  possessive ("nginx's activeness"), no term.
- Wrap-up of the naming (conductor; read by the human, not acked): the `@x` two-namespace hazard
  DISSOLVES once every `@sm.X` is a key (a mark and an entry both name keys); the cell-to-bearer
  relation (`:named-like` / "named-by") is `:identified-in` and its naming row closes;
  remaining open rows are 312 spelling (the closures' names against the "guarantees" form; the
  `kind__` prefix; `:rootness`; the identifying-store word; "individual"); the one semantic item
  still open is item against children (deep bare key with the singleton spelling as the narrow
  form, against an explicit mark); the next step after that is the rename pass over 311.
- `fnd-the-alias-row-has-two-levels-and-two-ends` (conductor; read, not acked) — the address row
  of § 11's table ("T's key means something only relative to P") has an identity level and a
  routing level, each with two ends, each end a positive entry and a closure. Within one store
  (one referent, one key): P's end is `:guarantees-unique-name` per shape (positive twin: the
  lookup yields the canonical key); T's end is `alias nothing-else` per key, measured (positive
  twin: `alias k'` entries). Across stores (one referent, one store): P's end is
  `:aliases-nothing-else` (positive twin: `:corresponds` by the transition owner); T's end is
  "no other home", the OPEN 312 cell, whose routing-level twin is this arc's `looked-up-in
  nothing-else` (positive twin: `:corresponds` from T's side). The law: two keys under one
  parent separate on EITHER end (the per-shape warrant implies the per-key instances, never the
  reverse); two things in two stores separate only with `:aliases-nothing-else` on every store
  of both legs (§ 3.2 as folded), the open thing's-end cell being needed only where one thing
  sits in two stores by design (cgroup-v1 hierarchies; a clustered filesystem under two boots),
  which decline and read UNKNOWN, so the cell may stay empty at the freeze (~SUSPECT; § 9's own
  argument); a thing is outside a region on T's end alone at every routing level of its own
  path, asking nothing of the region's store. Freeze consequences: write the unique-name
  warrant once as one row with two spellings; name `looked-up-in nothing-else` as the routing
  twin of the open identity cell. This resolves the "double spelling" flagged the turn before.
- ACKS (**[HUMAN]**, typed 2026-09-23): mParent for the referent left of `@`, and "the parent
  instance at the bind seat" for the key written there (the seats being § 1.6's three suppliers
  of a parent instance; for a singleton the bind seat is the mark); the minimum of one scheme
  per sort DROPPED (a sort may have no scheme, "exactly one primary" becomes "at most one", and
  no degenerate scheme is carved out for singletons); a bare key in an entry is deep, ruled by
  safe-defaulting and gradual enhancement, its spelling kept out of 311; the engine never reads
  key syntax at the model level (§ 2.8's sentence dies; any such reading would be a transparent
  optimisation, never user-visible). NIT (**[HUMAN]**): the two ends of the alias row are related
  but SEPARATE statements in 311; whether licensing needs both ends in agreement, either end, or
  a lattice (partial presence buys collision; dual presence and coherence buy elision), and
  what "a warrant" is at all, are 312's, after spelling is stable. The conductor's "one warrant,
  two spellings" and "either end suffices" are RETRACTED.
- `fnd-the-mirror-rule-is-not-derivable-from-may-read` (conductor; the human asked why a write
  must write its target's read-items, for a careful justification, and whether any reviewer
  had found it) — the rule as tuned added the may-read entries of the written key's containers,
  strictly below the level shared with the fact's key, to the Writeset. It is not derivable:
  "C may-read B" is influence, B toward C, and the rule needed realization, C's contents kept
  in B. The leak it repaired fires only where the walk already reads DISJOINT across sorts (a
  shared key space with unique-name, loop0 and sda as block devices, both filesystems honestly
  `:aliases-nothing-else`) and a container of the written key is kept in something on the
  fact's side; the uid-against-passwd shape collides anyway as known-unspoken. PROVENANCE:
  found by the conductor (§ 8, held unevaluated), raised by no panel report, in none of
  `311p`'s seven threads, attacked only by its author. Walked slowly on two books: a `cp` into a
  loop mount against a checksum fact on the image (the rule collides, correctly), and a
  `resolvectl query` populating a cache that may-reads `/etc/hosts` against a `cmp` fact on
  that file (the rule collides, wrongly, and the model could not say the cache is not kept in
  the file).
- WITHDRAWN and REPLACED (**[HUMAN]** objection, "a user typed may-read; you are describing
  something that may-writes", the counterexample demanded and given; **[HUMAN]** ack,
  2026-09-23) — the mirror rule as a rule over may-read is gone. The right sentence is the
  container describer's write-side statement, which the model already has as sort-level
  may-write (§ 2.5's entailment; the P's-end cell of § 11's write-path row): a loop filesystem
  declares `may-write <its image>` then `nothing-else`; a resolver cache declares `may-write
  nothing-else`. The two cases then separate, the loop write colliding with the image fact and
  the query sparing the hosts fact. What survives of the mirror is one sentence and one clause:
  a write inside a container is a write to that container, so the container's may-write
  applies; containers at or above the level both keys share do not contribute, since a
  filesystem's may-write names its disk and two files in it would otherwise collide through
  it. The safe default is existing law: a container with no declared may-write has an
  unbounded set and writes inside it collide with everything; a closure omitting the image is
  a false `nothing-else`, attributable. May-read entries are never consulted on the write side.
- Pending on the human, restated: the outside rule with its alias closure; the upward read's
  siting. Resolved this sitting: the mirror rule (replaced, acked); the key-syntax sentence
  (dead); item against children (deep).
- `rul-the-outside-rule-with-the-alias-closure` (conductor's detail; **[HUMAN]** "ack on
  closure", then "I ack", 2026-09-23) — for a region (a key given whole, resolved to a referent
  D) against a fact's key x, whose routing chain the lookups produced level by level, each
  level able to emit `alias nothing-else` (the referent at that level reachable by exactly this
  one entry; the path lookup's body checks a link count of one for a file, and for a directory
  the link count plus no other mount in `mountinfo` exposing it): four outcomes. The leaf
  compares SAME with D: x is D, SAME. Some other level compares SAME with D: the region covers
  x, the chokepoint verdict is UNKNOWN as for a key against its own container, one level
  suffices and no closure is needed (a second path is irrelevant once the path in hand passes
  through D). Every level compares DISJOINT with D and every level is closed: outside,
  DISJOINT. Otherwise unknown, and the folded collide half of § 2.8 applies. Only the fact's
  leg is asked; D needs no closure, since the region is its referent however named; region
  against region stays unknown unless one owner enumerates. Compare works on referents, never
  strings (`containment-by-path-prefix-lies` by construction). The conductor's first wording
  "inside if any level is SAME" was corrected by the human to the four outcomes above. Every
  level, not just the leaf, on two books: a `mount --bind /etc/app` into a docker volume (the
  leaf `app.env` has link count one and compares DISJOINT; only the directory level's
  `mountinfo` check withholds the closure); an LDAP alias entry `ou=engineering` pointing at
  `ou=eng` (alice's own entry is ordinary; only the OU level sees `objectClass: alias`) with a
  client that does not dereference aliases. Not covered: a grouping the thing was not named
  through (the upward read); a daemon writing from another mount namespace than the book's.
  New and additive: the per-level closure record (the thing's-end routing statement, emitted at
  evaluation under `rul-warrants-are-path-granular-and-built-at-evaluation`) and the rule that
  consumes it; their absence changes nothing.
- The upward read, sited (conductor, three sittings; **[HUMAN]** "ack what I've read thus far,
  mostly", 2026-09-23, disliking the spellings). Walked on packages: a drifted `apt-get install`
  upgrading nginx above two converged `cp` lines, one into `/etc/nginx`, one into `/etc/app`;
  the package bound given whole collides with both today, since neither file's chain has a
  package level. Three strawmen, all bodies with `dpkg -S` or `dpkg -L` typed in, differing
  only in role: (A) a second scheme of files that accepts `sm.Path` values, yielding the file and
  looked up in the package, needing one engine permission to run a loaded lookup on a value
  bound under another scheme; (B) the downward list, USER_STORY stage 7's `dpkg -L`, already
  in the model, one host read per package named in a bound; (C) a sort-level body on the
  package sort invoked with a file key's value, emitting a membership record and its closure.
  Costs: controller work negligible; host reads O(distinct files under a bound) for the upward
  read against O(distinct packages) for the downward list, each cheaper in different books
  (`dist-upgrade` against a directory region), both riding the one probe pass. The conductor
  first leaned A (no new record kind; the outside rule consumes a chain unchanged).
  **[HUMAN]** NACK of A's shape, not its meat: schemes stay singular ("one kind of thing that
  is a path"; two `sm.Path`-like schemes in different rDNS namespaces are meant to conflict
  safely, never to cohere); collaboration joins at the SORT, as `sm.Uid` and `sm.Username`
  join at `sm.User`. Conceded: the input's spelling belongs in the body's own matched arms, the
  seat 311 already uses (`net/*` against `kernel/*`), so a path-shaped value is answered and
  an inode number declined with no scheme named; and a file's package is a RELATION between
  two referents, never a spelling of the file, so encoding it as "the file as dpkg sees it" is
  what made a scheme appear. Corrected shape: a sort-level body on G, declared against the
  sort it places (the omitted `places "sm.File"` trailer, which is what puts the pair
  (G, T) in the engine's static table; the human caught its absence), invoked with a T key's
  value, every key the engine holds for the referent tried and disagreement refused. Its record
  is not new: `looked-up-in G:key` plus `looked-up-in nothing-else`, the thing's end of the
  ROUTE row (§ 14 found that row missing) at the across-catalogs level; the store's end is the
  downward list with its closure; the path lookup already speaks the same thing's end for
  directories. One parent per key holds by one clause: the parent is the route the key's own
  lookup supplied; routes another sort's upward body adds are additional routes on the key,
  walked by the outside rule and perished by writes to their catalog, never the parent. What
  the model gains: one declaration kind (a sort naming the sorts it places), one engine
  permission (invoke that sort's upward body with a placed key's value when a bound in that
  sort is in play and the key has no route in it), and no new relation. The two ends stay
  related but separate statements; their licensing law is 312's.
- ACKED (**[HUMAN]**, "mostly", 2026-09-23): the route row as a whole; the one-parent clause;
  the siting. The row: T's referent is reachable through P, a third relation beside identity
  and write reach, with the crossing lookup as its speaker. Thing's end: `looked-up-in P:key`
  from whichever lookup crossed P, with a closure scoped per route-sort (the path lookup's
  closure says nothing about packages). Store's end: the downward list and its closure. Two
  levels, within one catalog and across catalogs. Consumers: the outside rule (covered if any
  route of the bound's sort reaches the bound; outside if all are DISJOINT and that sort's
  closure is present) and perishing (a write to P's catalog perishes the route; `apt-get
  install` writing dpkg's database perishes every package route). The one-parent clause: the
  parent is the route the key's own lookup supplied, the link `compare()` walks and the seat
  supplying parent instances to singletons; routes another sort's upward body adds are walked
  by the outside rule and perished, never parents. Entries by role, names 312's: the route
  record and its closure (exist); a sort's declaration that it places another sort (owed; it
  is what puts the pair in the engine's table); the upward body on the placing sort, invoked
  with a placed key's value, every key held for the referent tried, disagreement refused
  (owed); the downward members body (today only as write reach, stage 7's `dpkg -L`; as
  membership owed or reused); one engine sentence. The human's characterisation, typed: with
  this ruled, every sort walls everything around it until it is mapped into the filesystem,
  spiritually, allowing for the few roots, which closes longstanding holes. Restated: cross-sort
  sparing exists only where the walk answers DISJOINT, which needs both sides in a shared key
  space; the stdlib's key spaces are those spaces, and a sort hung on nothing comparable walls
  (§ 14's hard ack). Default posture confirmed: a bound in sort G against a fact of sort T
  collides unless G's describer translates the bound into a comparable key space by the
  write-path row (what writing it writes; stage 5's `apt-get update` naming the list files)
  or the route row (membership); the upward body's absence is exactly that floor, its
  positive records add only collisions, and its closure alone spares, the attributable knife.
  Next, by the human: review under a fresh panel; dispatch still needs the typed ack.
