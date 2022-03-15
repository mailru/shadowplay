use crate::Printer;
use pretty::RcDoc;

fn with_min_max<'a, T: crate::Printer>(
    name: &'static str,
    min: &'a Option<T>,
    max: &'a Option<T>,
) -> RcDoc<'a, ()> {
    let args = match (min, max) {
        (None, None) => RcDoc::nil(),
        (Some(min), None) => RcDoc::text("[")
            .append(RcDoc::softline_())
            .append(min.to_doc())
            .append(RcDoc::softline_())
            .append(RcDoc::text("]"))
            .group()
            .nest(2),
        (None, Some(max)) => RcDoc::text("[")
            .append(RcDoc::softline_())
            .append(RcDoc::text("default"))
            .append(RcDoc::softline_())
            .append(RcDoc::text(","))
            .append(RcDoc::line())
            .append(max.to_doc())
            .append(RcDoc::text("]"))
            .group()
            .nest(2),
        (Some(min), Some(max)) => RcDoc::text("[")
            .append(RcDoc::softline_())
            .append(min.to_doc())
            .append(RcDoc::softline_())
            .append(RcDoc::text(","))
            .append(RcDoc::line())
            .append(max.to_doc())
            .append(RcDoc::text("]"))
            .group()
            .nest(2),
    };

    RcDoc::text(name).append(args)
}

impl<EXTRA> Printer for puppet_lang::typing::Pattern<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("Pattern")
            .append(RcDoc::text("["))
            .append(RcDoc::softline())
            .append(
                RcDoc::intersperse(
                    self.list
                        .iter()
                        .map(|x| x.to_doc().append(RcDoc::text(","))),
                    RcDoc::softline(),
                )
                .group(),
            )
            .nest(2)
            .append(RcDoc::softline())
            .append(RcDoc::text("]"))
            .group()
    }
}

impl<EXTRA> Printer for puppet_lang::typing::Regex<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("Regex")
            .append(RcDoc::text("["))
            .append(RcDoc::softline())
            .append(self.data.to_doc())
            .nest(2)
            .append(RcDoc::softline())
            .append(RcDoc::text("]"))
            .group()
    }
}

impl<EXTRA> Printer for puppet_lang::typing::TypeOptionalVariant<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            puppet_lang::typing::TypeOptionalVariant::TypeSpecification(v) => v.to_doc(),
            puppet_lang::typing::TypeOptionalVariant::Term(v) => crate::term::to_doc(v, false),
        }
    }
}

impl<EXTRA> Printer for puppet_lang::typing::TypeOptional<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("Optional")
            .append(RcDoc::text("["))
            .append(RcDoc::softline_())
            .append(self.value.to_doc())
            .nest(2)
            .append(RcDoc::softline())
            .append(RcDoc::text("]"))
            .group()
    }
}

impl<EXTRA> Printer for puppet_lang::typing::TypeArray<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let args = match &self.inner {
            None => vec![],
            Some(inner) => match (&self.min, &self.max) {
                (None, None) => vec![inner.to_doc()],
                (None, Some(max)) => vec![inner.to_doc(), RcDoc::text("default"), max.to_doc()],
                (Some(min), None) => vec![inner.to_doc(), min.to_doc()],
                (Some(min), Some(max)) => vec![inner.to_doc(), min.to_doc(), max.to_doc()],
            },
        };

        let args = if args.is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::text("[")
                .append(RcDoc::softline())
                .append(
                    RcDoc::intersperse(
                        args.into_iter().map(|x| x.append(RcDoc::text(","))),
                        RcDoc::softline(),
                    )
                    .group(),
                )
                .nest(2)
                .append(RcDoc::softline())
                .append(RcDoc::text("]"))
                .group()
        };

        RcDoc::text("Array").append(args).group()
    }
}

impl<EXTRA> Printer for puppet_lang::typing::Variant<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("Variant")
            .append(RcDoc::text("["))
            .append(RcDoc::softline())
            .append(
                RcDoc::intersperse(
                    self.list
                        .iter()
                        .map(|x| x.to_doc().append(RcDoc::text(","))),
                    RcDoc::softline(),
                )
                .group(),
            )
            .nest(2)
            .append(RcDoc::softline())
            .append(RcDoc::text("]"))
            .group()
    }
}

impl<EXTRA> Printer for puppet_lang::typing::Enum<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("Enum")
            .append(RcDoc::text("["))
            .append(RcDoc::softline())
            .append(
                RcDoc::intersperse(
                    self.list
                        .iter()
                        .map(|x| crate::term::to_doc(x, false).append(RcDoc::text(","))),
                    RcDoc::softline(),
                )
                .group(),
            )
            .nest(2)
            .append(RcDoc::softline())
            .append(RcDoc::text("]"))
            .group()
    }
}

