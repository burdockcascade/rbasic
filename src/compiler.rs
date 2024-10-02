use std::collections::HashMap;
use log::trace;
use crate::tokenizer::{Token, TokenType};
use crate::tokenizer::TokenType::In;
use crate::variant::Variant;
use crate::vm::{Instruction, Program};

macro_rules! expect_token {
    ($tokens:expr, $index:expr, $expected:pat) => {
        match $tokens.get($index) {
            Some(token) => {
                match token.token_type {
                    $expected => {
                        $index += 1;
                        token
                    },
                    _ => panic!("Unexpected token {:?}", token),
                }
            },
            None => panic!("Expected more tokens"),
        }
    };
}

macro_rules! consume_token {
    ($tokens:expr, $index:expr, $expected:pat) => {
        match $tokens.get($index) {
            Some(token) => {
                match token.token_type {
                    $expected => {
                        $index += 1;
                    },
                    _ => panic!("Unexpected token {:?}", token),
                }
            },
            None => panic!("Expected more tokens"),
        }
    };
}

struct Variable {
    name: String,
    value: Variant,
}

struct Function {
    name: String,
    locals: Vec<Variable>,
    instructions: Vec<Instruction>,
}

impl Function {
    // find or insert a local variable
    pub fn find_or_insert_local(&mut self, name: &str) -> usize {
        for (index, variable) in self.locals.iter().enumerate() {
            if variable.name == name {
                return index;
            }
        }
        self.locals.push(Variable {
            name: name.to_string(),
            value: Variant::Integer(0),
        });
        self.locals.len() - 1
    }
}

pub struct Compiler {
    functions: Vec<Function>,
    token_index: usize,
    tokens: Vec<Token>,
}

impl Compiler {
    pub fn new(tokens: Vec<Token>) -> Compiler {
        Compiler {
            functions: Vec::new(),
            token_index: 0,
            tokens,
        }
    }

    pub fn compile(&mut self) -> Program {
        self.functions.push(Function {
            name: "main".to_string(),
            locals: Vec::new(),
            instructions: Vec::new(),
        });

        while self.token_index < self.tokens.len() {
            let Some(token) = self.tokens.get(self.token_index) else {
                panic!("Unexpected end of file");
            };

            match token.token_type {
                TokenType::Var => self.parse_variable_declaration(0),
                _ => unimplemented!("{:?}", token),
            }
        }


        let mut instructions = Vec::new();
        // compile instructions from each function
        for function in &self.functions {
            instructions.extend(function.instructions.clone());
        }

        // insert Halt
        instructions.push(Instruction::Halt);

        Program {
            labels: HashMap::new(),
            instructions,
        }
    }

    fn parse_variable_declaration(&mut self, function_index: usize) {
        consume_token!(self.tokens, self.token_index, TokenType::Var);

        let identifier = expect_token!(self.tokens, self.token_index, TokenType::Identifier);
        let var_name = identifier.lexeme.as_ref().unwrap().to_string();
        let var_index = self.functions[function_index].find_or_insert_local(&var_name);

        trace!("Variable declaration: {:?}", identifier.lexeme);

        consume_token!(self.tokens, self.token_index, TokenType::Equal);
        self.parse_expression(function_index);

        self.functions[function_index].instructions.push(Instruction::SetLocal(var_index));
    }

    fn parse_expression(&mut self, function_index: usize) {
        self.parse_logical_or_expression(function_index);
    }

