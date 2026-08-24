//! The pure plan-invocation boundary shared by command-line and replay adapters.
//!
//! The boundary owns no filesystem, environment, terminal, clock, randomness, or process
//! interaction. Its inputs are the exact bytes an edge acquired; its output is an ordered
//! transcript for that edge to decorate and write.

/// Compute the deterministic content identity binding an invocation's records to its book.
///
/// SHA-256, through the receipt crate's one implementation. It replaced an FNV-1a-64 under
/// `28F:rul-digest-lands-now` (FNV is a drift-detector, and `rul-fixture-identity-never-production`
/// forbids that class at a production boundary, naming DEFAULT PERSISTENCE among them), and then a
/// hand-rolled FIPS 180-4 pass, which existed only because the kernel had no dependency that could
/// hash. It does now, and two implementations of one hash is the duplication `sha2` was chosen over.
///
/// Not a keyed MAC and not a signature: it answers "are these the same bytes", not "did someone
/// authorized produce them". Deliberately NOT domain-separated — a reader holding the file must be
/// able to reproduce it with an ordinary `sha256sum`.
#[must_use]
pub fn book_digest(source: &str) -> String {
    dorc_receipt::ids::span_digest_hex(source.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The published FIPS 180-4 vectors. They pinned a hand-rolled hash when this module had one;
    /// they stay because they are what proves the swap to `sha2` moved no recorded digest — the
    /// one-block ("abc"), two-block (56 bytes, exercising a second chunk and the length encoding),
    /// and empty (padding-only) cases.
    #[test]
    fn the_digest_matches_the_published_sha256_vectors() {
        assert_eq!(
            book_digest("abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            "FIPS 180-4 one-block vector"
        );
        assert_eq!(
            book_digest("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
            "FIPS 180-4 two-block vector"
        );
        assert_eq!(
            book_digest(""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "the empty message, which is pure padding"
        );
    }

    /// One flipped byte must move the digest. Trivially true for any real hash, and worth pinning
    /// anyway: this is the whole property the durable's book-identity check rests on.
    #[test]
    fn a_one_byte_change_moves_the_digest() {
        assert_ne!(
            book_digest("apt-get update"),
            book_digest("apt-get upgrade")
        );
        assert_eq!(book_digest("abc").len(), 64, "rendered as full hex");
    }

    #[test]
    fn digest_depends_on_exact_bytes() {
        assert_ne!(book_digest("hork\n"), book_digest("hork\r\n"));
    }
}
