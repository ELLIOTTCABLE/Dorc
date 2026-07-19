//! The prose model: words and paragraphs, the read-in normalizer, and the
//! with-holes stored form (`282` §3 / §5).
//!
//! `282:rul-words-and-paragraphs-only`: the only authored value is an ordered
//! series of words grouped into paragraphs. All other formatting is render-owned
//! and discarded at read-in, because LLMs wrap words strangely and that noise
//! must die at the boundary.

/// A whitespace-delimited authored word — the atomic unit of prose. By
/// construction a word carries no interior whitespace (the tokenizer never
/// produces one); consumers building param values must pass one word each.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Word(String);

impl Word {
    /// Wrap a word. The caller owns the no-interior-whitespace invariant.
    pub fn new(text: impl Into<String>) -> Self {
        Word(text.into())
    }

    /// The word's text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A param (hole) name within a template. errorloom compares param names by
/// value and never interprets them (`282` §4 `ParamValue` vocabulary).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ParamName(String);

impl ParamName {
    /// Wrap a param name.
    pub fn new(name: impl Into<String>) -> Self {
        ParamName(name.into())
    }

    /// The name's text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One lexical unit of the prose stream. Paragraph breaks are first-class tokens
/// so the word-diff aligner can align across them (`282` §5).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Token {
    /// A word.
    Word(Word),
    /// A boundary between two paragraphs (two-plus newlines in the raw text).
    ParagraphBreak,
}

/// A token together with its byte offset into the source text — the anchor the
/// span map is queried with during attribution.
pub(crate) struct Located {
    pub(crate) token: Token,
    pub(crate) start: usize,
}

/// Tokenize raw text into the prose stream per `282` §3: within a paragraph every
/// whitespace run (including single newlines) is merely a word separator; a run
/// containing two-plus newlines is a paragraph break. A leading break (before any
/// word) is dropped — there is no paragraph before the first.
pub(crate) fn tokenize_located(text: &str) -> Vec<Located> {
    let mut out: Vec<Located> = Vec::new();
    let mut word: Option<(usize, String)> = None;
    let mut gap_start: Option<usize> = None;
    let mut gap_newlines: usize = 0;
    for (idx, ch) in text.char_indices() {
        if ch.is_whitespace() {
            if let Some((start, s)) = word.take() {
                out.push(Located {
                    token: Token::Word(Word(s)),
                    start,
                });
            }
            if gap_start.is_none() {
                gap_start = Some(idx);
            }
            if ch == '\n' {
                gap_newlines = gap_newlines.saturating_add(1);
            }
        } else {
            if let Some(gs) = gap_start.take() {
                if gap_newlines >= 2 && !out.is_empty() {
                    out.push(Located {
                        token: Token::ParagraphBreak,
                        start: gs,
                    });
                }
                gap_newlines = 0;
            }
            match &mut word {
                Some((_, s)) => s.push(ch),
                None => word = Some((idx, String::from(ch))),
            }
        }
    }
    if let Some((start, s)) = word.take() {
        out.push(Located {
            token: Token::Word(Word(s)),
            start,
        });
    }
    out
}

/// Tokenize raw text into the prose stream (`282` §3). The offset-free surface
/// consumers read author prose with.
#[must_use]
pub fn tokenize(text: &str) -> Vec<Token> {
    tokenize_located(text)
        .into_iter()
        .map(|l| l.token)
        .collect()
}

/// Author prose in `282` §3 canonical form: ordered paragraphs of ordered words,
/// all render-owned layout collapsed away. Catalog prose fields store this form
/// (holes are layered on by re-holing — see [`FieldTemplate`]).
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Prose(Vec<Vec<Word>>);

impl Prose {
    /// Normalize raw text to canonical prose.
    #[must_use]
    pub fn normalize(text: &str) -> Self {
        let mut paragraphs: Vec<Vec<Word>> = Vec::new();
        let mut current: Vec<Word> = Vec::new();
        for tok in tokenize(text) {
            match tok {
                Token::Word(w) => current.push(w),
                Token::ParagraphBreak => paragraphs.push(std::mem::take(&mut current)),
            }
        }
        if !current.is_empty() {
            paragraphs.push(current);
        }
        Prose(paragraphs)
    }

    /// The paragraphs, each a word list.
    #[must_use]
    pub fn paragraphs(&self) -> &[Vec<Word>] {
        &self.0
    }
}

/// One piece of a stored template: an authored word, or a hole naming a param
/// interpolated at render time. Re-holing (`282` §5) turns rendered param values
/// back into holes.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Fragment {
    /// A literal authored word.
    Word(Word),
    /// A hole to be filled by the named param at render.
    Hole(ParamName),
}

/// A paragraph of a stored template.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Paragraph(Vec<Fragment>);

impl Paragraph {
    /// Build a paragraph from its fragments.
    #[must_use]
    pub fn new(fragments: Vec<Fragment>) -> Self {
        Paragraph(fragments)
    }

    /// The paragraph's fragments in order.
    #[must_use]
    pub fn fragments(&self) -> &[Fragment] {
        &self.0
    }
}

/// A catalog prose field in stored form: ordered paragraphs of word/hole
/// fragments. `promote` emits one per edited field for the consumer to write
/// into its catalog (`28A` §1: errorloom owns extraction, the consumer owns the
/// catalog).
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct FieldTemplate(Vec<Paragraph>);

impl FieldTemplate {
    /// Build a field template from its paragraphs.
    #[must_use]
    pub fn new(paragraphs: Vec<Paragraph>) -> Self {
        FieldTemplate(paragraphs)
    }

    /// The paragraphs in order.
    #[must_use]
    pub fn paragraphs(&self) -> &[Paragraph] {
        &self.0
    }
}
