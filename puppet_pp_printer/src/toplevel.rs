use crate::Printer;
use pretty::RcDoc;

fn definition_to_doc<'a, EXTRA>(
    keyword: &'static str,
    identifier: &'a puppet_lang::identifier::LowerIdentifier<EXTRA>,
    args: &'a puppet_lang::List<EXTRA, puppet_lang::argument::Argument<EXTRA>>,
    inherits: &'a Option<puppet_lang::identifier::LowerIdentifier<EXTRA>>,
    return_type: &'a Option<puppet_lang::typing::TypeSpecification<EXTRA>>,
    body: &'a puppet_lang::List<EXTRA, puppet_lang::statement::Statement<EXTRA>>,
) -> RcDoc<'a, ()> {
    let inherits = match inherits {
        Some(v) => RcDoc::text("inherits")
            .append(RcDoc::softline())
            .append(v.to_doc())
            .append(RcDoc::softline()),
        None => RcDoc::nil(),
    };

    let return_type = match return_type {
        Some(v) => RcDoc::text(">>")
            .append(RcDoc::softline())
            .append(v.to_doc())
            .append(RcDoc::softline()),
        None => RcDoc::nil(),
    };

    RcDoc::text(keyword)
        .append(RcDoc::softline())
        .append(identifier.to_doc())
        .append(RcDoc::softline())
        .append(crate::argument::list_to_rounded_doc(args))
        .append(RcDoc::softline())
        .append(inherits)
        .append(return_type)
        .append(crate::statement::statement_block_to_doc(body, true))
}

impl<EXTRA> Printer for puppet_lang::toplevel::ToplevelVariant<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            puppet_lang::toplevel::ToplevelVariant::Class(v) => definition_to_doc(
                "class",
                &v.identifier,
                &v.arguments,
                &v.inherits,
                &None,
                &v.body,
            ),
            puppet_lang::toplevel::ToplevelVariant::Definition(v) => {
                definition_to_doc("define", &v.identifier, &v.arguments, &None, &None, &v.body)
            }
            puppet_lang::toplevel::ToplevelVariant::Plan(v) => {
                definition_to_doc("plan", &v.identifier, &v.arguments, &None, &None, &v.body)
            }
            puppet_lang::toplevel::ToplevelVariant::TypeDef(v) => RcDoc::text("type")
                .append(RcDoc::softline())
                .append(v.identifier.to_doc())
                .append(RcDoc::softline())
                .append(
                    RcDoc::text("=")
                        .append(RcDoc::space())
                        .append(v.value.to_doc())
                        .nest(2),
                )
                .group()
                .nest(2),
            puppet_lang::toplevel::ToplevelVariant::FunctionDef(v) => definition_to_doc(
                "function",
                &v.identifier,
                &v.arguments,
                &None,
                &v.return_type,
                &v.body,
            ),
        }
    }
}

#[test]
fn test_idempotence_short() {
    let cases = vec![
        "class aaa::bbb () inherits zzz {\n  \n}",
        "class aaa::bbb () inherits zzz {\n  if $a {\n    1\n  } else {\n    2\n  }\n}",
        "function aaa::bbb (\n  $empty,\n  $a = 1,\n  String $b = 'a b c',\n) >> String {\n  \n}",
        "type Aaa::Bbb = String[1, 2]",
    ];

    for case in cases {
        let (_, v) = puppet_parser::toplevel::parse(puppet_parser::Span::new(case)).unwrap();

        let mut w = Vec::new();
        v.data.to_doc().render(50, &mut w).unwrap();
        let generated = String::from_utf8(w).unwrap();
        println!("{} ==>\n------\n{}\n------", case, generated);

        assert_eq!(&generated, case)
    }
}
