use nom::Slice;

use crate::Span;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    /// The offset represents the position of the fragment relatively to
    /// the input of the parser. It starts at offset 0.
    offset: usize,

    /// The line number of the fragment relatively to the input of the
    /// parser. It starts at line 1.
    line: u32,

    column: usize,
}

impl<'a> From<Span<'a>> for Location {
    fn from(span: Span) -> Self {
        Self {
            offset: span.location_offset(),
            line: span.location_line(),
            column: span.get_utf8_column(),
        }
    }
}

impl Location {
    pub fn new(offset: usize, line: u32, column: usize) -> Self {
        Self {
            offset,
            line,
            column,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    start: Location,
    end: Location,
}

impl Range {
    pub fn start(&self) -> &Location {
        &self.start
    }

    pub fn end(&self) -> &Location {
        &self.end
    }
}

impl<'a> From<(Span<'a>, &Range)> for Range {
    fn from(pair: (Span, &Range)) -> Self {
        let (start, end) = pair;
        Self {
            start: start.into(),
            end: end.end().clone(),
        }
    }
}

impl<'a> From<(&Range, Span<'a>)> for Range {
    fn from(pair: (&Range, Span)) -> Self {
        let (start, end) = pair;
        Self {
            start: start.start().clone(),
            end: end.into(),
        }
    }
}

impl<'a> From<(&Range, &Range)> for Range {
    fn from(pair: (&Range, &Range)) -> Self {
        let (start, end) = pair;
        Self {
            start: start.start().clone(),
            end: end.end().clone(),
        }
    }
}

impl<'a> From<(Span<'a>, Span<'a>)> for Range {
    fn from(pair: (Span, Span)) -> Self {
        let (start, end) = pair;
        let last_char_pos = end.char_indices().last().unwrap().0;
        Self {
            start: start.into(),
            end: end.slice(last_char_pos..end.len()).into(),
        }
    }
}

impl<'a> From<(&Span<'a>, &Span<'a>)> for Range {
    fn from(pair: (&Span, &Span)) -> Self {
        let (start, end) = pair;
        Self {
            start: (*start).into(),
            end: end.slice(end.len() - 1..end.len()).into(),
        }
    }
}

impl<'a> From<(&Span<'a>, &puppet_lang::expression::Accessor<Range>)> for Range {
    fn from(pair: (&Span, &puppet_lang::expression::Accessor<Range>)) -> Self {
        let (start, accessor) = pair;
        Self {
            start: (*start).into(),
            end: accessor.extra.end().clone(),
        }
    }
}

impl<'a> From<(&Range, &puppet_lang::expression::Accessor<Range>)> for Range {
    fn from(pair: (&Range, &puppet_lang::expression::Accessor<Range>)) -> Self {
        let (start, accessor) = pair;
        Self {
            start: start.start().clone(),
            end: accessor.extra.end().clone(),
        }
    }
}

impl<'a>
    From<(
        &Span<'a>,
        &Option<puppet_lang::expression::Accessor<Range>>,
        &Span<'a>,
    )> for Range
{
    fn from(
        tuple: (
            &Span,
            &Option<puppet_lang::expression::Accessor<Range>>,
            &Span,
        ),
    ) -> Self {
        let (start, accessor, end_span) = tuple;
        match accessor {
            Some(accessor) => Self::from((start, accessor)),
            None => Self::from((start, end_span)),
        }
    }
}

impl<'a>
    From<(
        &Span<'a>,
        &Option<puppet_lang::expression::Accessor<Range>>,
        &Range,
    )> for Range
{
    fn from(
        tuple: (
            &Span,
            &Option<puppet_lang::expression::Accessor<Range>>,
            &Range,
        ),
    ) -> Self {
        let (start, accessor, end) = tuple;
        match accessor {
            Some(accessor) => Self::from((start, accessor)),
            None => Self::from((*start, end)),
        }
    }
}

impl<'a>
    From<(
        &Range,
        &Option<puppet_lang::expression::Accessor<Range>>,
        &Span<'a>,
    )> for Range
{
    fn from(
        tuple: (
            &Range,
            &Option<puppet_lang::expression::Accessor<Range>>,
            &Span,
        ),
    ) -> Self {
        let (start, accessor, end) = tuple;
        match accessor {
            Some(accessor) => Self::from((start, accessor)),
            None => Self::from((start, *end)),
        }
    }
}

impl<'a>
    From<(
        &Range,
        &Option<puppet_lang::expression::Accessor<Range>>,
        &Range,
    )> for Range
{
    fn from(
        tuple: (
            &Range,
            &Option<puppet_lang::expression::Accessor<Range>>,
            &Range,
        ),
    ) -> Self {
        let (start, accessor, end) = tuple;
        match accessor {
            Some(accessor) => Self::from((start, accessor)),
            None => Self::from((start, end)),
        }
    }
}

impl<'a> From<&puppet_lang::expression::TermVariant<Range>> for Range {
    fn from(v: &puppet_lang::expression::TermVariant<Range>) -> Self {
        match v {
            puppet_lang::expression::TermVariant::String(v) => v.extra.clone(),
            puppet_lang::expression::TermVariant::Float(v) => v.extra.clone(),
            puppet_lang::expression::TermVariant::Integer(v) => v.extra.clone(),
            puppet_lang::expression::TermVariant::Boolean(v) => v.extra.clone(),
            puppet_lang::expression::TermVariant::Array(v) => v.extra.clone(),
            puppet_lang::expression::TermVariant::Identifier(v) => v.extra.clone(),
            puppet_lang::expression::TermVariant::Parens(v) => v.extra.clone(),
            puppet_lang::expression::TermVariant::Map(v) => v.extra.clone(),
            puppet_lang::expression::TermVariant::Variable(v) => v.extra.clone(),
            puppet_lang::expression::TermVariant::RegexpGroupID(v) => v.extra.clone(),
            puppet_lang::expression::TermVariant::Sensitive(v) => v.extra.clone(),
            puppet_lang::expression::TermVariant::TypeSpecitifaction(v) => v.extra.clone(),
            puppet_lang::expression::TermVariant::Regexp(v) => v.extra.clone(),
        }
    }
}

impl<'a> From<&puppet_lang::typing::TypeSpecificationVariant<Range>> for Range {
    fn from(v: &puppet_lang::typing::TypeSpecificationVariant<Range>) -> Self {
        match v {
            puppet_lang::typing::TypeSpecificationVariant::Float(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Integer(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Numeric(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::String(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Pattern(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Regex(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Hash(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Boolean(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Array(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Undef(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Any(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Optional(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Variant(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Enum(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Struct(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::ExternalType(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Sensitive(v) => v.extra.clone(),
            puppet_lang::typing::TypeSpecificationVariant::Tuple(v) => v.extra.clone(),
        }
    }
}

impl Range {
    pub fn new(
        start_offset: usize,
        start_line: u32,
        start_column: usize,
        end_offset: usize,
        end_line: u32,
        end_column: usize,
    ) -> Self {
        Self {
            start: Location::new(start_offset, start_line, start_column),
            end: Location::new(end_offset, end_line, end_column),
        }
    }
}
