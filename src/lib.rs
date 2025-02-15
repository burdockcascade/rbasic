use log::__private_api::Value;
use crate::compiler::Compiler;
use crate::tokenizer::Tokenizer;
use crate::variant::Variant;

mod tokenizer;
mod compiler;
mod vm;
mod variant;

pub struct ExecutionResult {
    pub(crate) return_value: variant::Variant,
}

pub fn run_file(filename: &str) -> Result<Variant, String> {
    let contents = std::fs::read_to_string(filename).expect("Error reading file");
    return evaluate(&contents);
}

pub fn evaluate(input: &str) -> Result<Variant, String> {
    let tokens = Tokenizer::tokenize(input.to_string());

    let program = Compiler::new(tokens).compile();

    let vm = vm::Vm::new(program);
    match vm.run() {
        Ok(Some(v)) => Ok(v),
        Ok(None) => Ok(Variant::Null),
        Err(e) => Err(e),
    }
}
