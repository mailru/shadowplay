use crate::puppet_pp_printer::Printer;
use pretty::RcDoc;

use super::common;

fn with_min_max<'a, T: crate::puppet_pp_printer::Printer>(
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

impl<EXTRA> Printer for crate::puppet_lang::typing::Pattern<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("Pattern")
            .append(RcDoc::text("["))
            .append(super::common::multiline_list(&self.list, None, |x| {
                x.to_doc()
            }))
            .append(RcDoc::text("]"))
            .group()
    }
}

impl<EXTRA> Printer for crate::puppet_lang::typing::Regex<EXTRA> {
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

impl<EXTRA> Printer for crate::puppet_lang::typing::TypeOptionalVariant<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            crate::puppet_lang::typing::TypeOptionalVariant::TypeSpecification(v) => v.to_doc(),
            crate::puppet_lang::typing::TypeOptionalVariant::Term(v) => {
                crate::puppet_pp_printer::term::to_doc(v, false)
            }
        }
    }
}

impl<EXTRA> Printer for crate::puppet_lang::typing::TypeOptional<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("Optional")
            .append(RcDoc::text("["))
            .append(RcDoc::softline_())
            .append(self.value.to_doc())
            .nest(2)
            .append(RcDoc::softline_())
            .append(RcDoc::text("]"))
            .group()
    }
}

fn has_args<EXTRA>(t: &crate::puppet_lang::typing::TypeSpecification<EXTRA>) -> bool {
    match &t.data {
        crate::puppet_lang::typing::TypeSpecificationVariant::Float(v) => {
            v.min.is_some() || v.max.is_some()
        }
        crate::puppet_lang::typing::TypeSpecificationVariant::Integer(v) => {
            v.min.is_some() || v.max.is_some()
        }
        crate::puppet_lang::typing::TypeSpecificationVariant::Numeric(_) => false,
        crate::puppet_lang::typing::TypeSpecificationVariant::String(v) => {
            v.min.is_some() || v.max.is_some()
        }
        crate::puppet_lang::typing::TypeSpecificationVariant::Pattern(v) => !v.list.is_empty(),
        crate::puppet_lang::typing::TypeSpecificationVariant::Regex(_) => true,
        crate::puppet_lang::typing::TypeSpecificationVariant::Hash(v) => {
            v.key.is_some() || v.value.is_some() || v.min.is_some() || v.max.is_some()
        }
        crate::puppet_lang::typing::TypeSpecificationVariant::Boolean(_) => false,
        crate::puppet_lang::typing::TypeSpecificationVariant::Array(v) => v.inner.is_some(),
        crate::puppet_lang::typing::TypeSpecificationVariant::Undef(_) => false,
        crate::puppet_lang::typing::TypeSpecificationVariant::Any(_) => false,
        crate::puppet_lang::typing::TypeSpecificationVariant::Optional(_) => true,
        crate::puppet_lang::typing::TypeSpecificationVariant::Variant(v) => !v.list.is_empty(),
        crate::puppet_lang::typing::TypeSpecificationVariant::Enum(v) => !v.list.is_empty(),
        crate::puppet_lang::typing::TypeSpecificationVariant::Struct(v) => {
            !v.keys.value.is_empty()
                || !v.keys.last_comment.is_empty()
                || !v.left_inner_comment.is_empty()
                || !v.right_inner_comment.is_empty()
        }
        crate::puppet_lang::typing::TypeSpecificationVariant::ExternalType(v) => {
            !v.arguments.is_empty()
        }
        crate::puppet_lang::typing::TypeSpecificationVariant::Sensitive(_) => true,
        crate::puppet_lang::typing::TypeSpecificationVariant::Tuple(_) => true,
    }
}

impl<EXTRA> Printer for crate::puppet_lang::typing::TypeArray<EXTRA> {
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
                .append(super::common::multiline_docs_list(
                    args,
                    self.inner.as_ref().map(|x| has_args(x)),
                ))
                .append(RcDoc::text("]"))
                .group()
        };

        RcDoc::text("Array").append(args).group()
    }
}

impl<EXTRA> Printer for crate::puppet_lang::typing::Variant<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("Variant")
            .append(RcDoc::text("["))
            .append(super::common::multiline_list(
                &self.list,
                self.list.first().map(|v| has_args(v)),
                |x| x.to_doc(),
            ))
            .append(RcDoc::text("]"))
            .group()
    }
}

