//! The transport engine (`282` §5): `(baseline tagged render + edited text) →
//! per-field prose edits | refusal`. Layer 1, zero Dorc types.
//!
//! The span map is the attribution authority; the word-diff is only alignment.
//! Refusals are blunt (`282:rul-internal-tool-sharp-edges`): no suggestions, no
//! fuzzy matching — a dump of both word streams, the region table, and the
//! offending hunk, and exit nonzero is the consumer's posture.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::ConsumerKey;
use crate::diff::{DiffOp, diff};
use crate::prose::{
    FieldTemplate, Fragment, Located, Paragraph, ParamName, Token, Word, tokenize, tokenize_located,
};
use crate::span::{Region, Span, TaggedRender};

/// The instantiated param values for one field: each declared param mapped to
/// the word-sequence it rendered as in the baseline world (`282` §5 re-holing
/// input). Values must be word-distinctive per `282` §3 or re-holing may refuse.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ParamValues(BTreeMap<ParamName, Vec<Word>>);

impl ParamValues {
    /// An empty table.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Declare `name`'s instantiated value words.
    pub fn insert(&mut self, name: ParamName, words: Vec<Word>) -> &mut Self {
        self.0.insert(name, words);
        self
    }

    /// Whether `name` is declared.
    #[must_use]
    pub fn contains(&self, name: &ParamName) -> bool {
        self.0.contains_key(name)
    }

    fn entries(&self) -> &BTreeMap<ParamName, Vec<Word>> {
        &self.0
    }
}

/// Per-field param tables keyed by consumer key: what each field's holes were
/// instantiated to, so promote can re-hole extracted prose (`282` §5).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ParamTables<K>(BTreeMap<K, ParamValues>);

impl<K> Default for ParamTables<K> {
    fn default() -> Self {
        ParamTables(BTreeMap::new())
    }
}

impl<K: ConsumerKey> ParamTables<K> {
    /// An empty set of tables.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Declare `key`'s param values.
    pub fn insert(&mut self, key: K, values: ParamValues) -> &mut Self {
        self.0.insert(key, values);
        self
    }

    fn values_for(&self, key: &K) -> Option<&ParamValues> {
        self.0.get(key)
    }
}

/// A successful promote: the fields whose prose changed, each mapped to its new
/// stored template. `282` §5 / `28A` §1 — layer-1 reports WHICH keys were edited;
/// the edits-only-in-the-defining-case rule is consumer policy.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PromoteOutcome<K> {
    field_edits: BTreeMap<K, FieldTemplate>,
}

impl<K: ConsumerKey> PromoteOutcome<K> {
    /// The edited fields and their new stored templates.
    #[must_use]
    pub fn field_edits(&self) -> &BTreeMap<K, FieldTemplate> {
        &self.field_edits
    }

    /// The keys that were edited (what the consumer's cross-case policy reads).
    pub fn edited_keys(&self) -> impl Iterator<Item = &K> {
        self.field_edits.keys()
    }

    /// Whether the edit changed no prose at all (a legal no-op).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.field_edits.is_empty()
    }
}

/// The closed set of reasons promote refuses (`282` §5).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RefusalClass {
    /// A `ParamValue` region's words changed — that is payload, not prose.
    PayloadEdited,
    /// A `ForeignText` (passthrough) region's words changed.
    ForeignEdited,
    /// An Arrangement region — or a paragraph break — changed; structure-bless it.
    ArrangementEdited,
    /// An edit could not be attributed to a single template region (a boundary
    /// insertion, or a change straddling two regions).
    AmbiguousBoundaryInsertion,
    /// Re-holing found overlapping / ambiguous param-value matches.
    AmbiguousRehole,
    /// Two instances of one template were edited to different results.
    ContradictoryEdits,
}

