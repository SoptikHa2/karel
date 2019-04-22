use std::error::Error;
use std::fs;
use std::io::stdin;
use std::io::Read;

mod core;
mod syntax;

/// Run with selected parameters. Use stdin and stdout for communication.
pub fn run(
    interactive: bool,
    source: Option<&str>,
    libraries: Option<Vec<&str>>,
    ignore_runtime_errors: bool,
) {
    // Load libraries
    println!("{:?}", source);
    let mut libs: Vec<String> = Vec::new();
    if let Some(library_paths) = libraries {
        let mut prepared_strings: Vec<String> = library_paths
            .iter()
            .map(|s| load_from_file_or_stdin(s).unwrap())
            .collect();
        libs.append(&mut prepared_strings);
    }

    if interactive {
        run_interactive(libs, ignore_runtime_errors);
    } else {
        let source = load_from_file_or_stdin(source.unwrap()).unwrap();
        run_from_source(source, libs, ignore_runtime_errors);
    }
}

fn load_from_file_or_stdin(filename_or_slash: &str) -> Result<String, Box<dyn Error>> {
    if filename_or_slash == "-" {
        // Load from stdin
        let mut buffer: Vec<u8> = Vec::new();
        stdin().read_to_end(&mut buffer)?;
        return Ok(String::from_utf8(buffer)?);
    } else {
        // Load from file
        return Ok(fs::read_to_string(&filename_or_slash)?);
    }
}

fn run_interactive(libraries: Vec<String>, ignore_runtime_errors: bool) {}

fn run_from_source(source: String, libraries: Vec<String>, ignore_runtime_errors: bool) {
    let mut vec = libraries;
    vec.push(source);

    let mut karel = core::Karel::new(core::Config::default());
    let syntax_parser = syntax::SyntaxParser::new(vec);

    let result = syntax_parser.run(&mut karel);
    match result {
        Ok(_) => {
            // Print result
            karel.print_karel();
        },
        Err(x) => {
            println!("An error occured: {}", x);
        }
    }
}