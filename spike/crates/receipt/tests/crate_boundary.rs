//! The dependency-edge fence: which crates may name which, read off the manifests.
//!
//! No type can express "this crate has no dependency", so this is a lexical gate over the
//! workspace's own manifests. It walks a non-empty set and names the crates it found, so a
//! walk that silently found nothing fails rather than passing.

use std::path::{Path, PathBuf};

fn crates_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map_or_else(PathBuf::new, Path::to_path_buf)
}

/// The crates permitted to name the implementation crate, which carries the randomness.
///
/// Checked BOTH ways: a crate outside this list naming it fails, and an entry that stops
/// naming it fails too. An anticipatory entry is the failure mode the second direction
/// exists for — it reads as a fence while permitting a crate nothing has yet checked.
///
/// `cli` names it DEV-ONLY: the write route's orchestration takes injected capabilities and links
/// no implementation, so the shipped binary cannot sign. The list does not record that distinction,
/// and must not — it answers which crates may reach the randomness at all, and a dev target reaches
/// it exactly as a production one would. The entry stays put when the edge becomes ordinary.
const MAY_NAME_IT: [&str; 1] = ["cli"];

fn manifests() -> Vec<(String, String)> {
    let root = crates_root();
    let mut found: Vec<(String, String)> = std::fs::read_dir(&root)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let manifest = entry.path().join("Cargo.toml");
            let name = entry.path().file_name()?.to_str()?.to_owned();
            let text = std::fs::read_to_string(&manifest).ok()?;
            Some((name, text))
        })
        .collect();
    found.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(
        found.len() > 5,
        "the manifest walk found {} crates; it is looking in the wrong place",
        found.len()
    );
    found
}

/// The dependency lines of one manifest, ignoring comment text so a rule that is merely
/// described does not read as a rule that is declared.
fn dependency_lines(manifest: &str) -> Vec<&str> {
    manifest
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .collect()
}

#[test]
fn the_pure_receipt_crate_names_no_other_workspace_crate() {
    // Its whole reason for existing is that the kernel may depend on it, which holds only
    // while it depends on nothing of ours and nothing nondeterministic.
    let manifest =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .unwrap_or_default();
    for line in dependency_lines(&manifest) {
        assert!(
            !line.starts_with("dorc-")
                && !line.starts_with("weft")
                && !line.starts_with("errorloom"),
            "the pure receipt crate named a workspace crate: {line}"
        );
    }
    for forbidden in ["age", "ed25519", "rand", "getrandom"] {
        assert!(
            !dependency_lines(&manifest)
                .iter()
                .any(|line| line.starts_with(forbidden)),
            "the pure receipt crate named {forbidden}"
        );
    }
    assert!(
        dependency_lines(&manifest)
            .iter()
            .any(|line| line.starts_with("sha2")),
        "the one dependency it does carry went missing"
    );
}

/// Whether `text` reaches into `krate`'s namespace, matching the crate name as a WHOLE
/// identifier.
///
/// A bare `contains("age::")` also matches every type whose name merely ends in those letters —
/// `ApplyArtifactImage::`, `Message::`, `Storage::` — so the fence would refuse ordinary code
/// while still catching nothing it does not catch here. A fence that fires on a name is a fence
/// people route around.
fn names_crate(text: &str, krate: &str) -> bool {
    let needle = format!("{krate}::");
    let mut from = 0_usize;
    while let Some(offset) = text.get(from..).and_then(|rest| rest.find(&needle)) {
        let at = from.saturating_add(offset);
        let preceded = text
            .get(..at)
            .and_then(|before| before.bytes().next_back())
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || byte == b'_');
        if !preceded {
            return true;
        }
        from = at.saturating_add(needle.len());
    }
    false
}

