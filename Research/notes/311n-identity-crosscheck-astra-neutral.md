Codex GPT-6-Astra (OpenAI lineage), neutral pass, 2026-09-16, exit status 0

I did not establish an ordinary ops situation that is wholly unrepresentable. Five concrete inconsistencies or losses survived the comparison with 311; an authorship problem follows them.

1. `different-schemes-falsely-conflict` — +SURE: §3.5 diagnoses legitimate differences as contradictory.

   Assume the account named `1000` has UID 2000, while UID 1000 belongs to Alice:

   ```sh
   sudo -u 1000 touch /srv/name-owner/ready
   sudo -u '#1000' touch /srv/id-owner/ready
   ```

   The two argument forms select different mSchemes: login-name lookup and numeric-UID lookup. Both can look up the value `1000` in the same account catalog, from the same mVantage, and correctly yield different mKey-Primaries.

   [311j §3.5](/C:/Users/ec/Sync/Code/Dorc/Research/notes/311j-identity-and-relation-model-two-species.md:498) nevertheless says that two mSchemes of one mSort resolving one string in one mParent to different mKey-Primaries cannot both be right. Separating mSchemes was precisely what made these two interpretations legitimate. Equal bytes across mSchemes do not establish equal input referents.

   There is a second, independent mistake: that paragraph invokes `:guarantees-unique-referent`. This warrants SAME from equal outputs; it does not warrant difference from unequal outputs. §2.2 expressly prohibits contradiction by unequal mTokens without `:guarantees-unique-name`.

   The immediate consequence is false withholding and false attribution, rather than under-execution. Existing machinery suffices: a disagreement requires an independently established SAME obligation between the inputs, followed by a warranted DISJOINT between their results. Neither shared spelling nor functionality supplies those premises. This faulty canary is new relative to 311 §3.5.

2. `cell-lifetimes-share-ancestry` — +SURE: §1.9’s promised reboot distinction does not follow from its new mCell definition.

   ```sh
   before=$(ssh web cat /proc/sys/kernel/random/boot_id)
   ssh web systemctl reboot
   until after=$(ssh web cat /proc/sys/kernel/random/boot_id) &&
         [ "$after" != "$before" ]; do
      sleep 1
   done
   ssh web systemctl enable nginx
   ssh web systemctl start nginx
   ```

   The relevant distinction is ordinary: durable enablement can survive the reboot; the running-service observation must answer for the new boot.

   In [311 §1.8 and §2.7](/C:/Users/ec/Sync/Code/Dorc/Research/notes/311-identity-and-relation-preparatory-model.md:167), the two mAspectSorts could borrow the service naming arrangement while choosing their own identifying stores. One could identify through persistent unit state, the other through the running manager.

   In [311j §1.9](/C:/Users/ec/Sync/Code/Dorc/Research/notes/311j-identity-and-relation-model-two-species.md:188), each mCell instead has a singleton mKey immediately under its mSort-Bearer. If both properties have the *same mBearer instance*, their ancestry above that instance is identical. Either both chains contain the boot or neither does. Different mPlacements do not change that: §2.4 expressly separates placement from identity.

   Therefore the sentence claiming that their mFullyQualifiedKeys differ exactly where one survives a reboot is unsupported.

   This is recoverable without new machinery. Give enablement and activity different mBearers—persistent unit state and a running-manager record—and connect a logical service to both through §2.5’s `:reaches`. Alternatively, retain persistent logical property identities and invalidate activity through its backing and effects. What was lost is the predecessor’s direct representation, not the ability to describe service management.

3. `separation-demands-excess-warrants` — +SURE: the revised DISJOINT rule demands more than its own lookup warrants require, and does not reproduce the motivating exercise.

   First, consider two distinct live processes in one PID namespace:

   ```sh
   kill -STOP 41
   kill -CONT 42
   ```

   Assume both processes remain alive throughout the relevant span; recycled identifiers are not involved. The second process is already running when probed.

   A PID mScheme can truthfully declare `:guarantees-unique-name` within this namespace: one process has one PID there. It cannot generally declare `:sole-route`, because that process can also have a PID in another namespace—the model’s own reason for retaining that warrant separately.

   At the shared namespace, different PIDs already prove different processes under §1.5. Any subsequent descent to their state mCells needs the appropriate containment warrants. But [311j §3.2](/C:/Users/ec/Sync/Code/Dorc/Research/notes/311j-identity-and-relation-model-two-species.md:420) requires `:sole-route` on **every** mKey of both legs, including the two differing PID mKeys themselves. It consequently returns UNKNOWN despite having the local inequality proof.

   This is a loss of expressible separation, not an unsafe DISJOINT. Requiring a canonical host-wide process identity would make the author discover information unnecessary to establish the local truth.

   The motivating unequal-depth case also fails the written rule:

   ```sh
   ip netns exec blue sysctl -w net.ipv4.ip_forward=1
   sysctl -w kernel.pid_max=4194304
   ```

   Using the exercise’s declarations, the deepest shared ancestor is the boot. The two nonempty legs begin with `sm.NetnsInode` and `sm.ProcSysPath`. They are different mSchemes, so §3.2 says UNKNOWN. The “one leg is empty” alternative does not apply; indeed, the preceding ancestor-self clause has already rejected that case.

   The [exercise’s claimed survival](/C:/Users/ec/Sync/Code/Dorc/Research/notes/312b-exercises/net-sysctls-are-per-namespace.md:407) and [312c §4’s claimed recovery](/C:/Users/ec/Sync/Code/Dorc/Research/notes/312c-attack-and-firming-ledger.md) therefore do not follow from the edited rule. This finding concerns the graph and warrants, not the strawman spellings. Broadly treating different mSchemes as disjoint would not repair it: that would break the strangers case.

