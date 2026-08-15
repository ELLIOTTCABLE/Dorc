/-!
# Minispec.Proofs

The tactic-churn zone. A proof file imports its unit and proves that unit's `Prop`:

```
import Minispec.LeqIsReflexive
theorem LeqIsReflexive_holds : Minispec.LeqIsReflexive := by …
```

The split is the point, and it is idiomatic Lean: a proof file is structurally unable to
touch a statement. Nobody reviews proofs; everybody may review statements. A proof that stops
going through is a finding about the code or the law — never a licence to edit the law, which
is a spec change and belongs to the human (`verified-core-discipline`: never weaken the
question to pass a check).

A hole here is a loud, counted TODO — legal mid-work, never silently merged. The binder's
census is what makes "never silently" true.

This file exists so the directory is a real lake target while the corpus is empty; the first
proof replaces the need for it.
-/