#[test]
fn the_namespace_fence_matches_a_crate_and_not_a_name_that_merely_ends_in_one() {
    // The fence's own regression: it exists to catch a reach into `age`, and a type called
    // `ApplyArtifactImage` is not one.
    assert!(names_crate("use age::Encryptor;", "age"));
    assert!(names_crate("let x = age::armor();", "age"));
    assert!(names_crate("age::Foo", "age"));
    assert!(!names_crate("ApplyArtifactImage::of_parts(...)", "age"));
    assert!(!names_crate("Message::new()", "age"));
    assert!(!names_crate("crate::storage::of()", "age"));
    assert!(names_crate("(ed25519_dalek::Signature)", "ed25519_dalek"));
}

#[test]
fn no_source_file_of_the_pure_crate_names_a_crypto_package() {
    // The manifest fence and this one answer different questions: one is what the crate may
    // link, the other is what its code reaches for.
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut checked = 0_u32;
    for entry in std::fs::read_dir(&src).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        for token in ["dorc_receipt_crypto", "ed25519_dalek", "age"] {
            assert!(
                !names_crate(&text, token),
                "{} reaches for {token}",
                path.display()
            );
        }
        checked = checked.saturating_add(1);
    }
    assert!(checked > 5, "the source walk checked only {checked} files");
}

#[test]
fn only_the_edge_may_name_the_crypto_implementation_crate() {
    // The implementation crate carries the randomness, so the set of crates allowed to name
    // it is an allow-list rather than a convention. Adding an entry here is the visible act.
    let mut named_by: Vec<String> = Vec::new();
    for (crate_dir, manifest) in manifests() {
        if dependency_lines(&manifest)
            .iter()
            .any(|line| line.starts_with("dorc-receipt-crypto"))
        {
            named_by.push(crate_dir);
        }
    }
    for name in &named_by {
        assert!(
            MAY_NAME_IT.contains(&name.as_str()),
            "{name} names the crypto implementation crate; only {MAY_NAME_IT:?} may"
        );
    }
}

#[test]
fn the_crypto_crate_names_the_pure_crate_and_no_other_workspace_crate() {
    let manifest = crates_root().join("receipt-crypto").join("Cargo.toml");
    let text = std::fs::read_to_string(&manifest).unwrap_or_default();
    let lines = dependency_lines(&text);
    assert!(
        lines.iter().any(|line| line.starts_with("dorc-receipt ")),
        "the crypto crate must name the pure crate"
    );
    for line in lines {
        if line.starts_with("dorc-") {
            assert!(
                line.starts_with("dorc-receipt "),
                "the crypto crate named another workspace crate: {line}"
            );
        }
    }
}

#[test]
fn the_armor_feature_is_named_explicitly() {
    // Canonical armor is not a default feature of that package, and the grammar requires it,
    // so a manifest that stopped naming it would fail at seal time rather than here.
    let manifest = crates_root().join("receipt-crypto").join("Cargo.toml");
    let text = std::fs::read_to_string(&manifest).unwrap_or_default();
    let age_line = dependency_lines(&text)
        .into_iter()
        .find(|line| line.starts_with("age "))
        .unwrap_or_default();
    assert!(
        age_line.contains("\"armor\""),
        "armor must be named: {age_line}"
    );
    assert!(
        age_line.contains("default-features = false"),
        "the unused surfaces stay off: {age_line}"
    );
    for surface in ["plugin", "ssh", "async", "cli-common", "unstable"] {
        assert!(
            !age_line.contains(&format!("\"{surface}\"")),
            "{surface} must stay off: {age_line}"
        );
    }
}

/// The files permitted to carry fixture key material, by crate-relative path.
///
/// Narrow to the file, not the crate, and two-way: an entry naming a file that no longer
/// carries any is as much a failure as material in a file with no entry. Adding a row is the
/// visible act.
const MAY_CARRY_FIXTURE_KEY_MATERIAL: [&str; 2] = [
    "cli/tests/receipt_route.rs",
    "receipt-crypto/tests/crypto_interop.rs",
];

/// Every `.rs` file under the workspace's crates, as crate-relative paths.
fn workspace_sources() -> Vec<(String, String)> {
    fn walk(dir: &Path, root: &Path, out: &mut Vec<(String, String)>) {
        for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, root, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                let relative = path
                    .strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/");
                out.push((relative, std::fs::read_to_string(&path).unwrap_or_default()));
            }
        }
    }
    let root = crates_root();
    let mut out = Vec::new();
    walk(&root, &root, &mut out);
    out
}

