pub type Program = Vec<Statement>;
pub type Block = Vec<Statement>;
pub type Identifier = String;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let {
        name: Identifier,
        initial: Expression,
    },
    If {
        condition: Expression,
        then: Block,
        otherwise: Option<Block>,
    },
    Else {
        then: Option<Block>,
    },
    Fn {
        name: Identifier,
        params: Vec<Parameter>,
        body: Block,
    },
    Loop {
        iterable: Option<Expression>,
        value: Option<Identifier>,
        then: Block,
    },
    Expr {
        expression: Expression,
    },
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(Identifier),
    Assign(Box<Expression>, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
}
