# gather-standards-management-models — DMTF CIM, IETF YANG/NMDA, IETF SNMP SMI as identity models of managed state

Lane 1 of ten. I read 311 at `63e49f29`. Every grade is `graded-by: subagent`. Line numbers refer to the archived .txt/.mof copies. For the DMTF PDFs they are the spec's printed margin line numbers plus the printed page.

> Conductor's note: the lane's two writes of this file failed with ENOSPC when drive C: filled; the lane handed the complete content back inline and the conductor wrote it here verbatim. Seven of the lane's graded sources were unregistered at hand-off (see Tooling problems); the conductor registers them from the lane's entry JSONs.

## Findings
- CIM is the closest standards-body counterpart to 311's `:identified-in` chain. A weak class is identified inside a scoping instance whose keys propagate into it, and this nests: ComputerSystem > OperatingSystem > LocalUser; System > FileSystem > LogicalFile; OperatingSystem > Process. The schema owner declares the scope once per class, with no per-shape variation. [A-dmtf-dsp0004-cim-infrastructure-2-8-2014] [A-dmtf-cim-logicalfile-class-2005] (+SURE)
- CIM's naming clause states 311's two-warrant split for namespaces. Equal namespace-path strings reference the same namespace; unequal strings license "no conclusion". That is `:guarantees-unique-referent` without `:guarantees-unique-name`. [A-dmtf-dsp0004-cim-infrastructure-2-8-2014] (+SURE)
- CIM separates CIM-object identity from managed-object identity, which is 311's MKey-vs-MReferent seam. The bridge between them is the Correlatable qualifier: an org-scoped, one-directional SAME generator whose failure yields "no conclusion". The spec explicitly keeps it distinct from the opaque InstanceID. [A-dmtf-dsp0004-cim-infrastructure-2-8-2014] (+SURE)
- CIM v3 removed Correlatable with "No replacement". It folded Weak, Propagated and Delete/IfDeleted into OCL constraints. The change list gives no rationale. [B-dmtf-dsp0004-cim-metamodel-3-0-2014] (+SURE on the removal; --WONDER why)
- CIM keys a file by path, not by inode:
  - LogicalFile is weak to its FileSystem, "not to a Directory", with "a full path name" as the unique Name.
  - UnixFile carries the inode number and a LinkCount ("Count of the number of names for this file") as non-key properties.
  - So hardlinks are separate instances with no declared sameness. This is a punt on GOTCHA containment-by-path-prefix-lies.
  [A-dmtf-cim-logicalfile-class-2005] [A-dmtf-cim-unixfile-class-2005] (+SURE on the text; ~SUSPECT that no other class restores inode identity, because I read only a subset of the schema)
- A remotely mounted filesystem is a FileSystem instance of the local System. One NFS export mounted on two hosts is therefore two instances with no declared correspondence (GOTCHA a-host-is-not-a-partition). [A-dmtf-cim-filesystem-class-2005] (+SURE)
- InstanceID `<OrgID>:<LocalID>` guarantees uniqueness within one namespace, plus a SHOULD against reusing an ID "to identify different underlying (real-world) elements". It is a minting-authority rule and a non-reuse promise, not an equality warrant. [A-dmtf-cim-softwareidentity-class-2011] (+SURE)
- CIM has two effect-shaped constructs, and neither is a per-verb writeset with a closure:
  - ServiceAffectsElement says "running the service may change, manage... the ManagedElement". It is instance-level with a coarse effect enumeration.
  - The Delete/IfDeleted qualifiers declare cascade deletion along associations. They are optional, and implementations "can ignore" them.
  [A-dmtf-cim-serviceaffectselement-class-2005] [A-dmtf-dsp0004-cim-infrastructure-2-8-2014] (+SURE)
- NMDA (running / intended / operational) is the closest anyone standardised to 311's persisted-vs-live and declared-vs-activated seam. Its `origin` annotation (intended, dynamic, system, learned, default, unknown) is a provenance tag on every operational config value. Its limits:
  - Intended-to-operational correspondence is by same path and key only.
  - `<operational>` may violate "the uniqueness of key values".
  - Missing resources and remnant configuration are named, but not modelled as identity events.
  [A-ietf-rfc8342-nmda-2018] (+SURE)
