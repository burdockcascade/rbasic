use std::collections::HashMap;
use log::{debug, trace};
use crate::variant::Variant;
use crate::vm::{Instruction, Program};
use crate::ScriptError;
use crate::tokenizer::{Token, TokenType};

struct Variable {
    name: String
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
            name: name.to_string()
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

    pub fn compile(&mut self) -> Result<Program, ScriptError> {
        debug!("Compiling program");

        while self.token_index < self.tokens.len() {
            let Some(token) = self.tokens.get(self.token_index) else {
                return Err(ScriptError::CompileError {
                    message: "Unexpected end of file".to_string(),
                });
            };

            match token.token_type {
                TokenType::Function => self.compile_function(),
                TokenType::EndOfInput => break,
                _ => unimplemented!("{:?}", token),
            }
        }

        let mut p = Program {
            labels: HashMap::new(),
            instructions: Vec::new()
        };

        // add functions to program
        for function in &self.functions {
            trace!("Function: {}", function.name);
            p.labels.insert(function.name.clone(), p.instructions.len());
            p.instructions.append(&mut function.instructions.clone());
        }

        Ok(p)
    }
    
    fn fetch_and_consume_token(&mut self, token_type: TokenType) -> &Token {
        let token = match self.tokens.get(self.token_index) {
            Some(token) => token,
            None => panic!("Expected more tokens"),
        };

        if token.token_type != token_type {
            panic!("Unexpected token {:?} but wanted {:?}", token, token_type);
        }

        self.token_index += 1;
        token
    }

    fn consume_token(&mut self, token_type: TokenType) {
        let _ = self.fetch_and_consume_token(token_type);
    }
    
    fn consume_optional_token(&mut self, token_type: TokenType){
        if self.current_token_is(token_type) {
            self.token_index += 1;

        }
    }
    
    fn next_token(&mut self) {
        self.token_index += 1;
    }
    
    fn current_token(&self) -> &Token {
        match self.tokens.get(self.token_index) {
            Some(token) => token,
            None => panic!("Expected more tokens"),
        }
    }
    
    fn peek_at_index(&self, index: usize) -> &Token {
        match self.tokens.get(index) {
            Some(token) => token,
            None => panic!("Expected more tokens"),
        }
    }
    
    fn peek_at_next_token(&self) -> &Token {
        self.peek_at_index(self.token_index + 1)
    }

    fn next_token_is(&self, token_type: TokenType) -> bool {
        match self.tokens.get(self.token_index + 1) {
            Some(token) => token.token_type == token_type,
            None => false,
        }
    }

    fn next_token_is_one_of(&self, token_types: &[TokenType]) -> bool {
        match self.tokens.get(self.token_index + 1) {
            Some(token) => token_types.contains(&token.token_type),
            None => false,
        }
    }
    
    fn current_token_is(&self, token_type: TokenType) -> bool {
        match self.tokens.get(self.token_index) {
            Some(token) => token.token_type == token_type,
            None => false,
        }
    }
    
    fn trace_current_token(&self) {
        trace!("Current token: {:?}", self.current_token());
    }
    
    fn trace_next_token(&self) {
        trace!("Next token: {:?}", self.peek_at_next_token());
    }
}

// Function parsing
impl Compiler {
    
    fn compile_assertion(&mut self, function_index: usize) {
        trace!("Compiling assertion");
        self.consume_token(TokenType::Assert);
        self.compile_expression(function_index);
        self.functions[function_index].instructions.push(Instruction::Assert);
        trace!("Assertion compiled");
    }

