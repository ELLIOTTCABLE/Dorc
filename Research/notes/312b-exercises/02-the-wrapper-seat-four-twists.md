# 312b-exercises/02 — The wrapper seat, four twists

> Exercise record (Fable, 2026-09-08). Not durable; takeaways live in `notes/312b`. Method: hold
> `notes/311` as the final types, push one concrete habit onto it, card only where it strains.
> Plain book-sh grounds the case; the model's response is prose in 311 vocabulary; no oracle
> spellings. Grades: +SURE / ~SUSPECT / -GUESS / --WONDER.

## The habit

```sh
sudo crontab -l | grep -q certbot || printf '0 3 * * * certbot renew\n' | sudo crontab -
env RUST_LOG=debug sudo tool serve
sudo env RUST_LOG=debug tool serve
SUDO=; [ "$(id -u)" -eq 0 ] || SUDO=sudo
$SUDO apt-get install -y nginx
```

Four things sudo does to a site that the crontab author, the tool author, and the book author
did not write down: it changes which user's crontab a spelling reaches; it scrubs the
environment, so two wrapper orders differ; its policy can pick a different context per guest
command; and it is often not a word at all but a variable.

## First glance

Blame the crontab oracle for reading root's crontab as alice's; blame the tool oracle for losing
`RUST_LOG`; blame nobody for the sudoers rule because nobody can see it; treat `$SUDO` as an
opaque command word and wall every line it prefixes. Three wrong seats and one honest but
terrible floor.

## Where the model holds

Twist one, the crontab pair, is the model working. Cron's natural key is named-in User with
AMBIENT supply; sudo lends User; the two sites reach two fully-qualified keys; no correspondence
exists; the alice-measured fact cannot license root's line. The cron author declares one thing
they know (crontabs are per user); the sudo author declares one thing they know (sudo lends a
user). Neither knows the other.

Twist two, the order pair, is the model working if the sudo author speaks. § 3.4's frames nest
in book order, innermost wins; sudo's environment lend is a sever (a fresh instance with a
policy-defined key set), so `env` outside it is lost and `env` inside it survives.
`rho-claim-ladder` already has the vocabulary. The residue is that the key set is policy
(`env_keep`), which is twist three.

## Where it strains

Twist three: sudoers can match on the guest command, so the context that `sudo -n sh -c check`
enters at probe time can differ from the one `sudo original` enters at apply time (`27Xf`).
§ 3.4 makes a wrapper's lends a function of the wrapper's own argv, the peel. Here the lend is a
function of the guest bytes, which the wrapper author cannot see and the admin wrote in a file
the book never mentions. Measure-in-context silently measures a different context.

Twist four: `$SUDO` is a two-valued command word. The value plane sees `$(id -u)` as a host
read, so `SUDO` is ⊤ and the site walls (`an-name-observation-census`: a non-literal command
word is a use of every name). The most common privilege idiom in real books gets the worst
answer.

## The card (twists three and four)

Unknowability class: vacant for three (the admin knows their sudoers; the sudo author knows the
possibility; the host can be asked); vacant-by-forfeit for four
(`FORFEITS:forfeit-value-narrowing-by-test`).

Can know at all. Three: the sudo author (that per-command policy exists, and that `sudo -l`
lists it); the admin (their file); the host (the read). Four: the engine (the two branches);
the admin (a literal); the stdlib (an `id` read making `$(id -u)` a measured value).

Can reasonably know. Three: the admin for their deployment; the sudo author for the default.
Four: the engine.

Danger. Three: a guest-sensitive policy on a probe-relevant command is rare, but the failure is
a confident wrong verdict in the wrong context, the worst probe object (`27C`). Leverage is every
sudo site; countability says the default (guest-insensitive) is stdlib-owned and the exception
is the admin's. Four: none; unknown is safe. The cost is attention, since every `$SUDO` line
stays in the plan.

Verification. Three: mechanical; the policy is readable in the denoted context, and a sudo
oracle that reads it and declines on a per-command rule turns a silent wrong context into a
loud can't-say. Four: the engine can show its two branches.

Transformation menu. Three: declare guest-insensitivity as the wrapper author's default, let
the admin override, and let the oracle read the policy and decline on surprise. Four: carry
the branch set through the value plane; a site under a two-valued wrapper has a set of two
entry chains; compare universally over the set. A fact whose key does not pass through User is
the same in both chains and survives; one that does is unknown. The model's key shape does the
work with no new declaration. Cheaper: the admin writes `SUDO=sudo` as a literal, which is a
cliff, not a fix.

Residue if nobody speaks. Three: measure-in-context with no policy read, which is today; the
hazard is documented, not closed. Four: wall.

## Leads and wants

- `lead-lends-may-depend-on-the-guest`: a wrapper's lend is a function of its peel by default
  and may be declared guest-sensitive; the sudo author declares insensitive-by-default and
  supplies a policy read that declines.
- `lead-policy-reads-decline-on-surprise`: the general shape for policy-dependent wrappers and
  tools (sudoers, APT hooks): read the policy in context, decline when it departs from the
  declared default. Converts a deployment fact the author cannot hold into a measurement the
  author can author.
- `lead-context-sets-at-a-site`: a site may be entered under a finite set of wrapper chains;
  compare quantifies universally over the set, as the universal meet already does over backing
  sets.
- `lead-narrowing-by-test-is-the-capture`: `forfeit-value-narrowing-by-test` is what turns
  `$SUDO` from ⊤ into a two-element set; its priority rises because the idiom is dominant.
- `want-id-read-in-stdlib`: `$(id -u)` as a measured value makes the wrapper known per host at
  probe time.

## Open

- Whether a guest-sensitive lend is expressible at all without the wrapper oracle seeing the
  guest bytes, which `rul-argv-flows-bytes-do-not` forbids for the book's bytes. ~SUSPECT the
  policy read sidesteps it: ask sudoers about the guest command by name, never ship the bytes.
- Whether the admin's per-deployment override is a book-side declaration or a lint
  acknowledgement; not this phase's question.
