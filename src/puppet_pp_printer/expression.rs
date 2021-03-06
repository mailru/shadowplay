use crate::puppet_pp_printer::Printer;
use pretty::{Doc, RcDoc};

pub fn infix_to_doc<'a>(
    left: RcDoc<'a, ()>,
    right: RcDoc<'a, ()>,
    op: &'static str,
) -> RcDoc<'a, ()> {
    left.append(RcDoc::softline())
        .append(RcDoc::text(op))
        .append(RcDoc::space())
        .append(right)
        .group()
}

fn assigment_to_doc<'a>(left: RcDoc<'a, ()>, right: RcDoc<'a, ()>) -> RcDoc<'a, ()> {
    left.append(RcDoc::line())
        .append(
            RcDoc::text("=")
                .append(RcDoc::space())
                .append(right)
                .nest(2),
        )
        .group()
        .nest(2)
}

impl<EXTRA> Printer for crate::puppet_lang::expression::Lambda<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        crate::puppet_pp_printer::argument::list_to_piped_doc(&self.args)
            .append(RcDoc::softline())
            .append(crate::puppet_pp_printer::statement::statement_block_to_doc(
                &self.body, true,
            ))
    }
}
impl<EXTRA> Printer for crate::puppet_lang::expression::FunctionCall<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let lambda = match &self.lambda {
            Some(v) => RcDoc::softline().append(v.to_doc()),
            None => RcDoc::nil(),
        };

        let parens = if self.args.is_empty() {
            RcDoc::text("()")
        } else {
            RcDoc::text("(")
                .append(RcDoc::softline_())
                .append(RcDoc::intersperse(
                    self.args
                        .iter()
                        .map(|x| crate::puppet_pp_printer::expression::to_doc(x, false)),
                    RcDoc::text(",").append(Doc::line()),
                ))
                // .append(self.args.last_comment.to_doc())
                .nest(2)
                .append(RcDoc::softline_())
                .group()
                .append(RcDoc::text(")"))
        };

        self.identifier
            .to_doc()
            .append(parens)
            .append(lambda)
            .group()
    }
}

impl<EXTRA> Printer for crate::puppet_lang::expression::ChainCall<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        to_doc(&self.left, false)
            .append(RcDoc::softline_())
            .append(RcDoc::text(".").append(self.right.to_doc()))
            .group()
    }
}

impl<EXTRA> Printer for crate::puppet_lang::expression::SelectorCase<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let case = match &self.case {
            crate::puppet_lang::expression::CaseVariant::Term(v) => {
                crate::puppet_pp_printer::term::to_doc(v, false)
            }
            crate::puppet_lang::expression::CaseVariant::Default(_) => RcDoc::text("default"),
        };

        crate::puppet_pp_printer::comment::comment_or(
            &self.comment,
            RcDoc::hardline(),
            RcDoc::nil(),
        )
        .append(case)
        .append(RcDoc::softline())
        .append(RcDoc::text("=>"))
        .append(RcDoc::softline())
        .append(to_doc(&self.body, false))
        .group()
    }
}
impl<EXTRA> Printer for crate::puppet_lang::expression::Selector<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        to_doc(&self.condition, false)
            .append(RcDoc::softline())
            .append(RcDoc::text("?"))
            .append(RcDoc::space())
            .append(RcDoc::text("{"))
            .append(RcDoc::line())
            .append(
                RcDoc::intersperse(
                    self.cases
                        .value
                        .iter()
                        .map(|x| x.to_doc().append(RcDoc::text(","))),
                    Doc::line(),
                )
                .group()
                .append(crate::puppet_pp_printer::comment::comment_or(
                    &self.cases.last_comment,
                    RcDoc::hardline(),
                    RcDoc::nil(),
                )),
            )
            .nest(2)
            .append(RcDoc::line())
            .append(RcDoc::text("}"))
            .group()
    }
}

