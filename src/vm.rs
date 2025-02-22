use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use log::{debug, trace};
use crate::variant::Variant;
use crate::ScriptError;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {

    Assert,

    // Variables
    SetLocal(usize),
    LoadLocal(usize),
    
    CreateArray(usize),
    CreateTable(usize),
    MemberAccess,
    SetMember,

    FunctionCall(String, usize),

    // Stack operations
    Push(Variant),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
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

    Jump(usize),
    JumpIfFalse(usize),
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

    pub fn run(mut self, entry_point: Option<String>) -> Result<Option<Variant>, ScriptError> {

        trace!("Program: {:?}", self.program);
        trace!("Entry point: {:?}", entry_point);
        trace!("Labels: {:?}", self.program.labels);
        
        if self.program.instructions.is_empty() {
            panic!("No instructions to run");
        }
        
        
        self.pc = match entry_point {
            Some(label) => {
                match self.program.labels.get(&label) {
                    Some(pc) => *pc,
                    None => 0
                }
            },
            None => match self.program.labels.get("main") {
                Some(pc) => *pc,
                None => panic!("No entry point found")
            }
        };

        let mut frame = StackFrame::new(0);
        
        trace!("Starting at program counter: {}", self.pc);

        loop {

            trace!("=== Loop iteration ===");

            let Some(instruction) = &self.program.instructions.get(self.pc) else {
                panic!("Invalid program counter");
            };
            
            trace!("Program counter: {}", self.pc);
            trace!("Instruction: {:?}", instruction);
            trace!("Frame ID: {}", frame.id);
            trace!("Frame locals -> {:?}", frame.locals);
            trace!("Frame operands -> {:?}", frame.operands);

            match instruction {

                Instruction::Assert => {
                    let value: bool = frame.pop_operand().into();
                    if !value {
                        return Err(ScriptError::RuntimeError {
                            message: "Assertion failed".to_string()
                        });
                    }
                    self.pc += 1;
                },

                Instruction::Push(ref variant) => {
                    frame.push_operand(variant.clone());
                    self.pc += 1;
                },

                Instruction::Pop => {
                    frame.pop_operand();
                    self.pc += 1;
                },

                // Local variables

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

                Instruction::CreateArray(size) => {
                    let mut array = Vec::with_capacity(*size);
                    for _ in 0..*size {
                        array.push(frame.pop_operand());
                    }
                    array.reverse();
                    frame.push_operand(Variant::Array(Rc::new(RefCell::new(array))));
                    self.pc += 1;
                },

                Instruction::MemberAccess => {
                    let index = frame.pop_operand();
                    let array = frame.pop_operand();
                    let value = match array {
                        Variant::Array(array) => {
                            let array = array.borrow();
                            let index: usize = index.into();
                            match array.get(index) {
                                Some(value) => value.clone(),
                                None => return Err(ScriptError::RuntimeError {
                                    message: "Index out of bounds".to_string()
                                })
                            }
                        },
                        Variant::Table(table) => {
                            let table = table.borrow();
                            let index: String = index.into();
                            match table.get(&index) {
                                Some(value) => value.clone(),
                                None => return Err(ScriptError::RuntimeError {
                                    message: "Key not found".to_string()
                                })
                            }
                        },
                        _ => return Err(ScriptError::RuntimeError {
                            message: "Not an array nor table".to_string()
                        })
                    };
                    frame.push_operand(value);
                    self.pc += 1;
                },

                Instruction::SetMember => {
                    let value = frame.pop_operand();
                    let index = frame.pop_operand();
                    let array = frame.pop_operand();
                    match array {
                        Variant::Array(array) => {
                            let mut array = array.borrow_mut();
                            let index: usize = index.into();
                            array[index] = value;
                        },
                        Variant::Table(table) => {
                            let mut table = table.borrow_mut();
                            let index: String = index.into();
                            table.insert(index, value);
                        },
                        _ => return Err(ScriptError::RuntimeError {
                            message: "Not an array nor table".to_string()
                        })
                    }
                    self.pc += 1;
                },

                Instruction::CreateTable(size) => {
                    let mut table = HashMap::new();
                    for _ in 0..*size {
                        let value = frame.pop_operand();
                        let key = frame.pop_operand();
                        let key: String = key.into();
                        table.insert(key, value);
                    }
                    frame.push_operand(Variant::Table(Rc::new(RefCell::new(table))));
                    self.pc += 1;
                },

                // Function calls

                Instruction::FunctionCall(ref label, num_args) => {

                    let pc = match self.program.labels.get(label) {
                        Some(pc) => pc,
                        None => return Err(ScriptError::RuntimeError {
                            message: format!("Label not found: {}", label)
                        })
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
                                return Err(ScriptError::RuntimeError {
                                    message: "Too many items on the stack".to_string()
                                });
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

                // Jump instructions

                Instruction::Jump(address) => {
                    self.pc = *address;
                },

                Instruction::JumpIfFalse(address) => {
                    let value: bool = frame.pop_operand().into();
                    if !value {
                        self.pc = *address;
                    } else {
                        self.pc += 1;
                    }
                },

                // Comparison instructions

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
                    let result: bool = b.into() || a.into();
                    frame.push_operand(Variant::Boolean(result));
                    self.pc += 1;
                },

                Instruction::And => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    let result: bool = b.into() && a.into();
                    frame.push_operand(Variant::Boolean(result));
                    self.pc += 1;
                },

                Instruction::Mod => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b % a);
                    self.pc += 1;
                },

                Instruction::Pow => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b.pow(&a));
                    self.pc += 1;
                },

                Instruction::Negate => {
                    let a = frame.pop_operand();
                    frame.push_operand(-a);
                    self.pc += 1;
                },

                Instruction::Halt => {
                    break;
                },

            }

            trace!("Frame locals <- {:?}", frame.locals);
            trace!("Frame operands <- {:?}", frame.operands);

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
    
    #[test]
    fn test_add_and_compare_false() {
        let program = Program {
            labels: HashMap::new(),
            instructions: vec![
                Instruction::Push(Variant::Integer(1)),
                Instruction::Push(Variant::Integer(2)),
                Instruction::Add,
                Instruction::Push(Variant::Integer(4)),
                Instruction::Equals,
                Instruction::Halt,
            ],
        };

        let vm = Vm::new(program);
        vm.run(None);

    }
}