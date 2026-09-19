Codex GPT-6-Astra (OpenAI lineage), adversarial pass, 2026-09-16, exit status 0

`local-route-promotes-lookup-identity` — wrong SAME without an author’s uniqueness warrant. +SURE the written rule admits this.

```sh
redis-cli -h 127.0.0.1 -p 16379 CONFIG SET timeout 60 >/dev/null
```

The machine has a local TCP proxy balancing connections between two existing Redis servers. Server A has `timeout=60`; server B has `timeout=0`. Nothing changes during the book. The probe reaches A; the original book command would reach B.

Give the Redis oracle the model’s advertised floor: an mScheme naming the endpoint and configuration item, with no `:yields` and no uniqueness warrants. Its probe accurately reports the value it read.

The walk:

- **311j §1.3:** the undeclared mScheme gets an identity `resolve()` and the local mRoute as parent.
- **§1.10:** same-spelled mKeys in one transit-free unwalled span become one mPlaceholder. There is no wrapper transition here: `redis-cli` opens the connection inside its binary.
- **§3.2:** one mPlaceholder licenses SAME without `:guarantees-unique-referent`.
- The fact measured on A consequently licenses removing the command whose execution would configure B. B remains at `timeout=0`.

The floor’s standup witness compares the unchanged endpoint key; it has acquired no backend identity. §3.3 has no mutation to invalidate anything. This is set-valued routing between two live servers, outside neither of your declared horizons. [A-311j:local-route-identity-exemption](C:/Users/ec/Sync/Code/Dorc/Research/notes/311j-identity-and-relation-model-two-species.md:202), [A-311j:shared-placeholder-sameness-rule](C:/Users/ec/Sync/Code/Dorc/Research/notes/311j-identity-and-relation-model-two-species.md:426)

This also locates the misplaced knowledge: the engine knows that the shell’s execution context is unchanged. It does **not** know that the client’s next request reaches the same server. That knowledge belongs to the connection and routing machinery; the generic tool author cannot infer it from an endpoint string either.

A narrower interpretation—only the route node itself receives the engine’s sameness exemption—would avoid this failure. That restriction is missing from the written rule, and **311q §4 explicitly endorses the shared-placeholder exemption**. The predecessor’s §7 had left the local-route claim as an untested concern; the rewrite removes that qualification. [A-311q:shared-instance-closure-record](C:/Users/ec/Sync/Code/Dorc/Research/notes/311q-attack-and-firming-ledger.md:115), [A-311:unresolved-local-route-claim](C:/Users/ec/Sync/Code/Dorc/Research/notes/311-identity-and-relation-preparatory-model.md:642)

---

`forwarding-alias-gets-separated` — the exercise’s declarations produce a wrong DISJOINT. +SURE.

```sh
sysctl -w net.ipv4.ip_forward=0
sysctl -w net.ipv4.conf.all.forwarding=1
```

This is a hardening fragment followed by an administrator’s forwarding override. Initially forwarding is enabled. The first line must run; the second initially probes as converged.

