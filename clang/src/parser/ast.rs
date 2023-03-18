pub type Block = Vec<Statement>;

#[derive(Debug, PartialEq, Clone)]
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
        params: Vec<Expression>,
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
    Return {
        value: Option<Expression>,
    },
    Break,
}

#[derive(Debug, PartialEq, Clone)]
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
    Break,
}

impl Expression {
    pub fn some(self) -> Option<Self> {
        Some(self)
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEquals,
    GreaterThanOrEquals,
    And,
    Or,
    Not,
}
