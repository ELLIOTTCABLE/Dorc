# gather-api-resource-identity-rfcs — resource identity in API design, IaC engines, and directory/web standards

311 read at `63e49f29` (Research/notes/311-identity-and-relation-model.md, 1063 lines). Every grade here is `graded-by: subagent`. Line numbers are from the archived copies `sources/<slug>.<ext>`. The NISTIR PDF is cited by its printed page numbers.

## Findings

- I'm +SURE this lane has no single "gold-mine" project. Each system solves identity for its own nouns, using a single minting authority. Breadth is cloud/API objects (items 24–26 and cloud analogues), package names (14, 15, 43), and directory and URI names. What carries over to 311 is the litigated rules, not the coverage. [A-kubernetes-api-conventions-identity-sections-2026] [A-google-aip-122-resource-names-2025] [A-terraform-plugin-framework-resource-identity-2025]
- Counter-thesis, confirmed ~SUSPECT-to-+SURE. Every API model here gets identity for free from one server: `uid`, `entryUUID`, a provider ID, a server-issued `resourceVersion`. Each one names the cross-authority case as out of scope or a human job:
  - Terraform: "each remote object it is managing will be bound to only one resource address, which is normally guaranteed by Terraform itself having created all objects" [B-terraform-cli-import-usage-2025]
  - LDAP: "clients MUST NOT expect to be able to perform arbitrary movements of entries and subtrees between servers" [A-ietf-rfc4511-ldap-protocol-modifydn-2006]
  - AWWW: "ownership ultimately resides in the hands of a single social entity" [A-w3c-architecture-of-the-www-vol1-2004]
  - AIP: "A resource must not change its name" [A-google-aip-180-backwards-compatibility-2025]
  - Kubernetes: cross-namespace owner references are refused, and a physical host re-created under an old Node name is "treated as the old one" [B-kubernetes-owner-references-2025] [B-kubernetes-object-names-and-ids-2025]
- The one place two minting authorities meet in code is Crossplane's `external-name`. It went badly, and it was resolved fail-closed. When the external system mints the name and the controller dies before saving it, the resource is "leaked". The runtime refuses to proceed ("The safest thing to do is to refuse to proceed") until a human deletes an annotation. The only escape is a per-resource-type declaration by the provider author, `WithDeterministicExternalName`, which is warrant-shaped. The trail is still open: crossplane#5918 (2024) and crossplane-runtime PR #850 (open 2025–2026). +SURE [A-crossplane-managed-resources-external-name-2026] [A-crossplane-runtime-managed-reconciler-2026]
- RFC 3986 §6 agrees with 311's asymmetry about unequal keys. Equality after normalization licenses "equivalent"; inequality licenses nothing ("URI comparison is not sufficient to determine whether two URIs identify different resources"). Each rung of the ladder above plain string comparison is licensed by the scheme's own specification (an mScheme owner declaring its own equivalences). AWWW adds the attribution: agents who conclude from unlicensed comparisons "take responsibility for any problems that result". +SURE [A-ietf-rfc3986-uri-generic-syntax-2005] [A-w3c-architecture-of-the-www-vol1-2004]
- CPE 2.3 Name Matching (NISTIR 7696) is the clearest refuted shape from a battle-tested standard, +SURE:
  - Two unequal wildcard-free attribute strings compare DISJOINT (Table 6-2 line 10), so unequal names license disjointness. 311 refuses exactly this without `:guarantees-unique-name`.
  - Its change log records "Removed all mention of and support for the logical value UNKNOWN."
  - It is deployed as NVD's CVE applicability matcher. [A-nist-ir7696-cpe-name-matching-2011]
- Kubernetes Server-Side Apply is the deployed analogue of 311 §3.5 (the committee law and attribution), ~SUSPECT close.
  - Ownership is recorded per field and server-side (`managedFields`). A conflicting apply is refused and names the other manager. `force` transfers ownership, equal values share it, and omitting a field releases it.
  - Schema authors declare list identity (`listType=map` + `listMapKey` vs `atomic`), which gives keyed versus whole-replace entry identity.
  - It punts outside apply. A plain update "never provokes failure", so ownership is recorded but not enforced. And after an atomic-to-granular topology change "the API server is unable to infer the new ownership". [A-kubernetes-server-side-apply-2025]
- Who declares a rename or alias differs by system and matches its layering, +SURE:
  - Terraform `moved`: the module author, scoped: "A module may only make moved statements about its own objects and objects of its child modules" [A-terraform-refactoring-moved-blocks-2025]
  - Pulumi `aliases`: the program author, including across stacks and projects. Aliases compose through the parent chain ("all combinations of parent and child aliases are computed") [A-pulumi-aliases-resource-option-2026]
  - Remote-object identity changes: only the provider author, via `MutableIdentity` [A-terraform-plugin-framework-resource-identity-2025]
  - LDAP aliases: whoever writes the directory, i.e. the store owner [A-ietf-rfc4512-ldap-models-2006]
  - AIP aliases (`users/me`): the API owner, and "all data returned from the API must use the canonical resource name" [A-google-aip-122-resource-names-2025]
  - This matches 311 §2.7 (only the transition owner declares a correspondence) in shape. None of these systems has a declarer who is a stranger to both sides.