impl fmt::Display for RefusalClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            RefusalClass::PayloadEdited => "payload-edited",
            RefusalClass::ForeignEdited => "foreign-edited",
            RefusalClass::ArrangementEdited => "arrangement-edited",
            RefusalClass::AmbiguousBoundaryInsertion => "ambiguous-boundary-insertion",
            RefusalClass::AmbiguousRehole => "ambiguous-rehole",
            RefusalClass::ContradictoryEdits => "contradictory-edits",
        };
        f.write_str(name)
    }
}

/// A baseline token with its region — the attributed-stream row a refusal dumps.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AttributedToken<K> {
    /// The token.
    pub token: Token,
    /// Its baseline region, if the byte was classified.
    pub region: Option<Region<K>>,
}

/// A refusal carrying the material for a blunt dump (`282:rul-internal-tool-
/// sharp-edges`): both word streams, the region table, and the offending hunk.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Refusal<K> {
    class: RefusalClass,
    offending: String,
    baseline: Vec<AttributedToken<K>>,
    edited: Vec<Token>,
    regions: Vec<Span<K>>,
}

impl<K> Refusal<K> {
    /// Which class of refusal this is.
    #[must_use]
    pub fn class(&self) -> RefusalClass {
        self.class
    }

    /// The attributed baseline word stream.
    #[must_use]
    pub fn baseline_stream(&self) -> &[AttributedToken<K>] {
        &self.baseline
    }

    /// The edited word stream.
    #[must_use]
    pub fn edited_stream(&self) -> &[Token] {
        &self.edited
    }

    /// The full region table.
    #[must_use]
    pub fn region_table(&self) -> &[Span<K>] {
        &self.regions
    }
}

fn show_token(token: &Token) -> String {
    match token {
        Token::Word(w) => w.as_str().to_owned(),
        Token::ParagraphBreak => String::from("¶"),
    }
}

impl<K: fmt::Debug> fmt::Display for Refusal<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "errorloom: refusal: {}", self.class)?;
        writeln!(f, "  {}", self.offending)?;
        writeln!(f, "--- baseline (attributed) ---")?;
        for row in &self.baseline {
            match &row.region {
                Some(region) => writeln!(f, "  {:<24}{:?}", show_token(&row.token), region)?,
                None => writeln!(f, "  {:<24}(unclassified)", show_token(&row.token))?,
            }
        }
        writeln!(f, "--- edited ---")?;
        for token in &self.edited {
            writeln!(f, "  {}", show_token(token))?;
        }
        writeln!(f, "--- regions ---")?;
        for span in &self.regions {
            writeln!(
                f,
                "  {}..{}  {:?}",
                span.range.start, span.range.end, span.region
            )?;
        }
        Ok(())
    }
}

impl<K: fmt::Debug> std::error::Error for Refusal<K> {}

/// Borrows the source streams so a refusal's blunt dump is materialized (with
/// clones) ONLY on the error path — the common success path clones nothing.
struct RefusalCtx<'a, K> {
    baseline: &'a TaggedRender<K>,
    located: &'a [Located],
    span_idx: &'a [Option<usize>],
    edited: &'a [Token],
}

impl<K: Clone> RefusalCtx<'_, K> {
    fn refuse(&self, class: RefusalClass, offending: String) -> Refusal<K> {
        let baseline = self
            .located
            .iter()
            .zip(self.span_idx)
            .map(|(l, si)| AttributedToken {
                token: l.token.clone(),
                region: si.and_then(|i| region_of(self.baseline, i)).cloned(),
            })
            .collect();
        Refusal {
            class,
            offending,
            baseline,
            edited: self.edited.to_vec(),
            regions: self.baseline.spans().to_vec(),
        }
    }
}

