//! A hand-rolled word-level diff (`282` §5: "Myers/patience over tokens;
//! latitude"). Hand-rolled to keep this deterministic kernel dependency-free and
//! off the network/deny surface; word streams are tiny (a paragraph of prose),
//! so an O(N·M) LCS is trivially adequate under the network-dominated perf
//! doctrine. Ties break toward `Delete`, keeping the edit script deterministic
//! (`inv-determinism`).

/// One alignment step between the baseline and edited token streams.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum DiffOp {
    /// Baseline `base` and edited `edit` are the same token.
    Equal {
        /// Baseline index.
        base: usize,
        /// Edited index.
        edit: usize,
    },
    /// Baseline `base` is absent from the edited stream.
    Delete {
        /// Baseline index.
        base: usize,
    },
    /// Edited `edit` is absent from the baseline stream.
    Insert {
        /// Edited index.
        edit: usize,
    },
}

/// A row-major LCS-length table addressed with saturating arithmetic so the
/// no-panic lint floor holds without a bespoke suppression.
struct Lcs {
    cols: usize,
    cells: Vec<u32>,
}

impl Lcs {
    fn new(rows: usize, cols: usize) -> Self {
        let size = rows
            .saturating_add(1)
            .saturating_mul(cols.saturating_add(1));
        Lcs {
            cols,
            cells: vec![0; size],
        }
    }

    fn index(&self, i: usize, j: usize) -> usize {
        i.saturating_mul(self.cols.saturating_add(1))
            .saturating_add(j)
    }

    fn get(&self, i: usize, j: usize) -> u32 {
        self.cells.get(self.index(i, j)).copied().unwrap_or(0)
    }

    fn set(&mut self, i: usize, j: usize, value: u32) {
        let idx = self.index(i, j);
        if let Some(cell) = self.cells.get_mut(idx) {
            *cell = value;
        }
    }
}

/// Align two token streams into an edit script (a minimal LCS decomposition).
pub(crate) fn diff<T: Eq>(base: &[T], edit: &[T]) -> Vec<DiffOp> {
    let n = base.len();
    let m = edit.len();
    let mut table = Lcs::new(n, m);
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            let value = if base.get(i) == edit.get(j) {
                table
                    .get(i.saturating_add(1), j.saturating_add(1))
                    .saturating_add(1)
            } else {
                table
                    .get(i.saturating_add(1), j)
                    .max(table.get(i, j.saturating_add(1)))
            };
            table.set(i, j, value);
        }
    }

    let mut ops: Vec<DiffOp> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < n && j < m {
        if base.get(i) == edit.get(j) {
            ops.push(DiffOp::Equal { base: i, edit: j });
            i = i.saturating_add(1);
            j = j.saturating_add(1);
        } else if table.get(i.saturating_add(1), j) >= table.get(i, j.saturating_add(1)) {
            ops.push(DiffOp::Delete { base: i });
            i = i.saturating_add(1);
        } else {
            ops.push(DiffOp::Insert { edit: j });
            j = j.saturating_add(1);
        }
    }
    while i < n {
        ops.push(DiffOp::Delete { base: i });
        i = i.saturating_add(1);
    }
    while j < m {
        ops.push(DiffOp::Insert { edit: j });
        j = j.saturating_add(1);
    }
    ops
}
