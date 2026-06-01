Dorc, my "dash orchestrator"
============================

My exploration of the feasability of replacing Ansible with, well, shell-script (because fuck YAML); while trying to best-effort retain some of the soundness and performance gains of the Big Boy orchestrators.

Rationale
---------

My core thesis:
 - Terraform, K8s, Nix, etc exist, and are good and pure and you should probably be using them ...

 - ... however, Ansible-alikes still have a (much-reduced) place. Specifically, I mean:

   1. **imperative, *not* declarative.** (the "stuff I want to fix here" is *defined* by 'what terraform or k8s suck at.' I much prefer declarative; but there are corner-cases all over the universe where it's just not feasible to perfectly align and totalistically describe every lil' bitsie of a given network of systems.)
   2. **push, *not* pull.** (Puppet et al. are great, but sometimes it's just nice to have one less thing to manage. further, one-less-thing-to-manage often translates to *direct* correcness and security-surface-area benefits.)
   3. **gradually-enriched.** (sure, I could write a 500-line YAML nightmare fully specifying the precise idempotency representation for a little toy tool I want to deploy onto a machine; but ... sometimes it's most efficient to *start* with `apt-get install the-thingie && the-thingie --start` and work your way up to all the Heavy, Correctness metadata and structuring. Again, NixOS exists; if you're gonna write the 500 lines of YAML upfront, just ... use the better tool, now?)

 - Given that the're useful in theory, though, we can probably (definitely) do better than Ansible's UX, using modern tools.

In pursuit of that, I want to follow to its logical conclusion this particular observation: **When Ansible gets annoying, one tends to fall back to "just write a goddamn shell-script and ship it to the host."**

Thus, Dorc: I'm going to attempt to build an Ansible-alike (by the above constraints; explicitly *not* trying to supplant/replace the Things That Are Already Good), but one whose *UX revolves around shell-scripts*, staying tightly-bound to "what you would do if this tool didn't exist", and simply enriching/supercharging that precise approach.

(Further elaborated in [./DESIGN.md](DESIGN.md).)
