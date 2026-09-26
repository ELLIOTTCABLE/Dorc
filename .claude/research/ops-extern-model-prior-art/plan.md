# plan — prior art for the 311 identity-and-relation model, after the first gather pass

> Conductor (Fable), 2026-09-26, the human present. Written after reading all ten lanes' gather
> files and hand-back summaries; NO primary has been read in main context. Every grade cited here is
> a lane's (`graded-by: subagent`). Nothing here is ruled; the "Fronts" section is a proposal for
> the human's ack. Certainty words are the conductor's.

## 1 · What was asked, and the shape of the answer

The ask: a precise, broad, abstract, battle-tested specification of the behaviour of ops externs —
ideally one project that "had to write it" and reached its own conclusions (an RFC repository that
litigated the territory; an academic model built to do statistics on ops).

The answer, +SURE across ten independent lanes: **no such project exists.** Nobody has written an
exhaustive, broad, abstract, battle-tested model of ops-extern IDENTITY across mutually-unknowing
authors. Every seam confirms the counter-thesis with a quotable punt (§2). What exists instead is a
set of roughly six battle-tested PARTIAL precedents at different altitudes (§3), plus a precedent
for almost every individual 311 rule scattered across standards, kernels, and theory (§4), plus a
handful of deployed CONTRADICTIONS worth a design sitting (§5), plus a short list of genuine gaps
where nothing was found at all (§6). The prior-art value is in the rules and the punts, not in any
one document.

## 2 · The counter-thesis, confirmed (the strongest quotable punts)

- Debian DEP-17: "dpkg assumes that every filename uniquely refers to a file on disk"; twelve
  aliasing failures; modelling aliases REJECTED in favour of eliminating them
  [A-debian-dep17-usrmerge-aliasing-2023].
- Burgess & Couch 2006 (the paper unifying aspects/closures/promises): "two parameters are identical
  iff they are defined by exactly the same get and set methods" — the name is the thing, as a
  definition [A-burgess-couch-modeling-next-generation-configuration-management-2006].
- Couch et al. 2003 name "the problem of referents" and change the problem; sharing between
  closures "is yet to be determined" [A-couch-hart-idhaw-kallas-seeking-closure-open-world-2003].
- CoLiS: symbolic links "simply ignored"; Rehearsal: "Puppet manifests have no aliasing"; Citac
  proposes skipping checks "for file resources with non-overlapping file paths"
  [A-colis-platform-maintainer-scripts-2022] [A-rehearsal-puppet-determinacy-2016]
  [A-citac-reliable-convergence-2016].
- Puppet's maintainers: a `[name, provider]` key "does not provide an airtight guarantee of
  uniqueness"; "two /x directories" when mounts are involved; two modules needing one resource
  cannot coexist (open since 2014) [A-puppet-pup1073-composite-package-namevar-2013]
  [A-puppet-pup6397-mount-path-not-unique-2016]
  [A-puppet-pup1968-duplicate-resources-forge-modules-2014].
- DMTF CMDBf: cross-repository identity "is seldom absolute and often must rely on heuristics",
  by "any combination of automated analysis and manual input"
  [A-dmtf-cmdbf-federation-dsp0252-2010]. ITU-T X.720: the resource-to-managed-object relation
  "is not modelled in a general way" [A-itu-x720-management-information-model-1992].
- Whole-system provenance: "little consensus" even on how a rename is represented; "no complete
  formal models of mainstream operating systems" [A-chan-cheney-provmark-provenance-expressiveness-2019].
- Anderson & Smith 2005: "no current configuration tool presents a satisfactory answer to the
  question of how conflicts can be resolved"; a shared lexicon "could not be meaningfully defined"
  [B-anderson-smith-configuration-tools-working-together-2005]. Delaet's 2010 comparison
  framework has no identity row [B-delaet-joosen-vanbrabant-survey-system-configuration-tools-2010].
