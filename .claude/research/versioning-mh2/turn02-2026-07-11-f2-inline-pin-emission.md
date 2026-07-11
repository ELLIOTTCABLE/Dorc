# turn02/f2 — inline pin-emission (gatherer: Opus subagent, 2026-07-11)

Front: machine-maintained version/hash pins living INLINE inside human-authored, human-managed
artifacts (Homebrew-style), vs sidecar lockfiles. Focus: authoring UX, drift/update workflows,
merge-conflict behavior, human-reads-vs-machine-owns boundary, failure modes — NOT hash mechanics.
Central filter per source: does the prior art assume the versioner AUTHORED the thing versioned
(`undershoots: yes`) or is a third-party describer (`undershoots: no`, the Dorc-oracle position)?

## Findings

- f2-fd1 +SURE: The dominant packaging-ecosystem pattern IS Dorc's proposed shape — inline pins
  in a human-authored *build descriptor spelled in a real language* (PKGBUILD=bash, Homebrew
  formula=Ruby, nixpkgs=Nix, Yocto recipe=bitbake), maintained by dedicated *in-place-rewrite*
  tooling (`updpkgsums`, `brew bump`/autobump, `nix-update`/r-ryantm). Every one of these PASSES
  the undershoots filter: the packager is a third party describing upstream software they did not
  author and which rots independently — structurally identical to a Dorc oracle-author. Their UX
  lessons transfer. [A-updpkgsums-manpage-2025][A-homebrew-formula-cookbook-2025][A-nixpkgs-fetchers-chapter-2025]
- f2-fd2 +SURE: The machine-owns / human-owns line these ecosystems draw is narrow: tooling owns
  ONLY the hash *value* (recomputes it in place on demand); the human owns the url, the
  version-tracking intent, and the opt-out. Homebrew makes the opt-out itself *in-band and spelled
  as code* — `no_autobump!`, or a `livecheck do … skip` block, or `deprecate!` — not a sidecar
  config file and not a magic comment. This directly models Dorc's "metadata is spelled in sh /
  narrowing-not-annotation" stance. [A-homebrew-autobump-2025]
- f2-fd3 ~SUSPECT: "Pure inline" is a partial myth — inline-pin ecosystems accrete two leaks. (a)
  A DERIVED SIDECAR CACHE for machine indexing: Arch's `.SRCINFO` is regenerated from the human
  PKGBUILD by `makepkg --printsrcinfo` and must be committed alongside it. (b) A COMMENT
  annotation so the bump-bot knows what a bare pin tracks: Renovate needs
  `pkgver=0.15.31 # renovate: datasource=github-tags depName=Azure/bicep` because `pkgver=` alone
  carries no provenance/update-intent. Caution for Dorc: a bare inline hash cannot say *what it
  pins* or *where to look for updates* — something (ideally sh-idiomatic, per house rule) must.
  [B-renovate-aur-jamie-magee-2023]
- f2-fd4 +SURE: The canonical inline-hash *authoring* idiom is fake-hash-then-copy-from-error
  (nixpkgs `lib.fakeHash`/`""`; Yocto's fetch error prints the exact `SRC_URI[sha256sum] = "…"`
  line to paste). Ergonomically loved, but nixpkgs documents its real failure mode: change a
  fetcher param (e.g. the url) WITHOUT changing the hash and the build silently reuses the OLD
  cached output — no error, pin goes stale-but-green. Plus fake-hash fetches disable TLS checks
  (an MITM window) unless one of the *exact* sanctioned fake constants is used. Dorc emission
  tooling must force-invalidate on change, never trust hash-silence. [A-nixpkgs-fetchers-chapter-2025][C-ianthehenry-nix-give-back-2021]
- f2-fd5 +SURE: Cleanest inline-vs-sidecar A/B is two near-identical source distros: Arch = INLINE
  `sha256sums=(…)` in PKGBUILD; Gentoo = SIDECAR `Manifest` file beside the ebuild. Gentoo's
  sidecar exists because verification scope EXCEEDS a single descriptor — it covers the whole tree
  (every distfile + all repo files) under one OpenPGP-signed "thick" Manifest (full-tree
  verification). Signal for Dorc: inline wins when the pinned set is per-descriptor and
  human-legible; sidecar wins when the set spans many artifacts or needs one signature over the
  whole tree. [A-gentoo-manifest-devmanual-2024]
- f2-fd6 +SURE: Drift failure mode, from the sidecar camp but applies equally to inline: when
  upstream changes a tarball IN PLACE (same filename, new bytes), the tool CANNOT auto-accept the
  new hash. Gentoo *requires the human* to diff old-vs-new distfile and record the finding in the
  commit message, because the mismatch "could be an innocent respin … or it could indicate either
  the previous or the new tarball is malicious." The hash-mismatch is a human-adjudication trigger,
  not an auto-bump. This is exactly the innocent-respin-vs-attack cell and maps onto turn01's
  hostile-host / TOFU rider. [A-gentoo-manifest-devmanual-2024]
