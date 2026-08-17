//! Seat citation resolution (`301` §5 anchors).
//!
//! v0's one anchor kind is `fn-seat`: the cited chokepoint function. A seat is not minted
//! here — it is the chokepoint the house architecture already mandates per behaviour
//! (`relational-compare-chokepoint`, `selector-chokepoint`, one-named-seat-per-invariant),
//! cited by this catalogue. Resolving it is a CHECKED CONFIRMATION rather than new
//! information, which is exactly why the check is worth having: it is the thing that goes red
//! when a rename moves the chokepoint out from under a law.
//!
//! # Why resolution is owner-scoped rather than a file-wide search
//!
//! A citation must name ONE declaration. `analysis::lattice` declares `fn join` seven times —
//! once in the trait and once per implementing combinator — so a search that accepted any `fn
//! join` in the file confirmed a citation against a declaration nobody chose, and would have
//! gone on confirming it after the intended one was deleted. Resolution therefore reads the
//! seat's OWN segments: `…::<module>::<Owner>::<fn>` resolves inside the block `Owner` names —
//! the `trait Owner` declaration, or the `impl … for Owner` / `impl Owner` blocks that Rust's
//! own `Owner::fn` path syntax reaches. Two candidates are a refusal, never a pick.
//!
//! The seat's three consumers are each simple because a seat is one function — the reach
//! check (one boolean region-hit), the mutant scope (one filtered function), and the rustdoc
//! backlink. Only resolution is built at v0; the other two are named seams.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// Where the cited seat was found.
#[derive(Clone, Debug)]
pub struct Resolved {
    /// Repo-relative path to the file declaring it.
    pub file: String,
    /// 1-based line of the declaration.
    pub line: usize,
}

/// One `trait`/`impl` block, and the name a citation reaches it by.
#[derive(Debug)]
struct Block {
    /// The header as written, for a refusal that a reader can go look at.
    header: String,
    /// The name `Owner::fn` must spell to reach this block.
    owner: String,
}

/// One `fn` declaration, with the block that owns it (`None` at module level).
#[derive(Debug)]
struct Declaration {
    name: String,
    line: usize,
    owner: Option<usize>,
}

/// Resolve `seat`, spelled `dorc_<crate>::<module>::[<Owner>::]<fn>`.
///
/// # Errors
/// When the crate, the module file, or the declaration cannot be found, or when the citation
/// is AMBIGUOUS. The message names which of those failed, because "seat unresolved" alone
/// sends a reader to the wrong place.
pub fn resolve(seat: &str, repo_root: &Path) -> Result<Resolved, String> {
    let mut segments = seat.split("::");
    let crate_seg = segments
        .next()
        .ok_or_else(|| format!("{seat}: empty citation"))?;
    let rest: Vec<&str> = segments.collect();
    let (module, owner, function) = match rest.as_slice() {
        [] => return Err(format!("{seat}: names a crate but no function")),
        [_] => return Err(format!("{seat}: names no module to look in")),
        [module, function] => (*module, None, *function),
        [module, .., owner, function] => (*module, Some(*owner), *function),
    };
    let crate_dir = crate_dir(crate_seg, repo_root)
        .ok_or_else(|| format!("{seat}: no crate directory for `{crate_seg}`"))?;
    let module_file = crate_dir.join("src").join(format!("{module}.rs"));
    if !module_file.is_file() {
        return Err(format!(
            "{seat}: no module file {}",
            relative(repo_root, &module_file)
        ));
    }
    let text = std::fs::read_to_string(&module_file)
        .map_err(|e| format!("{}: {e}", module_file.display()))?;
    let file = relative(repo_root, &module_file);
    let (blocks, declarations) = scan(&text);
    let matches: Vec<&Declaration> = declarations
        .iter()
        .filter(|d| d.name == function && owner_of(d, &blocks).as_deref() == owner)
        .collect();
    match matches.as_slice() {
        [one] => Ok(Resolved {
            file,
            line: one.line,
        }),
        [] => Err(unresolved(seat, &file, owner, function, &declarations)),
        many => Err(ambiguous(seat, &file, many, &blocks)),
    }
}

