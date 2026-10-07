# gather-naming-theory-classics — the classical theory of naming, binding and identity

Lane 9 of 10. 311 read at `63e49f29` (`git rev-parse --short HEAD`, 2026-09-26). All grades `graded-by: subagent`.
Page cites are the source's printed page numbers (PDF archives); line cites are for plain-text archives.

## Findings

- The classical systems theory supplies a closed, battle-tested vocabulary that covers most of 311's naming layer: naming scheme (name space + name-mapping algorithm + universe of values), `resolve(name, context)`, `bind`/`unbind`/`enumerate`/`compare`, default vs explicit context reference, qualified name, path name, root, search path, synonym, indirect name, unique identifier name space, stable binding, dangling reference, lost object, closure. Mapping table below. [A-saltzer-kaashoek-system-design-naming-2009] [A-saltzer-naming-and-binding-of-objects-1978] +SURE
- The classics already state 311's two warrants as two INDEPENDENT uniqueness rules chosen per naming scheme, with neither holding by default: "Some naming schemes have a rule that a name must map to exactly one value in a given context and a value must have only one name, while in other naming schemes one name may map to several values, or one value may have several names, even in the same context." A unique identifier name space adds "never be reused... once bound, will always remain bound to the same value" (a stable binding), and only with "a value can have only one name" do its names become useful "for comparing references to see if they are to the same object". [A-saltzer-kaashoek-system-design-naming-2009] +SURE
- The textbook says outright that comparing names cannot decide sameness. Its `compare` has three meanings: same name, bound to the same value, same stored contents. It says: "Unless one has some specific understanding of the underlying physical representation, the only way to distinguish the two cases may be to change the contents of one of the named storage containers and see if that causes the contents of the other one to change. ('Kick this one and see if that one squeals.')". That is a probe-by-write for interference, which the book offers as a last resort and leaves outside its model. [A-saltzer-kaashoek-system-design-naming-2009] +SURE
- Ontology engineering independently splits identity criteria into SUFFICIENT conditions (Γ → x=y) and NECESSARY conditions (x=y → Γ), and asserts each separately ("weak ICs, which are (only) necessary or (only) sufficient"). A sufficient criterion licenses SAME, which is ≈ `:guarantees-unique-referent`. The contrapositive of a necessary criterion licenses DISJOINT, which is ≈ `:guarantees-unique-name`. OntoClean keeps this: sharing essential properties "can be used to make conclusions about non-identity, if not about identity". This is the strongest outside precedent for 311 §1.5's two independent warrants. [A-guarino-welty-identity-unity-individuality-2000] [A-guarino-welty-overview-of-ontoclean-2009] +SURE
- WHERE 311 DIFFERS FROM ONTOLOGY: OntoClean HARD-CODES "Sortal Individuation. Every domain element must instantiate some property carrying an IC", and that every entity has "a unique most general property carrying a criterion for its identity". 311 refuses this. Its MSorts are floor/un-named by default, warrants are absent by default, and strangers' MSorts meet as KNOWN_UNSPOKEN. OntoClean's +O (a property SUPPLIES its own identity criterion) vs +I (a property only CARRIES an inherited one) is close to 311 §2.2, where only the primary MScheme's owner declares identity and secondary MSchemes yield into it. The difference is that OntoClean uses it to police subsumption, and 311 uses it for attribution. [A-guarino-welty-overview-of-ontoclean-2009] ~SUSPECT
- The data-modelling classic names the two failure modes the warrants guard against: "If things have aliases, then equality will not be detected... If symbols can be ambiguous (name several things), then spurious matches will occur". It also records the PUNT that 311 refuses: an information system "will assume that two representatives represent two different things". And it denies the premise that every entity must have a unique identifier ("not an inherent characteristic of information"). [B-kent-data-and-reality-2000] +SURE
- A TENSION with 311: Kent argues against qualified (weak-entity) identification. He says it smuggles a relationship into every reference and needs "Invariance of Qualifiers", or "enormous update anomalies" follow. 311 deliberately makes MKey-Primary meaningful only relative to its MParent-Store (a qualified identity) and relies on routing/lifecycle invalidation to bound the update problem. Kent's §3.3.2 conditions (uniqueness within qualifier, existence of qualifier, invariance of qualifier) read as the preconditions 311's MParent edge needs. [B-kent-data-and-reality-2000] ~SUSPECT
- COUNTER-THESIS, PARTLY REFUTED: naming theory does model interference and invalidation, though thinly. (a) Lampson's formal naming spec lists "Are there aliases, so that an update to one object can affect the value of others?" among the axes on which name-space specs vary. It also shows a non-atomic lookup returning a value that "the path name /a/x" never had at any instant, under a concurrent Rename+Write. [A-lampson-6826-naming-handout-2006] (b) Lampson's GNS bounds a cached resolution's validity by exactly the arcs its lookup crossed: "the result of a directory lookup can be safely cached until the minimum TX of any arc or link that was followed". GNS guarantees this by forbidding writes to those arcs before expiry, not by invalidating. [A-lampson-designing-a-global-name-service-1986] (c) V's naming detects stale resolutions "on use" by re-checking name against identifier at the authority, which is ≈ 311's `witness()`. [A-cheriton-mann-decentralizing-global-naming-1989] (d) Grapevine corrects stale hints by feedback. It records that general change-notification "to control resource bindings" was never built. [A-birrell-levin-needham-schroeder-grapevine-1982] ~SUSPECT
- COUNTER-THESIS, PARTLY CONFIRMED: no classic has a WRITE-SET or region model. Every treatment of interference either (i) freezes bindings (TX expiry in GNS, stable bindings / unique ids in Saltzer), (ii) detects staleness at use time, or (iii) punts to a human or to a fresh address space. Multics lets persistent links go stale with "no tables and backpointers" and fixes it with "a fresh address space". Plan 9's lexical names can be broken by "renamings and deletions made by one machine [that] may go unnoticed by others". Nothing in this literature answers 311's question "does this write reach the thing that license depends on". [A-saltzer-naming-and-binding-of-objects-1978] [A-pike-lexical-file-names-plan9-2000] +SURE
- Identity surviving moves and renames is a solved classic, in two styles. (1) Hide the address behind a stable name or unique id: Lampson's directory identifiers (DI), full names rooted at a DI, a link left behind when a subtree moves, and a "well-known table" in the new root so old roots stay resolvable. [A-lampson-designing-a-global-name-service-1986] (2) Resolve to a lower-layer unique id and compare those: NFS file handles carry an inode generation number because "If the server should happen to reuse the inode... client 2 will get the new file" (the recycled-MKey hazard). Plan 9 decides "same file" by (type, device, qid.path). [A-saltzer-kaashoek-system-design-naming-2009] [A-plan9-from-bell-labs-overview-1995] +SURE
- Recycled names are treated as a primary hazard. Saltzer's limited vs unlimited contexts: "In a limited context the names themselves are a scarce resource that must be... reused". The textbook: "Dangling references are nearly always a concern when the name space is limited because names from limited name spaces must be reused". Grapevine bans the case outright: "Recreation of a deleted entry is not allowed". This is directly 311 §3.3's `userdel alice; useradd alice`. [A-saltzer-naming-and-binding-of-objects-1978] [A-saltzer-kaashoek-system-design-naming-2009] [A-birrell-levin-needham-schroeder-grapevine-1982] +SURE
- Authority in federated name spaces is DELEGATION by hierarchy ("whoever operates a name server can be a naming authority... without asking permission of anyone else"). It includes authority over ABSENCE: an entity "covers a name if it authoritatively knows either the definition of the name or that the name is undefined". Unreachability is explicitly not absence. These parallel 311's committee law (each owner speaks about its own level) and its closures. But no classic treats cross-authority SAMENESS other than by a human or a transition-owner table. The one found is Plan 9's auth-server mapping "user rtm in one domain is the same person as user rtmorris in another", which is ≈ `:corresponds`. [A-saltzer-kaashoek-system-design-naming-2009] [A-cheriton-mann-decentralizing-global-naming-1989] [A-plan9-from-bell-labs-overview-1995] ~SUSPECT
- Plan 9 is the cleanest deployed "name resolves from a vantage" model. It has per-process name spaces, and `bind`/`mount` with replace/before/after union directories (a union directory replaces `$PATH`). There is no global name space; sanity comes from conventions. A process's name space is published as a replayable script at `/proc/n/ns` (≈ an MEntryChain made observable). Kernel Channels remember the rooted name used to reach them (≈ a lookup EMITTING its traversal). [A-plan9-use-of-name-spaces-1993] [A-pike-lexical-file-names-plan9-2000] +SURE
- Descriptive naming theory frames resolution as inference with SOUNDNESS vs COMPLETENESS, which are the same two failure directions 311 §0 separates. It uses per-attribute "closed" (closed-world) declarations ≈ 311's per-declaration closure sentinels. It excludes a candidate because "by definition a processor may have only one name" (a unique-name warrant used to conclude DISJOINT). [A-bowman-debray-peterson-reasoning-about-naming-systems-1993] ~SUSPECT
- Probabilistic record linkage (link / possible link / non-link, two bounded error types) mirrors SAME/UNKNOWN/DISJOINT structurally. But it is exactly the estimated, engine-generated SAME that 311 §4.1 excludes. It is a contrast, not a precedent. [A-fellegi-sunter-theory-for-record-linkage-1969] -GUESS
- BREADTH: the worked examples touch, of the 47 items, files/inodes/hardlinks/symlinks/mounts/chroot/rename (5, 6, 7, 23, 46 by analogy), DNS names and records (27, 40), hostnames (30), uids/login names (17, 18), pids (Plan 9 /proc), environment-variable search paths (37, 38), MAC addresses / interfaces (21, 22 via RFC 1498). Nothing touches packages, services, sysctls, firewall rules, containers, crontab lines or config-file entries. The theory is broad in ABSTRACTION but narrow in OPS DOMAIN. [A-saltzer-kaashoek-system-design-naming-2009] [A-saltzer-naming-binding-network-destinations-1982] +SURE

