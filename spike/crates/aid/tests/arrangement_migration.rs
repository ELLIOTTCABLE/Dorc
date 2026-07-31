//! Byte-identity pins for the chrome migrated into the arrangement registry
//! (`289:rul-arrangement-home-is-registry-plus-transcripts`).
//!
//! The migration is a STORAGE move: every migrated line must render exactly the bytes its
//! hardcoded predecessor did. Most of the moved chrome is covered by the e2e/loom goldens
//! already, but three stderr lines — the plan-summary yardstick, the decision-digest, and the
//! why-pointer — are asserted by NOTHING in the corpus, so moving them would have been an
//! unwitnessed change. Each expectation below is the pre-migration literal, frozen.
//!
//! These are not render-form contracts (`27V:rul-output-form-unwelded` still holds): a
//! deliberate rewording updates the entry AND its pin together. What they catch is the accident —
//! a dropped separator, a lost space, a word-order slip in the ordered-word sequence.

use dorc_aid::arrangement::{CONST_ARRANGEMENTS, arrangement_sentence};

fn rendered(slug: &str, values: &[&str]) -> String {
    arrangement_sentence(&CONST_ARRANGEMENTS, slug, None, values)
}

/// `emit_plan_summary`'s yardstick line, previously one `format!` in `cli/src/main.rs`.
#[test]
fn the_plan_summary_line_is_byte_identical() {
    assert_eq!(
        rendered("cli-plan-summary-line", &["12", "5", "1", "3", "3", "0"]),
        "dorc: plan-summary sites=12 elide=5 omit=1 guard=3 run=3 may-alias=0"
    );
}

/// The decision-digest line that follows it.
#[test]
fn the_decision_digest_line_is_byte_identical() {
    assert_eq!(
        rendered("cli-decision-digest-line", &["deadbeef"]),
        "dorc: decision-digest deadbeef"
    );
}

/// The aggregate why-pointer a `dorc plan` preview closes with.
#[test]
fn the_why_pointer_line_is_byte_identical() {
    assert_eq!(
        rendered("cli-why-pointer-line", &["webhost.sh"]),
        "dorc: run `dorc why` for the per-site cause-chains, or `dorc why webhost.sh:N` to query a source line"
    );
}

/// The lint report's two sentences. Both ARE golden-covered, but pinning them here keeps the
/// value-interleaving contract (words = values + 1, in this order) readable in one place —
/// including the plural-suffix positions, which are the easiest thing to mis-order.
#[test]
fn the_lint_sentences_are_byte_identical() {
    assert_eq!(
        rendered("lint-clean-sentence", &["3", "s", "1", ""]),
        "dorc lint: clean -- nothing found across 3 files, 1 source."
    );
    assert_eq!(
        rendered(
            "lint-summary-sentence",
            &["4", "s", "0", "s", "1", "", "1", ""]
        ),
        "dorc lint: 4 errors, 0 warnings, 1 info across 1 file."
    );
}

/// The four why-lens remediation hints, previously a class-keyed `&'static str` match in
/// `aid/src/diag.rs`. Like the three stderr lines above they are faceless — no transcript renders
/// them — so these literals are the whole net; the `[tag]` suffix is part of the prose
/// (`expected-why` needles substring-match it).
///
/// No longer the pre-migration bytes: `28G` Phase W1 respelled them twice, and the net now pins the
/// respell rather than the freeze. `elide` was ENGINE vocabulary reaching a user surface (the
/// admin-English carve, `28E` §8), and the em-dash violated `rul-ascii-output-forever`
/// (`28E` §0, human-typed). A migration net freezes bytes against ACCIDENTAL drift; it never
/// outranks a ruling that the frozen bytes were wrong.
#[test]
fn the_remediation_hints_are_byte_identical() {
    assert_eq!(
        rendered("why-remediation-provide-model", &[]),
        "to skip it, an oracle must declare a read-only probe for this kind [provide-model]"
    );
    assert_eq!(
        rendered("why-remediation-declare-identity", &[]),
        "to skip it, add the missing kind/selector/Query declaration [declare-identity]"
    );
    assert_eq!(
        rendered("why-remediation-resolve-dynamism", &[]),
        "to skip it, make the operand a literal Dorc can resolve+probe [resolve-dynamism]"
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
        PredictUnterminatedReason, SyntaxMalformed, SyntaxMalformedReason, SyntaxUnsupported,
        SyntaxUnsupportedReason, UnmodeledWriteRedirect, WhylogCorrupt, WhylogCorruptReason,
        render_body,
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
        SyntaxUnsupportedReason::SourceOfNonLiteralTarget,
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
