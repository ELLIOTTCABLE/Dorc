//! Smoke test: the smallest thing that exercises charon -> aeneas -> Lean, so a
//! later failure is attributable to the extract under test and not the pipeline.

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub enum Tri {
    Lo,
    Mid(u32),
    Hi,
}

pub fn widen(t: &Tri) -> u32 {
    match t {
        Tri::Lo => 0,
        Tri::Mid(x) => *x,
        Tri::Hi => u32::MAX,
    }
}

pub fn sum(xs: &[u32]) -> u32 {
    let mut acc = 0u32;
    let mut i = 0usize;
    while i < xs.len() {
        acc = acc.wrapping_add(xs[i]);
        i += 1;
    }
    acc
}
