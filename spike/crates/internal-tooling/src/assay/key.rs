use super::alloy::{self, Head};
use crate::sha256::Sha256;

#[derive(Debug, Clone)]
pub(super) struct Fixed {
    pub(super) jar: String,
    pub(super) adapter: String,
    pub(super) options: String,
}

// Hashes the bytes Alloy parsed, never a normalisation: a normaliser bug must show, not match.
pub(super) fn key(
    closure: &[(String, String)],
    command: &str,
    scope: &str,
    fixed: &Fixed,
) -> String {
    let mut modules: Vec<&(String, String)> = closure.iter().collect();
    modules.sort();
    let mut h = Sha256::default();
    h.field(b"assay-key/1");
    h.field(
        &u64::try_from(modules.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for (path, text) in modules {
        h.field(path.as_bytes());
        h.field(alloy::strip_commands(text).as_bytes());
    }
    for part in [command, scope, &fixed.jar, &fixed.adapter, &fixed.options] {
        h.field(part.as_bytes());
    }
    h.hex()
}

pub(super) fn command_text(module: &str, label: &str, nth_unnamed: usize) -> Option<String> {
    let commands: Vec<(Option<String>, String)> = alloy::items(module, 1)
        .into_iter()
        .filter_map(|item| match item.head() {
            Head::Command { name, .. } => Some((name, item.text)),
            Head::Sig { .. } | Head::Other => None,
        })
        .collect();
    commands
        .iter()
        .find(|(n, _)| n.as_deref() == Some(label))
        .or_else(|| {
            commands
                .iter()
                .filter(|(n, _)| n.is_none())
                .nth(nth_unnamed)
        })
        .map(|(_, text)| text.clone())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::super::{Inputs, Source, compile};
    use super::{Fixed, command_text, key};

    const SHARED: &str = "```alloy\nsig Speaker {}\nabstract sig Blurb extends Claim { speaker: one Speaker }\nrun bookScope {} for 4 but 3 Int\n```\n";

    fn keys(doc: &str) -> BTreeMap<(String, String), String> {
        let inputs = Inputs {
            doc: Source {
                name: "k.assay.md",
                text: doc,
            },
            shared: Some(Source {
                name: "shared.assay.md",
                text: SHARED,
            }),
            laws: None,
        };
        let compiled = compile(&inputs).expect("the test document compiles");
        let closure: Vec<(String, String)> = compiled
            .modules
            .iter()
            .map(|(n, r)| (n.clone(), r.text.clone()))
            .collect();
        let fixed = Fixed {
            jar: "jar".to_owned(),
            adapter: "adapter".to_owned(),
            options: "{}".to_owned(),
        };
        let mut out = BTreeMap::new();
        for (name, rendered) in &compiled.modules {
            for item in super::alloy::items(&rendered.text, 1) {
                if let super::Head::Command {
                    name: Some(label), ..
                } = item.head()
                {
                    let text =
                        command_text(&rendered.text, &label, 0).expect("the command is there");
                    let module = name.trim_end_matches(".als").to_owned();
                    out.insert((module, label), key(&closure, &text, "scope", &fixed));
                }
            }
        }
        out
    }

    const DOC: &str = "```alloy\nsig Gadget extends Blurb {}\npred loud[g: Gadget] { some g.speaker }\ncheck lawA { all g: Gadget | loud[g] } for 3\nrun lawA_premise { some Gadget } for 3\ncheck lawB { no Gadget } for 3\n```\n\n```sh\n# b.sh\n   frob x\n#} frob x\n#= this in Line\n   spin y\n#} spin y\n#= this in Line\n   twirl z\n#} twirl z\n#= this.cmd in Shword\n```\n";

    #[test]
    fn comments_and_whitespace_in_a_fence_move_no_key() {
        let reworded = DOC
            .replace(
                "pred loud[g: Gadget] { some g.speaker }",
                "-- a note\npred loud[g: Gadget] {\n      some   g.speaker   // why\n}\n\n\n",
            )
            .replace(
                "check lawB { no Gadget } for 3",
                "check lawB {\n  no Gadget /* none */\n}\n  for 3",
            );
        assert_eq!(keys(DOC), keys(&reworded));
    }

    #[test]
    fn a_law_edit_moves_only_its_own_key_and_a_line_edit_moves_its_readers() {
        let before = keys(DOC);
        let law = keys(&DOC.replace("check lawB { no Gadget }", "check lawB { lone Gadget }"));
        let moved = |after: &BTreeMap<(String, String), String>| -> Vec<String> {
            before
                .iter()
                .filter(|(k, v)| after.get(*k) != Some(*v))
                .map(|((m, l), _)| format!("{m}.{l}"))
                .collect()
        };
        assert_eq!(moved(&law), vec!["laws.lawB"]);
        let line = keys(&DOC.replace("#= this in Line\n   spin", "#= this.cmd in Shword\n   spin"));
        assert_eq!(
            moved(&line),
            vec![
                "book_b.b",
                "book_b.every_line",
                "book_b.line_1",
                "book_b.line_2",
                "book_b.line_3"
            ]
        );
    }
}