- ietf-interfaces binds a declared interface to an activated one by NAME equality. It admits it "may be impossible... to provide predictable and consistent names for system-controlled interfaces across insertion/removal cycles". IF-MIB allows two ifEntries with the same ifName, so no 1-1 mapping exists. [A-ietf-rfc8343-ietf-interfaces-2018] (+SURE)
- OpenConfig's pre-NMDA rationale punts the hard case: "It is not clear what to do when the intended and applied configuration differ", and it "makes no presumption". It also concedes that list keys like an interface name are "unlikely... actually configurable in any real system". [B-openconfig-opstate-draft-2015] (+SURE)
- YANG has exactly one sameness generator: equal key values inside one parent entry.
  - `unique` is a validity constraint that silently exempts entries missing a referenced leaf.
  - leafref and instance-identifier are referential integrity, never aliasing.
  - Nothing can say "these two entries are one thing".
  [A-ietf-rfc7950-yang11-2016] [A-ietf-rfc8407-yang-guidelines-2018] (+SURE)
- YANG `when` is a schema-declared write entailment: the server deletes a node whose `when` condition turns false after a modification. It works inside one datastore only. [A-ietf-rfc7950-yang11-2016] (+SURE)
- YANG avoids positional identity by design, which makes it a counter-design to 311 §2.9's positional catalogs:
  - Ordered DNS/NTP/RADIUS server lists and SSH authorized keys are keyed by "An arbitrary name".
  - Order is carried separately (`ordered-by user`, insert before/after a key).
  - Only keyless state lists fall back to positions (`port[3]`).
  [A-ietf-rfc7317-ietf-system-2014] [A-ietf-rfc7950-yang11-2016] (+SURE)
- SNMP ifIndex litigation is the best battle-testing record in this lane:
  - ifIndex must stay constant only between agent re-initialisations.
  - A "different" interface may not reuse an index before the next re-init, and "different" is left to implementors ("Any firm definition... would likely turn out to be inadequate").
  - ifCounterDiscontinuityTime acts as a witness instead of issuing a new index.
  - A sysUpTime reset is the lifecycle signal.

  This is the analogue of 311's lifecycle mutation, and of a `witness()` that cannot see a recycled MKey. [A-ietf-rfc2863-if-mib-2000] (+SURE)
- SNMP names observer-dependence directly. ENTITY-MIB distinguishes "multi-scoped" objects (identical instance values mean different things in different naming scopes) from "single-scoped" ones. IF-MIB says whether ifIndex values across scopes are the same interface is "the agent's choice". The naming scope (community or context) plays the MVantage role. [A-ietf-rfc6933-entity-mib-v4-2013] [A-ietf-rfc2863-if-mib-2000] (+SURE)
- ENTITY-MIB also has:
  - an agent-authored correspondence table, entAliasMappingTable;
  - scoped keys: serial numbers are comparable only within one manufacturer;
  - a containment tree;
  - admin handles that must survive a change of index;
  - an explicit punt: multiple agents "are not required to be equivalent or even consistent".
  [A-ietf-rfc6933-entity-mib-v4-2013] (+SURE)
- HOST-RESOURCES-MIB is the only IETF standard that enumerates host ops state:
  - processes keyed by "the system's native, unique identification number", i.e. a recycled pid;
  - installed software keyed 1..N, so any install or remove renumbers it;
  - a cache-staleness scalar;
  - a SET operation that kills a process;
  - filesystems with their remote source.

  Software identity is left open: "Different implementations may track software in varying ways". [A-ietf-rfc2790-host-resources-mib-2000] (+SURE)