- Unknown values, +SURE. Terraform treats unknowns as typed placeholders under a monotone refinement law. A known value in the initial plan must be identical in the final plan. An unknown may become any value of its type. "No unknown values are permitted in the New State." This is the closest deployed analogue of 311's mPlaceholder bound at a standup and of "partial measurement never widens". [A-terraform-resource-instance-change-lifecycle-2024]
- Unset vs empty vs null is litigated as a schema-author declaration per field, never inferred:
  - AIP-149: use `optional` only when the distinction matters.
  - Kubernetes: a pointer to distinguish unset from zero, and "Avoid designing APIs that require the distinction between unset and null".
  - CPE: `ANY` vs `NA`. ~SUSPECT relevant to the 311s section C finding that absence rendering is per-read. [A-kubernetes-api-conventions-identity-sections-2026] [A-nist-ir7696-cpe-name-matching-2011]
- Owner-declared equivalence recurs (analogue of per-shape warrants), +SURE:
  - AIP-129 lists the only normalizations a server may apply, and tells clients "the comparison method used to accurately compute if two values should be considered equal".
  - LDAP equality is "if and only if they would match according to the equality matching rule of the attribute type".
  - Terraform makes the provider decide "Normalization" vs "Drift".
  - [A-google-aip-129-server-modified-values-2023] [A-ietf-rfc4512-ldap-models-2006] [A-terraform-resource-instance-change-lifecycle-2024]
- AIP-129's "Fields must have a single owner, whether that is the client or the server" is a per-field ownership law. Its stated rationale is a failure mode: two owners cause "an infinite loop to correct the change". ~SUSPECT close to the committee law. [A-google-aip-129-server-modified-values-2023]
- Name vs token is universal, and the token exists for the recreated-name case:
  - Kubernetes `uid` "used to distinguish between objects with the same name that have been deleted and recreated"
  - RFC 4530 "DNs are not stable identifiers"
  - owner references carry name and UID
  - This matches GOTCHAS `a-recreated-name-is-a-new-referent`, solved by a second server-minted token. +SURE [A-kubernetes-api-conventions-identity-sections-2026] [A-ietf-rfc4530-ldap-entryuuid-2006] [B-kubernetes-owner-references-2025]
- Repology is the no-authority case at scale: about 9,300 merge rules and 5,400 split rules across roughly 300 repositories, maintained by humans since 2018.
  - Split groups discriminate by homepage URL and must end in a catch-all that flags `unclassified`, an explicit UNKNOWN, plus `warning` rules for new unexplained names.
  - Identity across authorities here is a curated, ordered rewrite system run by a committee of humans. It is not derived. +SURE [B-repology-rules-readme-2026]
- httpRange-14 lets a measurement decide which sort a name reaches (2xx, 303 or 4xx). For 4xx: "the nature of the resource is unknown". Unreachable reads unknown, not absent. ~SUSPECT relevant [B-w3c-tag-httprange14-resolution-2005]

## Candidate table

