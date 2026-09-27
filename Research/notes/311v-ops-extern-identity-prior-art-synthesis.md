# 311v — Prior art for identity across ops externs: the survey synthesis

> AI-authored (Fable, the successor conductor of the 2026-09-26 gather round; the human present
> and adjudicating). Notes-tier. A synthesis of the ten-lane prior-art survey whose evidence base
> is `.claude/research/ops-extern-model-prior-art/`: the graded manifest `sources.json`, the
> archived copies under `sources/`, the ten `gather-*.md` lane reports, the outgoing conductor's
> `conductor-ledger.md`, and the two turn logs. Every grade is `graded-by: subagent`, one read
> each; the synthesizing conductor read no primary. Where this document disagrees with a lane's
> mapping it says so.
>
> Written against `notes/311` at `7a63bae6`. The model is under active development, so this
> document describes its shapes in the survey's own terms and cites 311 sections only in the
> bridge table of 1.6-a-bridge-from-the-classics; it is meant to outlive the section numbers.
> Nothing here is ruled. The human's typed responses to the in-chat synthesis are banked in the
> research dir's `turn02-2026-09-26-notes.md` and shaped 6-the-bad-news.

## § 0-the-question-and-the-answer

The question, as the outgoing conductor recorded it: prior art for the identity-and-relation
model, meaning a precise, broad, abstract, battle-tested specification of the behaviour of ops
externs at any altitude, judged on four criteria (exhaustive, broad, abstract, more battle-tested
than ours). The human's two example shapes were a famous tool's RFC repository that litigated the
territory, and an academic modelling project that had to build a model of ops to do its analysis.

The answer is bimodal.

- Everything in the model that is a property of one lookup has been independently reinvented by
  three to six battle-tested lineages each: the two uniqueness warrants, the traversal as a
  fact's backing, recycled keys on the horizon, sameness across a transition as the transition
  owner's speech, and rootness declared per scheme (1-what-the-survey-corroborates). That is
  corroboration by convergent evolution.