- Both families classify persistence, but at different grain. SNMP does it per row: `StorageType` is readable as a column. NMDA does it per datastore. RFC 6643 says the two "persistency models... are quite different" and therefore translates every MIB object as read-only. [A-ietf-rfc2579-smiv2-textual-conventions-1999] [A-ietf-rfc8342-nmda-2018] [A-ietf-rfc6643-smiv2-to-yang-2012] (+SURE)
- RowStatus separates "exists in the agent" from "available for use by the managed device". The manager picks a new row's index, retrying with another pseudo-random value on collision. Identity is minted by collision, not by warrant. [A-ietf-rfc2579-smiv2-textual-conventions-1999] (+SURE)
- SMI scopes a row inside another table's row by putting a foreign column in its INDEX; this is SMI's `:identified-in`. AUGMENTS declares a one-to-one, same-identity extension that exists if and only if the base row exists. [A-ietf-rfc2578-smiv2-1999] (+SURE)
- Counter-thesis verdict: +SURE that none of the three families can express "writing X reaches Y" as an authored, closed writeset. Their only mechanisms are schema-level existence dependencies: AUGMENTS, YANG `when`-deletion, CIM Delete/IfDeleted (optional), RowStatus destroy, and CIM ServiceAffectsElement (per instance, coarse). ~SUSPECT that identity is always minted by one authority per scope. The only reconcilers across authorities are CIM Correlatable (removed in v3), provider-asserted CIM LogicalIdentity, and agent-asserted ENTITY-MIB alias maps. NMDA is the closest anyone got to 311's two-form seam, and it declines to define what divergence means. [A-ietf-rfc8342-nmda-2018] [B-openconfig-opstate-draft-2015] [A-dmtf-cim-logicalidentity-class-2005]
- The SMIng/SMIv3 merge failed partly on "how much the SNMP naming system should be changed", and the survey leaves object naming as open research. [B-schoenwaelder-pras-martinflatin-future-internet-mgmt-2003] (-GUESS on its weight)

## Candidate table
| [slug] | what | breadth / battle-tested | 311 analogues | punts | altitude |
|---|---|---|---|---|---|
| [A-dmtf-dsp0004-cim-infrastructure-2-8-2014] | CIM v2 metamodel | all CIM; WMI/OpenPegasus/SBLIM | Weak+Propagated = `:identified-in` chain; namespace rule = unique-referent w/o unique-name; Correlatable = one-way SAME; 8.3 = MKey vs MReferent; Delete/IfDeleted = lifecycle entailment | namespace sameness undecidable; Delete optional | schema/naming |
| [B-dmtf-dsp0004-cim-metamodel-3-0-2014] | CIM v3 change list | narrow read | removal record | Correlatable "No replacement" | schema |
| [A-dmtf-cim-logicalfile-class-2005] | CIM_LogicalFile | one class | path MKey in FS store | hardlinks, bind mounts | schema |
| [A-dmtf-cim-unixfile-class-2005] | CIM_UnixFile | one class | inode/linkcount non-key | inode never identity | schema |
| [A-dmtf-cim-filesystem-class-2005] | CIM_FileSystem | one class | FS weak to System; PersistenceType; hrFSMountPoint mapping | NFS on two hosts = two instances | schema |
| [A-dmtf-cim-softwareidentity-class-2011] | CIM_SoftwareIdentity | one class | InstanceID minting; ConcreteIdentity to files | "does NOT indicate... installed" | schema |
| [A-dmtf-cim-serviceaffectselement-class-2005] | ServiceAffectsElement | one class | nearest may-write | instance-level, no closure | effects |
| [A-dmtf-cim-logicalidentity-class-2005] | LogicalIdentity | one class | provider-asserted `:corresponds` | "most scenarios" key equality | schema |
| [B-dmtf-dsp1023-software-inventory-profile-2009] | SW Inventory profile | firmware scope; deployment unverified | Installed vs Available; Software Family correlation (at-least-one match) | LocalID "implementation specific" | profile |
| [A-ietf-rfc8342-nmda-2018] | NMDA | all YANG | declared-vs-activated; origin; remnant; missing resources | path+key only | architecture |
| [A-ietf-rfc7950-yang11-2016] | YANG 1.1 | language | keys in parent; unique; when; ordered-by | no aliasing | schema lang |
| [A-ietf-rfc8407-yang-guidelines-2018] | YANG BCP | review-derived | correlation by key type/leafref | "not possible to define one approach" | guidance |
| [A-ietf-rfc8343-ietf-interfaces-2018] | ietf-interfaces | deep deployment | name binding; system vs user controlled | unpredictable names | schema |
| [A-ietf-rfc7317-ietf-system-2014] | ietf-system | hostname/TZ/NTP/resolver/users/keys | label keys, order separate | positions avoided | schema |
| [B-openconfig-opstate-draft-2015] | OpenConfig rationale | operator reqs | intended/applied/derived; async | divergence semantics | architecture |
| [A-ietf-rfc2578-smiv2-1999] | SMIv2 | language | foreign INDEX; AUGMENTS | none | schema lang |
| [A-ietf-rfc2579-smiv2-textual-conventions-1999] | SMIv2 TCs | conventions | StorageType; RowStatus | index by collision | conventions |
| [A-ietf-rfc2863-if-mib-2000] | IF-MIB | 25+ yrs; litigation text | lifecycle; discontinuity witness; naming scope as MVantage; ifAlias | "different" left open | schema+litigation |
| [A-ietf-rfc2790-host-resources-mib-2000] | HOST-RESOURCES | host breadth; net-snmp | pid keys; positional SW index; freshness scalar; kill-by-set | SW identity open | schema |
| [A-ietf-rfc6933-entity-mib-v4-2013] | ENTITY-MIB v4 | 4 revisions | multi/single-scoped; containment; alias map; scoped serials | agents not consistent | schema |
| [A-ietf-rfc6643-smiv2-to-yang-2012] | SMIv2→YANG | bridge | INDEX→key; foreign→leafref | persistence incompatible | translation |
| [B-schoenwaelder-pras-martinflatin-future-internet-mgmt-2003] | survey | history | config vs operational; distribution vs activation | naming open | history |