4. `identifying-placements-collapse-properties` — +SURE on the literal rule: making the identifying parent a mandatory mPlacement prevents ordinary property-level sparing.

   Suppose the destination already has the desired contents, but its mode needs changing:

   ```sh
   chmod 0600 /srv/app.conf
   cp ./desired.conf /srv/app.conf
   ```

   Grant precise oracles: the first command’s relevant footprint is the mode mCell; the second command’s convergence fact reads the contents mCell. Their other dependencies are accounted for and unaffected.

   Under §1.9, these are different mCell mSorts under one file mBearer. [311j §2.4](/C:/Users/ec/Sync/Code/Dorc/Research/notes/311j-identity-and-relation-model-two-species.md:297) makes their mParent-Store an mPlacement of each. Both therefore have the same file among their mandatory placements.

   At the leaves, §3.2 makes the cross-mSort pair UNSPOKEN. A finished definition cannot rescue the separation: §2.5 requires a collision when the declared mPlacements overlap. Adding narrower placements does not remove the mandatory shared file.

   The predecessor did not require that identifying parent to be a placement; its §2.3 also explicitly allowed the measured backing to refine the declared read footprint. The new restriction defeats the stated distinction between identifying something and determining which writes affect it.

   ~SUSPECT: the intended rule may concern a write *to the whole parent*, rather than interpreting every pair of properties as overlapping because they share it. That interpretation could preserve the example, but it is not what the current placement comparison says. Merely supplying more complete oracles does not overcome the written restriction.

5. `shared-placeholders-bypass-warrants` — ~SUSPECT: the new instance-equality exception is broader than the evidence that can justify it.

   Consider a local client talking to a pool whose requests can land on different members:

   ```sh
   curl -fsS -X PUT https://pool.example/admin/flag -d enabled=1
   curl -fsS -X PUT https://pool.example/admin/flag -d enabled=1
   ```

   The two requests can address different pieces of member-local state without any book-visible routing write. This is set-valued lookup, not either excluded recycling horizon.

   At the naming floor, an author can bind that endpoint under an unwarranted mScheme. [311j §1.10](/C:/Users/ec/Sync/Code/Dorc/Research/notes/311j-identity-and-relation-model-two-species.md:208) says same-spelled local mKeys in one unwalled span share an mPlaceholder. The new §3.2 exception then treats one mPlaceholder as SAME without a lookup warrant. Yet one local shell, cwd and mount table say nothing about this binary’s endpoint selection.

   Sharing an already established namespace instance through a wrapper’s truthful sentinel is a different proof from sharing two occurrences of an unresolved name. The former justifies reflexivity; the latter can assume the very functionality §1.5 deliberately makes optional.

   This concern does **not** establish an end-to-end bad elision with an otherwise correct HTTP oracle: an oracle unable to bind its measurement to the later request must already decline. Nor does it apply when the landing is explicitly represented as a transit or as separate measured instances. The narrower finding is that the unrestricted placeholder rule itself appears to answer SAME where the lookup contract requires UNKNOWN. Retaining the existing warrant requirement for unresolved occurrences would address that without another identity mechanism.

6. `wrapper-closure-exceeds-knowledge` — ~SUSPECT: the sentinel asks a wrapper author to recognize independently invented vocabularies for the contexts it changes.

   ```sh
   sysctl -w net.ipv4.ip_forward=1
   ip netns exec blue sysctl -w net.ipv4.ip_forward=1
   ```

   Now vary the authors, not the book. The wrapper oracle uses `sm.NetNamespace`. An independently authored sysctl oracle uses `vendor.NetNamespace` as its ambient catalog. Both mSorts describe actual network namespaces, but neither owner has declared their relationship. This is expressly allowed by §1.2’s strangers case.

   The wrapper truthfully knows which namespace `ip netns exec` selects and lends an instance under `sm.NetNamespace`. But [311j §3.4](/C:/Users/ec/Sync/Code/Dorc/Research/notes/311j-identity-and-relation-model-two-species.md:478) says unmentioned catalog mSorts inherit after the completion sentinel. That includes `vendor.NetNamespace`, even though its ambient instance also changed.

   The wrapper author must therefore either recognize the stranger’s vocabulary, withhold the closure altogether, or make a false inheritance claim. The underlying namespace truth is knowable; the missing knowledge is that someone else chose another mSort to describe it.

   Existing machinery carries the cooperative version: the sysctl author can use the shared namespace mSort, or expose a dependency on it that composition can follow. But same-sort `:yields` cannot connect two already independent mSorts, and shared placements do not establish ambient-instance identity. The current text does not explain how the independent version remains conservatively unknown after the sentinel.

   There is an important limit to the failure: §2.7’s default observer-dependence can independently prevent transporting the host’s fact into blue. Thus false namespace inheritance does not automatically demonstrate a bad removal in this book. It still puts an unbounded vocabulary-recognition obligation on the wrapper’s “nothing else” claim—the wrong seat for that knowledge.

Two suspected representation failures did **not** survive:

- `descriptors-outlive-path-replacement`: in `exec 3>>p; mv replacement p; printf x >&3`, a descriptor mScheme can resolve through the descriptor table to the original file identity. The path mResolution can perish independently. §§1.6–1.7 and §3.3 already separate these cases.
- `multiple-inputs-need-parents`: base/overlay configuration does not require plural mParents. §2.9’s mCompositeSort preserves the input roles, and its placements carry interference. Reversing the roles changes the composite even when the inputs are the same.

No files were changed; no subagents or web research were used.