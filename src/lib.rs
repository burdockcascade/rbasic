use variant::Variant;
use crate::compiler::Compiler;
use crate::tokenizer::Tokenizer;

pub mod compiler;
pub mod vm;
pub mod variant;
mod tokenizer;

#[derive(Debug)]
pub enum ScriptError {
    SyntaxError {
        line: usize,
        column: usize,
        message: String,
    },
    CompileError {
        message: String,
    },
    RuntimeError {
        message: String,
    }
}

pub fn run_file(filename: &str, entry_point: Option<String>) -> Result<Option<Variant>, ScriptError> {
    let contents = std::fs::read_to_string(filename).expect("Error reading file");
    evaluate(&contents, entry_point)
}

pub fn evaluate(input: &str, entry_point: Option<String>) -> Result<Option<Variant>, ScriptError> {
    let tokens = Tokenizer::tokenize(input.to_string());

    let program = match Compiler::new(tokens).compile() {
        Ok(program) => program,
        Err(error) => return Err(error),
    };

    let vm = vm::Vm::new(program);
    match vm.run(entry_point) {
        Ok(result) => Ok(result),
        Err(error) => Err(error),
    }
}