These names address **the same kernel field**, not merely two settings with related effects. I checked Linux’s implementation: both registrations use `IPV4_DEVCONF_FORWARDING` in the namespace’s `devconf_all`, with the same handler. [A-linux:forwarding-controls-share-storage](https://github.com/torvalds/linux/blob/v6.12/net/ipv4/devinet.c#L2506)

Now use the published Michael/Rachel declarations without changing their semantics:

- Michael’s lookup yields `sm.ProcSysPath:net/ipv4/ip_forward` and `sm.ProcSysPath:net/ipv4/conf/all/forwarding`.
- Rachel’s `net/*` arm puts both in the same network namespace and grants `:guarantees-unique-name`, `:guarantees-unique-referent`, and `:aliases-nothing-else`.
- **311j §3.2:** the deepest shared ancestor is that namespace. The two legs have different values of the same primary mScheme, with the required warrants. The answer is DISJOINT.
- Michael’s footprint names the first key; Rachel’s finished definition adds nothing. Under `--risk-faultless-skips`, the second line’s measured convergence can survive the first line.
- The resulting book disables forwarding and removes the line that would restore it.

The false sentence is Rachel’s `:guarantees-unique-name` on `net/*`. A procfs pathname is not automatically a canonical identifier for kernel storage. [A-312b:kernel-path-uniqueness-warrant](C:/Users/ec/Sync/Code/Dorc/Research/notes/312b-exercises/net-sysctls-are-per-namespace.md:142), [A-311j:divergent-key-separation-rule](C:/Users/ec/Sync/Code/Dorc/Research/notes/311j-identity-and-relation-model-two-species.md:420)

Neither command uses shell redirection, so the exercise’s procfs-refusing File binder provides no protection. Both use the supported `sysctl -w KEY=VALUE` shape.

This is a faulty declaration in the actual specimen, not a missing representational mechanism. The model can carry the alias by canonicalizing both names or withholding the uniqueness warrant. The finding is that the exercise currently installs the dangerous warrant and calls its path spelling primary.

---

`different-schemes-trigger-false-conflicts` — a contradiction introduced by the rewrite. +SURE; its immediate failure direction is safe refusal.

```sh
git -C /srv/source worktree add --detach /srv/canary refs/heads/release
git -C /srv/source worktree add --detach /srv/stable refs/tags/release
```

The repository has a branch `release` pointing to commit A and a tag `release` pointing to commit B. This is an ordinary deployment arrangement.

Use two valid mSchemes of one commit mSort:

- a branch-name mScheme, with key `release`;
- a tag-name mScheme, also with key `release`.

Both look up names in the same repository, from the same vantage, and yield commit identifiers in the same object database. Each lookup is correct.

**311j §3.5 nevertheless says that two mSchemes resolving “one string, in one mParent, from one mVantage” to different primary keys imply that at most one is right.** It therefore withholds both. [A-311j:different-schemes-conflict-rule](C:/Users/ec/Sync/Code/Dorc/Research/notes/311j-identity-and-relation-model-two-species.md:490)

That conclusion does not follow:

1. **§§1.3–1.4 make the mScheme part of the meaning of an mKey.** Equality of the strings does not equate these two questions.
2. **§1.5’s unique-referent warrant means equal primary keys identify one referent.** It says nothing against two correct lookups returning different primary keys.
3. **§2.2 explicitly forbids using unequal tokens as contradictory evidence without the unique-name warrant.** §3.5 names the other warrant. Even substituting unique-name would not repair the branch/tag example: nothing established that the two questions should reach one commit.

The diff adds this falsification rule; the predecessor’s §3.5 did not contain it. Git remains representable, but this newly asserted law incorrectly rejects a valid representation. Prefixing the two input strings differently would evade the test without making its inference valid.

---

`unequal-depth-proof-does-not-follow` — the accepted comparison edit does not establish the exercise’s claimed survival. +SURE; again, safe UNKNOWN rather than under-execution.

```sh
ip netns exec blue sysctl -w net.ipv4.ip_forward=1
sysctl -w kernel.pid_max=4194304
```

Assume blue needs its write and `pid_max` already has the requested value. This is the exercise’s drifted-day claim in two lines.

Following its own identities, the chains are:

```text
route → boot → network namespace → net/ipv4/ip_forward
route → boot                     → kernel/pid_max
```

Under **311j §3.2**, their deepest SAME ancestor is the boot. Both remaining legs are nonempty. Their top mKeys belong to different mSchemes:

```text
sm.NetnsInode       versus       sm.ProcSysPath
```

The required “tops are mKeys of one mScheme” condition fails. The answer is **UNKNOWN**, so the second command needs a guard. All the stated `:aliases-nothing-else` warrants can be true without satisfying that condition. [A-311j:unequal-depth-comparison-rule](C:/Users/ec/Sync/Code/Dorc/Research/notes/311j-identity-and-relation-model-two-species.md:420)

The exercise instead says the second command survives, and **311q §4 explicitly records the edit as recovering that survival**. Those statements disagree with the edited rule. [A-312b:unequal-depth-survival-claim](C:/Users/ec/Sync/Code/Dorc/Research/notes/312b-exercises/net-sysctls-are-per-namespace.md:407), [A-311q:unequal-depth-closure-claim](C:/Users/ec/Sync/Code/Dorc/Research/notes/311q-attack-and-firming-ledger.md:107)

There is another exact defect in that clause: its empty-leg alternative is unreachable after “if either mKey is A itself, … UNKNOWN.” An empty leg is precisely that situation. The written rule therefore needs correction before this render can count as a validated exercise.

---

I did **not** establish an ordinary ops situation that 311j cannot represent after legal rearrangement. In particular, separating persistent and runtime service bearers preserves the mixed-lifetime case using existing machinery; I have not counted the apparent aspect-model regression as a finding.

These are symbolic walks through the documents, not claims about the current implementation. I read the requested documents and the predecessor diff, worked alone, and changed nothing. Source IDs marked `A` identify directly inspected text or primary implementation evidence.