- CIM removed its only cross-authority SAME generator (`Correlatable`) in v3 with "No replacement"
  [B-dmtf-dsp0004-cim-metamodel-3-0-2014]. OpenConfig: "It is not clear what to do when the
  intended and applied configuration differ" [B-openconfig-opstate-draft-2015].
- The systems VALIDATED at scale (LCFG, Traugott's trading floors, Prodspec, dpkg, Terraform, k8s)
  carry the FLATTEST identity models: a key unique in a store, minted by one authority.

Four punt-shapes recur (lane 10's taxonomy, corroborated by every other lane): (i) the name is
the thing; (ii) one minting authority; (iii) delegated to humans or heuristics; (iv) one domain only.

## 3 · The partial precedents that meet the four criteria piecewise

Ranked by how much of 311 they cover, with altitude. ~SUSPECT on the ranking; +SURE each is real.

1. **SNIA SMI-S Clause 7, "Correlatable and Durable Names"** — a deployed three-answer
   name-equality test keyed by NameFormat (same format + same name ⇒ same; same format + different
   name ⇒ different; different formats ⇒ unknown), identity WITHHELD from ambiguous schemes, names
   unique only within a scoping system, "no name is permanently durable"; ISO standard; Windows
   refuses non-conforming providers. One domain (storage), world-registered name formats. The
   closest deployed `compare()` [A-snia-smis-correlatable-durable-names-2020].
2. **RFC 8881 (NFSv4.1) filehandles + `unique_handles` + fileid/handle classes** — the two
   warrants as SEPARATE, server-declared, absent-by-default promises, with an explicit horizon
   ("MUST use filehandle comparisons only to improve performance, not for correct behavior") and
   migration classes. One domain; 25 years of litigation
   [A-ietf-rfc8881-nfs41-fileid-fsid-attributes-2020].
3. **RETRO (OSDI 2010) + BackTracker (SOSP 2003)** — per-object-type managers each naming in its
   own scheme (committee law), process identity ⟨bootgen, pid, pidgen, execgen⟩ (lifecycle +
   recycling), namei incurring a dependency on every directory entry crossed (the emitted
   traversal), finer-grained objects only when the manager mediated all writes ("finer buys sparing,
   coarse is safe"). Single kernel's view; research-tested [A-kim-retro-selective-reexecution-2010]
   [A-king-chen-backtracking-intrusions-2003].
4. **Ntzik & Gardner's POSIX fusion logic (2015) + SibylFS (SOSP 2015) + TxOS (SOSP 2009)** — the
   path footprint over every directory crossed ("the local update has a global effect"), a heap of
   references with an explicit hard-link test class validated against 21,070 traces, and conflict
   per kernel object with parent-directory reads as a traversal read-set. Filesystem only; the one
   seam where nobody punted [A-ntzik-gardner-posix-fusion-logic-2015] [A-sibylfs-posix-oracle-2015]
   [A-porter-txos-operating-system-transactions-2009].
5. **Saltzer & Kaashoek's naming chapters + Saltzer 1978 + Lampson GNS + OntoClean** — the closed
   classical vocabulary (naming scheme, `resolve(name, context)`, unique-identifier name space,
   stable binding, limited context, synonym, indirect name, authority over absence), the two
   uniqueness rules as independent per-scheme choices, OntoClean's sufficient-vs-necessary identity
   criteria and supplies-vs-carries, GNS cache validity bounded by the arcs crossed. Broad in
   abstraction, narrow in ops domain; no write-set anywhere
   [A-saltzer-kaashoek-system-design-naming-2009] [A-saltzer-naming-and-binding-of-objects-1978]
   [A-lampson-designing-a-global-name-service-1986]
   [A-guarino-welty-identity-unity-individuality-2000].
6. **IETF NMDA (RFC 8342) + IF-MIB (RFC 2863) + ENTITY-MIB; DMTF CIM DSP0004** — the standards
   bodies' two-form seam with `origin` provenance, the ifIndex litigation with a discontinuity
   witness, multi-scoped objects (observer-dependence named), agent-authored alias maps; CIM's
   nestable propagated keys and its unique-referent-without-unique-name namespace rule. Schema
   altitude; decades deployed [A-ietf-rfc8342-nmda-2018] [A-ietf-rfc2863-if-mib-2000]
   [A-ietf-rfc6933-entity-mib-v4-2013] [A-dmtf-dsp0004-cim-infrastructure-2-8-2014].

Honourable mentions, one line each: Solaris FMRI logical/universal schemes with authority
[A-solaris-fmri-man-2011]; RFC 4007 scoped addresses [A-rfc4007-ipv6-scoped-address-architecture-2005];
the Nix store model (store-relative referents, non-injective `resolved`)
[A-nix-manual-store-path-2026] [A-nixos-rfc0062-content-addressed-paths-2019]; Kubernetes SSA
field ownership [A-kubernetes-server-side-apply-2025]; Crossplane `external-name` fail-closed
[A-crossplane-runtime-managed-reconciler-2026]; OVAL's three-layer absence partition
[A-oval-language-specification-5112-2016]; the OVAL-board `chainlink` proposal
[B-oval-community-symlink-file-object-discussion-2025].

## 4 · Precedent map: 311 rule → where it already exists (compact)

- **Two independent warrants (§1.5)** — S&K p.64; OntoClean weak ICs; RFC 8881 MUST-same vs
  `unique_handles`; RFC 2181 (refuses unique name, keeps alias→canonical a function); CIM
  namespace rule; RFC 3986 §6; SMI-S 7.7; Puppet's non-isomorphic flag (a per-type "equal names
  license nothing").
- **Identity scoped in a parent (§1.6, §2.2)** — CIM weak/propagated keys; SMI foreign INDEX;
  LDAP RDN-in-superior; X.720 containment; RFC 4007 zones; FMRI authority; Prodspec asset-per-
  partition; inmanta `index File(host, path)`; Nix store-relative paths; k8s name-per-namespace;
  Kent's qualified identification (as an anti-pattern — see §5).
- **Emitted traversal / routing invalidation (§1.7, §2.9, §3.3)** — RETRO namei dependencies;
  BackTracker parent dirs; Ntzik–Gardner path footprint; TxOS parent-dir reads; Lampson GNS
  validity by arcs crossed; OVAL `chainlink`; Plan 9 Channels remembering the rooted name;
  DNAME/wildcard occlusion; CFEngine's anchor-deleting promise losing its region.
- **Committee law and attribution (§3.5)** — promise theory (own behaviour only; naming third
  parties "can apparently lead to contradictions"); k8s SSA managers with conflicts naming the other
  manager; AIP-129 single owner per field; Guix extension (extended owner defines compose); RETRO
  per-type managers; GRoot local-not-global; Terraform `moved` scoped to a module's own objects;
  AWWW "take responsibility for any problems that result".
- **Corresponds declared by the transition owner (§2.7)** — pid/uid namespace maps; Plan 9
  auth-server user mapping; ENTITY-MIB `entAliasMappingTable`; CIM Correlatable/LogicalIdentity
  (provider-asserted); Pulumi aliases; SMI-S correlatable names.
- **Lifecycle and recycled keys (§3.3, `witness()`)** — RETRO bootgen/pidgen; IF-MIB sysUpTime +
  discontinuity time; NFS generation numbers; Grapevine "recreation of a deleted entry is not
  allowed"; k8s `uid`; LDAP `entryUUID` ("DNs are not stable identifiers"); POSIX "at any given
  time"; `machine-id(5)` clone text; systemd DynamicUser recycling; cloud-init instance-id.
- **Absent / unset / error partition (`311s` §C)** — OVAL six/four/six-valued; NSS notfound vs
  unavail; RFC 2181's four name states; systemd rc 4 (still printing `inactive`); OpenSCAP
  enumerate-then-match; Chef `current_value_does_not_exist!`; Resource API "not listed ⇒ does not
  exist".
- **Two forms, declared vs activated (`311s` B7/B8)** — NMDA + `origin`; SNMP `StorageType`;
  SMF snapshots and layers; osquery `current_value`/`config_value`; OpenConfig intended/applied.
- **Observer-dependence (§2.8)** — ENTITY-MIB multi-scoped; OVAL `environmentvariable58` pid in
  the key; osquery `pid_with_namespace` as input; WOW64 per-architecture views; `uid_map`
  rendering per reader.
- **Lends (§3.4)** — chroot "one ingredient and nothing else"; `ip netns exec` lending a mount
  namespace too; sudoers command-dependent Defaults; abstract sockets escaping chroot.
- **Unknown values / placeholders (§1.10)** — Terraform's monotone refinement law; Nix "being stuck
  is contagious"; inmanta Unknowns.

## 5 · Where deployed prior art CONTRADICTS 311 (candidates for a design sitting)

- **CPE 2.3 name matching** (NVD, since 2011): unequal strings ⇒ DISJOINT, and "Removed all
  mention of and support for the logical value UNKNOWN" — the refuted shape, deployed, and known to
  be bad (the graded CPE-vs-purl material from an earlier round). Evidence FOR 311.
- **OntoClean** mandates "every domain element must instantiate some property carrying an IC" —
  311's warrants are absent by default. A principled opposite; worth one paragraph on why 311
  refuses it (strangers' vocabularies; floor mSorts).
- **Kent, *Data and Reality*** argues AGAINST qualified (weak-entity) identification unless
  qualifiers are invariant, else "enormous update anomalies" — 311's mKey-Primary is meaningful only
  relative to its mParent-Store and answers with routing/lifecycle invalidation. Kent's three
  conditions (uniqueness within / existence of / invariance of the qualifier) read as the
  preconditions the mParent edge needs; ~SUSPECT this is a genuine tension rather than a restatement.
- **Couch & Chiarini 2008**: statically declared consistency between distributed operators is
  "intractable"; observe non-convergence instead, attributed to the observing agent. The field's
  considered alternative to speech-declared footprints (§2.5/§2.6). 311 already chose speech; the
  argument deserves an explicit answer.
- **YANG avoids positional identity by design** (ordered lists keyed by "an arbitrary name",
  order carried separately) — a counter-design to §2.9's positional catalogs and the `ufw insert`
  example. DEP-17's "eliminate the aliases rather than model them" is the same move one level up.
- **RFC 9499 split DNS** ("a domain name that is notionally globally unique has different meanings
  for different network users") contradicts §1.6's "the DNS mRoot" EXAMPLE; a DNS name's `:root`
  holds per view. Example-level, not rule-level; cheap fix.
- **Promise theory** coincides with §3.5 but assumes away §0's premise ("promises made by
  different agents cannot be inconsistent if… the promiser has an overriding control"). Not a
  contradiction of a rule; a contradiction of the problem statement, worth naming in 311's preamble.

## 6 · Gaps: 311 objects with NO precedent found in ten lanes

- KNOWN_UNSPOKEN as distinct from UNKNOWN (no classic, standard, or tool separates "never
  spoken" from "unknown"; the nearest is Repology's `unclassified` and X.720's "does not exist from
  a management point of view").
- The may-write ENTAILMENT and the FINISHED DEFINITION as an authored, closed writeset (nearest:
  YANG `when`-deletion; CIM Delete/IfDeleted, optional; Puppet autorequire, create-order only;
  Citac's semantic preservation test, measured not declared).
- The region test and `:places` (nearest: RETRO's finer-only-if-mediated rule; OVAL `chainlink`).
- `:aliases-nothing-else` as store SELF-knowledge (nearest: OVAL `complete`; SMI-S withholding
  ambiguous VPD80 serials; RFC 8881 `unique_handles`, which is the same statement made by the
  server about its own store — ~SUSPECT this one IS a precedent and should be read).

These are the places where 311 is genuinely novel, which is both a warning (no battle-testing to
borrow) and the answer to "is anyone else's altitude close enough to steal from": on these four,
no.

## 7 · Fronts (proposal; the human adjudicates)

- **front-close-the-round** (in progress, no ack needed): serial registration of the ~110 pending
  entries; citation line numbers for lanes 3, 6, 7 (resume those three lanes only, no new searches);
  `validate.sh`; commit per lane. Manifest hygiene to record, not fix: the TxOS paper registered
  twice (A vs B, two clean-context readers); one `via` field mislabelled by lane 4; the OVAL spec
  archived as a `.docx` under `.html`.
- **front-contradiction-sitting** (recommended next; no reads needed): adjudicate §5 against 311
  with the human — expected outputs are `311u` entries or 311 footers, not rule changes. Cheap,
  high value, and it consumes what the lanes already extracted.
- **front-read-the-precedents** (needs the human's ack and a reading-lane choice): full reads of
  the §3 primaries. Two shapes:
  - *main-context, scoped*: SMI-S Clause 7 (~12 printed pages), RFC 8881 §4 + the `unique_handles`
    attribute text, Ntzik–Gardner (paper, ~25 pp), RETRO §§4–5 — roughly 60–80k tokens; the
    conductor writes a per-section "precedent ledger" against 311.
  - *clean-context Fable*: the long ones — Saltzer & Kaashoek ch.2 §2.2 + ch.3 (~100 pp), RFC 8342
    whole, DSP0004's identity clauses, OntoClean 2000 — with a brief to produce the same ledger and
    to re-grade the lanes' `subagent` grades on those sources to `top-level-agent`-tier scrutiny.
  Recommendation: both, split as above; the conductor's context is roughly half used.
- **front-human-fetches** (human-as-debugger, no agent work): Couch & Sun 2004 "On observed
  reproducibility in network configuration management" (SCP; paywalled — the seam's closest thing
  to may-read/observer machinery); Comer & Peterson 1989 / the 1984 Purdue precursor (403 to curl);
  Needham "Names" (Mullender ch.); Chen et al. FSE 2020 "Understanding and Discovering Software
  Configuration Dependencies" (an empirical cross-component dependency taxonomy — §2.5/§2.6
  territory; PDF URL in lane 10's leads); CfgNet (TSE 2023); Chiarini's dissertation; the
  Vanbrabant IM 2013 IMP paper (dl.ifip.org was down).
- **front-footprint-corpora** (DEFERRED; a new front, so not now): SELinux refpolicy
  `.fc`/`.if`, AppArmor abstractions, pledge/unveil, PaSh annotations as deployed may-read /
  may-write corpora — bears on §2.5/§2.6 and the finished definition, not on identity; lane 7 and
  lane 10 both flagged it as needing a dedicated pass.
- **front-tooling-repair** (report to the human; the skill is theirs): `new-source.sh`'s download
  runs inside the lock with `--retry 2` and no `--max-time`, and the `mkdir` lock is not FIFO, so ten
  lanes starved each other into 600-second timeouts and every lane ended with queued registrations;
  a per-lane staging file merged once by the conductor, or a `--max-time`, would remove both. The
  conductor's wrapper caused the serialisation; the script's read-modify-write made it necessary.

## 8 · Ask-list

- `ask-reading-lane-choice`: main-context scoped reads, clean-context Fable, both as split above,
  or neither until after the contradiction sitting?
- `ask-contradiction-sitting-next`: run §5 as the next sitting?
- `ask-human-fetches`: which of the front-human-fetches items to fetch and drop into `sources/`
  out-of-band?
- `ask-txos-duplicate`: keep both TxOS entries (append-only manifest; the A/B split is itself a
  datum) or note one as superseded in the notes?
- `ask-lane-resumes-for-citations`: resume lanes 3, 6, 7 for citation line numbers (each has its
  reading copies and phrase lists ready; no new downloads beyond their own queued registrations)?
