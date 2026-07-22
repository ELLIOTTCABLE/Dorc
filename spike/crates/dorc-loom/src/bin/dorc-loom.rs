//! `dorc-loom` is the read-only transcript-template inspection command.

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use dorc_loom::{DorcConsumer, DorcSectionEditRefusal, compile_preview, render_compile_preview};
use errorloom::{Case, EditableFragment, RenderComponent};

const USAGE: &str = "usage: dorc-loom <compile CASE...|vars <--used|--all> CASE...>";

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(message) => {
            let _ = writeln!(io::stderr(), "dorc-loom: {message}");
            ExitCode::from(2)
        }
    }
}

enum Command {
    Compile(Vec<PathBuf>),
    Vars { used: bool, cases: Vec<PathBuf> },
}

fn run() -> Result<ExitCode, String> {
    let command = parse_args()?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    match command {
        Command::Compile(cases) => compile_cases(&cases, &mut out),
        Command::Vars { used, cases } => print_variables(used, &cases, &mut out),
    }
}

fn parse_args() -> Result<Command, String> {
    let mut argv = std::env::args().skip(1);
    match argv.next().as_deref() {
        Some("compile") => {
            let cases = collect_cases(argv)?;
            Ok(Command::Compile(cases))
        }
        Some("vars") => {
            let mode = argv
                .next()
                .ok_or_else(|| format!("vars needs --used or --all\n{USAGE}"))?;
            let used = match mode.as_str() {
                "--used" => true,
                "--all" => false,
                _ => return Err(format!("unknown vars mode {mode:?}\n{USAGE}")),
            };
            Ok(Command::Vars {
                used,
                cases: collect_cases(argv)?,
            })
        }
        _ => Err(USAGE.to_owned()),
    }
}

fn collect_cases(argv: impl Iterator<Item = String>) -> Result<Vec<PathBuf>, String> {
    let mut cases = Vec::new();
    for arg in argv {
        if arg.starts_with('-') {
            return Err(format!("unknown option {arg:?}\n{USAGE}"));
        }
        cases.push(PathBuf::from(arg));
    }
    if cases.is_empty() {
        return Err(format!("no case files given\n{USAGE}"));
    }
    Ok(cases)
}

fn compile_cases(cases: &[PathBuf], out: &mut impl Write) -> Result<ExitCode, String> {
    let consumer = DorcConsumer::new();
    let mut refused = false;
    for path in cases {
        let case = load(path)?;
        let baseline = consumer
            .editable_baseline(&case)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        writeln!(out, "case: {}", path.display()).map_err(|error| error.to_string())?;
        let mut previews = Vec::new();
        let mut case_refusal = None;
        for (index, block) in case.replay().blocks().iter().enumerate() {
            let dirty = unreflow(block.output());
            if matches_editable_skeleton(&baseline, &dirty) {
                match compile_preview(&baseline, &dirty) {
                    Ok(preview) => previews.push((index, preview)),
                    Err(DorcSectionEditRefusal::Unchanged) => {}
                    Err(error) => case_refusal = Some((index, error, dirty)),
                }
            } else if resembles_diagnostic(&baseline, &dirty)
                && (dirty.contains("{{") || dirty.contains("}}"))
            {
                case_refusal = Some((
                    index,
                    DorcSectionEditRefusal::MarkerOutsideEditableSection,
                    dirty,
                ));
            }
        }
        if let Some((index, error, dirty)) = case_refusal {
            refused = true;
            writeln!(out, "refusal in replay {index}: {error:?}")
                .map_err(|write| write.to_string())?;
            writeln!(out, "baseline:\n{}", baseline.render().text())
                .map_err(|write| write.to_string())?;
            writeln!(out, "edited:\n{dirty}").map_err(|write| write.to_string())?;
            continue;
        }
        for (index, preview) in previews {
            writeln!(out, "replay: {index}").map_err(|error| error.to_string())?;
            writeln!(out, "{}", render_compile_preview(&preview))
                .map_err(|error| error.to_string())?;
        }
    }
    Ok(if refused {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    })
}