## Mapping: classical vocabulary → 311

| classical term (source) | 311 counterpart | fit / gap |
|---|---|---|
| naming scheme = name space + name-mapping algorithm + universe of values [A-saltzer-kaashoek-system-design-naming-2009] p.61–62 | MScheme (its `resolve()`) | close. 311 adds one accountable owner and per-shape declarations |
| `resolve(name, context)` [A-saltzer-kaashoek-system-design-naming-2009] p.63 | `resolve()` run in an MVantage, MKey looked up in its MParent-Catalog | close. The classics fold vantage and parent into one "context" argument; 311 separates them (MVantage is never part of identity) |
| context = partial mapping names→objects; catalog [A-saltzer-naming-and-binding-of-objects-1978] p.6, p.1 | MParent-Catalog | close |
| default context reference (working directory, default domain) vs explicit (qualified name) [A-saltzer-kaashoek-system-design-naming-2009] p.66–68 | ambient MParent supplied by MEntryChain vs the bind/yield seat supplying the MParent | close. "Many apparently puzzling problems in naming can be simply diagnosed: the name-mapping algorithm... used the wrong context reference" ≈ GOTCHAS same-name-different-referent-per-viewpoint |
| path name; recursion "must terminate somewhere, with... a single built-in context"; root [A-saltzer-kaashoek-system-design-naming-2009] p.67, p.72 | MFullyQualifiedKey terminating at MRoot / MRoute | close. Classics treat the root as unique per resolver; 311 makes each MRoot shape an MWorld and refuses cross-MWorld speech |
| universal name space ("a name always has the same meaning... no matter who uses it") [A-saltzer-kaashoek-system-design-naming-2009] p.62 | `:root` | close. 311 adds "Nothing is a MRoot by default" and cloned identifiers as the standing witness; classics do not discuss cloning |
| synonym / alias; indirect name (symlink, CNAME) [A-saltzer-kaashoek-system-design-naming-2009] p.72–73 | failure of `:guarantees-unique-name`; a `:yields` whose target is another name in the same scheme | close |
| hard link = binding to a lower-layer name (inode number) [A-saltzer-kaashoek-system-design-naming-2009] p.104–105 | path MScheme `:yields` the inode (§2.3 "dissolves it") | close. Saltzer 1978 footnote: a higher-level name bound to a lower-level one is NOT a synonym |
| unique identifier name space; stable binding; "value can have only one name" [A-saltzer-kaashoek-system-design-naming-2009] p.64 | MToken with `:guarantees-unique-referent` (+ `:guarantees-unique-name`) | close. Classics bundle "never reused" into the unique-id rule; 311 splits reuse off to the `witness()` horizon |
| limited vs unlimited context (names must be reused) [A-saltzer-naming-and-binding-of-objects-1978] p.21 | recycled MKeys (pid, inode) outrunning the unwalled span | close |
| search path / multiple lookup / union directory [A-saltzer-kaashoek-system-design-naming-2009] p.73–74; [A-plan9-use-of-name-spaces-1993] | a lookup crossing several catalogs; its emitted MTraversal | partial. Classics note the hazard ("the chance increases that two libraries will accidentally contain two unrelated programs") but have no notion of a traversal as an invalidation backing |
| user-dependent binding; closure [A-saltzer-naming-and-binding-of-objects-1978] p.11–12 | `:observer-dependence`; wrapper `:lends` | partial. Classics treat it as a feature to provide, not a qualifier on facts |
| unstable binding ("change unpredictably between definition and use") [A-saltzer-naming-and-binding-of-objects-1978] p.12 | routing invalidation of an MResolution | named, not modelled |
| dangling reference / lost object [A-saltzer-kaashoek-system-design-naming-2009] p.130 | lifecycle write; existence cell | partial |
| naming authority by delegation; "covers a name" incl. its absence [A-cheriton-mann-decentralizing-global-naming-1989] p.152 | committee law; closures (`alias nothing-else`, finished records) | analogous. Classics give authority over one's own subtree, not a law about what composes |
| directory identifier (DI) + full name rooted at a DI [A-lampson-designing-a-global-name-service-1986] p.3, p.9–10 | MKey-Primary of the directory's MParent-Store; MFullyQualifiedKey | close |
| cache valid until min TX of arcs followed [A-lampson-designing-a-global-name-service-1986] p.11 | MResolution backed by its MTraversal | close in shape. GNS forbids the write; 311 invalidates on it |
| (type, device, qid.path) "same file"; qid.version [A-plan9-from-bell-labs-overview-1995] | MFullyQualifiedKey; a state token distinct from identity | close |
| sufficient vs necessary identity criterion [A-guarino-welty-identity-unity-individuality-2000] | `:guarantees-unique-referent` vs `:guarantees-unique-name` | close, and independent in both |
| +O supplies vs +I carries identity [A-guarino-welty-overview-of-ontoclean-2009] | primary MScheme's owner declares identity; secondary MSchemes yield | close in spirit. OntoClean mandates a supplier for every entity; 311 does not |
| qualified name conditions (uniqueness within / existence of / invariance of qualifier) [B-kent-data-and-reality-2000] p.58–60 | MParent-Store edge carrying identity | close. Kent treats it as an anti-pattern; 311 embraces it with invalidation |
| no counterpart found | `compare()`'s KNOWN_UNSPOKEN; may-read / may-write / entailment; the region test; `:places` | GAP. No classic separates "never spoken" from "unknown", and none has a footprint/write-set model |

