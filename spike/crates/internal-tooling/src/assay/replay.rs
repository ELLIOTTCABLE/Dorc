//! Evaluate before solving (`notes/30Yf` § 6): a stored instance that still satisfies the current
//! command's formula is that command's answer, found by evaluation. The formula carries the facts
//! and the claim but not the declarations' own constraints, so replay is sound only while every
//! paragraph other than facts, assertions, and commands is byte-identical to the closure the
//! instance was found in, under the same scope string: then the constraints it satisfied still hold.

use std::path::{Path, PathBuf};

use super::alloy::{self, Head};
use crate::sha256::Sha256;

/// The declarations a stored instance was found under: every non-fact, non-assert, non-command
/// paragraph of the closure, and the scope.
pub(super) fn guard(closure: &[(String, String)], scope: &str) -> String {
    let mut modules: Vec<&(String, String)> = closure.iter().collect();
    modules.sort();
    let mut h = Sha256::default();
    h.field(b"assay-replay-guard/1");
    h.field(scope.as_bytes());
    for (path, text) in modules {
        h.field(path.as_bytes());
        for item in alloy::items(text, 1) {
            let constrains_worlds_only = matches!(item.head(), Head::Command { .. })
                || matches!(item.word(0), "fact" | "assert")
                || (item.word(0) == "private" && matches!(item.word(1), "fact" | "assert"));
            if !constrains_worlds_only {
                h.field(item.text.as_bytes());
            }
        }
    }
    h.hex()
}

/// Where a command's last instance lives: target dir, never the tree.
fn path(out: &Path, module: &str, label: &str) -> PathBuf {
    out.join("instances")
        .join(module)
        .join(format!("{label}.xml"))
}

/// The instance stored for a command, with the guard it was found under.
pub(super) fn load(out: &Path, module: &str, label: &str) -> Option<(String, String)> {
    let text = std::fs::read_to_string(path(out, module, label)).ok()?;
    let (guard, xml) = text.split_once('\n')?;
    Some((
        guard
            .strip_prefix("<!-- assay-guard ")?
            .strip_suffix(" -->")?
            .to_owned(),
        xml.to_owned(),
    ))
}

pub(super) fn store(out: &Path, module: &str, label: &str, guard: &str, xml: &str) {
    let file = path(out, module, label);
    if let Some(dir) = file.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(file, format!("<!-- assay-guard {guard} -->\n{xml}"));
}

pub(super) fn forget(out: &Path, module: &str, label: &str) {
    let _ = std::fs::remove_file(path(out, module, label));
}

/// The three belts over the instance itself: the command's bitwidth; every exact bound the scope
/// names met exactly; and every signature and field in the instance still declared.
pub(super) fn fits(xml: &str, bitwidth: u64, scope: &str, sigs: &[(String, Vec<String>)]) -> bool {
    let Some(instance) = tags(xml, "instance").into_iter().next() else {
        return false;
    };
    if attr(instance.0, "bitwidth").and_then(|b| b.parse::<u64>().ok()) != Some(bitwidth) {
        return false;
    }
    let mut ids: Vec<(String, String)> = Vec::new();
    for (open, body) in tags(xml, "sig") {
        let (Some(label), Some(id)) = (attr(open, "label"), attr(open, "ID")) else {
            return false;
        };
        ids.push((id.to_owned(), label.to_owned()));
        if attr(open, "builtin") == Some("yes") {
            continue;
        }
        if !sigs.iter().any(|(l, _)| l == label) {
            return false;
        }
        let atoms = body.matches("<atom ").count();
        if exact_bounds(scope)
            .iter()
            .any(|(n, sig)| label.rsplit('/').next() == Some(sig.as_str()) && atoms != *n)
        {
            return false;
        }
    }
    tags(xml, "field").iter().all(|(open, _)| {
        let owner = attr(open, "parentID").and_then(|p| ids.iter().find(|(id, _)| id == p));
        match (attr(open, "label"), owner) {
            (Some(field), Some((_, sig))) => sigs
                .iter()
                .any(|(l, fields)| l == sig && fields.iter().any(|f| f == field)),
            _ => false,
        }
    })
}

