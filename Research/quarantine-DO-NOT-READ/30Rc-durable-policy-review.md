# 30Rc - Durable policy review: first human-acked directions

> Tier: quarantined, living review of `plans/30R` and `quarantine/30Ra`.
> These directions are human-acked but deliberately softer than most Dorc
> rulings. Security policy remains comparatively malleable as research,
> implementation contact, and product experience improve. This is not an
> implementation plan. It will grow before an implementation-planning conductor
> reconciles it into an updated `30R`/`30Ra` pair.

## 30Rc:receipt-policy-is-user-configurable

**[ACKED, SOFT]** Receipt durability is controller policy, not one universal
posture. Dorc serves users ranging from time-poor homelabbers to teams using it as
glue among more battle-hardened systems. They legitimately differ on whether a
durable failure should withhold mutation.

The default leans secure and demanding. Dorc's product already earns the right to
say "no" by pairing refusal with one concrete way forward. Interactive use is the
primary workflow; a user who just typed `dorc apply` can normally investigate an
immediate pre-network refusal. CI, cron, and other unattended operation are
secondary modes whose better-resourced users can configure deliberately.

This does not imply that relaxed durability is the choice of stricter teams.
Teams running unattended or organizationally controlled applies may be especially
likely to require a crash-surviving pre-mutation receipt.

## 30Rc:pre-dispatch-receipt-gates-by-default

**[ACKED, SOFT]** Under the default policy, Dorc publishes the exact applicable
decision receipt before crossing the first mutative dispatch boundary. An
immediately recognizable failure that would prevent a sane, coherent later
`dorc why` refuses before mutation and names the configuration or override that
permits proceeding.

A recent successful plan is useful evidence that the controller filesystem was
writable, but never substitutes for the apply-side publication of the exact
decision, artifact, target, context, and invocation identity.

## 30Rc:mutation-dispatch-flips-durable-failure-direction

**[ACKED, SOFT]** The policy transition is one coherent event: after tunnel
standup and successful required receipt publication, immediately before Dorc
dispatches the first potentially mutative book command. Tunnel standup remains in
the fail-fast region. Once Dorc commits to dispatching command identity one, the
durability/debugging failure direction reverses.

After that boundary, a controller-side durable failure alone does not abort an
otherwise coherent apply. The operation may already be partial, the user may no
longer be present, and stopping cannot restore the missing history. Dorc continues
or aborts according to execution, transport, attribution, and orchestration
integrity, not according to whether later whylog material can still be persisted.

Crossing the boundary does not claim that the first command reached or mutated the
host. It records that Dorc spent the authority to dispatch it; remote outcome may
remain unknown.

## 30Rc:convenience-profile-expands-to-closed-defaults

**[ACKED, SOFT]** Dorc may provide a deliberately easy global escape hatch for a
low-risk, accepting user. Strawman spelling: `dorc --leave-me-alone`. It is a
configuration action in its own right and is not attachable to another command.

The action expands once into a closed, predetermined set of ordinary option
values. It immediately and synchronously prints every changed option, the cost the
user accepted, and the command that reverses that individual change. A common
intended reaction is to keep the convenience profile while manually walking back
one or more changes.

Future options remain at their unconfigured high-security defaults. They never
silently join an earlier convenience-profile expansion. The user must rerun the
profile or configure the new option individually. The effective individual
policies, not a generic "security off" bit, enter decision identity and durable
explanation.

Illustrative user-story wording supplied during the ruling, not settled prose:

```text
dorc just configured your system to show more plaintext in permanent, on-disk
logs. Reverse this one with `dorc --set durables=cbor`.
```

The example's option names and format remain strawman. The stable direction is
Show Our Work: convenience is allowed, hidden convenience is not.

## 30Rc:tty-is-mode-never-user-availability

**[ACKED]** TTY presence is an explicit Dorc mode signal for an active terminal
and the Unix pipe workflow. It does not imply that the user will remain available
during probing or apply, and it never grants consent to weaker security or
durability policy.

## 30Rc:direct-readable-structure-remains-a-goal

**[ACKED, SOFT]** Preserve the strong lean toward a plaintext-like, directly
inspectable structural envelope for as much material as can be justified. The UX
benefits in firefighting, old-version inspection, quoting, diffing, and external
debug handoff are load-bearing product value and are not sold away merely because
a binary candidate has a stronger parser story.

Readability and disclosure policy may differ across users and fields. That does not
yet justify multiple physical durable grammars.

## 30Rc:one-canonical-grammar-unless-proven-intractable

**[ACKED, SOFT]** Lean toward one canonical physical durable grammar. Rich versus
plain projection, filename disclosure, field retention, and receipt policy are
configurable and recorded per document; the underlying format is not presently a
user configuration surface. Multiple canonical formats would multiply hostile
parsers, damage models, compatibility obligations, and cryptographic joins.

Binary is not automatically the secure pole and text is not automatically the
insecure pole. The canonical grammar question remains open to implementation and
focused research. If one readable grammar proves intractable, format plurality may
be reopened explicitly rather than pre-paid now.

## 30Rc:policy-changes-never-rewrite-history

**[ACKED, SOFT]** Every durable records the exact projection and policy under which
it was written. Later configuration changes apply only to future invocations. They
never rewrite, downgrade, upgrade, or reinterpret historical durable contents.

## 30Rc:review-residue-remains-open

This review has not selected names, a text grammar, an encryption topology, key
custody, retention defaults, or the exact pre-dispatch publication artifact. It has
only narrowed the policy shape those later decisions must satisfy.
