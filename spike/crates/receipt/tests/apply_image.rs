#![expect(
    clippy::panic,
    reason = "clippy.toml's allow-panic-in-tests reaches `#[test]` functions in this crate but \
              not the plain fixture-building helpers beside them, and threading a Result through \
              fixtures that must succeed buys nothing"
)]
//! The `dorc-apply-artifact-image/1` container: what an apply actually uses, recorded exactly.
//!
//! The container may ENCODE an apply image and may never CHANGE it, so most of what follows is
//! one property said in many ways — every entry, path, mode, root, edge, entrypoint and byte
//! comes back as it went in.

use std::path::{Path, PathBuf};

use dorc_receipt::ids::ApplyArtifactImageId;
use dorc_receipt::image::{
    ApplyArtifactImage, ApplyEdge, ApplyEdgeKind, ApplyEntryBytes, ApplyEntryId, ApplyEntryKind,
    ApplyImageEntry, ApplyRoot, ApplyRootId, ApplyTopology, ImageRefusal, RecordedApplyPath,
    RecordedArtifactForm, RecordedMode,
};
use dorc_receipt::limits::{ByteLimit, CountLimit, ReceiptLimits};

fn path(text: &str) -> RecordedApplyPath {
    match RecordedApplyPath::of(text.as_bytes(), &ReceiptLimits::V1) {
        Ok(value) => value,
        Err(refusal) => panic!("{text:?} should be a legal recorded path: {refusal:?}"),
    }
}

fn mode(bits: u16) -> RecordedMode {
    match RecordedMode::octal(bits) {
        Some(value) => value,
        None => panic!("{bits:o} is past what a four-octal-digit field can spell"),
    }
}

fn file_entry(id: u32, at: &str, mode: RecordedMode, bytes: &[u8]) -> ApplyImageEntry {
    ApplyImageEntry::file(
        ApplyEntryId::of(id),
        path(at),
        mode,
        ApplyEntryBytes::of(bytes.to_vec()),
    )
}

/// A three-file multipart image: one plan and two bundles it loads.
fn multipart() -> ApplyArtifactImage {
    let entries = vec![
        file_entry(0, "plan.sh", mode(0o755), b"#!/bin/sh\n. ./a.sh\n"),
        file_entry(
            1,
            "lib/a.dorc-bundle.sh",
            RecordedMode::Unused,
            b"a() { :; }\n",
        ),
        file_entry(
            2,
            "lib/b.dorc-bundle.sh",
            RecordedMode::Unused,
            b"b() { :; }\n",
        ),
    ];
    let roots = vec![
        ApplyRoot::of(ApplyRootId::of(0), ApplyEntryId::of(1)),
        ApplyRoot::of(ApplyRootId::of(1), ApplyEntryId::of(2)),
    ];
    let topology = ApplyTopology::of(vec![
        ApplyEdge::of(
            ApplyEntryId::of(0),
            ApplyEntryId::of(1),
            ApplyEdgeKind::Loads,
        ),
        ApplyEdge::of(
            ApplyEntryId::of(1),
            ApplyEntryId::of(2),
            ApplyEdgeKind::Loads,
        ),
    ]);
    match ApplyArtifactImage::of_parts(
        RecordedArtifactForm::Multipart,
        entries,
        roots,
        vec![ApplyEntryId::of(0)],
        topology,
        &ReceiptLimits::V1,
    ) {
        Ok(image) => image,
        Err(refusal) => panic!("the multipart fixture should build: {refusal:?}"),
    }
}

