//! The Dorc AST — a deliberately-narrow, arena-indexed syntax tree for the slice
//! of POSIX sh the analyzer currently models. Everything outside the slice is
//! represented explicitly as [`NodeKind::Unsupported`] (the ⊤-reject node), never
//! silently dropped or best-effort'd (`inv-top-reject`).
//!
//! Three shape decisions are load-bearing for the analyzer downstream; do not
//! "simplify" them away:
//!
//! * **Arena, not a `Box` tree** (`dac-B`): nodes live in a `Vec` indexed by
//!   [`AstId`], so the analyzer/provenance layers can hang payloads on a stable
//!   id and overlay a graph without re-walking pointers.
//! * **Lossless quoting** (`haz-unquoted`): a [`Word`] is a list of [`WordPart`]s
//!   that records *how* each fragment was quoted. The analyzer needs this to know
//!   whether an expansion may word-split / glob (an unquoted `$x` can change a
//!   command's arity and its set of effect-targets).
//! * **Redirections are first-class** (`haz-redir-as-mutation`): a redirection is
//!   its own node attached to a command, because `: > /etc/x` mutates the
//!   filesystem regardless of the (no-op) command word.

use dorc_core::{AstId, BytePos, Span};

/// A parsed script. Owns the node arena; the [`root`](Ast::root) is the top-level
/// [`NodeKind::Script`]. Construct via [`AstBuilder`].
#[derive(Debug, Clone)]
pub struct Ast {
    nodes: Vec<Node>,
    root: AstId,
}

impl Ast {
    /// Resolve a node id minted for *this* arena.
    #[must_use]
    pub fn node(&self, id: AstId) -> &Node {
        &self.nodes[id.0 as usize]
    }

    #[must_use]
    pub fn root(&self) -> AstId {
        self.root
    }

    /// All nodes, paired with their ids (for whole-tree passes).
    pub fn iter(&self) -> impl Iterator<Item = (AstId, &Node)> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(i, n)| (AstId(i as u32), n))
    }
}

/// Bottom-up arena builder: allocate children, then the parent referencing their
/// ids. `alloc` is the only way to mint an [`AstId`], so ids always resolve.
#[derive(Debug, Default)]
pub struct AstBuilder {
    nodes: Vec<Node>,
}

impl AstBuilder {
    pub fn alloc(&mut self, node: Node) -> AstId {
        let id = AstId(u32::try_from(self.nodes.len()).unwrap_or(u32::MAX));
        self.nodes.push(node);
        id
    }

    /// Read back an already-`alloc`'d node. The recursive-descent parser needs this
    /// to inspect children it just built (their spans, and the command-word literal
    /// the ⊤-triggers key off) while the arena is still under construction.
    /// Additive accessor only — does not change how the arena is built.
    #[must_use]
    pub fn node(&self, id: AstId) -> &Node {
        &self.nodes[id.0 as usize]
    }

    /// Finish, designating `root` (must have been `alloc`'d into this builder).
    #[must_use]
    pub fn finish(self, root: AstId) -> Ast {
        Ast {
            nodes: self.nodes,
            root,
        }
    }
}

/// One AST node: its source span plus its kind.
#[derive(Debug, Clone)]
pub struct Node {
    pub span: Span,
    pub kind: NodeKind,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    /// Top-level: a sequence of complete commands.
    Script { items: Vec<AstId> },
    /// A `;`/newline-separated sequence (a compound body).
    List { items: Vec<AstId> },
    /// A simple command: optional leading assignments, the words (command name +
    /// args), and any redirections. A bare assignment-only command (`x=1`) has an
    /// empty `words`.
    Simple {
        assigns: Vec<AstId>,
        words: Vec<AstId>,
        redirs: Vec<AstId>,
    },
    /// `cmd1 | cmd2 | …`, optionally `!`-negated. `pipefail`/last-stage-status
    /// semantics are the analyzer's concern (`haz-seterr`/`haz-concurrency`).
    Pipeline { negated: bool, stages: Vec<AstId> },
    /// `left && right` / `left || right` (left-associative; nest for chains).
    AndOr {
        op: AndOrOp,
        left: AstId,
        right: AstId,
    },
    /// `( body )` — runs in a subshell (env/var mutations don't escape;
    /// `haz-concurrency`). Carries any redirections on the group.
    Subshell { body: AstId, redirs: Vec<AstId> },
    /// `{ body; }` — runs in the current shell.
    Group { body: AstId, redirs: Vec<AstId> },
    /// `if cond; then …; [elif …;] [else …;] fi`.
    If {
        cond: AstId,
        then_body: AstId,
        elifs: Vec<ElseIf>,
        else_body: Option<AstId>,
    },
    /// `case word in (pat|pat) body ;; … esac`.
    Case { word: AstId, arms: Vec<CaseArm> },
    /// `name() { body; }`.
    FuncDef {
        name: String,
        name_span: Span,
        body: AstId,
    },
    /// A word: a sequence of quoted/unquoted fragments (see [`WordPart`]).
    Word { parts: Vec<WordPart> },
    /// `name=value` (value `None` for `name=`). Used both as a leading
    /// command-assignment and as a standalone statement (`oracle_kind=package`,
    /// the dn-1 anchor).
    Assign {
        name: String,
        name_span: Span,
        value: Option<AstId>,
    },
    /// A redirection attached to a command (its own node — `haz-redir-as-mutation`).
    Redir {
        op: RedirOp,
        fd: Option<u32>,
        target: RedirTarget,
    },
    /// ⊤-reject: a construct outside the modeled subset (`inv-top-reject`,
    /// chord `ch-errnode`). Provenance-bearing (the `Node.span` is the raw range);
    /// the analyzer treats it as an absorbing ⊤ — un-probeable and un-skippable.
    /// Salvaged children, if any, are kept so unrelated analysis can continue.
    Unsupported {
        reason: UnsupportedReason,
        salvaged: Vec<AstId>,
    },
}

