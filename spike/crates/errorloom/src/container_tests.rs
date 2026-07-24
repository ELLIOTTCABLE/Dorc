//! Container tests (`282` §2): txtar + flat-frontmatter parse, byte-exact
//! round-trip, and the case-hygiene refusals (`28A` §1). Tests may unwrap freely.

use std::fmt::Write as _;

use crate::{
    Case, CaseError, CaseReadError, FrontmatterValue, MAX_CASE_BYTES, MAX_REPLAY_BLOCKS,
    MAX_REPLAY_COMMAND_BYTES, MAX_REPLAY_OUTPUT_BYTES, MAX_SECTION_BYTES, MAX_SECTION_COUNT,
    read_case,
};

const SAMPLE: &str = "---\ncode: render-heredoc-refused\nwhen-fires: the leaf-exact render would elide\nviews:\n  - verbose\n  - terse\n---\n-- book.sh --\n#!/bin/sh\ncat /etc/motd\n\n-- replay --\n$ tool plan --book=book.sh\nrender: error[render-heredoc-refused]: refused\n\n$ tool plan --format=jsonl\n{\"code\":\"render-heredoc-refused\"}\n";

#[test]
fn parse_round_trips_byte_identically() {
    let case = Case::parse(SAMPLE).expect("valid case");
    assert_eq!(case.to_text(), SAMPLE, "round-trip must be byte-identical");
}

#[test]
fn newlineless_output_still_round_trips_through_to_text() {
    // A real capture often lacks a trailing newline; injected into a non-final
    // block it must not fuse the next `$ ` command onto its last line (swe-F6).
    let mut case =
        Case::parse("---\n---\n-- replay --\n$ one\nold\n$ two\nold two\n").expect("valid case");
    case.set_replay_outputs(vec!["captured".to_owned(), "second\n".to_owned()]);
    let reparsed = Case::parse(&case.to_text()).expect("round-trips");
    assert_eq!(reparsed.replay().blocks().len(), 2);
    assert_eq!(reparsed.replay().blocks()[1].command(), "two");
}

#[test]
fn frontmatter_scalars_and_lists_parse() {
    let case = Case::parse(SAMPLE).expect("valid case");
    let fm = case.frontmatter();
    assert_eq!(fm.scalar("code"), Some("render-heredoc-refused"));
    assert_eq!(
        fm.get("views"),
        Some(&FrontmatterValue::List(vec![
            "verbose".to_owned(),
            "terse".to_owned()
        ]))
    );
    assert_eq!(fm.scalar("views"), None);
}

#[test]
fn sections_and_replay_split() {
    let case = Case::parse(SAMPLE).expect("valid case");
    assert_eq!(case.sections().len(), 1);
    assert_eq!(case.sections()[0].name(), "book.sh");
    assert_eq!(case.sections()[0].content(), "#!/bin/sh\ncat /etc/motd\n");
    let blocks = case.replay().blocks();
    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].command(), "tool plan --book=book.sh");
    assert_eq!(
        blocks[0].output(),
        "render: error[render-heredoc-refused]: refused\n"
    );
}

#[test]
fn empty_frontmatter_is_allowed() {
    let case = Case::parse("---\n---\n-- replay --\n$ tool go\nok\n").expect("valid");
    assert!(case.frontmatter().get("anything").is_none());
    assert_eq!(case.replay().blocks().len(), 1);
}

#[test]
fn crlf_refuses() {
    let err = Case::parse("---\r\ncode: x\r\n---\r\n-- replay --\r\n$ go\r\n").unwrap_err();
    assert_eq!(err, CaseError::ContainsCrlf);
}

#[test]
fn missing_frontmatter_refuses() {
    let err = Case::parse("-- replay --\n$ go\nok\n").unwrap_err();
    assert_eq!(err, CaseError::MissingFrontmatter);
}

#[test]
fn nested_frontmatter_refuses() {
    let err = Case::parse("---\nkey:\n  nested: deep\n---\n-- replay --\n$ go\nok\n").unwrap_err();
    assert!(matches!(err, CaseError::FrontmatterNotFlat { .. }));
}

#[test]
fn missing_replay_refuses() {
    let err = Case::parse("---\ncode: x\n---\n-- book.sh --\nhi\n").unwrap_err();
    assert_eq!(err, CaseError::NoReplaySection);
}

#[test]
fn replay_not_last_refuses() {
    let err = Case::parse("---\n---\n-- replay --\n$ go\nok\n-- trailer --\nx\n").unwrap_err();
    assert_eq!(err, CaseError::ReplayNotLast);
}

#[test]
fn unsafe_section_name_refuses() {
    let err = Case::parse("---\n---\n-- ../escape --\nx\n-- replay --\n$ go\nok\n").unwrap_err();
    assert!(matches!(err, CaseError::UnsafeSectionName { .. }));
}

#[test]
fn marker_collision_in_output_refuses() {
    // Fresh captured output carrying a marker line is caught at bless BEFORE it
    // is inlined — a committed file could never hold it (txtar would split it),
    // which is the "no escaping exists" sharp edge (`282` §2).
    let mut case = Case::parse("---\n---\n-- replay --\n$ go\nok\n").expect("parses");
    case.set_replay_outputs(vec!["-- sneaky --\n".to_owned()]);
    let err = case.check_hygiene(None).unwrap_err();
    assert!(matches!(err, CaseError::MarkerCollision { .. }));
}

#[test]
fn required_token_gate() {
    let ok = Case::parse(SAMPLE).expect("valid");
    ok.check_hygiene(Some("code"))
        .expect("both blocks surface the code");

    let missing = Case::parse("---\ncode: the-slug\n---\n-- replay --\n$ go\nno mention here\n")
        .expect("parses");
    let err = missing.check_hygiene(Some("code")).unwrap_err();
    assert!(matches!(
        err,
        CaseError::MissingRequiredToken { block: 0, .. }
    ));
}

