pub(crate) use crate::{RoxyType, Token};
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Call(Call),
    Get(Get),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Logical),
    Set(Set),
    Super(Super),
    This(This),
    Unary(Unary),
    Variable(Variable),
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Get {
    pub object: Box<Expr>,
    pub name: Token,
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: RoxyType,
}

#[derive(Debug, Clone)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Set {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Super {
    pub keyword: Token,
    pub method: Token,
}

#[derive(Debug, Clone)]
pub struct This {
    pub keyword: Token,
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Expr::Assign(assign) => write!(f, "{} = {}", assign.name.lexeme, assign.value),
            Expr::Binary(binary) => write!(
                f,
                "({} {} {})",
                binary.left, binary.operator.lexeme, binary.right
            ),
            Expr::Call(call) => {
                write!(f, "{}(", call.callee)?;
                for arg in &call.arguments {
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expr::Get(get) => {
                write!(f, "{}.{}", get.object, get.name.lexeme)
            }
            Expr::Grouping(grouping) => {
                write!(f, "(group {})", grouping.expr)
            }
            Expr::Literal(literal) => {
                write!(f, "{}", literal.value)
            }
            Expr::Logical(logical) => {
                write!(
                    f,
                    "({} {} {})",
                    logical.right, logical.operator.lexeme, logical.left
                )
            }
            Expr::Set(set) => write!(f, "{}.{} = {}", set.object, set.name.lexeme, set.value),
            Expr::Super(suuper) => {
                write!(f, "{} {}()", suuper.keyword.lexeme, suuper.method.lexeme)
            }
            Expr::This(this) => write!(f, "{}", this.keyword.lexeme),
            Expr::Unary(unary) => write!(f, "({}{})", unary.operator.lexeme, unary.right),
            Expr::Variable(var) => write!(f, "var({})", var.name.lexeme),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Block),
    Class(Class),
    Expression(ExpressionStmt),
    Function(Function),
    If(If),
    Print(Print),
    VariableStmt(VariableStmt),
    While(While),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: Token,
    pub superclass: Variable,
    pub methods: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Box<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Print {
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub keyword: Token,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct VariableStmt {
    pub name: Token,
    pub value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Stmt>,
}
