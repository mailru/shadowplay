use crate::Printer;
use pretty::{Doc, RcDoc};

fn infix_to_doc<'a>(left: RcDoc<'a, ()>, right: RcDoc<'a, ()>, op: &'static str) -> RcDoc<'a, ()> {
    left.append(RcDoc::line())
        .append(
            RcDoc::text(op)
                .append(RcDoc::softline())
                .append(right)
                .group(),
        )
        .group()
        .nest(2)
}

impl<EXTRA> Printer for puppet_lang::expression::Expression<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let v = match &self.value {
            puppet_lang::expression::ExpressionVariant::Term(v) => v.to_doc(),
            puppet_lang::expression::ExpressionVariant::Assign((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "=")
            }
            puppet_lang::expression::ExpressionVariant::And((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "and")
            }
            puppet_lang::expression::ExpressionVariant::Or((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "or")
            }
            puppet_lang::expression::ExpressionVariant::Equal((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "==")
            }
            puppet_lang::expression::ExpressionVariant::NotEqual((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "!=")
            }
            puppet_lang::expression::ExpressionVariant::Gt((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), ">")
            }
            puppet_lang::expression::ExpressionVariant::GtEq((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), ">=")
            }
            puppet_lang::expression::ExpressionVariant::Lt((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "<")
            }
            puppet_lang::expression::ExpressionVariant::LtEq((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "<=")
            }
            puppet_lang::expression::ExpressionVariant::ShiftLeft((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "<<")
            }
            puppet_lang::expression::ExpressionVariant::ShiftRight((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), ">>")
            }
            puppet_lang::expression::ExpressionVariant::Plus((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "+")
            }
            puppet_lang::expression::ExpressionVariant::Minus((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "-")
            }
            puppet_lang::expression::ExpressionVariant::Multiply((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "*")
            }
            puppet_lang::expression::ExpressionVariant::Divide((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "/")
            }
            puppet_lang::expression::ExpressionVariant::Modulo((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "%")
            }
            puppet_lang::expression::ExpressionVariant::ChainCall(_) => todo!(),
            puppet_lang::expression::ExpressionVariant::MatchRegex((left, right)) => infix_to_doc(
                left.to_doc(),
                RcDoc::text("/")
                    .append(&right.data)
                    .append(RcDoc::text("/")),
                "=~",
            ),
            puppet_lang::expression::ExpressionVariant::NotMatchRegex((left, right)) => {
                infix_to_doc(
                    left.to_doc(),
                    RcDoc::text("/")
                        .append(&right.data)
                        .append(RcDoc::text("/")),
                    "!~",
                )
            }
            puppet_lang::expression::ExpressionVariant::MatchType(_) => todo!(),
            puppet_lang::expression::ExpressionVariant::NotMatchType(_) => todo!(),
            puppet_lang::expression::ExpressionVariant::In((left, right)) => {
                infix_to_doc(left.to_doc(), right.to_doc(), "in")
            }
            puppet_lang::expression::ExpressionVariant::Not(v) => {
                RcDoc::text("!").append(v.to_doc())
            }
            puppet_lang::expression::ExpressionVariant::Selector(_) => todo!(),
            puppet_lang::expression::ExpressionVariant::FunctionCall(_) => todo!(),
            puppet_lang::expression::ExpressionVariant::BuiltinFunction(_) => todo!(),
        };

        self.comment
            .to_doc()
            .append(v)
            .append(self.accessor.to_doc())
    }
}

#[test]
fn test_idempotence_short() {
    let cases = vec![
        "123",
        "123.45 * 1\n  - 2",
        "123[1][\n    2, 3\n  ][4][5]",
        "[\n  (123.45),\n  146,\n]",
        "[\n  (\n    #comment\n    123.45),\n  146,\n]",
        "[\n  (\n    #comment\n    123.45),\n  146,\n  #ending_comment\n  \n]",
        "!$a",
        "/a/",
        "/a\\d/",
        "$z\n  =\n  11111111\n    and\n    2222",
    ];

    for case in cases {
        let (_, v) =
            puppet_parser::expression::parse_expression(puppet_parser::Span::new(case)).unwrap();

        let mut w = Vec::new();
        v.to_doc().render(11, &mut w).unwrap();
        let generated = String::from_utf8(w).unwrap();
        println!("{:?} ==> {}", case, generated);

        assert_eq!(&generated, case)
    }
}
