# errorloom

Executable transcript cases as the authoring-surface for a CLI tool's
user-facing prose. (**Write your messages directly into your test-files, while
looking directly at what your user sees.**)

Here's a live example, [from Dorc](https://github.com/ELLIOTTCABLE/Dorc/blob/326018ce/spike/crates/dorc-loom/cases/cmdsub-operand-top.txt),
the project for which I built this library:

```sh
---
code: cmdsub-operand-top
---
-- book.sh --
#!/bin/sh
# pi-webhost provisioning (curated package set from the fleet inventory)
set -e

apt-get update
apt-get install -y "$(cat /etc/webhost/pkgset)"
systemctl enable nginx

-- replay --
$ dorc plan --book=book.sh
note[cmdsub-operand-top]: operand 3 is a command-substitution `$(…)` /
   arithmetic / operator-form expansion, so Dorc cannot know its value until the
   command runs on the host — there is nothing to resolve and no read-only probe
   to check. Dorc elides a command only when it can prove the command's effect
   is already in place, so this one is left to run on every apply.
   --> book.sh:6:20
   |
 6 | apt-get install -y "$(cat /etc/webhost/pkgset)"
   |                    \__________________________/
   = repair: Give the operand a value Dorc can resolve statically — a literal,
      or a variable assigned from one — so Dorc can probe it and elide the
      command once it has converged. If the value must stay dynamic, load an
      oracle that vouches for this command's convergence, and Dorc will guard
      the command instead of running it every time.

$ dorc plan --book=book.sh --format=jsonl
{"code":"cmdsub-operand-top","severity":"note"}

```

(Note that none of this rendering is errorloom's; your renderer owns the
rendering and formatting, errorloom doesn't care. It only cares about the
distinct *difference* between structure and prose. The human diagnostic above
can carry typed editable provenance; the JSONL bytes are still tested but are
not editable unless that exact renderer also supplies provenance for them.)

That's the elevator pitch: no separate file to edit. You open your CLI's
shell-output E2E test-case, and modify the prose right there, looking at exactly
what your user sees whilei you do so.

