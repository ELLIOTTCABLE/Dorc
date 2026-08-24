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

use dorc_receipt::image::{
    ApplyArtifactImage, ApplyEdge, ApplyEdgeKind, ApplyEntryBytes, ApplyEntryId, ApplyEntryKind,
    ApplyImageEntry, ApplyRoot, ApplyRootId, ApplyTopology, RecordedApplyPath,
    RecordedArtifactForm, RecordedMode,
};
use dorc_receipt::limits::ReceiptLimits;

fn path(text: &str) -> RecordedApplyPath {
    match RecordedApplyPath::of(text.as_bytes(), &ReceiptLimits::V1) {
        Ok(value) => value,
        Err(refusal) => panic!("{text:?} should be a legal recorded path: {refusal:?}"),
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
        file_entry(
            0,
            "plan.sh",
            RecordedMode::Octal(0o755),
            b"#!/bin/sh\n. ./a.sh\n",
        ),
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
    // The degenerate shape: one path-less entry that is its own root and its own entrypoint.
    // A stream has no name to materialize under, which is why its path is absent rather than
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
    // The whole remit in one assertion set: re-materializing reproduces the image, not a
    // cousin of it.
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
    assert_eq!(plan.mode(), RecordedMode::Octal(0o755));
    assert_eq!(plan.bytes().get(), b"#!/bin/sh\n. ./a.sh\n");
}

#[test]
fn an_entry_may_carry_any_byte_including_the_containers_own_terminator() {
    // The container is length-framed, not delimiter-scanned. Content that spells the framing
    // is the vector that tells the two apart: a scanner would end the image early here.
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
    // The container carries no identity of its own, so the value compared against comes from
    // the document that names it. Parsing has to recompute and compare, or the digest in the
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