/// Extract per-field prose edits from an author's edited transcript, or refuse.
///
/// `baseline` is the machine's own tagged re-render; `edited` is the author's
/// text; `params` gives each field's instantiated hole values for re-holing.
///
/// # Errors
/// Returns a [`Refusal`] when an edit touches a non-prose region, cannot be
/// attributed to a single template region, re-holes ambiguously, or contradicts
/// another instance of the same template (`282` §5).
pub fn promote<K: ConsumerKey>(
    baseline: &TaggedRender<K>,
    edited: &str,
    params: &ParamTables<K>,
) -> Result<PromoteOutcome<K>, Refusal<K>> {
    let base_located = tokenize_located(baseline.text());
    let edit_tokens = tokenize(edited);
    let base_span_idx: Vec<Option<usize>> = base_located
        .iter()
        .map(|l| baseline.span_index_at(l.start))
        .collect();

    let ctx = RefusalCtx {
        baseline,
        located: &base_located,
        span_idx: &base_span_idx,
        edited: &edit_tokens,
    };

    let base_tokens: Vec<Token> = base_located.iter().map(|l| l.token.clone()).collect();
    let ops = diff(&base_tokens, &edit_tokens);
    let break_positions: Vec<usize> = base_located
        .iter()
        .filter(|l| matches!(l.token, Token::ParagraphBreak))
        .map(|l| l.start)
        .collect();

    let span_new_words = Attributor {
        baseline,
        base_located: &base_located,
        base_span_idx: &base_span_idx,
        edit_tokens: &edit_tokens,
        span_new_words: seed_template_spans(baseline),
        pending_del: Vec::new(),
        pending_ins: Vec::new(),
        left_anchor: None,
    }
    .run(&ops, &ctx)?;

    let orig_words = original_words(baseline, &base_located, &base_span_idx);
    let keys = edited_keys(baseline, &span_new_words, &orig_words);

    let mut field_edits: BTreeMap<K, FieldTemplate> = BTreeMap::new();
    for key in &keys {
        let param_values = params.values_for(key);
        let mut distinct: Vec<FieldTemplate> = Vec::new();
        for instance in instances_of(key, baseline.spans()) {
            let ft = reconstruct_instance(
                &instance,
                baseline,
                &span_new_words,
                param_values,
                &break_positions,
                &ctx,
            )?;
            if !distinct.contains(&ft) {
                distinct.push(ft);
            }
        }
        if distinct.len() > 1 {
            return Err(ctx.refuse(
                RefusalClass::ContradictoryEdits,
                format!("two instances of one template disagree: {key:?}"),
            ));
        }
        if let Some(ft) = distinct.into_iter().next() {
            field_edits.insert(key.clone(), ft);
        }
    }
    Ok(PromoteOutcome { field_edits })
}

fn region_of<K>(baseline: &TaggedRender<K>, idx: usize) -> Option<&Region<K>> {
    baseline.spans().get(idx).map(|s| &s.region)
}

fn is_template<K>(baseline: &TaggedRender<K>, idx: usize) -> bool {
    matches!(
        region_of(baseline, idx),
        Some(Region::TemplateLiteral { .. })
    )
}

fn seed_template_spans<K: ConsumerKey>(baseline: &TaggedRender<K>) -> BTreeMap<usize, Vec<Word>> {
    baseline
        .spans()
        .iter()
        .enumerate()
        .filter(|(_, s)| matches!(s.region, Region::TemplateLiteral { .. }))
        .map(|(i, _)| (i, Vec::new()))
        .collect()
}

/// Walks the edit script as anchor-bounded change-blocks, checking legality and
/// accumulating each template span's new word list. The unchanged non-template
/// regions (params, arrangement) act as anchors, so a fully-replaced template
/// region is still attributed correctly.
struct Attributor<'a, K> {
    baseline: &'a TaggedRender<K>,
    base_located: &'a [Located],
    base_span_idx: &'a [Option<usize>],
    edit_tokens: &'a [Token],
    span_new_words: BTreeMap<usize, Vec<Word>>,
    pending_del: Vec<usize>,
    pending_ins: Vec<usize>,
    left_anchor: Option<usize>,
}