#[test]
fn the_fixture_identity_is_unreachable_from_production() {
    // A private key committed to seal frozen vectors has exactly one legitimate home. No type
    // can say "this constant is not reachable from a shipped path", so the fence is lexical:
    // find every file carrying key material and require it to be one of the listed test files.
    // The walk asserts it found files, because a walk that matched nothing would pass while
    // proving nothing — the same failure shape as a corpus that loads zero vectors.
    let sources = workspace_sources();
    assert!(
        sources.len() > 20,
        "the source walk found only {} files; it is looking in the wrong place",
        sources.len()
    );

    let carriers: Vec<String> = sources
        .iter()
        // Spelled in halves so this file does not match its own search and report itself.
        .filter(|(_, text)| text.contains(concat!("AGE-SECRET", "-KEY-")))
        .map(|(path, _)| path.clone())
        .collect();

    for path in &carriers {
        assert!(
            MAY_CARRY_FIXTURE_KEY_MATERIAL.contains(&path.as_str()),
            "{path} carries fixture key material and is not on the list"
        );
    }
    for allowed in MAY_CARRY_FIXTURE_KEY_MATERIAL {
        assert!(
            carriers.iter().any(|path| path == allowed),
            "{allowed} is listed but carries no key material; the entry is stale"
        );
    }
}

/// Whether `text` names `ident` as a WHOLE identifier.
///
/// Substring matching is a false-positive generator — `age` occurs inside `storage`, `package`,
/// `message`, and `ApplyArtifactImage` — and a fence that cries wolf is one people learn to route
/// around rather than read. Every fence below matches on identifier boundaries.
fn names_identifier(text: &str, ident: &str) -> bool {
    fn is_ident_byte(byte: Option<&u8>) -> bool {
        byte.is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_')
    }
    let bytes = text.as_bytes();
    let mut from = 0_usize;
    while let Some(rest) = text.get(from..) {
        let Some(at) = rest.find(ident) else {
            return false;
        };
        let start = from.saturating_add(at);
        let end = start.saturating_add(ident.len());
        let bounded_left = start == 0 || !is_ident_byte(bytes.get(start.saturating_sub(1)));
        let bounded_right = !is_ident_byte(bytes.get(end));
        if bounded_left && bounded_right {
            return true;
        }
        from = start.saturating_add(1);
    }
    false
}

#[test]
fn the_fence_matcher_reads_identifier_boundaries_not_substrings() {
    // The regression this exists for: a fence written against `age` that also fires on every
    // `package`, `storage`, and `message` in the tree.
    assert!(names_identifier("use age::Encryptor;", "age"));
    assert!(names_identifier("let x = age ;", "age"));
    assert!(!names_identifier("mod storage;", "age"));
    assert!(!names_identifier("fn package() {}", "age"));
    assert!(!names_identifier("ApplyArtifactImage::over(b)", "age"));
    assert!(names_identifier(
        "ReceiptId::of_source_bytes(raw)",
        "of_source_bytes"
    ));
    assert!(!names_identifier(
        "fn of_source_bytes_v2() {}",
        "of_source_bytes"
    ));
    assert!(!names_identifier(
        "let my_of_source_bytes = 1;",
        "of_source_bytes"
    ));
    assert!(!names_identifier("", "age"));
}

/// Whether `text` carries an `impl` of `trait_name`, matched on identifier boundaries.
fn implements(text: &str, trait_name: &str) -> bool {
    text.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("impl ") && names_identifier(trimmed, trait_name)
    })
}