> This was built as internal tooling while heads-down on [Dorc][]. It is
> *entirely* AI-written; distrust that as suits your risk-profile. (I generally
> wouldn't.) It was lightly reviewed by me (a human); and I'm using it in anger
> on difficult work ... take that as you will. I will be maintaining and
> responding to any issues, though; I'm sharing this because I believe it has
> value.
>
> It's got sharp edges and may need refinement as I work with it; but
> it should be ready for use if you're curious.

   [Dorc]: <https://github.com/ELLIOTTCABLE/Dorc> "my POSIX-sh Ansible-alike orchestrator tool"


## Rationale: why?

At best, most tools store their error/help/status strings in a catalog; a table
of message templates. Authors edit that table in-place. (At worst, errors are
simply shat out inline, into running code, while working locally on a particular
task, with minimal thought about UX at all, lol.)

My problem with this: whoever writes a string at line 250 of a module (or line
1200 of a gargantuan catalog) is in *tool-author headspace*. They know things
the user doesn't; they can't see the carets, the surrounding output, or what the
message looks like in context. They can't see the *actual input-context the user
just produced and interacted with*, that caused the error for *them*.

The prose drifts away from what a user actually experiences.

errorloom inverts the direction: the authoring surface is the *executable
transcript case*, a recorded run of the tool (input state + the exact bytes the
command printed). Authors edit the *rendered transcript*, seeing exactly what a
user sees - effectively an end-to-end test-case. The renderer supplies an
editable tree beside the exact replay bytes it produced: immutable structure,
immutable data, and editable prose containing opaque variable identities.
errorloom transports an edit through that tree; the consumer compiles the
result into its own catalog. Output bytes without such a tree remain ordinary,
fully-tested transcript output and never become editable merely because they
look like prose. The catalog becomes a derived artifact; the committed
transcript is the source of truth.

Lineage (steal-instead-of-invent is The Way): cram / Mercurial t-tests, Go
`txtar` + `testscript`, `rustc tests/ui --bless`, insta, terraform-plugin-docs.
The only genuinely novel leg, as far as we could find, is the *diff-driven
extraction of edits back into the message catalog*.

(That is, the "you *actually do your editing* directly into the case-file's
output representation, inline. You edit no `.rs` source to improve your errors'
prose.)


## The case-file format

A case is one txtar archive with a `---`-fenced flat-YAML-subset frontmatter
head:

```sh
---
code: motd-refused
when-fires: the leaf-exact render would elide a heredoc-bearing leaf
---

-- book.sh --
#!/bin/sh
cat <<EOF >/etc/motd
hello
EOF

-- probe-results.txt --
site 0 effect=holds

-- replay --
$ mytool plan --book=book.sh < probe-results.txt
render: error[motd-refused]: refusing to elide the heredoc-bearing leaf

$ mytool plan --book=book.sh --format=jsonl < probe-results.txt
{"envelope":"lint/1","code":"motd-refused"}
```

 - **Frontmatter** is an opaque flat map to errorloom (`key: value` scalars and
   `key:` + `- item` lists; nested structures refuse). The schema - which keys
   mean what - belongs to you.
 - **File sections** are materialized verbatim to a temp dir before the replay
   runs. Names may be `/`-joined paths; absolute or `..`-climbing names refuse.
 - **The replay section** (always last) is a sequence of `$ `-prefixed command
    blocks, each followed by exactly what the command printed. Commands run
    sequentially in one shared temp cwd, so state flows between them by design.
    A replay command is opaque to generic errorloom. An embedding consumer may
    handle an exact invocation shape in-process; everything else may be routed
    explicitly to the generic executor.

Case/replay admission is deliberately bounded: 256 KiB per case file, 64
sections, 128 KiB per section, 32 replay blocks, 8 KiB per command, and 64 KiB
per committed replay output or live generic capture. `read_case` reads only one
byte beyond the case ceiling before decoding; parser and runner limits are typed
refusals rather than truncation.


### Case-hygiene gates

Blunt refusals, all generic, either NYI or out-of-scope:

 - CRLF in any section;
 - a replay-output line that parses as a txtar marker (there is no escaping. A
   committed file could never hold it, so this is caught on fresh output at
   bless);
 - a captured line leaking the sandbox's absolute path (cwd is the temp dir, so
   paths render relative);
 - and a configurable required-token gate - name a frontmatter key and every
   replay block's output must surface its value (e.g. "every replay must mention
   its own `error-code`" or whatever you like).


## CLI: the generic cram mode

The `errorloom` binary is the fully-generic cram tool. It deliberately selects
errorloom's generic replay executor; it has no consumer-specific
in-process driver and grants no catalog-edit authority. The environment is
entirely caller-injected (`env -i`-style): nothing ambient leaks in.

```sh
errorloom run   --shell=PATH [--path=DIR]... [--env=K=V]... [--require-token=KEY] CASE...
errorloom bless --shell=PATH [--path=DIR]... [--env=K=V]... [--require-token=KEY] CASE...
```

 - `run` materializes each case, executes its blocks, and compares each block's
   combined (`2>&1`) output to the committed transcript. It exits nonzero on any
   drift. Byte stability under re-execution is the run gate.
 - `bless` re-runs and re-inlines each block's actual output (structure-bless),
   then applies the hygiene gates and writes the case back.

`--shell` selects the executor; it is required because errorloom never discovers an
ambient shell. `--path` dirs become the child `PATH`; `--env K=V` sets exact
variables. Policy such as inert mocks is yours; a `--path` pointing at a mocks dir
keeps real tools out.

Both gates are one call deep in-process, if you'd rather drive them from your
own test-suite (`bless_structure` is `bless`; `run_case` yields the raw
captures):

