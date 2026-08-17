//! What the arrangement registry owes its callers
//! (`289:rul-arrangement-home-is-registry-plus-transcripts`).
//!
//! Two different jobs live here, and the difference is which pin OWNS the bytes.
//!
//! The value-interleaving ARITHMETIC (`words[0] v0 words[1] v1 …`, word runs = values + 1) is
//! stated over a synthesized row: it is a property of the seat, so it needs no committed prose and
//! must survive every rewording of it.
//!
//! A committed row's own BYTES are pinned here only while nothing else pins them. A row rendered by
//! a committed transcript already has the sanctioned pin — which re-blesses freely with the prose
//! (`27V:rul-output-form-unwelded`) — so a literal here would be a second, invisible owner, and a
//! prose edit through `dorc-loom compile`/`promote` would redden a crate its author never
//! opens. What is left is the genuinely FACELESS residue: rows no case owns and no transcript
//! renders, whose only net is a literal.

use dorc_aid::arrangement::{CONST_ARRANGEMENTS, OwnedArrangement, arrangement_sentence};
use dorc_aid::prose::ProseTier;

fn rendered(slug: &str, values: &[&str]) -> String {
    arrangement_sentence(&CONST_ARRANGEMENTS, slug, None, values)
}

/// The arithmetic every value-bearing chrome line is composed by, over a row nobody owns.
///
/// The empty word run is the case worth stating: a plural suffix rides in one, and two values
/// separated by nothing is exactly the position an off-by-one silently swaps.
#[test]
fn a_chrome_line_interleaves_its_values_between_its_word_runs() {
    let registry = vec![OwnedArrangement {
        slug: "harness-sentence".to_owned(),
        occurrence: None,
        when_used: "the interleaving fixture".to_owned(),
        why: "the interleaving fixture".to_owned(),
        words: Some(ProseTier::Slop(vec![
            "found ".to_owned(),
            " file".to_owned(),
            " across ".to_owned(),
            " source".to_owned(),
            ".".to_owned(),
        ])),
    }];
    assert_eq!(
        arrangement_sentence(&registry, "harness-sentence", None, &["3", "s", "1", ""]),
        "found 3 files across 1 source."
    );
    assert_eq!(
        arrangement_sentence(&registry, "harness-sentence", None, &["1", "", "2", "s"]),
        "found 1 file across 2 sources."
    );
}

/// The FACELESS why-lens remediation hints, previously a class-keyed `&'static str` match in
/// `aid/src/diag.rs`. No case owns them and no transcript renders them, so these literals are the
/// whole net; the `[tag]` suffix is part of the prose (`expected-why` needles substring-match it).
///
/// `why-remediation-resolve-dynamism` USED to be pinned here beside them and no longer is: it is
/// owned by `why-reason-cmdsub-opener.loom`, whose committed transcript renders it. Its bytes have a
/// home that re-blesses with a prose edit, and a second copy here would only refuse one.
///
/// These are not the pre-migration bytes either: `28G` Phase W1 respelled them twice, and the net
/// pins the respell rather than the freeze. A migration net freezes bytes against ACCIDENTAL drift;
/// it never outranks a ruling that the frozen bytes were wrong.
#[test]
fn the_faceless_remediation_hints_are_byte_identical() {
    assert_eq!(
        rendered("why-remediation-provide-model", &[]),
        "to skip it, an oracle must declare a read-only probe for this kind [provide-model]"
    );
    assert_eq!(
        rendered("why-remediation-declare-identity", &[]),
        "to skip it, add the missing kind/selector/Query declaration [declare-identity]"
    );
    assert_eq!(
        rendered("why-remediation-structural", &[]),
        "no user fix -- Dorc cannot model this construct [structural]"
    );
}

/// An arity slip against a REAL registry row is loud, and says which row and by how much. (The
/// release-build behaviour it replaces — degrading to `[unwritten: lint-summary-sentence]` — is
/// still what a shipped binary does; what changed is that no test run can pass through one
/// silently.)
#[test]
#[should_panic(expected = "arrangement `lint-summary-sentence`")]
fn a_seat_that_disagrees_with_its_entry_is_loud() {
    let _ = rendered("lint-summary-sentence", &["4"]);
}

