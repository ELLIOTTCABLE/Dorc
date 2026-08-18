# 30Ic - `command -v` across shells: specification, measurements, and idioms

> Tier: LLM-authored research note from an in-chat investigation requested by the
> human on 2026-08-18. This is EVIDENCE, not design authority and not a ruling.
> `plans/30I`, root human documents, and typed law outrank it. Confidence markers:
> `+SURE` directly measured or read in a primary source; `~SUSPECT` inference from
> those observations; `-GUESS` weak inference; `--WONDER` an open question.
>
> Scope: the shell behavior of `command -v`, the proposed output-slash test,
> one PATH-suppressed variant, and the observed shell-library use of sentinel
> variables. The short-term loading model, speaker/custody/vouch minting,
> long-term `command -v` support, and local-versus-remote environment policy are
> deliberately NOT designed here. Section 8 reserves those questions for the
> human-led `30I` design continuation.

## 0. Findings in one screen

- `fnd-command-v-asks-shell-resolution` [+SURE] - POSIX `command -v name`
  reports what the current shell would invoke under that name. Its answer space
  deliberately includes executable utilities, functions, aliases, builtins,
  reserved words, and implementation-provided facilities. It does not natively
  answer either "is an external executable present?" or "was this exact shell
  package loaded?" [A-posix-command-utility-2024].
- `fnd-slash-test-has-standard-basis` [+SURE] - POSIX Issue 8 requires an
  executable utility found through PATH to be printed as an absolute pathname,
  while functions and non-PATH builtins are printed as bare names. Testing the
  output for a slash therefore has a standards-based rationale; it is not an
  arbitrary invention.
- `fnd-slash-test-fails-real-floors` [+SURE] - pinned dash 0.5.12 and posh
  0.14.1 both print a bare executable name when an empty PATH component finds it
  in the current directory. That contradicts Issue 8's absolute-path output
  requirement and makes slash-presence insufficient over the actual floors.
- `fnd-busybox-applets-are-bare` [+SURE, relevance deferred] - Ubuntu's
  BusyBox 1.36.1 build is configured for standalone applets and reports applets
  such as `ls` as bare names independently of ordinary PATH lookup. This is a
  real run-target behavior, but this investigation did not establish that such a
  BusyBox mode belongs to Dorc's executor floor.
- `fnd-floor-function-cell-agrees` [+SURE] - for an ordinary defined function
  name, every measured shell returned status 0 and the bare function name. The
  ordinary positive function-definedness cell is much more stable than the full
  `command -v` classification surface.
- `fnd-path-suppression-separates-measured-external` [+SURE, empirical only] -
  `PATH=/dev/null command -v ordinary_name` kept defined functions visible and
  made an external `python3` unavailable on both floors without leaking the
  temporary PATH assignment. It remains a shell-namespace query: aliases,
  builtins, reserved words, and standalone applets are not excluded by this
  observation.
- `fnd-sentinel-guards-are-established` [+SURE] - package/library-owned
  `_..._LOADED` variables are an established shell-library include-guard idiom,
  present in actual libraries and published guidance. They are not a Dorc- or
  LLM-invented shape.
- `fnd-sentinels-inherit-like-shell-vars` [+SURE] - a matching sentinel in the
  parent environment is imported by both floors and can suppress loading. This
  is ordinary shell behavior. OpenSSH does not forward arbitrary client
  variables by default; explicit client/server configuration is required.

## 1. What POSIX specifies

`command -v` is specified as a query over the current shell execution
environment [A-posix-command-utility-2024]. The required categories are:

| resolved category | Issue 8 `command -v` output |
|---|---|
| executable utility | absolute pathname |
| PATH-associated regular builtin | absolute pathname |
| slash-bearing command name | absolute pathname |
| implementation-provided function found through PATH | absolute pathname |
| ordinary shell function | bare name |
| special builtin | bare name |
| regular builtin not associated with PATH | bare name |
| reserved word | bare name |
| alias | a command line representing its alias definition |
| absent or errored | no output; status greater than zero |

Three details are load-bearing for interpreting measurements:

1. `obs-not-found-status-is-only-nonzero` [+SURE] - POSIX promises only a
   status greater than zero for a failed `-v` query. The measured 1-versus-127
   split is conforming at this level and is harmless to an ordinary boolean
   guard.
2. `obs-command-v-is-broader-than-which` [+SURE] - the standard deliberately
   subsumes historical `type`, `whence`, and `which` use cases. ShellCheck's
   guidance likewise says `command -v` is correct for "what would run in this
   shell" and explicitly warns that functions, aliases, and builtins are part of
   that answer [B-shellcheck-command-guidance-2026].