    fn parse_logical_or_expression(&mut self, function_index: usize) {
        self.parse_logical_and_expression(function_index);

        loop {
            let token = match self.tokens.get(self.token_index) {
                Some(token) => token,
                None => break,
            };

            match token.token_type {
                TokenType::Or => {
                    self.token_index += 1;
                    self.parse_logical_and_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Or);
                }
                _ => break,
            }
        }
    }

    fn parse_logical_and_expression(&mut self, function_index: usize) {
        self.parse_equality_expression(function_index);

        loop {
            let token = match self.tokens.get(self.token_index) {
                Some(token) => token,
                None => break,
            };

            match token.token_type {
                TokenType::And => {
                    self.token_index += 1;
                    self.parse_equality_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::And);
                }
                _ => break,
            }
        }
    }

    fn parse_equality_expression(&mut self, function_index: usize) {
        self.parse_relational_expression(function_index);

        loop {
            let token = match self.tokens.get(self.token_index) {
                Some(token) => token,
                None => break,
            };

            match token.token_type {
                TokenType::DoubleEqual => {
                    self.token_index += 1;
                    self.parse_relational_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Equals);
                }
                TokenType::NotEqual => {
                    self.token_index += 1;
                    self.parse_relational_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::NotEqual);
                }
                _ => break,
            }
        }
    }

    fn parse_relational_expression(&mut self, function_index: usize) {
        self.parse_additive_expression(function_index);

        loop {
            let token = match self.tokens.get(self.token_index) {
                Some(token) => token,
                None => break,
            };

            match token.token_type {
                TokenType::DoubleEqual => {
                    self.token_index += 1;
                    self.parse_additive_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Equals);
                }
                TokenType::NotEqual => {
                    self.token_index += 1;
                    self.parse_additive_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::NotEqual);
                }
                TokenType::LessThan => {
                    self.token_index += 1;
                    self.parse_additive_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::LessThan);
                }
                TokenType::LessThanOrEqual => {
                    self.token_index += 1;
                    self.parse_additive_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::LessEqual);
                }
                TokenType::GreaterThan => {
                    self.token_index += 1;
                    self.parse_additive_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Greater);
                }
                TokenType::GreaterThanOrEqual => {
                    self.token_index += 1;
                    self.parse_additive_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::GreaterEqual);
                }
                _ => break,
            }
        }
    }

    fn parse_additive_expression(&mut self, function_index: usize) {
        self.parse_multiplicative_expression(function_index);

        loop {
            let token = match self.tokens.get(self.token_index) {
                Some(token) => token,
                None => break,
            };

            match token.token_type {
                TokenType::Plus => {
                    self.token_index += 1;
                    self.parse_multiplicative_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Add);
                }
                TokenType::Minus => {
                    self.token_index += 1;
                    self.parse_multiplicative_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Sub);
                }
                _ => break,
            }
        }
    }

    fn parse_multiplicative_expression(&mut self, function_index: usize) {
        self.parse_unary_expression(function_index);

        loop {
            let token = match self.tokens.get(self.token_index) {
                Some(token) => token,
                None => break,
            };

            match token.token_type {
                TokenType::Star => {
                    self.token_index += 1;
                    self.parse_unary_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Mul);
                }
                TokenType::Slash => {
                    self.token_index += 1;
                    self.parse_unary_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Div);
                }
                TokenType::Percent => {
                    self.token_index += 1;
                    self.parse_unary_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Mod);
                }
                _ => break,
            }
        }
    }

    fn parse_unary_expression(&mut self, function_index: usize) {
        let token = match self.tokens.get(self.token_index) {
            Some(token) => token,
            None => panic!("Expected more tokens"),
        };

        match token.token_type {
            TokenType::Minus => {
                self.token_index += 1;
                self.parse_primary_expression(function_index);
                self.functions[function_index].instructions.push(Instruction::Negate);
            }
            TokenType::Not => {
                self.token_index += 1;
                self.parse_primary_expression(function_index);
                self.functions[function_index].instructions.push(Instruction::Not);
            }
            _ => self.parse_primary_expression(function_index),
        }
    }

    fn parse_primary_expression(&mut self, function_index: usize) {
        let token = match self.tokens.get(self.token_index) {
            Some(token) => token,
            None => panic!("Expected more tokens"),
        };

        match token.token_type {
            TokenType::Integer => {
                let value: i64 = token.lexeme.as_ref().unwrap().parse().unwrap();
                self.functions[function_index].instructions.push(Instruction::Push(Variant::Integer(value)));
            }
            TokenType::Float => {
                let value: f64 = token.lexeme.as_ref().unwrap().parse().unwrap();
                self.functions[function_index].instructions.push(Instruction::Push(Variant::Float(value)));
            }
            TokenType::True => {
                self.functions[function_index].instructions.push(Instruction::Push(Variant::Boolean(true)));
            }
            TokenType::False => {
                self.functions[function_index].instructions.push(Instruction::Push(Variant::Boolean(false)));
            }
            TokenType::String => {
                let value = token.lexeme.as_ref().unwrap().to_string();
                self.functions[function_index].instructions.push(Instruction::Push(Variant::String(value)));
            }
            TokenType::Identifier => {
                let identifier = token.lexeme.as_ref().unwrap();
                let var_index = self.functions[function_index].find_or_insert_local(identifier);
                self.functions[function_index].instructions.push(Instruction::LoadLocal(var_index));
            }
            TokenType::LeftParen => {
                self.token_index += 1;
                self.parse_expression(function_index);
            }
            TokenType::RightParen => {
                self.token_index += 1;
            }
            TokenType::Not => {
                self.token_index += 1;
                self.parse_primary_expression(function_index);
                self.functions[function_index].instructions.push(Instruction::Not);
            }
            _ => unimplemented!("{:?}", token),
        }

        self.token_index += 1;
    }
}