## Candidate table

| [slug] | what it is | exhaustive / broad / abstract / battle-tested | 311 analogues | punts | altitude |
|---|---|---|---|---|---|
| [A-saltzer-kaashoek-system-design-naming-2009] | MIT systems textbook, naming chapters + UNIX/DNS/URL/NFS case studies | exhaustive for naming vocabulary / broad across OS+network / abstract / taught ~30 yrs, UNIX, DNS, NFS as lived evidence | MScheme, resolve, contexts→MParent/MVantage, root, synonyms, unique-id warrants, compare's meanings | interference ("kick and see") out of model | naming theory |
| [A-saltzer-naming-and-binding-of-objects-1978] | the originating LNCS chapter | exhaustive for 1978 OS naming / OS-internal / abstract / Multics, CTSS, CAL, UNIX | context, closure, limited contexts, user-dependent binding, unstable bindings, indirect entries | distributed naming "unsolved"; stale Multics links fixed by fresh address space | naming theory |
| [A-saltzer-naming-binding-network-destinations-1982] | Saltzer's service/node/attachment/path binding model (RFC 1498) | narrow (network destinations) / abstract / cited widely | layered yields; "one table records one of three bindings" | none relevant | naming theory |
| [B-shoch-inter-network-naming-addressing-routing-1978] | name/address/route trichotomy (IEN 19) | narrow / semi-abstract / canonical terminology | MKey / catalog / traversal | identity not addressed | terminology |
| [A-lampson-designing-a-global-name-service-1986] | DEC global name service design (PODC) | exhaustive for its design / global directories / precise spec / toy impl.; lineage of Grapevine, Clearinghouse | DI, full names, TX-bounded caches, move-with-link | descriptive names; no referent identity | naming + replication spec |
| [A-lampson-6826-naming-handout-2006] | Lampson's formal Spec of name spaces | formal / broad (FS, DNS, SNMP, Plan 9, device regs) / abstract / teaching | aliases-as-interference axis, non-atomic lookup, roots, views | no footprint model | formal spec |
| [A-birrell-levin-needham-schroeder-grapevine-1982] | deployed Xerox registration + mail service (CACM) | one domain / multi-authority / concrete / 1500 users | registries as authorities, no re-creation, hints | creation collisions to humans; no change notification | system design |
| [A-cheriton-mann-decentralizing-global-naming-1989] | V-system decentralized naming (TOCS) | files, windows, processes, connections / 3 authority levels / measured | on-use staleness check ≈ witness(), "covers" incl. absence, unreachable≠absent | Byzantine faults; incomplete listings | system design |
| [A-plan9-use-of-name-spaces-1993] | per-process name spaces (OSR) | OS-wide / concrete / deployed | MVantage, :lends, union catalogs | no identity theory | OS design |
| [A-plan9-from-bell-labs-overview-1995] | Plan 9 overview | OS-wide / concrete / deployed | (type, dev, qid.path) identity; :corresponds via auth mapping | /proc "only a view" | OS design |
| [A-pike-lexical-file-names-plan9-2000] | getting `..` right (USENIX) | narrow (paths) / deep / deployed | traversal emitted with each resolution; ns replay | external renames undetected | OS design |
| [A-guarino-welty-identity-unity-individuality-2000] | formal identity criteria (ECAI) | abstract / ontology-wide / methodology basis | two independent warrants; strangers' descriptions | no mutation | ontology |
| [A-guarino-welty-overview-of-ontoclean-2009] | OntoClean overview | abstract / taxonomies / tutorials, CYC anecdote | +O/+I ≈ primary vs secondary scheme; constitution ≠ subsumption | hard-codes sortal individuation | ontology |
| [B-kent-data-and-reality-2000] | data-modelling classic | broad over business data / abstract / decades of citation | aliases vs ambiguity failure modes; qualifiers; existence tests | "assume two representatives are two things" | data modelling |
| [A-bowman-debray-peterson-reasoning-about-naming-systems-1993] | naming as inference (preference hierarchy) | descriptive name services / formal / Univers, Profile | soundness/completeness; per-attribute closed world; unique-name exclusion | no mutation | formal naming |
| [A-fellegi-sunter-theory-for-record-linkage-1969] | probabilistic record linkage (JASA) | person/event records / formal / census practice | three-way decision, two error types | estimation, not speech | statistics (contrast) |

