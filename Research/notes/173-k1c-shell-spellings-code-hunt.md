# 173 — K1c: shell-spellings only — code-pattern hunt for non-file/non-package coordination (round 17, 2026-06-07)

> **Final, last-ditch K1 round** (human steer, 2026-06-07): drop adjacent-fields/abstraction; go CODE-FIRST
> and EXCLUSIVELY at shell-spellings. Read the installed shell Claude SKILLs + a high-level ops-best-
> practices page; slurp real scripts (GitHub, `Vendor/`); look **through code** for patterns. **GOAL:** find
> a pattern that **isn't filenames and isn't packages**, exposing some statically-**analyzable coordination**
> between commands. Feeds `plans/177` q1. Sources → `../sources.json`. AI-generated; confidence-marked.
>
> *Read for context (not graded — local, no URL):* the `shell-scripting-and-automation` + `ops-and-
> infrastructure` Claude SKILLs. The ops skill confirms the user's real coordination referents are
> non-file/non-package: 1Password connection-strings, DNS hosts, machine-names, providers (Neon/Vultr/AWS),
> injected via `op run` into the environment.

## Findings (lifted, most-load-bearing first)

- **k1c-answer (+SURE) — YES, there is one analyzable non-file/non-package coordination channel: the
  ENVIRONMENT.** Empirically pervasive: the canonical postgres entrypoint's `docker_setup_env()` is an
  env-var **coordination hub** — `POSTGRES_USER/DB/PASSWORD/INITDB_ARGS` set once, *"used elsewhere in the
  script"* by independent functions (initdb, psql, createdb) [B-postgres-docker-entrypoint-2024]; even the
  `Vendor/` build scripts coordinate over exported `LLVM_DIR`/`Z3_DIR`/`PATH`/`LD_LIBRARY_PATH`; 12-factor
  canonizes it (*"resource handles to the database and backing services"* live in env) [B-twelve-factor-config-2017].
  It is **statically analyzable** as `export`/assignment + named-use dataflow — and `export` *explicitly
  marks* the cross-command coordination scope.

- **k1c-handle-is-the-name (+SURE) — the coordination handle is the env-var NAME, convention-agreed across
  *independent* authors (the cross-oracle case, in the wild).** `KUBECONFIG` is read by `kubectl`, `helm`,
  `velero` — tools authored by *different* people — with **no central coordination**; scripts even declare a
  **precondition** on the shared handle: `[[ -z ${KUBECONFIG} ]] && die "KUBECONFIG needs to be set"` before
  using either tool [C-petergardfjall-helm-init-2024] (octocode aggregate: ~29 files show the
  `export KUBECONFIG=… ; kubectl … ; helm …` shape). **This is the env-var realization of the reverse-DNS
  handle (`plans/177` C2) — flat/conventional, not reverse-DNS, but the same idea: decentralized agreement
  on a shared name.** It is the strongest sh-native handle found on *existing-independent-value* +
  *analyzability*: real tools already read it, for their own reasons.

- **k1c-coreference-not-kind (+SURE) — the THIRD converging failure (the human's "two failures is a
  signal", now three).** Env-var coordination yields analyzable **co-reference** (which commands share the
  handle) **for free** — but **not the KIND.** The var name's *meaning* (`KUBECONFIG`→kube-context;
  `POSTGRES_DB`→a database) is **convention**, ungrounded (`095 f27`): Dorc cannot soundly know what
  `KUBECONFIG` *means* without a declaration/oracle. Same wall as `notes/171` (packaging) and `notes/172`
  (adjacent fields). **Three rounds, three confirmations: the cross-oracle KIND-link is irreducibly
  author-declared; co-reference — now including named env-handles — is the only *free* part** (`094 g1/g4`).

- **k1c-service-name-same-provider (+SURE) — the other pervasive pattern is within-provider, not the
  channel.** The most common non-file/non-package idiom by volume is `systemctl is-active "$svc"` ↔
  `systemctl restart "$svc"` over a shared service-name variable (dozens of repos). But it is the `094 g1`
  **probe/establish pair within ONE provider** (systemctl) — exactly the round-15 `systemctl.straw.sh`
  shape — *not* a cross-oracle channel. A shared service *literal* across **different** providers
  (`systemctl … nginx` vs the `nginx` binary) is the `094 g4` **token-collision hazard**, not a clean link.