impl<K: ConsumerKey> Attributor<'_, K> {
    fn run(
        mut self,
        ops: &[DiffOp],
        ctx: &RefusalCtx<'_, K>,
    ) -> Result<BTreeMap<usize, Vec<Word>>, Refusal<K>> {
        for op in ops {
            match *op {
                DiffOp::Delete { base } => self.pending_del.push(base),
                DiffOp::Insert { edit } => self.pending_ins.push(edit),
                DiffOp::Equal { base, edit } => {
                    let right_anchor = self.base_span_idx.get(base).copied().flatten();
                    self.flush(right_anchor, ctx)?;
                    if let (Some(si), Some(Token::Word(w))) =
                        (right_anchor, self.edit_tokens.get(edit))
                        && is_template(self.baseline, si)
                    {
                        self.span_new_words.entry(si).or_default().push(w.clone());
                    }
                    self.left_anchor = right_anchor;
                }
            }
        }
        self.flush(None, ctx)?;
        Ok(self.span_new_words)
    }

    fn flush(
        &mut self,
        right_anchor: Option<usize>,
        ctx: &RefusalCtx<'_, K>,
    ) -> Result<(), Refusal<K>> {
        if self.pending_del.is_empty() && self.pending_ins.is_empty() {
            return Ok(());
        }

        let mut target: Option<usize> = None;
        for &d in &self.pending_del {
            if matches!(
                self.base_located.get(d).map(|l| &l.token),
                Some(Token::ParagraphBreak)
            ) {
                return Err(ctx.refuse(
                    RefusalClass::ArrangementEdited,
                    String::from("a baseline paragraph break was removed"),
                ));
            }
            let si = self.base_span_idx.get(d).copied().flatten();
            match si.and_then(|i| region_of(self.baseline, i)) {
                Some(Region::TemplateLiteral { .. }) => {
                    if let Some(i) = si {
                        match target {
                            None => target = Some(i),
                            Some(t) if t == i => {}
                            Some(_) => {
                                return Err(ctx.refuse(
                                    RefusalClass::AmbiguousBoundaryInsertion,
                                    String::from("an edit straddles two template regions"),
                                ));
                            }
                        }
                    }
                }
                Some(Region::ParamValue { .. }) => {
                    return Err(ctx.refuse(
                        RefusalClass::PayloadEdited,
                        String::from(
                            "a param value's words were edited — that is payload, not prose",
                        ),
                    ));
                }
                Some(Region::ForeignText { .. }) => {
                    return Err(ctx.refuse(
                        RefusalClass::ForeignEdited,
                        String::from("passthrough foreign text was edited"),
                    ));
                }
                Some(Region::Arrangement { .. }) | None => {
                    return Err(ctx.refuse(
                        RefusalClass::ArrangementEdited,
                        String::from("render-owned structure was edited"),
                    ));
                }
            }
        }

        let insert_target = if self.pending_del.is_empty() {
            match (self.left_anchor, right_anchor) {
                (Some(l), Some(r)) if l == r && is_template(self.baseline, l) => Some(l),
                _ => None,
            }
        } else {
            target
        };

        if !self.pending_ins.is_empty() {
            let Some(it) = insert_target else {
                return Err(ctx.refuse(
                    RefusalClass::AmbiguousBoundaryInsertion,
                    String::from(
                        "an insertion could not be attributed to a single template region",
                    ),
                ));
            };
            for ins in std::mem::take(&mut self.pending_ins) {
                match self.edit_tokens.get(ins) {
                    Some(Token::Word(w)) => {
                        self.span_new_words.entry(it).or_default().push(w.clone());
                    }
                    Some(Token::ParagraphBreak) => {
                        return Err(ctx.refuse(
                            RefusalClass::ArrangementEdited,
                            String::from(
                                "a paragraph break was inserted (structure is render-owned at v1)",
                            ),
                        ));
                    }
                    None => {}
                }
            }
        }

        self.pending_del.clear();
        self.pending_ins.clear();
        Ok(())
    }
}

