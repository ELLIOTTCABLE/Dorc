CONTRIBUTING - tooling & project standards
------------------------------------------

First off, an issue is more useful than a patch.

It's 2026, and we all have our own LLMs; if your fix looks LLM generated, I'll
probably discard it and dispatch my own. LLMs do a poor job of handling some of
the critical invariants in this project, and I want my own eyes on them while
they work.

As a counter, though, this project is largely LLM-generated - so *hand-authored*
patches to the outlying regions are probably somewhat wasteful; as those regions
are currently churned fairly heavily by LLMs; broad strokes trumps minutae.
(This has exceptions - the core analysis-kernel, for instance, is subject to
higher scruitny and churns less often; human eyes are more useful there.)


## Tooling

I use [Mise][] heavily; it's easiest to manage the tooling via that.

First-time setup, after cloning the repo for the first time (not necessary in a
worktree):

```sh
curl https://mise.run | sh    # or <https://mise.jdx.dev/installing-mise.html>
mise trust                    # required by mise
mise run hello                # briefly setup project-local tooling (repo-local)
```

Compile and test:

```sh
mise run both gate            # runs a full build: unit-tests, E2E, and looms
mise run dorc --help          # run Dorc! (args are passed in raw.)

# rerun a single test-case
mise run test errexit_unknown_is_conservative
```

Our tests include both [errorloom][]s (executable, *authoritative* e2e txtar
files that you edit in-place; see below) and a small residue of non-loomable e2e
tests, all of which are run by the above `mise run gate`. `mise run tests` runs
*only* the Rust unit-tests, and is less valuable. `mise run` will list all
available tasks.

   [mise]: <https://mise.jdx.dev/getting-started.html> "mise-en-place, a cross-platform tool/version manager and dev-environment manager"
   [errorloom]: <https://github.com/ELLIOTTCABLE/Dorc/tree/main/spike/crates/errorloom> "our format for e2e tests and prose-authorship"


### Live-test

There's a (small) automated-test-against-live-machine that can be executed
againest either a Docker container or a VPS. (On Windows, `wslc.exe` is
substituted for Docker.)

```sh
mise run livetest
```

You should see an initial apply that can't skip much; then a re-apply that
indeed skips several converged commands (note the `elide=4`):

```console
livetest: pristine plan matches baseline (sites=12 elide=1 omit=0 guard=0 run=11)
livetest: apply 1 succeeded in 8s
livetest: converged plan matches baseline (sites=12 elide=4 omit=4 guard=0 run=4)
livetest: apply 2 succeeded in 3s
livetest: OK - probe shipped, plan matched baseline, applied for real, re-planned converged, re-applied clean
```

Teardown is automated unless it fails (or you pass `LIVETEST_KEEP=1`.)


### Use Dorc by hand

<!-- TODO: This section should be dogfooded. Requires local-pivot to be alive;
            but at the end of the day, this entire setup-section should be a
            *book*, not Markdown. -->

By hand, if you prefer, using `docker`/`wslc` to stand up a container you can
point `dorc` at to experiment:

1. **Create a target**. On a *nix box:

   ```sh
   ssh-keygen -t ed25519 -N '' -f .tmp/temp-ssh-key -q
   docker run -d --name dorc-target -p 2222:2222 \
      -e PUBLIC_KEY="$(cat .tmp/temp-ssh-key.pub)" \
      lscr.io/linuxserver/openssh-server:latest
   printf 'IdentityFile %s/.tmp/temp-ssh-key\nStrictHostKeyChecking accept-new\nUserKnownHostsFile %s/.tmp/temp-known-hosts\n' "$PWD" "$PWD" > .tmp/temp-ssh-config
   ```

   ... or, on Windows, one can use the new `wslc` instead of installing Docker
   explicitly, with some caveats:

    - as of July 2026, ships with a WSL pre-release; `wsl --update --pre-release`
      from an admin PowerShell;
    - and, because `wslc` is a sibling-VM to `wsl` itself, you need to bind to
      `0.0.0.0` to ensure access from *within* WSL.

   ```powershell
   ssh-keygen -t ed25519 -N '""' -f .tmp\temp-ssh-key -q
   wslc run -d --name dorc-target -p 0.0.0.0:2222:2222 `
      -e PUBLIC_KEY="$((Get-Content .tmp\temp-ssh-key.pub -Raw).Trim())" `
      lscr.io/linuxserver/openssh-server:latest
   "IdentityFile $PWD\.tmp\temp-ssh-key`nStrictHostKeyChecking accept-new`nUserKnownHostsFile $PWD\.tmp\temp-known-hosts`n" `
      | Set-Content -NoNewline .tmp\temp-ssh-config
   ```

   (Note: for this simple example, that's an Alpine image, and the ssh-user
   isn't root. Adjust as you see fit.)

2. **Bridge into WSL**, if you tend to work inside a WSL terminal:

   ```sh
   # /mnt/c always looks world-writable to Linux; ssh refuses to load a key with
   # those permissions. stage a chmod-fixed copy WSL's own ssh client can use:
   mkdir -p ~/.cache/dorc
   cp .tmp/temp-ssh-key ~/.cache/dorc/temp-ssh-key
   chmod 600 ~/.cache/dorc/temp-ssh-key
   printf 'IdentityFile ~/.cache/dorc/temp-ssh-key\nStrictHostKeyChecking accept-new\nUserKnownHostsFile ~/.cache/dorc/temp-known-hosts\n' \
      >~/.cache/dorc/temp-ssh-config
   ```

3. **Run Dorc**, with your config / flags:

   You now have an ssh destination. Point Dorc at it - bring your own book and
   oracle(s); this repo's own `cp` oracle is small, real, and already sitting in
   the tree if you'd rather copy-paste something that works than write one
   first:

   ```sh
   # demo-book.sh
   cp /etc/os-release /config/os-release-copy
   ```

   ```console
   # WSL2's NAT means 127.0.0.1 does not reach the container from here; resolve
   # instead thru the Windows host's address as WSL currently sees it.
   # (this changes across `wsl --shutdown`/reboot; re-run it if stale)
   ip route show | grep -i default | awk '{print $3}'
   172.28.0.1

   # - host-address: 127.0.0.1 (or on WSL2, the above-printed IP)
   # - config-path: must match, either the WSL-specific ~/.cache/dorc *or*
   #   outside WSL, the platform-native / repo-local .tmp config
   $ mise run dorc -- plan --book="$PWD/demo-book.sh" \
      -o "$PWD/Research/trial/r26/oracles/cp.oracle.sh" \
      --host linuxserver.io@<host-address>:2222 \
      --ssh-config "<ssh-config-path>"
   ```


## Authorship, prose, and error-messages

Effectively *all* user-facing prose in this project (error-messages, explanatory
output, generated comments) is edited *in-situ* in "looms". (Again, see the
[errorloom][] README.) You don't find a string in the source-code and change it,
you type *directly* into the e2e-case text-file, while looking directly at the
(machine-checked) input-source-code and command-invocation-sequence that produce
that state.

```console
nano spike\crates\aid\tests\cli-help-page.loom  # change a word,
mise run loom:compile                           # check your changes,
mise run loom:promote                           # and promote them to the catalog
git commit spike\crates\aid\tests\cli-help-page.loom \
   spike/crates/aid/src/arrangement_lock.rs \
   -m "(- re doc) Reword the ..."
```

Note that you *must* have a clean worktree for this. Loom-editing is *only*
allowed in a completely clean working-dir. This is because loom is a
*render-back* process - it reads what you changed, then re-renders everything
from scratch, and *overwrites* your edits. (You may `git add` before compiling
if you wish, but still, the only modified files must be looms if *any* edited
file is a loom.) The render-back depends on the entire project's state and is
inherently E2E.

After editing, the promotion will print your changes as a preview; after
confirming they rendered how you wanted, you can `git add` and commit.


## Commiting

This project uses granular committing. I *deeply* prefer helpful, pickaxe-able
history with a full accounting of what went wrong and what redirections
happened, to a sanitized, squashed history that loses mechanical context that's
useful when fighting fires and debugging.

Lots of small, focused commits while working.

Similarly, lean towards rebase over merges: I like a *semantically rich*
history, but not a messy one.

This project uses [.gitlabels][]; read the file ([./.gitlabels]()) before
writing your commit-message, it describes the format in detail. Please do not
elaborate on obvious information; the commit-message is for *out-of-band*
information, and queries about the files/components/crates it touches is one
`git show` away. Describe, briefly, the *rationale* (why/how), use imperative
voice, and keep it very brief. The code-changes describe themselves, always; the
commit-message does not.

The *purpose* of gitlabels is filtering: commits will be *ignored* / skimmed
over by readers, based on them; their honesty is critical. Please don't
cargo-cult what you see in previous commits.

*All* AI-assisted work *must* contain the `(AI)` gitlabel. If you wish to
differentiate, then, again, granular committing - commit your LLM's work
separately, then commit your own tweaks without the label; but if they're
mixed, they get the label. Period.

   [.gitlabels]: <https://github.com/ELLIOTTCABLE/.gitlabels>