#[test]
fn a_single_external_stream_round_trips_exactly() {
    // The degenerate shape: one path-less entry that is its own root and its own entrypoint. A
    // stream has no name to materialize under, which is why its path is absent rather than
    // fabricated.
    let source = b"#!/bin/sh\nset -eu\nufw allow 443/tcp\n".to_vec();
    let image = match ApplyArtifactImage::of_external_stream(
        ApplyEntryBytes::of(source.clone()),
        &ReceiptLimits::V1,
    ) {
        Ok(image) => image,
        Err(refusal) => panic!("a single stream should build: {refusal:?}"),
    };

    let encoded = image.encode().to_vec();
    let back = match ApplyArtifactImage::parse(&encoded, image.id(), &ReceiptLimits::V1) {
        Ok(back) => back,
        Err(refusal) => panic!("its own encoding should parse: {refusal:?}"),
    };

    assert_eq!(back.id(), image.id());
    assert_eq!(back.encode(), encoded.as_slice());
    assert_eq!(back.form(), RecordedArtifactForm::ExternalStream);
    assert_eq!(back.entries().len(), 1);
    let Some(entry) = back.entries().first() else {
        panic!("one entry");
    };
    assert_eq!(entry.kind(), ApplyEntryKind::Stream);
    assert!(entry.path().is_none(), "a stream has no path");
    assert_eq!(entry.mode(), RecordedMode::Unused);
    assert_eq!(entry.bytes().get(), source.as_slice());
    assert_eq!(back.entrypoints(), &[ApplyEntryId::of(0)]);
    assert_eq!(back.roots().len(), 1);
    assert!(back.topology().edges().is_empty());
}

#[test]
fn a_multi_file_image_round_trips_every_entry_path_mode_root_edge_and_byte() {
    // The whole remit in one assertion set: re-materializing reproduces the image, not a cousin
    // of it.
    let image = multipart();
    let encoded = image.encode().to_vec();
    let back = match ApplyArtifactImage::parse(&encoded, image.id(), &ReceiptLimits::V1) {
        Ok(back) => back,
        Err(refusal) => panic!("its own encoding should parse: {refusal:?}"),
    };

    assert_eq!(back.id(), image.id());
    assert_eq!(back.encode(), encoded.as_slice());
    assert_eq!(back.form(), image.form());
    assert_eq!(back.entrypoints(), image.entrypoints());
    assert_eq!(back.roots(), image.roots());
    assert_eq!(back.topology().edges(), image.topology().edges());
    assert_eq!(back.entries(), image.entries());

    // Spelled out once, so a change that made `PartialEq` shallow could not hide here.
    let Some(plan) = back.entries().first() else {
        panic!("three entries");
    };
    assert_eq!(plan.path().map(RecordedApplyPath::text), Some("plan.sh"));
    assert_eq!(plan.mode(), mode(0o755));
    assert_eq!(plan.bytes().get(), b"#!/bin/sh\n. ./a.sh\n");
}

#[test]
fn an_entry_may_carry_any_byte_including_the_containers_own_terminator() {
    // The container is length-framed, not delimiter-scanned. Content that spells the framing is
    // the vector that tells the two apart: a scanner would end the image early here.
    let hostile: Vec<u8> = [
        b"binary: \x00\xff\x7f\r not text\n".as_slice(),
        b"image-end\n".as_slice(),
        b"entry 9 file 0644 3 3\n".as_slice(),
        b"no trailing newline",
    ]
    .concat();
    let entries = vec![file_entry(0, "payload.bin", RecordedMode::Unused, &hostile)];
    let image = match ApplyArtifactImage::of_parts(
        RecordedArtifactForm::MirroredTree,
        entries,
        vec![ApplyRoot::of(ApplyRootId::of(0), ApplyEntryId::of(0))],
        vec![ApplyEntryId::of(0)],
        ApplyTopology::of(Vec::new()),
        &ReceiptLimits::V1,
    ) {
        Ok(image) => image,
        Err(refusal) => panic!("arbitrary content bytes are legal: {refusal:?}"),
    };
    let encoded = image.encode().to_vec();
    let back = match ApplyArtifactImage::parse(&encoded, image.id(), &ReceiptLimits::V1) {
        Ok(back) => back,
        Err(refusal) => panic!("length framing should survive its own terminator: {refusal:?}"),
    };
    let Some(entry) = back.entries().first() else {
        panic!("one entry");
    };
    assert_eq!(entry.bytes().get(), hostile.as_slice());
}