fn original_words<K: ConsumerKey>(
    baseline: &TaggedRender<K>,
    base_located: &[Located],
    base_span_idx: &[Option<usize>],
) -> BTreeMap<usize, Vec<Word>> {
    let mut orig: BTreeMap<usize, Vec<Word>> = BTreeMap::new();
    for (l, si) in base_located.iter().zip(base_span_idx) {
        if let (Token::Word(w), Some(i)) = (&l.token, si)
            && is_template(baseline, *i)
        {
            orig.entry(*i).or_default().push(w.clone());
        }
    }
    orig
}

fn edited_keys<K: ConsumerKey>(
    baseline: &TaggedRender<K>,
    span_new_words: &BTreeMap<usize, Vec<Word>>,
    orig_words: &BTreeMap<usize, Vec<Word>>,
) -> BTreeSet<K> {
    let mut keys: BTreeSet<K> = BTreeSet::new();
    for (idx, span) in baseline.spans().iter().enumerate() {
        if let Region::TemplateLiteral { key, .. } = &span.region {
            let new = span_new_words.get(&idx).cloned().unwrap_or_default();
            let orig = orig_words.get(&idx).cloned().unwrap_or_default();
            if new != orig {
                keys.insert(key.clone());
            }
        }
    }
    keys
}

/// Group a key's spans into instances (`282` §5 "two instances of one template" —
/// the same field rendered more than once). A `TemplateLiteral` starts a new
/// instance iff it is SEPARATED from the previous key-span (not byte-adjacent, so
/// arrangement or another field lies between) AND its paragraph index does not
/// advance — i.e. the render restarts rather than continuing to the next
/// paragraph. Param spans never decide; a hole-split paragraph stays one run.
/// (Edge: a hole sitting BETWEEN two instances attaches to the earlier one and
/// may spuriously refuse; sharp-edges v1 — flagged.)
fn instances_of<K: ConsumerKey>(key: &K, spans: &[Span<K>]) -> Vec<Vec<usize>> {
    let mut instances: Vec<Vec<usize>> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let mut max_para: Option<usize> = None;
    let mut prev_key_idx: Option<usize> = None;

    for (i, span) in spans.iter().enumerate() {
        if span.region.key() != Some(key) {
            continue;
        }
        if let Region::TemplateLiteral { paragraph, .. } = &span.region {
            let separated = prev_key_idx.is_some_and(|p| i != p.saturating_add(1));
            let advances = max_para.is_none_or(|mx| *paragraph > mx);
            if separated && !advances {
                instances.push(std::mem::take(&mut current));
                max_para = None;
            }
            max_para = Some(max_para.map_or(*paragraph, |mx| mx.max(*paragraph)));
        }
        current.push(i);
        prev_key_idx = Some(i);
    }
    if !current.is_empty() {
        instances.push(current);
    }
    instances
}

/// Rebuild one instance's stored field template: template spans contribute their
/// new words (literal `{param}` becomes a hole), param spans contribute a hole.
/// Paragraph boundaries are taken from the baseline's own paragraph-break token
/// positions (not the arrangement's slug), so a hole at a boundary lands in the
/// right paragraph. A final re-hole pass turns stray instantiated values back
/// into holes.
fn reconstruct_instance<K: ConsumerKey>(
    instance: &[usize],
    baseline: &TaggedRender<K>,
    span_new_words: &BTreeMap<usize, Vec<Word>>,
    param_values: Option<&ParamValues>,
    break_positions: &[usize],
    ctx: &RefusalCtx<'_, K>,
) -> Result<FieldTemplate, Refusal<K>> {
    let mut paragraphs: Vec<Vec<Fragment>> = Vec::new();
    let mut current: Vec<Fragment> = Vec::new();
    let mut prev_end: Option<usize> = None;

    for &i in instance {
        let Some(span) = baseline.spans().get(i) else {
            continue;
        };
        if let Some(pe) = prev_end
            && break_positions
                .iter()
                .any(|bp| *bp >= pe && *bp < span.range.start)
        {
            paragraphs.push(std::mem::take(&mut current));
        }
        match &span.region {
            Region::TemplateLiteral { .. } => {
                for w in span_new_words.get(&i).cloned().unwrap_or_default() {
                    match literal_hole(&w, param_values) {
                        Some(name) => current.push(Fragment::Hole(name)),
                        None => current.push(Fragment::Word(w)),
                    }
                }
            }
            Region::ParamValue { param, .. } => current.push(Fragment::Hole(param.clone())),
            _ => {}
        }
        prev_end = Some(span.range.end);
    }
    paragraphs.push(current);

    rehole(paragraphs, param_values, ctx).map(FieldTemplate::new)
}