| [slug] | what it is | exhaustive / broad / abstract / battle-tested | 311 analogues | punts | altitude |
|---|---|---|---|---|---|
| [A-google-aip-122-resource-names-2025] | Google AIP: resource names | exhaustive for API nouns; narrow (cloud APIs); abstract (any API); deployed across Google Cloud since 2019, repo issues | name unique per API (mKey-Primary in mParent-Store); relative vs full name (mFullyQualifiedKey to a service-name terminus); aliases for lookup only, canonical out | cross-API identity only by prefixing the service name | schema / naming rules |
| [A-google-aip-129-server-modified-values-2023] | AIP: single-owner fields, normalization | narrow; abstract; approved 2023 | committee law (single owner per field); owner-declared equivalence per format | normalizations limited to a closed list; aliases in references unresolved (issue #1485) | schema |
| [A-google-aip-133-create-user-specified-ids-2025] | AIP: create, user-specified IDs | narrow; deployed | natural key pushed back to the client so declarative tools can find objects; ALREADY_EXISTS | server-minted-only IDs declared unfit for declarative clients | schema |
| [A-google-aip-180-backwards-compatibility-2025] | AIP: compatibility | narrow | rename = remove + add | resource names must never change | schema |
| [B-google-aip-148-standard-fields-2023] | AIP: standard fields | narrow | `uid` (mToken) beside `name`; dot-namespaced annotation keys | purpose of `uid` never stated (issue #1580) | schema |
| [A-terraform-resource-instance-change-lifecycle-2024] | Terraform core contract doc | exhaustive for one provider contract; broad (thousands of resource types); abstract; ~10 years | mPlaceholder and monotone refinement; owner-decided normalization vs drift; import ID -> stub | "exclusive control over any remote object" assumed | mutation modelling |
| [A-terraform-plugin-framework-resource-identity-2025] | TF 1.12 resource identity | narrow; young (2025) | provider-owned identity; at most one remote object per provider (warrant near `:root`); immutable, with an opt-out | identity changes in place only by the provider's opt-out | protocol / schema |
| [A-terraform-tfplugin6-protocol-identity-2026] | TF plugin protocol v6 | normative | identity must include account, endpoint, location (mFullyQualifiedKey up to the authority); version mismatch = inequal | inequality is used only as non-match | protocol |
| [A-terraform-refactoring-moved-blocks-2025] | `moved` blocks | narrow; since v1.1 (2021) | scoped transition-owner declaration; transitive chaining | default: new name = new object | language |
| [B-terraform-cli-import-usage-2025] | `terraform import` how-to | narrow | import ID per resource type (opaque token vs natural key) | one-object-one-address left to the human | usage |
| [A-kubernetes-server-side-apply-2025] | SSA reference | exhaustive for field ownership; broad (all k8s objects and CRDs); GA since 1.22 | committee law + attribution; `listMapKey` entry identity; atomic whole-replace | non-apply updates not refused; topology changes lose ownership | mutation modelling |
| [A-kubernetes-api-conventions-identity-sections-2026] | SIG-Arch API conventions | broad; ~10 years | name / uid / resourceVersion (opaque, scoped); unset vs null; references | cross-namespace references refused | schema |
| [B-kubernetes-identifiers-design-proposal-2014] | original identifiers design | narrow; historical | UID "unique in time and space" vs Name "unique within a given scope at a particular time"; a second minter (kubelet) partitioned by source hash | — | naming theory (applied) |
| [B-kubernetes-object-names-and-ids-2025] | Names and IDs page | narrow | 4-part key; API version = a second scheme for one referent | re-created host under an old Node name treated as old | schema |
| [B-kubernetes-owner-references-2025] | owner references | narrow | relation carries name + UID; who may set it is gated | unresolvable owner "treated as absent" | mutation modelling |
| [A-crossplane-managed-resources-external-name-2026] | Crossplane MR docs | broad (every provider); since ~2019 | two-authority names; fail-closed leaked-resource rule; four reference schemes | human must clear the pending annotation | mutation modelling |
| [A-crossplane-runtime-managed-reconciler-2026] | Crossplane runtime code | the implementation | refuse-to-proceed; provider-declared deterministic-name warrant | — | implementation |
| [B-crossplane-managed-resource-api-patterns-2024] | Crossplane design one-pager | narrow | CR name vs external identifier; field ownership by provider | "having two controllers managing one resource ... would not work well" | design guidance |
| [A-pulumi-resource-names-and-identity-2026] | Pulumi identity forms | broad (all providers); since 2018 | logical name / URN (mFullyQualifiedKey) / physical ID (mToken as a placeholder output) | URN change = unrelated resources | language |
| [A-pulumi-aliases-resource-option-2026] | Pulumi aliases | narrow | author-declared alias, across stacks too; composes through parents | — | language |
| [A-ietf-rfc3986-uri-generic-syntax-2005] | URI generic syntax §6 | exhaustive for URI equivalence; 20 years | equality licenses SAME; inequality never DISJOINT; scheme-owner-licensed normalization | "different" never concluded | naming theory / normative |
| [A-w3c-architecture-of-the-www-vol1-2004] | AWWW §2 | broad (the web); 2004 Recommendation | ownership chain; licensing + responsibility (attribution); opacity; representation reuse | single owner assumed; aliases discouraged | naming architecture |
| [B-w3c-tag-httprange14-resolution-2005] | TAG resolution | narrow; long-litigated | measurement decides sort; 4xx = unknown | — | naming policy |
| [A-ietf-rfc4512-ldap-models-2006] | LDAP data model | exhaustive for directories; 20+ years | RDN unique among siblings; DN recursion; per-type matching rules; store-owned aliases | — | schema |
| [A-ietf-rfc4530-ldap-entryuuid-2006] | LDAP entryUUID | narrow | server-minted immutable token because DNs recycle | replication and migration unaddressed | schema |
| [A-ietf-rfc4511-ldap-protocol-modifydn-2006] | LDAP ModifyDN | narrow | rename preserving identity; subtree re-pointing; no alias dereference | no moves across servers | protocol |
| [B-repology-rules-readme-2026] | Repology ruleset | broad (~300 repos, packages only); since 2018 | curated merge/split; `unclassified` = UNKNOWN; flavors (parts) vs versions | identity delegated to human curators | curation |
| [A-nist-ir7696-cpe-name-matching-2011] | CPE 2.3 Name Matching | exhaustive for CPE names; NVD since 2011 | set relations =, ⊂, ⊃, ≠ vs `compare()`'s four answers; ANY/NA | unequal strings -> DISJOINT; UNKNOWN removed | matching algorithm |

## Citations

> [A-google-aip-122-resource-names-2025]:L148-156 (relevance: +1:SURE)
> It is sometimes valuable to provide an alias for common lookup patterns for resource IDs. For example, an API with `users` at the top of its resource hierarchy may wish to provide `users/me` as a shortcut ... APIs **may** provide programmatic aliases for common lookup patterns. However, all data returned from the API **must** use the canonical resource name.

> [A-google-aip-122-resource-names-2025]:L46-52 (relevance: -0:SUSPECT)
> Resources **must** expose a `name` field that contains its resource name. ... Resources **may** expose a separate, system-generated unique ID field [(`uid`)](./0148.md#uid). This field **must** apply the [`OUTPUT_ONLY`](./0203.md#output-only) field behavior classification.

> [A-google-aip-122-resource-names-2025]:L164-172 (relevance: -0:SUSPECT)
> sometimes it is necessary for services to refer to resources in an arbitrary API. In this situation, the service **should** use the _full resource name_, a schemeless URI with the owning API's service name, followed by the relative resource name

> [A-google-aip-129-server-modified-values-2023]:L22-26 (relevance: +1:SURE)
> Fields **must** have a single owner, whether that is the client or the server. Server owned fields **must** be indicated with the `OUTPUT_ONLY` field_behavior. All other types of fields **must** be considered to be owned by the client. The server **must** respect the value (or lack thereof) for all client owned fields and not modify them.

> [A-google-aip-129-server-modified-values-2023]:L99-102 (relevance: +1:SURE)
> When fields do not have a single owner they can cause issues for declarative clients. These clients may attempt to set values for fields that are overwritten by server set values, leading to the client entering an infinite loop to correct the change.

> [A-google-aip-129-server-modified-values-2023]:L126-129 (relevance: -0:SUSPECT)
> Importantly, the information surfaced to clients on the normalization of a field will not describe the normalization algorithm itself, but instead the comparison method used to accurately compute if two values should be considered equal.

> [A-google-aip-133-create-user-specified-ids-2025]:L197-205 (relevance: -0:SUSPECT)
> Declarative clients use the resource ID as a way to identify a resource for applying updates and for conflict resolution. The lack of a user-specified ID means a client is unable to find the resource unless they store the identifier locally, and can result in re-creating the resource. This in turn has a downstream effect on all resources that reference it

> [A-google-aip-180-backwards-compatibility-2025]:L132-136 (relevance: +1:SURE)
> A resource **must not** change its name. Unlike most breaking changes, this affects major versions as well

> [A-google-aip-180-backwards-compatibility-2025]:L95-96 (relevance: -1:GUESS)
> **Important:** Renaming a component is semantically equivalent to "remove and add".

> [B-google-aip-148-standard-fields-2023]:L145-149 (relevance: -1:GUESS)
> The output only `string uid` field refers to a system-assigned unique identifier for a resource. When provided, this field **must** be a UUID4 ... Declarative-friendly resources **should** include this field.

> [A-terraform-resource-instance-change-lifecycle-2024]:L51-55 (relevance: +1:SURE)
> There will often be parts of the object that the provider isn't yet able to predict, either because they will be decided by the remote system during the apply step or because they are derived from configuration values from other resource instances that are themselves not yet known. The provider must mark these by including unknown values in the state objects.

> [A-terraform-resource-instance-change-lifecycle-2024]:L178-183 (relevance: +1:SURE)
> Any attribute that had a known value in the **Initial Planned State** must have an identical value in the **Final Planned State**. Any attribute that had an unknown value in the **Initial Planned State** may either remain unknown in the second _or_ take on any known value that conforms to the unknown value's type constraint.

> [A-terraform-resource-instance-change-lifecycle-2024]:L254-257 (relevance: +1:SURE)
> Although Terraform typically expects to have exclusive control over any remote object that is bound to a resource instance, in practice users may make changes to those objects outside of Terraform, causing Terraform's records of the object to become stale.

> [A-terraform-resource-instance-change-lifecycle-2024]:L271-286 (relevance: -0:SUSPECT)
> Terraform Core expects a provider to carefully distinguish between the following two situations for each attribute: **Normalization**: the remote API has returned some data in a different form than was recorded in the **Previous Run State**, but the meaning is unchanged. ... **Drift**: the remote API returned data that is materially different

> [A-terraform-plugin-framework-resource-identity-2025]:L16-18 (relevance: +1:SURE)
> The resource identity must correspond to at most one remote object per provider, across all instances of that provider. ... The identity data for a remote object must not change during its lifecycle from creation to deletion, or until the provider upgrades the resource identity schema.

> [A-terraform-plugin-framework-resource-identity-2025]:L245-259 (relevance: -0:SUSPECT)
> Error: Unexpected Identity Change ... If the remote object has an identity that can be changed without being destroyed/re-created, this validation can be disabled by setting the resource.ResourceBehavior `MutableIdentity` field to `true`

> [A-terraform-tfplugin6-protocol-identity-2026]:L126-134 (relevance: +1:SURE)
> These attributes are intended for permanent identity data and must be wholly representative of all data necessary to compare two managed resource instances with no other data. This generally should include account, endpoint, location, and automatically generated identifiers. For some resources, this may include configuration-based data, such as a required name which must be unique.

> [A-terraform-tfplugin6-protocol-identity-2026]:L122-123 (relevance: -1:GUESS)
> When comparing identity_attributes data, differing versions should always be treated as inequal.

> [A-terraform-refactoring-moved-blocks-2025]:L12 (relevance: -0:SUSPECT)
> By default, Terraform interprets a change as an instruction to destroy the existing resource and create a new resource at the new address. You can use a `moved` block to update a resource address without destroying it.

> [A-terraform-refactoring-moved-blocks-2025]:L397-409 (relevance: +1:SURE)
> The multi-module refactoring situation is unusual in that it violates the typical rule that a parent module sees its child module as a "closed box" ... This compromise assumes that all three of these modules are maintained by the same people ... A module may only make `moved` statements about its own objects and objects of its child modules.

> [B-terraform-cli-import-usage-2025]:L26-31 (relevance: +1:SURE)
> Warning: Terraform expects that each remote object it is managing will be bound to only one resource address, which is normally guaranteed by Terraform itself having created all objects. If you import existing objects into Terraform, be careful to import each remote object to only one Terraform resource address.

> [B-terraform-cli-import-usage-2025]:L69-72 (relevance: -1:GUESS)
> The syntax of the given ID is dependent on the resource type being imported. For example, AWS instances use an opaque ID issued by the EC2 API, but AWS Route53 Zones use the domain name itself.

> [A-kubernetes-server-side-apply-2025]:L43-50 (relevance: +1:SURE)
> When trying to apply an object, fields that have a different value and are owned by another manager will result in a conflict. This is done in order to signal that the operation might undo another collaborator's changes. Writes to objects with managed fields can be forced, in which case the value of any conflicted field will be overridden, and the ownership will be transferred.

> [A-kubernetes-server-side-apply-2025]:L66-70 (relevance: -0:SUSPECT)
> When two or more appliers set a field to the same value, they share ownership of that field. Any subsequent attempt to change the value of the shared field, by any of the appliers, results in a conflict. Shared field owners may give up ownership of a field by making a Server-Side Apply patch request that doesn't include that field.

> [A-kubernetes-server-side-apply-2025]:L178-179 (relevance: +1:SURE)
> Managers identify distinct workflows that are modifying the object (especially useful on conflicts!)

> [A-kubernetes-server-side-apply-2025]:L228-230 (relevance: +1:SURE)
> Unless you specify a forced override, an apply operation that encounters field-level conflicts always fails; by contrast, if you make a change using **update** that would affect a managed field, a conflict never provokes failure of the operation.

> [A-kubernetes-server-side-apply-2025]:L301-302 (relevance: -0:SUSPECT)
> `listMapKey` ... A list of field names whose values uniquely identify entries in the list. ... If configured as `atomic`, the entire list is replaced during merge. At any point in time, a single manager owns the list. If `set` or `map`, different managers can manage entries separately.

> [A-kubernetes-server-side-apply-2025]:L342-346 (relevance: -1:GUESS)
> When a `listType`, `mapType`, or `structType` changes from `atomic` to `map`/`set`/`granular`, the API server is unable to infer the new ownership of these fields. Because of that, no conflict will be produced when objects have these fields updated.

> [A-kubernetes-api-conventions-identity-sections-2026]:L240-243 (relevance: +1:SURE)
> uid: a unique in time and space value (typically an RFC 4122 generated identifier ...) used to distinguish between objects with the same name that have been deleted and recreated

> [A-kubernetes-api-conventions-identity-sections-2026]:L1169-1173 (relevance: +1:SURE)
> The only way for a client to know the expected value of resourceVersion is to have received it from the server in response to a prior operation, typically a GET. This value MUST be treated as opaque by clients and passed unmodified back to the server. Clients should not assume that the resource version has meaning across namespaces, different kinds of resources, or different servers.

> [A-kubernetes-api-conventions-identity-sections-2026]:L1259-1260 (relevance: +1:SURE)
> unclear semantics on creation. If a referenced resource is created after its reference, there is no way to know if it is the one that is expected or if it is a different one created with the same name.

> [A-kubernetes-api-conventions-identity-sections-2026]:L966 (relevance: -1:GUESS)
> Avoid designing APIs that require the distinction between unset and `null`.

> [B-kubernetes-identifiers-design-proposal-2014]:L9-18 (relevance: -0:SUSPECT)
> `UID`: A non-empty, opaque, system-generated value guaranteed to be unique in time and space; intended to distinguish between historical occurrences of similar entities. `Name`: A non-empty string guaranteed to be unique within a given scope at a particular time; used in resource URLs; provided by clients at creation time

> [B-kubernetes-identifiers-design-proposal-2014]:L98-102 (relevance: -1:GUESS)
> Since UID is not provided, kubelet generates one. Since Namespace is not provided, kubelet generates one. The generated namespace should be deterministic and cluster-unique for the source, such as a hash of the hostname and file path.

> [B-kubernetes-object-names-and-ids-2025]:L38 (relevance: +1:SURE)
> In cases when objects represent a physical entity, like a Node representing a physical host, when the host is re-created under the same name without deleting and re-creating the Node, Kubernetes treats the new host as the old one, which may lead to inconsistencies.

> [B-kubernetes-owner-references-2025]:L46-51 (relevance: -0:SUSPECT)
> Cross-namespace owner references are disallowed by design. ... A namespaced owner **must** exist in the same namespace as the dependent. If it does not, the owner reference is treated as absent, and the dependent is subject to deletion once all owners are verified absent.

> [A-crossplane-managed-resources-external-name-2026]:L573-575 (relevance: +1:SURE)
> When an external system like AWS generates nondeterministic resource names it's possible for a provider to create a resource but not record that it did. When this happens the provider can't manage the resource.

> [A-crossplane-managed-resources-external-name-2026]:L632-639 (relevance: +1:SURE)
> Anytime an external system generates a resource's name there is a risk the provider could leak the resource. The safest thing for a provider to do when it detects that it might have leaked a resource is to stop and wait for human intervention. This ensures the provider doesn't create duplicates of the leaked resource. Duplicate resources can be costly and dangerous.

> [A-crossplane-managed-resources-external-name-2026]:L699-702 (relevance: -0:SUSPECT)
> If the provider reconciled an old version with an outdated `crossplane.io/external-name` annotation it could mistakenly determine that the resource didn't exist. The provider would create a new resource, and leak the existing one.

> [A-crossplane-runtime-managed-reconciler-2026]:L1192-1196 (relevance: +1:SURE)
> If we started but never completed creation of an external resource we may have lost critical information. For example if we didn't persist an updated external name which is non-deterministic, we have leaked a resource. The safest thing to do is to refuse to proceed. However, if the resource has a deterministic external name, it is safe to proceed.

> [A-crossplane-runtime-managed-reconciler-2026]:L931-937 (relevance: +1:SURE)
> WithDeterministicExternalName specifies that the external name of the MR is deterministic. If this value is not "true", the provider will not re-queue the managed resource in scenarios where creation is deemed incomplete. This behaviour is a safeguard to avoid a leaked resource due to a non-deterministic name generated by the external system.

> [B-crossplane-managed-resource-api-patterns-2024]:L103-108 (relevance: -0:SUSPECT)
> CR name is the identifier for resource in Kubernetes environment. But for external resources, this can be either `id`, `uid` or `name` properties of the resource. The one we want to name is the one that is used when _referring to_ that resource in the same environment. For example, in AWS, VPCs do not have a name but `VpcID` properties that is used for identification.

> [B-crossplane-managed-resource-api-patterns-2024]:L271-273 (relevance: -0:SUSPECT)
> Though you might refer to it for configuration purposes, having two controllers managing one resource (VirtualNetwork CR and Subnet CR controllers) would not work well.

> [A-pulumi-resource-names-and-identity-2026]:L446 (relevance: +1:SURE)
> Any change to the URN of a resource causes the old and new resources to be treated as unrelated—the new one will be created (since it was not in the prior state) and the old one will be deleted

> [A-pulumi-resource-names-and-identity-2026]:L110-113 (relevance: -1:GUESS)
> This random suffix serves two purposes: It ensures that two stacks for the same project can be deployed without their resources colliding. ... It allows Pulumi to do zero-downtime resource updates.

> [A-pulumi-aliases-resource-option-2026]:L24 (relevance: +1:SURE)
> Pulumi identifies resources by their URN, which encodes the resource name, so the alias tells Pulumi to treat the old URN as equivalent to the new one

> [A-pulumi-aliases-resource-option-2026]:L435 (relevance: +1:SURE)
> Aliases are implicitly inherited from a parent so that if a parent is moved (new name, new type, etc.) all children will also be aliased appropriately. ... If there are aliases for both the parent and the child, all combinations of parent and child aliases are computed

> [A-ietf-rfc3986-uri-generic-syntax-2005]:L2111-2117 (relevance: +1:SURE)
> Even though it is possible to determine that two URIs are equivalent, URI comparison is not sufficient to determine whether two URIs identify different resources. For example, an owner of two different domain names could decide to serve the same resource from both, resulting in two different URIs. Therefore, comparison methods are designed to minimize false negatives while strictly avoiding false positives.

> [A-ietf-rfc3986-uri-generic-syntax-2005]:L2100-2106 (relevance: +1:SURE)
> Because URIs exist to identify resources, presumably they should be considered equivalent when they identify the same resource. However, this definition of equivalence is not of much practical use, as there is no way for an implementation to compare two resources unless it has full knowledge or control of them.

> [A-ietf-rfc3986-uri-generic-syntax-2005]:L2259-2264 (relevance: -0:SUSPECT)
> The syntax and semantics of URIs vary from scheme to scheme, as described by the defining specification for each scheme. Implementations may use scheme-specific rules, at further processing cost, to reduce the probability of false negatives.

> [A-ietf-rfc3986-uri-generic-syntax-2005]:L495-512 (relevance: -1:GUESS)
> URI references in information retrieval systems are designed to be late-binding: the result of an access is generally determined when it is accessed and may vary over time ... the resource referred to by the URI is actually a sameness of characteristics as observed over time

> [A-w3c-architecture-of-the-www-vol1-2004]:L887-893 (relevance: +1:SURE)
> URI owners are responsible for avoiding the assignment of equivalent URIs to multiple resources. Thus, if a URI scheme specification does provide for the delegation of individual or organized sets of URIs, it should take pains to ensure that ownership ultimately resides in the hands of a single social entity. Allowing multiple owners increases the likelihood of URI collisions.

> [A-w3c-architecture-of-the-www-vol1-2004]:L957-968 (relevance: +1:SURE)
> Different URIs do not necessarily refer to different resources but there is generally a higher computational cost to determine that different URIs refer to the same resource. ... Agents that reach conclusions based on comparisons that are not licensed by the relevant specifications take responsibility for any problems that result

> [A-w3c-architecture-of-the-www-vol1-2004]:L1063 (relevance: -0:SUSPECT)
> That fact that dereferencing two different URIs produces identical representations does not imply that the two URIs are aliases.

> [A-w3c-architecture-of-the-www-vol1-2004]:L1185 (relevance: -0:SUSPECT)
> Agents making use of URIs SHOULD NOT attempt to infer properties of the referenced resource.

> [B-w3c-tag-httprange14-resolution-2005]:L86-95 (relevance: -0:SUSPECT)
> a) If an "http" resource responds to a GET request with a 2xx response, then the resource identified by that URI is an information resource; b) If an "http" resource responds to a GET request with a 303 (See Other) response, then the resource identified by that URI could be any resource; c) If an "http" resource responds to a GET request with a 4xx (error) response, then the nature of the resource is unknown.

