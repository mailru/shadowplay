use crate::Printer;
use pretty::RcDoc;

impl<EXTRA> Printer for puppet_lang::resource_collection::SearchExpression<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match &self.value {
            puppet_lang::resource_collection::ExpressionVariant::Equal((left, right)) => {
                crate::expression::infix_to_doc(
                    RcDoc::text(&left.name),
                    crate::term::to_doc(right, false),
                    "==",
                )
            }
            puppet_lang::resource_collection::ExpressionVariant::NotEqual((left, right)) => {
                crate::expression::infix_to_doc(
                    RcDoc::text(&left.name),
                    crate::term::to_doc(right, false),
                    "!=",
                )
            }
            puppet_lang::resource_collection::ExpressionVariant::And((left, right)) => {
                crate::expression::infix_to_doc(left.to_doc(), right.to_doc(), "and")
            }
            puppet_lang::resource_collection::ExpressionVariant::Or((left, right)) => {
                crate::expression::infix_to_doc(left.to_doc(), right.to_doc(), "or")
            }
            puppet_lang::resource_collection::ExpressionVariant::Parens(v) => RcDoc::text("(")
                .append(v.to_doc().nest(2))
                .append(RcDoc::text(")"))
                .group(),
        }
    }
}

impl<EXTRA> Printer for puppet_lang::resource_collection::ResourceCollection<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let search_expression = match &self.search_expression {
            Some(v) => RcDoc::softline()
                .append(RcDoc::text("<|"))
                .append(RcDoc::softline())
                .append(v.to_doc())
                .nest(2)
                .append(RcDoc::softline())
                .append(RcDoc::text("|>")),
            None => RcDoc::nil(),
        };

        crate::comment::comment_or(&self.comment, RcDoc::hardline(), RcDoc::nil())
            .append(self.type_specification.to_doc())
            .append(search_expression)
    }
}

impl<EXTRA> Printer for puppet_lang::statement::ResourceAttribute<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let value = match &self.value {
            puppet_lang::statement::ResourceAttributeVariant::Name((k, v)) => k
                .to_doc()
                .append(RcDoc::column(|w| {
                    let offset = (w / 10 + 1) * 10;
                    RcDoc::text(format!("{} =>", " ".repeat(offset - w)))
                }))
                .append(RcDoc::softline())
                .append(crate::expression::to_doc(v, false))
                .group()
                .nest(2),
            puppet_lang::statement::ResourceAttributeVariant::Group(v) => RcDoc::text("*")
                .append(RcDoc::softline())
                .append(RcDoc::column(|w| {
                    let offset = (w / 10 + 1) * 10;
                    RcDoc::text(format!("{} =>", " ".repeat(offset - w)))
                }))
                .append(RcDoc::column(|w| RcDoc::text(format!("=> ?? {} ??", w))))
                .append(RcDoc::softline())
                .append(crate::term::to_doc(v, false))
                .group()
                .nest(2),
        };

        crate::comment::comment_or(&self.comment, RcDoc::hardline(), RcDoc::nil()).append(value)
    }
}

impl<EXTRA> Printer for puppet_lang::statement::Resource<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let inner = if self.attributes.value.is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::hardline().append(RcDoc::intersperse(
                self.attributes.value.iter().map(|elt| elt.to_doc()),
                RcDoc::text(",").append(RcDoc::hardline()),
            ))
        };

        let inner = inner.append(crate::comment::to_doc(&self.attributes.last_comment));

        crate::expression::to_doc(&self.title, false)
            .append(RcDoc::text(":"))
            .append(inner)
            .nest(2)
    }
}

