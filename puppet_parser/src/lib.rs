pub mod argument;
pub mod class;
pub mod common;
pub mod double_quoted;
pub mod expression;
pub mod identifier;
pub mod regex;
pub mod resource_collection;
pub mod single_quoted;
pub mod statement;
pub mod term;
pub mod toplevel;
pub mod typing;

use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq)]
pub struct ParseError<'a> {
    span: Span<'a>,
    message: Option<String>,
    url: Option<String>,
}

impl<'a> ParseError<'a> {
    pub fn new(message: String, span: Span<'a>, url: Option<String>) -> Self {
        Self {
            span,
            message: Some(message),
            url,
        }
    }

    pub fn protect_with_url<O, M, F>(
        mut message_generator: M,
        mut parser: F,
    ) -> impl FnMut(Span<'a>) -> IResult<O>
    where
        M: FnMut(Span<'a>) -> (String, &str) + Copy,
        F: nom::Parser<Span<'a>, O, ParseError<'a>>,
        O: Clone,
    {
        move |input: Span| {
            parser.parse(input).map_err(|err| match err {
                nom::Err::Error(_err) => {
                    let (err, url) = message_generator(input);
                    nom::Err::Failure(ParseError::new(err, input, Some(url.to_string())))
                }
                e => e,
            })
        }
    }

    pub fn protect<O, M, F>(
        mut message_generator: M,
        mut parser: F,
    ) -> impl FnMut(Span<'a>) -> IResult<O>
    where
        M: FnMut(Span<'a>) -> String + Copy,
        F: nom::Parser<Span<'a>, O, ParseError<'a>>,
        O: Clone,
    {
        move |input: Span| {
            parser.parse(input).map_err(|err| match err {
                nom::Err::Error(_err) => {
                    let err = if input.is_empty() {
                        "Unexpected EOF".to_string()
                    } else {
                        message_generator(input)
                    };
                    nom::Err::Failure(ParseError::new(err, input, None))
                }
                e => e,
            })
        }
    }

    pub fn fatal<O>(message: String, span: Span<'a>) -> IResult<O>
    where
        O: Clone,
    {
        Err(nom::Err::Failure(ParseError::new(message, span, None)))
    }

    pub fn span(&self) -> &Span<'a> {
        &self.span
    }

    pub fn message(&self) -> &Option<String> {
        &self.message
    }

    pub fn url(&self) -> &Option<String> {
        &self.url
    }
}

impl<'a> std::fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(message) = &self.message {
            write!(f, "{}", message)?
        }
        write!(
            f,
            " at line {} column {}",
            self.span.location_line(),
            self.span.get_utf8_column()
        )
    }
}

// That's what makes it nom-compatible.
impl<'a> nom::error::ParseError<Span<'a>> for ParseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: nom::error::ErrorKind) -> Self {
        Self::new(format!("parse error {:?}", kind), input, None)
    }

    fn append(_input: Span<'a>, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        Self::new(format!("unexpected character '{}'", c), input, None)
    }
}

impl<'a> nom::error::FromExternalError<LocatedSpan<&'a str>, std::num::ParseIntError>
    for ParseError<'a>
{
    fn from_external_error(
        span: LocatedSpan<&'a str>,
        _kind: nom::error::ErrorKind,
        e: std::num::ParseIntError,
    ) -> Self {
        Self {
            span,
            message: Some(format!("{}", e)),
            url: None,
        }
    }
}

pub type IResult<'a, O> = nom::IResult<Span<'a>, O, ParseError<'a>>;

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

impl<'a> From<(Span<'a>, Span<'a>)> for Range {
    fn from(pair: (Span, Span)) -> Self {
        let (start, end) = pair;
        Self {
            start: start.into(),
            end: end.into(),
        }
    }
}