#[test]
fn a_recomputed_identity_that_disagrees_refuses_before_an_image_exists() {
    // The container carries no identity of its own, so the value compared against comes from the
    // document that names it. Parsing has to recompute and compare, or the digest in the
    // skeleton would be a claim nothing checked.
    let image = multipart();
    let other = ApplyArtifactImage::of_external_stream(
        ApplyEntryBytes::of(b"different".to_vec()),
        &ReceiptLimits::V1,
    );
    let Ok(other) = other else {
        panic!("the second fixture should build");
    };
    assert_ne!(image.id(), other.id());
    assert!(
        ApplyArtifactImage::parse(image.encode(), other.id(), &ReceiptLimits::V1).is_err(),
        "an image whose identity is not the one asked for is not that image"
    );
}

#[test]
fn two_images_differing_only_in_one_content_byte_have_different_identities() {
    let one = ApplyArtifactImage::of_external_stream(
        ApplyEntryBytes::of(b"payload".to_vec()),
        &ReceiptLimits::V1,
    );
    let two = ApplyArtifactImage::of_external_stream(
        ApplyEntryBytes::of(b"payloae".to_vec()),
        &ReceiptLimits::V1,
    );
    match (one, two) {
        (Ok(one), Ok(two)) => assert_ne!(one.id(), two.id()),
        _ => panic!("both fixtures should build"),
    }
}

// ---- the committed corpus ------------------------------------------------------------------