Breadth against the 47 items. These are read-backed only; an item not listed was not seen, not proven absent.
- **CIM:**
  - Seen: 1/2 via Service Started and deprecated StartMode (-GUESS that EnabledState covers the rest); 7; 14/15; 17-20 partly (UserID); 23 (read-only flag); 30 and 40 via DNSSettingData (read, not registered).
  - Not seen: 11-13, 16, 24-26, 29/31/41, 35, 36, 37/38, 39.
- **YANG:**
  - Seen: 21 (in the NMDA example), 22, 30, 35, 39, 40, 17 (name and password).
  - Not seen: packages, services, files, sysctls, mounts, processes. The standard modules are router-shaped.
- **SNMP:**
  - Seen: 21/22; 23 (read-only flag) and 46; 14/15 folded into one name string; processes; hardware.
  - Not seen: 16, crontab, firewall, sysctls, accounts, environment.

## Citations
> [A-dmtf-dsp0004-cim-infrastructure-2-8-2014]:p106 spec lines 3678-3686 (relevance: +1:SURE)
> CIM provides a mechanism to identify instances within the context of other associated instances. [...] This mechanism allows weak instances to be identifiable in a global scope even though its own key properties do not provide such uniqueness on their own. The remaining keys come from the scoping class and provide the necessary context. These keys are called propagated keys, because they are propagated from the scoping instance to the weak instance.

> [A-dmtf-dsp0004-cim-infrastructure-2-8-2014]:p107 spec lines 3747-3753 (relevance: -0:SUSPECT)
> A weak class may in turn be a scoping class for another class. [...] No more than one association may reference a weak class with a weak reference. [...] Key properties may propagate across multiple weak associations.

> [A-dmtf-dsp0004-cim-infrastructure-2-8-2014]:p79 spec lines 2627-2628 (relevance: -0:SUSPECT)
> The values of key properties and key references are determined once at instance creation time and shall not be modified afterwards.

> [A-dmtf-dsp0004-cim-infrastructure-2-8-2014]:p124 spec lines 4367-4372 (relevance: +1:SURE)
> The resulting method may or may not be able to determine whether two namespace paths reference the same namespace. For example, there may be alias names for namespaces, or different ports exposing access to the same namespace. Often, specifications using object paths need to revert to the minimally possible conclusion which is that namespace paths with equal string representations reference the same namespace, and that for namespace paths with unequal string representations no conclusion can be made about whether or not they reference the same namespace.

> [A-dmtf-dsp0004-cim-infrastructure-2-8-2014]:p127 spec lines 4425-4429 (relevance: +1:SURE)
> Two different CIM objects (e.g., instances) can still represent aspects of the same managed object. In other words, identity at the level of CIM objects is separate from identity at the level of the represented managed objects.

