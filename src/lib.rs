use crate::compiler::Compiler;
use crate::tokenizer::Tokenizer;

mod tokenizer;
mod compiler;
mod vm;
mod variant;

pub fn run_file(filename: &str) {

    let contents = std::fs::read_to_string(filename).expect("Error reading file");

    let tokens = Tokenizer::tokenize(contents);

    let program = Compiler::new(tokens).compile();

    let vm = vm::Vm::new(program);
    vm.run();

}