/// `elif cond; then body`.
#[derive(Debug, Clone)]
pub struct ElseIf {
    pub cond: AstId,
    pub body: AstId,
}

/// One `case` arm: its patterns (each a [`NodeKind::Word`]) and body.
#[derive(Debug, Clone)]
pub struct CaseArm {
    pub patterns: Vec<AstId>,
    pub body: AstId,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndOrOp {
    And,
    Or,
}

/// A fragment of a word, recording its quoting (the `haz-unquoted` model).
/// Whether the whole word may word-split/glob is *derived* from these: an
/// unquoted [`Param`](WordPart::Param)/[`CommandSubst`](WordPart::CommandSubst)
/// can split; quoted ones cannot. See [`Word::may_split`].
#[derive(Debug, Clone)]
pub enum WordPart {
    /// Unquoted literal text (subject to glob if it contains `*?[`).
    Literal(String),
    /// `'…'` — fully literal, no expansion, no splitting.
    SingleQuoted(String),
    /// `"…"` — expansions happen inside but the result does not word-split.
    DoubleQuoted(Vec<WordPart>),
    /// `$name` / `${name}` — a *simple* parameter expansion (unquoted ⇒ may split).
    Param { name: String },
    /// `$( … )` / `` `…` `` — command substitution; body is a `List`/`Script` node.
    CommandSubst(AstId),
    /// A parameter expansion with operators (`${x:-y}`, `${#x}`, `${x%…}`) — kept
    /// opaque for now (treated ⊤-ward by the analyzer), not decoded.
    ParamComplex,
    /// `$(( … ))` — arithmetic expansion. A ⊤-trigger (dynamic); flagged, not eval'd.
    Arithmetic,
}

impl WordPart {
    /// Does this part, appearing unquoted at top level, permit word-splitting?
    #[must_use]
    pub fn splits_unquoted(&self) -> bool {
        matches!(
            self,
            WordPart::Param { .. } | WordPart::CommandSubst(_) | WordPart::ParamComplex
        )
    }
}

/// Convenience view over a [`NodeKind::Word`]'s parts.
#[derive(Debug, Clone, Copy)]
pub struct Word<'a> {
    pub parts: &'a [WordPart],
}

impl Word<'_> {
    /// True if the word can expand to a different number of fields (an unquoted
    /// expansion present) — i.e. its arity / effect-target set is not statically
    /// fixed. Such words degrade their command toward ⊤ (`haz-unquoted`).
    #[must_use]
    pub fn may_split(&self) -> bool {
        self.parts.iter().any(WordPart::splits_unquoted)
    }

    /// If the word is a single literal (one [`Literal`](WordPart::Literal) or
    /// [`SingleQuoted`](WordPart::SingleQuoted) part), return it. This is the only
    /// case the analyzer treats as a statically-known token (command names, sub-
    /// verbs, the dn-1 anchors). Anything else is not statically a fixed string.
    #[must_use]
    pub fn as_literal(&self) -> Option<&str> {
        match self.parts {
            [WordPart::Literal(s)] | [WordPart::SingleQuoted(s)] => Some(s),
            _ => None,
        }
    }
}

/// A redirection's target.
#[derive(Debug, Clone)]
pub enum RedirTarget {
    /// `> file`, `< file`, `>> file` — a word (the path; itself an effect-target).
    Word(AstId),
    /// `<<EOF` / `<<'EOF'` — the here-document body, and whether the delimiter was
    /// quoted (quoted ⇒ no expansion in the body). The body is *generated content*;
    /// if it is itself code (`cat <<EOF >script`), the analyzer must STOP there
    /// (generated-deferred code is data, not this script's control-flow).
    HereDoc { body: String, quoted: bool },
    /// `>&2`, `<&-`, … — a file-descriptor dup/close.
    Fd(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirOp {
    /// `<`
    Read,
    /// `>`
    Write,
    /// `>>`
    Append,
    /// `<<` (here-doc)
    HereDoc,
    /// `>&` / `<&` (fd dup)
    Dup,
}

/// Why a construct was ⊤-rejected. The named variants are the canonical
/// ⊤-trigger set (synthesis note 160 §2); `Other` is the catch-all for
/// not-yet-modeled-but-not-dangerous grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnsupportedReason {
    /// `eval`, `.`/`source` of a dynamic target, dynamic command name `"$cmd" …`.
    DynamicExecution,
    /// `$(( … ))` arithmetic expansion.
    ArithmeticExpansion,
    /// lvalue-taking builtins: `unset "$x"`, `printf -v`, `${!ref}`, `test -v`.
    DynamicLValue,
    /// `for`/`while`/`until` loops — not in the current modeled subset.
    Loop,
    /// grammar the parser recognises but the subset does not yet model.
    Unmodeled(&'static str),
}

impl Node {
    /// Helper for the parser/tests: a `Script` node spanning the whole source.
    #[must_use]
    pub fn script(items: Vec<AstId>, len: u32) -> Node {
        Node {
            span: Span::new(BytePos(0), BytePos(len)),
            kind: NodeKind::Script { items },
        }
    }
}