impl<EXTRA> Printer for puppet_lang::typing::ExternalType<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let args = if self.arguments.is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::text("[")
                .append(RcDoc::softline())
                .append(
                    RcDoc::intersperse(
                        self.arguments
                            .iter()
                            .map(|x| crate::expression::to_doc(x, false).append(RcDoc::text(","))),
                        RcDoc::softline(),
                    )
                    .group(),
                )
                .nest(2)
                .append(RcDoc::softline())
                .append(RcDoc::text("]"))
                .group()
        };

        RcDoc::intersperse(self.name.iter().map(RcDoc::text), RcDoc::text("::")).append(args)
    }
}

impl<EXTRA> Printer for puppet_lang::typing::TypeHash<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let mut args = match (&self.key, &self.value) {
            (None, None) => vec![],
            (Some(key), Some(value)) => vec![key.to_doc(), value.to_doc()],
            _ =>
            // If you specify a key type, a value type is mandatory.
            {
                unreachable!()
            }
        };

        match (&self.min, &self.max) {
            (None, None) => (),
            (None, Some(max)) => args.extend(vec![RcDoc::text("default"), max.to_doc()]),
            (Some(min), None) => args.push(min.to_doc()),
            (Some(min), Some(max)) => args.extend(vec![min.to_doc(), max.to_doc()]),
        };

        let args = if args.is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::text("[")
                .append(RcDoc::softline())
                .append(
                    RcDoc::intersperse(
                        args.into_iter().map(|x| x.append(RcDoc::text(","))),
                        RcDoc::softline(),
                    )
                    .group(),
                )
                .nest(2)
                .append(RcDoc::softline())
                .append(RcDoc::text("]"))
                .group()
        };

        RcDoc::text("Hash").append(args).group()
    }
}

impl<EXTRA> Printer for puppet_lang::typing::TypeStructKey<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            puppet_lang::typing::TypeStructKey::String(v) => v.to_doc(),
            puppet_lang::typing::TypeStructKey::Optional(v) => RcDoc::text("Optional")
                .append(RcDoc::text("["))
                .append(RcDoc::softline_())
                .append(v.value.to_doc())
                .nest(2)
                .append(RcDoc::softline())
                .append(RcDoc::text("]"))
                .group(),
            puppet_lang::typing::TypeStructKey::NotUndef(v) => RcDoc::text("NotUndef")
                .append(RcDoc::text("["))
                .append(RcDoc::softline_())
                .append(v.value.to_doc())
                .nest(2)
                .append(RcDoc::softline())
                .append(RcDoc::text("]"))
                .group(),
        }
    }
}
impl<EXTRA> Printer for puppet_lang::typing::TypeStructKV<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        self.key
            .to_doc()
            .append(RcDoc::softline())
            .append(RcDoc::text("=>"))
            .append(RcDoc::softline())
            .append(self.value.to_doc())
            .group()
    }
}
impl<EXTRA> Printer for puppet_lang::typing::TypeStruct<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("Struct")
            .append(RcDoc::text("[{"))
            .append(RcDoc::softline())
            .append(
                RcDoc::intersperse(
                    self.keys
                        .value
                        .iter()
                        .map(|x| x.to_doc().append(RcDoc::text(","))),
                    RcDoc::softline(),
                )
                .group()
                .append(self.keys.last_comment.to_doc()),
            )
            .nest(2)
            .append(RcDoc::softline())
            .append(RcDoc::text("}]"))
            .group()
    }
}

impl<EXTRA> Printer for puppet_lang::typing::TypeSensitive<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let inner = match &self.value {
            puppet_lang::typing::TypeSensitiveVariant::TypeSpecification(v) => v.to_doc(),
            puppet_lang::typing::TypeSensitiveVariant::Term(v) => crate::term::to_doc(v, false),
        };

        RcDoc::text("Sensitive")
            .append(RcDoc::text("["))
            .append(RcDoc::softline())
            .append(inner)
            .nest(2)
            .append(RcDoc::softline())
            .append(RcDoc::text("]"))
            .group()
    }
}

