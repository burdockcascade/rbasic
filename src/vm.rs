use std::collections::HashMap;
use log::{debug, trace};
use crate::variant::Variant;

#[derive(Debug, Clone)]
pub enum Instruction {
    Push(Variant),
    SetLocal(usize),
    LoadLocal(usize),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equals,
    Not,
    Return,
    Halt,
    Negate,
    LessThan,
    LessEqual,
    Greater,
    GreaterEqual,
    NotEqual,
    Or,
    And,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub(crate) labels: HashMap<String, usize>,
    pub(crate) instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub id: u32,
    pub locals: Vec<Variant>,
    pub operands: Vec<Variant>,
    pub return_address: Option<usize>,
}

impl StackFrame {
    pub fn new(id: u32) -> StackFrame {
        StackFrame {
            id,
            locals: Vec::new(),
            operands: Vec::new(),
            return_address: None,
        }
    }

    #[inline]
    pub fn pop_operand(&mut self) -> Variant {
        self.operands.pop().unwrap()
    }

    #[inline]
    pub fn push_operand(&mut self, operand: Variant) {
        self.operands.push(operand);
    }

    #[inline]
    pub fn get_local(&self, index: usize) -> Variant {
        self.locals[index].clone()
    }

    #[inline]
    pub fn set_local(&mut self, index: usize, value: Variant) {
        if index >= self.locals.len() {
            self.locals.resize(index + 1, value);
        } else {
            self.locals[index] = value;
        }
    }

}

pub struct Vm {
    pub program: Program,
    pub stack: Vec<StackFrame>,
    pub pc: usize,
}

impl Vm {

    pub fn new(program: Program) -> Vm {
        Vm {
            program,
            stack: Vec::new(),
            pc: 0,
        }
    }

    pub fn run(mut self) {

        if self.program.instructions.is_empty() {
            panic!("No instructions to run");
        }

        let mut frame = StackFrame {
            id: 0,
            locals: Vec::with_capacity(16),
            operands: Vec::with_capacity(16),
            return_address: None,
        };

        loop {

            let Some(instruction) = &self.program.instructions.get(self.pc) else {
                panic!("Invalid program counter");
            };

            trace!("");
            trace!("Frame: {:?}", frame.id);
            trace!("PC: {}", self.pc);
            trace!("{:?}", instruction);

            match instruction {

                Instruction::Push(ref variant) => {
                    frame.push_operand(variant.clone());
                    self.pc += 1;
                },

                Instruction::Pop => {
                    frame.pop_operand();
                    self.pc += 1;
                },

                Instruction::SetLocal(index) => {
                    let value = frame.pop_operand();
                    frame.set_local(*index, value);
                    self.pc += 1;
                },

                Instruction::LoadLocal(index) => {
                    let value = frame.get_local(*index);
                    frame.push_operand(value);
                    self.pc += 1;
                },

                Instruction::Equals => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b == a));
                    self.pc += 1;
                },

                Instruction::Add => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b + a);
                    self.pc += 1;
                },

                Instruction::Sub => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b - a);
                    self.pc += 1;
                },

                Instruction::Mul => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b * a);
                    self.pc += 1;
                },

                Instruction::Div => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b / a);
                    self.pc += 1;
                },

                Instruction::Not => {
                    let a = frame.pop_operand();
                    frame.push_operand(!a);
                    self.pc += 1;
                },

                Instruction::Greater => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b > a));
                    self.pc += 1;
                },

                Instruction::LessThan => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b < a));
                    self.pc += 1;
                },

                Instruction::LessEqual => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b <= a));
                    self.pc += 1;
                },

                Instruction::GreaterEqual => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b >= a));
                    self.pc += 1;
                },

                Instruction::NotEqual => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b != a));
                    self.pc += 1;
                },

                Instruction::Or => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b.as_bool() || a.as_bool()));
                    self.pc += 1;
                },

                Instruction::And => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b.as_bool() && a.as_bool()));
                    self.pc += 1;
                },

                Instruction::Halt => {
                    break;
                },

                _ => unimplemented!("Instruction not implemented: {:?}", instruction),
            }

            trace!("Operands stack: {:?}", frame.operands);
            trace!("Locals stack: {:?}", frame.locals);

        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_compare() {
        let program = Program {
            labels: HashMap::new(),
            instructions: vec![
                Instruction::Push(Variant::Integer(1)),
                Instruction::Push(Variant::Integer(2)),
                Instruction::Add,
                Instruction::Push(Variant::Integer(3)),
                Instruction::Equals,
                Instruction::Halt,
            ],
        };

        let vm = Vm::new(program);
        vm.run();

    }
}