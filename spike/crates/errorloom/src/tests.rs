//! The crate's reason to have its own tests (`282:rul-own-crate-own-tests`):
//! the round-trip property (`282` §5) over a seeded-random fake consumer, plus a
//! table exercising every refusal class. The fake (tiny template store + param
//! substitution + arbitrary re-wrap renderer) is the seed of d2's toy consumer,
//! so it drives errorloom ONLY through its public API.

use std::collections::BTreeMap;

use super::*;
use crate::prose::tokenize_located;

type Key = u32;

/// `SplitMix64` — a seeded, injected PRNG (`inv-determinism` applies to tests too:
/// no wallclock, no ambient randomness). Bitwise/wrapping ops only, so the
/// no-arithmetic-side-effects floor holds without a suppression.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: usize) -> usize {
        let Ok(m) = u64::try_from(n) else { return 0 };
        if m == 0 {
            return 0;
        }
        usize::try_from(self.next_u64().checked_rem(m).unwrap_or(0)).unwrap_or(0)
    }

    fn range(&mut self, lo: usize, hi_inclusive: usize) -> usize {
        let span = hi_inclusive.saturating_sub(lo).saturating_add(1);
        lo.saturating_add(self.below(span))
    }

    fn chance(&mut self, num: usize, den: usize) -> bool {
        self.below(den) < num
    }

    fn fresh(&mut self, prefix: char) -> Word {
        Word::new(format!("{prefix}{:x}", self.next_u64()))
    }
}

/// A minimal fake consumer: a catalog of stored templates, per-field param
/// values, and a field render order.
struct Fake {
    catalog: BTreeMap<Key, FieldTemplate>,
    params: BTreeMap<Key, BTreeMap<ParamName, Vec<Word>>>,
    order: Vec<Key>,
}

fn push_chunk(text: &mut String, spans: &mut Vec<Span<Key>>, region: Region<Key>, content: &str) {
    if content.is_empty() {
        return;
    }
    let start = text.len();
    text.push_str(content);
    spans.push(Span {
        range: start..text.len(),
        region,
    });
}

fn paragraph_chunks(
    key: Key,
    paragraph: usize,
    para: &Paragraph,
    pmap: Option<&BTreeMap<ParamName, Vec<Word>>>,
) -> Vec<(Region<Key>, String)> {
    let mut chunks: Vec<(Region<Key>, String)> = Vec::new();
    let mut words: Vec<String> = Vec::new();
    for frag in para.fragments() {
        match frag {
            Fragment::Word(w) => words.push(w.as_str().to_owned()),
            Fragment::Hole(name) => {
                if !words.is_empty() {
                    chunks.push((tl(key, paragraph), words.join(" ")));
                    words = Vec::new();
                }
                let value: Vec<Word> = pmap.and_then(|m| m.get(name)).cloned().unwrap_or_default();
                let joined = value.iter().map(Word::as_str).collect::<Vec<_>>().join(" ");
                chunks.push((pv_region(key, name.as_str()), joined));
            }
        }
    }
    if !words.is_empty() {
        chunks.push((tl(key, paragraph), words.join(" ")));
    }
    chunks
}

impl Fake {
    /// Render to a tagged render. `wrap_seed` varies intra-paragraph separators
    /// (space vs single newline) so the round-trip is tested modulo whitespace.
    fn render(&self, wrap_seed: u64) -> TaggedRender<Key> {
        let mut sep = Rng::new(wrap_seed);
        let mut text = String::new();
        let mut spans: Vec<Span<Key>> = Vec::new();
        for (fi, key) in self.order.iter().enumerate() {
            if fi > 0 {
                push_chunk(&mut text, &mut spans, arr("field-sep"), "\n\n");
            }
            let Some(ft) = self.catalog.get(key) else {
                continue;
            };
            let pmap = self.params.get(key);
            for (p, para) in ft.paragraphs().iter().enumerate() {
                if p > 0 {
                    push_chunk(&mut text, &mut spans, arr("para-blank"), "\n\n");
                }
                let chunks = paragraph_chunks(*key, p, para, pmap);
                let last_idx = chunks.len().saturating_sub(1);
                for (ci, (region, mut content)) in chunks.into_iter().enumerate() {
                    if ci != last_idx {
                        content.push_str(if sep.chance(1, 2) { "\n" } else { " " });
                    }
                    push_chunk(&mut text, &mut spans, region, &content);
                }
            }
        }
        TaggedRender::new(text, spans).expect("fake render must produce a valid span cover")
    }

    fn param_tables(&self) -> ParamTables<Key> {
        let mut pt = ParamTables::new();
        for (key, pmap) in &self.params {
            let mut pv = ParamValues::new();
            for (name, words) in pmap {
                pv.insert(name.clone(), words.clone());
            }
            pt.insert(*key, pv);
        }
        pt
    }
}

