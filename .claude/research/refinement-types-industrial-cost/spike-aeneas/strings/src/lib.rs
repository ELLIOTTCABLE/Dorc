//! Isolating the `&'static str` wall the census hit: which spelling of a static
//! string aeneas can carry, if any. Lenient mode reports every failure at once, so
//! one run classifies all four.

#![allow(dead_code)]

pub fn return_literal() -> &'static str {
    "systemctl.oracle.sh:12"
}

pub const SLUG: &str = "systemctl.oracle.sh:12";

pub fn return_const() -> &'static str {
    SLUG
}

pub fn consume_str(s: &str) -> usize {
    s.len()
}

pub fn return_bytes() -> &'static [u8] {
    b"systemctl.oracle.sh:12"
}
