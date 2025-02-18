use std::collections::HashMap;
use log::{debug, trace};
use crate::runtime::variant::Variant;

#[derive(Debug, Clone)]
pub enum Instruction {

    Push(Variant),

    // Variables
    SetLocal(usize),
    LoadLocal(usize),

    FunctionCall(String, usize),

    // Stack operations
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
    pub labels: HashMap<String, usize>,
    pub instructions: Vec<Instruction>,
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
            locals: Vec::with_capacity(16),
            operands: Vec::with_capacity(16),
            return_address: None,
        }
    }

    pub fn pop_operand(&mut self) -> Variant {
        self.operands.pop().unwrap()
    }

    pub fn push_operand(&mut self, operand: Variant) {
        self.operands.push(operand);
    }

    pub fn get_local(&self, index: usize) -> Variant {
        if index >= self.locals.len() {
            panic!("Local variable not found: {}", index);
        } else {
            self.locals[index].clone()
        }
    }

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
            stack: Vec::with_capacity(16),
            pc: 0,
        }
    }

    pub fn run(mut self, entry_point: Option<String>) -> Result<Option<Variant>, String> {

        trace!("Program: {:?}", self.program);
        trace!("Entry point: {:?}", entry_point);
        trace!("Labels: {:?}", self.program.labels);

        debug!("Running program");

        if self.program.instructions.is_empty() {
            panic!("No instructions to run");
        }

        match entry_point {
            Some(label) => {
                match self.program.labels.get(&label) {
                    Some(pc) => self.pc = *pc,
                    None => self.pc = 0
                }
            },
            None => self.pc = 0
        }

        let mut frame = StackFrame::new(0);

        loop {

            trace!(">>> Loop iteration");

            let Some(instruction) = &self.program.instructions.get(self.pc) else {
                panic!("Invalid program counter");
            };
            
            trace!("Program counter -> {}", self.pc);
            trace!("Frame -> {:?}", frame);
            trace!("Instruction -> {:?}", instruction);

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

                Instruction::FunctionCall(ref label, num_args) => {

                    let pc = match self.program.labels.get(label) {
                        Some(pc) => pc,
                        None => panic!("Label not found: {}", label)
                    };

                    // Create a new frame
                    let mut new_frame = StackFrame::new(frame.id + 1);
                    new_frame.return_address = Some(self.pc + 1);

                    // copy values from stack into new frame arguments
                    for _ in 0..*num_args {
                        let value = frame.pop_operand();
                        new_frame.push_operand(value);
                    }

                    // Push the current frame onto the stack
                    self.stack.push(frame);

                    // Set the new frame as the current frame
                    frame = new_frame;
                    self.pc = *pc;
                },

                Instruction::Return => {
                    match frame.return_address {
                        Some(address) => {

                            if frame.operands.len() > 1 {
                                panic!("Too many return values");
                            }

                            let return_value = if frame.operands.is_empty() {
                                Variant::Null
                            } else {
                                frame.pop_operand()
                            };

                            self.pc = address;
                            frame = self.stack.pop().unwrap();

                            frame.push_operand(return_value);
                        },
                        None => {
                            let return_value = if frame.operands.is_empty() {
                                Variant::Null
                            } else {
                                frame.pop_operand()
                            };
                            
                            return Ok(Some(return_value));
                        }
                    }
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

            trace!("Frame <- {:?}", frame);

        }

        debug!("Program halted");

        if frame.operands.is_empty() {
            Ok(None)
        } else {
            Ok(Some(frame.pop_operand()))
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
        vm.run(None);

    }
}