fn literal_hole(word: &Word, param_values: Option<&ParamValues>) -> Option<ParamName> {
    let inner = word.as_str().strip_prefix('{')?.strip_suffix('}')?;
    if inner.is_empty() {
        return None;
    }
    let name = ParamName::new(inner);
    match param_values {
        Some(v) if v.contains(&name) => Some(name),
        _ => None,
    }
}

fn rehole<K: ConsumerKey>(
    paragraphs: Vec<Vec<Fragment>>,
    param_values: Option<&ParamValues>,
    ctx: &RefusalCtx<'_, K>,
) -> Result<Vec<Paragraph>, Refusal<K>> {
    let Some(values) = param_values else {
        return Ok(paragraphs.into_iter().map(Paragraph::new).collect());
    };
    let mut out: Vec<Paragraph> = Vec::new();
    for frags in paragraphs {
        out.push(Paragraph::new(rehole_paragraph(
            frags,
            values.entries(),
            ctx,
        )?));
    }
    Ok(out)
}

fn rehole_paragraph<K: ConsumerKey>(
    frags: Vec<Fragment>,
    values: &BTreeMap<ParamName, Vec<Word>>,
    ctx: &RefusalCtx<'_, K>,
) -> Result<Vec<Fragment>, Refusal<K>> {
    let mut matches: Vec<(usize, usize, ParamName)> = Vec::new();
    for (name, value) in values {
        if value.is_empty() {
            continue;
        }
        let mut start = 0;
        while start < frags.len() {
            let end = start.saturating_add(value.len());
            if end <= frags.len() && window_matches(&frags, start, value) {
                matches.push((start, end, name.clone()));
                start = end;
            } else {
                start = start.saturating_add(1);
            }
        }
    }
    if matches.is_empty() {
        return Ok(frags);
    }

    matches.sort_by_key(|m| m.0);
    let mut prev_end: Option<usize> = None;
    for (s, e, _) in &matches {
        if prev_end.is_some_and(|pe| *s < pe) {
            return Err(ctx.refuse(
                RefusalClass::AmbiguousRehole,
                String::from("declared param values overlap in the extracted prose"),
            ));
        }
        prev_end = Some(*e);
    }

    let mut result: Vec<Fragment> = Vec::new();
    let mut i = 0;
    let mut mi = 0;
    while i < frags.len() {
        if let Some((s, e, name)) = matches.get(mi)
            && i == *s
        {
            result.push(Fragment::Hole(name.clone()));
            i = *e;
            mi = mi.saturating_add(1);
            continue;
        }
        if let Some(fr) = frags.get(i) {
            result.push(fr.clone());
        }
        i = i.saturating_add(1);
    }
    Ok(result)
}

fn window_matches(frags: &[Fragment], start: usize, value: &[Word]) -> bool {
    for (k, vw) in value.iter().enumerate() {
        match frags.get(start.saturating_add(k)) {
            Some(Fragment::Word(w)) if w == vw => {}
            _ => return false,
        }
    }
    true
}
