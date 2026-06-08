# 17N weak-points — quarantined (extracted to avoid poisoning the adversarial rounds)

> Moved out of `plans/17N` per the human (2026-06-07). Rationale: an adversarially-framed
> "here are my weakest points" list, sitting in the live synthesis, hands a clean-context
> adversarial crosscheck its checklist — it should rediscover these (or not) independently.
> The *neutral* open questions stay in 17N as plain `oq-*` / "unsettled". This file is a
> record so the thinking isn't lost; it is **not** a live doc and should not be cited forward.

## The three weakest points (as originally written in 17N's lean)

- **wp-1 — the lean assumes the *engineer*, not the lazy *admin*, carries kind-grounding.**
  Re-test under "the other user." **STATUS: superseded/RESOLVED, moved back into the 17N body.**
  The `151` X4 input-quality finding (cheap inference fires on scruffy admin books; careful
  engineer-grade oracles drive the analyzer to ⊤) is no longer an open doubt — the human welded
  full complex-script handling (constant-prop / interproc / variables / heredocs;
  `eval` the sole punt). So the cheap spine being insufficient for the high-value audience is a
  *settled constraint*, not a weakness: the expensive flow-analysis path is required, not optional.

- **wp-2 — `owl:sameAs` + CISA's "grouping is unsolved" bite harder if a future feature needs
  cross-provider *sameness*** (apt's nginx ≡ brew's nginx), not just per-provider naming. Genuinely
  open; left in 17N only as a neutral note under the cross-authority regime.

- **wp-3 — MUST-grade-to-correlate is *asserted*, not proven; selector-*meaning* pinning (opt-2's
  risk, `oq-3`) is where the cooperation contract re-enters the grounding problem and may not be
  cleanly firewall-able.** Genuinely open; left in 17N as `oq-1`/`oq-3`, neutrally.