```rust
use errorloom::{Case, RunEnv, check_run};

#[test]
fn transcripts_are_byte_stable() -> Result<(), Box<dyn std::error::Error>> {
    let case = Case::parse(&std::fs::read_to_string("cases/motd-refused.loom")?)?;
    let env = RunEnv::new()
        .path_dir("target/debug")   // the tool under test
        .path_dir("tests/mocks");   // inert stand-ins for everything else
    let report = check_run(&case, &env)?;
    assert!(report.is_clean(), "{:#?}", report.drifts());
    Ok(())
}
```

The generic CLI deliberately stops at transcript execution and structure-bless.
Template syntax, payload lookup, command dispatch, provenance production, and
catalog generation are consumer policy. Current errorloom provides the generic
executor and identity-preserving transport those tools build on; the target API below
adds the consumer-neutral driver/result boundary between them.


## Library API: replay driving

The generic executor, editable transport, and the consumer-neutral driver/result
boundary all exist; the embedding Dorc consumer drives exact-result provenance and owns
the durable compile/promote loop built on top. Locating editable output by matching
rendered contents was rejected and is not a compatibility surface.

Every replay produces exact bytes. An embedding consumer may first try to drive
the original command text itself, returning either `Decline` or a handled result
containing those exact bytes plus an optional typed `EditableRender`. Driving a
command and exposing editable prose are separate capabilities: an in-process
machine renderer may return bytes only, while a future external driver could
return provenance if it preserves the mapping explicitly.

The reusable generic executor is a mechanism, not an implicit fallback. The
embedding application decides whether a decline is fatal or routes it to that
executor. Thus the standalone `errorloom` CLI chooses generic execution, while a
consumer-specific tool can compose its in-process driver with a controlled
fallback. Errorloom itself knows no consumer command names, flags, output formats,
or template syntax.

For example, a consumer may handle `mytool plan FILE` directly and return editable
message regions. It may decline `mytool plan FILE | jq --pretty`; the configured
generic executor still tests the final transformed bytes, but arbitrary
transformation destroys edit authority unless a future transformation-aware
driver explicitly preserves it.


## Library API: editable transport

For one exact replay result, your renderer may hand errorloom an
`EditableRender`: those exact rendered bytes expressed as an ordered tree of
three component classes.

 - `Structure` is immutable layout: frames, labels, carets, and whitespace.
 - `FixedVariable` is immutable rendered data outside editable prose.
 - `EditableSection` contains an ordered `Text | Variable` series. Section and
   variable IDs are opaque consumer types.

`transport_edit` accepts ordinary prose edits in exactly one section while
preserving every untouched variable by identity before tokenization.
`transport_edit_allow_removal` additionally accepts a uniquely-attributable
variable omission. Both are bounded and refuse structure edits, fixed-data
edits, cross-section changes, ambiguous attribution, and excessive work.

Changing surrounding text may move an untouched variable to a different byte
offset without requiring an explicit template marker. Existing rendered values
may also be relocated within the same editable section when one unique
identity-preserving interpretation exists. Consumer markers are the fail-clear
fallback for destroyed anchors, equal-value ambiguity, duplication/new
occurrences, or cross-section movement, not routine editing ceremony.

The returned `SectionEdit` is deliberately not a catalog template. A consumer
compiles its text using its own strict syntax, resolves names against its typed
payload, and regenerates a concrete render. For example, Dorc owns the spelling
`{{path}}`; errorloom sees those bytes only as consumer-editable text. This is
the boundary that lets other consumers use another template language without
putting that language into errorloom.

Structure regeneration and CI fixpoint checking remain generic. Implement
`CaseRenderer`, pass the committed cases to `fixpoint_check`, and use
`structure_bless` only when case transcripts and the generated catalog are
clean. The consumer-specific compile/promote command is responsible for
catalog writes and for proving its own compile-to-render fixpoint.


## Status

Pre-1.0, experimental, extracted from a design spike. `publish = false` until a
LICENSE is chosen. The prose model deliberately starts small - words and
paragraphs only; paragraph *restructuring* (adding/removing breaks) refuses.
Expect the API to move; growable public enums are `#[non_exhaustive]`.