## Citations

> [A-saltzer-kaashoek-system-design-naming-2009]:p.64 (relevance: +1:SURE)
> Different naming schemes have different rules about the uniqueness of name-to-value mappings. Some naming schemes have a rule that a name must map to exactly one value in a given context and a value must have only one name, while in other naming schemes one name may map to several values, or one value may have several names, even in the same context. Another kind of uniqueness rule is that of a unique identifier name space, which provides a set of names that will never be reused for the lifetime of the name space and, once bound, will always remain bound to the same value. Such a name is said to have a stable binding. If a unique identifier name space also has the rule that a value can have only one name, the unique names become useful for keeping track of objects over a long period of time, for comparing references to see if they are to the same object

> [A-saltzer-kaashoek-system-design-naming-2009]:p.62 (relevance: +1:SURE)
> The name-mapping algorithm is usually controlled by an additional parameter, known as a context. For a given naming scheme, there can be many different contexts, and a single name of the name space may map to different values when the resolver uses different contexts.

> [A-saltzer-kaashoek-system-design-naming-2009]:p.66 (relevance: +1:SURE)
> Many apparently puzzling problems in naming can be simply diagnosed: the name-mapping algorithm, for whatever reason, used the wrong context reference.

> [A-saltzer-kaashoek-system-design-naming-2009]:p.67 (relevance: -0:SUSPECT)
> This recursion may be repeated several times, but it must terminate somewhere, with the invocation of a name resolver that has a single built-in context.

> [A-saltzer-kaashoek-system-design-naming-2009]:p.75–76 (relevance: +1:SURE)
> the invoker might have one of three different questions in mind: 1. Are the two names the same? 2. Are the two names bound to the same value? 3. If the value or values are actually the identifiers of storage containers, such as memory cells or disk sectors, are the contents of the storage containers the same?
> Even this answer may still not reveal as much as expected because the two names may resolve to two names of a different, lower-layer naming scheme, in which case the same questions need to be asked recursively about the lower-layer names.
> Unless one has some specific understanding of the underlying physical representation, the only way to distinguish the two cases may be to change the contents of one of the named storage containers and see if that causes the contents of the other one to change. ("Kick this one and see if that one squeals.")