fn vectors(kind: &str) -> Vec<(String, Vec<u8>)> {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("vectors")
        .join("image")
        .join(kind);
    let mut out: Vec<(String, Vec<u8>)> = std::fs::read_dir(&root)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|item| {
            let at = item.path();
            Some((
                at.file_name()?.to_str()?.to_owned(),
                std::fs::read(&at).ok()?,
            ))
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    // A corpus walk that found nothing would otherwise pass in silence.
    assert!(
        !out.is_empty(),
        "no {kind} vectors under {}",
        root.display()
    );
    out
}

/// Rebuild an image from its own parts through the public constructors.
///
/// This is what proves the ENCODER and the PARSER agree. `encode()` hands back the bytes an
/// image was minted or read with, so comparing a parsed image's bytes to the file they came from
/// is vacuous; re-minting runs the encoder again from the parsed model, and the bytes have to
/// land in the same place.
fn remint(image: &ApplyArtifactImage) -> Result<ApplyArtifactImage, ImageRefusal> {
    if image.form() == RecordedArtifactForm::ExternalStream {
        let Some(entry) = image.entries().first() else {
            panic!("an external stream has one entry");
        };
        return ApplyArtifactImage::of_external_stream(
            ApplyEntryBytes::of(entry.bytes().get().to_vec()),
            &ReceiptLimits::V1,
        );
    }
    ApplyArtifactImage::of_parts(
        image.form(),
        image.entries().to_vec(),
        image.roots().to_vec(),
        image.entrypoints().to_vec(),
        ApplyTopology::of(image.topology().edges().to_vec()),
        &ReceiptLimits::V1,
    )
}

#[test]
fn every_valid_vector_parses_and_re_encodes_to_the_same_bytes() {
    let limits = ReceiptLimits::V1;
    let mut failures: Vec<String> = Vec::new();
    for (name, bytes) in vectors("valid") {
        let expected = ApplyArtifactImageId::of_canonical_image(&bytes);
        let parsed = match ApplyArtifactImage::parse(&bytes, expected, &limits) {
            Ok(parsed) => parsed,
            Err(refusal) => {
                failures.push(format!("{name}: refused {refusal:?}"));
                continue;
            }
        };
        if parsed.encode() != bytes.as_slice() {
            failures.push(format!("{name}: did not hold its own bytes"));
            continue;
        }
        match remint(&parsed) {
            Ok(again) if again.encode() == bytes.as_slice() && again.id() == parsed.id() => {}
            Ok(_) => failures.push(format!("{name}: re-encoding drifted")),
            Err(refusal) => failures.push(format!("{name}: would not re-mint: {refusal:?}")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn every_invalid_vector_is_refused_for_its_own_departure() {
    // Each vector is handed the identity of its own bytes, so a refusal can never be the
    // identity check standing in for the departure the vector is about.
    let limits = ReceiptLimits::V1;
    let mut wrong: Vec<String> = Vec::new();
    for (name, bytes) in vectors("invalid") {
        let expected = ApplyArtifactImageId::of_canonical_image(&bytes);
        match ApplyArtifactImage::parse(&bytes, expected, &limits) {
            Ok(_) => wrong.push(format!("{name}: accepted")),
            Err(ImageRefusal::IdentityMismatch) => {
                wrong.push(format!("{name}: parsed whole; only the identity refused"));
            }
            Err(_) => {}
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn the_corpus_covers_every_form_and_both_entry_kinds() {
    // A corpus that quietly lost a form would keep passing, so the shapes are named here.
    let mut forms: Vec<&'static str> = Vec::new();
    let mut kinds: Vec<&'static str> = Vec::new();
    for (_, bytes) in vectors("valid") {
        let expected = ApplyArtifactImageId::of_canonical_image(&bytes);
        let Ok(image) = ApplyArtifactImage::parse(&bytes, expected, &ReceiptLimits::V1) else {
            continue;
        };
        forms.push(image.form().token());
        for entry in image.entries() {
            kinds.push(entry.kind().token());
        }
    }
    for form in ["external-stream", "multipart", "mirrored-tree", "flattened"] {
        assert!(forms.contains(&form), "no {form} vector");
    }
    for kind in ["stream", "file"] {
        assert!(kinds.contains(&kind), "no {kind} entry in the corpus");
    }
}

// ---- topology ------------------------------------------------------------------------------

/// A chain of `count` entries, each loading the next.
fn chain(count: u32, limits: &ReceiptLimits) -> Result<ApplyArtifactImage, ImageRefusal> {
    let mut entries = Vec::new();
    let mut edges = Vec::new();
    for index in 0..count {
        entries.push(file_entry(
            index,
            &format!("step{index}.sh"),
            RecordedMode::Unused,
            b"x\n",
        ));
        if index.saturating_add(1) < count {
            edges.push(ApplyEdge::of(
                ApplyEntryId::of(index),
                ApplyEntryId::of(index.saturating_add(1)),
                ApplyEdgeKind::Loads,
            ));
        }
    }
    ApplyArtifactImage::of_parts(
        RecordedArtifactForm::Multipart,
        entries,
        vec![ApplyRoot::of(ApplyRootId::of(0), ApplyEntryId::of(0))],
        vec![ApplyEntryId::of(0)],
        ApplyTopology::of(edges),
        limits,
    )
}

#[test]
fn the_depth_bound_refuses_at_boundary_plus_one() {
    // Boundary-minus / at / plus on the one bound a load chain can drive.
    let mut limits = ReceiptLimits::V1;
    limits.topology_depth = CountLimit::of(3);
    assert!(chain(2, &limits).is_ok(), "under the bound");
    assert!(chain(3, &limits).is_ok(), "at the bound");
    assert!(
        matches!(chain(4, &limits), Err(ImageRefusal::TopologyDepth)),
        "past the bound"
    );
}

#[test]
fn a_cycle_is_recorded_rather_than_refused() {
    // The container reports what an apply uses; it does not adjudicate whether the book is
    // sensible. A mutually-loading pair is legal sh, so refusing it here would refuse an apply
    // before its intent was ever published.
    let entries = vec![
        file_entry(0, "plan.sh", mode(0o755), b"x\n"),
        file_entry(1, "a.sh", RecordedMode::Unused, b"y\n"),
    ];
    let edges = vec![
        ApplyEdge::of(
            ApplyEntryId::of(0),
            ApplyEntryId::of(1),
            ApplyEdgeKind::Loads,
        ),
        ApplyEdge::of(
            ApplyEntryId::of(1),
            ApplyEntryId::of(0),
            ApplyEdgeKind::Loads,
        ),
    ];
    let image = ApplyArtifactImage::of_parts(
        RecordedArtifactForm::Multipart,
        entries,
        vec![ApplyRoot::of(ApplyRootId::of(0), ApplyEntryId::of(0))],
        vec![ApplyEntryId::of(0)],
        ApplyTopology::of(edges),
        &ReceiptLimits::V1,
    );
    let Ok(image) = image else {
        panic!("a cycle must not refuse: {image:?}");
    };
    assert_eq!(image.topology().edges().len(), 2, "both edges are kept");
}

#[test]
fn edges_reach_canonical_order_however_they_were_supplied() {
    // Edge order carries no information — an edge is a fact about a pair — so the mint sorts
    // rather than refusing, and two orderings of one set encode identically.
    let build = |edges: Vec<ApplyEdge>| {
        ApplyArtifactImage::of_parts(
            RecordedArtifactForm::Multipart,
            vec![
                file_entry(0, "plan.sh", mode(0o755), b"x\n"),
                file_entry(1, "a.sh", RecordedMode::Unused, b"y\n"),
                file_entry(2, "b.sh", RecordedMode::Unused, b"z\n"),
            ],
            vec![ApplyRoot::of(ApplyRootId::of(0), ApplyEntryId::of(0))],
            vec![ApplyEntryId::of(0)],
            ApplyTopology::of(edges),
            &ReceiptLimits::V1,
        )
    };
    let first = ApplyEdge::of(
        ApplyEntryId::of(0),
        ApplyEntryId::of(1),
        ApplyEdgeKind::Loads,
    );
    let second = ApplyEdge::of(
        ApplyEntryId::of(0),
        ApplyEntryId::of(2),
        ApplyEdgeKind::Loads,
    );
    match (build(vec![first, second]), build(vec![second, first])) {
        (Ok(one), Ok(two)) => {
            assert_eq!(one.id(), two.id());
            assert_eq!(one.encode(), two.encode());
        }
        other => panic!("both orderings should build: {other:?}"),
    }
}

// ---- bounds and shape ----------------------------------------------------------------------

#[test]
fn each_aggregate_bound_refuses_before_anything_is_read() {
    let build = |limits: &ReceiptLimits| {
        ApplyArtifactImage::of_parts(
            RecordedArtifactForm::Multipart,
            vec![
                file_entry(0, "plan.sh", mode(0o755), b"xx\n"),
                file_entry(1, "a.sh", RecordedMode::Unused, b"yy\n"),
            ],
            vec![ApplyRoot::of(ApplyRootId::of(0), ApplyEntryId::of(0))],
            vec![ApplyEntryId::of(0)],
            ApplyTopology::of(vec![ApplyEdge::of(
                ApplyEntryId::of(0),
                ApplyEntryId::of(1),
                ApplyEdgeKind::Loads,
            )]),
            limits,
        )
    };
    assert!(build(&ReceiptLimits::V1).is_ok());

    let mut entries_bound = ReceiptLimits::V1;
    entries_bound.image_entries = CountLimit::of(1);
    assert!(matches!(
        build(&entries_bound),
        Err(ImageRefusal::OverBound {
            what: "image-entries"
        })
    ));

    let mut per_entry = ReceiptLimits::V1;
    per_entry.image_entry_bytes = ByteLimit::of(1);
    assert!(matches!(
        build(&per_entry),
        Err(ImageRefusal::OverBound {
            what: "image-entry-bytes"
        })
    ));

    let mut aggregate = ReceiptLimits::V1;
    aggregate.image_bytes = ByteLimit::of(32);
    assert!(matches!(
        build(&aggregate),
        Err(ImageRefusal::OverBound {
            what: "image-bytes"
        })
    ));

    let mut edge_bound = ReceiptLimits::V1;
    edge_bound.topology_edges = CountLimit::of(0);
    assert!(matches!(
        build(&edge_bound),
        Err(ImageRefusal::OverBound {
            what: "topology-edges"
        })
    ));
}

#[test]
fn the_external_stream_form_is_reachable_only_through_its_own_constructor() {
    // The rigid single-stream shape cannot be assembled by hand, so nothing can hand-build an
    // image that says Dorc did not emit its bytes while carrying a tree that says otherwise.
    let refused = ApplyArtifactImage::of_parts(
        RecordedArtifactForm::ExternalStream,
        vec![ApplyImageEntry::stream(
            ApplyEntryId::of(0),
            ApplyEntryBytes::of(b"x\n".to_vec()),
        )],
        vec![ApplyRoot::of(ApplyRootId::of(0), ApplyEntryId::of(0))],
        vec![ApplyEntryId::of(0)],
        ApplyTopology::of(Vec::new()),
        &ReceiptLimits::V1,
    );
    assert!(matches!(refused, Err(ImageRefusal::EntryShape { .. })));
}

#[test]
fn several_roots_may_land_in_one_entry() {
    // What a flattened artifact is: the authored roots were folded into one stream, and the
    // image records that rather than inventing one root per file.
    let image = ApplyArtifactImage::of_parts(
        RecordedArtifactForm::Flattened,
        vec![file_entry(0, "plan.sh", mode(0o755), b"#!/bin/sh\n")],
        vec![
            ApplyRoot::of(ApplyRootId::of(0), ApplyEntryId::of(0)),
            ApplyRoot::of(ApplyRootId::of(1), ApplyEntryId::of(0)),
        ],
        vec![ApplyEntryId::of(0)],
        ApplyTopology::of(Vec::new()),
        &ReceiptLimits::V1,
    );
    let Ok(image) = image else {
        panic!("many roots to one entry is the flattened shape: {image:?}");
    };
    assert_eq!(image.roots().len(), 2);
    assert_eq!(image.entries().len(), 1);
}

/// One stable word per refusal, so a vector's expected answer can be written down.
fn label(refusal: &ImageRefusal) -> String {
    match refusal {
        ImageRefusal::OverBound { what } => format!("over-bound:{what}"),
        ImageRefusal::Structure { what } => format!("structure:{what}"),
        ImageRefusal::Empty { what } => format!("empty:{what}"),
        ImageRefusal::Identity { what } => format!("identity:{what}"),
        ImageRefusal::DuplicatePath => "duplicate-path".to_owned(),
        ImageRefusal::PathContainsPath => "path-contains-path".to_owned(),
        ImageRefusal::Path(inner) => format!("path:{inner:?}"),
        ImageRefusal::EntryShape { what } => format!("entry-shape:{what}"),
        ImageRefusal::SecondStream => "second-stream".to_owned(),
        ImageRefusal::Dangling { what } => format!("dangling:{what}"),
        ImageRefusal::UnaccountedEntry => "unaccounted-entry".to_owned(),
        ImageRefusal::EdgeOrder => "edge-order".to_owned(),
        ImageRefusal::TopologyDepth => "topology-depth".to_owned(),
        ImageRefusal::UnknownToken { what } => format!("unknown-token:{what}"),
        ImageRefusal::LengthMismatch => "length-mismatch".to_owned(),
        ImageRefusal::IdentityMismatch => "identity-mismatch".to_owned(),
        ImageRefusal::TrailingBytes => "trailing-bytes".to_owned(),
    }
}

/// Which condition each invalid vector exists to exercise.
///
/// Without this, a vector that stopped reaching its own departure would still be refused — by
/// something upstream — and the corpus would keep passing while testing nothing. Four vectors
/// were in exactly that state when this table was first written: each had lost the framing
/// newline after its content block, so all four refused at framing rather than at the path,
/// identity, or entry-shape condition their names claim.
const EXPECTED: &[(&str, &str)] = &[
    ("bytes-after-image-end", "trailing-bytes"),
    ("carriage-return-in-header", "unknown-token:form"),
    ("content-length-overdeclared", "structure:content-framing"),
    ("content-length-past-end", "length-mismatch"),
    ("content-length-underdeclared", "structure:content-framing"),
    ("count-leading-zero", "structure:entrypoints"),
    ("dangling-edge-child", "dangling:edge"),
    ("dangling-entrypoint", "dangling:entrypoint"),
    ("dangling-root-target", "dangling:root"),
    ("duplicate-edge", "edge-order"),
    ("duplicate-entrypoint", "identity:entrypoint"),
    ("edges-out-of-order", "edge-order"),
    ("entry-id-noncontiguous", "identity:entry"),
    (
        "external-stream-is-a-file",
        "entry-shape:external-stream-shape",
    ),
    (
        "external-stream-with-edges",
        "entry-shape:external-stream-shape",
    ),
    ("file-without-path", "entry-shape:file-path"),
    ("missing-content-framing", "structure:content-framing"),
    ("missing-path-framing", "structure:path-framing"),
    ("mode-absent-token", "unknown-token:mode"),
    ("mode-five-digits", "unknown-token:mode"),
    ("mode-non-octal", "unknown-token:mode"),
    ("mode-prefixed", "unknown-token:mode"),
    ("mode-three-digits", "unknown-token:mode"),
    ("no-image-end", "structure:image-end"),
    ("path-backslash", "path:IllegalByte { byte: 92 }"),
    ("path-case-collision", "duplicate-path"),
    ("path-colon", "path:IllegalByte { byte: 58 }"),
    ("path-contains-path", "path-contains-path"),
    ("path-device-stem", "path:DeviceStem"),
    ("path-device-stem-bare", "path:DeviceStem"),
    ("path-dot-component", "path:DotComponent"),
    ("path-empty-component", "path:EmptyComponent"),
    ("path-exact-duplicate", "duplicate-path"),
    ("path-leading-slash", "path:LeadingSeparator"),
    ("path-length-past-end", "length-mismatch"),
    ("path-non-ascii", "path:IllegalByte { byte: 195 }"),
    ("path-nul-byte", "path:IllegalByte { byte: 0 }"),
    ("path-question-mark", "path:IllegalByte { byte: 63 }"),
    ("path-trailing-dot", "path:ComponentTrailingDot"),
    ("path-trailing-slash", "path:TrailingSeparator"),
    ("path-trailing-space", "path:ComponentTrailingSpace"),
    ("path-traversal", "path:DotDotComponent"),
    ("root-id-noncontiguous", "identity:root"),
    ("stream-with-mode", "entry-shape:stream-mode"),
    ("stream-with-path", "entry-shape:stream-path"),
    ("two-streams", "second-stream"),
    ("unaccounted-entry", "unaccounted-entry"),
    ("unknown-edge-kind", "unknown-token:edge"),
    ("unknown-form", "unknown-token:form"),
    ("unsupported-version", "structure:version"),
    ("zero-entries", "empty:entries"),
    ("zero-entrypoints", "empty:entrypoints"),
    ("zero-roots", "empty:roots"),
];

#[test]
fn each_invalid_vector_refuses_at_the_condition_it_is_named_for() {
    let mut wrong: Vec<String> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    for (file, bytes) in vectors("invalid") {
        let name = file.strip_suffix(".applyimage").unwrap_or(&file).to_owned();
        let expected = ApplyArtifactImageId::of_canonical_image(&bytes);
        let got = match ApplyArtifactImage::parse(&bytes, expected, &ReceiptLimits::V1) {
            Ok(_) => "ACCEPTED".to_owned(),
            Err(refusal) => label(&refusal),
        };
        match EXPECTED.iter().find(|(known, _)| *known == name) {
            Some((_, want)) if *want == got => {}
            Some((_, want)) => wrong.push(format!("{name}: wanted {want}, got {got}")),
            None => wrong.push(format!("{name}: not in the table (got {got})")),
        }
        seen.push(name);
    }
    for (known, _) in EXPECTED {
        assert!(
            seen.iter().any(|name| name == known),
            "the table names {known}, which the corpus no longer carries"
        );
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
