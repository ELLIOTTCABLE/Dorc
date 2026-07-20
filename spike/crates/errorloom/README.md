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
distinct *difference* between strutcture, and prose.)

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
user sees - effectively an end-to-end test-case. errorloom then extracts the
edits back into the catalog by a word-level diff, attributed through the
render's own provenance. The catalog becomes a derived artifact; the committed
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

The `errorloom` binary is the fully-generic cram tool. The environment is
entirely caller-injected (`env -i`-style): nothing ambient leaks in.

```sh
errorloom run   [--path=DIR]... [--env=K=V]... [--require-token=KEY] CASE...
errorloom bless [--path=DIR]... [--env=K=V]... [--require-token=KEY] CASE...
```

 - `run` materializes each case, executes its blocks, and compares each block's
   combined (`2>&1`) output to the committed transcript. It exits nonzero on any
   drift. Byte stability under re-execution is the run gate.
 - `bless` re-runs and re-inlines each block's actual output (structure-bless),
   then applies the hygiene gates and writes the case back.

`--path` dirs are searched for each command's program and become the child
`PATH`; `--env K=V` sets exact variables. Policy such as inert mocks is yours; a
`--path` pointing at a mocks dir keeps real tools out.

Both gates are one call deep in-process, if you'd rather drive them from your
own test-suite (`bless_structure` is `bless`; `run_case` yields the raw
captures):

```rust
use errorloom::{Case, RunEnv, check_run};

#[test]
fn transcripts_are_byte_stable() -> Result<(), Box<dyn std::error::Error>> {
    let case = Case::parse(&std::fs::read_to_string("cases/motd-refused.txt")?)?;
    let env = RunEnv::new()
        .path_dir("target/debug")   // the tool under test
        .path_dir("tests/mocks");   // inert stand-ins for everything else
    let report = check_run(&case, &env)?;
    assert!(report.is_clean(), "{:#?}", report.drifts());
    Ok(())
}
```

The primary **prose-promote** flow is deliberately *not* in the CLI: it needs
consumer callbacks (see below).


## Library API: the promote flow and the two bless modes

Prose-promote is library-only because it needs a [`Consumer`] (a baseline tagged
render for a case, apply field-edits, re-render a case) and a two-method [`Git`]
(`head_version_of`, `dirty_paths`; a subprocess-`git` impl ships, a `FakeGit` is
provided for tests). errorloom drives the loop; the catalog and case schema stay
consumer-side.

There's two bless modes under a mechanical exclusivity law (meaning both *must*
be executed separately):

 - `prose_bless`: structure is frozen; an author edited words in a transcript.

   Legal only when the touched-set is case-files-only and the catalog is
   clean.

   errorloom re-renders each dirty case from the current catalog (the baseline),
   word-diffs it against the author's edited transcript, attributes each change
   through the span map, re-holes param values, regenerates the catalog, and
   re-renders the corpus. A "baseline-verify" re-renders with current state
   and requires it to match HEAD's transcript *everywhere except prose regions*
   - a mismatch means the structure moved, so it refuses with "structure-bless
   first". (This verify is the 'never-both'.)

 - `structure_bless`: catalog prose is frozen; code or arrangement changed.

   Legal only when the touched-set is code-only with case prose untouched. Every
   transcript is regenerated from scratch; prose provably cannot have drifted
   because it only flows from the unchanged catalog.

 - In both classes dirty, or a dirty (hand-edited) catalog, is refused.
   errorloom's approach is the only accepted approach.

Finally, the CI fixpoint gate (`fixpoint_check`): every committed case must
re-render to its own committed bytes. A catalog hand-edit (prose or metadata)
moves the render off the committed transcript, so the gate catches it. This is
what lets the promote flow be trusted without a hand-maintained authorship
roster.

A consumer, sketched:

```rust
use std::collections::BTreeMap;
use errorloom::{Case, Consumer, FieldTemplate, ParamTables, TaggedBaseline, TaggedRender};

struct MyTool {
    catalog: BTreeMap<(String, String), String>,
}

impl Consumer for MyTool {
    type Key = (String, String); // opaque to errorloom; Dorc's is `(code, field)`
    type Error = String;

    // The attribution baseline: render from CURRENT catalog state, one Span
    // per byte-run (the span-map contract, below)
    fn tagged_render(&self, case: &Case) -> Result<TaggedBaseline<Self::Key>, String> {
        let (text, spans) = todo!("your tagged renderer");
        let render = TaggedRender::new(text, spans).map_err(|e| e.to_string())?;
        Ok(TaggedBaseline::new(render, ParamTables::new()))
    }

    // The transcript block the author edits, as it sits on disk
    fn editable_text(&self, case: &Case) -> Result<String, String> {
        Ok(case.replay().blocks().first()
            .map(|b| b.output().to_owned()).unwrap_or_default())
    }

    fn apply_field_edits(&mut self, edits: &BTreeMap<Self::Key, FieldTemplate>)
    -> Result<(), String> {
        todo!("write each template back into your catalog")
    }

    fn render_case(&self, case: &Case) -> Result<String, String> {
        todo!("the whole case text, re-rendered from CURRENT catalog state")
    }
}
```

... and the drive loop:

```rust
use std::path::Path;
use errorloom::{CaseFile, SubprocessGit, fixpoint_check, prose_bless};

let mut corpus = Vec::new();
for path in case_paths { // git-relative; they're compared against `git status`
    corpus.push(CaseFile::new(&path, std::fs::read_to_string(&path)?));
}
let git = SubprocessGit::new(".");

// e.g. in CI: every committed case must re-render to its own committed bytes.
fixpoint_check(&tool, &corpus)?;

// An author edited words in a transcript. Extract, regenerate, write back;
// the review surface is the resulting git diff
let blessed = prose_bless(&mut tool, &git, &corpus, Path::new("src/catalog.rs"))?;
for (path, text) in blessed.regenerated() {
    std::fs::write(path, text)?;
}

// ... and `structure_bless(&tool, &git, &corpus, ...)` in the same shape.
```


## The span-map contract

Your tagged-renderer hands `promote` a [`TaggedRender`]: the rendered bytes plus
a [`Span`] map classifying every run as a [`Region`]. The map is validated on
construction to be a **gap-free, non-overlapping total cover** of the render
bytes - region lookup must be total. Whitespace and other structure between
prose runs are covered as `Arrangement` runs; there are no holes.

The regions:

 - **`TemplateLiteral { key, paragraph, instance }`** - the field's own prose
   words, the ONLY editable class.
 - **`ParamValue { key, param, instance }`** - interpolated payload; editing it
   refuses (it is data, not prose).
 - **`ForeignText { param }`** - passthrough foreign text; editing it refuses.
 - **`Arrangement { slug }`** - render-owned structure (numbering, connectives,
   tier words); edit it by structure-bless, not prose-bless.

Instance discriminators (`instance: Option<InstanceId>`, Dorc's
`28A:rul-tagged-render-emits-instance-ids`): a field may render more than once
in a transcript. When you stamp each render with an `InstanceId`, promotion
groups spans into instances by *exact* identity; when absent, it falls back to
structural inference (paragraph/adjacency). Opting in is per-key,
all-or-nothing.


## Status

Pre-1.0, experimental, extracted from a design spike. `publish = false` until a
LICENSE is chosen. The prose model deliberately starts small - words and
paragraphs only; paragraph *restructuring* (adding/removing breaks) refuses.
Expect the API to move; growable public enums are `#[non_exhaustive]`.