- f2-fd7 +SURE: Merge-conflict behavior is THE decisive axis for lockfile design (both the
  practitioner and the peer-reviewed source converge): flat, line-based, alphabetically-sorted
  formats (go.sum, Cargo.lock) merge cleanly; nested formats (package-lock.json, Gemfile.lock)
  conflict even when both branches resolved the *same* version. Consequence for inline pins: a pin
  inherits its *descriptor's* merge behavior — one pin co-located with its url means two unrelated
  pin-bumps rarely collide, BUT a machine pin-bump and a human edit to the SAME descriptor DO
  collide (whereas a pure sidecar the human never touches cannot conflict with human edits). Inline
  deliberately couples machine-churn to the human-edit surface. [B-nesbitt-lockfile-tradeoffs-2026][A-lockfile-design-space-arxiv-2025]
- f2-fd8 +SURE: WHY lockfiles went sidecar (the contrast class, sharpened): the sidecar carries
  multi-consumer transitive *resolution* + integrity of a resolved graph — data that is (a) not
  human-authored, (b) regenerated wholesale, (c) by definition "not meant to be edited by
  developers." NONE of that applies to a Dorc oracle pin (a per-descriptor, human-owned,
  third-party-described fact with no transitive-resolution graph). The forces that justify sidecars
  are largely ABSENT for Dorc, which strengthens the inline position (corroborates turn01
  vers1-fd-inline-pins-thread-koob). The one lockfile virtue Dorc SHOULD copy: "generate by
  default" is the single biggest predictor of adoption — emission must be automatic, not opt-in.
  [A-filippo-gosum-not-a-lockfile-2026][A-lockfile-design-space-arxiv-2025][B-nesbitt-lockfile-tradeoffs-2026]
