//! Where the book first OBSERVES or MUTATES a name (`30Pb:fnd-emission-legality-covers-all-shell-state`).
//!
//! # Why a census rather than a lattice
//!
//! The emission planner asks one question of the book, at one position: *does anything above this
//! `.` observe or mutate a name the bundle binds?* That is not a dataflow fact — it is an
//! occurrence question over the program text — so this is one walk over the AST with no fixpoint,
//! and the answer it keeps per name is the EARLIEST position, because "is there a use above P" is
//! exactly "is the earliest use before P".
//!
//! # The enumeration is closed, and everything outside it is a use
//!
//! `silence-licenses-nothing`: what counts is enumerated below, and anything the walk cannot decide
//! is a use of EVERY name rather than of none. Two constructs land there today and both are the
//! lexer's own loss rather than a modelling choice — a command word that is not a literal, and a
//! `ParamExpansion` whose base the lexer could not name, or whose operator it does not model
//! (`28O:res-load-inert-conservatism`). The decoded base and operand word narrow it at the one
//! seat below and nowhere else.
//!
//! Not this module's question: whether the unit carries a DYNAMISM OPENER (an unresolvable load, a
//! definition vector, a string-execution site). That is `plan::region::CensusOpeners`, and the two
//! are deliberately separate — one is about a name, the other about a population.

use std::collections::BTreeMap;

use dorc_core::{AstId, BytePos};
use dorc_syntax::ast::{Ast, NodeKind, ParamOp, WordPart};

/// Every name the book observes or mutates, at the earliest position it does so.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NameUseCensus {
    earliest: BTreeMap<String, BytePos>,
    universal: Option<BytePos>,
}

impl NameUseCensus {
    /// Walk one book.
    ///
    /// Every node is visited, reachable or not: an unreached use is still text above the `.`, and
    /// reading reachability here would make the answer depend on a fold that has no business
    /// deciding where a definition may stand.
    #[must_use]
    pub fn of(ast: &Ast) -> Self {
        let mut census = Self::default();
        for index in 0..ast.len() {
            let id = AstId(u32::try_from(index).unwrap_or(u32::MAX));
            let node = ast.node(id);
            let at = node.span.lo;
            match &node.kind {
                NodeKind::Simple { words, .. } => census.simple_command(ast, words, at),
                NodeKind::Assign { name, .. }
                | NodeKind::FuncDef { name, .. }
                | NodeKind::ForLoop { var: name, .. } => census.uses(name, at),
                NodeKind::Word { parts } => census.word(parts, at),
                NodeKind::Unsupported { .. } => census.uses_everything(at),
                _ => {}
            }
        }
        census
    }

    /// The earliest position at which the book observes or mutates `name`, universal uses included.
    #[must_use]
    pub fn first_use_of(&self, name: &str) -> Option<BytePos> {
        match (self.earliest.get(name).copied(), self.universal) {
            (Some(named), Some(all)) => Some(named.min(all)),
            (named, all) => named.or(all),
        }
    }

    /// Does the book observe or mutate `name` anywhere before `before`?
    ///
    /// The predicate the hoist legality question is stated in: a bundle-bound name the book already
    /// touches above the `.` cannot be lifted over that touch, because lifting would change which
    /// binding is live where the author reads it.
    #[must_use]
    pub fn uses_above(&self, name: &str, before: BytePos) -> bool {
        self.first_use_of(name).is_some_and(|at| at < before)
    }