> [A-ietf-rfc4512-ldap-models-2006]:L399-404 (relevance: +1:SURE)
> No two values of an attribute may be equivalent. Two values are considered equivalent if and only if they would match according to the equality matching rule of the attribute type. Or, if the attribute type is defined with no equality matching rule, two values are equivalent if and only if they are identical.

> [A-ietf-rfc4512-ldap-models-2006]:L433-435 (relevance: -0:SUSPECT)
> An entry's relative distinguished name must be unique among all immediate subordinates of the entry's immediate superior (i.e., all siblings).

> [A-ietf-rfc4512-ldap-models-2006]:L860-878 (relevance: +1:SURE)
> Each alias entry contains, within the 'aliasedObjectName' attribute ..., a name of some object. The distinguished name of the alias entry is thus also a name for this object. NOTE - The name within the 'aliasedObjectName' is said to be pointed to by the alias. It does not have to be the distinguished name of any entry. ... Any particular entry in the DIT may have zero or more alias names. It therefore follows that several alias entries may point to the same entry. An alias entry may point to an entry that is not a leaf entry and may point to another alias entry.

> [A-ietf-rfc4530-ldap-entryuuid-2006]:L87-89 (relevance: +1:SURE)
> However, DNs are not stable identifiers. That is, a new object may be identified by a DN that previously identified another (now renamed or deleted) object.

