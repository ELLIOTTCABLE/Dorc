> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> Anthropic Fable lane, NEUTRAL stance (24Ka): verbatim extract from commit 19df800 on branch ai/24Ka-langreview. Byte-authoritative copy = the branch commit; canonical corpus home pending cherry-pick.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

# 24Ka — PRE-REGISTERED LESSONS (the gate artifact)

Minted 2026-07-05, after Phase-1 research, BEFORE any corpus exposure beyond the dispatch
brief + system-prompt AGENTS.md (see integrity disclosure in 24Ka-...-01). These are the
review instrument: findings must cite a lesson number or declare themselves lesson-independent.
Citations refer to the source ledger in 24Ka-langreview-01-skillup-ledger.md.

Format: N. name — claim. FALSIFIER: how a real design violates it detectably. [sources]

1. substrate-envelope — Define the execution envelope as named shells+versions and validate
   every blessed spelling empirically against the strictest shell inside it; "POSIX" as a
   target is descriptive text, not a testable substrate. Non-POSIX constructs split into
   de-facto-universal (local) and parse-fatal (dotted fnames die at parse time on dash/ash):
   the dialect must know which class every one of its spellings is in.
   FALSIFIER: a blessed idiom that parse-fails or diverges on a shell inside the claimed
   envelope; or no stated envelope at all. [main-6, main-9, shell-2, shell-17]

2. erasability — Every dialect construct must have defined, correct behavior when the tool is
   ABSENT: plain execution of the artifact remains correct (possibly slower/less checked).
   Valid-under-substrate is what decides whether the substrate absorbs you (TS) or strangles
   you (CoffeeScript); it is also the abandonment story.
   FALSIFIER: an artifact that misbehaves — not merely loses value — when run by plain sh
   without the tool. [main-1, shell-15, shell-16, grad-20]

3. paste-survival — Spellings must survive transplantation: pasted into a terminal, a foreign
   repo, a Makefile recipe, a CI yaml. fish's costliest incompatibility was pasted `&&`; the
   clipboard is a compatibility surface distinct from execution.
   FALSIFIER: a blessed spelling that errors or silently changes meaning when copy-pasted into
   the common surrounding contexts. [shell-7, shell-11]

4. never-lie — A best-effort analyzer's trust contract: silence must be safe, reports must be
   true, annotations are hints the tool may verify, never blind gates. Dialyzer bought
   adoption with "never wrong for defect detection"; the tool must not demand rewrites to
   satisfy its inference.
   FALSIFIER: a surface that invites/requires overclaiming annotations with no detection
   story; or diagnostics that are wrong in the direction users cannot audit. [main-4, grad-10]

5. one-marker-one-intent — One terse marker overloaded across semantically-different roles
   conveys mechanism-instead-of-intent and becomes a paid-for regret (Scala implicit).
   Markers should be intent-named; count roles per marker.
   FALSIFIER: any single mark/sigil/word carrying 2+ unrelated semantic roles depending on
   position or context. [main-8, reg-13]