> [A-dmtf-dsp0004-cim-infrastructure-2-8-2014]:p122 spec lines 4293-4302 (relevance: -0:SUSPECT)
> Because CIM allows multiple implementations, it is not sufficient to think of the name of a CIM instance as just the combination of its key properties. The instance name must also identify the implementation that actually hosts the instances. [...] The compound key of non-embedded instance objects shall be unique across all non-embedded instances of the class (not including subclasses) within the namespace.

> [A-dmtf-dsp0004-cim-infrastructure-2-8-2014]:p74 spec lines 2408-2423 (relevance: +1:SURE)
> The Correlatable qualifier is used to define sets of properties that can be compared to determine if two CIM instances represent the same resource entity. For example, these instances may cross logical/physical boundaries, CIM server scopes, or implementation interfaces. [...] The property values of all role names within at least one matching organization name / set name pair shall match in order to conclude that the two instances represent the same resource entity. Otherwise, no conclusion can be reached and the instances may or may not represent the same resource entity.

> [A-dmtf-dsp0004-cim-infrastructure-2-8-2014]:p75 spec lines 2471-2477 (relevance: +1:SURE)
> InstanceID is merely an opaque identifier of a CIM instance, whereas Correlatable is not opaque and can be used to draw conclusions about the identity of the underlying resource entity of two or more instances. DMTF-defined Correlatable qualifiers are defined in the CIM Schema on a case-by-case basis. There is no central document that defines them.

> [A-dmtf-dsp0004-cim-infrastructure-2-8-2014]:p92 spec lines 3131-3147, 3165-3166 (relevance: -0:SUSPECT)
> CIM-compliant implementations can ignore optional qualifiers [...] For associations: The qualified association shall be deleted if any of the objects referenced in the association are deleted and the respective object referenced in the association is qualified with IfDeleted. [...] CIM clients shall chase associations according to the modeled semantic and delete objects appropriately.

> [B-dmtf-dsp0004-cim-metamodel-3-0-2014]:p99-100 Annex H spec lines 3225-3226 + Table H-1 (relevance: -0:SUSPECT)
> OCL: replaces 11 qualifiers (ClassConstraint, Delete, IfDelete, MaxLen, MaxValue, MethodConstraint, MinLen, MinValue, Propagated, PropertyConstraint, Weak) / Correlatable  No replacement  invariant constraint

> [B-dmtf-dsp0004-cim-metamodel-3-0-2014]:p23 spec lines 754-757 (relevance: -1:GUESS)
> The class name and the name value pairs of all key properties in an instance shall uniquely identify that instance in the scope in which it is instantiated.

> [A-dmtf-cim-logicalfile-class-2005]:L42-54 (relevance: +1:SURE)
> A unique identifier (such as a full path name) is required as a Name value. Since Files are weak to their FileSystem (and not to a Directory which would provide a more granular naming algorithm), care must be taken to make LogicalFile's Name unique for a given Creation ClassName and FileSystem. A full path name is one way to do this.

> [A-dmtf-cim-unixfile-class-2005]:L73-79 (relevance: -0:SUSPECT)
> "Count of the number of names for this file." [...] uint64 LinkCount; / "File Inode number, as printed by \"ls -i\"." [...] string FileInodeNumber;

> [A-dmtf-cim-filesystem-class-2005]:L4-7, L55-61 (relevance: -0:SUSPECT)
> A file or dataset store local to a System (such as a ComputerSystem or an ApplicationSystem) or remotely mounted from a file server. / MappingStrings { "MIB.IETF|HOST-RESOURCES-MIB.hrFSMountPoint", ... }

> [A-dmtf-cim-softwareidentity-class-2011]:L13-19, L48-75 (relevance: +1:SURE)
> SoftwareIdentity does NOT indicate whether the software is installed, executing, etc. / <LocalID> is chosen by the business entity and SHOULD not be re-used to identify different underlying (real-world) elements.

> [A-dmtf-cim-serviceaffectselement-class-2005]:L4-10 (relevance: +1:SURE)
> Instantiating this association indicates that running the service may change, manage, provide functionality for,or pose some burden on the ManagedElement.