    fn compile_block(&mut self, function_index: usize) {
        
        self.consume_optional_token(TokenType::Do);

        while self.token_index < self.tokens.len() {
            
            let token = self.current_token();

            trace!("=== Compiling statement ===");

            match token.token_type {
                TokenType::End => {
                    trace!("End of block");
                    self.next_token();
                    break;
                }
                TokenType::Identifier => {
                    if self.next_token_is(TokenType::LeftParen) {
                        self.compile_function_call(function_index, false);
                    } else if self.next_token_is_one_of(&[TokenType::Equal, TokenType::LeftBracket, TokenType::Dot]) {
                        self.compile_variable_assignment(function_index);
                    } else {
                        panic!("Unexpected token {:?}", token);
                    }
                }
                TokenType::Var => self.compile_variable_declaration(function_index),
                TokenType::If => self.compile_if_statement(function_index),
                TokenType::Return => self.compile_return_statement(function_index),
                TokenType::Assert => self.compile_assertion(function_index),
                TokenType::While => self.compile_while_statement(function_index),
                TokenType::Semicolon => self.token_index += 1,
                TokenType::EndOfInput => break,
                _ => unimplemented!("{:?}", token),
            }
        }
    }

    fn compile_return_statement(&mut self, function_index: usize) {
        trace!("Compiling return statement");
        self.next_token();
        self.compile_expression(function_index);
        self.functions[function_index].instructions.push(Instruction::Return);
        trace!("Return statement compiled");
    }

    fn compile_function(&mut self) {
        self.consume_token(TokenType::Function);

        let identifier = self.fetch_and_consume_token(TokenType::Identifier);
        let function_name = identifier.lexeme.as_ref().unwrap().to_string();

        debug!("Compiling function: {}", function_name);

        self.consume_token(TokenType::LeftParen);

        let mut parameters = Vec::new();

        loop {
            let token = self.current_token();

            match token.token_type {
                TokenType::Identifier => {
                    let identifier = token.lexeme.as_ref().unwrap().to_string();
                    parameters.push(identifier);
                    self.next_token();
                }
                TokenType::Comma => {
                    self.next_token();
                }
                TokenType::RightParen => {
                    self.next_token();
                    break;
                }
                _ => panic!("Unexpected token {:?}", token),
            }
            
        }

        self.functions.push(Function {
            name: function_name,
            locals: Vec::new(),
            instructions: Vec::new(),
        });

        // move values off stack and into locals
        for parameter in parameters {
            let var_index = self.functions.last_mut().unwrap().find_or_insert_local(&parameter);
            self.functions.last_mut().unwrap().instructions.push(Instruction::SetLocal(var_index));
        }

        let function_index = self.functions.len() - 1;
        self.compile_block(function_index);

        // if last instruction is not return then add a return instruction
        if self.functions.last().unwrap().instructions.last() != Some(&Instruction::Return) {
            self.functions.last_mut().unwrap().instructions.push(Instruction::Push(Variant::Integer(0)));
            self.functions.last_mut().unwrap().instructions.push(Instruction::Return);
        }
    }

    fn compile_variable_declaration(&mut self, function_index: usize) {
        trace!("Variable declaration start");
        self.consume_token(TokenType::Var);
        self.compile_variable_assignment(function_index);
        trace!("Variable declaration end");
    }
    