6. read-site-primacy — Evaluate every spelling at its USE-site under maintenance conditions
   (six months later, someone else's screen): declared once, read forever; brevity is a
   non-goal; names must be greppable and web-searchable (punctuation isn't).
   FALSIFIER: a spelling defended by write-side convenience (fewer chars) rather than a
   use-site reading argument; constructs unfindable by grep/search. [main-7, erg-2, erg-12]

7. false-friends — Reusing a substrate spelling while altering its semantics produces
   persistent, instruction-resistant errors (worse than novel syntax). If the semantics
   differ from plain sh, the spelling must visibly differ; if the spelling is plain sh, the
   semantics must be exactly sh's.
   FALSIFIER: a construct that reads as ordinary sh but behaves differently under the tool in
   ways not derivable from sh semantics. [erg-11, erg-7]

8. rc-context-zoo — In sh, an exit status's meaning depends on the consuming context (if-
   position, `||`/`&&` chains, `!`, pipeline position, `$?` capture, set -e immunity rules,
   subshells). Any semantic partition of exit statuses must specify behavior under EVERY
   consuming context or it re-creates the errexit zoo (convoluted, version-drifting special
   rules; no community consensus even decades later).
   FALSIFIER: a blessed rc-partition with undefined or surprising meaning in any standard
   consuming context; partition semantics that invert silently under `!` or `||`. [main-2,
   shell-9, shell-14]

9. epochs-before-users — A spelled, per-artifact version/epoch marker and machined compat
   gates (automated old-corpus regression) must exist BEFORE external authors. Promised
   compat without machinery fails (Cox); the mechanism is what lets defaults change later
   (editions, BASH_COMPAT, shopt-groups, per-module language versions). Bash's own maintainer
   regrets not adding compat levels earlier.
   FALSIFIER: artifacts carry no version/epoch marker; no automated corpus-compat gate exists;
   evolution plan is "we'll be careful". [reg-4, reg-16, reg-15, shell-8, shell-1]

10. dozen-user-freeze — The respell window closes at the first external author, not at 1.0
    (Feldman's dozen friends; Ritchie's three sites). "Strawman-tier, subject to change"
    labels are self-deception UNLESS each strawman spelling has a budgeted mechanical
    migration story (codemod/formatter/rewriter) — then and only then does the label mean
    something.
    FALSIFIER: a strawman-labeled spelling already load-bearing across many artifacts with no
    migration tooling story. [reg-9, reg-2, grad-6]

11. recognizer-is-API — For an inference-driven dialect, the recognized-idiom set IS the
    public API (Hyrum's law applies to the analyzer): every pattern rewarded today becomes
    unremovable, and verdict churn across analyzer releases is a compat surface needing an
    explicit policy (TS learned this; community had to invent "no new red squiggles").
    FALSIFIER: no enumerated, versioned ledger of recognized idioms; verdict-affecting changes
    shipped without a compat policy. [grad-17, reg-4, main-4]

12. escape-hatch-permanence — Opt-outs (unanalyzed regions, refusals, suppressions) are
    permanent and viral, never transitional. They must be visible, scoped to the smallest
    unit, carry a "why", be greppable, and be ratchet-lintable ("no new opt-outs").
    FALSIFIER: an opt-out spelling that is invisible, unscoped, reason-free, or unlintable.
    [grad-16, grad-8, grad-4, main-5]

13. inference-carries-weight — Successful gradual systems demand annotations only at
    interface boundaries and infer everything else; per-expression annotation burdens fail
    adoption; bootstrapping annotations from observed behavior (traces/probes) beats manual
    annotation. Dialyzer's "no code rewrites of any kind" is the pole to respect.
    FALSIFIER: flagship examples needing marks on a majority of lines; a "good" path that
    requires constant per-command annotation. [grad-13, grad-4, grad-14, main-4]

14. two-audiences-two-profiles — Serve the scrappy operator and the correctness engineer with
    NAMED, distinct strictness profiles in one runtime, connected by a per-artifact ratchet
    ladder (typed:false->strict; OSH->YSH), not one surface stretched over both. You must be
    able to name the dimension where the audiences pull opposite ways; if you can't, you've
    collapsed them.
    FALSIFIER: no per-artifact profile/ladder distinction; or inability to state where
    admin-optimal and engineer-optimal spellings conflict. [erg-10, erg-4, grad-8, shell-1,
    main-3]

15. metadata-rots — Annotations the tool cannot cross-check against observed behavior WILL
    drift (stub repositories, typed islands that disagree when merged). Prefer metadata that
    executes or is verified against execution; surface staleness as a first-class state.
    FALSIFIER: an annotation channel with no freshness/consistency check against the thing it
    describes. [grad-4, grad-18, grad-15]

16. emitted-sh-legibility — Machine-emitted shell inside user-facing artifacts becomes
    user-maintained the day the tool leaves. It must be idiomatic (TS goal 4), clearly
    bounded/attributable, debuggable by a shell-literate human WITHOUT the tool, and
    idempotently regenerable; otherwise it is autoconf.
    FALSIFIER: emitted artifact a shell-literate reader cannot debug unaided; regeneration
    that destroys hand edits without warning. [shell-17, main-1]

17. static-parseability-engineered — The blessed subset must be recognizable without executing
    the script; constructs whose meaning needs runtime state (alias, eval, dynamic names)
    are out or explicitly fenced. Oils proved sh CAN be statically parsed — by defining the
    dynamic parts out.
    FALSIFIER: a blessed idiom whose recognition depends on runtime information. [shell-3]

18. diagnostics-teach — The analyzer's messages are the language's primary teaching channel:
    each refusal/warning must name the blessed idiom to write instead (actionability) and use
    the same vocabulary as the docs; BUT assume no automatic outcome-improvement from nicer
    prose — the evidence is contested; test messages like product code.
    FALSIFIER: diagnostics that describe mechanism without naming the fix-idiom; error
    vocabulary diverging from doc vocabulary. [erg-5, erg-12, erg-13]

19. ecosystem-tools-tax — Every deviation from substrate grammar taxes the entire toolchain
    forever (shellcheck, shfmt, editors, highlighters); a dialect that breaks the linter
    ecosystem pays on every seat. Winners win on tooling network-effects, not theory quality.
    FALSIFIER: the blessed corpus fails shellcheck/shfmt parsing today. [grad-18, shell-10]

20. one-spelling-per-intent — Every synonymous blessed spelling multiplies the recognizer
    surface and splits the corpus into recognized/unrecognized dialects; bless ONE spelling
    per intent and let tooling normalize toward it (the gofmt effect). (Weakest-sourced
    lesson: part expert judgment; treat findings hung ONLY on this as lower-confidence.)
    FALSIFIER: an intent in the feature-ledger with 2+ blessed spellings and no deprecation/
    normalization story. [grad-18 analogy, shell-10]

21. namespace-reservation — Reserve tool-owned name-space explicitly (prefix, sigil-space, or
    verb-list) so language growth never collides with user code, and check collisions against
    the substrate's live namespace (utilities, builtins). Sigils-as-reserved-namespace is
    Wall's stated growth mechanism; PowerShell's verb-list is the other pole.
    FALSIFIER: tool-recognized names indistinguishable from arbitrary user names; a blessed
    name that shadows or collides with a real-world utility/builtin. [reg-13, reg-7, shell-5]

22. demo-defaults — Defaults chosen to make the flagship demo shortest become what 95% of the
    at-scale corpus is written in (implicit-any era; typed:false default; == coercion). Re-ask
    every default at 200-file scale; the default IS the dialect.
    FALSIFIER: a default whose recorded justification is demo brevity or one early user's
    convenience. [reg-2, grad-16, grad-8]

23. optional-vs-gradual-discipline — Types-with-zero-runtime-effect (Bracha optional) and
    runtime-enforced gradual (Siek-Taha) have categorically different safety properties;
    deployed "gradual" systems are mostly optional, and users do NOT write the boundary checks
    themselves (<3% measured). Know which one each boundary is, and never use enforcement
    language for erasure surfaces.
    FALSIFIER: safety claims in docs that presume enforcement the runtime does not perform at
    that boundary. [grad-7, grad-19, grad-15, grad-9]

24. punctuation-budget — sh is already sigil-saturated; bracket/punctuation mismatch is the
    single most common novice error class, and familiar-looking punctuation carries zero
    inherent learnability (Randomo). New marks compete for the same visual channel as live sh
    operators; prefer words; audit each mark against nearby sh meanings.
    FALSIFIER: an annotation-mark reusing a character with live sh semantics in adjacent
    positions; marks distinguished only by punctuation subtleties. [erg-6, erg-2, main-2]

## Post-exposure additions

(none yet — any lesson added after corpus exposure will be logged here, dated, and flagged
as post-exposure per the brief)
