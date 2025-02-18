use runtime::{variant, vm};
use compilation::compiler::Compiler;
use compilation::tokenizer::Tokenizer;
use runtime::variant::Variant;

mod runtime;
mod compilation;

pub struct ExecutionResult {
    pub(crate) return_value: variant::Variant,
}

pub fn run_file(filename: &str, entry_point: Option<String>) -> Result<Variant, String> {
    let contents = std::fs::read_to_string(filename).expect("Error reading file");
    evaluate(&contents, entry_point)
}

pub fn evaluate(input: &str, entry_point: Option<String>) -> Result<Variant, String> {
    let tokens = Tokenizer::tokenize(input.to_string());

    let program = Compiler::new(tokens).compile();

    let vm = vm::Vm::new(program);
    match vm.run(entry_point) {
        Ok(Some(v)) => Ok(v),
        Ok(None) => Ok(Variant::Null),
        Err(e) => Err(e),
    }
}
