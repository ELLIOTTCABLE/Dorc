# Companion note: `k8s-initcontainer-pv-seed.sh` (the inside seat)

> Tier: LLM-authored round note, r26 ops-glue-residue writing phase. The book and manifest
> beside this note are FROZEN EVIDENCE — imagination-tier, not runnable, never to be
> executed or applied. Renders are illustrative; render form is explicitly unwelded.

Domain: Kubernetes, from the inside. Posture unchanged and worth restating before anything
else: **Dorc versus Kubernetes is not a choice, and this book does not make it one.** k8s
owns the workload — scheduling, restarts, health, rollout, the whole reconciliation loop —
and it owns them better than anything Dorc will ever do. What it does not own, by explicit
design, is the *content of the shell script you put inside an init container*. It tells you
that script must be idempotent and then hands you nothing to make it so. That sentence is
the entire seat.

---

## 1. The seat, verified against the adjudication discriminator

The round's test for a real dorc-inside seat has two conjuncts: (a) the wrapped script
re-runs against mutable state, and (b) the wrapper does not itself provide the
check/idempotence/attention machinery for that seat. This seat passes both, in Kubernetes'
own words rather than ours.

**(a) It re-runs, and the docs enumerate when.** *"Because init containers can be
restarted, retried, or re-executed, init container code should be idempotent. In
particular, code that writes into any `emptyDir` volume should be prepared for the
possibility that an output file already exists."* And: *"If the Pod restarts, or is
restarted, all init containers must execute again."* The state is mutable and it persists —
a PersistentVolume outlives every pod that mounts it. This is the exact cell turn C marked
ALIVE: *an init container that WRITES to a PV*.

**(b) Kubernetes refuses to help, at validation.** *"Init containers have all of the fields
of an app container. However, Kubernetes prohibits `readinessProbe` from being used because
init containers cannot define readiness distinct from completion. This is enforced during
validation."* Nor `lifecycle`, `livenessProbe`, or `startupProbe` — the API reference is
blunt: *"Init containers may not have Lifecycle actions, Readiness probes, Liveness probes,
or Startup probes."* There is no dry-run, no `--check`, no changed-detection, no
convergence verb of any kind. **The seat's entire convergence signal is the script's exit
code**, and the script is the only thing that can compute it.

So the escape-hatch framing holds exactly: Kubernetes hands you raw `sh`, tells you to make
it idempotent, and disclaims. Same shape as chezmoi's run-scripts and Ansible's `shell`
tasks — this is the sixth member of the mandate-idempotence-assist-nothing family, and the
only one where the refusal is enforced by an admission-time validation error.

Worth naming the counterweight in the same breath: **the seat is real but narrow.** The
median init container is `until nslookup mydb; do sleep 2; done`, and Dorc adds nothing to
that. The live cell is specifically the one that writes to persistent storage.

---

## 2. The renders

There is no `dorc plan` here. This is headless: no TTY, no consent moment, nobody watching.
`dorc-run` probes, folds, guards, runs — in one shot — and passes stdout, stderr, and the
exit code through byte-for-byte, so the kubelet sees exactly the script it ran. What it
leaves behind is a why-log.

### A. First pod start — empty volume

`kubectl logs deploy/tileserv -c seed-tiles` shows the script's own bytes and nothing else:
the `sha256sum -c` OK lines and whatever `tileserv-migrate` prints. Dorc says nothing at
the user's stream. The why-log is where it talks:

```
$ kubectl exec deploy/tileserv -c tileserv -- cat /var/log/dorc/seed.whylog
dorc-run seed-tiles.sh   2026-07-28T09:14:02Z   headless, no consent moment
 35  find "$DATA" ... '.stage.*'   ... -exec rm -rf {} +   # runs: unmodeled ('find')
 36  find "$DATA" ... '.seeding-*' ... -exec rmdir {} +    # runs: unmodeled ('find')
 41  if [ ! -d "$DATA/v/$WANT" ]                           # runs: measured absent
 42     if mkdir "$lock" 2>/dev/null                       # runs: acquired
 44     curl -fsS -o "$stage/tiles.tar.gz" ...             # runs: unmodeled ('curl')
 45     curl -fsS -o "$stage/SHA256SUMS" ...               # runs: unmodeled ('curl')
 46     ( cd "$stage" && sha256sum -c SHA256SUMS )         # runs
 47     mkdir -p "$stage/payload" "$DATA/v"                # runs: diverged (absent)
 48     tar -C "$stage/payload" -xzf ...                   # runs: unmodeled ('tar')
 55     mv "$stage/payload" "$DATA/v/$WANT"                # runs
 81  if [ "$(readlink "$DATA/current")" != "v/$WANT" ]     # runs: measured "" (no pointer)
 83     ln -s "v/$WANT" "$DATA/.next"                      # runs: diverged
 84     mv "$DATA/.next" "$DATA/current"                   # runs
 88  tileserv-migrate --data ... --to-layout 3             # runs: diverged
                                                           #   sm.tiles.Store:/var/lib/tiles/current@layout
applied: 14 sites; 0 elided, 0 omitted; 41.2s
```

Unimpressive on purpose. Day zero of anything is everything running.

### B. Restart storm — the node drained, the pod came back, the volume did not move

```
 35  find ... '.stage.*'   ...                             # runs: nothing to reclaim
 36  find ... '.seeding-*' ...                             # runs: nothing to reclaim
 41  # if [ ! -d "$DATA/v/$WANT" ]; then ... fi            # omitted: branch dead - 10 sites
 41                                                        #   measured: v/2026-06 present
 81  # if [ "$(readlink "$DATA/current")" != "v/$WANT" ]   # omitted: branch dead - 3 sites
 81                                                        #   measured: "v/2026-06"
 88  # tileserv-migrate --data ... --to-layout 3           # converged: layout already 3
applied: 2 sites; 1 elided, 13 omitted; 0.09s
```

`kubectl logs deploy/tileserv -c seed-tiles` is empty. Exit 0. Ninety milliseconds.

**Be honest about this render: a well-written hand-rolled script does the same thing.**
Every guard in this book is one the author wrote; Dorc did not invent them and does not
claim to. The delta on *this* day is the why-log — an account of which guard answered and
what it measured — and nothing else. Anyone selling this render as the value proposition is
selling the wrong thing.

### C. Partial seed — SIGKILL mid-fetch during a version bump

The pod was killed while pulling 2026-06 over an existing 2026-05. The volume now holds a
stale pointer, a half-downloaded staging tree, and a held lock. Nothing in Kubernetes
cleans any of that up. The re-run:

```
 35  find ... '.stage.*' ...                               # runs: reclaimed 1 tree (2.1 GiB)
 36  find ... '.seeding-*' ...                             # runs: reclaimed 1 lock
 41  if [ ! -d "$DATA/v/$WANT" ]                           # runs: measured absent
 ...  [ the fetch block runs again, in full ]
 81  if [ "$(readlink "$DATA/current")" != "v/$WANT" ]     # runs: measured "v/2026-05"
 88  tileserv-migrate --data ... --to-layout 3             # runs: diverged
applied: 14 sites; 0 elided, 0 omitted; 43.8s
```

The load-bearing property is not in the render: **the app container never saw partial
data.** `current` pointed at a complete 2026-05 tree the entire time, because the only way
a payload directory comes into existence is a rename of a fully-verified staging tree. That
invariant is the author's, expressed in ordinary sh, and it is the thing that makes the
naive-looking `[ ! -d "$DATA/v/$WANT" ]` guard actually sound.

And the near-miss variant, which is why the two guards are separate questions at all —
killed *between* the payload rename and the pointer flip:

```
 41  # if [ ! -d "$DATA/v/$WANT" ]; then ... fi            # omitted: branch dead - 10 sites
 81  if [ "$(readlink "$DATA/current")" != "v/$WANT" ]     # runs: measured "v/2026-05"
 83     ln -s "v/$WANT" "$DATA/.next"                      # runs
 84     mv "$DATA/.next" "$DATA/current"                   # runs
 88  # tileserv-migrate ...                                # converged: layout already 3
applied: 4 sites; 1 elided, 10 omitted; 0.11s
```