> [A-dmtf-cim-logicalidentity-class-2005]:L4-12 (relevance: -0:SUSPECT)
> indicating that two ManagedElements represent different aspects of the same underlying entity. [...] In most scenarios, the Identity relationship is determined by the equivalence of Keys or some other identifying properties of the related Elements.

> [B-dmtf-dsp1023-software-inventory-profile-2009]:p18 spec lines 466-468, 494-496 (relevance: -0:SUSPECT)
> Software Family may be used to correlate instances of the same software across namespaces or management infrastructures, regardless of version. [...] shall belong to the same Software Family when at least one of the Software Families modeled for the first CIM_SoftwareIdentity instance matches at least one of the Software Families modeled for the second

> [A-ietf-rfc8342-nmda-2018]:L195-202 (relevance: -0:SUSPECT)
> the relationship between the branches is not machine readable

> [A-ietf-rfc8342-nmda-2018]:L737-743 (relevance: +1:SURE)
> a client can determine to what extent the intended configuration is currently in use by checking to see whether the contents of <intended> also appear in <operational>. <intended> does not persist across reboots

> [A-ietf-rfc8342-nmda-2018]:L826-836 (relevance: +1:SURE)
> Only semantic constraints MAY be violated. These are the YANG "when", "must", "mandatory", "unique", "min-elements", and "max-elements" statements; and the uniqueness of key values. [...] <operational> does not persist across reboots.

> [A-ietf-rfc8342-nmda-2018]:L849-855, L867-870 (relevance: +1:SURE)
> <operational> may contain nodes for both the previous and current configuration [...] The data appears in <intended> but does not appear in <operational>.

> [A-ietf-rfc8342-nmda-2018]:L905-937 (relevance: +1:SURE)
> As configuration flows into <operational>, it is conceptually marked with a metadata annotation [RFC7952] that indicates its origin. [...] learned [...] DHCP. [...] unknown: represents configuration for which the system cannot identify the origin.

> [A-ietf-rfc8342-nmda-2018]:L965-979 (relevance: -0:SUSPECT)
> the origin would be reported as "learned", even when a learned value is the same as the configured value.

> [A-ietf-rfc7950-yang11-2016]:L4742-4756 (relevance: +1:SURE)
> The combined values of all the leafs specified in the key are used to uniquely identify a list entry.

> [A-ietf-rfc7950-yang11-2016]:L4787-4791, L4837-4839 (relevance: -0:SUSPECT)
> MUST be unique within all list entry instances in which all referenced leafs exist or have default values. / [...] are thus not taken into account when the "unique" constraint is enforced

> [A-ietf-rfc7950-yang11-2016]:L4915-4925 (relevance: -0:SUSPECT)
> In an "ordered-by user" list, the attributes "insert" and "key" [...] can be used to control where in the list the entry is inserted.

> [A-ietf-rfc7950-yang11-2016]:L7752-7754 (relevance: +1:SURE)
> If a request modifies a configuration data node such that any node's "when" expression becomes false, then the node in the data tree with the "when" expression is deleted by the server.

> [A-ietf-rfc7950-yang11-2016]:L9376-9378 (relevance: -0:SUSPECT)
> Predicates are used only for specifying the values for the key nodes for list entries, a value of a leaf-list entry, or a positional index for a list without keys.

> [A-ietf-rfc8407-yang-guidelines-2018]:L2639-2653 (relevance: -0:SUSPECT)
> Sometimes the list keys are not identical for configuration data and the corresponding operational state. [...] It is not possible to define one approach that will be optimal for all data models.

> [A-ietf-rfc8343-ietf-interfaces-2018]:L374-390 (relevance: +1:SURE)
> the system tries to apply the interface configuration in the intended configuration with the same name as the new interface. [...] it may be impossible for an implementation to provide predictable and consistent names for system-controlled interfaces across insertion/removal cycles

> [A-ietf-rfc8343-ietf-interfaces-2018]:L462-466 (relevance: -0:SUSPECT)
> The IF-MIB allows two different ifEntries to have the same ifName. Devices that support this feature [...] cannot have a 1-1 mapping between the "name" leaf and ifName.

