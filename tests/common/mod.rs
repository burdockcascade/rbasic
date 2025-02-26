use rbasic::evaluate;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

pub fn test_script(script: &str) {
    //TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("Logger error");
    assert!(evaluate(script, None).is_ok());
}

pub fn test_code_snippet_ok(script: &str) {
    let script = format!("function main() {} end", script);
    test_script(script.as_str());
}