fn owner_of(declaration: &Declaration, blocks: &[Block]) -> Option<String> {
    declaration
        .owner
        .and_then(|index| blocks.get(index))
        .map(|block| block.owner.clone())
}

fn unresolved(
    seat: &str,
    file: &str,
    owner: Option<&str>,
    function: &str,
    declarations: &[Declaration],
) -> String {
    let elsewhere = declarations.iter().filter(|d| d.name == function).count();
    match owner {
        None => format!("{seat}: {file} declares no module-level `fn {function}`"),
        Some(_) if elsewhere == 0 => {
            format!("{seat}: {file} declares no `fn {function}` at all")
        }
        Some(owner) => format!(
            "{seat}: {file} declares `fn {function}` {elsewhere} time(s), none of them under \
             `{owner}` — the citation names an owner that does not have it"
        ),
    }
}

/// An ambiguous citation is a refusal, not a pick: a resolver that took the first match would
/// keep confirming the seat after the declaration somebody meant was deleted.
fn ambiguous(seat: &str, file: &str, matches: &[&Declaration], blocks: &[Block]) -> String {
    let mut out = format!(
        "{seat}: ambiguous — {} declarations in {file} answer to this citation, so it names \
         none of them:",
        matches.len()
    );
    for found in matches {
        let header = found
            .owner
            .and_then(|index| blocks.get(index))
            .map_or("module level", |block| block.header.as_str());
        let _ = write!(out, "\n    line {}: {header}", found.line);
    }
    out
}

/// `dorc_core` → `spike/crates/core`. The `dorc_` prefix is the crate-name convention, not part
/// of the directory.
fn crate_dir(crate_seg: &str, repo_root: &Path) -> Option<PathBuf> {
    let bare = crate_seg.strip_prefix("dorc_").unwrap_or(crate_seg);
    let dir = repo_root
        .join("spike")
        .join("crates")
        .join(bare.replace('_', "-"));
    dir.is_dir().then_some(dir)
}

/// Walk `text` once, collecting every `trait`/`impl` block and every `fn` declaration with the
/// block that owns it.
///
/// Nesting is tracked by brace depth over comment- and string-stripped lines, so a `fn` inside
/// a function body belongs to nothing and a brace inside a doc-comment or a literal does not
/// move the depth. Rust's own parser this is not — but its failure direction is a loud refusal,
/// never a citation quietly confirmed against the wrong declaration.
fn scan(text: &str) -> (Vec<Block>, Vec<Declaration>) {
    let mut blocks: Vec<Block> = Vec::new();
    let mut declarations: Vec<Declaration> = Vec::new();
    let mut open: Option<(usize, usize)> = None;
    let mut depth: usize = 0;
    for (index, raw) in text.lines().enumerate() {
        let line = index.saturating_add(1);
        let code = strip_comment_and_literals(raw);
        let trimmed = code.trim();
        if depth == 0
            && let Some((owner, header)) = block_header(trimmed)
        {
            open = Some((blocks.len(), depth));
            blocks.push(Block { header, owner });
        } else if let Some(name) = declared_fn(trimmed) {
            let owns = open.filter(|(_, at)| depth == at.saturating_add(1));
            declarations.push(Declaration {
                name,
                line,
                owner: owns.map(|(index, _)| index),
            });
        }
        depth = depth
            .saturating_add(code.matches('{').count())
            .saturating_sub(code.matches('}').count());
        if depth == 0 {
            open = None;
        }
    }
    (blocks, declarations)
}