> [A-ietf-rfc7317-ietf-system-2014]:L1190-1198, L1212 (relevance: -0:SUSPECT)
> list server { key name; ordered-by user; [...] "An arbitrary name for the DNS server.";

> [B-openconfig-opstate-draft-2015]:L354-360 (relevance: -0:SUSPECT)
> the value of a configuration variable may not reflect the intended configured value at a given point in time.

> [B-openconfig-opstate-draft-2015]:L714-723 (relevance: -0:SUSPECT)
> in practice, it is unlikely that this list key is actually configurable in any real system

> [B-openconfig-opstate-draft-2015]:L733-737 (relevance: +1:SURE)
> It is not clear what to do when the intended and applied configuration differ. The proposal made in this document makes no presumption

> [A-ietf-rfc2578-smiv2-1999]:L1576-1581, L1671-1676 (relevance: -0:SUSPECT)
> The objects can be columnar objects from the same and/or another conceptual table [...] objects specified in a conceptual row's INDEX clause need not be columnar objects of that conceptual row.

> [A-ietf-rfc2578-smiv2-1999]:L1690-1699 (relevance: -1:GUESS)
> creation of a base conceptual row implies the correspondent creation of any conceptual row augmentations.

> [A-ietf-rfc2579-smiv2-textual-conventions-1999]:L1078-1083 (relevance: +1:SURE)
> A row which is volatile(2) is lost upon reboot. A row which is either nonVolatile(3), permanent(4) or readOnly(5), is backed up by stable storage.

> [A-ietf-rfc2579-smiv2-textual-conventions-1999]:L364-372 (relevance: -0:SUSPECT)
> `notInService', which indicates that the conceptual row exists in the agent, but is unavailable for use by the managed device

> [A-ietf-rfc2579-smiv2-textual-conventions-1999]:L644-660, L907-917 (relevance: -1:GUESS)
> the management station should simply select a new pseudo-random number and retry the operation. / it is possible for the managed device to create its own instances during the time between [...] `createAndWait' and [...] `active'.

> [A-ietf-rfc2863-if-mib-2000]:L604-626 (relevance: +1:SURE)
> its ifIndex value is not re-used by a *different* dynamically added interface until after the following re-initialization [...] The exact meaning of a "different" interface is hard to define, and there will be gray areas. Any firm definition in this document would likely turn out to be inadequate.

> [A-ietf-rfc2863-if-mib-2000]:L651-664 (relevance: +1:SURE)
> a management application can now detect counter discontinuities without the ifIndex value of the interface being changed.

> [A-ietf-rfc2863-if-mib-2000]:L689-705 (relevance: +1:SURE)
> The possibility of ifIndex value re-assignment must be accommodated by a management application whenever the value of sysUpTime is reset to zero. [...] It is the agent's choice as to whether the same or different ifIndex values identify the same or different interfaces in different naming scopes.

> [A-ietf-rfc2863-if-mib-2000]:L753-762 (relevance: -0:SUSPECT)
> an interface must retain its assigned ifAlias value across reboots, even if an agent chooses a new ifIndex value for the interface.

> [A-ietf-rfc2790-host-resources-mib-2000]:L1503-1505, L1754-1757 (relevance: +1:SURE)
> Wherever possible, this should be the system's native, unique identification number. / This value shall be in the range from 1 to the number of pieces of software installed on the host.

> [A-ietf-rfc2790-host-resources-mib-2000]:L1664-1669, L1702-1707 (relevance: -0:SUSPECT)
> Different implementations may track software in varying ways. / allows a management station to obtain a guarantee that no data in this table is older than the indicated time.

> [A-ietf-rfc2790-host-resources-mib-2000]:L1588-1591 (relevance: -1:GUESS)
> Setting this value to invalid(4) shall cause this software to stop running and to be unloaded.

> [A-ietf-rfc6933-entity-mib-v4-2013]:L250-258 (relevance: +1:SURE)
> A MIB object for which identical instance values identify different managed information in different naming scopes is called a "multi-scoped" MIB object.

> [A-ietf-rfc6933-entity-mib-v4-2013]:L423-428 (relevance: +1:SURE)
> multiple instances of the Entity MIB are not required to be equivalent or even consistent.

> [A-ietf-rfc6933-entity-mib-v4-2013]:L1554-1558 (relevance: -0:SUSPECT)
> comparisons between instances of [...] entPhysicalSerialNum objects are only meaningful amongst entPhysicalEntries with the same value of entPhysicalMfgName.

> [A-ietf-rfc6933-entity-mib-v4-2013]:L1605-1613 (relevance: -0:SUSPECT)
> including those resulting in a change of the physical entity's entPhysicalIndex value.

> [A-ietf-rfc6933-entity-mib-v4-2013]:L2162-2180 (relevance: -0:SUSPECT)
> entAliasMappingIdentifier.3.15 == rptrPortGroupIndex.5.2 / entAliasMappingIdentifier.3.22 == ifIndex.17

> [A-ietf-rfc6643-smiv2-to-yang-2012]:L1493-1508 (relevance: -0:SUSPECT)
> the persistency models of the underlying protocols, SNMP and NETCONF, are quite different.

> [B-schoenwaelder-pras-martinflatin-future-internet-mgmt-2003]:p93 (relevance: -1:GUESS)
> no consensus could be reached on [...] how much the SNMP naming system should be changed to identify nested data types on the wire.

> [B-schoenwaelder-pras-martinflatin-future-internet-mgmt-2003]:p91 (relevance: -1:GUESS)
> It is necessary to distinguish between the distribution of configurations and the activation of a certain configuration.

## Leads not pulled
- RFC 8344 (ietf-ip). Not pulled for time.
- RFC 3411 (SNMP contexts). It would sharpen the MVantage analogue.
- RFC 3780/3781/3216 (SMIng). Fetched, not read.
- DMTF DSP1004 and the filesystem, DNS-client and IP-interface profiles.
- WMI and SBLIM provider source. These would show whether deployed providers kept the path-key and Weak rules.
- CIM classes I read but did not register:
  - FileIdentity.
  - DirectoryContainsFile: at most one Directory per file.
  - Process: keyed by Handle inside OperatingSystem.
  - Account: Name in System, UserID non-key, and an LDAP DN is allowed as Name.
  - DNSSettingData: configured DNSServerAddresses vs "the actual DNS Servers being utilized", which is CIM's configured-vs-in-use seam.
- IETF/DMTF liaison documents comparing the models. Not found; -GUESS that none exists.
- The utwente author copy of the 2003 survey returned 403, so I used a course mirror.

## Search log
- Seeds (curl from rfc-editor): 11 RFCs kept; 3780 and 3216 fetched but unread.
- Seeds (curl from dmtf.org): 3 DSPs kept.
- github search_code `"<OrgID>:<LocalID>" "InstanceID" filename:CIM_ManagedElement.mof`: led to the OpenPegasus CIM241 schema; 6 class files kept.
- Two more github code searches: 0 useful results.
- Kagi (OpenConfig; Schönwälder 2003; SMI/CIM/YANG naming; opsawg): 2 kept.
- Kagi (2003 PDF; SMIng naming; RFC 3216): 1 kept.

## Tooling problems
- **Disk full.** C: reached 100% at about 14:15, and both writes of this file failed with ENOSPC. I freed about 2 MB by deleting only my own duplicate scratch PDFs, and free space kept draining.
- register.sh accepts only real PDFs as out-of-band files, so CIM MOFs had to be registered file by file.
- The register lock is heavily contended (30+ queued across lanes). One of my registrations timed out after 600 s, and 6 more were cut off by the stop order.
- There is no Python in this shell.
- UNREGISTERED at hand-off (read and graded; entry JSONs in the scratchpad as `entry-<slug>.json`): [A-dmtf-cim-logicalfile-class-2005] (lock timeout), [A-dmtf-cim-unixfile-class-2005], [A-dmtf-cim-softwareidentity-class-2011], [A-dmtf-cim-serviceaffectselement-class-2005], [A-dmtf-cim-logicalidentity-class-2005], [A-dmtf-cim-filesystem-class-2005], [B-schoenwaelder-pras-martinflatin-future-internet-mgmt-2003]. The six CIM entries point at `raw.githubusercontent.com/OpenPegasus/OpenPegasus/master/pegasus/Schemas/CIM241/DMTF/{System,Core}/CIM_*.mof`; the survey at `hit.bme.hu/~jakab/edu/litr/TMN/old/Future_Internet_Man_Tech_Pras_ComMag03Oct.pdf`.