- Everything in the model that composes several strangers' statements into a separation has no
  precedent anywhere: the walk from the shared ancestor, the store's self-knowledge closure, the
  region test and the placing lookup, and the address-then-write-path test
  (2-what-has-no-precedent). Ten lanes independently confirmed why. Every system that reached
  that spot took one of four exits (+SURE across all ten lanes, first stated in
  `gather-wildcard-and-counter-thesis.md`):
  1. the name is the thing (dpkg, `try`, `unveil`, OVAL items, every CM tool's catalog key);
  2. one minting authority (TxOS, Riker, RETRO, Nix, Kubernetes, Terraform, ConfSolve);
  3. delegate reconciliation to humans or heuristics (CMDBf, SACM, Repology);
  4. stay inside one domain (SMI-S, WOW64, SibylFS).

Neither example shape exists as asked. The famous tools' RFC repositories are nearly silent on
identity (Salt's SEPs, Ansible's proposals, Chef's RFCs, OpenTofu's RFCs; Puppet's litigation
lives in Jira); the territory was litigated in bug trackers one case at a time and never as a
model (3.3-the-litigation-record). The closest academic project, Mancoosi, built a broad
system-configuration metamodel to simulate Debian upgrades, declared itself "skeptical that
static analysis can fully solve the problem", and replaced maintainer scripts with a
non-Turing-complete DSL [B-mancoosi-d21-system-configuration-metamodel-2009]
[B-mancoosi-d32-maintainer-script-dsl-2009]. Its skepticism is about deriving the touched-file
set from script text, a route Dorc already refuses (`KNOBS:kDEPS`, trace-don't-derive); Dorc asks
authors for at-most sets instead. The human's read (typed, 2026-09-26): Dorc has gone declarative
too, spelling its declarations in sh; the survey's finding corroborates that read rather than
contradicting the product.

The survey cannot say whether the model is the minimal composing model, because there is no
other composing model to compare it against. That question stays with the crosscheck burndown
(`notes/312cg`).

## § 1-what-the-survey-corroborates

Each shape below is +SURE per the lanes' reads and ~SUSPECT on the conductor's reading of the
lanes rather than of the sources.

### § 1.1-two-independent-warrants-absent-by-default

The split of identity licensing into two per-lookup warrants, one licensing sameness from
equality and one licensing distinctness from inequality, each declared separately and absent by
default, is the best-corroborated shape in the model.

- Ontology engineering splits identity criteria into sufficient conditions (equality follows)
  and necessary conditions (whose contrapositive licenses non-identity), asserts them separately
  as "weak ICs", and notes that shared essential properties "can be used to make conclusions
  about non-identity, if not about identity"
  [A-guarino-welty-identity-unity-individuality-2000] [A-guarino-welty-overview-of-ontoclean-2009].
- NFSv4.1 makes equal filehandles a MUST for same-file, makes one-to-one only a SHOULD, and adds
  a per-filesystem attribute the server declares, absent by default, that "two distinct
  filehandles are guaranteed to refer to two different file system objects"; clients "MUST use
  filehandle comparisons only to improve performance, not for correct behavior"
  [A-ietf-rfc8881-nfs41-fileid-fsid-attributes-2020].
- DNS refuses a unique-name requirement ("There is no such requirement in the DNS" that a host
  have one official name) while keeping the CNAME lookup a function ("only one such canonical
  name for any one alias") [A-rfc2181-dns-clarifications-1997].
- CIM's namespace rule licenses same from equal strings and "no conclusion" from unequal ones
  [A-dmtf-dsp0004-cim-infrastructure-2-8-2014]; URI comparison is "designed to minimize false
  negatives while strictly avoiding false positives" and "is not sufficient to determine whether
  two URIs identify different resources" [A-ietf-rfc3986-uri-generic-syntax-2005].
- The classical text states the two uniqueness rules as separate per-scheme choices, and gives
  the stable-binding unique-identifier space as the case where names become useful "for
  comparing references to see if they are to the same object"
  [A-saltzer-kaashoek-system-design-naming-2009]. The outgoing ledger's caution stands: that
  passage presents the rules as alternatives across schemes and implies their independence
  rather than stating it; RFC 8881 is the stronger precedent.
- The refuted shape is deployed. CPE 2.3 name matching answers DISJOINT from unequal
  wildcard-free strings and its changelog records "Removed all mention of and support for the
  logical value UNKNOWN", live at NVD [A-nist-ir7696-cpe-name-matching-2011]. Windows Installer's
  component rules impose unique naming "across applications, products, product versions, and
  companies" as a social contract, and document that its breach "damages" reference counting and
  deletes shared files [A-msi-organizing-applications-into-components-2021]
  [A-msi-component-rules-broken-2021]. That is what a wrong DISJOINT costs.

### § 1.2-the-traversal-as-a-facts-backing

The lookup emitting the routing keys it crossed, so that a write to any of them invalidates the
resolution while leaving the target object alone, has been reinvented in security provenance,
transactional operating systems, separation logic, naming theory, and assessment languages.

- RETRO's directory manager records that a namei lookup "incur[s] a dependency from every
  directory entry traversed" [A-kim-retro-selective-reexecution-2010]; BackTracker splits the
  file object from the filename object and makes every open "affected by all parent directories
  of the filename" [A-king-chen-backtracking-intrusions-2003]; in TxOS "many kernel objects are
  only read in a transaction, such as the parent directories in a path lookup"
  [A-porter-txos-operating-system-transactions-2009].
- Fusion logic makes a path a footprint over every directory it crosses, so that removing a
  directory invalidates a path to a disjoint sibling that passes through it: "the local update
  has a global effect" [A-ntzik-gardner-posix-fusion-logic-2015].
- Lampson's global name service bounds a cached resolution's validity by "the minimum TX of any
  arc or link that was followed" [A-lampson-designing-a-global-name-service-1986]; Plan 9 channels
  remember the rooted name that reached them and the paper admits "renamings and deletions made
  by one machine may go unnoticed by others" [A-pike-lexical-file-names-plan9-2000].
- Riker records failed resolutions as "anti-dependencies", making absence along the route a fact
  a later change can invalidate [A-curtsinger-riker-incremental-builds-2022].
- The sharpest hit is a live OVAL board thread that independently reinvented the emitted chain
  (a `chainlink` entity per intermediate symlink) and the floor rule beneath it: an absent child
  entity means "not collected", so the chain's end needs an explicit `does not exist`
  [B-oval-community-symlink-file-object-discussion-2025].

### § 1.3-sameness-across-a-transition-is-the-transition-owners-word

Every deployed cross-authority sameness in the survey is a table written by whoever owns the
transition, never by a stranger to either side.

- The kernel translates pids across namespace layers ("translated into the corresponding PID
  value in the receiving process's PID namespace") [A-linux-pid-namespaces-man-2026] and makes
  `uid_map` a write-once table authored by the namespace's creator
  [A-linux-user-namespaces-man-2026].
- Plan 9's auth server holds "mappings between user names in different domains"
  [A-plan9-from-bell-labs-overview-1995]; NFS declares fileid and handle classes across
  migration, and without the declaration the client "is forced to assume that no object has been
  renamed" [A-ietf-rfc8881-nfs41-fileid-fsid-attributes-2020].
- Terraform's `moved` block may speak only about "its own objects and objects of its child
  modules" [A-terraform-refactoring-moved-blocks-2025]; Pulumi aliases compose through the parent
  chain [A-pulumi-aliases-resource-option-2026]; only a provider may declare an identity mutable
  [A-terraform-plugin-framework-resource-identity-2025].
- CIM had this as the `Correlatable` qualifier, org-scoped and one-directional, and CIM v3
  removed it with "No replacement" [A-dmtf-dsp0004-cim-infrastructure-2-8-2014]
  [B-dmtf-dsp0004-cim-metamodel-3-0-2014]. ENTITY-MIB keeps an agent-authored alias-mapping
  table beside the concession that multiple agents "are not required to be equivalent or even
  consistent" [A-ietf-rfc6933-entity-mib-v4-2013].

### § 1.4-recycled-keys-stay-on-the-horizon

The model's placement of recycled keys outside what its standup witness can see matches every
system that has a live host.

- NFS carries a generation number because "if the server should happen to reuse the inode of
  the old file for the new file, remote procedure calls from client 2 will get the new file"
  [A-saltzer-kaashoek-system-design-naming-2009]; a file handle's generation "sees a recycled
  inode that st_ino cannot" [A-linux-name-to-handle-at-man-2026]; POSIX pins the device-inode
  pair as unique only "at any given time" [A-posix-sys-stat-file-identity-2024] and the Austin
  Group re-litigated exactly that ("uniqueness can only guaranteed at an instant of time")
  [A-austin-group-defect-1314-file-identity-2020].
- Grapevine forbids the case outright: "Recreation of a deleted entry is not allowed"
  [A-birrell-levin-needham-schroeder-grapevine-1982]. Kubernetes minted `uid` "to distinguish
  between objects with the same name that have been deleted and recreated"
  [A-kubernetes-api-conventions-identity-sections-2026]; LDAP minted `entryUUID` because "DNs are
  not stable identifiers" [A-ietf-rfc4530-ldap-entryuuid-2006].
- IF-MIB keeps `ifIndex` constant only between agent re-initialisations, leaves "different
  interface" to implementors ("Any firm definition in this document would likely turn out to be
  inadequate"), and uses a counter-discontinuity timestamp as the witness instead of a new index
  [A-ietf-rfc2863-if-mib-2000]. systemd recycles dynamic uids and closes the store rather than
  the name [A-systemd-exec-man-2026]; D-Bus unique names "are never reused for two different
  connections to the same bus", a warrant scoped to one bus [A-dbus-specification-2024].

### § 1.5-rootness-is-declared-per-scheme-and-absent-by-default

- Solaris FMRIs are the closest deployed analogue: every identifier carries an authority,
  explicit or "implicitly that of the local fault management domain"; each scheme is declared
  logical (identical FMRIs "native to distinct fault management domains do not necessarily
  identify the same actual resource") or universal, by its owner, in the specification rather
  than the data; the package scheme is universal only "if package name and package version
  conventions are adhered to" [A-solaris-fmri-man-2011].
- A Nix store path "will always reference exactly one store object" but only within one store;
  "Different stores may disagree on what a given store path refers to" and referential integrity
  holds only "if store paths do not cross store boundaries" [A-nix-manual-store-path-2026]
  [A-nix-manual-store-object-2026]. RFC 62's two-glibc case is one output identifier reaching
  two objects in two stores, refused by one-realisation-per-store
  [A-nixos-rfc0062-content-addressed-paths-2019].
- Zone-qualified IPv6 literals "MUST NOT be sent on the wire unless every node that interprets
  the format agrees on the semantics" [A-rfc4007-ipv6-scoped-address-architecture-2005];
  machine-id "should be either missing or an empty file" in images used on multiple machines
  [A-systemd-machine-id-man-2026]; cloud-init's trust mode makes clones "detect their first boot
  as a subsequent boot" [A-cloud-init-first-boot-determination-2026].

### § 1.6-a-bridge-from-the-classics

The naming-theory lane built a term-for-term mapping from the classical systems vocabulary to
the model's objects (`gather-naming-theory-classics.md`, its mapping table). It is reproduced
here in compressed form as a reading aid for anyone arriving from that literature, and for the
record that the model's naming half has fifty years of teaching behind it. It proposes no
renaming; "context", the widest classical term, is already overloaded in this corpus, and every
retained name has survived several deliberate passes.

| classical term | model object | fit |
| --- | --- | --- |
| naming scheme (name space, mapping algorithm, universe of values) | mScheme | close; the model adds one accountable owner and per-shape declarations |
| `resolve(name, context)` | `resolve()` run in an mVantage, the mKey looked up in its mParent-Catalog | close; the classics fold vantage and parent into one argument, the model separates them |
| context, catalog | mParent-Catalog | close |
| default vs explicit context reference | the ambient mParent from the mEntryChain vs the bind seat | close |
| path name terminating at a built-in context; root | mFullyQualifiedKey terminating at a mRoot or the mRoute | close; the classics treat the root as unique per resolver, the model makes each terminus an mWorld |
| universal name space | `:root` | close; the classics do not discuss cloning |
| synonym, indirect name | a `:guarantees-unique-name` failure; a `:yields` whose target is another name | close |
| hard link as a binding to a lower-layer name | the path mScheme yielding the inode | close; the 1978 text says a higher-level name bound to a lower one is not a synonym |
| unique-identifier space, stable binding | a warranted mToken | close; the classics bundle never-reused into the rule, the model splits it off to the witness horizon |
| limited context (names must be reused) | recycled mKeys | close |
| search path, union directory | a lookup crossing several catalogs; its emitted mTraversal | partial; the classics note the hazard and have no traversal as backing |
| user-dependent binding, closure | `:observer-dependence`; a wrapper's `:lends` | partial |
| unstable binding | routing invalidation | named, not modelled |
| cache valid until the min TX of arcs followed | a mResolution backed by its mTraversal | close in shape; GNS forbids the write, the model invalidates on it |
| sufficient vs necessary identity criterion | `:guarantees-unique-referent` vs `:guarantees-unique-name` | close, and independent in both |
| supplies vs carries an identity criterion | primary mScheme owner declares; secondary schemes yield | close in spirit |
| qualified-name conditions (uniqueness within, existence of, invariance of the qualifier) | the mParent-Store edge | close; Kent treats it as an anti-pattern, the model embraces it with invalidation |
| no counterpart | KNOWN_UNSPOKEN; may-read and may-write with their closures; the region test; `:places` | the novel half |

## § 2-what-has-no-precedent

The store's self-knowledge closure as a declared warrant, the ordering of address before write
path with the rule that a finished write set generates no distinctness, the region test over
emitted traversals, the placing lookup, the separation-from-one-definition rule, and the walk
itself have no counterpart in any lane. The negative space around them is informative.

- Debian's DEP-17 states that "dpkg assumes that every filename uniquely refers to a file on
  disk", enumerates twelve ways the usr-merge broke that assumption across five separately
  path-keyed stores (ownership, triggers, diversions, alternatives, statoverrides), records file
  loss and a moratorium, and then rejects teaching dpkg about aliases in favour of eliminating
  them: "the project prefers to finish the transition without relying on changes to dpkg"
  [A-debian-dep17-usrmerge-aliasing-2023].
- The CMDB federation standard says cross-repository identity "is seldom absolute and often must
  rely on heuristics" and is done by "any combination of automated analysis and manual input"
  [A-dmtf-cmdbf-federation-dsp0252-2010]. The IETF's endpoint-identity model delegated "same
  endpoint" to organisation-chosen attributes and never became an RFC
  [B-ietf-sacm-information-model-draft-2017]. The whole-system provenance community states
  "there is little consensus" even on representing a rename, and "no complete formal models of
  mainstream operating systems" [A-chan-cheney-provmark-provenance-expressiveness-2019].
- Every project that verified, tested, or repaired configuration code keys the world by path and
  assumes aliasing away in its own words: CoLiS's path resolution "simply ignore[s]" symbolic
  links [A-colis-platform-maintainer-scripts-2022]; Rehearsal says "Puppet manifests have no
  aliasing" [A-rehearsal-puppet-determinacy-2016]; Citac proposes omitting preservation tests "for
  file resources with non-overlapping file paths" [A-citac-reliable-convergence-2016]; FSMove
  says every file "is associated with an inode rather than a path" and then keys its interference
  relation by the dereferenced path string, minting a fresh inode for any path unseen in the trace
  [A-fsmove-puppet-fault-detection-2020] [B-fsmove-fstrace-domains-source-2020]. The one non-punt,
  SibylFS, models hard links and directory links against twenty-one thousand traces and is
  filesystem-only with no mounts [A-sibylfs-posix-oracle-2015].

So the spot the model stands on is real and everyone else stepped around it. That corroborates
the problem, not the answer.

Three of the objects the outgoing conductor listed as unprecedented deserve a narrower reading.

- The two safe bottoms, "no generator ever spoke" against "a link is unmeasured", have no license
  consumer that distinguishes them; the distinction is aid-plane (which author to point at), and
  prior art's silence is what systems with no hint channel would show. SMI-S Clause 7 has the
  three-answer compare (same format and name, same; same format and different name, different;
  different formats, unknown) and folds the two bottoms, as everyone does
  [A-snia-smis-correlatable-durable-names-2020]. OVAL separates a collected object's flag from an
  item's status from a result, which is the nearest three-layer precedent, on the read side
  (4.1-cant-say-partitions).
- The may-write entailment and its finished record belong to the footprint model (`plans/30U`),
  which the identity model uses and does not redefine. The survey's silence there is about
  effects, not identity. The closest shapes are categorical (pledge's at-most subsystem
  declaration [A-openbsd-pledge-man-2026]) or schema-level (YANG's `when`-deletion
  [A-ietf-rfc7950-yang11-2016], CIM's optional cascade qualifiers
  [A-dmtf-dsp0004-cim-infrastructure-2-8-2014], dpkg triggers matched "against the path included
  in the triggering package, not against the truename" [A-dpkg-triggers-specification-2025]).
- The exclusion of containers at or above the shared level from the write-path test has a real
  rhyme: Rehearsal found that plain read/write-set commutativity "is not effective because many
  resources may create overlapping directories (e.g., /usr and /etc)", called it "false
  sharing", and special-cased idempotent directory creation
  [A-rehearsal-puppet-determinacy-2016]. Couch and Sun's algebra makes the same move at the
  level of parameters: "Actions that agree on any common values commute, including actions that
  act on different parts of the configuration" [A-couch-sun-algebraic-structure-convergence-2003].

## § 3-mutually-unaware-collaboration-the-siblings

"Collaboration" here means several authors, unaware of each other, whose descriptions or writes
meet over one thing. The survey found it in three forms: a small set of systems that designed
for it, a large set of consensus realities that accreted without anyone designing them, and the
litigation record of where it went wrong. Then the retreats, and the trails still moving. These
are the siblings to mine when the model's collaboration story is sat (the human, 2026-09-26:
"a discussion about collaboration inside this model is pending").

### § 3.1-designed-for-many-authors

Every deliberate design in the survey resolves multiple authors by partitioning authority and
attributing conflicts, never by composing strangers' descriptions of one thing. The pattern is
uniform enough to state as a finding (+SURE): designed collaboration is separation plus a
conflict rule plus attribution.

- Kubernetes server-side apply is the fullest deployed instance. Ownership is recorded per
  field, server-side, under a named manager; a conflicting apply "always fails" and names the
  other manager; `force` transfers ownership; equal values share it; omitting a field releases
  it; schema authors declare list-entry identity (`listMapKey`) against whole-replace lists. Its
  punts are instructive: a plain update "never provokes failure", so ownership is recorded but
  not enforced off the apply path, and after a topology change "the API server is unable to
  infer the new ownership" [A-kubernetes-server-side-apply-2025]. The AIP rationale for
  single-owner fields is a failure mode, two owners "leading to the client entering an infinite
  loop to correct the change" [A-google-aip-129-server-modified-values-2023].
- NixOS merges one option from many modules by override priority ("All option definitions that
  do not have the lowest priority value are discarded") and then by the option type's own merge:
  strings and enums refuse, lists concatenate with a separate order, attribute sets join by key,
  `uniq` admits one author; the conflict error names each file
  [A-nixos-manual-option-definitions-2026] [A-nixos-manual-option-types-2026]. Its `/etc` builder
  accepts two keys landing on one target only when their sources are one store path ("mismatched
  duplicate entry" otherwise) [A-nixos-etc-module-source-2026]; Guix refuses the same case
  outright, "instead of letting them through, eventually leading to a build failure"
  [A-guix-services-scm-source-2026].
- Guix service extension is the nearest thing to an owner-defined composition law: only the
  extended service's owner defines `compose` and `extend`, contributors speak only about their
  own service, and "There can be only one instance of an extensible service type"
  [A-guix-manual-service-types-and-services-2026] [A-guix-manual-service-reference-2026].
- Solaris gives every identifier an authority and every scheme a declared reach
  [A-solaris-fmri-man-2011]; SMF layers deliveries and, where one property arrives from two
  files in one layer, tags the instance "in-conflict" and refuses to start it, while other
  readers "see a random property setting from amongst all appropriate values"
  [A-solaris-smf-man-2011]. That is the survey's one deployed refuse-both, and two consumers of
  one contradiction apply opposite policies.
- The two-party protocols between a package system and an administrator who never meet are
  designed collaboration at the file grain: Debian's conffile three-way rule (whichever single
  party changed the file wins; both changed, the admin is asked; admin-deleted is never
  recreated) [A-debian-policy-files-conffiles-statoverride-2025]
  [B-debian-policy-conffile-handling-appendix-2025]; RPM's `.rpmnew` and `.rpmsave` beside a
  per-file list of cells the packager expects to drift [A-rpm-spec-file-directives-2026];
  update-alternatives inferring manual mode from observed divergence and warning that
  out-of-order maintainer calls "flip-flop" it [A-dpkg-update-alternatives-manual-2025];
  diversions and statoverrides as third-party routing writes on the file-list catalog
  [B-debian-policy-diversions-appendix-2025]. Policy bans hard links to conffiles because editors
  and dpkg alike "break the link".
- CFEngine and Augeas designed for a file with several writers. CFEngine names the co-owned file
  ("its contents are also being managed from another source like a software package manager")
  and chooses partial convergent editing over "all or nothing" ownership
  [B-cfengine-promising-editing-file-content-2012]; Augeas is built so that other tools need not
  preserve "any AUGEAS-specific annotations" [B-lutterkort-augeas-configuration-api-2008].
- Crossplane's `external-name` is the one place two minting authorities meet in code. When the
  external system mints a name and the controller dies before recording it, the runtime refuses
  to proceed ("The safest thing to do is to refuse to proceed") until a human clears an
  annotation; the only escape is a per-resource-type provider declaration that the external
  name is deterministic [A-crossplane-managed-resources-external-name-2026]
  [A-crossplane-runtime-managed-reconciler-2026].
- The theory and the validated practice both answer by separation. Promise theory's agents "can
  only make promises about their own behaviour", third parties in a promise body "can
  apparently lead to contradictions", and inconsistency is assumed away by "overriding control"
  [B-bergstra-burgess-static-theory-of-promises-2014] [B-burgess-some-notes-about-promise-theory-2015].
  Couch's closure "will presume that it is the sole manager of its configuration"
  [A-couch-hart-idhaw-kallas-seeking-closure-open-world-2003]; Burgess and Couch prescribe "One
  manager for one aspect" and "Document all remaining overlaps"
  [A-burgess-couch-modeling-next-generation-configuration-management-2006]. Prodspec at Google:
  "Only one automation system should be responsible for a given part of production", and
  parallel rollouts "must modify distinct aspects of the assets"
  [B-palatin-prodspec-annealing-intent-based-actuation-2021]. Traugott's trading floors: "Never
  log into a machine to change anything on it" [B-traugott-huddleston-bootstrapping-infrastructure-1998].
  LCFG gave each subsystem its own class script over `host.subsystem.attribute`
  [B-anderson-towards-high-level-machine-configuration-1994].
- The one attempt to make separate tools' descriptions cohere, Anderson and Smith's interchange
  proposal, sidestepped the shared vocabulary ("such a standard lexicon could not be meaningfully
  defined") and concluded that "no current configuration tool presents a satisfactory answer to
  the question of how conflicts can be resolved"
  [B-anderson-smith-configuration-tools-working-together-2005]. Naming authority in the classics
  is delegation by hierarchy, including authority over absence ("covers a name if it
  authoritatively knows either the definition of the name or that the name is undefined")
  [A-cheriton-mann-decentralizing-global-naming-1989]; Grapevine fell back on "some human-level
  centralization of name creation" per registry [A-birrell-levin-needham-schroeder-grapevine-1982].

The model's committee law is the same principle (one author per statement) applied to
composition rather than partition. The survey has the principle everywhere and the application
nowhere.

### § 3.2-consensus-by-accretion

Far more common than designed collaboration is a consensus reality that nobody declared: a
spelling convention that became identity, several identifiers accreting for one thing as each
prior one failed, or a curated vocabulary with an explicit unknown. Each is a shape the model
already names, and the survey shows what it looks like in the wild.

- A convention that became identity. Plan 9 has "no global name space", and "for a process to
  function sensibly the local name spaces must adhere to global conventions"
  [A-plan9-use-of-name-spaces-1993]. dpkg's assumption that a filename is the file was never
  designed; DEP-17 discovered it decades in [A-debian-dep17-usrmerge-aliasing-2023], and dpkg's
  trigger interests match "Only textually identical filenames"
  [A-dpkg-triggers-specification-2025]. Puppet's identity is `(type, title)` because the 2006
  pitch called an element "a kind of named hash" needing only "the element type and the element
  name" [B-kanies-puppet-next-generation-cm-2006]; the composite package key came eight years
  later after a "long, drawn out battle" and is admitted "not an airtight guarantee"
  [A-puppet-pup1073-composite-package-namevar-2013]. Salt's documented idiom for two states on
  one file is two IDs with one `name` [A-salt-highstate-id-and-name-declaration-2026]. Chef 13
  made equal names "entirely separate" resources and admits it "makes it impossible to safely
  deliver notifications to the right resource" [A-chef-deprecation-resource-cloning-chef3694-2026].
  Kubernetes treats a physical host "re-created under the same name" as the old Node "which may
  lead to inconsistencies" [B-kubernetes-object-names-and-ids-2025]; Terraform's one-object-one-address
  is "normally guaranteed by Terraform itself having created all objects" and otherwise the
  importer's problem [B-terraform-cli-import-usage-2025]. HOST-RESOURCES-MIB keys installed
  software 1 to N, so any install renumbers the table [A-ietf-rfc2790-host-resources-mib-2000].
  In the model these are the unwarranted floor: a scheme with an identity resolve and no
  warrants, honest exactly because it claims nothing.
- Several identifiers accreting for one thing. udev offers nine or more `/dev/disk/by-*` schemes
  for one block device, each added when the last proved unstable, with a rule comment admitting an
  NVMe symlink "might get overridden" and `by-diskseq` added for race-free reopening
  [A-systemd-persistent-storage-udev-rules-2025] [B-archwiki-persistent-block-device-naming-2026].
  OCI keeps an ImageID, per-layer DiffIDs, an ordered ChainID, content digests, and "zero, one,
  or many tags" for one image [A-oci-image-spec-config-identifiers-2025]
  [A-oci-distribution-spec-tags-digests-2025]. systemd units have "four names" through load-path
  symlinks, with drop-ins read for every alias and an alias introduced at reload migrating the
  old state "to a new stub orphaned unit" [A-systemd-unit-man-2026]. LVM admits "different VGs
  with the same name can appear" after moving disks and falls back to the UUID
  [A-lvm-manual-unique-names-2025]. In the model these are several derivations for one topic,
  reconciled by coherence and never by priority; the survey shows the derivations arriving one
  failure at a time.
- A curated vocabulary with an explicit unknown. CPE names are minted by NIST and adopted by
  vendors, and the matching spec concedes "name matching distinctions are often use-case
  dependent" [A-nist-ir7696-cpe-name-matching-2011]. Repology reconciles package names across
  some three hundred repositories with roughly nine thousand merge and five thousand split rules,
  curated by humans since 2018, and every split group must end in a catch-all that flags
  `unclassified` [B-repology-rules-readme-2026]. Split-horizon DNS broke the notionally global
  name space by accretion of views ("not a standardized part of the DNS, but they are widely
  implemented") [A-rfc9499-dns-terminology-2024], and RFC 4592 had to redefine existence itself
  because RFC 1034 read literally means "all possible domains exist"
  [A-rfc4592-dns-wildcards-2006]. IF-MIB leaves "different interface" to each implementor, and
  ENTITY-MIB names "multi-scoped" objects whose identical instance values mean different things
  in different naming scopes [A-ietf-rfc2863-if-mib-2000] [A-ietf-rfc6933-entity-mib-v4-2013].
  In the model this is the reconciliation labor of yielding one stranger's scheme into another's,
  which the survey shows has a home only where someone budgets for it (6-the-bad-news).
- A vendor-published aliasing table. Windows publishes, per registry key, whether a 32-bit
  observer's path is redirected to a different physical key or shares one physical copy mapped
  into each view, with subkeys inheriting the parent's rule; the merged `HKEY_CLASSES_ROOT` view
  and the indexical `HKEY_CURRENT_USER` link ride the same table, and the synchronising
  "reflection" mechanism was retired [A-microsoft-wow64-registry-keys-affected-2022]. That is a
  store's describer publishing its own aliases and observer-dependence, one domain, once.

### § 3.3-the-litigation-record

Where collaboration went wrong, the record is uniform in a second way (+SURE across lanes 3, 4,
6, 8): the wrong answer was nearly always a wrong DISJOINT, its cost ranged from cycling to
data loss, and the fix was never to model the alias. Each fix minted a token, refused, reverted
a composition, or eliminated the alias. The human's typed read is that most of the world's
failures come from a poor or forgotten identity model; this record is the evidence for it, and
it is also the evidence that nobody has shipped the model that would have prevented them.

- Data loss from a wrong DISJOINT across strangers. DEP-17: several path-keyed stores each broke
  separately under aliasing, with "unexpected file loss" mitigated by a "file move moratorium"
  [A-debian-dep17-usrmerge-aliasing-2023]. Windows Installer: two components installing one
  resource under one key path, "removal of either component removes the common resource" and
  "the reference-counting mechanism is damaged" [A-msi-component-rules-broken-2021]. Fix: eliminate
  the aliases; publish a social contract and blame the vendor.
- Cycling and duplication from a wrong key. DSC's timezone resource had the timezone value as its
  only key, so two blocks compiled as two instances and the node cycled "the timezone back and
  forth"; fixed by a synthetic singleton key [A-microsoft-dsc-single-instance-resource-2020].
  Crossplane leaked externally-minted resources and created duplicates; fixed by refuse-and-wait
  [A-crossplane-managed-resources-external-name-2026]. Puppet's file type assumed "the path is a
  unique identifier (namevar), while that's not true when Mounts are involved: there are
  actually two /x directories"; fixed by reverting the autorequire, not by modelling the mount
  [A-puppet-pup6397-mount-path-not-unique-2016]. Puppet's uniqueness key for a path differs
  between compile and apply on a trailing slash, so two resources pass compilation and crash at
  apply, unresolved [B-puppet-pup6770-uniqueness-key-trailing-slash-2016].
- Label and referent conflated. Salt requisites "match on both the ID Declaration and the name
  parameter", so a state whose name equals another's ID resolves to itself ("Recursive requisite
  found", stale-closed) and `require_in` silently dropped an edge on the same collision, fixed
  in 2021 [B-salt-issue-5667-name-treated-as-id-2013]
  [B-salt-issue-59922-requisite-in-id-name-conflict-2021]. Puppet's `host` type keyed by hostname
  cannot express `127.0.0.1` and `::1` both named `localhost`, closed Won't Fix
  [B-puppet-pup1928-host-key-not-unique-2014].
- Composition refused rather than modelled. Two Forge modules that each need `Package['foo']`
  cannot coexist; the "constraints" proposal, a module stating a need about a resource it does
  not own, never shipped, and the ticket has been accepted and unresolved since 2014
  [A-puppet-pup1968-duplicate-resources-forge-modules-2014]. Puppet's autorequire, the most any
  CM tool did toward a cross-type table, is "ensure-blind", so deletes cycle, also since 2014
  [B-puppet-pup2451-autorequire-absent-cycle-2014].
- The store's own tokens turning out unstable. btrfs subvolumes share a device number while
  reusing inode numbers, `find` aborts on the duplicates, and the kernel discussion concluded
  "with its current framing the problem is unsolvable"; the adopted uniquifier "still is not
  guaranteed" unique [B-lwn-btrfs-inode-number-epic-problem-2021]
  [B-lwn-btrfs-inode-number-epic-solutions-2021]. overlayfs documents that st_ino and st_dev "can
  change over the lifetime of a non-directory object" and that a copy-up "will 'break' the link"
  [A-linux-overlayfs-documentation-2025]. FUSE "does not have to guarantee uniqueness" of st_ino
  [B-libfuse-high-level-api-use-ino-2025]. NixOS exhausted its static uid range and moved uids
  to a mutable map that a restore without `/var/lib/nixos` silently mismatches
  [A-nixos-rfc0052-dynamic-ids-2019]; Nix's two-glibc case put one output identifier on two
  objects across two stores [A-nixos-rfc0062-content-addressed-paths-2019].
- Absence and applicability mis-partitioned. OVAL's `not applicable` was "never completely
  thought out", interpreters disagree whether an `rpm_object` on a Debian host is not applicable
  or does not exist, and the result is "overwhelmed in all cases" when combined
  [B-oval-issue-301-not-applicable-result-2018]. systemd's `is-active` on a missing unit exited 3
  until a 2022 change made it 4, while stdout still prints `inactive`
  [B-systemd-issue-25680-is-active-nonexistent-2022]. cloud-init's identity check "will sometimes
  fail to determine the current instance ID" when the metadata service is flaky, "which makes it
  impossible to determine if this is an instance's first or subsequent boot"
  [A-cloud-init-first-boot-determination-2026].
- Which referent a path names, decided differently by tools that meet on one host. OVAL's
  symlink debate framed it as preserving an "illusion" versus "switching files" and settled on a
  separate test yielding only the terminus [B-oval-issue-107-symlink-target-2013]; InSpec flipped
  its own reading in 2016 [B-inspec-issue-665-symlink-permissions-2016]; OVAL and Goss stat the
  link, InSpec's transport and osquery report the target
  [A-train-file-follow-symlink-default-2026] [A-osquery-file-table-implementation-2026]
  [A-goss-gossfile-reference-2026]. AppArmor "identifies files by name rather than by label",
  admits "It is unclear at this point how AppArmor should support separate namespaces", and
  denies disconnected files outright [A-apparmor-technical-documentation-2007]; the label camp's
  `setfiles` still resolves a hard link matching two specs by "the last matching specification",
  with a warning [B-selinux-setfiles-man-2026].
- In-file identity. Augeas numbered nodes are "the odd-man out" for idempotence and a value
  predicate "can't be used to create new entries" [B-augeas-issue-68-idempotent-changes-2013]
  [B-puppet-augeas-resource-tips-2017]; CFEngine's anchored insert "will reverse the order of the
  lines and will not converge", and a promise that deletes its own anchor loses its region
  [A-cfengine-insert-lines-reference-2024] [A-cfengine-edit-line-reference-2024].
- An erroring call is not a no-op. SibylFS found FreeBSD replacing a symlink with a new file on
  a call that returns ENOTDIR, breaking POSIX's invariant that an error "should leave the
  underlying file system state unchanged" [A-sibylfs-posix-oracle-2015].

### § 3.4-the-retreats

Systems and papers that reached the composition problem and withdrew, by name, so the pattern
is on record.

- CIM's `Correlatable` qualifier, the schema's only cross-namespace sameness generator, removed
  in v3 with "No replacement" and no rationale [B-dmtf-dsp0004-cim-metamodel-3-0-2014].
- The IETF SACM information model, expired 2017, its working group concluded
  [B-ietf-sacm-information-model-draft-2017].
- OpenConfig's operational-state rationale: "It is not clear what to do when the intended and
  applied configuration differ. The proposal made in this document makes no presumption"
  [B-openconfig-opstate-draft-2015]; NMDA's answer is an `origin` tag and a concession that the
  relationship between branches "is not machine readable" [A-ietf-rfc8342-nmda-2018].
- Couch and Chiarini: "the problem of statically determining whether a particular set of
  distributed operators share a fixed point is intractable", replaced by observation attributed
  to the observing agent [A-couch-chiarini-dynamic-consistency-convergent-operators-2008]. Couch's
  closures: consistency "is only nontrivial when two closures share a resource or parameter. The
  exact nature of that sharing is yet to be determined"
  [A-couch-hart-idhaw-kallas-seeking-closure-open-world-2003].
- Puppet's Resource API "will not implement support for multiple providers at this time" and
  floats, unbuilt, "allowing definitions to declare (partial) equivalence to other definitions"
  [A-puppet-resource-api-specification-2023]. Grapevine never built general change notification
  [A-birrell-levin-needham-schroeder-grapevine-1982]. Windows retired registry reflection
  [A-microsoft-wow64-registry-keys-affected-2022].
- Kent, on discovering that two representatives are one thing: "I don't know of any modeling
  system which can cope with that adequately" [B-kent-data-and-reality-2000].
- Mancoosi, to a DSL [B-mancoosi-d21-system-configuration-metamodel-2009].

### § 3.5-trails-still-moving

Open litigation the lanes touched and did not register, worth a look when the collaboration
sitting happens. Locations are in the lanes' leads sections.

- Crossplane issue #5918 and crossplane-runtime PR #850: the leaked-resource rule failing under
  asynchronous providers, open 2024 to 2026 (`gather-api-resource-identity-rfcs.md`).
- OVAL board discussion #261 (the `chainlink` proposal, unratified) and the OVAL 6.0
  definition-structure vote of 2024-11 (`gather-assessment-languages-and-verifiers.md`).
- DSC v3's promise that "In a future release, DSC will raise an error" on conflicting instances,
  where today "later instances override" [A-microsoft-dsc-v3-resource-properties-2025].
- Terraform's resource identity, introduced 2025, and OpenTofu's clean-room reimplementation
  (`gather-api-resource-identity-rfcs.md`).
- NixOS/nix issue #9259, coarser derivation equivalence (`gather-cm-schema-and-module-systems.md`).
- Puppet's "constraints" thread on puppet-dev, the unshipped answer to two modules and one
  resource (`gather-cm-big-four-resource-semantics.md`; needs a human to load Google Groups).
- The Google AIP issues on why `uid` is needed on declarative resources (#1580) and aliases
  versus normalization (#1485), both open.

## § 4-aid-attribution-and-observability-the-siblings

Dorc's aid plane (root `AID-NEEDS.md`) must tell people what the engine declined, why, and
whose statement each answer rested on, with each link's epistemic tier a typed field. The survey
was scoped to identity, but identity litigation is exactly where prior systems' error handling is
most visible. This section is organised by what the aid plane needs, so that each sibling is
filed beside the row it can inform.

### § 4.1-cant-say-partitions

The battle-tested partition of "no answer" has a closed-world marker, separates absent from
not-collected from error from not-applicable, and carries each up through truth tables.

- OVAL does it on three layers with MUST-level truth tables: a collected object's six-valued
  flag (`error`, `complete`, `incomplete`, `does not exist`, `not collected`, `not applicable`),
  an item's or entity's four-valued status, and a six-valued result. The strongest piece is
  `incomplete` as the closed-world marker: "It cannot be assumed that no additional matching
  OVAL Items exist on the system", so a none-exist test may conclude only under `complete`
  [A-oval-language-specification-5112-2016]. The tracker's own gloss: "In the absence of errors,
  and ignoring bugs, whatever the scanner found must be interpreted to be all there was to find"
  [B-oval-issue-83-check-existence-2013]. The weak point is `not applicable`, which never
  surfaces through a conjunction [B-oval-issue-301-not-applicable-result-2018]; XCCDF moved
  applicability up a layer and carries a nine-valued result that separates `error`, `unknown`,
  `notapplicable`, and `notchecked` [A-nist-ir7275r4-xccdf-12-2011].
- NSS separates `notfound` ("The lookup succeeded, but the requested entry was not found") from
  `unavail` (the source unreadable or unreachable) [B-glibc-nsswitch-conf-man-2026], and nscd
  keeps a negative cache with its own TTL [B-nscd-man-2026]. DNS enumerates four exclusive
  states of a name, separating "exists, but has no associated RRs" from "does not exist at all"
  [A-rfc2181-dns-clarifications-1997]. SMF has a `legacy_run` state for instances that "might or
  might not be running" [A-solaris-smf-man-2011].
- Unreachable is not absent, first-party: a 4xx response means "the nature of the resource is
  unknown" [B-w3c-tag-httprange14-resolution-2005]; V's clients "have no way of distinguishing
  such names from names bound by a manager that is temporarily down or unavailable"
  [A-cheriton-mann-decentralizing-global-naming-1989].
- OpenSCAP's systemd probe enumerates the unit-file catalog and then matches, so a nonexistent
  unit yields no item and the object flag reads `does not exist` rather than `ActiveState=inactive`;
  a D-Bus failure reads `error` online and `not collected` offline
  [A-openscap-systemdunitproperty-probe-2026]. systemctl's own manual says its LSB exit-code
  mapping "is imperfect, so it is better to not rely on those return values but to look for
  specific unit states and substates instead" [A-systemctl-man-2026].

These are the first-party precedents for `Research/GOTCHAS.md`'s `unreachable-is-not-absent` and
`a-read-folds-absent-into-a-value`, and the nearest siblings of the verdict tier's rc partition.

### § 4.2-folding-absence-the-anti-pattern

The tools that fold absence into a value are as instructive as the ones that partition it, since
an oracle author reading them decides whether a tool's output can carry an absence fact at all.

- osquery drops the row on any failed `lstat`, whether the cause is ENOENT, ELOOP, or EACCES;
  its `file` table can return a few thousand of two million files with no completeness flag, and
  the report was closed WONTFIX; the maintainers concede an empty column means "there's
  effectively no data to be returned", "a bug", or "platform skew", visible only as a verbose-log
  cast warning [A-osquery-file-table-implementation-2026]
  [B-osquery-issue-7306-file-table-incompleteness-2021] [B-osquery-issue-6319-empty-column-casting-2020].
- InSpec folds any nonzero `dpkg -s` into "not installed" (missing, purged, locked, and
  binary-missing alike) [A-inspec-package-resource-source-2026], and its canonical custom-resource
  example raises `ResourceSkipped` when a config file is absent, turning absence into
  not-applicable [A-inspec-custom-resources-docs-2026]. Testinfra enumerates systemd's exit code 4
  ("no such unit") and then returns `rc == 0`, so a missing unit reads as not running
  [A-testinfra-service-module-2026]. Ansible's file module folds a `PermissionError` into `absent`
  [A-ansible-builtin-file-module-code-2026].
- The contrast case: ohai's `shard_seed` raises rather than defaulting when DMI is unreadable
  [B-ohai-shard-seed-plugin-2016], while Facter's `uuid` reads the DMI product UUID with no clone
  caveat [A-facter-core-facts-schema-2026].

The folds happen at the rendering layer (systemd prints `inactive` for a unit that does not
exist) or in the tool's exit-code contract (Testinfra); the verdict tier's structural defense is
that anything at or above 2 is can't-say and can't-say always runs.

### § 4.3-naming-the-other-author

Where a system detects a conflict between authors, the deployed norm is to refuse and name.
"Refuse both and attribute both" has precedent.

- Kubernetes server-side apply refuses a conflicting apply and names the other manager; the
  manager field exists so that conflicts are attributable ("especially useful on conflicts!")
  [A-kubernetes-server-side-apply-2025].
- NixOS's merge error lists each definition with its file ("In `file.nix': 13 / In `custom
  place': 42") [A-nixos-manual-option-definitions-2026]; DSC's compiler names the differing
  non-key property ("Resources have identical key properties but there are differences in the
  following non-key properties") [A-microsoft-dsc-single-instance-resource-2020]; Puppet's
  catalog refuses with "Duplicate declaration: %{resource} is already declared; cannot redeclare"
  and an alias conflict names both parties [A-puppet-resource-catalog-duplicate-alias-code-2026];
  Guix names the duplicate `/etc` entry and refuses early [A-guix-services-scm-source-2026];
  tmpfiles applies the earliest file and logs "All other conflicting entries" as errors
  [A-systemd-tmpfiles-d-man-2026].
- Promise theory's rule for conflicting promises is refuse-both: "If different agents issue
  conflicting promises at least one of these fails to have the expected degree of control. In
  any case once such conflicting promise are noticed the trust in both issuers needs
  reconsideration" [B-bergstra-burgess-static-theory-of-promises-2014]. SMF's `in-conflict` tag
  is the same rule deployed [A-solaris-smf-man-2011].
- Attribution as a design principle rather than a feature: "Agents that reach conclusions based
  on comparisons that are not licensed by the relevant specifications take responsibility for
  any problems that result" [A-w3c-architecture-of-the-www-vol1-2004]; and in the observed-not-declared
  school, "the agent discovering the inconsistency is exactly the agent whose operators should
  change to react to it" [A-couch-chiarini-dynamic-consistency-convergent-operators-2008].

### § 4.4-origin-and-provenance-tags

The closest sibling of the aid plane's tiered links (measured, vouched, claimed, derived,
consented) is a provenance tag on every value; the closest sibling of the why-chain is a
per-object-type action history.

- NMDA marks every value that flows into the operational datastore with an `origin` (`intended`,
  `system`, `learned`, `default`, `unknown`), and specifies that a learned value reports
  `learned` "even when a learned value is the same as the configured value"
  [A-ietf-rfc8342-nmda-2018]. DNS ranks data by provenance ("Data from a primary zone file, other
  than glue data" first) and forbids merging response RRs into cached RRsets
  [A-rfc2181-dns-clarifications-1997].
- RETRO keeps an action history graph with one repair manager per object type, each naming
  objects in its own scheme, and refines to finer objects only where the manager "mediated all
  modifications to the larger object" [A-kim-retro-selective-reexecution-2010]; BackTracker's
  dependency graph tracks a file across renames by inode and version
  [A-king-chen-backtracking-intrusions-2003]. The provenance community has no agreed
  representation even for a rename: SPADE uses two nodes, CamFlow attaches the new path to the
  file object and drops the old [A-chan-cheney-provmark-provenance-expressiveness-2019].
- Attribution that got acted on: FSMove reports each missing dependency or notifier with the
  producing and consuming blocks, and sixty-two of ninety-two reports were confirmed and fixed
  upstream [A-fsmove-puppet-fault-detection-2020]; Citac attributes each state diff to a step by
  strace snapshot [A-citac-reliable-convergence-2016]; Bcfg2's two-way validation reports
  "extra entries", the residue outside the specification, by heuristic
  [B-bcfg2-architecture-client-2013].
- Who decides whether a difference is a change: Terraform requires the provider to distinguish
  "Normalization" (different form, same meaning) from "Drift" (materially different)
  [A-terraform-resource-instance-change-lifecycle-2024]; the AIPs expose "the comparison method
  used to accurately compute if two values should be considered equal" rather than the
  normalization itself [A-google-aip-129-server-modified-values-2023].

### § 4.5-witnesses-and-freshness

The model's standup witness (re-read through the same entry and compare) has siblings in every
system that keeps identity across time without a central invalidation channel.

- IF-MIB detects counter discontinuities "without the ifIndex value of the interface being
  changed", and treats a `sysUpTime` reset as the signal that every index may have been
  reassigned [A-ietf-rfc2863-if-mib-2000]. HOST-RESOURCES-MIB carries a last-update scalar so a
  manager can "obtain a guarantee that no data in this table is older than the indicated time"
  [A-ietf-rfc2790-host-resources-mib-2000]. SMI classifies persistence per row (`volatile`,
  `nonVolatile`, `permanent`, `readOnly`) and separates a row that "exists in the agent, but is
  unavailable for use" from one in service [A-ietf-rfc2579-smiv2-textual-conventions-1999].
- V detects stale resolutions on use at the authority ("the manager recognizes that the
  directory's name does not match the name provided in the client's request, and returns an
  error") [A-cheriton-mann-decentralizing-global-naming-1989]; cloud-init's check mode compares
  the cached instance identifier against the one read at runtime
  [A-cloud-init-first-boot-determination-2026]; a stale file handle reads ESTALE after the inode
  is reissued [A-linux-name-to-handle-at-man-2026].
- Kubernetes makes `resourceVersion` opaque and warns clients not to "assume that the resource
  version has meaning across namespaces, different kinds of resources, or different servers"
  [A-kubernetes-api-conventions-identity-sections-2026]; Plan 9 keeps `qid.version` as a state
  token distinct from the identity triple [A-plan9-from-bell-labs-overview-1995].
- Terraform's unknown values obey a monotone refinement law: a known value in the initial plan
  "must have an identical value in the Final Planned State", an unknown may become any value of
  its type, and "No unknown values are permitted in the New State"
  [A-terraform-resource-instance-change-lifecycle-2024]. That is the nearest deployed sibling of
  a placeholder bound at standup and of "partial measurement never widens".
- Chef's `identity` exists so a resource can be "correlated across Chef runs" and its
  `load_current_value` distinguishes "the actual value legitimately does not exist" from an
  unfilled object [A-chef-resource-rb-identity-provides-code-2026]
  [A-chef-rfc056-load-and-converge-2015]; CFEngine locks a kept promise by a hash of "promiser,
  associated attributes, and context" [A-cfengine-promises-reference-2024]. The classics: GNS
  bounds a cache by expiry and DNS "does not call for explicit invalidation of changed entries.
  Instead, it uses expiration" [A-lampson-designing-a-global-name-service-1986]
  [A-saltzer-kaashoek-system-design-naming-2009].

### § 4.6-fail-closed-with-a-marker

Where a system cannot decide safely it stops, leaves a human-visible marker, and offers the type
author an escape. That is the deployed shape of withhold plus hint plus author decline.

- Crossplane: "The safest thing for a provider to do when it detects that it might have leaked
  a resource is to stop and wait for human intervention", with an annotation the human clears
  and a per-type `WithDeterministicExternalName` declaration as the escape
  [A-crossplane-managed-resources-external-name-2026] [A-crossplane-runtime-managed-reconciler-2026].
- Terraform raises "Unexpected Identity Change" and offers the provider a `MutableIdentity`
  opt-out [A-terraform-plugin-framework-resource-identity-2025]. Nix's first implementation of
  one-realisation-per-store "just forbids fetching a path if the local system has a different
  realisation for the same drv output", called "simple and correct" and possibly "not
  good-enough in practice" [A-nixos-rfc0062-content-addressed-paths-2019].
- Debian's conffile protocol has exactly one interaction moment, when both parties changed the
  file [B-debian-policy-conffile-handling-appendix-2025]; DEP-17's response to file loss was a
  moratorium [A-debian-dep17-usrmerge-aliasing-2023]. NixOS's `stateVersion` documentation is a
  marker in prose: "Do not change this value unless you have manually inspected all the changes
  it would make to your configuration" [A-nixos-state-version-option-2026].
- Prodspec refuses turndown-by-absence because "The most common failure mode of configuration
  systems is returning partial content" [B-palatin-prodspec-annealing-intent-based-actuation-2021].

### § 4.7-status-is-not-speech

`Research/GOTCHAS.md`'s `nonzero-status-is-not-speech` has first-party precedent, and every CM
tool's dry-run trusts a per-resource simulation with no model of prior simulated writes.

- OVAL's `shellcommand` reports every run as `exists` and sets `error` only when the content
  author opts in with `error_if_exit_status_not_0`; the implementers "do not have enough
  knowledge of the intent of the command to know if it constitutes marking the item as 'error'"
  [A-oval-independent-definitions-schema-2026] [B-oval-community-shellcommand-status-discussion-2025].
  OVAL's only effects model is a warning to authors to "DO NO HARM".
- Salt: "Test mode does not predict if the changes will be successful or not"
  [A-salt-writing-state-modules-return-contract-2026]. Ansible: "Check mode is just a
  simulation", modules without support "report nothing and do nothing", and the file module
  warns "Create user up to this point in real play" [A-ansible-checkmode-diffmode-doc-2026]
  [A-ansible-builtin-file-module-code-2026]. Chef's `:before` notification and Salt's `prereq`
  both trust a per-resource why-run as a prediction of change
  [B-chef-rfc058-before-notification-whyrun-2015] [A-salt-requisites-doc-2026]; Puppet's noop
  suppresses refresh and only logs "what would have happened" [B-puppet-core-lang-relationships-2026].

The probe phase's refusal to simulate (a probe measures a fact, never dry-runs the mutator)
is the structural answer; the survey shows no CM tool takes it.

### § 4.8-validators-beside-contracts

For the record and not as a recommendation: every survey system that placed obligations on
describers shipped a validator with the obligation. Puppet's `--strict` checks the
`canonicalize` fixpoint [A-puppet-resource-api-specification-2023]; Windows Installer's component
rules are ICE-enforced at build [A-msi-organizing-applications-into-components-2021]; Terraform
errors on an identity change [A-terraform-plugin-framework-resource-identity-2025]; OVAL content
validates against its schemas. Dorc's posture (the human, typed 2026-09-26) is U-shaped: the tool
must behave well in the complete absence of tooling, on pathological targets, before any
engineering effort goes to behaving better on sane ones. `dorc lint` is the existing home when
that turn comes.

## § 5-contradictions-and-tensions

The lanes surfaced seven places where deployed prior art contradicts a rule or an example of the
model. Each is disposed here; none needs a sitting.

- `ten-split-dns-against-the-root-example` (~SUSPECT). The DNS terminology BCP says a domain
  name "that is notionally globally unique has different meanings for different network users"
  [A-rfc9499-dns-terminology-2024], and the model's parent section still offers the DNS root as
  an example of a globally comparable terminus. `notes/311t` §3 already retired DNS-as-container.
  The example is stale at the footer level; the rule is untouched.
- `ten-kent-against-qualified-identification` (-GUESS dissolves). Kent argues that qualified
  identification smuggles a relationship into every reference and needs "Invariance of
  Qualifiers", or "enormous update anomalies" follow [B-kent-data-and-reality-2000]. His three
  conditions (uniqueness within the qualifier, existence of the qualifier, invariance of the
  qualifier) are the preconditions the model's parent edge needs, and the third is what the
  model handles by invalidation on a routing or lifecycle write rather than by storage. Dorc
  stores nothing and re-measures (`KNOBS:kSTATE`); the anomaly Kent fears is a row going stale.
- `ten-ontoclean-sortal-individuation` (not actionable). OntoClean hard-codes "Every domain
  element must instantiate some property carrying an IC"
  [A-guarino-welty-overview-of-ontoclean-2009]; the model makes warrants absent by default. A
  genuine fork, but OntoClean is taxonomy hygiene where one author owns every entity, and the
  model's law (never a false answer while every statement behind it is true) forces the default
  the other way.
- `ten-couch-chiarini-observe-never-declare` (bears on value, not soundness). The field's two
  principal theorists concluded that declared consistency between distributed operators is
  intractable and replaced it with observed non-convergence
  [A-couch-chiarini-dynamic-consistency-convergent-operators-2008]. This is `DESIGN.md`'s second
  sensitivity argued from the other side. The model dodges the intractability by collecting only
  what each author volunteers and colliding by default; the survey neither strengthens nor
  weakens that bet (`KNOBS:kHALVES`).
- `ten-promise-theory-assumes-away-strangers`. Promise theory coincides with the committee law
  and then assumes non-interference via "overriding control"
  [B-bergstra-burgess-static-theory-of-promises-2014]. Corroborates the law, silent on the premise.
- `ten-yang-and-dep17-redesign-the-world`. YANG keys ordered server lists by "An arbitrary name"
  and carries order separately, falling back to positions only for keyless state lists
  [A-ietf-rfc7317-ietf-system-2014] [A-ietf-rfc7950-yang11-2016]; DEP-17 eliminates aliases.
  Both avoid positional and aliased identity by not having it, which is unavailable to a tool
  that takes the world as found. This is why the model's positional-catalog example exists at all.
- `ten-cpe-is-the-refuted-shape-deployed`. Evidence for the model rather than against it
  (1.1-two-independent-warrants-absent-by-default); listed because the outgoing ledger filed it
  as a contradiction.

## § 6-the-bad-news

The in-chat synthesis carried a longer list; the human's typed responses (banked in the research
dir's turn 2 log) pruned it and corrected its framing. What survives is recorded here with the
ruling or lean beside it. Nothing here is owed work.

- `bad-no-registry-is-the-price-carrying-constraint` (+SURE of the survey). Every system that
  achieved breadth has a registry, a single minting authority, or a curator (1-what-the-survey-corroborates,
  3.2-consensus-by-accretion). The model requires a stranger to identify into a shared key space
  before anything of theirs spares, so the stdlib's key spaces are the de-facto registry and
  filing into them is the price of survival. The human's ruling: no registry is welded out, for
  the short term, for base-case correctness, for the model's flexibility, against scope creep,
  and above all for security; a registry can be added and never removed; the model's rooting
  behaviour is the fail-safe that makes the weld livable (a describer may live in their own
  corner and collide with everything). The survey adds only that the price is real and that
  nobody has written down who pays it.
- `bad-the-ramp-has-a-cliff` (+SURE of the texts). Every system in the survey has a cliff
  between the local user and the shared library (local manifests versus the Forge, a flake
  versus nixpkgs, a shell task versus a module) and none pretends otherwise. Under the model the
  step from an in-book verdict function to any cross-vocabulary survival is learning the model.
  `KNOBS:kBURDEN`'s no-cliff principle survives only if the concretization hides the structure
  behind stdlib defaults, and no such defaults exist. The human: acked; owed to the spelling
  sittings; "how to extract value at low authorship quantities" is unsolved and may be
  unsolvable under the model.
- `bad-theory-unvalidated-and-failures-unmodelled` (+SURE of both halves). The validated
  systems carry the flattest identity models and the rich theories were never validated
  (`gather-academic-theory-of-system-administration.md`); and the litigation record
  (3.3-the-litigation-record) is a catalogue of real failures from a poor or forgotten identity
  model. The human's read is the second half: most of the world's failures are identity
  failures, and a rich model is suspected to be a net win for usability and stability, modulo
  the cliff. The honest picture is that the field has oscillated between no model and paying in
  bugs, and a rich model that never shipped; nobody has shipped one. The model's refutations so
  far are thought experiments; its next refutation must come from outside, and nothing outside
  exists yet (zero stdlib oracles; the adequacy bite open). The lanes' breadth tables against the
  forty-seven items of `notes/311r` show sysctls, containers, firewall rules, cron, certificates,
  SQL, and LVM uncovered by nearly all prior art: a free, unwalked falsification set.
- `bad-reconciliation-labor-has-no-home` (+SURE of the survey, -GUESS of the size). Where
  strangers' vocabularies met at scale, reconciliation was human curation at scale
  (3.2-consensus-by-accretion). The model's mechanism, one stranger's scheme yielding into
  another's, is the same one, per tool, per pair, with no curator. The human: acked; a
  collaboration-inside-the-model discussion is pending.
- `bad-one-ruling-generates-the-weight` (+SURE). The typed ruling that mutually unaware speakers
  doing ordinary things must never yield a wrong elision is what the walk and both closures
  serve. Every battle-tested system assumed strangers' things disjoint by default and paid
  occasionally and visibly (3.3-the-litigation-record). Nobody has paid the model's price, so
  nobody knows if it can be paid. Its cost lands on the flagged tier, which `USER_STORY.md`
  admits nearly everyone will turn on and forget; the product's real mode is the heavy one.
- `bad-the-flag-absorbs-unsolved-corners` (+SURE of the accretion). The consent flag now gates
  more than survival. The survey's opt-in-to-risk flags all became defaults in practice
  (cloud-init's trust mode [A-cloud-init-first-boot-determination-2026]; Puppet's non-isomorphic
  types [A-puppet-type-rb-isomorphism-autorelation-code-2026]). The human: known; "flagged" now
  often glosses somehow-configurable and the grain is open; the lean stays toward one flag,
  because the dangers are subtle and no accessible split is expected to help a mid-experienced
  admin. An inventory of what the flag gates, in one place, is the cheap record.
- `bad-the-admin-seat-is-unexplored` (~SUSPECT). Every real identity bug in the survey was
  reported from the operator's end, by someone with the world in front of them
  (3.3-the-litigation-record). The human: admin versus oracle author is a gradient, treated as a
  binary only for design-forcing reasons; the admin's identity seat is a tiny oracle, a
  load-path override, and the like; how they use it and how ergonomic it is are unexplored.
- `bad-deletes-were-never-sat` (+SURE). The survey's destruction cases are both wrong-DISJOINT
  on a delete (DEP-17, the MSI component rules), and the most-litigated CM tool's cross-type
  table is ensure-blind with delete cycles open for a decade. The model files deletion as a
  routing write; the product half (what a plan with a delete in it looks like, whether absence is
  ever converged) is `TODO-ADDTL.md`'s "deletes-are-hard, sitting-visible, never sat".
- Leans, each a lean and not a finding. `bad-time-is-a-horizon` (--WONDER): every survey system
  with a live host made time a first-class axis of identity (4.5-witnesses-and-freshness), and
  the systems that ignored it were verification tools; the model puts time on the outside-churn
  horizon and re-measures, consistent with `KNOBS:kSTATE`. `bad-versioning-looks-less-deferrable`
  (-GUESS): Solaris's package scheme is universal only under version conventions, Terraform
  treats a version mismatch as inequal, and half the package-domain litigation is versions
  (`ROADMAP.md`'s `design-mh2-version-layer`). `bad-observer-dependence-will-cargo-cult`
  (~SUSPECT): dependent-by-default is the honest choice, User is the one observer every describer
  meets, and independence-of-User will be declared reflexively until it is wrong once.
- A process lesson. The research question was scoped to identity. The finding the human acked
  hardest, that the best-matched academic project went declarative, arrived as an unseeded find
  in a lane briefed on verification tooling. Next time, ask the product question directly.

## § 7-what-would-change-a-conclusion

- No further breadth research for this question (~SUSPECT). Ten lanes, several hundred graded
  sources, the counter-thesis held ten times. More breadth adds citations and moves no shape.
- A narrow primary-read set could change something. RETRO §§4.4 to 5.3, whose refinement rule
  (finer objects only where the manager mediated all writes to the coarser one) rhymes with the
  open hold on cells and entries under one parent [A-kim-retro-selective-reexecution-2010]. Ntzik
  and Gardner 2015, whose frame rule may or may not cover the alias-above-the-leaf case the
  region test walks [A-ntzik-gardner-posix-fusion-logic-2015]. RFC 8881's identity clauses, to
  settle whether `unique_handles` is per-store unique-name (the conductor's -GUESS) or a
  precedent for the store's self-knowledge closure (the outgoing ledger's ~SUSPECT)
  [A-ietf-rfc8881-nfs41-fileid-fsid-attributes-2020]. SMI-S Clause 7 as the nearest deployed
  compare [A-snia-smis-correlatable-durable-names-2020]. Under a hundred pages in all. The long
  confirmatory reads (the systems textbook, OntoClean, the CIM infrastructure specification) are
  not worth the sitting.
- One seam for a later lane, for the footprint model rather than identity: the hand-litigated
  footprint corpora in the wild (SELinux reference-policy file contexts and interfaces, AppArmor
  abstractions, pledge's subsystem table, the PaSh per-command annotations), which could size how
  large real at-most sets are. Flagged by two lanes; not opened, by ruling.
- Leads the lanes could not pull, carried from the ledger for a human with a browser: Couch and
  Sun 2004 on observed reproducibility (paywalled; the closest thing in the theory seam to an
  observer model); Comer and Peterson 1989 and its 1984 Purdue precursor; Needham's "Names"
  chapter; Ntzik's 2018 concurrent specification and the technical report with full symlink
  rules; BuildFS; Lepiller's intra-update sniping paper; Chen's 2020 configuration-dependency
  taxonomy and CfgNet; the Puppet constraints thread; MultiarchSpec; the ServiceNow reconciliation
  engine's documentation (robots.txt refuses fetch tools); 3GPP TS 32.300.

## § 8-the-lanes-and-the-manifest

The evidence base is `.claude/research/ops-extern-model-prior-art/`. At close: two hundred
fifty-five manifest entries and two hundred fifty-five archived copies, one to one;
`validate.sh` reports zero unregistered citations. Each lane's report carries Findings, a
candidate table judged on the four criteria and the forty-seven-item breadth yardstick, verbatim
citations with archive locators, leads not pulled, and a search log. By yield:

- `gather-wildcard-and-counter-thesis.md` (lane 10): the four punt-shapes; DEP-17; SMI-S; RETRO
  and BackTracker; the seams-to-redraw list.
- `gather-naming-theory-classics.md` (lane 9): the classics-to-model mapping table; the two
  warrants' classical statement; the counter-thesis split for interference.
- `gather-academic-verification-of-configuration-code.md` (lane 5a): every verifier's aliasing
  punt in its own words; SibylFS; fusion logic; Mancoosi.
- `gather-platform-specs-filesystem-packages-installers.md` (lane 6): RFC 8881's separated
  warrants; POSIX and the Austin defect; overlayfs; dpkg triggers; the package-versus-admin
  protocols; MSI; OCI; udev.
- `gather-platform-specs-services-network-names.md` (lane 7): Solaris FMRIs; RFC 4007; the DNS
  RFCs; the kernel's namespace translations; wrappers' lends; the systemd exit-code trail.
- `gather-standards-management-models.md` (lane 1): CIM's weak keys and the removed
  `Correlatable`; NMDA and `origin`; the IF-MIB litigation; ENTITY-MIB.
- `gather-api-resource-identity-rfcs.md` (lane 8): Crossplane; RFC 3986 §6 and AWWW; CPE; server-side
  apply; Terraform's unknowns; Repology.
- `gather-assessment-languages-and-verifiers.md` (lane 2): OVAL's partitions and the `chainlink`
  thread; osquery's folds; which tools stat the link.
- `gather-cm-big-four-resource-semantics.md` (lane 3): the Puppet tracker litigation; autorequire;
  Salt's conflation; the RFC repositories' silence.
- `gather-cm-schema-and-module-systems.md` (lane 4): the Nix store; NixOS and Guix merge rules;
  DSC's singleton; inmanta's declared index; Augeas and CFEngine on in-file identity.
- `gather-academic-theory-of-system-administration.md` (lane 5b): the operator algebras; Burgess
  and Couch's accessor-equality definition; promise theory; the flat-models-validated finding.

Manifest hygiene carried from `conductor-ledger.md`, recorded and not fixed: the TxOS paper was
registered twice by two lanes and the human ruled one grade, so lane 10's A-grade entry stands
and lane 5a's citations were re-pointed; the Nix store-object entry's `via` names the GitHub MCP
tool where the real call was `gh api`; the OVAL language specification is a `.docx` archived
under an `.html` extension, cited by section beside its line numbers; lane 2's six citation
placeholders (`L{…}`) were never resolved to archive lines. The skill's `new-source.sh` gained a
download timeout and a manifest lock during this round, narrow and exercised; the lock is not
FIFO.
