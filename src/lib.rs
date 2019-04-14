use std::fs;
use std::io::stdin;
use std::io::Read;
use std::error::Error;

mod syntax;
mod core;

/// Run with selected parameters. Use stdin and stdout for communication.
pub fn run(interactive: bool, source: Option<&str>, libraries: Option<Vec<&str>>, ignore_runtime_errors: bool) {

}

fn load_from_file_or_stdin(filename_or_slash: &str) -> Result<String, Box<dyn Error>> {
    if filename_or_slash == "-" {
        // Load from stdin
        let mut buffer: Vec<u8> = Vec::new();
        stdin().read_to_end(&mut buffer)?;
        return Ok(String::from_utf8(buffer)?);
    }else{
        // Load from file
        return Ok(fs::read_to_string(&filename_or_slash)?);
    }
}

fn run_interactive(libraries: Vec<&str>) {

}

fn run_from_source(source: &str, libraries: Vec<&str>) {

}

fn print_state(karel: &core::Karel){
    
}