> [A-ietf-rfc4530-ldap-entryuuid-2006]:L202-204 (relevance: -0:SUSPECT)
> Servers SHALL generate and assign a new UUID to each entry upon its addition to the directory and provide that UUID as the value of the 'entryUUID' operational attribute. An entry's UUID is immutable.

> [A-ietf-rfc4511-ldap-protocol-modifydn-2006]:L1919-1920 (relevance: -1:GUESS)
> The server SHALL NOT dereference any aliases in locating the objects named in entry or newSuperior.

> [A-ietf-rfc4511-ldap-protocol-modifydn-2006]:L1951-1957 (relevance: +1:SURE)
> Note that X.500 restricts the ModifyDN operation to affect only entries that are contained within a single server. ... In general, clients MUST NOT expect to be able to perform arbitrary movements of entries and subtrees between servers or between naming contexts.

> [B-repology-rules-readme-2026]:L5-8 (relevance: -0:SUSPECT)
> There can be a huge discrepancy in how packages for a single project are named and versioned in different repositories, so Repology needs a flexible ruleset in order to overcome the differences, match packages, and make versions comparable.

> [B-repology-rules-readme-2026]:L37-46 (relevance: +1:SURE)
> Add a group of rules to distinguish packages by upstream URL (wwwpart, wwwpat and sourceforge conditions are allowed). Such group must end with a catch-all rule for packages not matched by specific rules. ... `- { name: <ambiguous name>, addflag: unclassified }`

