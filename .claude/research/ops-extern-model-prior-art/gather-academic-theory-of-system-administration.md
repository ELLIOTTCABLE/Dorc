# gather-academic-theory-of-system-administration — the LISA/AIMS/SCP theory literature of configuration management (1994–2021)

Lane 5b. 311 read at `63e49f29`. Every grade is `graded-by: subagent`.

Locator convention: every archived copy here is a PDF. `txtL<a>-<b>` means lines of
`pdftotext -layout sources/<slug>.pdf -` (mechanical, non-LLM extraction; reproducible). Most LISA
papers are two-column, so a `-layout` line interleaves both columns. Quotes below are the one
column's sentence reassembled across those lines, verbatim in wording.

## Findings

- The theory literature DID build operator algebras (idempotence, statelessness, commutativity =
  "consistency", bands, fixed-point intersection, orthogonal product spaces, field-embedded
  absolute/relative operators). In every one of them the carrier's identity is PRESUPPOSED by the
  coordinate system: "a vector of bits called the configuration", "orthogonal regions S1, S2 of a
  product space", "a vector of its individual configuration parameters Xi". None asks when two
  coordinates are one thing. +SURE [A-couch-sun-algebraic-structure-convergence-2003]
  [A-couch-chiarini-theory-of-closure-operators-2008] [A-burgess-couch-system-rollback-totalised-fields-2011]
  [B-burgess-on-theory-of-system-administration-2000]
- HEADLINE (counter-thesis confirmed, with an explicit rule): Burgess & Couch 2006, the paper
  written to unify aspects, closures and promises, states identity outright as accessor equality:
  "two parameters are identical iff they are defined by exactly the same get and set methods", with
  location deliberately hidden behind the accessor. Same value stored in many places = many
  parameters bound by an aspect constraint of "value identity" (hostname in every /etc file). This
  is the "name is the thing" shape, stated as a definition, by the two principal theorists. +SURE
  [A-burgess-couch-modeling-next-generation-configuration-management-2006]
- Couch et al. 2003 name "the problem of referents" explicitly — "any naming scheme complex enough
  to precisely specify a subsystem is too complex to remember" — and resolve it by CHANGING THE
  PROBLEM (referents become behaviours tested yes/no) rather than modelling aliasing; closure
  consistency is "only nontrivial when two closures share a resource or parameter. The exact
  nature of that sharing is yet to be determined." The clearest explicit punt in the lane. +SURE
  [A-couch-hart-idhaw-kallas-seeking-closure-open-world-2003]
- Promise theory DOES carry a committee-law analogue — "A promising agent can only describe its
  own behavior or behaviors of others that it has directly observed"; "Agents ... can only make
  promises about their own behaviour"; referring to third parties in a promise body "can apparently
  lead to contradictions ... avoid this where possible" — but its identity story is attached, not
  derived ("inanimate objects to which we attach identity"), and non-interference is ASSUMED via
  "overriding control": promises from different agents "cannot be inconsistent if promise bodies
  are dealing with ... states of affairs about which the promiser has an overriding control", and a
  conflict proves someone lacked control, repaired by trust update. So: 311's committee law
  coincides; 311's premise that strangers' things alias undetectably is exactly what promise theory
  assumes away. ~SUSPECT (on the "coincides/assumes away" reading; the quotes are +SURE)
  [A-burgess-couch-modeling-next-generation-configuration-management-2006]
  [B-bergstra-burgess-static-theory-of-promises-2014] [B-burgess-some-notes-about-promise-theory-2015]
- Promise theory's own CFEngine binding punts identity to host naming: "The promiser and promisees
  are named in whatever convention is used by the system, e.g. filesystem names, URIs, process
  names ... Aliases can be improvised using ... variables". +SURE
  [B-burgess-some-notes-about-promise-theory-2015]
