# 312b-exercises/03 — Store identity across hosts

> Exercise record (Fable, 2026-09-08). Not durable; takeaways live in `notes/312b`. Method: hold
> `notes/311` as the final types, push one concrete habit onto it, card only where it strains.
> Plain book-sh grounds the case; the model's response is prose in 311 vocabulary; no oracle
> spellings. Grades: +SURE / ~SUSPECT / -GUESS / --WONDER.

## The habit

```sh
ssh alpha 'acct --db /srv/people/accounts.db --directory red enable 7'
ssh beta  'acct --db /mnt/team/people.db     --directory red disable 7'
ssh alpha 'acct --db /srv/people/accounts.db --directory red enable 7'
```

`/srv/people` on alpha and `/mnt/team` on beta are one NFS export (GOTCHAS
`a-host-is-not-a-partition`). Line two really runs. Line three must run, because line two
disabled the same row. Two neighbours: a unix socket on the same shared mount
(`a-socket-on-a-shared-mount-is-not-shared`), and the ssh host key as identity
(`a-host-key-identifies-an-endpoint-not-a-machine`).

## First glance

The acct author, describing accounts, identifies them in Host: accounts are per machine. Alpha
and beta measure as different machines; line two's footprint at beta is disjoint from line
three's backing at alpha; under the flag line three's elision survives; the account stays
disabled. Cardinal sin, and it looks reasonable from the acct author's chair.

## Where the model holds, and the one law it wants

`311` § 2.2: an mSort's store is at most one, declared by its owner. The acct author
knows their database file; they do not know NFS. If they identify accounts in the database File
(SITE-supplied from `--db`), the chain runs outward through mSorts whose owners do know: File
:identified-in Filesystem by the File owner's `resolve()`; Filesystem :identified-in a local device or
an NFS export by the filesystem owner, per filesystem type; the export :identified-in a server
the client resolves from its own mVantage. At that step the chain meets a hostname resolved from
beta's mVantage, and `resolution-is-set-valued` with `a-name-resolves-from-a-vantage` make the
link unknown. `compare()` answers unknown; the footprint collides with the backing; line three
guards; the guard re-checks at apply and runs. Safe, coarse, correct.

The strain is not in the model; it is in what the acct author was tempted to write. The model
needs a law it does not yet state: identify in the narrowest store your state
actually lives in, never a coarser one you assume partitions it. Identifying a file-backed mSort
in Host is a deployment claim the tool author cannot hold, and it is the highest-leverage wrong
DISJOINT in the design. Structurally routing the claim outward is the whole
push-half-the-work-to-the-neighbour-who-knows move, made mechanical.

## Where SAME comes from, if wanted

For this book nothing needs SAME; unknown is enough. Where SAME is wanted (a fact at alpha
standing for the mCell at beta), the chain has one seat that can close it: the mount line.
`mount alpha:/srv/people /mnt/team` is a transition the admin wrote in plain sh, and the mount
oracle's owner can derive from its argv a mCorrespondence between mKeys under `/mnt/team` at beta
and mKeys under `/srv/people` on the server named `alpha` from beta's mVantage (`311` § 2.5, the
transition owner's generator). That last clause is host sameness, and
`26M:ack-authored-host-sameness-parallel` already gives it to the admin. Three authors compose:
the File owner (scoping), the mount owner (mCorrespondence from argv), the admin (that this
`alpha` is that `alpha`). Each speaks only about their own thing.

## The neighbours

The socket: a File mNaturalKey reaching a kernel object. If the docker author lazily uses a
File coordinate for the socket path, the File owner's correct warrants make it SAME across the
shared mount, and a fact about the daemon transports to a host where the connect fails. The
model's answer is § 2.7: the socket is an mSort :named-like File, :identified-in the kernel. The net
for lazy borrowing is that the File owner's `resolve()` declines on mReferents outside File's ontology
(sockets, FIFOs, devices), exactly as `30T`'s binder already declines; then a borrowed File
coordinate on a socket path reads unknown, not SAME.

The host key: a mToken with no warrants. Clones share it; a serial line lacks it. It feeds the
standup `witness()` and nothing else; `26Ob:nack-disk-stamp-is-e-stop-only` is the same shape. The
model already has no place for it to license anything, which is right.

## The card (the acct author's temptation)

Unknowability class: by-design for the engine (it holds no notion of host identity); vacant
for the chain (each link has an owner); the last link, host sameness, is the admin's by typed
ruling.

Can know at all: the acct author (the store is the `--db` file); the File owner (inode in
filesystem); the filesystem owner (per-type identity: UUID, server and export); the admin (that
the export's server is the ssh target); the host (each `resolve()`).

Can reasonably know: exactly the above, one link each. The acct author must not be asked about
NFS; the filesystem owner must not be asked about accounts.

Danger. The wrong DISJOINT (identify in Host) is cardinal, flag-gated, attributed to the acct
author, and reaches every book naming the mSort. The narrowness law makes the wrong claim
unnatural to write and lint-able. A wrong mCorrespondence from a mount line is a wrong SAME,
attributed to the mount owner's mDerivation or to the admin's host-sameness line; the remedy
forks two ways, which `30V:rul-remedies-may-fork` already allows.

Verification. Every link but the last is a `resolve()`. The last is the admin's word, and the standup
`witness()` can catch a changed target but not a wrong pairing.

Residue if nobody speaks. Unknown at the mVantage step; guard; correct for this book. Attention
cost only.

## Leads and wants

- `lead-identify-in-the-narrowest-store`: a naming law for stores; a file-backed
  mSort identified in Host is a smell the lint names.
- `lead-mount-lines-generate-correspondences`: the transition owner for a mount is the mount
  oracle; the mCorrespondence derives from the argv the admin wrote.
- `lead-cross-host-same-bottoms-out-in-the-admin`: every cross-host SAME chain ends at host
  sameness, which is the admin's seat by typed ruling; the model needs that seat to exist.
- `lead-reads-decline-outside-their-ontology`: an mSort owner's `resolve()` declines on mReferents its
  mSort does not describe; the mechanical net for lazy coordinate borrowing.
- `lead-warrantless-tokens-are-witness-only`: endpoint witnesses (host keys, stamps) never
  license; already true, worth stating.

## Open

- What the filesystem owner's store for an NFS export is, concretely, and whether
  any server-side mToken exists that survives the mVantage problem (--WONDER; NFSv4's server
  scope in EXCHANGE_ID, unverified).
- Whether "narrowest store" is decidable from an mSort's own declarations (mPlacement set versus
  store) so the lint is mechanical rather than a taught rule.
