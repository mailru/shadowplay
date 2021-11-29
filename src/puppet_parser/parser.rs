use lazy_static::__Deref;
use nom;
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq)]
pub struct ParseError<'a> {
    span: Span<'a>,
    message: Option<String>,
}

impl<'a> ParseError<'a> {
    pub fn new(message: String, span: Span<'a>) -> Self {
        Self {
            span,
            message: Some(message),
        }
    }

    pub fn protect<O, M, F>(
        mut message_generator: M,
        mut parser: F,
    ) -> impl FnMut(Span<'a>) -> IResultUnmarked<O>
    where
        M: FnMut(Span<'a>) -> String + Copy,
        F: nom::Parser<Span<'a>, O, ParseError<'a>>,
        O: Clone,
    {
        move |input: Span| {
            // println!("PARSING PROTECTED: {:?}", input);
            parser.parse(input).map_err(|err| match err {
                nom::Err::Error(_err) => {
                    let err = if input.is_empty() {
                        "Unexpected EOF".to_string()
                    } else {
                        message_generator(input)
                    };
                    nom::Err::Failure(ParseError::new(err, input))
                }
                e => e,
            })
        }
    }

    pub fn fatal<O>(message: String, span: Span<'a>) -> IResultUnmarked<O>
    where
        O: Clone,
    {
        Err(nom::Err::Failure(ParseError::new(message, span)))
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
        Self::new(format!("parse error {:?}", kind), input)
    }

    fn append(_input: Span<'a>, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        Self::new(format!("unexpected character '{}'", c), input)
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
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Marked<T: Clone> {
    pub data: T,
    pub line: u32,
    pub column: usize,
}

impl<T: Clone> Marked<T> {
    pub fn new(span: &Span, data: T) -> Self {
        Self {
            data,
            line: span.location_line(),
            column: span.get_column(),
        }
    }

    pub fn map<U: Clone, F: FnOnce(T) -> U>(self, f: F) -> Marked<U> {
        let line = self.line;
        let column = self.column;
        let data = f(self.data);
        Marked { data, line, column }
    }
}

impl<'a> From<LocatedSpan<&'a str>> for Marked<&'a str> {
    fn from(val: LocatedSpan<&'a str>) -> Self {
        Marked {
            data: val.deref(),
            line: val.location_line(),
            column: val.get_column(),
        }
    }
}

impl<O: Clone> Marked<O> {
    pub fn parse<'a, F>(mut parser: F) -> impl FnMut(Span<'a>) -> IResult<O>
    where
        F: nom::Parser<Span<'a>, O, ParseError<'a>>,
    {
        move |input| nom::combinator::map(|i| parser.parse(i), |v| Marked::new(&input, v))(input)
    }
}

pub type IResult<'a, O> = nom::IResult<Span<'a>, Marked<O>, ParseError<'a>>;

pub type IResultUnmarked<'a, O> = nom::IResult<Span<'a>, O, ParseError<'a>>;

pub type IResultUnit<'a> = nom::IResult<Span<'a>, (), ParseError<'a>>;