> [B-repology-rules-readme-2026]:L97-100 (relevance: -1:GUESS)
> Rule order matters, as multiple rules may match a single package, and they are applied in order. Furthermore, changes applied by earlier rules affect further matches

> [B-repology-rules-readme-2026]:L590-600 (relevance: -1:GUESS)
> Flavors are used to distinguish a set of packages denoting multiple versions of a project and a set of packages denoting a multiple parts or variants of a project.

> [A-nist-ir7696-cpe-name-matching-2011]:p.12 Table 6-2 (relevance: +1:SURE)
> 10 | i | k | ≠ ... (i is a wildcard-free attribute-value string, e.g., "foo"; k is a wildcard-free attribute-value string that is not identical to i, e.g., "bar") ... When comparing string literals, matching results MUST be insensitive to lexical case.

> [A-nist-ir7696-cpe-name-matching-2011]:p.13 Table 6-4 (relevance: +1:SURE)
> If any attribute relation is DISJOINT (≠) | Then CPE name relation is DISJOINT (≠)

> [A-nist-ir7696-cpe-name-matching-2011]:p.23 Appendix C (relevance: +1:SURE)
> Release 1 – 30 June 2010 ... Removed all mention of and support for the logical value UNKNOWN. ... Release 2 – 22 April 2011 ... Removed the INTERSECT function, and clarified the definitions of SUBSET and SUPERSET.