/// Every typed REASON renders real words (`28L:rul-reason-enums-not-sibling-codes`).
///
/// The reason migrations replaced ~95 composed sentences with two hand-written tables — an
/// enum-to-slug map in `diag.rs` and the matching `ARRANGEMENTS` rows — and almost none of the
/// pairs is reached by a committed transcript. Two things can silently be wrong across a
/// hand-written pair, and BOTH degrade to the same greppable placeholder rather than to a crash: a
/// slug that names no row, and a word-run count that cannot serve the values its seat passes. This
/// walks every variant of every migrated reason and asserts the render is neither.
///
/// Completeness is NOT compiler-forced — adding a variant without listing it here compiles green.
/// The forcing that does exist is one tier down: the reason maps are exhaustive matches, so a new
/// variant stops the build at its own map, and `params_of_raw`'s exhaustive destructuring makes a
/// new payload field an error at the seat. What this list adds is the check no signature can make:
/// that the slug on the far side of the map is a row, and that the row fits its seat.
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one line PER VARIANT, like the fixture table it resembles — the length IS the \
              coverage, and splitting it by family would hide which reasons are unlisted"
)]
fn every_migrated_reason_renders_words_not_a_placeholder() {
    use dorc_aid::diag::{
        CfgInlineRefused, CfgInlineRefusedReason, CfgTopNode, CfgTopNodeReason, Diag, DiagCode,
        EscalationPolicy, FootprintIncoherent, FootprintIncoherentReason, PredictLexError,
        PredictOutOfDialect, PredictOutOfDialectReason, PredictUnterminated,
        PredictUnterminatedReason, RecordsHeaderMismatch, RecordsIntegrityRefused, SyntaxMalformed,
        SyntaxMalformedReason, SyntaxUnsupported, SyntaxUnsupportedReason, UnmodeledWriteRedirect,
        WhylogCorrupt, WhylogCorruptReason, render_body,
    };
    use dorc_core::{BytePos, Capability, EscalationDial, Interner, Span};

    let name = || "retry".to_owned();
    let mut codes: Vec<DiagCode> = Vec::new();

    for reason in [
        CfgTopNodeReason::UnsupportedConstruct,
        CfgTopNodeReason::NestingBound,
    ] {
        codes.push(DiagCode::CfgTopNode(CfgTopNode { reason }));
    }
    for reason in [
        CfgInlineRefusedReason::Redefined { name: name() },
        CfgInlineRefusedReason::RecursiveCall { name: name() },
        CfgInlineRefusedReason::DepthBudget {
            name: name(),
            budget: 2,
        },
        CfgInlineRefusedReason::UnmodeledPositional {
            name: name(),
            construct: "$@",
        },
        CfgInlineRefusedReason::WriteRedirect {
            name: name(),
            redirect: UnmodeledWriteRedirect::ToPath {
                path: "/etc/hosts".to_owned(),
            },
        },
        CfgInlineRefusedReason::WriteRedirect {
            name: name(),
            redirect: UnmodeledWriteRedirect::ToDynamicTarget,
        },
        CfgInlineRefusedReason::PerCallNodeBudget {
            name: name(),
            estimate: 80,
            budget: 64,
        },
        CfgInlineRefusedReason::PerBookNodeBudget {
            name: name(),
            spliced: 1000,
            estimate: 80,
            budget: 1024,
        },
    ] {
        codes.push(DiagCode::CfgInlineRefused(CfgInlineRefused { reason }));
    }
    for reason in [
        FootprintIncoherentReason::OmitsOwnCoordinate,
        FootprintIncoherentReason::MalformedDerivedCoordinate,
    ] {
        codes.push(DiagCode::FootprintIncoherent(FootprintIncoherent {
            reason,
        }));
    }
    for which in [
        RecordsHeaderMismatch::Nonce,
        RecordsHeaderMismatch::Attempt,
        RecordsHeaderMismatch::Host,
        RecordsHeaderMismatch::Book,
    ] {
        codes.push(DiagCode::RecordsIntegrityRefused(RecordsIntegrityRefused {
            which,
        }));
    }
    for reason in [
        WhylogCorruptReason::Headerless,
        WhylogCorruptReason::HeaderTagMissing,
        WhylogCorruptReason::ResultsBlockOverruns,
        WhylogCorruptReason::EndSentinelMissing,
    ] {
        codes.push(DiagCode::WhylogCorrupt(WhylogCorrupt { reason }));
    }
    for dial in [
        EscalationDial::NoEscalation,
        EscalationDial::VouchedOnly,
        EscalationDial::AnyProbe,
    ] {
        for capability in [
            Capability::Root,
            Capability::NonRootNopasswd,
            Capability::Degraded,
        ] {
            codes.push(DiagCode::EscalationPolicy(EscalationPolicy {
                dial,
                capability,
                entry_forms: "sudo -n".to_owned(),
            }));
        }
    }
    for reason in [
        SyntaxUnsupportedReason::ParserStalled,
        SyntaxUnsupportedReason::NestingBound,
        SyntaxUnsupportedReason::ReservedWordInCommandPosition,
        SyntaxUnsupportedReason::ConstructTrailingRedirection {
            construct: "if",
            closer: "fi",
        },
        SyntaxUnsupportedReason::ForWithoutVariableName,
        SyntaxUnsupportedReason::ForWithoutInList,
        SyntaxUnsupportedReason::ForListWordHasExpansion,
        SyntaxUnsupportedReason::ForListNotTerminated,
        SyntaxUnsupportedReason::LoopJumpInBody,
        SyntaxUnsupportedReason::LoopJumpInBodyOrCondition,
        SyntaxUnsupportedReason::BackgroundAmp,
        SyntaxUnsupportedReason::OperatorWithoutCommand,
        SyntaxUnsupportedReason::DoubleSemicolonOutsideCase,
        SyntaxUnsupportedReason::ExpectedACommand,
        SyntaxUnsupportedReason::ArithmeticAsCommand,
        SyntaxUnsupportedReason::DynamicCommandName,
        SyntaxUnsupportedReason::EvalConstructedCode,
        SyntaxUnsupportedReason::SourceOfDynamicTarget,
        SyntaxUnsupportedReason::UnsetDynamicLvalue,
        SyntaxUnsupportedReason::PrintfWritesLvalue,
        SyntaxUnsupportedReason::TestReferencesLvalue,
        SyntaxUnsupportedReason::ExpectedAWord,
    ] {
        codes.push(DiagCode::SyntaxUnsupported(SyntaxUnsupported { reason }));
    }
    for reason in [
        SyntaxMalformedReason::ExpectedThenAfterIf,
        SyntaxMalformedReason::ExpectedThenAfterElif,
        SyntaxMalformedReason::ExpectedFiToCloseIf,
        SyntaxMalformedReason::ExpectedInAfterCaseWord,
        SyntaxMalformedReason::ExpectedEsacToCloseCase,
        SyntaxMalformedReason::ExpectedDoToOpenLoopBody,
        SyntaxMalformedReason::ExpectedDoneToCloseLoop,
        SyntaxMalformedReason::UnterminatedCaseArm,
        SyntaxMalformedReason::ExpectedRparenAfterCasePattern,
        SyntaxMalformedReason::UnterminatedSubshell,
        SyntaxMalformedReason::UnterminatedBraceGroup,
    ] {
        codes.push(DiagCode::SyntaxMalformed(SyntaxMalformed { reason }));
    }
    for reason in [
        PredictOutOfDialectReason::MalformedFunctionHeader,
        PredictOutOfDialectReason::FunctionBodyMustStartWithBrace,
        PredictOutOfDialectReason::CheckBodyOutOfDialect,
        PredictOutOfDialectReason::AndOrListNotLedByCommand,
        PredictOutOfDialectReason::AndOrListItemNotCommand,
        PredictOutOfDialectReason::ExpectedDoAfterWhileTest,
        PredictOutOfDialectReason::ExpectedThenAfterIfTest,
        PredictOutOfDialectReason::ExpectedInAfterCaseScrutinee,
        PredictOutOfDialectReason::UnterminatedCaseExpectedEsac,
        PredictOutOfDialectReason::ExpectedPipeOrRparenInCaseArmPattern,
        PredictOutOfDialectReason::CasePatternOutOfDialect,
        PredictOutOfDialectReason::ExpectedCaseArmPattern,
        PredictOutOfDialectReason::ShiftCountNotLiteralInteger,
        PredictOutOfDialectReason::StatementDoesNotStartWithWord,
        PredictOutOfDialectReason::AnnotationNeedsValueWord,
        PredictOutOfDialectReason::OutOfDialectToken {
            lex: PredictLexError::UnmodeledByte,
        },
        PredictOutOfDialectReason::OutOfDialectToken {
            lex: PredictLexError::BacktickCommandSubstitution,
        },
        PredictOutOfDialectReason::OutOfDialectToken {
            lex: PredictLexError::UnterminatedQuote,
        },
        PredictOutOfDialectReason::UnexpectedTokenInCommand,
        PredictOutOfDialectReason::EmptyCommand,
        PredictOutOfDialectReason::ExpectedAWord,
        PredictOutOfDialectReason::ExpectedLbracketToOpenTest,
        PredictOutOfDialectReason::TestOperatorNotStringComparison,
        PredictOutOfDialectReason::ExpectedRbracketToCloseTest,
        PredictOutOfDialectReason::TrailingBindMarkWithValue,
        PredictOutOfDialectReason::MarkNeedsVerbOrCoordinate,
        PredictOutOfDialectReason::TrailingBindMarkWord,
        PredictOutOfDialectReason::MarkNeedsPayload,
        PredictOutOfDialectReason::MalformedMarkTarget,
        PredictOutOfDialectReason::SelectorNotPosixName,
    ] {
        codes.push(DiagCode::PredictOutOfDialect(PredictOutOfDialect {
            reason,
        }));
    }
    for reason in [
        PredictUnterminatedReason::FunctionBody,
        PredictUnterminatedReason::Block { keyword: "done" },
        PredictUnterminatedReason::CaseArm,
        PredictUnterminatedReason::IfThen,
    ] {
        codes.push(DiagCode::PredictUnterminated(PredictUnterminated {
            reason,
        }));
    }

    let interner = Interner::default();
    let span = Span::new(BytePos(0), BytePos(1));
    for code in codes {
        let slug = code.slug();
        let body = render_body(&Diag::new(code, span), &interner);
        assert!(
            !body.contains("[unwritten:"),
            "`{slug}`: a reason rendered the placeholder — its slug names no registry row, or its \
             row cannot serve the seat's value count: {body}"
        );
        assert!(
            body.trim() != "sm",
            "`{slug}`: a reason rendered no words at all: {body:?}"
        );
    }
}