> [A-saltzer-kaashoek-system-design-naming-2009]:p.74 (relevance: -0:SUSPECT)
> As the number of libraries and names in the search path increases, the chance increases that two libraries will accidentally contain two unrelated programs that happen to export the same name.

> [A-saltzer-kaashoek-system-design-naming-2009]:p.104 (relevance: -0:SUSPECT)
> With mounted file systems, synonyms become a more difficult problem because per mounted file system there is an address space of inode numbers. Every inode number has a default context: the disk on which it is located.

> [A-saltzer-kaashoek-system-design-naming-2009]:p.105–106 (relevance: -1:GUESS)
> The reason is that ".." is resolved in the new default context, "/Scholarly/CSE499/www", rather than what might have been the intended context

> [A-saltzer-kaashoek-system-design-naming-2009]:p.127 (relevance: -0:SUSPECT)
> Deciding what constitutes the unique identity of a system that is constructed of replaceable components is ultimately a convention that requires an arbitrary choice by the designer of the naming scheme.
> The MAC address is thus properly viewed only as the unique name of a specific hardware component, not of the system in which it is embedded.

> [A-saltzer-kaashoek-system-design-naming-2009]:p.130 (relevance: +1:SURE)
> Dangling references are nearly always a concern when the name space is limited because names from limited name spaces must be reused. An object that incorrectly uses old names may make serious mistakes and even cause damage to an unrelated object that now has that name
> In some cases, it may be possible to deal with dangling references by considering names to be simply hints that require verification.

> [A-saltzer-kaashoek-system-design-naming-2009]:p.137 (relevance: -0:SUSPECT)
> Unambiguous syntactic interpretation of relative names requires that the context reference be a unique path name. Since the browser derives the context reference from the path name of the object that contained the relative name, and that object's path name does not have to be unique, it follows that syntactic interpretation of relative names will intrinsically be ambiguous.

> [A-saltzer-kaashoek-system-design-naming-2009]:p.180–181 (relevance: -0:SUSPECT)
> the DNS design does not call for explicit invalidation of changed entries. Instead, it uses expiration.
> whoever operates a name server can be a naming authority, which means that he or she may add authoritative records to that name server.

> [A-saltzer-kaashoek-system-design-naming-2009]:p.186 (relevance: +1:SURE)
> If the server should happen to reuse the inode of the old file for the new file, remote procedure calls from client 2 will get the new file, the one created by client 1, instead of the old file. The generation number allows NFS to avoid this incorrect behavior.

> [A-saltzer-naming-and-binding-of-objects-1978]:p.12 (relevance: +1:SURE)
> Yet another problem in using names is unstable bindings; that is, bindings that change unpredictably between definition and use. For example, file system catalogs often serve as contexts, and usually those catalogs permit names to be deleted or changed.

> [A-saltzer-naming-and-binding-of-objects-1978]:p.21 (relevance: +1:SURE)
> In an unlimited naming context, every name assigned can be different from every other name that has ever been or ever will be assigned in that context. Character string names are usually from unlimited naming contexts, as are unique identifiers, by definition. In a limited context the names themselves are a scarce resource that must be allocated and, most importantly, must be reused.
> Note that when a higher-level name is bound, through a context, to a lower-level name, the higher and lower level names are not considered synonyms.

> [A-saltzer-naming-and-binding-of-objects-1978]:p.19 (relevance: -0:SUSPECT)
> one result of employing the objects of others is that the creator of an object may have no idea of whether or not that object is still named by other objects in the system

> [A-saltzer-naming-and-binding-of-objects-1978]:p.103–104 (relevance: +1:SURE)
> When that happens, any old pointers to this procedure found in linkage sections of other procedures become obsolete, and should be readjusted. Multics does not attempt to maintain the tables and backpointers that would be required to automatically readjust interprocedure linkage pointers
> The user can request a fresh address space at any time, and that fixes the problem by initializing a fresh set of contexts, all consistent with one another.

> [A-saltzer-naming-and-binding-of-objects-1978]:p.84–85 (relevance: -0:SUSPECT)
> A system is distributed from the point of view of naming whenever two or more parallel and independently operating naming systems are asked to cooperate coherently with each other.
> Sharing between systems seems to involve maintaining contexts that work with multiple, independent name generators.

> [A-saltzer-naming-binding-network-destinations-1982]:L195-201 (relevance: -0:SUSPECT)
> Each of these three requirements includes the idea of preserving identity, whether of service, node or attachment point. To preserve an identity, one must arrange that the name used for identification not change during moves of the kind required.

> [A-saltzer-naming-binding-network-destinations-1982]:L272-287 (relevance: -0:SUSPECT)
> The design mistake is to believe that this table allows one to give the Lockheed DIALOG service a new name, merely by changing this table entry.

> [A-saltzer-naming-binding-network-destinations-1982]:L355-358 (relevance: -1:GUESS)
> Changing tables superficially appears to be what rebinding is all about, but the need to change more than one table is the tip-off that something deeper is going on.

> [B-shoch-inter-network-naming-addressing-routing-1978]:L230-235 (relevance: -1:GUESS)
> The names of different individuals may yield the same phone number.
> One name may yield several phone numbers

> [A-lampson-designing-a-global-name-service-1986]:p.3 (relevance: +1:SURE)
> The client sees a structure much like a Unix file system. There is a tree of directories (see figure 1), each with a unique directory identifier (DI) and a name by which it can be reached from its parent.