#[test]
fn materialized_files_are_relative() {
    let case =
        Case::parse("---\n---\n-- hosts/web1/probe.txt --\nsite 0\n-- replay --\n$ go\nok\n")
            .expect("valid");
    let files = case.materialized_files();
    assert_eq!(files.len(), 1);
    assert!(files[0].0.is_relative());
    assert_eq!(files[0].1, "site 0\n");
}

#[test]
fn set_replay_outputs_rewrites_only_output() {
    let mut case = Case::parse(SAMPLE).expect("valid");
    case.set_replay_outputs(vec!["new one\n".to_owned(), "new two\n".to_owned()]);
    assert_eq!(case.replay().blocks()[0].output(), "new one\n");
    assert_eq!(
        case.replay().blocks()[1].command(),
        "tool plan --format=jsonl"
    );
}

#[test]
fn raw_layout_excludes_only_replay_outputs() {
    let head = "---\ncode: one\n---\npreamble\n-- book.sh --\nbook\n\n-- replay --\n$ tool one\nold\n\n$ tool two\nold two\n";
    let prose = head.replace(
        "old\n\n$ tool two\nold two",
        "new words\n\n$ tool two\nnewer words",
    );
    let frontmatter = prose.replace("code: one", "code: two");
    let command = prose.replace("$ tool two", "$ tool changed");
    let whitespace = prose.replace("preamble\n", "preamble \n");
    let head_layout = Case::raw_layout(head).expect("layout");
    assert!(head_layout.same_non_replay_output_bytes(
        head,
        &Case::raw_layout(&prose).expect("layout"),
        &prose,
    ));
    for changed in [&frontmatter, &command, &whitespace] {
        assert!(!head_layout.same_non_replay_output_bytes(
            head,
            &Case::raw_layout(changed).expect("layout"),
            changed,
        ));
    }
}

#[test]
fn parser_limits_admit_the_boundary_and_refuse_the_next_item_before_storage() {
    let section = format!("{}\n", "s".repeat(MAX_SECTION_BYTES.saturating_sub(1)));
    let at_section = format!("---\n---\n-- book --\n{section}-- replay --\n$ go\nok\n");
    assert!(Case::parse(&at_section).is_ok());
    let over_section = format!(
        "---\n---\n-- book --\n{}\n-- replay --\n$ go\nok\n",
        "s".repeat(MAX_SECTION_BYTES)
    );
    assert!(matches!(
        Case::parse(&over_section),
        Err(CaseError::LimitExceeded {
            component: "section bytes",
            ..
        })
    ));

    let commands = (0..MAX_REPLAY_BLOCKS)
        .map(|_| "$ go\nok\n")
        .collect::<String>();
    assert!(Case::parse(&format!("---\n---\n-- replay --\n{commands}")).is_ok());
    assert!(matches!(
        Case::parse(&format!("---\n---\n-- replay --\n{commands}$ go\nok\n")),
        Err(CaseError::LimitExceeded {
            component: "replay block count",
            ..
        })
    ));

    let command = "c".repeat(MAX_REPLAY_COMMAND_BYTES);
    assert!(Case::parse(&format!("---\n---\n-- replay --\n$ {command}\nok\n")).is_ok());
    assert!(matches!(
        Case::parse(&format!(
            "---\n---\n-- replay --\n$ {}\nok\n",
            "c".repeat(MAX_REPLAY_COMMAND_BYTES.saturating_add(1))
        )),
        Err(CaseError::LimitExceeded {
            component: "replay command bytes",
            ..
        })
    ));

    let output = "o".repeat(MAX_REPLAY_OUTPUT_BYTES);
    assert!(Case::parse(&format!("---\n---\n-- replay --\n$ go\n{output}")).is_ok());
    assert!(matches!(
        Case::parse(&format!(
            "---\n---\n-- replay --\n$ go\n{}",
            "o".repeat(MAX_REPLAY_OUTPUT_BYTES.saturating_add(1))
        )),
        Err(CaseError::LimitExceeded {
            component: "committed replay output bytes",
            ..
        })
    ));

    let mut sections = String::new();
    for index in 0..MAX_SECTION_COUNT.saturating_sub(1) {
        writeln!(sections, "-- {index} --\nx").expect("string write");
    }
    assert!(Case::parse(&format!("---\n---\n{sections}-- replay --\n$ go\nok\n")).is_ok());
    assert!(matches!(
        Case::parse(&format!(
            "---\n---\n{sections}-- extra --\nx\n-- replay --\n$ go\nok\n"
        )),
        Err(CaseError::LimitExceeded {
            component: "section count",
            ..
        })
    ));
}

#[test]
fn bounded_file_admission_refuses_over_limit_and_non_utf8_before_parse() {
    let dir = std::env::temp_dir().join(format!("errorloom-limit-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("test directory");
    let oversized = dir.join("oversized.loom");
    std::fs::write(&oversized, vec![b'x'; MAX_CASE_BYTES.saturating_add(1)]).expect("test case");
    assert!(matches!(
        read_case(&oversized),
        Err(CaseReadError::TooLarge)
    ));
    let malformed = dir.join("malformed.loom");
    std::fs::write(&malformed, b"---\n---\n-- replay --\n$ go\n\xff").expect("test case");
    assert!(matches!(
        read_case(&malformed),
        Err(CaseReadError::NonUtf8(_))
    ));
    let _ = std::fs::remove_dir_all(&dir);
}