    fn compile_variable_assignment(&mut self, function_index: usize) {
        
        // fetch variable name
        let identifier = self.fetch_and_consume_token(TokenType::Identifier);
        let var_name = identifier.lexeme.as_ref().unwrap().to_string();
        let var_index = self.functions[function_index].find_or_insert_local(&var_name);
        
        trace!("Variable assignment start for {}", var_name);

        // if next token is an assignment operator then it's a simple assignment
        if self.current_token_is(TokenType::Equal) {
            self.token_index += 1;
            self.compile_expression(function_index);
            self.functions[function_index].instructions.push(Instruction::SetLocal(var_index));
            trace!("Variable assignment (simple) end for {}", var_name);
            return;
        }
        
        // Load the variable
        trace!("Fetching variable: {}", var_name);
        self.functions[function_index].instructions.push(Instruction::LoadLocal(var_index));
        
        // otherwise it's a member access or array access
        loop {
            trace!("looping");
            match self.current_token().token_type {
                TokenType::Equal => {
                    self.token_index += 1;
                    self.compile_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::SetMember);
                    trace!("Variable assignment (equal) end for {}", var_name);
                    return;
                }
                TokenType::LeftBracket => {
                    trace!("Array access: {}", var_name);
                    self.consume_token(TokenType::LeftBracket);
                    self.compile_expression(function_index);
                    self.consume_token(TokenType::RightBracket);
                    if self.current_token().token_type == TokenType::Equal {
                        trace!("Array equal");
                        self.token_index += 1;
                        self.compile_expression(function_index);
                        self.functions[function_index].instructions.push(Instruction::SetMember);
                        trace!("Variable assignment (leftbracket) end for {}", var_name);
                        break;
                    } else {
                        self.functions[function_index].instructions.push(Instruction::MemberAccess);
                    }
                }
                TokenType::Dot => {
                    self.token_index += 1;
                    self.compile_member_access(function_index, var_name.clone());
                    self.consume_token(TokenType::Equal);
                    self.compile_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::SetLocal(var_index));
                }
                _ => panic!("Unexpected token {:?}", self.peek_at_next_token()),
            }
        }
    }

    fn compile_function_call(&mut self, function_index: usize, is_assignment: bool) {
        
        let identifier = self.fetch_and_consume_token(TokenType::Identifier).lexeme.as_ref().unwrap().to_string();
        
        trace!("Compiling function call: {}", identifier);
        
        self.consume_token(TokenType::LeftParen);

        let mut arg_count = 0;

        // compile arguments
        loop {
            match self.current_token().token_type {
                TokenType::RightParen => {
                    trace!("Function call arguments compiled");
                    self.next_token();
                    break;
                }
                TokenType::Comma => {
                    self.next_token();
                }
                _ => {
                    trace!("Compiling function call argument");
                    self.compile_expression(function_index);
                    arg_count += 1;
                    trace!("Function call argument compiled");
                }
            }
        }

        // add function call instruction
        self.functions[function_index].instructions.push(Instruction::FunctionCall(identifier.to_string(), arg_count));

        // if not an assignment, pop the result
        if !is_assignment {
            //self.functions[function_index].instructions.push(Instruction::Pop);
        }
        
        trace!("Function call compiled for {} with {} arguments", identifier, arg_count);
    }
    
}

// Control flow parsing
impl Compiler {

    fn compile_if_statement(&mut self, function_index: usize) {

        self.consume_token(TokenType::If);

        self.compile_expression(function_index);

        if self.next_token_is_one_of(&[TokenType::Then, TokenType::Do]) {
            self.token_index += 1;
        }

        let then_index = self.functions[function_index].instructions.len();
        self.functions[function_index].instructions.push(Instruction::JumpIfFalse(0));

        self.compile_block(function_index);

        let end_index = self.functions[function_index].instructions.len();
        self.functions[function_index].instructions[then_index] = Instruction::JumpIfFalse(end_index);
    }

    fn compile_while_statement(&mut self, function_index: usize) {
        self.consume_token(TokenType::While);

        let start_index = self.functions[function_index].instructions.len();

        self.compile_expression(function_index);

        self.consume_token(TokenType::Do);

        let then_index = self.functions[function_index].instructions.len();
        self.functions[function_index].instructions.push(Instruction::JumpIfFalse(0));

        self.compile_block(function_index);

        self.functions[function_index].instructions.push(Instruction::Jump(start_index));

        let end_index = self.functions[function_index].instructions.len();
        self.functions[function_index].instructions[then_index] = Instruction::JumpIfFalse(end_index);
    }
    
}


// Expression parsing
impl Compiler {

    fn compile_expression(&mut self, function_index: usize) {
        trace!("Starting expression");
        self.compile_logical_or_expression(function_index);
        trace!("Ending expression");
    }

