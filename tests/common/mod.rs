use rbasic::evaluate;
use log::{error, info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

pub fn test_script(script: &str) {
    //TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("Logger error");
    assert!(evaluate(script, None).is_ok());
}

pub fn test_code_snippet(script: &str) {
    let script = format!("{} {} {}", "function main()", script, "end");
    test_script(script.as_str());
}