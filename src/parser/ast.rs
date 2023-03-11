pub type Block = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let {
        name: Expression,
        initial: Expression,
    },
    If {
        condition: Expression,
        then: Block,
        otherwise: Option<Block>,
    },
    Fn {
        name: Expression,
        params: Vec<Parameter>,
        body: Block,
    },
    Loop {
        iterable: Option<Expression>,
        value: Option<Expression>,
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
    Identifier(String),
    Assign(Box<Expression>, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Infix(Box<Expression>, Op, Box<Expression>),
    Prefix(Op, Box<Expression>),
    List(Vec<Expression>),
    Dict(Vec<(Expression, Expression)>),
}

impl Expression {
    pub fn some(self) -> Option<Self> {
        Some(self)
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    Assign,
    LessThan,
    GreaterThan,
    LessThanOrEquals,
    GreaterThanOrEquals,
    And,
    Or,
    Not,
}