impl<EXTRA> Printer for puppet_lang::typing::TypeTuple<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let mut args: Vec<_> = self.list.iter().map(|v| v.to_doc()).collect();
        match (&self.min, &self.max) {
            (None, None) => (),
            (None, Some(max)) => args.extend(vec![RcDoc::text("default"), max.to_doc()]),
            (Some(min), None) => args.push(min.to_doc()),
            (Some(min), Some(max)) => args.extend(vec![min.to_doc(), max.to_doc()]),
        };

        let args = RcDoc::text("[")
            .append(RcDoc::softline())
            .append(
                RcDoc::intersperse(
                    args.into_iter().map(|x| x.append(RcDoc::text(","))),
                    RcDoc::softline(),
                )
                .group(),
            )
            .nest(2)
            .append(RcDoc::softline())
            .append(RcDoc::text("]"))
            .group();

        RcDoc::text("Tuple").append(args).group()
    }
}

impl<EXTRA> Printer for puppet_lang::typing::TypeSpecificationVariant<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            puppet_lang::typing::TypeSpecificationVariant::Float(v) => {
                with_min_max("Float", &v.min, &v.max)
            }
            puppet_lang::typing::TypeSpecificationVariant::Integer(v) => {
                with_min_max("Integer", &v.min, &v.max)
            }
            puppet_lang::typing::TypeSpecificationVariant::Numeric(_) => RcDoc::text("Numeric"),
            puppet_lang::typing::TypeSpecificationVariant::String(v) => {
                with_min_max("String", &v.min, &v.max)
            }
            puppet_lang::typing::TypeSpecificationVariant::Pattern(v) => v.to_doc(),
            puppet_lang::typing::TypeSpecificationVariant::Regex(v) => v.to_doc(),
            puppet_lang::typing::TypeSpecificationVariant::Hash(v) => v.to_doc(),
            puppet_lang::typing::TypeSpecificationVariant::Boolean(_) => RcDoc::text("Boolean"),
            puppet_lang::typing::TypeSpecificationVariant::Array(v) => v.to_doc(),
            puppet_lang::typing::TypeSpecificationVariant::Undef(_) => RcDoc::text("Undef"),
            puppet_lang::typing::TypeSpecificationVariant::Any(_) => RcDoc::text("Any"),
            puppet_lang::typing::TypeSpecificationVariant::Optional(v) => v.to_doc(),
            puppet_lang::typing::TypeSpecificationVariant::Variant(v) => v.to_doc(),
            puppet_lang::typing::TypeSpecificationVariant::Enum(v) => v.to_doc(),
            puppet_lang::typing::TypeSpecificationVariant::Struct(v) => v.to_doc(),
            puppet_lang::typing::TypeSpecificationVariant::ExternalType(v) => v.to_doc(),
            puppet_lang::typing::TypeSpecificationVariant::Sensitive(v) => v.to_doc(),
            puppet_lang::typing::TypeSpecificationVariant::Tuple(v) => v.to_doc(),
        }
    }
}

impl<EXTRA> Printer for puppet_lang::typing::TypeSpecification<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        crate::comment::comment_or(&self.comment, RcDoc::hardline(), RcDoc::nil())
            .append(self.data.to_doc())
    }
}

#[test]
fn test_idempotence_short() {
    let cases = vec![
        "Float",
        "Float[1]",
        "Float[1, 2]",
        "Float[\n  default,\n  2]",
        "Integer",
        "String[1,\n  2]",
        "Pattern[\n  /a/, /b/,\n]",
        "Regex[ // ]",
        "Regex[\n  /aaaaaaaaaaa/\n]",
        "Optional[\n  #comment\n  Regex[\n    /aaaaaaaaaaa/\n  ] ]",
        "Array[\n  Integer,\n]",
        "Array[\n  Integer,\n  2, ]",
        "Array[\n  Integer,\n  2, 4, ]",
        "Variant[\n  String[1,\n    2],\n  Integer,\n]",
        "Enum[ 1,\n  aaaaaa,\n  3, ]",
        "Some::Type",
        "Some::Type[\n  1, ]",
        "Hash[\n  String,\n  Integer,\n  default,\n  1, ]",
        "Struct[{ a\n  =>\n  Integer,\n}]",
        "Struct[{\n  Optional[\n    a ] =>\n  Integer,\n}]",
        "Struct[{\n  NotUndef[\n    a ] =>\n  Integer,\n}]",
        "Sensitive[\n  1 ]",
        "Sensitive[\n  String ]",
        "Tuple[\n  String,\n  Integer,\n  default,\n  100, ]",
    ];

    for case in cases {
        let (_, v) =
            puppet_parser::typing::parse_type_specification(puppet_parser::Span::new(case))
                .unwrap();

        let mut w = Vec::new();
        v.to_doc().render(11, &mut w).unwrap();
        let generated = String::from_utf8(w).unwrap();
        println!("{} ==>\n------\n{}\n------", case, generated);

        assert_eq!(&generated, case)
    }
}