/// `exactly N Sig` clauses of a scope, as Alloy renders it.
fn exact_bounds(scope: &str) -> Vec<(usize, String)> {
    let words: Vec<&str> = scope.split([' ', ',']).filter(|w| !w.is_empty()).collect();
    words
        .windows(3)
        .filter(|w| w.first() == Some(&"exactly"))
        .filter_map(|w| Some((w.get(1)?.parse().ok()?, (*w.get(2)?).to_owned())))
        .collect()
}

/// Every `<name …>…</name>` element: its opening tag's text and its body (empty when self-closed).
fn tags<'a>(xml: &'a str, name: &str) -> Vec<(&'a str, &'a str)> {
    let open = format!("<{name} ");
    let close = format!("</{name}>");
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(at) = rest.find(&open) {
        let Some(tail) = rest.get(at..) else { break };
        let Some(end) = tail.find('>') else { break };
        let head = tail.get(..end).unwrap_or("");
        let after = tail.get(end.saturating_add(1)..).unwrap_or("");
        if head.ends_with('/') {
            out.push((head, ""));
            rest = after;
            continue;
        }
        let body_end = after.find(&close).unwrap_or(after.len());
        out.push((head, after.get(..body_end).unwrap_or("")));
        rest = after.get(body_end..).unwrap_or("");
    }
    out
}

fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!(" {name}=\"");
    let at = tag.find(&needle)?.saturating_add(needle.len());
    let rest = tag.get(at..)?;
    rest.get(..rest.find('"')?)
}

#[cfg(test)]
mod tests {
    use super::{fits, guard};

    const XML: &str = r#"<alloy builddate="x">
<instance bitwidth="4" maxseq="3" command="Check k for 3" tracelength="1">
<sig label="Int" ID="1" parentID="2" builtin="yes">
</sig>
<sig label="this/A" ID="4" parentID="2">
   <atom label="A$0"/>
   <atom label="A$1"/>
</sig>
<field label="f" ID="5" parentID="4">
   <tuple> <atom label="A$1"/> <atom label="A$1"/> </tuple>
</field>
</instance>
</alloy>"#;

    fn sigs(fields: &[&str]) -> Vec<(String, Vec<String>)> {
        vec![(
            "this/A".to_owned(),
            fields.iter().map(|f| (*f).to_owned()).collect(),
        )]
    }

    #[test]
    fn an_instance_fits_only_its_bitwidth_its_exact_bounds_and_its_declarations() {
        assert!(fits(XML, 4, "3 but exactly 2 A", &sigs(&["f"])));
        assert!(!fits(XML, 5, "3", &sigs(&["f"])), "another bitwidth");
        assert!(
            !fits(XML, 4, "3 but exactly 3 A", &sigs(&["f"])),
            "an exact bound unmet"
        );
        assert!(!fits(XML, 4, "3", &sigs(&[])), "a field no longer declared");
        assert!(!fits(XML, 4, "3", &[]), "a signature no longer declared");
    }

    #[test]
    fn the_guard_moves_with_declarations_and_never_with_facts() {
        // F1 in 30Yf's breakpoint: an added sig fact leaves the command's formula true of a
        // stale instance, so any declaration change must force a solve.
        let base = vec![(
            "m.als".to_owned(),
            "module m\nsig A { f: lone A }\nfact { some A }\ncheck k { no a: A | a.f = a } for 3\n"
                .to_owned(),
        )];
        let edit = |from: &str, to: &str| vec![(base[0].0.clone(), base[0].1.replace(from, to))];
        let g = guard(&base, "3");
        assert_eq!(
            g,
            guard(&edit("some A", "#A > 1"), "3"),
            "a fact edit keeps it"
        );
        assert_eq!(
            g,
            guard(&edit("no a: A", "all a: A"), "3"),
            "a claim edit keeps it"
        );
        assert_ne!(
            g,
            guard(
                &edit("sig A { f: lone A }", "sig A { f: lone A } { f != this }"),
                "3"
            )
        );
        assert_ne!(g, guard(&edit("lone A }", "one A }"), "3"));
        assert_ne!(g, guard(&base, "4"));
    }
}