/// Every production source file in the workspace, keyed by a slash-normalized relative path.
///
/// Walks `crates/*/src` only: a test target is not a production boundary, and the whole point of
/// several fences below is the distinction. The walk asserts it found sources, so a fence that
/// silently looked in the wrong place fails rather than passing over an empty set.
fn production_sources() -> Vec<(String, String)> {
    fn walk(dir: &Path, prefix: &str, out: &mut Vec<(String, String)>) {
        for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let joined = if prefix.is_empty() {
                name.to_owned()
            } else {
                format!("{prefix}/{name}")
            };
            if path.is_dir() {
                walk(&path, &joined, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                out.push((joined, std::fs::read_to_string(&path).unwrap_or_default()));
            }
        }
    }

    let root = crates_root();
    let mut out: Vec<(String, String)> = Vec::new();
    for entry in std::fs::read_dir(&root).into_iter().flatten().flatten() {
        let src = entry.path().join("src");
        if !src.is_dir() {
            continue;
        }
        let Some(krate) = entry
            .path()
            .file_name()
            .and_then(|n| n.to_str().map(str::to_owned))
        else {
            continue;
        };
        walk(&src, &format!("{krate}/src"), &mut out);
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(
        out.len() > 20,
        "the production source walk found {} files; it is looking in the wrong place",
        out.len()
    );
    out
}

/// The production files naming `ident` as a whole identifier.
fn production_naming(ident: &str) -> Vec<String> {
    production_sources()
        .into_iter()
        .filter(|(_, text)| names_identifier(text, ident))
        .map(|(path, _)| path)
        .collect()
}

/// Assert the set of production files naming `ident` is exactly `allowed`, both ways.
///
/// Two-way on purpose: a file that gains the mention fails, and an entry that no longer has it
/// fails too, so the list cannot rot into a description of what used to be true.
fn fence(ident: &str, allowed: &[&str]) {
    let found = production_naming(ident);
    for path in &found {
        assert!(
            allowed.contains(&path.as_str()),
            "{path} names `{ident}`; only {allowed:?} may"
        );
    }
    for entry in allowed {
        assert!(
            found.iter().any(|path| path == entry),
            "the allow-list entry {entry} no longer names `{ident}`; remove it rather than \
             leaving a fence describing what used to be true"
        );
    }
}

#[test]
fn every_crate_allowed_to_name_the_crypto_implementation_still_names_it() {
    // The other direction of `only_the_edge_may_name_the_crypto_implementation_crate`. A stale
    // entry there would keep passing while quietly widening what the fence permits.
    let named_by: Vec<String> = manifests()
        .into_iter()
        .filter(|(_, manifest)| {
            dependency_lines(manifest)
                .iter()
                .any(|line| line.starts_with("dorc-receipt-crypto"))
        })
        .map(|(krate, _)| krate)
        .collect();
    assert_eq!(
        named_by.len(),
        MAY_NAME_IT.len(),
        "the set naming the crypto implementation crate is {named_by:?}; the allow-list is \
         {MAY_NAME_IT:?}, and the two must agree exactly"
    );
    for entry in MAY_NAME_IT {
        assert!(
            named_by.iter().any(|krate| krate == entry),
            "{entry} is allowed to name the crypto implementation crate and does not; the \
             allow-list has gone stale"
        );
    }
}

#[test]
fn the_identity_mint_is_reachable_from_one_production_file() {
    // A document identity is controller-minted per document. The seam is honest, but nothing in
    // the type stops a production file calling it with fixed bytes, so the gate over its callers
    // is lexical. `ids.rs` both declares it and drives it from its own fixture source, which
    // lives behind `#[cfg(test)]`.
    fence("of_source_bytes", &["receipt/src/ids.rs"]);
}

#[test]
fn the_image_identity_mint_is_reachable_from_one_production_file() {
    // The mint takes bare bytes, which is the shape `content-identities-hash-in-their-constructor`
    // exists to forbid, and no type can say "these bytes are a complete canonical encoding". So the
    // gate over its callers is lexical and two-way, exactly as the document-identity seam's is.
    // Three entries because the fence matches NAMES rather than calls, and each names it for a
    // different reason: `ids.rs` DECLARES it; `image.rs` is the one production file that CALLS it,
    // being the sole mint, which validates, encodes, hashes and stores in one operation; and
    // `lib.rs` names it inside a `compile_fail` doc example proving an image identity cannot cross
    // into another domain. A fence listing only the caller would fail on the other two rather than
    // catching anything.
    fence(
        "of_canonical_image",
        &[
            "receipt/src/ids.rs",
            "receipt/src/image.rs",
            "receipt/src/lib.rs",
        ],
    );
}

#[test]
fn the_planning_input_identity_mint_is_driven_from_one_production_file() {
    // Same shape and same reason as the image mint's fence: bare bytes in, and no type can say
    // "these are the complete canonical encoding of the planner's inputs". `ids.rs` DECLARES it;
    // `plan/src/planning_input.rs` is the one production file that CALLS it, being the module that
    // owns the typed inputs value and its encoding. A third entry means a second seat is deciding
    // what the planner's inputs were, which is exactly what must not appear quietly.
    fence(
        "of_canonical_inputs",
        &["plan/src/planning_input.rs", "receipt/src/ids.rs"],
    );
}

#[test]
fn the_presented_plan_identity_mint_is_driven_from_one_production_file() {
    // Two entries, each for a different reason, on `of_canonical_image`'s terms: `ids.rs` DECLARES
    // it, and `plan/src/erasability.rs` is the one production file that CALLS it — the seat that
    // already owns the canonical identity plane, and therefore the only one holding a complete
    // settled surface to hash. A third entry means a second seat is naming a surface identity,
    // which is what must not appear quietly.
    fence(
        "of_canonical_decision",
        &["plan/src/erasability.rs", "receipt/src/ids.rs"],
    );
}

#[test]
fn the_rehydration_floor_is_decided_at_one_seat() {
    // The GUARANTEE this carries, and where it came from: `plan`'s untracked-adapter inventory
    // enumerates every seat that deliberately carries an untracked grade, and the durable's
    // rehydration floor — an absent or unreadable grade reads MOST-influenced — used to be one of
    // its entries. That inventory walks `plan`'s own sources, so once the floor lives here it is
    // outside what the inventory can see: the guarantee has to travel with its subject rather than
    // the entry being dropped.
    //
    // Two-way and NAME-matched, on `of_canonical_image`'s terms. `reingested.rs` declares the grade
    // and decides it; `lib.rs` names the floor in the doc example that pins a recorded grade never
    // becoming a live account. A THIRD file naming the floor means a second seat is deciding a
    // grade, which is exactly the thing that must not appear quietly — and a seat that decides one
    // WITHOUT naming the floor is not flooring at all, which is why the floor rather than the
    // reader is the subject.
    fence(
        "MostInfluenced",
        &["receipt/src/lib.rs", "receipt/src/reingested.rs"],
    );
}

#[test]
fn verification_material_is_supplied_from_one_production_file() {
    // The resolver is the seam through which a permissive verifier could reach the reader, and a
    // fence covering only the crypto crate's NAME would not see one written elsewhere: a
    // resolver returning a verifier that answers yes need never mention that crate.
    for trait_name in [
        "VerificationKeyResolver",
        "TrustedReceiptVerificationKey",
        "SelfAssertedReceiptVerificationKey",
    ] {
        let found: Vec<String> = production_sources()
            .into_iter()
            .filter(|(_, text)| implements(text, trait_name))
            .map(|(path, _)| path)
            .collect();
        for path in &found {
            assert!(
                path == "receipt-crypto/src/lib.rs",
                "{path} implements `{trait_name}`; only the implementation crate may"
            );
        }
    }
    // Two-way: the implementation crate must still carry the two it is allowed to, or the fence
    // above is guarding a surface that moved.
    for trait_name in [
        "TrustedReceiptVerificationKey",
        "SelfAssertedReceiptVerificationKey",
    ] {
        let found: Vec<String> = production_sources()
            .into_iter()
            .filter(|(_, text)| implements(text, trait_name))
            .map(|(path, _)| path)
            .collect();
        assert_eq!(
            found,
            vec!["receipt-crypto/src/lib.rs".to_owned()],
            "the implementation crate must still implement {trait_name}"
        );
    }
}

/// Every production file permitted to name the read-back wrapper.
///
/// The wrapper is what makes a value from a document unusable as a live one, so the interesting
/// question is not whether it is sound — no accessor hands its contents out — but WHERE it is
/// spoken. Each entry is a seat that has looked at recorded material, and the list is how a new
/// one becomes a diff somebody reads rather than an import somebody adds.
///
/// File-narrow rather than crate-narrow: naming a crate would permit every future file inside it,
/// which is most of what this is for. Two-way, so a stale entry fails as loudly as a new mention.
///
/// Today every entry but one sits inside this crate, and that is the fact worth pinning: nothing
/// downstream READS a document yet. When the report surface does, its file joins this list.
///
/// The exception, `plan/src/lib.rs`, names the wrapper only inside the doc examples that pin the
/// seal across the crate seam — this crate cannot host them, seeing neither the live account nor
/// the live decision they must fail to produce. It is a naming, not a consumption, and the fence
/// matches names deliberately: one that tried to tell the two apart would be guessing.
const MAY_NAME_THE_READ_BACK_WRAPPER: [&str; 6] = [
    "plan/src/lib.rs",
    "receipt/src/graph.rs",
    "receipt/src/lib.rs",
    "receipt/src/outcome.rs",
    "receipt/src/reader.rs",
    "receipt/src/reingested.rs",
];

#[test]
fn every_consumer_of_the_read_back_wrapper_is_enumerated() {
    // Answered by NAME rather than by call, for the reason the image-identity fence gives: a file
    // that merely mentions the wrapper in a signature is as much a consumer as one that calls a
    // decomposition, and a fence that could tell them apart would be re-implementing the compiler.
    fence("Reingested", &MAY_NAME_THE_READ_BACK_WRAPPER);
}

#[test]
fn the_read_back_fence_would_fail_on_a_stale_entry_and_on_a_new_mention() {
    // The fence verified in its FAILING direction. A fence checked only against the tree as it
    // stands is half-verified: it demonstrates that today passes, which is the half that carries
    // no information. Both directions are exercised here against the real walk.
    let found = production_naming("Reingested");
    assert!(
        !found.is_empty(),
        "the walk found no file naming the wrapper; it is looking in the wrong place"
    );

    let stale = "receipt/src/limits.rs";
    assert!(
        !found.iter().any(|path| path == stale),
        "{stale} was chosen because it names no wrapper; pick another for this check"
    );
    assert!(
        !MAY_NAME_THE_READ_BACK_WRAPPER.contains(&stale),
        "the stale-entry direction is only exercised by an entry the list does not hold"
    );

    let dropped = MAY_NAME_THE_READ_BACK_WRAPPER[0];
    assert!(
        found.iter().any(|path| path == dropped),
        "{dropped} is listed and no longer names the wrapper; the entry has gone stale"
    );
    let narrowed: Vec<&str> = MAY_NAME_THE_READ_BACK_WRAPPER
        .into_iter()
        .filter(|entry| *entry != dropped)
        .collect();
    assert!(
        found.iter().any(|path| !narrowed.contains(&path.as_str())),
        "dropping {dropped} from the list must leave a file the fence would refuse; if it does \
         not, the fence permits a mention it never checked"
    );
}

#[test]
fn the_fixture_signature_stand_in_never_reaches_a_production_file() {
    // The graph corpus signs its documents with an inert deterministic stand-in. It is confined
    // by living in a test target; this asserts that rather than trusting it.
    for ident in ["inert_signature", "InertSigner", "InertKey"] {
        let found = production_naming(ident);
        assert!(
            found.is_empty(),
            "the fixture signature stand-in reached production: {found:?} names `{ident}`"
        );
    }
    // Non-empty walk: the assertions above are vacuous if the walk found nothing, and this is
    // the one that would notice.
    assert!(
        production_naming("ReceiptId")
            .iter()
            .any(|p| p == "receipt/src/ids.rs"),
        "the production walk cannot see the crate it is fencing"
    );
}