- **k1c-env-already-core (+SURE) — the genuinely useful bit.** Env-state is **already a bake-into-core
  category** the analyzer must model for correctness (`09A §3a` shell-execution-environment state;
  `099 W5` ambient). So named-env-var co-reference **rides machinery Dorc already needs** — and it is a
  *cleaner* co-reference channel than positional-arg tracking (the handle is an explicit, stable NAME).
  Split: `PATH`/`LD_LIBRARY_PATH`/`HOME`/`TMPDIR` are **language/OS-level** (Dorc may know them like it
  knows `[ -f ]`); `DATABASE_URL`/`KUBECONFIG`/`AWS_PROFILE` are **convention-level** (the name needs a
  declaration to *mean* anything).

- **k1c-env-is-a-hazard (~SUSPECT).** The same ambient power makes env a **soundness hazard**: the postgres
  entrypoint defensively unsets `PGHOST= PGHOSTADDR=` to stop env-coordination *leakage* into a subcommand
  [B-postgres-docker-entrypoint-2024]. Implicit, mutable, ambient env coordination is precisely the
  non-local state that is hard to bound (`099 W5`); the channel is double-edged.

- **k1c-ci-gha (+SURE) — GitHub Actions is the strongest real-world MODEL (human lead), and it confirms the
  negative a 4th time.** Independently-authored steps/actions coordinate over **named handles, sh-spelled
  and statically analyzable**: `echo "name=value" >> "$GITHUB_OUTPUT"` (write) ↔ `${{ steps.x.outputs.name }}`
  (read) [B-github-actions-workflow-commands-2024]; inputs/state arrive as **prefixed env vars**
  (`INPUT_<NAME>`, `STATE_<NAME>`) — the k1c env-handle with a flat namespace; and `action.yml` declares
  **typed input/output contracts** (the Pact/UTI shape of `notes/172`, realized in a mainstream system).
  This is the best concrete **template** for an *author-declared-but-analyzable-and-sh-native* channel — but
  note every handle/name is still **author-declared** (the output name in the `echo`, the input name in
  `action.yml`): co-reference analyzable, **kind/meaning declared**. Fourth confirmation of the no-magic
  negative.

- **k1c-ci-kcomms (+SURE) — GitHub Actions independently arrived at Dorc's `kCOMMS` two-lane split, forced
  by a CVE.** Freeform **log** annotations stay on stdout (`::notice::`/`::error::`); **coordination/state**
  moved to dedicated environment **files** (`GITHUB_ENV`/`GITHUB_OUTPUT`/`GITHUB_STATE`/`GITHUB_PATH`)
  [B-github-actions-workflow-commands-2024]. The migration was *forced* by an injection vuln — verbatim:
  *"to avoid untrusted logged data to use save-state and set-output workflow commands without the intention
  of the workflow author we have introduced a new set of environment files"* [B-github-actions-setoutput-deprecation-2022]
  (the 2020 `::set-env::`/`::add-path::` deprecation was the same class). **Real-world, hindsight-rich
  confirmation that signalling must not share a lane with freeform output** — `KNOBS kCOMMS` (human-owned;
  flagged, not edited). Even value-level: `GITHUB_ENV` may not set `NODE_OPTIONS`.

## Verdict for `plans/177` q1
**q1 answered: a real, analyzable, non-file/non-package coordination channel exists — the environment, esp.
convention-named cross-tool handles (`KUBECONFIG` model).** It is pervasive, already core-modeled, and it
*sharpens the handle-spelling*: a **conventional environment-variable name** beats a reverse-DNS string on
existing-independent-value + analyzability (real tools already read it). **But it delivers co-reference
only; the KIND stays convention/author-declared** — the third converging confirmation of `plans/177`'s
"no-magic" lean. Net effect on the synthesis: the free-co-reference surface *extends* beyond filenames to
named env-handles (a real gain), without cracking grounding (unchanged). **GitHub Actions (the human's
lead)** is the strongest real-world *template* for the author-declared-yet-analyzable channel
(`echo "name=value" >> "$GITHUB_OUTPUT"` + typed `action.yml`) and independently confirms `kCOMMS` — it
moved coordination onto environment *files* after a stdout-injection CVE — a 4th confirmation of the
negative, plus a transport bonus.

