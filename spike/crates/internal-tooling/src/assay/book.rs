use super::alloy::{self, Head};

pub(super) const EVERY_LINE: &str = "every_line";

pub(super) fn every_line(members: &[(String, String)], clause: &str) -> Option<String> {
    (members.len() >= 2).then(|| {
        let bodies: Vec<String> = members
            .iter()
            // A block, since a body may juxtapose formulas, which only a block conjoins.
            .map(|(_, body)| format!("{{ {body} }}"))
            .collect();
        format!(
            "check {EVERY_LINE} {{ {} }} for {clause}",
            bodies.join(" and ")
        )
    })
}

pub(super) fn check_formula(text: &str) -> Option<(String, String)> {
    let item = alloy::items(text, 1).into_iter().next()?;
    let Head::Command {
        check: true,
        name: Some(name),
        scope: None,
        ..
    } = item.head()
    else {
        return None;
    };
    let open = item
        .toks
        .get(2)
        .filter(|t| alloy::text(&item.text, t) == "{")?;
    let close = item
        .toks
        .last()
        .filter(|t| alloy::text(&item.text, t) == "}")?;
    let mut depth: usize = 0;
    for (i, tok) in item.toks.iter().enumerate().skip(2) {
        match alloy::text(&item.text, tok) {
            "{" | "(" | "[" => depth = depth.saturating_add(1),
            "}" | ")" | "]" => {
                depth = depth.saturating_sub(1);
                if depth == 0 && i.saturating_add(1) != item.toks.len() {
                    return None;
                }
            }
            _ => {}
        }
    }
    let formula = item.text.get(open.end..close.start)?.trim().to_owned();
    Some((name, formula))
}

#[cfg(test)]
mod tests {
    use super::{check_formula, every_line};

    #[test]
    fn the_conjunction_carries_each_body_verbatim_and_needs_two() {
        let members = vec![
            ("line_2".to_owned(), "(a)".to_owned()),
            ("line_4".to_owned(), "(a) implies (b)".to_owned()),
        ];
        assert_eq!(
            every_line(&members, "5 but exactly 3 Line").as_deref(),
            Some("check every_line { { (a) } and { (a) implies (b) } } for 5 but exactly 3 Line")
        );
        assert_eq!(every_line(&members[..1], "5"), None);
    }

    #[test]
    fn only_an_inline_scope_less_check_joins() {
        assert_eq!(
            check_formula("check k { all x: A | { x in B } }"),
            Some(("k".to_owned(), "all x: A | { x in B }".to_owned()))
        );
        assert_eq!(check_formula("check k"), None, "names an assertion");
        assert_eq!(check_formula("run k { a }"), None);
        assert_eq!(check_formula("check k { a } expect 1"), None);
    }
}