- f2-fd9 +SURE: "Bot rewrites a human-authored file at scale" has rich, healthy, settled prior art
  (Renovate/Dependabot rewriting workflow YAML; r-ryantm rewriting `.nix`; pre-commit autoupdate
  rewriting `rev:`; BrewTestBot autobump every 3h). The consistent settled UX: the bot proposes a
  PR (never silent in-place mutation of the human's mainline), keeps a human-legible version string
  ALONGSIDE the opaque pin (`…@<sha> # v5.0.0`), and the human reviews/merges. Emission-as-PR with
  a legible companion string is the norm Dorc should match. [B-emmer-pin-github-actions-2025]
- f2-fd10 ~SUSPECT: Counter-evidence to a smooth per-feature pin gradient (turn01
  vers1-position-silence-gate): pip's hash-checking mode is deliberately ALL-OR-NOTHING — one
  `--hash` on any line flips global hash-mode, and then EVERY requirement incl. transitive deps
  must be hashed or the install errors, "because a partially-hashed requirements file is of little
  use and thus likely an error: a malicious actor could slip bad code in via one of the unhashed
  requirements." pip judged partial pinning *actively dangerous* and forbade it. Worth stress-testing
  the "gradually pin some features / silence licenses correctness on the un-pinned" strawman against
  this. [A-pip-secure-installs-hash-checking-2024]
- f2-fd11 -GUESS: The undershoot gradient across the corpus has three bands, not two. PASS
  (third-party publishes a reusable *description* of upstream — Dorc's exact case): PKGBUILD,
  Homebrew, nixpkgs, Gentoo, Yocto. MID (a consumer pins someone-else's artifact into *their own*
  project, no reusable description published): GitHub-Actions SHA-pinning, pip `--hash`, go.sum,
  Cargo.lock. UNDERSHOOTS-YES (author == versioner): a project's own release CI emitting its own
  formula/tag. The packaging band is why this front matters — its authoring-UX is the most
  transferable, but note its pins are single-artifact, so it is silent on Dorc's harder
  in-body-chronology need (that's front f3, not here). [A-homebrew-formula-cookbook-2025][B-emmer-pin-github-actions-2025]

## Citations

> [A-filippo-gosum-not-a-lockfile-2026]:§"Manifests and lockfiles" (relevance: +SURE — canonical primary author, the sharpest statement of the inline/combined-vs-sidecar tradeoff)
> The lockfile (e.g. `uv.lock`, `package-lock.json`, `Cargo.lock`) is a relatively recent
> innovation in some ecosystems, and it lists the actual versions used in the most recent build.
> It is not really human-readable, and is ignored by dependents, allowing the rapid spread of
> supply-chain attacks. … In Go, `go.mod` serves as both manifest and lockfile … All that with a
> single, human-readable file: `go.mod`.

> [A-lockfile-design-space-arxiv-2025]:§2.1 Definition 3–4 (relevance: +SURE — peer-reviewed; the crispest human-owns-vs-machine-owns boundary in the corpus)
> A lockfile is a file that is automatically generated by the package manager and that is not
> meant to be edited by developers. … The dependency specification file is primarily maintained by
> developers, and also by automated dependency bots such as Dependabot.
>
> (§3.3.1) Although recording as much information as possible for each dependency may seem a good
> choice, it also makes the lockfiles lengthier and harder to be reviewed by human developers.
>
> (§1 / recommendations) some approaches, such as Go's strategy to always generate a lockfile by
> default, facilitates their adoption by developers, others, such as Gradle that requires
> specializing the build … fails to make the lockfile usable.

> [B-nesbitt-lockfile-tradeoffs-2026]:§"Flat vs nested" + §"What works" (relevance: +SURE — practitioner synthesis of the merge-conflict axis and the generate-by-default adoption law)
> Flat structures merge better. When each package is an independent entry, two developers adding
> different dependencies don't touch the same lines. … Nested structures mirror dependency trees
> but cascade changes … causing a conflict even when both branches resolved to the same version.
>
> Generate by default. Go's lockfile gets committed in nearly every project because `go mod tidy`
> creates it automatically. … The single biggest predictor of lockfile adoption is whether the
> tool creates one without being asked.

> [A-gentoo-manifest-devmanual-2024]:§"Updating Manifest files" + §"Thin and thick Manifests" (relevance: +SURE — the sidecar half of the A/B; the human-adjudicates-drift failure mode; undershoots:no)
> Updating existing entries within a manifest must be done with care. Upstream changing the tarball
> in-place without a new filename could be an innocent respin of the tarball, or it could indicate
> either the previous or the new tarball is malicious. Developers should diff the old and new
> versions of the distfile, comparing the two, and note the differences in the commit message …
>
> Thick Manifests add checksums for all files in the repository, and an OpenPGP signature. This
> provides both for integrity and authenticity checking when the repository is transmitted over
> insecure channels.

> [A-nixpkgs-fetchers-chapter-2025]:§Caveats + §"the fake hash method" (relevance: +SURE — canonical; the silent-stale-pin and MITM failure modes of inline-hash authoring; undershoots:no)
> When changing any fetcher parameters, always update the output hash. … Otherwise, existing store
> objects that match the output hash will be re-used rather than fetching new content.
>
> A common mistake is to update a fetcher parameter, such as `url`, without updating the hash …
> **This will produce the same output as before!**
>
> `https://` URLs are secure when using the fake hash method *only if* you use one of the listed
> fake hashes. If you use any other hash, the download will be exposed to MITM attacks even if you
> use HTTPS URLs. … the `--insecure` flag will be passed to the underlying call to `curl`.

> [A-homebrew-autobump-2025]:whole page (relevance: +SURE — machine-owns-the-bump, opt-out spelled in-band as code; undershoots:no)
> Every 3 hours, a GitHub Action opens a new pull request to upgrade them to the latest version, if
> needed. … To exclude a package from the autobump list, it must have one of the following: an
> active `deprecate!` or `disable!` call; a `livecheck do` block containing a `skip` call; a
> `no_autobump!` call. … When using `no_autobump!`, a reason for exclusion must be provided.

> [B-renovate-aur-jamie-magee-2023]:§"Updating checksums and .SRCINFO" (relevance: +SURE — inline-pin maintenance is multi-tool; the comment-annotation + derived-sidecar leaks; undershoots:no)
> pkgver=0.15.31 # renovate: datasource=github-tags depName=Azure/bicep
>
> So I've automated the `PKGBUILD` update, but that's only half of the work. The checksums and
> `.SRCINFO` must be updated before pushing to the AUR. Unfortunately, Renovate can't do that
> (yet …), but GitHub Actions can! … it runs `updpkgsums` and `makepkg --printsrcinfo > .SRCINFO`.

> [A-updpkgsums-manpage-2025]:§DESCRIPTION (relevance: +SURE — canonical; the named exemplar in-place pin-rewriter; undershoots:no)
> updpkgsums will perform an in place update of the checksums in the path specified by [build file],
> defaulting to PKGBUILD in the current working directory.

> [A-homebrew-formula-cookbook-2025]:§"An introduction" + ToC (relevance: +SURE — inline sha256 in a human Ruby descriptor authored via brew create/audit; undershoots:no)
> A formula is a package definition written in Ruby. It can be created with `brew create <URL>` …
> Read over a simple one, e.g. `brew edit etl` … [sections: `livecheck` blocks · Excluding formula
> from autobumping · Updating formulae]

> [C-ianthehenry-nix-give-back-2021]:§18.6 (fake-hash) + §18.4 rant (relevance: ~SUSPECT — opinionated first-hand, but the "lift human-instructions into computer-instructions" framing mirrors Dorc's thesis and the fake-hash critique is vivid; undershoots:no)
> The ability to lift "instructions for humans" into "instructions for computers" is one of the
> great superpowers that Nix gives you.
>
> It then says "if all else fails, put a fake hash and run some command and copy the error" …
> Actually really it says *don't* do this, because downloading a package is not a very good way to
> find out what the hash *should* be, because man in the middle attacks. … When obtaining hashes
> with fake hash method, TLS checks are disabled. … I am upset and afraid.

> [B-emmer-pin-github-actions-2025]:§Solution + §Automating + §Drawbacks (relevance: +SURE — bot rewrites human YAML in place, opaque pin + legible companion comment, automerge risk; undershoots:mid)
> - uses: actions/checkout@08c6903cd8c0fde910a37f88322edcfb5dd907a8 # v5.0.0
>
> Renovate has a "helper" preset named `helpers:pinGitHubActionDigestsToSemver` that will pin every
> GitHub Action you have to a commit SHA, and it will keep a human-readable version string up to
> date in a trailing comment. … `"automerge": true` will still leave you vulnerable to an attacker
> pushing new Git version tags.

> [A-pip-secure-installs-hash-checking-2024]:§"Hash-checking Mode" + §"Additional restrictions" (relevance: +SURE — counter-evidence to partial/gradual pinning; undershoots:mid)
> Note that hash-checking is an all-or-nothing proposition. Specifying `--hash` against *any*
> requirement will activate this mode globally. … Hashes are required for *all* requirements. This
> is because a partially-hashed requirements file is of little use and thus likely an error: a
> malicious actor could slip bad code into the installation via one of the unhashed requirements.

## Source registrations
(Registered into this directory's script-owned `sources.json` on 2026-07-11; the pending-JSON blocks formerly here were removed after registration — the manifest is the canonical record.)

## Residue

- res-1: Homebrew `brew bump-formula-pr` / `livecheck` deep mechanics not fully read — I captured
  the Cookbook ToC + the Autobump doc in full, which cover the human-owns-vs-machine-owns question;
  the per-command flag detail was out of scope for f2 (that's hash/version mechanics). Issue #10620
  (`bump-formula-pr` won't update `resource` block sha256 except for PyPI resources) is a real
  partial-coverage failure mode I saw only as a search snippet — noted, not graded.
- res-2: pre-commit `autoupdate` (bumps `rev:` in the human `.pre-commit-config.yaml`; `--freeze`
  swaps tag→commit-hash pin) is a clean fourth instance of the bot-edits-human-file pattern, and
  issue #1957 (autoupdate proposing a DOWNGRADE because the "latest tag" predates a newer untagged
  rev) is a nice ordering failure mode. Seen only via search snippets — NOT read in full, so NOT
  graded/kept. If f2 wants a fifth bot-edits-human-file source, this is the cheapest next fetch.
- res-3: The Harvard LIL post frames `pip-compile --generate-hashes` explicitly as TOFU
  ("Trust On First Use") — a direct bridge to turn01's hostile-host/TOFU-first-contact rider
  (vers1-fd-fences). Seen only as a snippet; the canonical pip doc I kept implies TOFU but does not
  use the term. Flagged for the conductor as a ~SUSPECT connection worth one confirming fetch if the
  TOFU-fence interaction becomes load-bearing.
- res-4: GLEP 74 (Gentoo full-tree Manifest spec) — I fetched a middle section that turned out to be
  file-verification mechanics (adjacent to the transport-verification EXCLUSION), so I did not mine
  its "Rationale / Stand-alone format" section. The devmanual source already supplies the
  sidecar-choice evidence I needed; GLEP 74's explicit standalone-format rationale remains an
  unmined angle if a deeper inline-vs-sidecar justification is wanted (mind the transport fence).
- res-5: Yocto/Buildroot inline `SRC_URI[sha256sum]` corroborates the packaging-band inline pattern
  (error message prints the exact line to paste — same fake-hash-then-copy idiom as nixpkgs) but I
  did not keep a dedicated Yocto source; it would be redundant with the nixpkgs + PKGBUILD evidence.
  Deliberately not taken.
- res-6: Debian `watch`/`uscan` is an update-DETECTION mechanism (scans upstream for new versions)
  rather than an inline-pin-emission mechanism — the pins it feeds live in `debian/changelog` +
  the tarball, not inline-rewritten into a human descriptor. Off-center for f2; deliberately not
  kept beyond confirming it's the detection half, not the emission half.
- res-7: No show-stoppers — every kept source was fully reachable and read. No paywalls hit.