    fn compile_logical_or_expression(&mut self, function_index: usize) {
        self.compile_logical_and_expression(function_index);

        loop {
            match self.current_token().token_type {
                TokenType::Or => {
                    self.next_token();
                    self.compile_logical_and_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Or);
                }
                _ => break,
            }
        }
    }

    fn compile_logical_and_expression(&mut self, function_index: usize) {
        self.compile_relational_expression(function_index);

        loop {
            match self.current_token().token_type {
                TokenType::And => {
                    self.next_token();
                    self.compile_relational_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::And);
                }
                _ => break,
            }
        }
    }

    fn compile_relational_expression(&mut self, function_index: usize) {
        self.compile_additive_expression(function_index);

        loop {
            match self.current_token().token_type {
                TokenType::DoubleEqual => {
                    self.next_token();
                    self.compile_additive_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Equals);
                }
                TokenType::NotEqual => {
                    self.next_token();
                    self.compile_additive_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::NotEqual);
                }
                TokenType::LessThan => {
                    self.next_token();
                    self.compile_additive_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::LessThan);
                }
                TokenType::LessThanOrEqual => {
                    self.next_token();
                    self.compile_additive_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::LessEqual);
                }
                TokenType::GreaterThan => {
                    self.next_token();
                    self.compile_additive_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Greater);
                }
                TokenType::GreaterThanOrEqual => {
                    self.next_token();
                    self.compile_additive_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::GreaterEqual);
                }
                _ => break,
            }
        }
    }

    fn compile_additive_expression(&mut self, function_index: usize) {
        self.compile_multiplicative_expression(function_index);

        loop {
            match self.current_token().token_type {
                TokenType::Plus => {
                    self.token_index += 1;
                    self.compile_multiplicative_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Add);
                }
                TokenType::Minus => {
                    self.token_index += 1;
                    self.compile_multiplicative_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Sub);
                }
                _ => break,
            }
        }
    }

    fn compile_multiplicative_expression(&mut self, function_index: usize) {
        self.compile_unary_expression(function_index);
        
        loop {
            match self.current_token().token_type {
                TokenType::Star => {
                    self.token_index += 1;
                    self.compile_unary_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Mul);
                }
                TokenType::Slash => {
                    self.token_index += 1;
                    self.compile_unary_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Div);
                }
                TokenType::Percent => {
                    self.token_index += 1;
                    self.compile_unary_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Mod);
                },
                TokenType::Caret => {
                    self.token_index += 1;
                    self.compile_unary_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::Pow);
                }
                _ => break,
            }
        }
    }

    fn compile_unary_expression(&mut self, function_index: usize) {
        match self.current_token().token_type {
            TokenType::Minus => { // fixme: this may be broken
                self.token_index += 1;
                self.compile_primary_expression(function_index);
                self.functions[function_index].instructions.push(Instruction::Negate);
            }
            TokenType::Not | TokenType::Bang => {
                self.token_index += 1;
                self.compile_primary_expression(function_index);
                self.functions[function_index].instructions.push(Instruction::Not);
            }
            _ => self.compile_primary_expression(function_index),
        }
    }

    fn compile_primary_expression(&mut self, function_index: usize) {
        
        trace!("Primary expression: {:?}", self.current_token());

        match self.current_token().token_type {
            TokenType::LeftParen => {
                self.next_token();
                self.compile_expression(function_index);
                self.next_token();
            },
            TokenType::RightParen => {
                self.next_token();
            },
            TokenType::Integer => {
                let value: i64 = self.current_token().lexeme.as_ref().unwrap().parse().unwrap();
                self.functions[function_index].instructions.push(Instruction::Push(Variant::Integer(value)));
                self.next_token();
            },
            TokenType::Float => {
                let value: f64 = self.current_token().lexeme.as_ref().unwrap().parse().unwrap();
                self.functions[function_index].instructions.push(Instruction::Push(Variant::Float(value)));
                self.next_token();
            },
            TokenType::True => {
                self.functions[function_index].instructions.push(Instruction::Push(Variant::Boolean(true)));
                self.next_token();
            },
            TokenType::False => {
                self.functions[function_index].instructions.push(Instruction::Push(Variant::Boolean(false)));
                self.next_token();
            },
            TokenType::String => {
                let value = self.current_token().lexeme.as_ref().unwrap().to_string();
                self.functions[function_index].instructions.push(Instruction::Push(Variant::String(value)));
                self.next_token();
            },
            TokenType::Identifier => self.compile_read_identifier(function_index),
            TokenType::LeftBracket => self.compile_array_declaration(function_index),
            TokenType::LeftBrace => self.compile_table_declaration(function_index),
            _ => unimplemented!("{:?}", self.current_token()),
        }
    }
    
    fn compile_read_identifier(&mut self, function_index: usize) {
        let identifier = self.current_token();
        let name = identifier.lexeme.as_ref().unwrap().to_string();
        
        trace!("Starting identifier chain: {}", name);
        
        match self.next_token_is(TokenType::LeftParen) {
            true => {
                self.compile_function_call(function_index, false);
            },
            false => {
                let var_index = self.functions[function_index].find_or_insert_local(name.as_str());
                self.functions[function_index].instructions.push(Instruction::LoadLocal(var_index));
            }
        }
        
        self.next_token();
        
        // break if no more tokens
        if self.token_index >= self.tokens.len() {
            return;
        }
        
        loop {
            match self.current_token().token_type {
                TokenType::LeftParen => {
                    self.compile_function_call(function_index, false);
                },
                TokenType::LeftBracket => {
                    trace!("Compiling array access");
                    self.next_token();
                    self.compile_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::MemberAccess);
                    self.consume_token(TokenType::RightBracket);
                    trace!("Array access compiled");
                },
                TokenType::Dot => {
                    trace!("Compiling member access");
                    self.next_token();
                    self.compile_expression(function_index);
                    self.functions[function_index].instructions.push(Instruction::MemberAccess);
                    trace!("Member access compiled");
                },
                _ => {
                    trace!("Identifier chain compiled");
                    break;
                }
            }
        }
    }

    fn compile_array_declaration(&mut self, function_index: usize) {

        self.consume_token(TokenType::LeftBracket);

        let mut elements = 0;

        loop {
            match self.tokens.get(self.token_index) {
                Some(token) => match token.token_type {
                    TokenType::RightBracket => {
                        self.next_token();
                        break
                    },
                    TokenType::Comma => self.token_index += 1,
                    _ => {
                        self.compile_expression(function_index);
                        elements += 1;
                    }
                },
                None => break,
            }
        }

        self.functions[function_index].instructions.push(Instruction::CreateArray(elements));

    }

    fn compile_table_declaration(&mut self, function_index: usize) {

        self.consume_token(TokenType::LeftBrace);

        let mut elements = 0;

        loop {
            match self.tokens.get(self.token_index) {
                Some(token) => match token.token_type {
                    TokenType::RightBrace => {
                        self.next_token();
                        break
                    },
                    TokenType::Comma => self.token_index += 1,
                    _ => {
                        self.compile_expression(function_index);
                        self.consume_token(TokenType::Colon);
                        self.compile_expression(function_index);
                        elements += 1;
                    }
                },
                None => break,
            }
        }

        self.functions[function_index].instructions.push(Instruction::CreateTable(elements));

    }

    fn compile_member_access(&mut self, function_index: usize, identifier: String) {

        trace!("Member access: {}", identifier);

        let var_index = self.functions[function_index].find_or_insert_local(&identifier);
        self.functions[function_index].instructions.push(Instruction::LoadLocal(var_index));

        self.token_index += 1;
        self.compile_expression(function_index);

        //self.consume_token(TokenType::RightBracket);

        self.functions[function_index].instructions.push(Instruction::MemberAccess);
    }

}