- Couch & Chiarini 2008 abandon declared (syntactic) consistency between distributed operators as
  intractable and replace it with OBSERVED non-convergence ("relative to the observer"; "the agent
  discovering the inconsistency is exactly the agent whose operators should change"). This is the
  field's considered alternative to 311's speech-based interference: measure, never declare
  footprints. +SURE on content; relevance ~SUSPECT
  [A-couch-chiarini-dynamic-consistency-convergent-operators-2008]
- Multiple authors: every source answers by SEPARATION, never by a relation. "One manager for one
  aspect" [A-burgess-couch-modeling-next-generation-configuration-management-2006]; the closure
  "will presume that it is the sole manager of its configuration"
  [A-couch-hart-idhaw-kallas-seeking-closure-open-world-2003]; "Never log into a machine to change
  anything on it" [B-traugott-huddleston-bootstrapping-infrastructure-1998]; "Only one automation
  system should be responsible for a given part of production"
  [B-palatin-prodspec-annealing-intent-based-actuation-2021]; per-subsystem class scripts owning
  `host.subsystem.attribute` [B-anderson-towards-high-level-machine-configuration-1994]. Anderson &
  Smith: "no current configuration tool presents a satisfactory answer to the question of how
  conflicts can be resolved" and a shared lexicon "could not be meaningfully defined"
  [B-anderson-smith-configuration-tools-working-together-2005]. +SURE
- Battle-tested identity rules are primary-key-in-a-store, full stop: Prodspec "An asset ID must be
  unique within a partition" and "requires clean resource naming from the infrastructure provider"
  [B-palatin-prodspec-annealing-intent-based-actuation-2021]; Puppet/BCFG modality-conflict
  detection "allowing certain parameters of resources to be unique within a device, for example
  the filename of file resources" [B-delaet-joosen-vanbrabant-survey-system-configuration-tools-2010];
  LCFG `host.subsystem.attribute` [B-anderson-towards-high-level-machine-configuration-1994]. These
  are 311's mKey-Primary-in-mParent-Store with an implicit `:guarantees-unique-name` taken for
  granted and no mScheme/`:yields` layer at all. ~SUSPECT (mapping is mine)
- Validation: theory papers (Couch & Sun, Couch & Chiarini, Burgess & Couch 2006/2011, promise
  theory) are unvalidated or toy-prototype (Tufts Masters prototypes, /etc/services only). Validated
  systems — LCFG at Edinburgh (300–400 hosts, 1994), Traugott's trading floors (~15,000 hosts),
  Prodspec at Google ("millions of resources") — are exactly the ones with the flattest identity
  models (a key, a gold server, an asset ID). +SURE
- The Edinburgh line's formal endpoint (ConfSolve, 2012) makes identity trivial by closing the
  world: an object IS its allocation in the model, a `ref` resolves only to instances declared in
  the model, and children's lifetime is tied to their parent. It is also deliberately
  referent-agnostic ("no built-in classes with special meanings such as Machine or File"), which
  coincides with 311's refusal of engine-side world knowledge — but it buys that by owning the
  whole world, the opposite of 311's mutually-unknowing authors. +SURE on content, ~SUSPECT on the
  mapping [B-hewson-anderson-gordon-declarative-automated-configuration-2012]
- The comparison framework of the field (Delaet 2010) has rows for relations, conflicts, access
  control — and NO row for identity or aliasing. Counter-thesis evidence at the survey level.
  +SURE [B-delaet-joosen-vanbrabant-survey-system-configuration-tools-2010]
- One near-miss worth the conductor's eye: Traugott warns that `/net/<server>` makes "host names
  become a part of the file name", breaking moves — a practitioner's statement of 311's
  route-qualified-handle failure of `:guarantees-unique-name`. -GUESS on the mapping
  [B-traugott-huddleston-bootstrapping-infrastructure-1998]
- Brief correction: "The maelstrom" (LISA 2001) is Couch & Daniels, not Couch & Sun (every citing
  paper here agrees). +SURE [A-couch-hart-idhaw-kallas-seeking-closure-open-world-2003]

## Candidate table

| slug | what it is | exhaustive / broad / abstract / battle-tested | 311 analogues | punts | altitude |
|---|---|---|---|---|---|
| [A-burgess-couch-modeling-next-generation-configuration-management-2006] | unifying theory: aspects, closures, promises | not exhaustive / broad examples (hostname, DNS–DHCP–MAC, resolv.conf, packages, NFS) / abstract / not validated | committee law (promise principle 1); cell≈aspect; `:corresponds`≈"value identity" constraint; binding as two-sided | identity = same get/set methods; location hidden | mutation + naming, theory |
| [A-couch-hart-idhaw-kallas-seeking-closure-open-world-2003] | closure theory + prototypes | no / web, services, /etc/services, rpm / abstract / student prototypes | per-author sole ownership; exterior vs interior params ≈ may-read vs hidden; union-not-closure = interference | "problem of referents" dodged; shared-parameter sharing "yet to be determined" | mutation theory |
| [A-couch-sun-algebraic-structure-convergence-2003] | semigroup/band algebra of actions | no / files, chmod, editfiles / highly abstract / no | commutation = consistency ≈ DISJOINT-or-agreeing; statelessness | parameters treated as atoms; baseline deferred | mutation algebra |
| [A-couch-chiarini-dynamic-consistency-convergent-operators-2008] | statistical consistency of operators | no / mount→web examples / abstract / no | attribution to the observing agent | declared consistency called intractable | mutation, observed |
| [A-couch-chiarini-theory-of-closure-operators-2008] | acceptance ⊇ assurance operators | no / web server / abstract / no | orthogonality = DISJOINT premise | product-space decomposition assumed | mutation algebra |
| [A-burgess-couch-system-rollback-totalised-fields-2011] | field-embedded operators, rollback impossibility | no / scalar params / abstract / no | lifecycle-ish: absolute C(q) erases history | non-orthogonal dependencies an open problem; one policy per data item | mutation algebra |
| [B-burgess-on-theory-of-system-administration-2000] | foundational theory (preprint) | primitive-op table (file create/delete/rename/edit, ACL, mount, copy, process, device) / abstract / no | none direct | redundancy group G "of no concern"; pid a label | mutation, game theory |
| [B-bergstra-burgess-static-theory-of-promises-2014] | promise theory foundations | no / generic / abstract / no | committee law; observer-relative keeping | identity "attached"; conflict impossible under "overriding control" | naming/speech theory |
| [B-burgess-some-notes-about-promise-theory-2015] | promise theory applied (CFEngine, BGP, YANG) | no / files, processes, interfaces, BGP, DB rows, containers / semi-abstract / CFEngine deployed | committee law; agent boundary is a modelling choice | promiser named by host convention; aliases via variables | speech/modelling |
| [B-anderson-smith-configuration-tools-working-together-2005] | multi-tool interchange proposal | no / generic / semi / no | multiple authors; origin provenance ≈ attribution | lexicon sidestepped; "no late binding" | schema |
| [B-anderson-towards-high-level-machine-configuration-1994] | LCFG origin | ~400 resources / DNS, amd, auth, inetd, www, NIS / concrete / Edinburgh 300–400 hosts | fixed key scheme ≈ mFullyQualifiedKey with host root | the key is the thing | schema + deployment |
| [B-traugott-huddleston-bootstrapping-infrastructure-1998] | gold-server practice | 16-step sequence / DNS, NIS, NFS, automount, crontab, apps / concrete / ~15,000 hosts | route-qualified handle warning | identity imposed by uniformity + single author | practice |
| [B-delaet-joosen-vanbrabant-survey-system-configuration-tools-2010] | comparison framework, 11 tools | framework-exhaustive, not state-exhaustive / secondary | modality conflict = same-name collision | no identity row at all | assessment of tools |
| [B-hewson-anderson-gordon-declarative-automated-configuration-2012] | ConfSolve: typed OO constraint config language → CSP | no / VMs, racks, DB roles, Cauldron suite / formally specified / benchmarks only | mParent via nesting + lifetime; refs to declared instances; referent-agnostic classes | closed world: identity = allocation in the model | schema + synthesis |
| [B-palatin-prodspec-annealing-intent-based-actuation-2021] | Google intent actuation | generic assets / jobs, LB, schema, firmware / abstract data model / Google-scale | mKey-Primary in mParent-Store; references as marked fields; cells ("distinct aspects") | identity = provider's clean naming | mutation + schema |

## Citations

> [A-burgess-couch-modeling-next-generation-configuration-management-2006]:txtL167-178 (relevance: +1:SURE)
> "Definition 1: Configuration parameter A configuration parameter is a unit of configuration information. It can be manipulated by use of specified get and set methods, where get returns the parameter's value and set specifies a new value. A parameter's location within the system is not important, we refer to it indirectly, i.e., by a method or access service which conceals that specific location."

> [A-burgess-couch-modeling-next-generation-configuration-management-2006]:txtL305-308 (relevance: +1:SURE)
> "Since the definition of a parameter arises from the ability to get and set it, two parameters are identical iff they are defined by exactly the same get and set methods."

> [A-burgess-couch-modeling-next-generation-configuration-management-2006]:txtL196-225 (relevance: -0:SUSPECT)
> "Suppose that two data values are required to have the same value, but are stored in different places and accessed via different means. They are different parameters, but can be considered to be members of the same aspect, bound together by the aspect constraint of ``value identity.''" ... "Consider the number of times the hostname of the current host appears inside files in /etc. Under our definition, each occurrence is a separate parameter, but the aspect ``hostname'' embodies all of them"

> [A-burgess-couch-modeling-next-generation-configuration-management-2006]:txtL733-752 (relevance: +1:SURE)
> "1. A promising agent can only describe its own behavior or behaviors of others that it has directly observed. ... 3. There is no reason to identify an autonomous agent with a ``system'' or a ``machine.'' One can create many autonomous agents within a single system, as components."

> [A-burgess-couch-modeling-next-generation-configuration-management-2006]:txtL419-438 (relevance: -0:SUSPECT)
> "1. Factor services onto independent closures, e.g., virtual machines where possible (to eliminate aspect overlaps). 2. One manager for one aspect. Maintain clear separations in the source of aspect control ... 4. Document all remaining overlaps"

> [A-burgess-couch-modeling-next-generation-configuration-management-2006]:txtL391-396 (relevance: -1:GUESS)
> "The strength of generative configuration management is that identity relationships among aspects (where several parameters must have precisely the same value) are addressed by generating multiple files from the same hierarchy of values, thus solving the aspect consistency problem implicitly."

> [A-couch-hart-idhaw-kallas-seeking-closure-open-world-2003]:txtL86-111 (relevance: +1:SURE)
> "The problem of referents [11, 18, 19] arises from the complexity of the systems being configured. In a large and complex network, how does one specify how a particular subsystem should behave? This is a matter of referring to the subsystem and its parameters ``by name'' and assigning values to each parameter. The problem is that any naming scheme complex enough to precisely specify a subsystem is too complex to remember and use effectively." ... "clever tricks such as value inheritance and environmental acquisition do not eliminate the problem; they simply transform the problem of referents into the equivalent problem of keeping inherited attributes correct ... the solution is not to refine the solution, but to change the problem."

> [A-couch-hart-idhaw-kallas-seeking-closure-open-world-2003]:txtL175-180 (relevance: -0:SUSPECT)
> "A ``closure'' is a programming language term [41] for a name-binding environment in which setting a variable and then reading it always gives the value to which it was set, independent of the settings of other things. In a closure, the meanings of names are independent of one another, unique, and persistent."

> [A-couch-hart-idhaw-kallas-seeking-closure-open-world-2003]:txtL696-705 (relevance: +1:SURE)
> "If the closures have disjoint parameter spaces, then they are trivially consistent because conflicts are impossible. Consistency is only nontrivial when two closures share a resource or parameter. The exact nature of that sharing is yet to be determined, and there is a danger of over-limiting closures so that they become impractical to construct."

> [A-couch-hart-idhaw-kallas-seeking-closure-open-world-2003]:txtL336-340 (relevance: -0:SUSPECT)
> "Our web closure will presume that it is the sole manager of its configuration. Any violation of that contract will seriously affect our closure's ability to manage itself, because there will be latent effects of changes that the closure did not make during self-management."

> [A-couch-hart-idhaw-kallas-seeking-closure-open-world-2003]:txtL475-495 (relevance: -1:GUESS)
> "The union of several closures need not be a closure. Consider two closures: 1. rpm --install ... 2. make install ... The problem comes from the way they interact when a make install (using autoconf) binds to a dynamic library provided by a distinct rpm -install."

> [A-couch-sun-algebraic-structure-convergence-2003]:txtL74-77 (relevance: +1:SURE)
> "To simplify notation, we ignore parameters for actions. Similar actions with different parameters are treated as differing actions."

> [A-couch-sun-algebraic-structure-convergence-2003]:txtL337-339,353-357 (relevance: +1:SURE)
> "We have thus proven that configuration parameters exist in all cases, without assuming that they do. Existence of parameters is an algebraic property that arises from assumptions of action equivalence, idempotence, and statelessness!" ... "consider each action to be operating on an extremely large vector of bits called ``the configuration''. Every action asserts some fixed values for a subset of bits of the configuration. Actions that agree on any common values commute, including actions that act on different parts of the configuration and thus cannot conflict."

> [A-couch-sun-algebraic-structure-convergence-2003]:txtL158-162 (relevance: -1:GUESS)
> "idempotence and convergence of operations is only meaningful relative to a choice of baseline state of a system ... we will presume that the baseline state is appropriately limited so that idempotence is present, and defer study of the structure of the baseline state for later work."

> [A-couch-chiarini-theory-of-closure-operators-2008]:txtL466-475 (relevance: -0:SUSPECT)
> "Theorem 3. Suppose that two operators O1, O2 operate on orthogonal regions S1, S2 of a product space S1 × S2, so that O1 affects the chosen subset of S1 and O2 affects S2 only. Then {O1, O2} is a closure operator. ... In other words, closure operators acting on independent entities can be composed into one closure operator."

> [A-couch-chiarini-theory-of-closure-operators-2008]:txtL148-156 (relevance: -1:GUESS)
> "But one operator's ``arbitrary'' might be another operator's ``crucial''; consider an operator O2 whose goal is to tune the performance of the web server. Then the choice of document root — unimportant to the basic act of setting up the web server — changes from incidental to crucial"

> [A-couch-chiarini-dynamic-consistency-convergent-operators-2008]:txtL171-180,247-253 (relevance: -0:SUSPECT)
> "we conclude that the problem of statically determining whether a particular set of distributed operators share a fixed point is intractable" ... "Whether O2 and O4 conflict depends upon what they mean or intend, rather than how they are coded. ... the codings — and their semantics — are distributed in a network, and centrally collecting that information in a ubiquitous network is both intractable and unreasonable."

> [A-couch-chiarini-dynamic-consistency-convergent-operators-2008]:txtL291-296,471-473 (relevance: -0:SUSPECT)
> "Agents reason on their own and in isolation. ... The definition of an inconsistent set of operators is thus relative to the observer." ... "The key to the usefulness of this theorem in practice is that the agent discovering the inconsistency is exactly the agent whose operators should change to react to it."

> [A-burgess-couch-system-rollback-totalised-fields-2011]:txtL793-795 (relevance: -1:GUESS)
> "In configuration terms, one cannot have more than one policy for a data item"

> [A-burgess-couch-system-rollback-totalised-fields-2011]:txtL900-902,1032-1040 (relevance: -0:SUSPECT)
> "consider a system as a controlled by a vector of its individual configuration parameters Xi" ... "Problem 2 We have not taken into account operators that depend on one another in non-orthogonal fashion[22]. Dependencies between operators add potentially severe complications to this account. There is a deeper issue with roll-back in partial systems. If a system is in contact with another system ... then the other system becomes a part of the total system and we must write a hypothetical journal for the entire system"

> [B-burgess-on-theory-of-system-administration-2000]:txtL284-291,446-449 (relevance: -1:GUESS)
> "A complete configuration instruction can be thought of geometrically as a point in a vector space, which is found by adding together instructions of linearly independent (orthogonal) types." ... "The permutation or invariance group G is of no concern to this paper except as a matter of principle for the most pedantic."

> [B-burgess-on-theory-of-system-administration-2000]:txtL813-826,868-872 (relevance: -1:GUESS)
> Primitive type table: "Create file / Delete file / Rename file / Edit file / Access control / Request resource (Read/Mount) / Copy file / Process control / Process priority / Configure device" ... "it would be unimportant if one swapped the process ID's of two programs. The process ID is just a label which has no bearing on the performance of the system"

> [B-bergstra-burgess-static-theory-of-promises-2014]:txtL494-496,540-546 (relevance: -0:SUSPECT)
> "The promiser and promisee are both assumed to be `agents', i.e. humans or inanimate objects to which we attach identity in the story of promises." ... "A promiser does not have to reveal its identity of course ... The lack of such information about a promiser is simply a defect in the knowledge of the receiving agent"

> [B-bergstra-burgess-static-theory-of-promises-2014]:txtL1514-1530 (relevance: +1:SURE)
> "Promises are local constructions, whereas obligations are non-local. The source of a promise is localized in a single entity that has all of the information and self-control to be available to resolve conflicts and problems with multiple promises. ... obligations can be inconsistent, but promises cannot. More precisely: consistency of promises is a matter that can be verified at the level of sources only. Promises made by different agents cannot be inconsistent if promise bodies are dealing with actions or states of affairs about which the promiser has an overriding control against other agents. If different agents issue conflicting promises at least one of these fails to have the expected degree of control. In any case once such conflicting promise are noticed the trust in both issuers needs reconsideration"

> [B-burgess-some-notes-about-promise-theory-2015]:txtL264-278 (relevance: +1:SURE)
> "1. Agents are autonomous. They can only make promises about their own behaviour. No other agent can impose a promise upon them. ... 5. The internal workings of agents are assumed to be unknown. ... we may choose the boundary of an agent wherever we please to hide or expose different levels of information"

> [B-burgess-some-notes-about-promise-theory-2015]:txtL579-582,452-454 (relevance: -0:SUSPECT)
> "We can now suppose what happens when referring to third-party agents in the body of a promise. In some circumstances, this can apparently lead to contradictions. As a general rule, it seems to be in the interest of clarity to avoid this where possible." ... "Assume that common information known to different agents is inconsistent, i.e. that each agent has its own version which may or may not agree with one another."

> [B-burgess-some-notes-about-promise-theory-2015]:txtL608-611 (relevance: +1:SURE)
> "The CFEngine syntax is a flat key-value language, which highlights promiser and promisees, and has a fixed type set. The promiser and promisees are named in whatever convention is used by the system, e.g. filesystem names, URIs, process names, storage devices, etc. Aliases can be improvised using the general capacity to make variables and manipulate data."

> [B-anderson-smith-configuration-tools-working-together-2005]:txtL145-157 (relevance: -0:SUSPECT)
> "A configuration description specifies a complete, unambiguous and instantaneous configuration for the target machine or system. ... Unambiguous means that no further logic is required to determine the parameters of the deployment engine. There are no references, no late binding, nor any database queries required."

> [B-anderson-smith-configuration-tools-working-together-2005]:txtL258-266 (relevance: +1:SURE)
> "We have sidestepped the issue of a standard lexicon (that is, defining a standard set of keys whose values should be similarly interpreted by all compliant deployment engines.) We argue that such a standard lexicon could not be meaningfully defined in the present environment, where there is so little commonality between existing tools."

> [B-anderson-smith-configuration-tools-working-together-2005]:txtL285-298 (relevance: +1:SURE)
> "Consider a ``highly-secured'' class, and a ``web-server'' class, each managed by different teams. These classes have different objectives, and overlapping domains. It is entirely possible specifications from one class will conflict with another. To our knowledge, no current configuration tool presents a satisfactory answer to the question of how conflicts can be resolved."

> [B-anderson-towards-high-level-machine-configuration-1994]:txtL179-182,83-86 (relevance: -0:SUSPECT)
> "The configuration scripts use common routines to consult the database for resources of the form host.subsystem.attribute = value" ... "The lack of modularity in the configuration process also makes it difficult for different people to maintain the configuration of separate subsystems"

> [B-traugott-huddleston-bootstrapping-infrastructure-1998]:txtL279-283 (relevance: -0:SUSPECT)
> "We developed a rule that worked very well in practice and saved us a lot of heartache: ``Never log into a machine to change anything on it. Always make the change on the gold server and let the change propagate out.''"

> [B-traugott-huddleston-bootstrapping-infrastructure-1998]:txtL617-628 (relevance: -1:GUESS)
> "Another serious danger is the use of /net. ... Worse, it reduces the flexibility of your infrastructure, because host names become a part of the file name. This prevents you from moving a file to a new server without changing every script and configuration file which refers to it."

> [B-traugott-huddleston-bootstrapping-infrastructure-1998]:txtL566-575 (relevance: -1:GUESS)
> "In keeping with the virtual machine concept, it is important that every process on every host see the exact same file namespace."

> [B-delaet-joosen-vanbrabant-survey-system-configuration-tools-2010]:txtL592-600 (relevance: +1:SURE)
> "BCFG and Puppet can detect modality conflict such as a file managed twice in a specification. Cfengine3 also detects modality conflicts such as an instable configuration that does not converge. ... Puppet also supports modality conflicts by allowing certain parameters of resources to be unique within a device, for example the filename of file resources."

> [B-delaet-joosen-vanbrabant-survey-system-configuration-tools-2010]:txtL462-468 (relevance: -1:GUESS)
> "In most systems, this is based on the path of the file that contains the code or specification. But in most programming languages and system configuration tools, the relation between the name of the file and the contents of the file is very limited or even non-existing."

> [B-hewson-anderson-gordon-declarative-automated-configuration-2012]:txtL74-80,118-122,273-277 (relevance: -1:GUESS)
> "3. All classes are equal: there are no built-in classes with special meanings such as Machine or File." ... "The lifetime of the NetworkInterface instance is tied to that of its parent object, and is not shared between different instances of Machine." ... "Object reference variables may be declared using var v as ref c. This creates a reference which will resolve at solve-time to an instance of c elsewhere in the model, whereas the declaration var v as c allocates a new instance of c."

> [B-hewson-anderson-gordon-declarative-automated-configuration-2012]:txtL796-800 (relevance: -1:GUESS)
> "In the special case of references, the value is the fully-qualified name of the target variable, in which members of sets may be accessed via index"

> [B-palatin-prodspec-annealing-intent-based-actuation-2021]:txtL375-383,421-434 (relevance: +1:SURE)
> "An asset is a very generic abstraction with the following structure: A string identifier, simply called ``asset ID''. A payload. Zero or more addons. ... The type of the payload defines the type of the asset." ... "An asset ID must be unique within a partition. Assets within partitions are not structured ... asset fields can contain references to other assets. This makes it possible to create multiple hierarchies on the same assets. ... Those reference fields are explicitly marked as such"

> [B-palatin-prodspec-annealing-intent-based-actuation-2021]:txtL300-302 (relevance: +1:SURE)
> "Prodspec is authoritative and Annealing is built to recover state as needed from production. This avoids the need of a state file like Terraform, but requires clean resource naming from the infrastructure provider and impacts turndown management."

> [B-palatin-prodspec-annealing-intent-based-actuation-2021]:txtL900-904,828-831 (relevance: -0:SUSPECT)
> "Only one automation system should be responsible for a given part of production." ... "Intent-based rollouts allow for rollouts impacting the same assets running in parallel ... The only constraint is that those rollouts must modify distinct aspects of the assets"

> [B-palatin-prodspec-annealing-intent-based-actuation-2021]:txtL1001-1009 (relevance: -1:GUESS)
> "Many of our users have requested ``turndown-by-absence'' ... we generally won't support it, as it poses too great a risk to production at large. The most common failure mode of configuration systems is returning partial content"

## Leads not pulled

- Couch & Sun, "On observed reproducibility in network configuration management", SCP 53 (2004) ·
  a state-machine model distinguishing actual vs OBSERVED state and "completely observed
  parameters" — plausibly the closest thing in this literature to 311's may-read/observer
  machinery · paywalled (ScienceDirect); academia.edu copy needs login; not on Couch's Tufts page ·
  ask the human to fetch, or ResearchGate request.
- Burgess, "An approach to understanding policy based on autonomy and voluntary cooperation",
  DSOM 2005, LNCS 3775 · the foundational promise-theory paper · dl.ifip.org connection timed out
  twice (see Tooling) · retry `https://dl.ifip.org/db/conf/dsom/dsom2005/Burgess05.pdf`.
- Vanbrabant & Joosen, "A framework for integrated configuration management tools" (IMP), IM 2013 ·
  "models all relevant interdependencies between parameters" per its abstract · same dl.ifip.org
  outage · retry `https://dl.ifip.org/db/conf/im/im2013/BrabantJ13.pdf` or KU Leuven lirias.
- Ramshaw, Sahai, Saxe, Singhal, "Cauldron: a policy-based design tool" (POLICY 2006) and DMTF CIM ·
  CIM-based object model with references, cited by ConfSolve as its closest ancestor · CIM is
  standards (lane 1); Cauldron paper not pulled · IEEE.
- Couch & Daniels, "The maelstrom: network service debugging via 'ineffective procedures'", LISA
  2001 · ordering theorem (n² repetitions satisfy unknown precedences) · summarised faithfully inside
  [A-couch-chiarini-dynamic-consistency-convergent-operators-2008]; low identity relevance ·
  usenix legacy lisa01.
- Marc Chiarini's dissertation "Minimizing the Cost of Configuration Changes in Self-Managing Systems
  via Closures, Matchings, and Marriages" (Tufts 2009) · only an ACM DL landing page found · Tufts
  Digital Library / ProQuest.
- Anderson, "System Configuration", SAGE Short Topics #14 (2006) · defines "aspect" as "a part of
  configuration specified by one human person or administrator" (per the 2006 Burgess & Couch
  paper, txtL124-125) — a per-author unit · print booklet, not found free.