## Citations (verbatim; [slug]:loc, cite-certainty)

> [B-postgres-docker-entrypoint-2024]:docker_setup_env / docker_process_sql (relevance: +1:SURE)
> # Loads various settings that are used elsewhere in the script
> # This should be called before any other functions
> docker_setup_env() { file_env 'POSTGRES_PASSWORD'; file_env 'POSTGRES_USER' 'postgres'; file_env
> 'POSTGRES_DB' "$POSTGRES_USER"; file_env 'POSTGRES_INITDB_ARGS'; : "${POSTGRES_HOST_AUTH_METHOD:=}" … }
> … docker_process_sql() { local query_runner=( psql … --username "$POSTGRES_USER" … ); … PGHOST=
> PGHOSTADDR= "${query_runner[@]}" "$@"; }   # <- defensive unset = env-coordination is a hazard

> [C-petergardfjall-helm-init-2024]:helm-init.sh (relevance: -0:SUSPECT)
> # A script that sets up a secure Tiller server in the kubernetes cluster pointed to by the KUBECONFIG.
> which kubectl … || die_with_error "error: kubectl needs to be on your PATH"
> which helm   … || die_with_error "error: helm needs to be on your PATH"
> [[ -z ${KUBECONFIG} ]] && die_with_error "KUBECONFIG needs to be set"
> └ two independently-authored tools coordinate over the shared KUBECONFIG handle; the script declares a
>   precondition on it (the cross-oracle channel, realized as a convention-named env var).

> octocode aggregate (search observation, not a single source; via the K1c search) (relevance: +1:SURE)
> ~29 shell files matched `export KUBECONFIG=… ; kubectl … ; helm …`; dozens more matched
> `systemctl is-active "$svc"` ↔ `systemctl restart "$svc"`. The two dominant non-file/non-package
> coordination shapes in real ops sh: (1) env-var handles [cross-tool], (2) service-name probe/establish
> [within systemctl].

> [B-github-actions-workflow-commands-2024]:About / toolkit table (relevance: +1:SURE)
> Actions can communicate with the runner machine to set environment variables, output values used by other
> actions … Most workflow commands use the echo command in a specific format, while others are invoked by
> writing to a file.
> … core.exportVariable → GITHUB_ENV · core.setOutput → GITHUB_OUTPUT · core.saveState → GITHUB_STATE ·
> core.addPath → GITHUB_PATH · core.getInput → env INPUT_{NAME} · core.getState → env STATE_{NAME} ·
> (debug/notice/warning/error stay on stdout `::`).  Due to security restrictions, GITHUB_ENV cannot be
> used to set the NODE_OPTIONS environment variable.

> [B-github-actions-setoutput-deprecation-2022]:body (relevance: +1:SURE)
> To avoid untrusted logged data to use save-state and set-output workflow commands without the intention of
> the workflow author we have introduced a new set of environment files to manage state and output.
> … `echo "::set-output name={name}::{value}"` → `echo "{name}={value}" >> $GITHUB_OUTPUT`

## Open / next (→ plans/177)
- Fold k1c into `plans/177`: extend C2 (the handle) to *"reverse-DNS string OR a convention-named env-var,
  whichever already has independent consumers"*; add the env-channel **hazard** (ambient leakage, `099 W5`)
  to the open questions; keep the **co-reference-only / kind-declared** negative as now-triple-confirmed.
- Untaken (declared): PID/job coordination (`pid=$(pgrep x); kill "$pid"` — producer/consumer dataflow,
  kind opaque); lock-files/flock (file-ish); `trap`/signal handlers (control-flow). None expected to beat
  co-reference-only.