fn builtin_many1_to_doc<'a, EXTRA>(
    name: &'a str,
    elt: &'a crate::puppet_lang::builtin::Many1<EXTRA>,
    with_parens: bool,
) -> RcDoc<'a, ()> {
    let args_list = RcDoc::intersperse(
        elt.args
            .iter()
            .map(|x| crate::puppet_pp_printer::expression::to_doc(x, false)),
        RcDoc::text(",").append(Doc::line()),
    )
    .group()
    // .append(v.args.last_comment.to_doc())
    .nest(2);

    let lambda = match &elt.lambda {
        Some(v) => RcDoc::softline().append(v.to_doc()),
        None => RcDoc::nil(),
    };

    let args_list = match ((with_parens || elt.lambda.is_some()), elt.args.is_empty()) {
        (_, true) => RcDoc::text("()"),
        (true, false) => RcDoc::text("(")
            .append(RcDoc::softline_())
            .append(args_list)
            .append(RcDoc::softline_())
            .append(RcDoc::text(")")),
        (false, false) => RcDoc::softline()
            .append(args_list)
            .append(RcDoc::softline_()),
    };

    RcDoc::text(name)
        .append(RcDoc::softline_())
        .append(args_list)
        .append(lambda)
}

impl<EXTRA> Printer for crate::puppet_lang::builtin::BuiltinVariant<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            crate::puppet_lang::builtin::BuiltinVariant::Undef => RcDoc::text("undef"),
            crate::puppet_lang::builtin::BuiltinVariant::Tag(v) => {
                builtin_many1_to_doc("tag", v, false)
            }
            crate::puppet_lang::builtin::BuiltinVariant::Require(v) => {
                builtin_many1_to_doc("require", v, false)
            }
            crate::puppet_lang::builtin::BuiltinVariant::Include(v) => {
                builtin_many1_to_doc("include", v, false)
            }
            crate::puppet_lang::builtin::BuiltinVariant::Realize(v) => {
                builtin_many1_to_doc("realize", v, true)
            }
            crate::puppet_lang::builtin::BuiltinVariant::CreateResources(v) => {
                builtin_many1_to_doc("create_resources", v, true)
            }
            crate::puppet_lang::builtin::BuiltinVariant::Return(v) => match v.as_ref() {
                None => RcDoc::text("return()"),
                Some(v) => RcDoc::text("return")
                    .append(RcDoc::text("("))
                    .append(RcDoc::softline_())
                    .append(crate::puppet_pp_printer::expression::to_doc(v, false))
                    .nest(2)
                    .append(RcDoc::softline_())
                    .append(RcDoc::text(")")),
            },
            crate::puppet_lang::builtin::BuiltinVariant::Template(v) => {
                builtin_many1_to_doc("template", v, true)
            }
        }
    }
}
pub fn to_doc<EXTRA>(
    expr: &crate::puppet_lang::expression::Expression<EXTRA>,
    hide_toplevel_variable_tag: bool,
) -> RcDoc<()> {
    let v = match &expr.value {
        crate::puppet_lang::expression::ExpressionVariant::Term(v) => {
            crate::puppet_pp_printer::term::to_doc(v, hide_toplevel_variable_tag)
        }
        crate::puppet_lang::expression::ExpressionVariant::Assign((left, right)) => {
            assigment_to_doc(to_doc(left, false), to_doc(right, false))
        }
        crate::puppet_lang::expression::ExpressionVariant::And((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), "and")
        }
        crate::puppet_lang::expression::ExpressionVariant::Or((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), "or")
        }
        crate::puppet_lang::expression::ExpressionVariant::Equal((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), "==")
        }
        crate::puppet_lang::expression::ExpressionVariant::NotEqual((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), "!=")
        }
        crate::puppet_lang::expression::ExpressionVariant::Gt((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), ">")
        }
        crate::puppet_lang::expression::ExpressionVariant::GtEq((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), ">=")
        }
        crate::puppet_lang::expression::ExpressionVariant::Lt((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), "<")
        }
        crate::puppet_lang::expression::ExpressionVariant::LtEq((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), "<=")
        }
        crate::puppet_lang::expression::ExpressionVariant::ShiftLeft((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), "<<")
        }
        crate::puppet_lang::expression::ExpressionVariant::ShiftRight((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), ">>")
        }
        crate::puppet_lang::expression::ExpressionVariant::Plus((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), "+")
        }
        crate::puppet_lang::expression::ExpressionVariant::Minus((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), "-")
        }
        crate::puppet_lang::expression::ExpressionVariant::Multiply((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), "*")
        }
        crate::puppet_lang::expression::ExpressionVariant::Divide((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), "/")
        }
        crate::puppet_lang::expression::ExpressionVariant::Modulo((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), "%")
        }
        crate::puppet_lang::expression::ExpressionVariant::ChainCall(v) => v.to_doc(),
        crate::puppet_lang::expression::ExpressionVariant::MatchRegex((left, right)) => {
            infix_to_doc(
                to_doc(left, false),
                RcDoc::text("/")
                    .append(&right.data)
                    .append(RcDoc::text("/")),
                "=~",
            )
        }
        crate::puppet_lang::expression::ExpressionVariant::NotMatchRegex((left, right)) => {
            infix_to_doc(
                to_doc(left, false),
                RcDoc::text("/")
                    .append(&right.data)
                    .append(RcDoc::text("/")),
                "!~",
            )
        }
        crate::puppet_lang::expression::ExpressionVariant::MatchType((left, right)) => {
            infix_to_doc(to_doc(left, false), right.to_doc(), "=~")
        }
        crate::puppet_lang::expression::ExpressionVariant::NotMatchType((left, right)) => {
            infix_to_doc(to_doc(left, false), right.to_doc(), "!~")
        }
        crate::puppet_lang::expression::ExpressionVariant::In((left, right)) => {
            infix_to_doc(to_doc(left, false), to_doc(right, false), "in")
        }
        crate::puppet_lang::expression::ExpressionVariant::Not(v) => {
            RcDoc::text("!").append(to_doc(v, false))
        }
        crate::puppet_lang::expression::ExpressionVariant::Selector(v) => v.to_doc(),
        crate::puppet_lang::expression::ExpressionVariant::FunctionCall(v) => v.to_doc(),
        crate::puppet_lang::expression::ExpressionVariant::BuiltinFunction(v) => v.to_doc(),
    };

    crate::puppet_pp_printer::comment::comment_or(&expr.comment, RcDoc::hardline(), RcDoc::nil())
        .append(v)
        .append(expr.accessor.to_doc())
}