impl<EXTRA> Printer for crate::puppet_lang::typing::Enum<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("Enum")
            .append(RcDoc::text("["))
            .append(common::multiline_list(&self.list, None, |x| {
                crate::puppet_pp_printer::term::to_doc(x, false)
            }))
            .append(RcDoc::text("]"))
            .group()
    }
}

impl<EXTRA> Printer for crate::puppet_lang::typing::ExternalType<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let args = if self.arguments.is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::text("[")
                .append(super::common::multiline_list(&self.arguments, None, |x| {
                    crate::puppet_pp_printer::expression::to_doc(x, false)
                }))
                .append(RcDoc::text("]"))
                .group()
        };

        RcDoc::intersperse(self.name.iter().map(RcDoc::text), RcDoc::text("::")).append(args)
    }
}

impl<EXTRA> Printer for crate::puppet_lang::typing::TypeHash<EXTRA> {
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
            let multiline = match (
                self.key.as_ref().map(|v| has_args(v)),
                self.value.as_ref().map(|v| has_args(v)),
            ) {
                (None, None) => None,
                (Some(true), _) | (_, Some(true)) => Some(true),
                _ => None,
            };
            RcDoc::text("[")
                .append(super::common::multiline_docs_list(args, multiline))
                .append(RcDoc::text("]"))
                .group()
        };

        RcDoc::text("Hash").append(args).group()
    }
}

impl<EXTRA> Printer for crate::puppet_lang::typing::TypeStructKey<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            crate::puppet_lang::typing::TypeStructKey::String(v) => v.to_doc(),
            crate::puppet_lang::typing::TypeStructKey::Optional(v) => RcDoc::text("Optional")
                .append(RcDoc::text("["))
                .append(RcDoc::softline_())
                .append(v.value.to_doc())
                .nest(2)
                .append(RcDoc::softline_())
                .append(RcDoc::text("]"))
                .group(),
            crate::puppet_lang::typing::TypeStructKey::NotUndef(v) => RcDoc::text("NotUndef")
                .append(RcDoc::text("["))
                .append(RcDoc::softline_())
                .append(v.value.to_doc())
                .nest(2)
                .append(RcDoc::softline_())
                .append(RcDoc::text("]"))
                .group(),
        }
    }
}
impl<EXTRA> Printer for crate::puppet_lang::typing::TypeStructKV<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        self.key
            .to_doc()
            .append(RcDoc::softline_())
            .append(RcDoc::column(|w| {
                let offset = (w / crate::puppet_pp_printer::ARROW_STEP + 1)
                    * crate::puppet_pp_printer::ARROW_STEP;
                RcDoc::text(format!("{}=>", " ".repeat(offset - w)))
            }))
            .append(RcDoc::softline())
            .append(self.value.to_doc())
            .nest(2)
            .group()
    }
}
impl<EXTRA> Printer for crate::puppet_lang::typing::TypeStruct<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("Struct")
            .append(RcDoc::text("[{"))
            .append(RcDoc::hardline())
            .append(
                RcDoc::intersperse(
                    self.keys.value.iter().map(|x| x.to_doc()),
                    RcDoc::text(",").append(RcDoc::hardline()),
                )
                .group()
                .append(crate::puppet_pp_printer::comment::to_doc(
                    &self.keys.last_comment,
                )),
            )
            .nest(2)
            .append(RcDoc::hardline())
            .append(RcDoc::text("}]"))
            .group()
    }
}

impl<EXTRA> Printer for crate::puppet_lang::typing::TypeSensitive<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let inner = match &self.value {
            crate::puppet_lang::typing::TypeSensitiveVariant::TypeSpecification(v) => v.to_doc(),
            crate::puppet_lang::typing::TypeSensitiveVariant::Term(v) => {
                crate::puppet_pp_printer::term::to_doc(v, false)
            }
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

impl<EXTRA> Printer for crate::puppet_lang::typing::TypeTuple<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let mut args: Vec<_> = self.list.iter().map(|v| v.to_doc()).collect();
        match (&self.min, &self.max) {
            (None, None) => (),
            (None, Some(max)) => args.extend(vec![RcDoc::text("default"), max.to_doc()]),
            (Some(min), None) => args.push(min.to_doc()),
            (Some(min), Some(max)) => args.extend(vec![min.to_doc(), max.to_doc()]),
        };

        let args = RcDoc::text("[")
            .append(super::common::multiline_docs_list(args, None))
            .append(RcDoc::text("]"))
            .group();

