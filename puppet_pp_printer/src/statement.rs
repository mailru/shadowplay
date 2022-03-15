use crate::Printer;
use pretty::RcDoc;

pub fn statement_block_to_doc<'a, EXTRA>(
    elt: &'a puppet_lang::List<EXTRA, puppet_lang::statement::Statement<EXTRA>>,
    with_parens: bool,
) -> RcDoc<'a, ()> {
    let inner = RcDoc::intersperse(elt.value.iter().map(|x| x.to_doc()), RcDoc::hardline())
        .append(elt.last_comment.to_doc());

    if with_parens {
        RcDoc::text("{")
            .append(RcDoc::hardline())
            .append(inner)
            .nest(2)
            .append(RcDoc::hardline())
            .append(RcDoc::text("}"))
    } else {
        inner
    }
}

fn condition_and_statement_to_doc<'a, EXTRA>(
    keyword: RcDoc<'a, ()>,
    elt: &'a puppet_lang::statement::ConditionAndStatement<EXTRA>,
) -> RcDoc<'a, ()> {
    crate::comment::comment_or(
        &elt.comment_before_elsif_word,
        RcDoc::hardline(),
        RcDoc::nil(),
    )
    .append(keyword)
    .append(RcDoc::softline())
    .append(crate::expression::to_doc(&elt.condition, false).nest(2))
    .append(RcDoc::softline())
    .append(statement_block_to_doc(&elt.body, true))
}

impl<EXTRA> Printer for puppet_lang::statement::IfElse<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let r = condition_and_statement_to_doc(RcDoc::text("if"), &self.condition);

        let r = r.append(RcDoc::intersperse(
            self.elsif_list.iter().map(|elt| {
                RcDoc::softline().append(condition_and_statement_to_doc(RcDoc::text("elsif"), elt))
            }),
            RcDoc::nil(),
        ));

        match &self.else_block {
            None => r,
            Some(elt) => r
                .append(crate::comment::comment_or(
                    &self.comment_before_else_word,
                    RcDoc::hardline(),
                    RcDoc::softline(),
                ))
                .append(RcDoc::text("else"))
                .append(crate::comment::comment_or(
                    &self.comment_before_else_body,
                    RcDoc::hardline(),
                    RcDoc::softline(),
                ))
                .append(statement_block_to_doc(&elt, true)),
        }
    }
}

impl<EXTRA> Printer for puppet_lang::expression::CaseVariant<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            puppet_lang::expression::CaseVariant::Term(v) => crate::term::to_doc(v, false),
            puppet_lang::expression::CaseVariant::Default(_) => RcDoc::text("default"),
        }
    }
}

impl<EXTRA> Printer for puppet_lang::statement::CaseElement<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let matches_list = if self.matches.len() == 1 {
            self.matches.first().unwrap().to_doc()
        } else {
            RcDoc::text("[")
                .append(RcDoc::softline())
                .append(RcDoc::intersperse(
                    self.matches.iter().map(|x| x.to_doc()),
                    RcDoc::softline(),
                ))
                .append(RcDoc::softline())
                .append(RcDoc::softline())
                .append(RcDoc::text("]"))
        };

        crate::comment::comment_or(&self.comment, RcDoc::hardline(), RcDoc::nil())
            .append(matches_list)
            .append(RcDoc::softline_())
            .append(RcDoc::text(":"))
            .append(RcDoc::softline())
            .append(statement_block_to_doc(&self.body, true))
            .group()
    }
}

impl<EXTRA> Printer for puppet_lang::statement::Case<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let inner = RcDoc::intersperse(
            self.elements.value.iter().map(|x| x.to_doc()),
            RcDoc::hardline(),
        )
        .append(self.elements.last_comment.to_doc());

        RcDoc::text("case")
            .append(RcDoc::softline())
            .append(crate::expression::to_doc(&self.condition, false).nest(2))
            .append(RcDoc::softline())
            .append(RcDoc::text("{"))
            .append(RcDoc::hardline())
            .append(inner)
            .nest(2)
            .append(RcDoc::hardline())
            .append(RcDoc::text("}"))
    }
}

impl<EXTRA> Printer for puppet_lang::statement::ResourceDefaults<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let inner = RcDoc::intersperse(
            self.args.value.iter().map(|(k, v)| {
                crate::term::to_doc(k, false)
                    .append(RcDoc::softline())
                    .append(RcDoc::text("=>"))
                    .append(RcDoc::softline())
                    .append(crate::expression::to_doc(v, false))
                    .append(RcDoc::text(","))
            }),
            RcDoc::hardline(),
        )
        .append(self.args.last_comment.to_doc());

        RcDoc::text(&self.name)
            .append(RcDoc::softline())
            .append(RcDoc::text("{"))
            .append(RcDoc::hardline())
            .append(inner)
            .nest(2)
            .append(RcDoc::hardline())
            .append(RcDoc::text("}"))
    }
}

impl<EXTRA> Printer for puppet_lang::statement::Statement<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let v = match &self.value {
            puppet_lang::statement::StatementVariant::Expression(v) => {
                crate::expression::to_doc(v, false)
            }
            puppet_lang::statement::StatementVariant::RelationList(_) => todo!(),
            puppet_lang::statement::StatementVariant::IfElse(v) => v.to_doc(),
            puppet_lang::statement::StatementVariant::Unless(v) => {
                condition_and_statement_to_doc(RcDoc::text("unless"), v)
            }
            puppet_lang::statement::StatementVariant::Case(v) => v.to_doc(),
            puppet_lang::statement::StatementVariant::Toplevel(v) => v.data.to_doc(),
            puppet_lang::statement::StatementVariant::ResourceDefaults(v) => v.to_doc(),
        };

        crate::comment::comment_or(&self.comment, RcDoc::hardline(), RcDoc::nil()).append(v)
    }
}

#[test]
fn test_idempotence_short() {
    let cases = vec![
        "unless !$a {\n  $a = 1\n  $b = $a + 1\n}",
        "unless !$a {\n  $a = 1\n  unless (($a + $a + $a))\n  {\n    $b = $a + 1\n    unless (($a + $a\n          + $a)) {\n      $b = $a + 1\n      unless (($a + $a\n            + $a)) {\n        $b = $a + 1\n      }\n    }\n  }\n}",
        "if $a {\n  undef\n}",
        "if $a {\n  undef\n} else {\n  $c\n}",
        "if $a {\n  undef\n}\n#comment1\nelse {\n  1\n}",
        "if $a {\n  undef\n}\n#comment1\nelse\n#comment2\n{\n  1\n}",
        "if $a {\n  undef\n} \n#comment\nelsif !$a {\n  $a\n} elsif !$b {\n  $b\n} else {\n  1\n}",
        "if $a {\n  undef\n} elsif !$a {\n  $a\n}",
        "case $a {\n  \n  #comment\n  1: {\n    $b\n  }\n}",
        "case $a {\n  \n  #comment\n  1: {\n    $b\n  }\n  default: {\n    \n  }\n}",
        "Exec {\n  command => test,\n  provider => shell,\n  # comment\n  #line2\n  #line3\n}",
    ];

    for case in cases {
        let (_, v) =
            puppet_parser::statement::parse_statement_list(puppet_parser::Span::new(case)).unwrap();

        let mut w = Vec::new();
        statement_block_to_doc(&v, false)
            .render(25, &mut w)
            .unwrap();
        let generated = String::from_utf8(w).unwrap();
        println!("{} ==>\n------\n{}\n------", case, generated);

        assert_eq!(&generated, case)
    }
}