> [A-lampson-designing-a-global-name-service-1986]:p.9–10 (relevance: +1:SURE)
> A full name begins with the DI of the root directory for that name.
> When combining name services, it is prudent to make the old roots well-known in the new root, so that old names can still be looked up.
> an entry in ANSI for IBM, with the link ANSI/DEC/IBM as its value, gives names beginning ANSI/IBM the same meaning they had before the takeover.

> [A-lampson-designing-a-global-name-service-1986]:p.11 (relevance: +1:SURE)
> Since it is impractical for the service to keep track of clients that are doing this and notify them when there is a change, caching must be paid for either by enforcing a slow rate of change on the naming database, or by tolerating some inaccuracy in the cached information.
> The rule is: an arc or link may not be changed until its TX has expired, except that an arc may be deleted by a subtree move if it is replaced by a link to the moved subtree
> the result of a directory lookup can be safely cached until the minimum TX of any arc or link that was followed.

> [A-lampson-designing-a-global-name-service-1986]:p.16 (relevance: -0:SUSPECT)
> (D2) Looking up the FN di/n1/.../nk yields a directory if for the entire duration of the lookup operation di is the root, and each nj is defined in the directory di/n1/.../nj-1 and always yields the directory reference drj

> [A-lampson-6826-naming-handout-2006]:p.5 (relevance: +1:SURE)
> The MemNames specs are reasonably simple, but they are clumsy for operations on directories such as Rename. More fundamentally, they don't handle aliasing, where the same object has more than one name.

> [A-lampson-6826-naming-handout-2006]:p.14 (relevance: +1:SURE)
> Are there aliases, so that an update to one object can affect the value of others?
> Are the updates atomic, or it is possible for reads to see intermediate states?

> [A-lampson-6826-naming-handout-2006]:p.15 (relevance: +1:SURE)
> This means that Read(/a/x) can return 3 even though there was never any instant at which the path name /a/x had the value 3, or indeed was defined at all.
> A hard link differs from a soft link because the connection it establishes between a name and a file cannot be broken by changing the binding of some other name.

> [A-lampson-6826-naming-handout-2006]:p.2 (relevance: -0:SUSPECT)
> People often try to distinguish a name (what something is) from an address (where it is) or a route (how to find it). This is a matter of levels of abstraction and must not be taken as absolute.

> [A-lampson-6826-naming-handout-2006]:p.18 (relevance: -0:SUSPECT)
> For the NFS network file system, a root is named by a host name or IP address, plus a file system name or handle on that host. If that name or address gets assigned to another machine, too bad.
> In general it is a good idea to have absolute names (unique identifiers) for directories. This at least ensures that you won't use the wrong directory if the information about where to find it turns out to be wrong.

> [A-cheriton-mann-decentralizing-global-naming-1989]:p.152 (relevance: +1:SURE)
> We say an entity covers a name if it authoritatively knows either the definition of the name or that the name is undefined.
> clients have no way of distinguishing such names from names bound by a manager that is temporarily down or unavailable.

> [A-cheriton-mann-decentralizing-global-naming-1989]:p.154–155 (relevance: +1:SURE)
> Cache consistency is maintained by detecting and discarding stale cache entries on use.
> On-use cache consistency checking ensures that invalidation and reuse of directory identifiers never causes names to be mapped incorrectly.
> the manager recognizes that the directory's name does not match the name provided in the client's request, and returns an error indication to the client.

> [A-birrell-levin-needham-schroeder-grapevine-1982]:p.270 (relevance: +1:SURE)
> Deleting names is straightforward. A deleted entry is marked as such and retained in the data base with a version timestamp. Further updates to a deleted entry are not allowed. Recreation of a deleted entry is not allowed.
> We instead fell back on observations about the way in which systems of this nature are used. For each registry there is usually some human-level centralization of name creation

> [A-birrell-levin-needham-schroeder-grapevine-1982]:p.270 (relevance: -0:SUSPECT)
> Other registration service clients that use the registration data base to control resource bindings may also desire notification of changes to certain entries. ... We have not provided this general facility in the present implementation

> [A-birrell-levin-needham-schroeder-grapevine-1982]:p.273 (relevance: -1:GUESS)
> It has allowed us to separate the concept of naming a recipient from that of addressing the recipient. For example, the fact that a recipient is named Birrell.pa says nothing about where his messages should be sent.

> [A-plan9-use-of-name-spaces-1993]:§Examples (relevance: +1:SURE)
> Although there is no global name space, for a process to function sensibly the local name spaces must adhere to global conventions. Nonetheless, the use of local name spaces is critical to the system.
> (Compare this to the ad hoc and special-purpose idea of the PATH variable, which is not used in the Plan 9 shell.)

> [A-plan9-from-bell-labs-overview-1995]:§Implementation of Name Spaces (relevance: +1:SURE)
> Each file in Plan 9 is uniquely identified by a set of integers: the type of the channel (used as the index of the function call table), the server or device number distinguishing the server from others of the same type (decided locally by the driver), and a qid formed from two 32-bit numbers called path and version.
> If the file recovered from a walk has the same type, device, and qid path as an entry in the mount table, they are the same file and the corresponding substitution from the mount table is made.

> [A-plan9-from-bell-labs-overview-1995]:§The cpu command and proxied authentication (relevance: -0:SUSPECT)
> it also contains mappings between user names in different domains, for example saying that user rtm in one domain is the same person as user rtmorris in another.