> [A-nist-ir7696-cpe-name-matching-2011]:p.8 §5 (relevance: -0:SUSPECT)
> CPE's developers have chosen not to define a single notion of "name match" because experience has shown that name matching distinctions are often use-case dependent.

> [A-nist-ir7696-cpe-name-matching-2011]:p.17 §7.3 (relevance: -0:SUSPECT)
> ;; In this specification, unquoted wildcard characters in the target ;; yield an undefined result. ... if (source = NA or target = NA) then return DISJOINT (≠).

## Leads not pulled

- KEP-555 (Server-Side Apply) and `sigs.k8s.io/structured-merge-diff` docs. They would give the design rationale and the conflict-message format (whether the error text names the other manager verbatim). I didn't pull them because the SSA reference already states the rules, and I spent the budget on the Crossplane trail. They are at github.com/kubernetes/enhancements/tree/master/keps/sig-api-machinery/555-server-side-apply.
- crossplane/crossplane#5918 and crossplane-runtime PR #850. I read both via `gh` (issue body and comments), but did not register them. They are the live trail on the leaked-resource rule failing under async providers ("lies to the crossplane-runtime reconciler about LateInitialization"). Use `gh issue view 5918 --repo crossplane/crossplane`.
- Crossplane #624, "External resource names should be configurable" (2019). It is the origin of `external-name` and lists five different pre-existing naming schemes. I read it, but did not register it; it's cited by [B-crossplane-managed-resource-api-patterns-2024].
- aip-dev/google.aip.dev issue #1580 (why `uid` is needed on declarative resources; open, no answer) and #1485 (aliases in references vs AIP-129 normalization; open). I read both, but did not register them because they are short open questions.
- OpenTofu `rfc/`. It has no RFC on resource identity, import, or moved (checked the directory listing). Resource identity was accepted as opentofu#2854 (2025), closed; Martin Atkins wrote that it must be implemented clean-room from Terraform. There is no design text to grade.
- Martin Atkins on cty unknown-value refinements (hashicorp/hcl PR #590, "Unknown Values can become more known"). This is the core-author statement of the monotone-refinement law. I didn't pull it because the lifecycle doc states the law normatively.
- Terraform issue-tracker litigation on "provider computes id" / duplicate-import detection. Kagi surfaced only third-party commercial detectors (ControlMonkey), not a core issue. A targeted `gh search issues --repo hashicorp/terraform "same remote object"` returned nothing.
- AIP-162 (revisions, `@latest` server-defined aliases), AIP-154 (etag freshness), AIP-135 (delete). I downloaded them but didn't read them in full. AIP-162's server-specified aliases are the relevant part.
- X.501 naming text: ITU-T, not freely reachable as a whole. RFC 4512 carries the adapted alias text.
- SWID (ISO/IEC 19770-2): no free normative text found; not pursued.

## Search log

- `mcp__kagi-ken__kagi_search_fetch` · terraform resource identity 1.12; resource-instance-change-lifecycle; apparentlymart unknowns; opentofu rfc identity · 4 kept
- `mcp__kagi-ken__kagi_search_fetch` · terraform duplicate import same object; moved block rationale; MutableIdentity · 2 kept
- `mcp__kagi-ken__kagi_search_fetch` · crossplane external-name docs; crossplane design doc; pulumi aliases; pulumi URN · 4 kept
- `gh search issues` · aip-dev/google.aip.dev: rename resource / uid / alias / declarative terraform · 0 registered (2 read)
- `gh search issues` · hashicorp/terraform: resource identity / moved / same remote object / duplicate import · 0 registered
- `gh search issues` · crossplane/crossplane and crossplane-runtime: external-create-pending / leaked / lost external name · 0 registered (3 read), led to the reconciler source
- `gh api` / `gh search code` · locating markdown sources (web-unified-docs, kubernetes/website, pulumi/docs, crossplane/docs, repology-rules) · n/a
- direct curl of seeds: AIPs, RFC 3986/4511/4512/4530, AWWW, httpRange-14 mail, NISTIR 7696 · 14 kept

## Tooling problems

- `register.sh`: heavy lock contention across lanes (dozens of queued registrations; `mkdir` locking is not FIFO). My first Terraform batch waited more than 10 minutes. No timeouts at the time of writing.
- `git clone` of repology-rules into the scratchpad left an empty checkout on Windows. This is likely a reserved-filename or path issue. I worked around it by reading blobs with `git show HEAD:<path>` to count rules.
- The harness system prompt pushed Bash `cat`/`sed` over Read/Write. Per the user's standing rule I used Read/Write/Edit for file text. `sed` was used only for the AWWW tag-strip (scratch reading copy); citations are from line numbers in the archived HTML.