3. `obs-command-v-subenvironment-caveat` [+SURE] - POSIX warns that a query in
   a subshell or separate utility execution environment may be unable to report
   aliases, functions, or special builtins correctly. This makes a
   command-substitution parser less portable than the output table alone would
   suggest, even though the measured shells retained functions in the tested
   command substitutions.

POSIX PATH has two relevant qualifications [A-posix-path-environment-2024]:

- a zero-length PATH prefix is the legacy spelling for the current directory;
  strictly conforming applications should spell `.` instead; and
- an unset or null PATH has implementation-defined search behavior.

## 2. Measurement method and versions

All measurements were non-mutating `-c` invocations under WSL Ubuntu. No Dorc
fixture was executed. The exact shell set was:

| shell | measured version / provenance |
|---|---|
| dash | Ubuntu `0.5.12-6ubuntu5` |
| posh | Ubuntu `0.14.1` |
| Bash | Ubuntu `5.2.21`, invoked with `--posix` |
| zsh | Ubuntu `5.9`, invoked with `-f` |
| BusyBox ash | Ubuntu BusyBox `1.36.1-6ubuntu3.1`; standalone applets enabled |

The command-category matrix defined an ordinary function, attempted two
aliases, queried a reserved word and builtins, then queried present and absent
external names. The PATH matrix ran from `/usr/bin` under PATH values
`/usr/bin`, `.`, `:`, and the null string, and queried both `ls` and external
`python3`. A final cell queried `/usr/bin/ls`, `./ls`, and non-executable
`/etc/passwd`.

`[A-local-shell-measurement-2026]` denotes these direct observations below.

## 3. Command-category results

| subject | dash | posh | BusyBox ash | Bash POSIX | zsh |
|---|---|---|---|---|---|
| function `f` | `f`, 0 | `f`, 0 | `f`, 0 | `f`, 0 | `f`, 0 |
| alias to `/bin/true` | alias text, 0 | absent, 1 | alias text, 0 | alias text, 0 | alias text, 0 |
| alias to `true` | alias text, 0 | absent, 1 | alias text, 0 | alias text, 0 | alias text, 0 |
| reserved word `if` | `if`, 0 | absent, 1 | `if`, 0 | `if`, 0 | `if`, 0 |
| builtin `break` | `break`, 0 | `break`, 0 | `break`, 0 | `break`, 0 | `break`, 0 |
| builtin `command` | `command`, 0 | `command`, 0 | `command`, 0 | `command`, 0 | `command`, 0 |
| `printf` | `printf`, 0 | `/bin/printf`, 0 | `printf`, 0 | `printf`, 0 | `printf`, 0 |
| ordinary external `ls` | `/bin/ls`, 0 | `/bin/ls`, 0 | `ls`, 0 | `/bin/ls`, 0 | `/bin/ls`, 0 |
| absent name | empty, 127 | empty, 1 | empty, 127 | empty, 1 | empty, 1 |

Interpretation:

- `obs-posh-category-surface-is-narrower` [+SURE] - the pinned posh did not
  expose aliases or the tested reserved word through `command -v`, and classified
  `printf` as the external `/bin/printf` where the other measured shells used a
  builtin. This does not perturb the ordinary function-positive cell, but it
  refutes any assumption that the full classification is floor-identical.
- `obs-alias-output-can-contain-slashes` [+SURE] - dash, BusyBox ash, Bash, and
  zsh printed alias-definition text. An alias whose replacement contains a path
  therefore produces slash-bearing output that is not itself a reusable pathname.
- `obs-broader-shells-add-more-categories` [+SURE from manuals] - FreeBSD sh,
  OpenBSD ksh, mksh, and zsh document tracked aliases, autoloaded functions,
  implementation builtins, or `FPATH`/`fpath` behavior. Moving beyond the floor
  does not collapse the answer space toward a function/executable bit.

## 4. PATH and slash results

For an external `python3` while the working directory was `/usr/bin`:

| PATH | dash | posh | BusyBox ash | Bash POSIX | zsh |
|---|---|---|---|---|---|
| `/usr/bin` | `/usr/bin/python3` | `/usr/bin/python3` | `/usr/bin/python3` | `/usr/bin/python3` | `/usr/bin/python3` |
| `.` | `./python3` | `./python3` | `./python3` | `/usr/bin/./python3` | `./python3` |
| `:` | `python3` | `python3` | `python3` | `/usr/bin/./python3` | `python3` |
| null | `python3` | `python3` | `python3` | `/usr/bin/python3` | `python3` |