> [A-pike-lexical-file-names-plan9-2000]:§The Implementation (relevance: +1:SURE)
> Since this implementation uses only local operations to maintain its names, it is possible to confuse it by external changes to the file system. Deleting or renaming directories and files that are part of a Cname, or modifying the mount table, can introduce errors.
> in a networked environment, with machines sharing a remote file server, renamings and deletions made by one machine may go unnoticed by others.

> [A-pike-lexical-file-names-plan9-2000]:§Names in Plan 9 (relevance: -0:SUSPECT)
> Finally, symbolic links are symbolic, like macros: they evaluate the associated names each time they are accessed. Bindings, on the other hand, are evaluated only once, when the bind is executed; after the binding is set up, the kernel associates the underlying files, rather than their names.

> [A-guarino-welty-identity-unity-individuality-2000]:§3.2 (relevance: +1:SURE)
> In many cases, it suffices to recognize that a property carries some (kind of) IC, without telling exactly which IC it is. To achieve this goal, we can introduce weak ICs, which are (only) necessary or (only) sufficient for identity.

> [A-guarino-welty-identity-unity-individuality-2000]:§3 (relevance: -0:SUSPECT)
> two (incomplete) descriptions of a person (like two records in different databases) can be different while referring to the same individual.

> [A-guarino-welty-overview-of-ontoclean-2009]:p.4 (relevance: +1:SURE)
> identity criteria are conditions used to determine equality (sufficient conditions) and that are entailed by equality (necessary conditions).

> [A-guarino-welty-overview-of-ontoclean-2009]:p.5 (relevance: +1:SURE)
> a further distinction is made to mark those properties that supply (rather just carrying) their "own" identity criteria, which are not inherited from the subsuming properties.
> Such criterion can be used to make conclusions about non-identity, if not about identity.

> [A-guarino-welty-overview-of-ontoclean-2009]:p.6 (relevance: +1:SURE)
> Sortal Individuation. Every domain element must instantiate some property carrying an IC (+I). In this way we satisfy Quine's dicto "No entity without identity"
> Together, the two assumptions imply that every entity must instantiate a unique most general property carrying a criterion for its identity.

> [B-kent-data-and-reality-2000]:p.70–71 (relevance: +1:SURE)
> If things have aliases, then equality will not be detected if two different names for the same thing are compared.
> If symbols can be ambiguous (name several things), then spurious matches will occur. Different things will be judged to be the same, because their names match.

> [B-kent-data-and-reality-2000]:p.44 (relevance: +1:SURE)
> An information system may or may not be able to detect the creation of two representatives for the same thing. It will assume that two representatives represent two different things.

> [B-kent-data-and-reality-2000]:p.57–58 (relevance: +1:SURE)
> In this case, the name does not go with the entity, but is an "attribute" of the relationship between the entity and the scope (i.e., it goes with the directory entry).
> It is sometimes asserted that each entity represented in the system must have a unique identifier. I contend that this is a requirement imposed by a particular data model (and it may make many things easier to cope with), but it is not an inherent characteristic of information.

> [B-kent-data-and-reality-2000]:p.60 (relevance: -0:SUSPECT)
> Such a relationship must really be invariant (unmodifiable). The relationship constitutes information that is redundantly scattered about everywhere that this entity is referenced, with the potential for enormous update anomalies if the information can change.

> [B-kent-data-and-reality-2000]:p.13–14 (relevance: -0:SUSPECT)
> After we discover that "the butler did it", have we established that they are "the same entity"? Shall we require the modeling system to collapse their two representatives into one? I don't know of any modeling system which can cope with that adequately.

> [B-kent-data-and-reality-2000]:p.147 (relevance: -0:SUSPECT)
> But if one record type contains social security numbers instead, then this knowledge is lost. As far as the system is concerned, there are no potential relationships here. It is only in the minds of users, and in procedural logic buried in programs, that any suspicion lurks that these might in fact refer to the same people.

> [A-bowman-debray-peterson-reasoning-about-naming-systems-1993]:p.5–6 (relevance: -0:SUSPECT)
> Instead of making the closed-world assumption over an entire database, however, we make the distinction on an attribute by attribute basis.

> [A-bowman-debray-peterson-reasoning-about-naming-systems-1993]:p.18 (relevance: -0:SUSPECT)
> In this case, since by definition a processor may have only one name, the object that is returned cannot be one that the client intends to identify.

> [A-bowman-debray-peterson-reasoning-about-naming-systems-1993]:p.20 (relevance: -1:GUESS)
> the object's machine-readable address is either a now attribute--one that currently describes the object--in which case the system must be designed to flush the binding if there is any chance that it has become stale, or it is a sometime attribute

> [A-fellegi-sunter-theory-for-record-linkage-1969]:p.52–54 (relevance: -1:GUESS)
> There will be however some cases in which we shall find ourselves unable to make either of these decisions at specified levels of error (as defined below) so that we allow a third decision, denoted A2, a possible link.
> in applications the decision A2 will require expensive manual linkage operations

## Leads not pulled