fn resembles_diagnostic(baseline: &dorc_loom::DorcEditableBaseline, dirty: &str) -> bool {
    baseline
        .render()
        .components()
        .iter()
        .find_map(|component| match component {
            RenderComponent::Structure(text) => Some(dirty.starts_with(text)),
            RenderComponent::FixedVariable { .. } | RenderComponent::EditableSection(_) => None,
        })
        .unwrap_or(false)
}

fn print_variables(
    used: bool,
    cases: &[PathBuf],
    out: &mut impl Write,
) -> Result<ExitCode, String> {
    let consumer = DorcConsumer::new();
    for path in cases {
        let case = load(path)?;
        let baseline = consumer
            .editable_baseline(&case)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        writeln!(out, "case: {}", path.display()).map_err(|error| error.to_string())?;
        if used {
            for (name, value) in baseline.used_variables() {
                writeln!(out, "{{{{{}}}}} = {value:?}", name.0)
                    .map_err(|error| error.to_string())?;
            }
        } else {
            for (name, value) in baseline.all_variables() {
                writeln!(out, "{{{{{}}}}} = {value:?}", name.0)
                    .map_err(|error| error.to_string())?;
            }
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn load(path: &PathBuf) -> Result<Case, String> {
    let source =
        fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    Case::parse(&source).map_err(|error| format!("{}: {error}", path.display()))
}

// case files wrap prose; core tags do not
fn unreflow(render: &str) -> String {
    let mut lines = render.lines().peekable();
    let mut out = Vec::new();
    if let Some(first) = lines.next() {
        out.push(join_continuations(first, &mut lines, "   "));
    }
    while let Some(line) = lines.next() {
        if line.trim_start().starts_with("= ") {
            out.push(normalize_layout(&join_continuations(
                line, &mut lines, "      ",
            )));
        } else {
            out.push(normalize_layout(line));
        }
    }
    out.join("\n")
}

fn matches_editable_skeleton(baseline: &dorc_loom::DorcEditableBaseline, dirty: &str) -> bool {
    let mut rest = dirty;
    for (index, component) in baseline.render().components().iter().enumerate() {
        match component {
            RenderComponent::Structure(text)
            | RenderComponent::FixedVariable { rendered: text, .. } => {
                let Some(remaining) = rest.strip_prefix(text) else {
                    return false;
                };
                rest = remaining;
            }
            RenderComponent::EditableSection(_) => {
                let anchor: String = baseline
                    .render()
                    .components()
                    .get(index.saturating_add(1)..)
                    .unwrap_or_default()
                    .iter()
                    .take_while(|component| {
                        !matches!(component, RenderComponent::EditableSection(_))
                    })
                    .map(component_text)
                    .collect();
                if anchor.is_empty() {
                    return true;
                }
                let Some(offset) = rest.find(&anchor) else {
                    return false;
                };
                rest = &rest[offset..];
            }
        }
    }
    rest.is_empty()
}

fn component_text(
    component: &RenderComponent<dorc_loom::SectionKey, dorc_loom::SectionVariableId>,
) -> String {
    match component {
        RenderComponent::Structure(text)
        | RenderComponent::FixedVariable { rendered: text, .. } => text.clone(),
        RenderComponent::EditableSection(section) => section
            .fragments()
            .iter()
            .map(|fragment| match fragment {
                EditableFragment::Text(text)
                | EditableFragment::Variable { rendered: text, .. } => text.clone(),
            })
            .collect(),
    }
}

fn normalize_layout(line: &str) -> String {
    if let Some(rest) = line.strip_prefix("   = ") {
        return format!("  = {rest}");
    }
    if let Some(rest) = line.strip_prefix("  ")
        && rest
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_digit())
    {
        return format!(" {rest}");
    }
    line.to_owned()
}

fn join_continuations<'a>(
    first: &str,
    lines: &mut std::iter::Peekable<impl Iterator<Item = &'a str>>,
    indent: &str,
) -> String {
    let mut joined = first.to_owned();
    while lines
        .peek()
        .is_some_and(|line| line.starts_with(indent) && !line.trim_start().starts_with("-->"))
    {
        let line = lines.next().unwrap_or_default();
        joined.push(' ');
        joined.push_str(line.trim());
    }
    joined
}