- Desai et al., "Directing change using Bcfg2", LISA 2006 · read in full; no identity content
  (revision-numbered change orchestration only); not registered for lack of relevance. Earlier Bcfg
  papers (2003 tech report; "pay-as-you-go") not pulled.
- Burgess, "A site configuration engine" (Computing Systems 1995) at `markburgess.org/papers/paper1.pdf`
  · not registered because an earlier round graded a Burgess/CFEngine convergence source that may be
  this paper.
- Couch et al. AIMS 2009 "Dynamics of resource closure operators" and AIMS 2007 "Estimating
  reliability of conditional promises" · downloaded, not read · Tufts publications page.

## Search log

- kagi · Couch Chiarini dynamic consistency; Burgess Couch rollback; Couch Sun algebraic structure; Anderson Smith working together; Delaet survey; Prodspec · kept 5
- kagi · Seeking closure; theory of closures; Chiarini dissertation; Burgess theory of sysadmin; Traugott bootstrapping; Desai Bcfg2 directing change · kept 4
- curl · Couch Tufts publications directory listing · surfaced aims-08-mael, aims-08-ops · kept 2
- (via citation) Burgess & Couch LISA 2006, from the AIMS 2008 reference lists · kept 1; Anderson 1994 from Anderson & Smith refs · kept 1
- kagi · Burgess DSOM 2005 policy/autonomy; Hewson ConfSolve; Vanbrabant IMP; Couch Sun observed reproducibility; Anderson SAGE booklet; promise theory "own behaviour" · kept 1 (PromiseMethod)
- kagi · Bergstra Burgess static theory of promises; promise theory agent identity; Prodspec loginonline · kept 2
- kagi · Chiarini dissertation pdf; ontology of system administration identity; Couch cost model; observed reproducibility tufts · kept 0 (ontology hits were software-CM / network-device ontology papers, off-remit)