/// The name a citation reaches this block by, plus the header as written.
///
/// A `trait Name` is reached as `Name`; an `impl Trait for Type` and an inherent `impl Type`
/// are both reached as `Type`, which is exactly what Rust's `Type::method` path syntax does.
fn block_header(line: &str) -> Option<(String, String)> {
    let header = line.trim_end_matches('{').trim().to_owned();
    let bare = line
        .trim_start_matches("pub ")
        .trim_start_matches("pub(crate) ")
        .trim_start_matches("unsafe ");
    if let Some(tail) = bare.strip_prefix("trait ") {
        return Some((base_name(tail)?, header));
    }
    let tail = bare.strip_prefix("impl")?;
    let tail = skip_generics(tail);
    let target = tail.split_once(" for ").map_or(tail, |(_, ty)| ty);
    Some((base_name(target)?, header))
}

/// Step past an `impl`'s generic parameter list, matching angle brackets.
fn skip_generics(tail: &str) -> &str {
    let mut chars = tail.char_indices();
    if chars.next().map(|(_, c)| c) != Some('<') {
        return tail;
    }
    let mut depth = 1usize;
    for (index, c) in chars {
        match c {
            '<' => depth = depth.saturating_add(1),
            '>' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return tail.get(index.saturating_add(1)..).unwrap_or("");
                }
            }
            _ => {}
        }
    }
    tail
}

/// The bare type name a citation spells: `&'a SortedSet<T>` → `SortedSet`.
fn base_name(target: &str) -> Option<String> {
    let name: String = target
        .trim()
        .trim_start_matches('&')
        .split_whitespace()
        .find(|token| !token.starts_with('\'') && *token != "mut" && *token != "dyn")?
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    (!name.is_empty()).then_some(name)
}

/// The name this line declares, if it declares a `fn` — trait signature and inherent
/// definition alike, since a seat may be either.
fn declared_fn(line: &str) -> Option<String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let position = tokens.iter().position(|t| *t == "fn")?;
    let spelled = tokens.get(position.saturating_add(1))?;
    let name: String = spelled
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    let tail = spelled.get(name.len()..).unwrap_or("");
    let declares = tail.is_empty() || tail.starts_with('(') || tail.starts_with('<');
    (declares && !name.is_empty()).then_some(name)
}

/// Drop the line's `//` comment and blank out its string/char literals, so neither can move the
/// brace depth. Crude by design: the file being read is Rust the workspace already compiles.
fn strip_comment_and_literals(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut in_string = false;
    let mut escaped = false;
    let mut previous = '\0';
    for c in line.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        match c {
            '"' => in_string = true,
            '/' if previous == '/' => {
                out.pop();
                break;
            }
            _ => out.push(c),
        }
        previous = c;
    }
    out
}

fn relative(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TWO_JOINS: &str = "\
pub trait Lattice: Clone + Eq {
    fn bottom() -> Self;
    fn join(&self, other: &Self) -> Self;
}

impl<T: Clone + Eq> Lattice for Flat<T> {
    fn join(&self, other: &Self) -> Self {
        let joined = other.clone();
        joined
    }
}

impl<T: Ord + Clone> Lattice for Powerset<T> {
    fn join(&self, other: &Self) -> Self { self.union(other) }
}
";

    #[test]
    fn a_citation_resolves_inside_the_owner_it_names() {
        // The whole repair: three `fn join` declarations, and each citation reaches exactly the
        // one its own segments spell. A file-wide search confirmed all three against whichever
        // came first, which is a citation that cannot go red when its declaration is deleted.
        let (blocks, declarations) = scan(TWO_JOINS);
        let owner_of_join = |owner: &str| {
            declarations
                .iter()
                .filter(|d| d.name == "join" && owner_of(d, &blocks).as_deref() == Some(owner))
                .count()
        };
        assert_eq!(owner_of_join("Lattice"), 1, "the trait's own signature");
        assert_eq!(owner_of_join("Flat"), 1, "the impl for Flat");
        assert_eq!(owner_of_join("Powerset"), 1, "the impl for Powerset");
        assert_eq!(owner_of_join("MapL"), 0, "a combinator with no such impl");
    }

    #[test]
    fn a_body_local_declaration_belongs_to_nothing() {
        // `fn join`'s body here declares a `let`, but a body could equally declare a nested
        // `fn`; counting one would attribute a private helper to the seat and make the citation
        // ambiguous for a reason no reader could see.
        let nested = "\
impl<T> Flat<T> {
    fn join(&self) -> Self {
        fn join(x: u8) -> u8 { x }
        self.clone()
    }
}
";
        let (blocks, declarations) = scan(nested);
        let owned = declarations
            .iter()
            .filter(|d| d.name == "join" && owner_of(d, &blocks).as_deref() == Some("Flat"))
            .count();
        assert_eq!(owned, 1, "the nested helper is not the seat");
    }

    #[test]
    fn an_ambiguous_citation_is_refused_rather_than_picked() {
        // Two declarations answering one citation means the citation names neither. Picking the
        // first would leave the seat green after somebody deleted the one that was meant.
        let twins = "\
impl Flat {
    fn join(&self) -> Self { self.clone() }
}

