use std::env;
use std::fs::File;
use std::io::ErrorKind;
use log::{error, info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use rbasic::run_file;

fn main() {
    
    // enable trace logging
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("TODO: panic message");
    
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a filename is provided
    if args.len() < 2 {
        error!("Please provide a filename as an argument.");
        return;
    }

    // Get the filename
    let filename = &args[1];

    // Attempt to open the file
    let file_result = File::open(filename);

    // Handle the result
    match file_result {
        Ok(_) => {
            info!("Successfully opened file: {}", filename);
            run_file(filename);
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                error!("Error: File '{}' not found.", filename);
            }
            other_error => {
                error!("Error opening file: {:?}", other_error);
            }
        },
    }
}