use crate::token::{Token, TokenType};

#[derive(Default, Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub token: Token,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub identifier: Identifier,
    pub expression: Expression,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub expression: Expression,
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in self.statements.iter() {
            match stmt {
                Statement::LetStatement(s) => write!(
                    f,
                    "{} {} = {};",
                    s.token.literal, s.identifier.value, s.expression.value
                )?,
                Statement::ReturnStatement(s) => {
                    write!(f, "{} {};", s.token.literal, s.expression.value)?
                }
                _ => (),
            }
        }
        Ok(())
    }
}
