//! The bounded, exact compile receipt (`282:rul-promote-requires-fresh-compilation`).
//!
//! This is deliberately a small plain-text packet rather than a hash or a general
//! serialization format. Exact candidate bytes are the identity checked by promote.

use std::fmt;

/// Receipt schema currently understood by this binary.
pub const RECEIPT_SCHEMA: u32 = 1;
/// Bump when an interpretation-affecting compiler rule changes.
pub const RECEIPT_SEMANTICS_EPOCH: u32 = 1;
/// Conservative receipt resource limits. The ordinary corpus is much smaller.
pub const MAX_RECEIPT_BYTES: usize = 2 * 1024 * 1024;
/// Maximum selected cases represented by one receipt.
pub const MAX_RECEIPT_CASES: usize = 64;
/// Maximum replay results represented by one receipt.
pub const MAX_RECEIPT_REPLAYS: usize = 512;
/// Maximum UTF-8 bytes in one framed text field.
pub const MAX_RECEIPT_FIELD_BYTES: usize = 256 * 1024;

/// One fully-owned inspected replay. `interpretation` is empty for bytes-only results.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InspectedReplay {
    /// Original command bytes.
    pub command: String,
    /// Exact replay result bytes.
    pub output: String,
    /// Complete compiler interpretation and concrete render, when editable.
    pub interpretation: String,
}

/// Exactly the compile result an author inspected before a promotion may proceed.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InspectedCompilation {
    /// Repository-relative selected case paths in canonical lexical order.
    pub cases: Vec<(String, String)>,
    /// The complete catalog source consumed by the compilation.
    pub catalog: String,
    /// Replays in case and source order.
    pub replays: Vec<InspectedReplay>,
}

/// A malformed, unsupported, or over-limit receipt.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ReceiptError {
    /// A resource bound was exceeded before an allocation or append.
    Limit(&'static str),
    /// The packet grammar is not the current closed grammar.
    Malformed(&'static str),
}

impl fmt::Display for ReceiptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Limit(what) => write!(f, "receipt limit exceeded: {what}"),
            Self::Malformed(what) => write!(f, "malformed receipt: {what}"),
        }
    }
}

impl std::error::Error for ReceiptError {}

/// A successful exact recomputation. Its constructor is private so parsing cannot
/// manufacture future promotion authority.
#[derive(Debug)]
pub struct ValidatedCompilation {
    _private: (),
}

/// Encode a canonical, length-framed packet after checking all resource bounds.
///
/// # Errors
/// Returns a refusal when a field, count, path, or total packet exceeds its limit.
pub fn encode(inspection: &InspectedCompilation) -> Result<Vec<u8>, ReceiptError> {
    if inspection.cases.is_empty() || inspection.cases.len() > MAX_RECEIPT_CASES {
        return Err(ReceiptError::Limit("case count"));
    }
    if inspection.replays.len() > MAX_RECEIPT_REPLAYS {
        return Err(ReceiptError::Limit("replay count"));
    }
    let mut previous = None;
    for (path, text) in &inspection.cases {
        if !safe_repo_path(path) || previous.is_some_and(|prior: &String| prior >= path) {
            return Err(ReceiptError::Malformed("case path ordering"));
        }
        previous = Some(path);
        check_field(path)?;
        check_field(text)?;
    }
    check_field(&inspection.catalog)?;
    for replay in &inspection.replays {
        check_field(&replay.command)?;
        check_field(&replay.output)?;
        check_field(&replay.interpretation)?;
    }

    let mut out = Vec::new();
    out.extend_from_slice(b"dorc-loom-receipt\n");
    out.extend_from_slice(b"schema: 1\nsemantics: 1\nidentity-mode: exact\n");
    frame(&mut out, "catalog", &inspection.catalog)?;
    for (path, text) in &inspection.cases {
        frame(&mut out, "case-path", path)?;
        frame(&mut out, "case-text", text)?;
    }
    for replay in &inspection.replays {
        frame(&mut out, "replay-command", &replay.command)?;
        frame(&mut out, "replay-output", &replay.output)?;
        frame(&mut out, "interpretation", &replay.interpretation)?;
    }
    if out.len() > MAX_RECEIPT_BYTES {
        return Err(ReceiptError::Limit("total bytes"));
    }
    Ok(out)
}