        RcDoc::text("Tuple").append(args).group()
    }
}

impl<EXTRA> Printer for crate::puppet_lang::typing::TypeSpecificationVariant<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            crate::puppet_lang::typing::TypeSpecificationVariant::Float(v) => {
                with_min_max("Float", &v.min, &v.max)
            }
            crate::puppet_lang::typing::TypeSpecificationVariant::Integer(v) => {
                with_min_max("Integer", &v.min, &v.max)
            }
            crate::puppet_lang::typing::TypeSpecificationVariant::Numeric(_) => {
                RcDoc::text("Numeric")
            }
            crate::puppet_lang::typing::TypeSpecificationVariant::String(v) => {
                with_min_max("String", &v.min, &v.max)
            }
            crate::puppet_lang::typing::TypeSpecificationVariant::Pattern(v) => v.to_doc(),
            crate::puppet_lang::typing::TypeSpecificationVariant::Regex(v) => v.to_doc(),
            crate::puppet_lang::typing::TypeSpecificationVariant::Hash(v) => v.to_doc(),
            crate::puppet_lang::typing::TypeSpecificationVariant::Boolean(_) => {
                RcDoc::text("Boolean")
            }
            crate::puppet_lang::typing::TypeSpecificationVariant::Array(v) => v.to_doc(),
            crate::puppet_lang::typing::TypeSpecificationVariant::Undef(_) => RcDoc::text("Undef"),
            crate::puppet_lang::typing::TypeSpecificationVariant::Any(_) => RcDoc::text("Any"),
            crate::puppet_lang::typing::TypeSpecificationVariant::Optional(v) => v.to_doc(),
            crate::puppet_lang::typing::TypeSpecificationVariant::Variant(v) => v.to_doc(),
            crate::puppet_lang::typing::TypeSpecificationVariant::Enum(v) => v.to_doc(),
            crate::puppet_lang::typing::TypeSpecificationVariant::Struct(v) => v.to_doc(),
            crate::puppet_lang::typing::TypeSpecificationVariant::ExternalType(v) => v.to_doc(),
            crate::puppet_lang::typing::TypeSpecificationVariant::Sensitive(v) => v.to_doc(),
            crate::puppet_lang::typing::TypeSpecificationVariant::Tuple(v) => v.to_doc(),
        }
    }
}

impl<EXTRA> Printer for crate::puppet_lang::typing::TypeSpecification<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        crate::puppet_pp_printer::comment::comment_or(
            &self.comment,
            RcDoc::hardline(),
            RcDoc::nil(),
        )
        .append(self.data.to_doc())
    }
}

#[test]
fn test_idempotence_short() {
    let cases = vec![
        "Float",
        "Float[1.0]",
        "Float[1.0,\n  2.2]",
        "Float[\n  default,\n  2.1]",
        "Integer",
        "String[1,\n  2]",
        "Pattern[\n  /a/,\n  /b/\n]",
        "Regex[ // ]",
        "Regex[\n  /aaaaaaaaaaa/\n]",
        "Optional[\n  #comment\n  Regex[\n    /aaaaaaaaaaa/\n  ]]",
        "Array[\n  Integer ]",
        "Array[\n  Integer,\n  2 ]",
        "Array[\n  Integer,\n  2, 4 ]",
        "Variant[\n  String[1,\n    2],\n  Integer\n]",
        "Enum[\n  1,\n  aaaaaa,\n  3\n]",
        "Some::Type",
        "Some::Type[\n  1 ]",
        "Hash[\n  String,\n  Integer,\n  default,\n  1\n]",
        "Struct[{\n  a\n                               =>\n    Integer\n}]",
        "Struct[{\n  Optional[\n      a]\n                               =>\n    Integer\n}]",
        "Struct[{\n  NotUndef[\n      a]\n                               =>\n    Integer\n}]",
        "Sensitive[\n  1 ]",
        "Sensitive[\n  String ]",
        "Tuple[\n  String,\n  Integer,\n  default,\n  100\n]",
    ];

    for case in cases {
        let (_, v) = crate::puppet_parser::typing::parse_type_specification(
            crate::puppet_parser::Span::new(case),
        )
        .unwrap();

        let mut w = Vec::new();
        v.to_doc().render(11, &mut w).unwrap();
        let generated = String::from_utf8(w).unwrap();
        println!("{} ==>\n------\n{}\n------", case, generated);

        assert_eq!(&generated, case)
    }
}
