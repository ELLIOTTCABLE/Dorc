# gather-cm-schema-and-module-systems — schema/module-system CM tools and config-file-structure APIs

311 read at `git rev-parse --short HEAD` = 63e49f29. All grades `graded-by: subagent`.

## Findings

- Every schema tool here identifies an entry by a NAME/PATH/KEY within its own tree and treats unequal keys as different things; none carries anything like 311's `:guarantees-unique-name` as a separate, withholdable warrant. Where a key turned out not to name one thing, each tool had to litigate it after the fact, and the fixes are exactly the "these two entries are one thing" rulings the counter-thesis asks for (next four bullets). ~SUSPECT (strong across 8 tools; I found no counterexample) [A-nixos-manual-option-types-2026] [A-microsoft-dsc-single-instance-resource-2020] [A-inmanta-language-reference-2026] [B-quattor-pan-language-book-2025]
- NixOS `environment.etc` is keyed by attribute name but emits a separate `target` path; when two differently-named entries land on one target the builder accepts them iff their sources are the same store path ("duplicate entry") and fails otherwise ("mismatched duplicate entry"). That is a SAME-by-content ruling at the real referent (the /etc path), not at the key. Guix, for the same situation, refuses outright ("duplicate '~a' entry for /etc"). +SURE [A-nixos-etc-module-source-2026] [A-guix-services-scm-source-2026]
- PowerShell DSC's xTimeZone is the clearest refuted-shape in the lane: its only key was the timezone VALUE, so two blocks setting different zones compiled as two "instances" and the node cycled "the timezone back and forth"; the fix made the machine's timezone a singleton cell (synthetic key `IsSingleInstance='Yes'`). PSDSC's compiler rejects equal keys with differing non-key properties; DSC v3 currently does not even do that ("In a future release, DSC will raise an error"; meanwhile "later instances override"). +SURE [A-microsoft-dsc-single-instance-resource-2020] [A-microsoft-dsc-v3-resource-properties-2025]
- inmanta is the only tool with a first-class DECLARED identity key: `index Host(name)`; constructing a second instance with the same index values returns the first (SAME from key equality, i.e. a declared `:guarantees-unique-referent`), and an index may include a relation (`index File(host, path)`) so the key is scoped in its parent (311's `:identified-in`). It has no counterpart to the name-uniqueness warrant: two different index values are simply two entities. +SURE [A-inmanta-language-reference-2026]
- Nix's store is the most precise abstract identity model in the lane, and it independently arrives at several 311 shapes: a store path "will always reference exactly one store object" but only within a store ("Different stores may disagree on what a given store path refers to"; "referential integrity" only "if store paths do not cross store boundaries"); two identity generators (input- vs content-addressing) with explicitly defined equivalence relations and a deliberately non-injective `resolved` that makes two derivations one; "Being stuck is contagious" for not-yet-known content addresses (cf. 311 MPlaceholders). RFC 62's "two-glibc" issue is a litigated case of one Output Id reaching two objects in two stores, fixed by one-realisation-per-store plus closure and refusal. +SURE [A-nix-manual-store-object-2026] [A-nix-manual-store-path-2026] [A-nix-manual-input-addressing-2026] [A-nixos-rfc0062-content-addressed-paths-2019]
- Multi-author merge, by tool: NixOS resolves one option by override priority (lowest number wins, others discarded) and then by the option TYPE's merge (str/enum: conflict error; listOf: concatenate, order by mkOrder; attrsOf: join by key; uniq: one author only), with per-file attribution in the conflict error. Guix lets only the EXTENDED service's owner define `compose`/`extend`; contributors speak only about their own service — the closest analogue to 311's committee law (§3.5). Pan is ordered overwrite of one tree with `final` as the only ownership lock and `?=` for defaults. CFEngine promises inside a file are not reconciled across bundles at all. +SURE [A-nixos-manual-option-definitions-2026] [A-nixos-manual-option-types-2026] [A-guix-manual-service-types-and-services-2026] [B-quattor-pan-language-book-2025] [A-cfengine-promises-reference-2024]
- Entries inside files: Augeas gives three identities — unique keyed paths (create-or-update works), repeated fixed labels (restored "by position"), and `seq` numbered nodes (restored "by key", i.e. by the number); the maintainers call numbered nodes "the odd-man out" for idempotence, and practitioners replace positional paths with value predicates (`*[ipaddr='127.0.0.1']`) that cannot create entries. CFEngine identifies a line by its CONTENT ("does this line exist") and places it by an anchor regex; a region is found by start/end anchors and a promise that deletes its own anchor loses its region. Neither tool emits anything like 311's positional MTraversal; both escape positional identity by moving to content identity. +SURE [B-lutterkort-augeas-configuration-api-2008] [B-augeas-issue-68-idempotent-changes-2013] [B-puppet-augeas-resource-tips-2017] [A-cfengine-edit-line-reference-2024] [A-cfengine-insert-lines-reference-2024]
- Cross-object entailment is never in a type schema; it is author-declared on an edge: Guix service extensions (and extending a missing service with a default value instantiates it), Bcfg2 bundles ("Contained entries are assumed to be inter-dependent"; changed bundle restarts its services), Quattor component pre/post plus user-written cross-element/cross-machine validators. That matches 311 §2.6 (entailment declared by the owner, not derived). ~SUSPECT [A-guix-services-scm-source-2026] [B-bcfg2-architecture-client-2013] [B-quattor-pan-language-book-2025]
- The residue (state outside the schema): NixOS pins defaults to the machine's install history (`stateVersion`), moves the uid (the primary key of a user) out of the schema into mutable `/var/lib/nixos/uid-map` (RFC 52), measures /etc ownership from the artifact itself (link target under /etc/static, a self-kept /etc/.clean ledger) and leaves everything else alone, and in overlay mode lets admin edits win. Bcfg2 reports "extra entries" by heuristic 2-way validation. DSC's `_purge` makes list-member ownership a per-instance flag. inmanta keeps "unmanaged resources" in a separate inventory. CFEngine's own docs name the co-owned file ("someone else's file ... managed from another source like a software package manager") and choose partial convergent editing. +SURE [A-nixos-state-version-option-2026] [A-nixos-rfc0052-dynamic-ids-2019] [A-nixos-setup-etc-activation-2026] [B-nixos-manual-etc-overlay-2026] [B-bcfg2-architecture-client-2013] [B-microsoft-dsc-v3-purge-property-2025] [B-inmanta-unmanaged-resources-2026] [B-cfengine-promising-editing-file-content-2012]
- Counter-thesis verdict: CONFIRMED in its main form, with specific exceptions. Schema-first tools do sidestep identity by being the world: identity = path/key in the tool's own tree, and aliasing between two keys, or between two types over one system object, is not modelled by any of them (no tool relates a DSC Registry instance to a DSC Environment instance, a NixOS `environment.etc` entry to a `systemd.services` unit file, etc.). The exceptions are all at the moment a second author or the residue appears, and each is a narrow, local ruling rather than a model: NixOS /etc target collision (SAME by content), Guix /etc collision (refuse), PSDSC equal-key rule (SAME iff values equal), inmanta index (SAME by declared key), Nix CA `resolved` + one-realisation-per-store. Nothing here is a general, exhaustive identity model across many ops domains. ~SUSPECT [A-nixos-etc-module-source-2026] [A-microsoft-dsc-single-instance-resource-2020] [A-nixos-rfc0062-content-addressed-paths-2019]
- The brief's seed "Lutterkort's LISA 2008 paper" is actually a 2008 Linux Symposium (OLS) paper, vol. 2 pp. 47-56. +SURE [B-lutterkort-augeas-configuration-api-2008]

## Candidate table

| [slug] | what it is | exhaustive / broad / abstract / battle-tested | 311 analogues | punts | altitude |
|---|---|---|---|---|---|
| [A-nix-manual-store-object-2026] [A-nix-manual-store-path-2026] [A-nix-manual-input-addressing-2026] | Nix store abstract model | exhaustive for one domain (built artifacts); narrow; highly abstract (inference rules); battle-tested (20 yrs, formalised 2023+) | MKey-Primary scoped in MParent-Store; unique-referent warrant within a store; no speech across stores (MWorlds); defined equivalence relations as MDerivations; deferred outputs ~ MPlaceholders | identity of anything outside the store; a store path's referent is store-relative | naming/identity theory for artifacts |
| [A-nixos-rfc0062-content-addressed-paths-2019] | RFC for CA derivations | one domain; abstract; litigated via RFC + experimental rollout | two derivations -> one object (non-injective resolve); one id -> two objects across stores (two-glibc) refused; closure invariant; per-user trust views (observer) | self-reference rewriting "is obviously a hack" | mutation/identity of builds |
| [A-nixos-manual-option-definitions-2026] [A-nixos-manual-option-types-2026] | NixOS module merge semantics | broad (whole OS), schema-first; merge rules abstract; very battle-tested | committee law partially (priorities, type merge), per-file attribution; attrsOf = keyed catalog; listOf = unkeyed concatenation | identity = option path; no aliasing between options | schema |
| [A-nixos-etc-module-source-2026] [A-nixos-setup-etc-activation-2026] [B-nixos-manual-etc-overlay-2026] | NixOS /etc generation + activation | one domain (/etc); concrete; battle-tested | two keys -> one target, SAME iff same source; ownership measured from artifact (link target) + ledger; admin residue left alone | un-owned /etc content; files under /etc not declared | mutation (whole-file ownership) |
| [A-nixos-state-version-option-2026] [A-nixos-rfc0052-dynamic-ids-2019] | NixOS residue: stateVersion, uid-map | narrow; litigated via RFC | MKey-Natural (username) vs MKey-Primary (uid) minted outside the schema; lifecycle of mutable data | data migrations, restored backups | schema-vs-state boundary |
| [A-guix-manual-service-types-and-services-2026] [A-guix-manual-service-reference-2026] [A-guix-services-scm-source-2026] | Guix service composition | broad (whole OS); abstract (DAG fold); battle-tested (~10 yrs) | extended owner defines compose/extend = committee law; extension edge = entailment; type-identity of targets; /etc duplicate refused | aliasing between service types; mutable state | schema |
| [A-microsoft-dsc-v3-resource-properties-2025] [B-microsoft-dsc-v3-purge-property-2025] [A-microsoft-dsc-single-instance-resource-2020] | DSC key/instance model | broad (any resource); declared keys; PSDSC battle-tested since 2013, v3 young | declared key (x-dsc-key) ~ unique-referent within a type; singleton cell (IsSingleInstance); `_purge` = whole-write vs member ownership | cross-type aliasing; v3 does not reject conflicts yet | schema/contract |
| [A-inmanta-language-reference-2026] [B-inmanta-unmanaged-resources-2026] | inmanta DSL `index`, relations, resource ids | broad-ish (network/infra); abstract; modest deployment | declared identity key, key scoped in parent relation; Unknowns ~ MPlaceholders; resource id = type + agent + id attribute | index inequality = distinct; unmanaged resources outside model | schema |
| [B-quattor-pan-language-book-2025] | Pan language for Quattor | broad (whole machine, per-host profiles); abstract; ~20 yrs at grid sites | path = identity; `final` = ownership lock; cross-machine validation via external paths | "machine profile names are equivalent to the hostname"; list index as identity | schema |
| [B-bcfg2-architecture-client-2013] [B-bcfg2-literal-configuration-specification-2013] [B-bcfg2-configuration-entries-2013] | Bcfg2 literal spec + 2-way validation | broad (Path/Package/Service/...); concrete; dormant project | residue as a first-class report (extra entries); bundle = entailment; identity (type, name); altsrc = many names one source | aliasing; heuristic extras | read-only assessment + mutation |
| [A-cfengine-edit-line-reference-2024] [A-cfengine-insert-lines-reference-2024] [B-cfengine-promising-editing-file-content-2012] [A-cfengine-promises-reference-2024] | CFEngine promises + line editing | broad (many promise types); line model generic; ~30 yrs | line identity = content; region = anchor pair; anchor invalidated by own write (routing mutation at line scale); promise identity = hash of promise | cross-bundle conflicts; ordering "will not converge" | mutation |
| [B-lutterkort-augeas-configuration-api-2008] [B-augeas-issue-68-idempotent-changes-2013] [B-puppet-augeas-resource-tips-2017] | Augeas tree + lens semantics, and its idempotence litigation | exhaustive for config-file syntax (hundreds of lenses); abstract (bidirectional lenses); very battle-tested | keyed vs positional vs seq entries; path expressions as lookups; value predicates as secondary keys; coexistence with every other writer | "which entry is this" pushed to callers; no uniqueness declared by lenses | config-file structure API |

Breadth against the 47 items (a rough map, ~SUSPECT): NixOS/Guix can declare most persisted items (2, 3, 5, 6, 9, 11-13 via boot.kernel.sysctl, 14-16 only as "installed in profile", 17-20, 23, 28, 30, 32, 35, 36 not applicable, 39, 40, 44, 45) but none of the live/activation-instance items (1 live state, 4, 7, 24, 25, 37, 47 live) and no positional item (29, 31 as rules; they regenerate whole rulesets). Pan/Quattor and Bcfg2 cover a similar persisted set via components/entry types. DSC covers what resources exist for (registry, files, packages, services; Windows-centric). Augeas/CFEngine edit_line cover the in-file halves of 3, 6, 9, 28, 29 (ufw file), 31, 32, 39, 40, 44, 45 with content or positional identity and nothing else. Nix store covers only built artifacts (a slice of 14/15/43). None expresses 7 (inode/hardlink), 8, 10, 21-22, 37-38 as identity questions.

## Citations

> [B-lutterkort-augeas-configuration-api-2008]:p49 (relevance: +1:SURE)
> The multitude of producers and consumers of configuration data makes it imperative that AUGEAS be useful without the support of these producers and consumers. ... it should be possible to use these tools side-by-side with AUGEAS. As a consequence, AUGEAS should not rely on such tools preserving any AUGEAS-specific annotations

> [B-lutterkort-augeas-configuration-api-2008]:p49 (relevance: +1:SURE)
> Multiple siblings can have the same label ... The n-th child with that label can be picked out with alias[n], and the last such child, with the special notation alias[last()].

> [B-lutterkort-augeas-configuration-api-2008]:p51-52 (relevance: +1:SURE)
> seq str is similar to label; in the get direction, it sets the label of the enclosing subtree to a number. When that subtree is used in an iteration, the numbers are consecutive, starting from 1.

> [B-lutterkort-augeas-configuration-api-2008]:p54 (relevance: +1:SURE)
> The former construct, using seq restores spacing by key, the number of the host entry in this case, whereas the latter restores it by position. When a new host entry is inserted into the tree under a new key, e.g. 10000, all existing entries keep their spacing

> [B-lutterkort-augeas-configuration-api-2008]:p55 (relevance: -0:SUSPECT)
> as far as the typechecker and the put direction of lenses are concerned, all tree nodes labeled foo are identical, no matter whether they are a leaf or whether they are the root of complicated subtrees.

> [B-puppet-augeas-resource-tips-2017]:L152 (relevance: +1:SURE)
> **It will add the line every time Puppet runs.** This is one of the biggest gotchas to using Augeas in Puppet.

> [B-puppet-augeas-resource-tips-2017]:L227 (relevance: +1:SURE)
> Which one of these is the line we want? It's most likely `/files/etc/hosts/1`, but you can't be sure. Fortunately, Augeas lets us identify a unique path using components of the item at that path.

> [B-puppet-augeas-resource-tips-2017]:L311 (relevance: +1:SURE)
> You can use complicated paths like this to modify existing values, but unfortunately, they can't be used to create new entries. ... To match on two values they both need to be set, and you can't set them at the same time

> [B-puppet-augeas-resource-tips-2017]:L637 (relevance: -0:SUSPECT)
> For nodes whose children are numbered sequentially (like the children of `/files/etc/hosts`), you need to invent a new label for the new child. ... since Augeas treats labels as strings, 01 and 1 are different

> [B-augeas-issue-68-idempotent-changes-2013]:(comment by georgehansper) (relevance: +1:SURE)
> This works for nodes that have a unique path (no repetition) ... The odd-man out is nodes with 'numbered' paths, like /etc/hosts entries. ... There is no way to create 127.0.0.1 if it is not found, within the scope of augeas itself.

> [B-augeas-issue-68-idempotent-changes-2013]:(comment by lutter) (relevance: -0:SUSPECT)
> No, none of this was ever implemented. It's a good amount of work, and I am a little hesitant to add this since it creates a completely new language

> [A-cfengine-edit-line-reference-2024]:L15-19 (relevance: +1:SURE)
> File editing is not just a single kind of promise but a whole range of 'promises within files'. ... inside each file are new objects that can make promises, quite separate from files' external attributes.

> [A-cfengine-edit-line-reference-2024]:L168-171 (relevance: +1:SURE)
> Restrict edits to a specific region of a file based on `select_start` and `select_end` regular expressions. If the beginning and ending regular expressions match more than one region only the first region will be selected for editing.

> [A-cfengine-edit-line-reference-2024]:L191,L285 (relevance: +1:SURE)
> The region selected with `select_region` exists during the lifetime of the promise. ... if the marker for a region needs to be removed, then it cannot be used as a marker.

> [A-cfengine-insert-lines-reference-2024]:L45-47 (relevance: +1:SURE)
> This will reverse the order of the lines and will not converge, since the anchoring after the marker applies independently for each new line.

> [A-cfengine-insert-lines-reference-2024]:L118,L462 (relevance: +1:SURE)
> The default is to treat the promiser as a literal string of convergent lines. ... Simply put, the 'does this line exist' test will be changed to a regexp match.

> [B-cfengine-promising-editing-file-content-2012]:L619 (relevance: +1:SURE)
> forces you to take over the ownership of the file `all or nothing' and determine its entire content yourself.

> [B-cfengine-promising-editing-file-content-2012]:L712,L974-977 (relevance: +1:SURE)
> Full power to customize file even with multiple managers. ... Edit a file with multiple promises about its state, when you do not want to determine the entire content of the file, or if it is unsafe to make unilateral changes, e.g. because its contents are also being managed from another source like a software package manager.

> [B-cfengine-promising-editing-file-content-2012]:L1519 (relevance: -0:SUSPECT)
> promise because it is fundamentally an order dependent configuration process.

> [A-cfengine-promises-reference-2024]:L93-95 (relevance: -0:SUSPECT)
> When a promise is validated (has an outcome of kept or repaired) it is locked for ifelapsed minutes (1 by default). Locks are based on a hash of the promise (promiser, associated attributes, and context).

> [A-cfengine-promises-reference-2024]:L130-133 (relevance: -1:GUESS)
> The language does not specifically disallow the use of the same attribute multiple times within a given promise. As a general rule the last definition wins but the behavior is not clearly defined and this should be avoided.

> [A-nixos-manual-option-definitions-2026]:L89-101 (relevance: +1:SURE)
> A module can override the definitions of an option in other modules by setting an *override priority*. All option definitions that do not have the lowest priority value are discarded. By default, option definitions have priority 100 and option defaults have priority 1500. ... `mkForce` is equal to `mkOverride 50`, and `mkDefault` is equal to `mkOverride 1000`.

> [A-nixos-manual-option-definitions-2026]:L117-118 (relevance: -0:SUSPECT)
> Note that this is different from override priorities: setting an order does not affect whether the definition is included or not.

> [A-nixos-manual-option-definitions-2026]:L199-201 (relevance: -0:SUSPECT)
> error: Cannot merge definitions of `foo'. Definition values:
> - In `file.nix': 13
> - In `custom place': 42

> [A-nixos-manual-option-types-2026]:L16,L240 (relevance: +1:SURE)
> All definitions must have the same value, after priorities. An error is thrown in case of a conflict. ... A string. Multiple definitions cannot be merged.

> [A-nixos-manual-option-types-2026]:L441-447 (relevance: +1:SURE)
> A list of *`t`* type ... Multiple definitions are merged with list concatenation. ... `types.attrsOf` *`t`* An attribute set of where all the values are of *`t`* type. Multiple definitions result in the joined attribute set.

> [A-nixos-manual-option-types-2026]:L541-542 (relevance: -0:SUSPECT)
> Ensures that type *`t`* cannot be merged. It is used to ensure option definitions are provided only once.

> [A-nixos-etc-module-source-2026]:L36-45 (relevance: +1:SURE)
> if ! [ -e "$out/etc/$target" ]; then ln -s "$src" "$out/etc/$target" else echo "duplicate entry $target -> $src" if [ "$(readlink "$out/etc/$target")" != "$src" ]; then echo "mismatched duplicate entry $(readlink "$out/etc/$target") <-> $src" ret=1

> [A-nixos-etc-module-source-2026]:L158-163 (relevance: -0:SUSPECT)
> target = lib.mkOption { ... Name of symlink (relative to {file}`/etc`). Defaults to the attribute name.

> [A-nixos-setup-etc-activation-2026]:L29-37 (relevance: +1:SURE)
> # Returns 1 if the argument points to the files in /etc/static. That means either argument is a symlink to a file in /etc/static or a directory with all children being static.

> [A-nixos-setup-etc-activation-2026]:L81,L109,L140-146 (relevance: +1:SURE)
> # Use /etc/.clean to keep track of copied files. ... warn "$target directory contains user files. Symlinking may fail."; ... # Delete files that were copied in a previous version but not in the current.

> [B-nixos-manual-etc-overlay-2026]:L28-36 (relevance: -0:SUSPECT)
> The overlay is atomically replaced during system switch. However, files that have been modified will NOT be overwritten. ... the changes are not completely gone, they are still in the upperdir of the previous overlay in `/.rw-etc/upper`.

> [A-nixos-state-version-option-2026]:L228-229,L243-253 (relevance: +1:SURE)
> This option defines the first version of NixOS you have installed on this particular machine, and is used to maintain compatibility with application data (e.g. databases) created on older NixOS versions. ... Most users should **never** change this value after the initial install ... Do **not** change this value unless you have manually inspected all the changes it would make to your configuration, and migrated your data accordingly.

> [A-nixos-rfc0052-dynamic-ids-2019]:L14,L29 (relevance: +1:SURE)
> A lot of NixOS modules are assigning static uids/gids to their users. This has resulted in less than 90 static ids left in the reserved range from 0 to 400. ... a central list of ids is annoying to maintain and leads to merge conflicts.

> [A-nixos-rfc0052-dynamic-ids-2019]:L53 (relevance: +1:SURE)
> These ids are persistent over the lifetime of a NixOS system, even when services are disabled and enabled again. The generated mapping from names to ids is stored in `/var/lib/nixos/uid-map`/`/var/lib/nixos/gid-map`, so if this directory is backed up, the mappings will persist too when restoring.

> [A-nixos-rfc0052-dynamic-ids-2019]:L22,L104 (relevance: -0:SUSPECT)
> without a central static uid mapping conflicts can occur, which will result in an error during the system build. ... If data is restored from a backup without restoring `/var/lib/nixos` ... then the service can fail to start. ... fixable by ... manually changing `/var/lib/nixos/{u,g}id-map` to map the name to the old id, by recursively `chown`ing the restored data to the new id

> [A-nix-manual-store-object-2026]:L42-50 (relevance: +1:SURE)
> The references of a store object ... is a field of a store object, and thus intrinsic by definition. ... The requisites of a store object are almost intrinsic --- some store paths do not precisely refer to a unique single store object. Exactly what store object is being referenced, and what in turn *its* references are, depends on the store in question. Different stores may disagree on what a given store path refers to. ... The referrers of a store object are completely extrinsic

> [A-nix-manual-store-path-2026]:L20-22 (relevance: +1:SURE)
> Think of a store path base name as an opaque, unique identifier: The only way to obtain a store path base name is by adding or building store objects. A store path base name will always reference exactly one store object.

> [A-nix-manual-store-path-2026]:L125-131 (relevance: +1:SURE)
> the full rendered store path is not just derived from the referenced store object itself, but depends on the store that the store object is in. ... Nix can only guarantee referential integrity if store paths do not cross store boundaries.

> [A-nix-manual-input-addressing-2026]:L5-7 (relevance: +1:SURE)
> "Input addressing" means the address the store object by the *way it was made* rather than *what it is*. ... Even if two store paths have the same contents, if they are produced in different ways, and one is input-addressed, then they will have different store paths, and thus guaranteed to not be the same store object.

> [A-nix-manual-input-addressing-2026]:L29,L160-162 (relevance: -0:SUSPECT)
> Every non-injective function induces an equivalence relation on its domain ... Being stuck is contagious. A derivation with deferred outputs has, for this purpose, unknown outputs of its own ... The condition therefore propagates downstream from the floating output that caused it.

> [A-nixos-rfc0062-content-addressed-paths-2019]:L171 (relevance: +1:SURE)
> `resolved` is (intentionally) not injective: If `drv` and `drv'` only differ because one depends on `dep` and the other on `dep'`, but `dep` and `dep'` are content-addressed and have the same output hash, then `resolved(drv)` and `resolved(drv')` will be equal.

> [A-nixos-rfc0062-content-addressed-paths-2019]:L339-345 (relevance: +1:SURE)
> Bob actually has a different `glibc` (living in a different store path) than Alice. ... `firefox` has both Alice's `glibc` and Bob's `glibc` in his closure (despite having only one specified in the derivation). ... The first step to that end, is to enforce the fact that a store can't have more than one realisation for each derivation output.

> [A-nixos-rfc0062-content-addressed-paths-2019]:L359,L454-458 (relevance: -0:SUSPECT)
> realisations should form a closure in a store ... The current implementation has a naive approach that just forbids fetching a path if the local system has a different realisation for the same drv output. This approach is simple and correct, but it's possible that it might not be good-enough in practice

> [A-guix-manual-service-types-and-services-2026]:(see archived copy; "There can be only one instance") (relevance: +1:SURE)
> There can be only one instance of an extensible service type such as udev-service-type. If there were more, the service-extension specifications would be ambiguous.

> [A-guix-manual-service-types-and-services-2026]:(see archived copy; "compose") (relevance: +1:SURE)
> compose: This is the procedure to compose the list of extensions to services of this type. ... extend: This procedure defines how the value of the service is extended with the composition of the extensions.

> [A-guix-manual-service-reference-2026]:L451 (relevance: -0:SUSPECT)
> If this is #f, services of this type cannot be extended.

> [A-guix-manual-service-reference-2026]:L670 (relevance: -1:GUESS)
> Two different OS configurations or sets of channels can lead to the same system, bit-for-bit

> [A-guix-services-scm-source-2026]:L921-934 (relevance: +1:SURE)
> (when (set-contains? seen file) (raise (formatted-message (G_ "duplicate '~a' entry for /etc") file))) ... ;; Detect duplicates early instead of letting them through, eventually ;; leading to a build failure of "etc.drv".

> [A-guix-services-scm-source-2026]:L1304-1338 (relevance: -0:SUSPECT)
> (G_ "no target of type '~a' for service '~a'") ... (G_ "more than one target service of type '~a'")

> [A-guix-services-scm-source-2026]:L1346-1350 (relevance: +1:SURE)
> Return SERVICES, a list, augmented with any services targeted by extensions and missing from SERVICES. Only service types with a default value can be instantiated; other missing services lead to a '&missing-target-service-error'.

> [A-microsoft-dsc-v3-resource-properties-2025]:L60-71 (relevance: +1:SURE)
> DSC uses key resource properties to uniquely identify instances of the resource on a system. ... Instances in a configuration document with the same values for their key properties are _conflicting instances_. ... In a future release, DSC will raise an error ... If you define different settings for conflicting instances, DSC invokes the resource for each conflicting instance during every **Set** operation. In this case, later instances override any settings defined by an earlier conflicting instance

> [B-microsoft-dsc-v3-purge-property-2025]:L31-33 (relevance: -0:SUSPECT)
> When `_purge` is `true`, the resource removes unmanaged entries. The resource treats any entries not listed in the instance's desired state as invalid. When `_purge` is `false` or not specified, the resource ignores unmanaged entries.

> [A-microsoft-dsc-single-instance-resource-2020]:L51-57 (relevance: +1:SURE)
> A resource instance is considered unique if the combination of the values of all of its key properties is unique. In its previous implementation, the xTimeZone resource had only one property--**TimeZone**, which was required to be a key. ... a configuration such as the one above would compile and run without warning. Each of the **xTimeZone** resource blocks is considered unique. This would cause the configuration to be repeatedly applied to the node, cycling the timezone back and forth.

> [A-microsoft-dsc-single-instance-resource-2020]:L221-224 (relevance: +1:SURE)
> Test-ConflictingResources : A conflict was detected between resources ... Resources have identical key properties but there are differences in the following non-key properties: 'TimeZone'.

> [A-microsoft-dsc-single-instance-resource-2020]:L10-13 (relevance: -0:SUSPECT)
> Currently, there is no built-in DSC feature to do this. That might change in the future.

> [A-inmanta-language-reference-2026]:(see archived copy; "Indexes and queries") (relevance: +1:SURE)
> Index definitions make sure that an entity is unique. An index definition defines a list of properties that uniquely identify an instance of an entity. If a second instance is constructed with the same identifying properties, the first instance is returned instead.

> [A-inmanta-language-reference-2026]:(see archived copy; "index File(host, path)") (relevance: +1:SURE)
> For indices on relations (instead of attributes) an alternative syntax can be used ... index File(host, path) a = File[host=vm1, path="/etc/passwd"] # normal index lookup b = vm1.files[path="/etc/passwd"] # selector style index lookup # a == b

> [A-inmanta-language-reference-2026]:(see archived copy; "Assigning a value to the same variable twice") (relevance: -0:SUSPECT)
> A value can be assigned to a variable exactly once. ... Assigning a value to the same variable twice will produce a compiler error, unless the values are identical.

> [B-inmanta-unmanaged-resources-2026]:(see archived copy; "Unmanaged resources are resources") (relevance: -0:SUSPECT)
> Unmanaged resources are resources that live in the network that are not yet managed by the orchestrator. ... Discovered resources only exist in the discovered resources database. They don't exist in the configuration model.

> [B-quattor-pan-language-book-2025]:L58-59 (relevance: -0:SUSPECT)
> Declarative language allows easier merging of configurations from different administrators.

> [B-quattor-pan-language-book-2025]:L969-989 (relevance: +1:SURE)
> Assignment statements are used to modify a part of the configuration tree by replacing the subtree identified by its path ... If the ``final`` modifier is used, then the path and any children of that path may not be subsequently modified. Attempts to do so will result in a fatal error.

> [B-quattor-pan-language-book-2025]:L545-551 (relevance: -0:SUSPECT)
> Types have been bound to two paths with these ``bind`` statements. ... Configuration in other paths can be added without being subject to these type definitions. A global schema can be defined by binding a type definition to the root path '/'.

> [B-quattor-pan-language-book-2025]:L1400-1404 (relevance: -0:SUSPECT)
> A list is an ordered list of elements with the indexing starting at zero. ... The order of a list is significant and maintained in the serialized representation of the configuration.

> [B-quattor-pan-language-book-2025]:L2474-2480 (relevance: +1:SURE)
> the full external path must be constructed for the ``value`` function. ... The above code also assumes that the machine profile names are equivalent to the hostname. If another convention is being used, then the hostname will have to be converted to the corresponding machine name.

> [B-bcfg2-architecture-client-2013]:L38-53 (relevance: +1:SURE)
> The configuration describes the complete target state of the machine. That is, all aspects of client configuration should be represented in this specification. ... heuristic checks are executed for configuration not included in the configuration specification. We refer to this inventory process as 2-way validation ... unspecified configuration that should be removed.

> [B-bcfg2-architecture-client-2013]:L66-70 (relevance: -0:SUSPECT)
> Contained entries are assumed to be inter-dependent. To address this, the client re-verifies each entry in any bundle containing an updates configuration entry. Also, services contained in modified bundles are restarted.

> [B-bcfg2-literal-configuration-specification-2013]:L34-50 (relevance: -0:SUSPECT)
> Bundles are groups of inter-dependent configuration entities. The purpose of bundles is to encode installation-time dependencies ... A number of configuration entities exist including Path, Package, Service, etc. Each of these correspond to the obvious system item.

> [B-bcfg2-configuration-entries-2013]:L16-20,L36-40 (relevance: -1:GUESS)
> an abstract entry is defined in a bundle. This entry includes a type (the XML tag) and a name attribute. Then this entry is bound for a client ... The ``altsrc`` attribute lets you remap configuration entry names on the server side so you can reuse a single concrete representation for multiple abstract entries.

## Leads not pulled

- LCFG component/resource docs · the UK-origin model where each component owns a resource namespace; brief item 5 · not pulled: out of budget after Pan/Bcfg2/inmanta, and Anderson's LCFG papers are lane 5b's · lcfg.org "LCFG components" / github.com/LCFG
- Quattor ncm-ncd (component dispatcher) · components run on `/software/components/<name>` with pre/post deps; I read its pod (github.com/quattor/ncm-ncd src/main/scripts/ncm-ncd.pod) but it states no subtree-ownership rule, so not registered · the ownership rule likely lives in NCM::Component docs (quattor/documentation docs/Unittesting/Quattor_Component.rst or perl-CAF)
- NixOS RFC 0042 (structured `settings` options) · generating whole config files from a typed attrset instead of line edits; bears on "NixOS generates whole files to dodge in-file identity" · not read · github.com/NixOS/rfcs/blob/master/rfcs/0042-config-option.md
- nixpkgs `lib/modules.nix` doc comments · exact merge algorithm, `highestPrio`, definitions-with-locations · not read (manual sections sufficed) · github.com/NixOS/nixpkgs/blob/master/lib/modules.nix
- NixOS/nix issue #9259 · coarser derivation equivalence (the identity relation is still being litigated) · not read · github.com/NixOS/nix/issues/9259
- Config::Model (dod38fr) · a Perl schema-over-config-files tool with an Augeas backend; possibly a per-file schema with declared keys · not read · metacpan.org/pod/Config::Model ; github.com/dod38fr/config-model/wiki
- augeasproviders / Narcissus (raphink) · declarative idempotent layers over Augeas; how they key entries · not read · github.com/hercules-team/augeasproviders_core
- DSC v3 resource manifest `export` and resource kinds (group/adapter/importer) · adjacency/external scoping of instances (read `instances.md`, not registered: scoping of references, not of identity) · MicrosoftDocs/PowerShell-Docs-DSC dsc-3.0/concepts/resources/{instances,kinds}.md
- CFEngine custom promise types / `classes` and normal ordering · convergence model beyond locks · not read · docs.cfengine.com reference/language-concepts/normal-ordering

## Search log

- kagi · CFEngine reference edit_line bundle insert_lines select_region · 3 kept
- kagi · CFEngine 3 reference promise types convergence ifelapsed · 1 kept
- kagi · NixOS manual writing modules option definitions mkOverride mkMerge · 2 kept (primaries via nixpkgs repo)
- kagi · Augeas configuration API LISA 2008 Lutterkort paper · 1 kept
- kagi · puppet augeas type "match" idempotent insert "last()" onlyif path expression · 1 kept
- kagi · augeas path expressions documentation seq nodes numbered entries · 0 kept
- kagi · augeas wiki "Path expressions" "seq" · 1 kept (issue #68)
- kagi · DSC v3 resource manifest schema _exist _inDesiredState _purge · 1 kept
- kagi · Microsoft DSC v3 resource instance key properties identity · 1 kept
- kagi · inmanta language reference index entity relation · 2 kept
- kagi · Pan language Quattor profile schema /software/components ncm component · 1 kept (via quattor/pan repo)
- kagi · quattor pan language reference manual "include" "final" "bind" "validation" book · 0
- kagi · Bcfg2 documentation "extra entries" client reporting unmanaged · 3 kept (via Bcfg2/bcfg2 repo docs)
- kagi · Bcfg2 "Path" "Package" "Service" entry types abstract structure bundle documentation · 0
- gh · repos/NixOS/rfcs/contents/rfcs listing · 2 kept (RFC 52, RFC 62)
- gh · repos/NixOS/nix doc/manual/source/store tree · 3 kept
- gh · MicrosoftDocs/PowerShell-Docs-DSC code search "Key property" · 1 kept (singleInstance)
- fetch · guix.gnu.org manual Service-Composition / Service-Types-and-Services / Service-Reference · 2 kept + services.scm via codeberg

## Tooling problems

- register.sh lock contention across lanes was severe (dozens of queued registrations); several of my registrations waited many minutes in background. A `timeout 110` wrapper I tried once killed a queued registration mid-wait; it did not corrupt anything, and I re-queued without timeouts.
- [A-nix-manual-store-object-2026]'s `via` field was written as "mcp__github__get_file_contents via gh api repos/NixOS/nix/git/trees/master (doc/manual/source/store)"; the actual call was `gh api "repos/NixOS/nix/git/trees/master?recursive=1"` (Bash), not the GitHub MCP tool. The mislabel is in the manifest; flagging it rather than hand-editing.
- Several sources were archived as `.html` though the fetched bytes are raw markdown/code (raw.githubusercontent serves text/plain); line numbers above are from those archived copies and match the raw text.
- Guix and inmanta pages are true HTML; citations to them give an anchor phrase instead of a line number where I cite before archiving completed (see the Citations section).