- Needham, "Names", ch. 5 in Mullender (ed.) *Distributed Systems* (1989 / 2nd ed. 1993). Cited by both Plan 9 papers as the naming reference. It is a book chapter. The ACM landing page only shows the ToC and I found no free copy. A library copy or the book itself is needed.
- Watson, "Identifiers (naming) in distributed systems", LNCS 105 (1981). Springer paywall only. Comer 1984 and Bowman et al. cite it for "general characterizations of the resolution process".
- Comer & Peterson, "Understanding naming in distributed systems", *Distributed Computing* 3(2) 1989. It models naming as syntax-directed string translation and gives precise definitions of synonyms, unique names and relative names. Springer paywall. The Purdue tech-report precursor ("A Name Resolution Model for Distributed Systems", CSD-TR 1984, docs.lib.purdue.edu article=1410) returned HTTP 403 to curl. The human could fetch it in a browser.
- Terry, "Structure-free name management for evolving distributed environments", ICDCS 1986. The only copy found was a geocities.ws PDF of uncertain provenance. Not pulled.
- Mann, "Decentralized naming in distributed computer systems", Stanford PhD 1987 (STAN-CS-87-1179). It holds the formal fault-tolerance treatment behind Cheriton & Mann. Stanford tech-report archive.
- Guarino & Welty, "Evaluating ontological decisions with OntoClean", CACM 45(2) 2002. It is superseded here by the authors' 2009 overview. The CACM text is behind ACM (cacm.acm.org has an HTML rendering that was not tried).
- Balakrishnan et al., "A Layered Naming Architecture for the Internet", SIGCOMM 2004 (pdos.csail.mit.edu/archive/6.824-2004/papers/doa.pdf). It covers flat persistent identifiers above hostnames. It surfaced in a search and was not read.
- Krakowiak, *Middleware Architecture*, ch. 3 "Naming and Binding" (lig-membres.imag.fr). This is a secondary survey that cites Watson, Comer & Peterson and Needham 1993. It could lead to more primaries.
- Gifford et al., "Semantic file systems", SOSP 1991. Lampson's handout cites it. Its views where a directory is a query are ≈ `:places` / placing lookups.
- Radin & Schneider, IBM TR 00.2757 (1976). The unique-identifier object map is cited by Saltzer. Not online.

## Search log

- kagi · 5 queries (Saltzer–Kaashoek ch.2 pdf; Saltzer NBO 1978; Lampson GNS; Plan 9 names; Guarino–Welty 2000) · kept 5 (+ RFC 1498 surfaced)
- curl · docdrop textbook / MIT NBO / umass + bwlampson GNS / 9p.io names.pdf (font-garbled) / aminer GW2000 · kept 4
- mcp fetch · doc.cat-v.org names, lexnames, 9 (overview); 9p.io names.html failed (encoding error) · kept 3
- kagi · 5 queries (OntoClean CACM; OntoClean overview; Kent Data and Reality; Comer & Peterson; Shoch) · kept 4 (OntoClean overview, Kent, Shoch IEN 19, Bowman–Debray–Peterson)
- kagi · 5 queries (Grapevine; Cheriton & Mann; Needham Names; Watson 1981; Terry 1986) · kept 2 (Grapevine, Cheriton–Mann); 3 → Leads
- kagi · 4 queries (naming theory survey; formal FS path-resolution model; Fellegi–Sunter; alias detection theory) · kept 2 (Lampson 6.826 HO12, Fellegi–Sunter)

## Tooling problems

- `register.sh` contention: registrations queue for minutes behind other lanes. `new-source.sh`'s download `curl -fsSL --retry 2` (line 82) has no `--max-time`, so a hung download by any lane holds the shared lock indefinitely. It did not hang during this run, but the risk is there.
- Purdue's docs.lib.purdue.edu returns 403 to curl (bot blocking). I did not work around it.
- 9p.io `names.pdf` extracts with a broken font encoding, and mcp-fetch of 9p.io `names.html` fails on the encoding declaration. The cat-v mirror was used instead.
- One `curl` to doc.cat-v.org failed with a transient schannel TLS error. mcp-fetch succeeded.
- UNCONFIRMED at hand-off (queued in `register.sh` behind other lanes; all were fetched, fully read and graded; entry JSONs are in `$SC`):
  - [A-plan9-use-of-name-spaces-1993] → `entry-plan9-names.json`
  - [A-pike-lexical-file-names-plan9-2000] → `entry-plan9-lexnames.json`
  - [A-plan9-from-bell-labs-overview-1995] → `entry-plan9-overview.json`
  - [A-guarino-welty-identity-unity-individuality-2000] → `entry-l9-gw2000.json`
  - [A-guarino-welty-overview-of-ontoclean-2009] → `entry-l9-ontoclean.json`
  - [B-shoch-inter-network-naming-addressing-routing-1978] → `entry-l9-shoch.json`
  - [A-saltzer-naming-binding-network-destinations-1982] → `entry-l9-rfc1498.json`
  - [A-bowman-debray-peterson-reasoning-about-naming-systems-1993] → `entry-l9-bdp.json`
  - [B-kent-data-and-reality-2000] → `entry-l9-kent.json`
  - [A-birrell-levin-needham-schroeder-grapevine-1982] → `entry-l9-grapevine.json`
  - [A-cheriton-mann-decentralizing-global-naming-1989] → `entry-l9-cheriton.json`
  Confirmed: Saltzer–Kaashoek 2009, Saltzer 1978, Lampson GNS 1986, Lampson 6.826 HO12, Fellegi–Sunter 1969.
  Caveat: the three cat-v.org Plan 9 URLs gave one transient schannel TLS error to curl. If `new-source.sh` fails on them, the fallback is the 9p.io copies (`/sys/doc/names.pdf` etc.). The names.pdf text layer is garbled, but the archive is still valid.
- The textbook copy is a third-party mirror (docdrop) of the printed Part I. Part I is not on OCW (OCW hosts only Part II, chs 7–11).