/// Validate the closed packet grammar and all receipt bounds.
///
/// # Errors
/// Returns a refusal for malformed, unsupported, non-UTF-8, or oversized packets.
pub fn parse(packet: &[u8]) -> Result<InspectedCompilation, ReceiptError> {
    if packet.len() > MAX_RECEIPT_BYTES {
        return Err(ReceiptError::Limit("total bytes"));
    }
    let prefix = b"dorc-loom-receipt\nschema: 1\nsemantics: 1\nidentity-mode: exact\n";
    let Some(mut rest) = packet.strip_prefix(prefix) else {
        return Err(ReceiptError::Malformed(
            "magic, schema, semantics, or identity mode",
        ));
    };
    let mut catalog = None;
    let mut cases = Vec::new();
    let mut replays = Vec::new();
    let mut pending_path = None;
    let mut pending_replay: Option<(String, String)> = None;
    while !rest.is_empty() {
        let (tag, value, next) = read_frame(rest)?;
        rest = next;
        match tag {
            "catalog" if catalog.is_none() && cases.is_empty() && replays.is_empty() => {
                catalog = Some(value);
            }
            "case-path" if catalog.is_some() && pending_path.is_none() && replays.is_empty() => {
                pending_path = Some(value);
            }
            "case-text" if pending_path.is_some() && replays.is_empty() => {
                let path = pending_path
                    .take()
                    .ok_or(ReceiptError::Malformed("case pairing"))?;
                if !safe_repo_path(&path)
                    || cases
                        .last()
                        .is_some_and(|(prior, _): &(String, String)| prior >= &path)
                {
                    return Err(ReceiptError::Malformed("case paths"));
                }
                cases.push((path, value));
            }
            "replay-command" if pending_replay.is_none() && pending_path.is_none() => {
                pending_replay = Some((value, String::new()));
            }
            "replay-output" if pending_replay.is_some() => {
                let (command, _) = pending_replay
                    .take()
                    .ok_or(ReceiptError::Malformed("replay pairing"))?;
                pending_replay = Some((command, value));
            }
            "interpretation" if pending_replay.is_some() => {
                let (command, output) = pending_replay
                    .take()
                    .ok_or(ReceiptError::Malformed("replay pairing"))?;
                replays.push(InspectedReplay {
                    command,
                    output,
                    interpretation: value,
                });
            }
            _ => return Err(ReceiptError::Malformed("field order or tag")),
        }
        if cases.len() > MAX_RECEIPT_CASES || replays.len() > MAX_RECEIPT_REPLAYS {
            return Err(ReceiptError::Limit("record count"));
        }
    }
    if catalog.is_none() || cases.is_empty() || pending_path.is_some() || pending_replay.is_some() {
        return Err(ReceiptError::Malformed("required fields"));
    }
    Ok(InspectedCompilation {
        cases,
        catalog: catalog.unwrap_or_default(),
        replays,
    })
}

/// Mint a witness only after a current inspected compilation exactly matches a validated receipt.
///
/// # Errors
/// Returns a refusal unless the packet is valid and byte-identical to the recomputation.
pub fn validate_current(
    packet: &[u8],
    current: &InspectedCompilation,
) -> Result<ValidatedCompilation, ReceiptError> {
    let _ = parse(packet)?;
    if packet == encode(current)? {
        Ok(ValidatedCompilation { _private: () })
    } else {
        Err(ReceiptError::Malformed(
            "receipt does not match current compilation",
        ))
    }
}

