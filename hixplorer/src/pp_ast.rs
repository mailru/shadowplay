pub struct Definition {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Body,
}

pub struct Parameter {
    pub name: String,
    pub value: Expression,
    pub type_expr: Expression,
}

pub struct Body {
    operations: Vec<Expression>,
}

pub enum Expression {
    String(String),
    DoubleQuotedString(String),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Subtract((Box<Expression>, Box<Expression>)),
    Add((Box<Expression>, Box<Expression>)),
    Multiply((Box<Expression>, Box<Expression>)),
    Not(Box<Expression>),
    Array(Vec<Expression>),
    Or((Box<Expression>, Box<Expression>)),
    And((Box<Expression>, Box<Expression>)),
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for elt in &self.operations {
            writeln!(f, "{}", elt)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // TODO quoting
            Expression::String(v) => {
                write!(f, "'{}'", v)
            }
            // TODO quoting
            Expression::DoubleQuotedString(v) => {
                write!(f, "\"{}\"", v)
            }
            Expression::Boolean(v) => {
                if *v {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Expression::Integer(v) => {
                write!(f, "{}", v)
            }
            Expression::Float(v) => {
                write!(f, "{}", v)
            }
            Expression::Subtract((v1, v2)) => {
                write!(f, "{} - {}", v1, v2)
            }
            Expression::Add((v1, v2)) => {
                write!(f, "{} + {}", v1, v2)
            }
            Expression::Multiply((v1, v2)) => {
                write!(f, "{} * {}", v1, v2)
            }
            Expression::Not(v) => {
                write!(f, "!{}", v)
            }
            // TODO formatting
            Expression::Array(v) => {
                write!(
                    f,
                    "{}",
                    v.iter()
                        .map(|elt| format!("{}", elt))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            // TODO extra parens
            Expression::Or((v1, v2)) => {
                write!(f, "({} || {})", v1, v2)
            }
            // TODO extra parens
            Expression::And((v1, v2)) => {
                write!(f, "({} && {})", v1, v2)
            }
        }
    }
}

impl std::fmt::Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {} ", self.name)?;
        if self.parameters.is_empty() {
            writeln!(f, "() {{")?
        } else {
            writeln!(f, "(")?;
            let params_string = self
                .parameters
                .iter()
                .map(|p| format!("  ${} = {}", p.name, p.value))
                .collect::<Vec<_>>()
                .join(",\n");
            writeln!(f, "{}\n) {{", params_string)?
        }
        write!(f, "{}", self.body)?;
        writeln!(f, "}}")
    }
}

fn main() {
    let c = Definition {
        name: "some::value".to_owned(),
        parameters: vec![Parameter {
            name: "hello".to_owned(),
            value: Expression::Boolean(true),
            type_expr: Expression::Boolean(true),
        }],
        body: Body {
            operations: vec![
                Expression::String("test".to_owned()),
                Expression::DoubleQuotedString("test".to_owned()),
                Expression::Or((
                    Box::new(Expression::Boolean(true)),
                    Box::new(Expression::Or((
                        Box::new(Expression::Boolean(true)),
                        Box::new(Expression::Boolean(false)),
                    ))),
                )),
            ],
        },
    };

    println!("{}", c)
}
