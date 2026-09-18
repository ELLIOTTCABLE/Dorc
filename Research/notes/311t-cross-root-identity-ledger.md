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
  different catalogs (a public resolver against libc plus nscd, GOTCHA 48); one placeholder ⇒
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
- `kill-emulator-presents-real-id` — GOTCHA 59. localstack answers `sts get-caller-identity` with
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
  somewhere). Status: horizon at parity with GOTCHA 53 absent that oracle; with it, a wall.
- `kill-reissued-address-to-a-stranger` — cloud addresses are reissued to other tenants within
  minutes of a delete; a name still pointing at the old address reaches a stranger; ssh's host-key
  check dies (GOTCHA 43, barely). Kills: any unique-referent on address-shaped keys. Catches:
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