fn check_field(value: &str) -> Result<(), ReceiptError> {
    if value.len() > MAX_RECEIPT_FIELD_BYTES {
        Err(ReceiptError::Limit("field bytes"))
    } else {
        Ok(())
    }
}

fn frame(out: &mut Vec<u8>, tag: &str, value: &str) -> Result<(), ReceiptError> {
    let header = format!("{tag} {}\n", value.len());
    if out
        .len()
        .saturating_add(header.len())
        .saturating_add(value.len())
        .saturating_add(1)
        > MAX_RECEIPT_BYTES
    {
        return Err(ReceiptError::Limit("total bytes"));
    }
    out.extend_from_slice(header.as_bytes());
    out.extend_from_slice(value.as_bytes());
    out.push(b'\n');
    Ok(())
}

fn read_frame(input: &[u8]) -> Result<(&str, String, &[u8]), ReceiptError> {
    let Some(line_end) = input.iter().position(|byte| *byte == b'\n') else {
        return Err(ReceiptError::Malformed("frame header"));
    };
    let line_bytes = input
        .get(..line_end)
        .ok_or(ReceiptError::Malformed("frame header"))?;
    let line =
        std::str::from_utf8(line_bytes).map_err(|_| ReceiptError::Malformed("header UTF-8"))?;
    let Some((tag, length)) = line.split_once(' ') else {
        return Err(ReceiptError::Malformed("frame header"));
    };
    if tag.is_empty() || length.is_empty() || !length.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(ReceiptError::Malformed("frame length"));
    }
    let length: usize = length
        .parse()
        .map_err(|_| ReceiptError::Malformed("frame length overflow"))?;
    if length > MAX_RECEIPT_FIELD_BYTES {
        return Err(ReceiptError::Limit("field bytes"));
    }
    let body_start = line_end.saturating_add(1);
    let body_end = body_start
        .checked_add(length)
        .ok_or(ReceiptError::Malformed("frame length overflow"))?;
    let newline = body_end
        .checked_add(1)
        .ok_or(ReceiptError::Malformed("frame length overflow"))?;
    if input.get(body_end) != Some(&b'\n') || newline > input.len() {
        return Err(ReceiptError::Malformed("truncated frame"));
    }
    let value_bytes = input
        .get(body_start..body_end)
        .ok_or(ReceiptError::Malformed("truncated frame"))?;
    let value = std::str::from_utf8(value_bytes)
        .map_err(|_| ReceiptError::Malformed("field UTF-8"))?
        .to_owned();
    let rest = input
        .get(newline..)
        .ok_or(ReceiptError::Malformed("truncated frame"))?;
    Ok((tag, value, rest))
}

fn safe_repo_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains(['\\', ':', '\0'])
        && path.split('/').all(|part| !matches!(part, "" | "." | ".."))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inspection() -> InspectedCompilation {
        InspectedCompilation {
            cases: vec![("cases/a.txt".to_owned(), "case".to_owned())],
            catalog: "catalog".to_owned(),
            replays: vec![InspectedReplay {
                command: "dorc plan".to_owned(),
                output: "output".to_owned(),
                interpretation: "view".to_owned(),
            }],
        }
    }

    #[test]
    fn exact_packet_round_trips_and_rejects_changed_current_content() {
        let original = inspection();
        let packet = encode(&original).expect("receipt encodes");
        assert_eq!(parse(&packet), Ok(original.clone()));
        assert!(validate_current(&packet, &original).is_ok());
        let mut changed = original;
        changed.replays[0].output.push('!');
        assert!(validate_current(&packet, &changed).is_err());
    }

    #[test]
    fn parser_refuses_unknown_trailing_and_oversized_forms() {
        let packet = encode(&inspection()).expect("receipt encodes");
        let mut trailing = packet.clone();
        trailing.extend_from_slice(b"unknown 0\n\n");
        assert!(parse(&trailing).is_err());
        assert!(parse(&vec![b'x'; MAX_RECEIPT_BYTES.saturating_add(1)]).is_err());
    }
}