`fnd-zero-prefix-defeats-slash-test` [+SURE] - the `PATH=:` row is enough to
refute slash-presence as a total external-versus-function discriminator over
the two actual floors. Although the zero-length prefix is a legacy feature,
leading, trailing, and adjacent colons are ordinary shell states and both pinned
binaries support them.

`fnd-dash-padvance-explains-bare-output` [+SURE from current source, exact
version corroborated by measurement] - dash's path expansion omits the slash
when a PATH component has zero length, and `describe_command()` prints that
expanded string directly. The current dash source therefore explains the
observed result without relying on presentation accidents
[A-dash-command-source-2026].

`fnd-busybox-standalone-bypasses-path` [+SURE] - the tested BusyBox announces:
"The shell in this build is configured to run built-in utilities without $PATH
search." Its FAQ documents this as optional standalone-shell behavior, disabled
by `defconfig` but enabled in some builds [A-busybox-standalone-shell-2026]. The
measured `command -v ls -> ls` is therefore configuration-sensitive and not
evidence about every BusyBox ash. Its relevance to Dorc's eventual executor
floor remains unmeasured.

`fnd-existing-slash-is-not-executable-proof` [+SURE] - with `PATH=/etc`, dash
did not report the non-executable `passwd`; but `command -v /etc/passwd`
nevertheless succeeded and printed `/etc/passwd`. BusyBox ash did the same;
posh, Bash, and zsh refused it. Capturing a slash-bearing result does not by
itself establish executability on all measured shells.

## 5. The proposed slash idiom

The investigated family was approximately:

```sh
case $(command -v "$name" 2>/dev/null) in
*/*) external ;;
*)   shell-resident-or-absent ;;
esac
```

The evidence establishes four independent limitations:

1. `lim-external-can-be-bare` [+SURE] - empty PATH components on the floors and
   standalone BusyBox applets produce bare external or external-like command
   names.
2. `lim-alias-can-contain-slash` [+SURE] - alias-definition text can contain a
   slash without being an executable pathname.
3. `lim-bare-name-has-many-species` [+SURE] - functions, special builtins,
   non-PATH builtins, and reserved words share the standard's bare-name shape.
4. `lim-command-substitution-has-spec-caveat` [+SURE] - the standard explicitly
   warns that subenvironment queries may lose shell-resident categories.

Adding `-x`, for example `candidate=$(command -v ...)` followed by a slash test
and `[ -x "$candidate" ]`, can make some external-executable checks fail in the
conservative direction. It does not turn the result into exact function or
package identity, and it retains the PATH and command-substitution variation.

No searched source showed output-slash parsing as an established shell-library
include-guard idiom. `command -v` itself is highly idiomatic; parsing its output
for package/function identity was not found to be.

## 6. PATH-suppressed query

This stronger experimental form was measured:

```sh
PATH=/dev/null command -v example_common_query >/dev/null 2>&1
```

On both floors:

- a defined ordinary function remained visible and returned 0;
- external `python3` became unavailable (dash 127, posh 1); and
- the caller's PATH value was unchanged after the command.

With output captured under `PATH=/dev/null`, the broader matrix still exposed
aliases where supported, reserved words where supported, and non-PATH builtins.
The form therefore suppresses ordinary PATH executables in the measured shells;
it does not identify an exact function definition or source file. BusyBox
standalone applets remain a separate configuration-sensitive exception.

`obs-path-suppression-is-not-found-idiom` [+SURE within the search performed] -
Kagi and GitHub searches found no established use of `PATH=/dev/null command -v`
as a portable function-definedness idiom. This is a bounded negative search,
not proof of absence.

## 7. Sentinel-variable observations

The common shell-library shape is:

```sh
[ -n "${package_loaded-}" ] && return 0
package_loaded=1
```

or an equivalent caller-side guard around `.`. Evidence that this is an
established idiom:

- Shellac carries `SHELLAC_LOADED` plus one
  `_SHELLAC_LOADED_<module>_<library>` variable per loaded library, and its
  source files begin with a set/unset guard [B-shellac-sentinel-library-2026].
- 1Password's agent-hooks libraries use `_LIB_OS_LOADED`-style guards
  [B-onepassword-sentinel-library-2026].
- Published shell-library guidance describes `_MYLIB_LOADED` as an idempotent
  sourcing guard [C-bash-coding-standard-2026].
- A broader GitHub code search found the same family in shell libraries,
  plugin managers, prompt frameworks, and sourced utility collections. This is
  prevalence evidence only; those projects are mostly Bash-oriented and do not
  establish a POSIX contract by themselves.

`obs-sentinel-floor-semantics-agree` [+SURE] - using a nounset-safe expansion,
dash and posh agreed on all three tested states: unset -> load, exact value ->
reuse, wrong value -> load.