impl Ord for Flat {
    fn join(&self) -> Self { self.clone() }
}
";
        let (blocks, declarations) = scan(twins);
        let matches: Vec<&Declaration> = declarations
            .iter()
            .filter(|d| d.name == "join" && owner_of(d, &blocks).as_deref() == Some("Flat"))
            .collect();
        assert_eq!(matches.len(), 2);
        let message = ambiguous(
            "dorc_analysis::lattice::Flat::join",
            "f.rs",
            &matches,
            &blocks,
        );
        assert!(message.contains("ambiguous"));
        assert!(message.contains("impl Flat"), "each candidate is named");
        assert!(message.contains("impl Ord for Flat"));
    }

    #[test]
    fn a_declaration_is_recognized_in_both_seat_shapes() {
        // A seat is either a trait's signature or an inherent definition; a checker that saw
        // only one would silently pass a citation naming the other.
        assert_eq!(
            declared_fn("fn join(&self, other: &Self) -> Self;").as_deref(),
            Some("join")
        );
        assert_eq!(
            declared_fn("pub fn insert(&mut self, value: T) -> bool {").as_deref(),
            Some("insert")
        );
        assert_eq!(
            declared_fn("fn from_iter<I: IntoIterator<Item = T>>(i: I)").as_deref(),
            Some("from_iter")
        );
        assert_eq!(declared_fn("let joined = out.get(k).join(v);"), None);
        assert_eq!(
            declared_fn("fn joinery(&self) {}").as_deref(),
            Some("joinery")
        );
    }

    #[test]
    fn an_impl_target_is_read_past_its_generics_and_borrows() {
        assert_eq!(
            block_header("impl<T: Ord + Clone> Lattice for Powerset<T> {").map(|(o, _)| o),
            Some("Powerset".to_owned())
        );
        assert_eq!(
            block_header("impl<'a, T> IntoIterator for &'a SortedSet<T> {").map(|(o, _)| o),
            Some("SortedSet".to_owned())
        );
        assert_eq!(
            block_header("pub trait BoundedLattice: Lattice {").map(|(o, _)| o),
            Some("BoundedLattice".to_owned())
        );
        assert_eq!(
            block_header("impl<K, V> SortedMap<K, V> {").map(|(o, _)| o),
            Some("SortedMap".to_owned())
        );
    }

    #[test]
    fn a_brace_in_a_comment_or_a_literal_does_not_move_the_depth() {
        // Depth is what decides whether a `fn` belongs to the block above it, so a stray brace
        // in prose would silently re-parent every declaration after it.
        let noisy = "\
impl Flat {
    /// Renders as `{}` — a brace in prose {
    fn join(&self) -> Self {
        let _ = \"} {\";
        self.clone()
    }
}
";
        let (blocks, declarations) = scan(noisy);
        assert_eq!(
            declarations
                .iter()
                .filter(|d| owner_of(d, &blocks).as_deref() == Some("Flat"))
                .count(),
            1
        );
    }
}