A 40 GiB download not repeated, because the two questions were asked separately. That is
ordinary defensive scripting; Dorc's contribution is that the fold is *reported* rather
than silent.

### D. Failure — a corrupted mirror

`sha256sum -c` fails. Under `set -eu` the script dies; the init container exits non-zero;
the kubelet restarts it per the Pod `restartPolicy` (an `Always` Pod gives its init
containers effective `OnFailure`), backing off 10s, 20s, 40s, capped at 300s, and the pod
shows `Init:Error` then `Init:CrashLoopBackOff`. `kubectl logs --previous` is the only
forensics anyone gets, so that is where Dorc speaks — on stderr, only on failure:

```
$ kubectl logs deploy/tileserv -c seed-tiles --previous
sha256sum: WARNING: 1 of 412 computed checksums did NOT match
dorc: seed-tiles.sh:46 failed (rc 1); 3 sites had run, 0 elided.
      the guard above it -- seed-tiles.sh:41, `[ ! -d "$DATA/v/$WANT" ]` --
      measured v/2026-06 ABSENT, which is why the fetch was entered.
      /var/lib/tiles/current is unchanged and still points at v/2026-05;
      no site downstream of :46 ran.
```

That last sentence is the one an on-call engineer needs at 03:00 and the one nothing in
this seat currently produces. Note the shape: silence on success, narration on failure —
Dorc's own doctrine that deciding and explaining fail in opposite directions, landing
exactly right in a seat where the only channel is a log nobody reads unless something broke.

---

## 3. The sharp finding: this seat collapses `kHALVES`

`kHALVES` is welded far toward the elide-half because *attention is the product* — a guard
makes the book fast and safe but not shorter, and shorter is what Dorc is for. Inside an
init container, run the exclusion-check in the other direction and the tension evaporates:

- **Attention:** there is no reader. No plan is presented, no consent is given, nobody
  looks at anything until something fails. An elided line and a guarded line are equally
  invisible, so the elide-half's entire premium is unrealizable.
- **Wall-clock:** the probe and the apply run on the same box, back to back, in the same
  process. There is no controller, no round-trip, no batching. Eliding pays its check at
  probe time; guarding pays it inline moments later. The difference is microseconds.
- **Mutation risk:** identical. `( check ) || cmd` does not run `cmd` when the check holds.
- **Safety:** the guard-half is *better*. It fails toward run, and it re-checks in-sequence
  where nothing can be stale.

**Therefore the survival tier buys nothing here.** `--risk-faultless-skips`, footprints,
`disturbance_reaches_only`, the whole stage-5-through-7 apparatus — every bit of it exists
to keep *attention* on drifted days, and there is no attention to keep. The one place in
the design that ships naked human trust is unnecessary in this seat, which makes the inside
seat the design's **safest cell**, not its most compromised one. I did not expect that
going in and I think it is the most useful thing in this book.

Concrete consequence worth ruling on: a `dorc-run` invocation with no TTY could refuse the
survival tier outright, and lose nothing.

---

## 4. Why this shape

### Baked into the image, not a ConfigMap

I picked baked. The reasons, in the order they actually decide it:

1. **The runtime dependency does not go away.** A `#!/usr/bin/env dorc-run` shebang needs
   `dorc-run` on the image regardless. The ConfigMap route's headline benefit — "change the
   script without rebuilding the image" — evaporates the moment the interpreter has to be
   baked anyway.
2. **A migration belongs to the app version it migrates to.** `--to-layout 3` and the
   tileserv 1.9.3 that reads layout 3 must move together or not at all. Making the script
   independently editable is not a feature here; it is a way to deploy a schema change
   without a rollout.
3. **The ConfigMap route has real friction.** `defaultMode` must be set for executability
   (`0755`), and the field carries an octal/decimal trap — *"YAML accepts both octal and
   decimal values, JSON requires decimal values for mode bits"* — so a manifest
   round-tripped through a JSON-flavoured tool can silently change the mode. If mounted via
   `subPath` (the usual way to land one file in an existing directory) it *never updates*:
   *"A container using a ConfigMap as a `subPath` volume mount will not receive updates when
   the ConfigMap changes."* And there is a 1 MiB ceiling.