    /// Every name the book names at all, in a deterministic order — the RESERVED set an emitted-name
    /// allocator must stay clear of (`30Pb:fnd-emitted-names-need-freshness-and-hygiene`).
    ///
    /// Deliberately every USE rather than every top-level BINDING: over-broad here costs a longer
    /// digest in a book that merely mentions the name, and the thing it must not miss is a book that
    /// DEFINES a name the emission was about to mint.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.earliest.keys().map(String::as_str)
    }

    /// The earliest construct that is a use of EVERY name, where the book carries one — the ⊤ arm,
    /// exposed so a disclosure can say which line closed the question.
    #[must_use]
    pub const fn first_universal_use(&self) -> Option<BytePos> {
        self.universal
    }

    fn uses(&mut self, name: &str, at: BytePos) {
        self.earliest
            .entry(name.to_owned())
            .and_modify(|first| *first = (*first).min(at))
            .or_insert(at);
    }

    /// THE ⊤ SEAT: a construct the walk cannot decode is a use of every name.
    fn uses_everything(&mut self, at: BytePos) {
        self.universal = Some(self.universal.map_or(at, |first| first.min(at)));
    }

    /// A simple command's own name-uses: the word in command position is a CALL, and a handful of
    /// builtins name a function or variable in their operands rather than calling it.
    ///
    /// The operand-naming set is closed and grows by name only, like every other decidable set in
    /// this engine (`dec-decidable-set-v0`'s posture): `command -v`/`-V` and `type` ask whether a
    /// name is bound, `unset`/`unalias` remove one, `alias` binds one, and `export`/`readonly` bind
    /// a variable. Word-reads inside the operands are counted by the `Word` arm independently, so
    /// nothing here has to walk into them.
    fn simple_command(&mut self, ast: &Ast, words: &[AstId], at: BytePos) {
        let literal = |word: &AstId| match &ast.node(*word).kind {
            NodeKind::Word { parts } => dorc_syntax::sem::const_literal_text(parts),
            _ => None,
        };
        let Some(head) = words.first() else {
            return;
        };
        let Some(name) = literal(head) else {
            return self.uses_everything(at);
        };
        let operands = |skip: usize| -> Vec<Option<String>> {
            words.iter().skip(1 + skip).map(literal).collect()
        };
        let mut names: Vec<Option<String>> = match name.as_str() {
            "eval" => return self.uses_everything(at),
            "command" | "type" => operands(usize::from(name == "command")),
            "unset" | "unalias" | "export" | "readonly" => operands(0),
            _ => {
                self.uses(&name, at);
                return;
            }
        };
        self.uses(&name, at);
        for operand in names.drain(..) {
            match operand {
                Some(operand) if operand.starts_with('-') => {}
                Some(operand) => self.uses(operand.split('=').next().unwrap_or(&operand), at),
                None => self.uses_everything(at),
            }
        }
    }

    /// A word's own reads: every parameter expansion names a variable the book observes.
    fn word(&mut self, parts: &[WordPart], at: BytePos) {
        for part in parts {
            match part {
                WordPart::Literal(_) | WordPart::SingleQuoted(_) => {}
                WordPart::DoubleQuoted(inner) => self.word(inner, at),
                WordPart::Param { name } => self.uses(name, at),
                // The decoded base is the read; the operand word carries its own reads. An
                // operator the syntax does not model, or a base it could not name, is ⊤.
                WordPart::ParamExpansion { base, op } => {
                    if base.is_empty() {
                        self.uses_everything(at);
                    } else {
                        self.uses(base, at);
                    }
                    match op {
                        ParamOp::EmptyDefault { .. } | ParamOp::Length => {}
                        ParamOp::Substitute { word, .. } => self.word(word, at),
                        ParamOp::Trim { pattern, .. } => self.word(pattern, at),
                        ParamOp::Unmodelled => self.uses_everything(at),
                    }
                }
                WordPart::Arithmetic | WordPart::CommandSubst(_) => self.uses_everything(at),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NameUseCensus;
    use dorc_core::BytePos;

    fn census(src: &str) -> NameUseCensus {
        NameUseCensus::of(&dorc_syntax::parse(src).value)
    }

    /// The whole enumerated set, in one book, each spelling above a sentinel position.
    #[test]
    fn every_enumerated_shape_is_a_use_of_the_name_it_names() {
        let src = concat!(
            "hork_helper x\n",              // a call
            "command -v hork_command\n",    // a definedness probe
            "type hork_type\n",             // the same question, another spelling
            "unset -f hork_unset\n",        // a removal
            "unalias hork_unalias\n",       // an alias removal
            "export HORK_EXPORT=1\n",       // a variable binding
            "printf '%s' \"$HORK_READ\"\n", // a variable read
            "HORK_ASSIGN=2\n",              // a bare assignment
            "hork_funcdef() { :; }\n",      // a definition
            "for HORK_LOOP in a b; do :; done\n",
        );
        let census = census(src);
        for name in [
            "hork_helper",
            "hork_command",
            "hork_type",
            "hork_unset",
            "hork_unalias",
            "HORK_EXPORT",
            "HORK_READ",
            "HORK_ASSIGN",
            "hork_funcdef",
            "HORK_LOOP",
        ] {
            assert!(
                census.first_use_of(name).is_some(),
                "{name} is observed or mutated by this book"
            );
        }
        assert_eq!(
            census.first_universal_use(),
            None,
            "nothing here is undecodable, so no name is used by accident"
        );
    }

    /// A name the book never touches is not used, and the position question is answered against the
    /// EARLIEST touch rather than against any later one.
    #[test]
    fn an_untouched_name_is_free_and_a_touched_one_is_pinned_at_its_first() {
        let census = census("hork one\nwombat two\nhork three\n");
        assert!(!census.uses_above("gribble", BytePos(u32::MAX)));
        assert_eq!(census.first_use_of("hork"), Some(BytePos(0)));
        assert!(census.uses_above("hork", BytePos(1)));
        assert!(!census.uses_above("wombat", BytePos(1)));
    }

    /// The ⊤ arm: a command word the lexer could not resolve to a literal, and an arithmetic
    /// expansion, whose text the lexer never captures, are each a use of EVERY name — including
    /// names the book never spells.
    #[test]
    fn an_undecodable_construct_is_a_use_of_every_name() {
        for src in ["\"$CMD\" arg\n", "printf '%s' \"$((1 + 1))\"\n"] {
            let census = census(src);
            assert!(
                census.uses_above("a_name_this_book_never_spells", BytePos(u32::MAX)),
                "{src:?} decodes to nothing, so it may touch anything"
            );
        }
    }

    /// A TRIM names its base and whatever its pattern reads, and NOTHING else. The `${0%/*}`
    /// script-relative idiom is spelled this way, so a ⊤ here would close the hoist question for
    /// every book that uses it.
    #[test]
    fn a_trim_expansion_names_its_base_and_its_pattern_only() {
        let census = census("printf '%s' \"${ROOT%/$SUFFIX}\"\n");
        assert_eq!(census.first_universal_use(), None);
        assert!(census.uses_above("ROOT", BytePos(u32::MAX)));
        assert!(census.uses_above("SUFFIX", BytePos(u32::MAX)));
        assert!(!census.uses_above("a_name_this_book_never_spells", BytePos(u32::MAX)));
    }

    /// …and a `${name-}` DOES decode, because that one spelling keeps its name — the narrowing the
    /// load lane's `ParamExpansion` decode carries through, at this one seat.
    #[test]
    fn the_nounset_safe_expansion_names_the_one_variable_it_reads() {
        let census = census("printf '%s' \"${WOMBAT_ROOT-}\"\n");
        assert_eq!(census.first_universal_use(), None);
        assert!(census.uses_above("WOMBAT_ROOT", BytePos(u32::MAX)));
    }
}