/// Each `RecordsHeaderMismatch` variant renders ITS OWN registry row.
///
/// The census above proves only that no variant rendered the placeholder; four variants all
/// reaching ONE row would satisfy it, and that is exactly the failure a reason-enum invites (one
/// arm's slug pasted into the next). This asks the registry for each row's own sentence and asserts
/// the seat rendered THAT one — a relationship, never a byte pin, so authoring the prose moves
/// nothing here (`prose-pins-live-where-the-prose-does`).
///
/// Only the Host variant has a committed transcript — one payload world per case — so for the
/// other three this is the whole net.
#[test]
fn every_header_mismatch_renders_its_own_component() {
    use dorc_aid::diag::{
        Diag, DiagCode, RecordsHeaderMismatch, RecordsIntegrityRefused, render_body,
    };
    use dorc_core::Interner;

    let interner = Interner::default();
    for (which, slug) in [
        (
            RecordsHeaderMismatch::Nonce,
            "records-integrity-refused-nonce",
        ),
        (
            RecordsHeaderMismatch::Attempt,
            "records-integrity-refused-attempt",
        ),
        (
            RecordsHeaderMismatch::Host,
            "records-integrity-refused-host",
        ),
        (
            RecordsHeaderMismatch::Book,
            "records-integrity-refused-book",
        ),
    ] {
        let own = rendered(slug, &[]);
        assert!(
            !own.is_empty() && !own.contains("[unwritten:"),
            "`{slug}` has no words to render — seed or author the row first: {own:?}"
        );
        let body = render_body(
            &Diag::new_spanless_site(DiagCode::RecordsIntegrityRefused(RecordsIntegrityRefused {
                which,
            })),
            &interner,
        );
        assert!(
            body.contains(&own),
            "`{which:?}` did not render `{slug}`'s own sentence: {body:?}"
        );
    }
}