## Tooling problems

- `dl.ifip.org` refused connections on three attempts (AIMS 2008, DSOM 2005, IM 2013). AIMS 2008
  read from the author's Tufts copy; the other two are Leads.
- LOCK TIMEOUT (per brief, registration stopped): `register.sh` reported "lock timeout after 600s"
  for [B-traugott-huddleston-bootstrapping-infrastructure-1998] and
  [B-palatin-prodspec-annealing-intent-based-actuation-2021]; the
  [B-hewson-anderson-gordon-declarative-automated-configuration-2012] call exited without a
  confirmation line. These three remain UNREGISTERED (read and graded; entry JSONs in `$SC`).
  The other 12 are in `sources.json`. The note below is superseded on the other three.
  Disk was full at the time; no partial archive files or stale lock were left behind (checked).
  The three pending entries, verbatim, so they survive scratch cleanup (pipe each line to
  `register.sh $RD <slug>`):
  - B-traugott-huddleston-bootstrapping-infrastructure-1998:
    `{"url":"https://static.usenix.org/publications/library/proceedings/lisa98/full_papers/traugott/traugott.pdf","grading-certainty":"+1:SURE","grading-reasoning":"not A: an experience/practice paper (peer-reviewed LISA, first-party, read in full) with no formal model, so it is a battle-testing primary rather than a theory primary; not C: authors ran it across ~15,000 hosts on trading floors, it is the canonical source of the 'gold server' and infrastructures.org line, and it is stable on USENIX.","relevance-certainty":"-1:GUESS","relevance-description":"Identity handled by imposing it, not modelling it: one gold server, 'Never log into a machine to change anything on it', uniform filesystem namespace on every host, NIS domain = DNS domain one-to-one, role CNAMEs, symlink farms for location transparency; warns /net makes 'host names become a part of the file name', which breaks moves (a route-qualified-handle observation). Battle-tested single-author discipline as the answer to multiple authors.","graded-by":"subagent","published":"1998-12","via":"mcp__kagi-ken__kagi_search_fetch(queries: ['Traugott Huddleston \"Bootstrapping an infrastructure\" LISA 1998'])"}`
  - B-palatin-prodspec-annealing-intent-based-actuation-2021:
    `{"url":"https://www.usenix.org/sites/default/files/palatin.pdf","grading-certainty":"+1:SURE","grading-reasoning":"not A: an engineering article in ;login: online (edited, not peer-reviewed) that describes rather than specifies the model, with figures missing from the text layer; not C: first-party, written by the system's co-creator about a system Google runs at 'millions of resources' and 'hundreds of changes per second', read in full, stable on USENIX.","relevance-certainty":"-0:SUSPECT","relevance-description":"Industrial desired-state actuator with an explicit identity rule: an asset is a string asset ID plus a typed payload, and 'An asset ID must be unique within a partition' (a primary key scoped in a store); assets reference each other through explicitly marked fields forming multiple hierarchies. Admits the dependency: 'requires clean resource naming from the infrastructure provider'. Multi-author rules: 'Only one automation system should be responsible for a given part of production'; parallel rollouts on one asset 'must modify distinct aspects of the assets'. Rejects turndown-by-absence because partial config is the most common failure. Identity is punted to the provider's naming; no aliasing model.","graded-by":"subagent","published":"2021-12","via":"mcp__kagi-ken__kagi_search_fetch(queries: ['\"Prodspec and Annealing\" usenix loginonline Palatin assertions intent'])"}`
  - B-hewson-anderson-gordon-declarative-automated-configuration-2012:
    `{"url":"https://jahewson.me/docs/ConfSolve-LISA12.pdf","grading-certainty":"+1:SURE","grading-reasoning":"not A: peer-reviewed LISA 2012 with a formally specified grammar, type system and translation, read in full, but a research prototype (~1900 lines F#/OCaml) evaluated only on synthetic VM-placement and HP Cauldron benchmarks, never deployed; not C: first-party (Edinburgh/LCFG group with MSR), rigorous, and the author's own copy.","relevance-certainty":"-1:GUESS","relevance-description":"The Edinburgh line's formal endpoint: a closed-world object model where identity is allocation in the model. 'var v as c allocates a new instance of c' vs 'var v as ref c ... will resolve at solve-time to an instance of c elsewhere in the model'; a nested child's 'lifetime ... is tied to that of its parent object, and is not shared'; a reference's output value 'is the fully-qualified name of the target variable'. Deliberately referent-agnostic: 'All classes are equal: there are no built-in classes with special meanings such as Machine or File' (coincides with 311's refusal of engine-side world knowledge). No aliasing: two declarations are two objects by construction, because the model is the world.","graded-by":"subagent","published":"2012-12","via":"mcp__kagi-ken__kagi_search_fetch(queries: ['Hewson Anderson \"A declarative approach to automated configuration\" LISA 2012 pdf'])"}`
- `register.sh` takes minutes per call under cross-lane lock contention. At hand-off, 9 of 15 slugs
  are in `sources.json`; these 6 were fully read, graded, and QUEUED via `register.sh` but not yet
  confirmed (entry JSONs are in `$SC/entry-*.json`; re-run if absent):
  [B-traugott-huddleston-bootstrapping-infrastructure-1998] (entry-traugott.json),
  [B-delaet-joosen-vanbrabant-survey-system-configuration-tools-2010] (entry-delaet.json),
  [B-anderson-towards-high-level-machine-configuration-1994] (entry-anderson-1994.json),
  [B-palatin-prodspec-annealing-intent-based-actuation-2021] (entry-prodspec.json),
  [B-burgess-some-notes-about-promise-theory-2015] (entry-promise-method.json),
  [B-hewson-anderson-gordon-declarative-automated-configuration-2012] (entry-confsolve.json).
  `validate.sh` will flag these as unresolved until the queue drains; not run by this lane.
- Brief item "Couch & Sun, The maelstrom" is misattributed; it is Couch & Daniels (LISA 2001).