The ConfigMap route is right for exactly one case: an operator that generates the script per
tenant. That is a different book.

### The why-log lands on an emptyDir, and this seat has no `dorc why --last`

emptyDir, mounted read-only into the app container so the log is reachable by `kubectl
exec` or a debug pod, and so nothing of ours lands on the user's data volume. It survives
container crashes (*"The data in an `emptyDir` volume is safe across container crashes"*)
and dies with the pod (*"When a Pod is removed from a node for any reason, the data in the
`emptyDir` is deleted permanently"*).

That is the honest limitation and it should be stated in the synthesis note: **the
retrieval half of the recovery story does not survive this seat.** USER_STORY promises "ask
tomorrow; ask next week" — a receipt you can interrogate after everything is apparently
done. In an init container the container is gone, the pod may be gone, and `dorc why` has
nowhere to run. What survives is whatever went to stderr and got captured by the cluster's
log pipeline. So the failure-time stderr narration in render D is not a nice-to-have here;
it is the *only* durable channel, and the design should treat it that way rather than as an
exception to byte-transparency.

The alternative — why-log to a `.dorc/` directory on the PV, which survives everything — I
rejected for the manifest but it is defensible, and for a book whose whole job is writing
to that PV anyway, the "no residue on the user's volume" objection is weaker than usual.
Worth a ruling.

### `dorc compile` may be the better face here, and my brief pinned the other one

My brief specified `dorc-run` with dorc in the image, so that is the primary. But the
compile face fits this seat better on three counts, and §3 explains why it costs nothing:

- **No Dorc in the image.** A guards-only artifact is plain POSIX sh. For a shop that
  security-reviews base images and counts CVEs, "one more static binary in every init
  container across the estate" is a real conversation and "a shell script" is not.
- **The artifact is reviewable in the PR**, next to the manifest, in the one place a
  Kubernetes shop actually spends attention. This is where the attention product
  *relocates* to — author-time, not apply-time. Honestly bounded, though: compile time does
  not know the live world, so what you can show at review is the *shape* (which sites are
  guarded, by which oracle, on whose vouch), not a world-relative plan. That is a lint and
  audit product, not the USER_STORY plan.
- **The no-fork rule holds trivially.** Chef-solo died of a two-code-path split; the rule
  taken from it is that offline/compile mode may narrow but must never fork semantics. Here
  the compiled artifact is the same book with every site guarded and none elided — a strict
  narrowing, and per §3 a narrowing with no cost.

---

## 5. Real vs. invented

| In the book / manifest | Status |
| --- | --- |
| The idempotence mandate, the `readinessProbe` validation refusal, `Init:N/M` / `Init:Error` / `Init:CrashLoopBackOff`, restart backoff 10s→300s | REAL, quoted in §7 |
| `restartPolicy: Always` ⇒ init containers use effective `OnFailure` | REAL |
| ReadWriteOnce semantics incl. "multiple pods ... on the same node", and access modes not being a write lock | REAL |
| `strategy: Recreate`, `fsGroup`, `fsGroupChangePolicy: OnRootMismatch`, `runAsUser` | REAL |
| effective-init-request max-not-sum, reserved for the Pod's life | REAL |
| emptyDir shared across init and app containers; lifetime | REAL |
| `kubectl logs <pod> -c <init> --previous` | REAL |
| ConfigMap `defaultMode` octal/decimal trap, `subPath` no-update, 1 MiB cap | REAL (used to justify *not* using it) |
| Shebang honoured because the kernel executes the ENTRYPOINT | REAL mechanism |
| Every guard in the script — sweep, lock, atomic rename, pointer flip, bounded wait | REAL, ordinary POSIX; busybox-safe for an Alpine image |
| --- | --- |
| `#!/usr/bin/env dorc-run`, `dorc compile` | INVENTED (sanctioned palette; design prose only — no binary exists) |
| `DORC_WHYLOG` as the why-log sink | INVENTED (sanctioned lane; the env name is strawman, rename-in-place) |
| `tileserv_migrate__is_converged`, `sm.tiles.Store`, the `@layout` selector, the binds/marks | INVENTED (existing ruled dialect; `sm.` namespace deliberately unreal) |
| Why-log and stderr renders | INVENTED (render form unwelded) |
| `tileserv` / `tileserv-migrate` | INVENTED, standing in for any app with a versioned data directory |

Nothing outside the palette. No escalation from this book — the one thing I wanted and
could not have (a durable `dorc why` in this seat) is a *limitation to state*, not a
feature to invent, and §4 states it.

---

## 6. Chafe-points

- **`chafe-no-durable-why-in-ephemeral-seats`** — §4. The recovery story's retrieval half
  assumes something is still around to ask. Init containers, Jobs, and CI steps are all
  ephemeral seats; the design currently has one answer (the why-log durable) and it does
  not survive them. The stderr-on-failure lane is the real answer and should be promoted
  from exception to design.
- **`chafe-migration-cell-is-request-relative`** — `sm.tiles.Store:<dir>@layout` is a cell
  whose truth is "the layout is at the version *this invocation asked for*". That is
  request-relative, like USER_STORY's `@synced`, and it works — but a cell ought to name
  world-state. The alternative (`@layout-3`, a selector carrying the version) needs a
  variable inside a selector, and per-position charsets for entity/selector are
  human-deferred. Flagged, not resolved.
- **`chafe-no-disturbs-arm-and-that-is-correct`** — I deliberately did not write
  `tileserv_migrate__disturbs()`. Nothing runs after line 88 in this book, so a footprint
  arm would buy exactly zero here. It belongs in the *published* oracle, for other people's
  books. Noting it because the temptation to write one "for completeness" is exactly the
  kind of ceremony that makes a gradual-enhancement tool feel like a declarative one.
- **`chafe-find-with-side-effects-is-a-bad-oracle-target`** — the two sweep lines run on
  every invocation because `find -exec rm` is a mutation whose convergence ("no stale trees
  older than 30 minutes") is a predicate over a directory listing. It is *modelable*, but
  the oracle would be re-implementing find's predicate language — the same
  embedded-sublanguage problem the node book hit with jsonpath, in a different tool. The
  faithful-delegation escape does not work here, because the command mutates. **Read-only
  tools with embedded query languages have an out; mutating ones with embedded query
  languages do not.** I think that is a real and previously unnamed boundary.
- **`chafe-guard-soundness-rests-on-an-unstated-invariant`** — `[ ! -d "$DATA/v/$WANT" ]`
  is a sound convergence check *only because* nothing else can create that directory
  half-formed. That invariant lives in a comment. Dorc lifts the guard and reports it, but
  it cannot know why the guard is trustworthy; it is trusting the author's judgment about
  their own filesystem discipline, which is `rul-vouch-is-verdict-authoring` operating on a
  hand-written book guard rather than an oracle. Worth noticing that the admin's own guards
  carry the same trust weight as an oracle's vouch, with none of the attribution machinery.

---

## 6b. Honest ledger

- **Spent:** one shebang line, and `dorc-run` in the init image (or nothing at all, on the
  compile face — §4). Two oracle arms, written once by whoever ships `tileserv-migrate`,
  amortised across every book that seeds this data. No manifest changes beyond one env var.
  No controller, no operator, no CRD, no cluster-side anything.
- **Gained, restart-storm day (the common one):** honestly, **near-zero over a
  well-written hand-rolled script.** Both do nothing. The delta is the why-log. Render B
  says this out loud and the synthesis note should too — this is the render that looks
  most impressive and means least.
- **Gained, partial-seed day:** the guards were separate questions, so a kill between the
  payload landing and the pointer flip costs one symlink instead of a 40 GiB re-download.
  Again: an attentive author writes that themselves. Dorc's contribution is that the fold
  is *reported* instead of silent.
- **Gained, failure day (render D):** the whole purchase. "Site 46 failed; the guard above
  it measured X, which is why the fetch was entered; nothing downstream ran; `current` is
  unchanged." Nothing in this seat produces that today, and the seat's only forensics is a
  log you read after something broke.
- **Gained, structurally:** the idempotence Kubernetes mandates becomes *machinery* rather
  than author-discipline — the same sentence USER_STORY's chezmoi story earns, from the
  sixth ecosystem to make the same demand and offer the same nothing.
- **Gained at review time:** on the compile face, a rendered guard-shape in the PR, in the
  one place a Kubernetes shop actually spends attention.
- **Not gained: the attention product.** There is no reader, no plan, no consent moment.
  Dorc's third priority is structurally absent from this seat. What is left is correctness
  and the report lane, and pretending otherwise would be the single easiest way to oversell
  dorc-inside.
- **Not gained: a durable `dorc why`.** The container is gone; §4.
- **Not gained: anything for the median init container.** `until nslookup mydb; do sleep 2;
  done` needs none of this. The live cell is specifically the one that writes to persistent
  storage, and turn C's caveat that these cells' content is typically small stands.
- **Not gained, and correctly so:** anything about the workload. Restart policy, backoff,
  scheduling, readiness, rollout — Kubernetes owns all of it and owns it well. This book
  touches the inside of one container's script and nothing else.

---

## 7. Citations

All accessed **2026-07-28**; kubernetes.io serving **v1.36**.

- Init containers: <https://kubernetes.io/docs/concepts/workloads/pods/init-containers/> —
  the idempotence mandate; *"If the Pod restarts, or is restarted, all init containers must
  execute again"*; *"Regular init containers (in other words: excluding sidecar containers)
  do not support the `lifecycle`, `livenessProbe`, `readinessProbe`, or `startupProbe`
  fields"*; *"Kubernetes prohibits `readinessProbe` from being used because init containers
  cannot define readiness distinct from completion. This is enforced during validation"*;
  the effective-init-request rule; *"if the Pod `restartPolicy` is set to Always, the init
  containers use `restartPolicy` OnFailure"*; the "Pod restart reasons" list.
  **Doc bug noted:** that list gives "init container completion record lost to garbage
  collection" (combined with all-containers-terminated under `Always`) as a restart
  *trigger*, and the very next paragraph says the Pod *will not* be restarted when the
  completion record is lost to garbage collection. Read as: GC-loss alone is not a restart;
  GC-loss plus all-containers-terminated is.
  **Also:** the page's own "wait for a Service" example is
  `for i in {1..100}; do sleep 1; if nslookup myservice; then exit 0; fi; done; exit 1`
  under `sh -c` in a `busybox:1.28` image. Brace ranges are a bashism; under ash the word
  is a literal and **the loop runs exactly once**. Kubernetes' own documentation ships the
  bounded-retry bug that turn C found in the k3s installer. A shape-recognizer for this is
  pure `kWARN-rich` payoff and would fire on the canonical example in the canonical doc.
  (Read, reasoned about, and deliberately never executed.)
- API reference: <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/> —
  *"Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or
  Startup probes."*
- Sidecar containers: <https://kubernetes.io/docs/concepts/workloads/pods/sidecar-containers/> —
  the contrast that makes the refusal deliberate rather than incidental: an init container
  with `restartPolicy: Always` is a sidecar and *does* support the probes; *"If a
  `readinessProbe` is specified for this init container, its result will be used to
  determine the `ready` state of the Pod."* So Kubernetes knows how to attach a readiness
  predicate to an init-position container — it declines to do so for run-to-completion ones
  specifically, because completion *is* the predicate. The hole is by design.
- Pod lifecycle: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/> —
  *"the kubelet restarts them with an exponential backoff delay (10s, 20s, 40s, …), that is
  capped at 300 seconds (5 minutes)"*; *"For init containers that exit with an error, the
  kubelet restarts the init container if the Pod level `restartPolicy` is either `OnFailure`
  or `Always`."*
- Debug init containers: <https://kubernetes.io/docs/tasks/debug/debug-application/debug-init-containers/> —
  `Init:N/M`, `Init:Error`, `Init:CrashLoopBackOff`; `kubectl logs <pod-name> -c <init-container>`.
- Persistent volumes: <https://kubernetes.io/docs/concepts/storage/persistent-volumes/> —
  *"`ReadWriteOnce`: the volume can be mounted as read-write by a single node. ReadWriteOnce
  access mode still can allow multiple pods to access (read from or write to) that volume
  when the pods are running on the same node."* And the one that justifies the lock:
  *"Volume access modes do not enforce write protection once the storage has been mounted."*
- Volumes: <https://kubernetes.io/docs/concepts/storage/volumes/> — emptyDir sharing and
  lifetime; *"A container crashing does not remove a Pod from a node. The data in an
  `emptyDir` volume is safe across container crashes."* Also: **`gitRepo` was removed in
  1.36**, and the docs' recommended replacement is *this book's own pattern* — *"To
  provision a Pod that has a Git repository mounted, you can mount an `emptyDir` volume
  into an init container that clones the repo using Git, then mount the EmptyDir into the
  Pod's container."* Kubernetes deleted a declarative feature and told people to write a
  shell script instead. That is the residue thesis, shipped by the incumbent.
- ConfigMap: <https://kubernetes.io/docs/concepts/configuration/configmap/> — 1 MiB cap;
  subPath no-update; immutability. `defaultMode` octal/decimal wording from the pod-v1 API
  reference above.
- Security context: <https://kubernetes.io/docs/tasks/configure-pod-container/security-context/> —
  *"By default, Kubernetes recursively changes ownership and permissions for the contents of
  each volume to match the `fsGroup` ... For large volumes, checking and changing ownership
  and permissions can take a lot of time, slowing Pod startup"*; `fsGroupChangePolicy:
  OnRootMismatch`.
- Jobs: <https://kubernetes.io/docs/concepts/workloads/controllers/job/> — not used in this
  book, but the adjacent seat, and its docs say the same thing louder: *"your application
  needs to handle the case when it is restarted in a new pod. In particular, it needs to
  handle temporary files, locks, incomplete output and the like caused by previous runs"*
  and *"even if you specify `.spec.parallelism = 1` and `.spec.completions = 1` and
  `.spec.template.spec.restartPolicy = "Never"`, the same program may sometimes be started
  twice."* The Job seat is the same seat with a wider concurrency mouth.

---

## 8. Flags for the conductor

- **26K synthesis, headline:** §3 — the inside seat collapses `kHALVES`. Attention and
  wall-clock differences between elide and guard both go to zero in a headless
  same-machine seat, so the guard-half is not a fallback there, it is the correct and
  strictly safer answer, and the survival tier is dead weight. Candidate ruling: a
  no-TTY `dorc-run` refuses `kSURVIVAL-trusted` by construction.
- **26K synthesis, limitations half:** `chafe-no-durable-why-in-ephemeral-seats`. The
  recovery story's "ask next week" does not survive ephemeral seats (init containers,
  Jobs, CI steps). Stderr-on-failure should be promoted from exception to the designed
  answer for that whole class.
- **Cross-book finding (both my books, different tools):** oracles for tools whose read
  verbs carry embedded query sublanguages in a single argument (kubectl jsonpath, `find`
  predicates, `jq`, `psql -c`) cannot argparse-walk to the fact. Read-only ones have an
  escape — faithful delegation, understand nothing, re-run and report. **Mutating ones do
  not**, and that boundary is worth a line in the corpus.
- **Delegation is an exfiltration surface** (from the node book, applies here too): a
  blanket "delegate any read-only verb" arm ships whatever the user asked for into the
  report lane. Read-only ≠ safe-to-report.
- **`dorc compile` deserves promotion in the k8s story** (§4): no Dorc binary in the image,
  artifact reviewable in the PR, no-fork rule trivially satisfied, and §3 says the
  guards-only narrowing costs nothing in this seat. My brief pinned `dorc-run` as primary
  so I wrote that; the conductor may want the emphasis the other way round.
- **Jenkins `sh` step** (turn D's narrowly-refuted DEAD verdict) is the same shape as this
  book — an interpreter selector on a long-lived agent — and would be a cheap second
  inside-exhibit if one is wanted. This book already carries the structural argument, so I
  do not think it is needed.