fn gen_fake(rng: &mut Rng) -> Fake {
    let num_fields = rng.range(1, 3);
    let mut order = Vec::new();
    let mut catalog = BTreeMap::new();
    let mut params: BTreeMap<Key, BTreeMap<ParamName, Vec<Word>>> = BTreeMap::new();
    for f in 0..num_fields {
        let key = u32::try_from(f).unwrap_or(0);
        order.push(key);

        let param_count = rng.below(3);
        let mut pmap: BTreeMap<ParamName, Vec<Word>> = BTreeMap::new();
        let mut pnames: Vec<ParamName> = Vec::new();
        for pi in 0..param_count {
            let name = ParamName::new(format!("p{f}_{pi}"));
            let count = rng.range(1, 2);
            let words: Vec<Word> = (0..count).map(|_| rng.fresh('v')).collect();
            pmap.insert(name.clone(), words);
            pnames.push(name);
        }

        let paragraph_count = rng.range(1, 2);
        let mut field_paragraphs: Vec<Paragraph> = Vec::new();
        for _ in 0..paragraph_count {
            let count = rng.range(1, 4);
            let mut frags: Vec<Fragment> = Vec::new();
            let mut has_word = false;
            for _ in 0..count {
                if !pnames.is_empty() && rng.chance(3, 10) {
                    frags.push(Fragment::Hole(pnames[rng.below(pnames.len())].clone()));
                } else {
                    frags.push(Fragment::Word(rng.fresh('w')));
                    has_word = true;
                }
            }
            if !has_word {
                frags.insert(0, Fragment::Word(rng.fresh('w')));
            }
            field_paragraphs.push(Paragraph::new(frags));
        }

        catalog.insert(key, FieldTemplate::new(field_paragraphs));
        params.insert(key, pmap);
    }
    Fake {
        catalog,
        params,
        order,
    }
}

/// Build the author's edited text by swapping one template span's words for
/// `new_words` in the baseline token stream, then serializing (`282` §3 form).
fn make_edit(baseline: &TaggedRender<Key>, span_idx: usize, new_words: &[Word]) -> String {
    let Some(span) = baseline.spans().get(span_idx) else {
        return baseline.text().to_owned();
    };
    let range = span.range.clone();
    let mut tokens: Vec<Token> = Vec::new();
    let mut emitted = false;
    for located in tokenize_located(baseline.text()) {
        let in_span = located.start >= range.start && located.start < range.end;
        match located.token {
            Token::Word(_) if in_span => {
                if !emitted {
                    tokens.extend(new_words.iter().cloned().map(Token::Word));
                    emitted = true;
                }
            }
            other => tokens.push(other),
        }
    }
    serialize(&tokens)
}

fn serialize(tokens: &[Token]) -> String {
    let mut out = String::new();
    let mut need_sep = false;
    for token in tokens {
        match token {
            Token::Word(w) => {
                if need_sep {
                    out.push(' ');
                }
                out.push_str(w.as_str());
                need_sep = true;
            }
            Token::ParagraphBreak => {
                out.push_str("\n\n");
                need_sep = false;
            }
        }
    }
    out
}

#[test]
fn edit_confined_to_one_template_region_round_trips() {
    for seed in 0..500u64 {
        let mut rng = Rng::new(seed);
        let mut fake = gen_fake(&mut rng);
        let baseline = fake.render(rng.next_u64());

        let template_spans: Vec<usize> = baseline
            .spans()
            .iter()
            .enumerate()
            .filter(|(_, s)| matches!(s.region, Region::TemplateLiteral { .. }))
            .map(|(i, _)| i)
            .collect();
        if template_spans.is_empty() {
            continue;
        }
        let chosen = template_spans[rng.below(template_spans.len())];
        let new_words: Vec<Word> = (0..rng.range(1, 4)).map(|_| rng.fresh('z')).collect();
        let edited = make_edit(&baseline, chosen, &new_words);

        let params = fake.param_tables();
        let outcome = match promote(&baseline, &edited, &params) {
            Ok(outcome) => outcome,
            Err(refusal) => panic!("seed {seed}: unexpected refusal\n{refusal}"),
        };
        for (key, ft) in outcome.field_edits() {
            fake.catalog.insert(*key, ft.clone());
        }

        let rerender = fake.render(rng.next_u64());
        assert_eq!(
            tokenize(rerender.text()),
            tokenize(&edited),
            "seed {seed}: promote -> regenerate -> re-render did not reproduce the edited words",
        );
    }
}

#[test]
fn edit_after_a_hole_re_holes_and_keeps_structure() {
    let baseline = build(vec![
        (tl(1, 0), "the file "),
        (pv_region(1, "path"), "srcval"),
        (tl(1, 0), " created"),
    ]);
    let params = tables(vec![(1, vec![("path", vec!["srcval"])])]);
    let outcome = promote(&baseline, "the file srcval made", &params).expect("clean prose edit");

    let expected = FieldTemplate::new(vec![Paragraph::new(vec![
        Fragment::Word(Word::new("the")),
        Fragment::Word(Word::new("file")),
        Fragment::Hole(ParamName::new("path")),
        Fragment::Word(Word::new("made")),
    ])]);
    assert_eq!(outcome.field_edits().get(&1), Some(&expected));
}