impl<EXTRA> Printer for puppet_lang::statement::ResourceSet<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let is_virtual = if self.is_virtual {
            RcDoc::text("@")
        } else {
            RcDoc::nil()
        };

        // just one-liner
        if self.list.value.len() == 1
            && self.list.value.first().unwrap().attributes.value.is_empty()
            && self
                .list
                .value
                .first()
                .unwrap()
                .attributes
                .last_comment
                .is_empty()
        {
            return crate::comment::comment_or(&self.comment, RcDoc::hardline(), RcDoc::nil())
                .append(is_virtual)
                .append(self.name.to_doc())
                .append(RcDoc::softline())
                .append(RcDoc::text("{"))
                .append(RcDoc::softline())
                .append(RcDoc::intersperse(
                    self.list.value.iter().map(|elt| elt.to_doc()),
                    RcDoc::nil(),
                ))
                .nest(2)
                .append(RcDoc::softline())
                .append(RcDoc::text("}"));
        }

        let inner = RcDoc::intersperse(
            self.list.value.iter().map(|elt| elt.to_doc()),
            RcDoc::text(";").append(RcDoc::hardline()),
        )
        .append(crate::comment::to_doc(&self.list.last_comment));

        crate::comment::comment_or(&self.comment, RcDoc::hardline(), RcDoc::nil())
            .append(is_virtual)
            .append(self.name.to_doc())
            .append(RcDoc::softline())
            .append(RcDoc::text("{"))
            .append(RcDoc::softline())
            .append(inner)
            .nest(2)
            .append(RcDoc::hardline())
            .append(RcDoc::text("}"))
    }
}

impl<EXTRA> Printer for puppet_lang::statement::RelationEltVariant<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            puppet_lang::statement::RelationEltVariant::ResourceSet(v) => v.to_doc(),
            puppet_lang::statement::RelationEltVariant::ResourceCollection(v) => v.to_doc(),
        }
    }
}

impl<EXTRA> Printer for puppet_lang::statement::RelationElt<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        if self.data.value.len() == 1 && self.data.last_comment.is_empty() {
            return self.data.value.first().unwrap().to_doc();
        }

        let inner = RcDoc::intersperse(
            self.data
                .value
                .iter()
                .map(|x| x.to_doc().append(RcDoc::text(","))),
            RcDoc::softline(),
        )
        .group()
        .append(crate::comment::to_doc(&self.data.last_comment));

        RcDoc::text("[")
            .append(RcDoc::softline())
            .append(inner)
            .nest(2)
            .append(RcDoc::softline())
            .append(RcDoc::text("]"))
    }
}

impl<EXTRA> Printer for puppet_lang::statement::RelationType<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self.variant {
            puppet_lang::statement::RelationVariant::ExecOrderRight => RcDoc::text("->"),
            puppet_lang::statement::RelationVariant::NotifyRight => RcDoc::text("~>"),
            puppet_lang::statement::RelationVariant::ExecOrderLeft => RcDoc::text("<-"),
            puppet_lang::statement::RelationVariant::NotifyLeft => RcDoc::text("<~"),
        }
    }
}

impl<EXTRA> Printer for puppet_lang::statement::Relation<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        self.relation_type
            .to_doc()
            .append(RcDoc::space())
            .append(self.relation_to.to_doc())
    }
}

impl<EXTRA> Printer for puppet_lang::statement::RelationList<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let head = self.head.to_doc();
        match &self.tail {
            None => head,
            Some(v) => head.append(RcDoc::softline()).append(v.to_doc()),
        }
    }
}

#[test]
fn test_idempotence_short() {
    let cases = vec![
        "Class[ a ] -> Class[ b::c ]",
        "[ Class[ a ], Class[ b ], ] -> Class[ b::c ]",
        "Class[ a ] -> ClassB <| (abc != 1) and c == test or (c == notest and abc == 1) |>",
        "file { '/etc/passwd':\n    ensure           => file,\n    mode   => '0644'\n}",
        "file { '/etc/passwd':\n    ensure           => file,\n    mode   => '0644';\n  '/etc/group':\n    ensure           => file\n}",
    ];

    for case in cases {
        let (_, v) =
            puppet_parser::statement::parse_statement_list(puppet_parser::Span::new(case)).unwrap();

        let mut w = Vec::new();
        crate::statement::statement_block_to_doc(&v, false)
            .render(100, &mut w)
            .unwrap();
        let generated = String::from_utf8(w).unwrap();
        println!("{} ==>\n------\n{}\n------", case, generated);

        assert_eq!(&generated, case)
    }
}