`obs-sentinel-can-arrive-from-environment` [+SURE] - POSIX shells initialize
valid-name variables from the process environment
[A-posix-shell-environment-2024]. Direct measurement confirmed that launching
either floor with an exact sentinel in its environment made the guard read it as
loaded. That is ordinary sh behavior rather than a Dorc-specific channel.

`obs-ssh-does-not-forward-arbitrary-client-env` [+SURE] - OpenSSH requires a
client `SendEnv`/`SetEnv` request and a matching server `AcceptEnv`; the server
default accepts no arbitrary variables [A-openssh-environment-boundary-2026].
Ansible exposes remote environment injection explicitly through its
`environment` keyword [B-ansible-remote-environment-2026]. This investigation
did not settle Dorc's local-versus-remote environment policy.

POSIX reserves variable names containing lowercase letters for applications,
which provides a standards-grounded namespace option for library sentinels but
does not prevent application-to-application collision
[A-posix-path-environment-2024].

## 8. Design questions intentionally not adjudicated here

The following sections are reserved for the human-led design continuation and
must be worked into `plans/30I` or another authoritative home rather than ruled
in this evidence note:

### 8.1 Long-term meaningful `command -v` support - RESERVED

What semantic questions should retain `command -v` as the idiomatic authored
route; what model is required; and which cross-floor cells can be deferred?

### 8.2 Short-term sentinel constant propagation - RESERVED

Which variable-test spellings are admitted; how initial state is treated; what
the target file must assign; and what is merely contract versus machine proof?

### 8.3 Speaker, custody, and vouch minting - RESERVED

Whether a variable test itself communicates authorship, merely makes a source
edge decidable, or is inadmissible to speaker minting without a separate exact
source/definition proof.

### 8.4 Local and remote environment policy - PARKED

Whether controller-local and remote target books inherit, scrub, or explicitly
receive invocation environment, and whether one default can serve both without
surprising experienced sh authors.

### 8.5 BusyBox executor relevance - PARKED

Whether standalone-app-enabled BusyBox ash is a common/default executor in any
target population Dorc promises. The behavior is measured; its product weight is
not.

## 9. Source ledger

Grades: A = primary standard, implementation source/manual, or direct
measurement; B = authoritative tool/project documentation or a real project
implementation; C = community/style evidence useful only for idiom prevalence.

- `[A-posix-command-utility-2024]` - The Open Group, POSIX Issue 8 `command`:
  https://pubs.opengroup.org/onlinepubs/9799919799/utilities/command.html
- `[A-posix-shell-environment-2024]` - The Open Group, Shell Command Language:
  https://pubs.opengroup.org/onlinepubs/9799919799/utilities/V3_chap02.html
- `[A-posix-path-environment-2024]` - The Open Group, Environment Variables:
  https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap08.html
- `[A-local-shell-measurement-2026]` - direct WSL measurements recorded in
  sections 2-7; exact package versions listed in section 2.
- `[A-dash-command-source-2026]` - dash `exec.c`, `padvance()` and
  `describe_command()`:
  https://raw.githubusercontent.com/tklauser/dash/master/src/exec.c
- `[A-dash-command-manual-2026]` - dash manual, PATH and `command -v`:
  https://man7.org/linux/man-pages/man1/dash.1.html
- `[A-busybox-standalone-shell-2026]` - BusyBox FAQ, standalone shell:
  https://busybox.net/FAQ.html#standalone_shell
- `[A-bash-command-manual-2026]` - Bash Reference Manual, builtins and aliases:
  https://www.gnu.org/software/bash/manual/html_node/Bourne-Shell-Builtins.html
- `[A-openssh-environment-boundary-2026]` - OpenSSH `AcceptEnv`:
  https://man.openbsd.org/sshd_config.5#AcceptEnv
- `[B-shellcheck-command-guidance-2026]` - ShellCheck SC2230:
  https://www.shellcheck.net/wiki/SC2230
- `[B-shellac-sentinel-library-2026]` - Shellac library loader and sentinels:
  https://github.com/rawiriblundell/shellac
- `[B-onepassword-sentinel-library-2026]` - 1Password agent-hooks `lib/os.sh`:
  https://github.com/1Password/agent-hooks/blob/main/lib/os.sh
- `[B-ansible-remote-environment-2026]` - Ansible remote environment guide:
  https://docs.ansible.com/ansible/latest/playbook_guide/playbooks_environment.html
- `[C-bash-coding-standard-2026]` - idempotent sourcing guard guidance:
  https://github.com/Open-Technology-Foundation/bash-coding-standard/blob/main/docs/BCS-Bash-Ref/10_Sourcing-Libraries-and-Modules/04_Idempotent-sourcing-guards.md