#[test]
fn no_op_edit_reports_no_field_edits() {
    let baseline = build(vec![(tl(1, 0), "steady words here")]);
    let outcome = promote(&baseline, "steady words here", &ParamTables::new()).expect("no-op");
    assert!(outcome.is_empty());
}

#[test]
fn every_refusal_class_fires() {
    let empty = ParamTables::new();

    let payload = build(vec![
        (tl(1, 0), "the file "),
        (pv_region(1, "path"), "alpha"),
        (tl(1, 0), " here"),
    ]);
    assert_refuses(
        &payload,
        "the file beta here",
        &empty,
        RefusalClass::PayloadEdited,
    );

    let foreign = build(vec![
        (tl(1, 0), "see "),
        (ft_region("detail"), "boomstick"),
        (tl(1, 0), " ok"),
    ]);
    assert_refuses(
        &foreign,
        "see dynamite ok",
        &empty,
        RefusalClass::ForeignEdited,
    );

    let arrangement = build(vec![
        (tl(1, 0), "start"),
        (arr("conn"), " and "),
        (tl(2, 0), "end"),
    ]);
    assert_refuses(
        &arrangement,
        "start or end",
        &empty,
        RefusalClass::ArrangementEdited,
    );

    let boundary = build(vec![(tl(1, 0), "hello "), (pv_region(1, "x"), "world")]);
    assert_refuses(
        &boundary,
        "hello there world",
        &empty,
        RefusalClass::AmbiguousBoundaryInsertion,
    );

    let rehole = build(vec![(tl(1, 0), "alpha beta gamma tail")]);
    let overlapping = tables(vec![(
        1,
        vec![("p", vec!["alpha", "beta"]), ("q", vec!["beta", "gamma"])],
    )]);
    assert_refuses(
        &rehole,
        "alpha beta gamma tail2",
        &overlapping,
        RefusalClass::AmbiguousRehole,
    );

    let contradictory = build(vec![
        (tl(1, 0), "foo"),
        (arr("sep"), " then "),
        (tl(1, 0), "foo"),
    ]);
    assert_refuses(
        &contradictory,
        "bar then baz",
        &empty,
        RefusalClass::ContradictoryEdits,
    );
}

#[test]
fn tokenizer_matches_the_prose_model() {
    let toks = tokenize("one   two\nthree\n\n\nfour");
    assert_eq!(
        toks,
        vec![
            Token::Word(Word::new("one")),
            Token::Word(Word::new("two")),
            Token::Word(Word::new("three")),
            Token::ParagraphBreak,
            Token::Word(Word::new("four")),
        ],
    );
    let prose = Prose::normalize("  a b\n\nc  ");
    assert_eq!(prose.paragraphs().len(), 2);
}

#[test]
fn tagged_render_rejects_a_gap() {
    let spans = vec![
        Span {
            range: 0..3,
            region: tl(1, 0),
        },
        Span {
            range: 4..7,
            region: tl(1, 0),
        },
    ];
    assert!(TaggedRender::new(String::from("abc.def"), spans).is_err());
}

fn build(chunks: Vec<(Region<Key>, &str)>) -> TaggedRender<Key> {
    let mut text = String::new();
    let mut spans = Vec::new();
    for (region, content) in chunks {
        let start = text.len();
        text.push_str(content);
        spans.push(Span {
            range: start..text.len(),
            region,
        });
    }
    TaggedRender::new(text, spans).expect("fixture must be a valid span cover")
}

type ParamSpec<'a> = (Key, Vec<(&'a str, Vec<&'a str>)>);

fn tables(entries: Vec<ParamSpec<'_>>) -> ParamTables<Key> {
    let mut pt = ParamTables::new();
    for (key, params) in entries {
        let mut pv = ParamValues::new();
        for (name, words) in params {
            pv.insert(
                ParamName::new(name),
                words.into_iter().map(Word::new).collect(),
            );
        }
        pt.insert(key, pv);
    }
    pt
}

fn assert_refuses(
    baseline: &TaggedRender<Key>,
    edited: &str,
    params: &ParamTables<Key>,
    expected: RefusalClass,
) {
    match promote(baseline, edited, params) {
        Ok(outcome) => panic!("expected {expected}, got Ok({:?})", outcome.field_edits()),
        Err(refusal) => assert_eq!(refusal.class(), expected, "wrong class; dump:\n{refusal}"),
    }
}

fn tl(key: Key, paragraph: usize) -> Region<Key> {
    Region::TemplateLiteral { key, paragraph }
}

fn pv_region(key: Key, name: &str) -> Region<Key> {
    Region::ParamValue {
        key,
        param: ParamName::new(name),
    }
}

fn ft_region(name: &str) -> Region<Key> {
    Region::ForeignText {
        param: ParamName::new(name),
    }
}

fn arr(slug: &str) -> Region<Key> {
    Region::Arrangement {
        slug: ArrangementSlug::new(slug),
    }
}