#[test]
fn test_idempotence_short() {
    let cases = vec![
        "123",
        "4.0 + 5.1",
        "'hello universe'",
        "\"hello\n universe\"",
        "\"hello ${universe}\"",
        "\"hello ${::universe}\"",
        "\"hello ${universe[\n    0\n  ]}\"",
        "\"hello ${funcall()} suffix\"",
        "\"hello ${funcall(\n  1,\n  2\n)} suffix\"",
        "123.45 * 1\n- 2",
        "123[1][\n    2, 3\n  ][4][5]",
        "[\n  (123.45),\n  146,\n]",
        "[\n  (\n    #comment\n    123.45),\n  146,\n]",
        "[\n  (\n    #comment\n    123.45),\n  146,\n  #ending_comment\n]",
        "!$a",
        "/a/",
        "/a\\d/",
        "$z\n  = 11111111\n    and 2222",
        "$z\n  = 11111111\n    + 2222\n    + 3333",
        "1 + 1 + 1\n+ 1 + 1 + 1\n+ 1 + 1 + 1\n+ 1 + 1 + 1\n+ 1",
        "(1 or 2)\nand (3 + 4\n  * 5)\nor (true\n  and (!true\n    and false))",
        "$v.call1()\n.call2(1,\n  2)\n.call3withlongname()",
        "fn(1, 2)\n|$a, $b| {\n  1\n}",
        "$v ? {\n  1 => a,\n  \n  #comment\n  2 => b,\n  default\n  => c,\n}",
        "undef",
        "require\na::b, c",
        "create_resources\n(1, 2)",
        "realize(1,\n  2) |$a,\n  $b| {\n  1\n}",
        "return()",
        "return(\n  aaaaaaaaaaaaaaa\n)",
    ];

    for case in cases {
        let (_, v) = crate::puppet_parser::expression::parse_expression(
            crate::puppet_parser::Span::new(case),
        )
        .unwrap();

        let mut w = Vec::new();
        to_doc(&v, false).render(11, &mut w).unwrap();
        let generated = String::from_utf8(w).unwrap();
        println!("{} ==>\n------\n{}\n------", case, generated);

        assert_eq!(&generated, case)
    }
}
