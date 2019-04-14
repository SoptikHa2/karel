use std::collections::HashMap;

use crate::core::*;

pub struct SyntaxParser<'a> {
    pointer: Option<usize>,
    source: Vec<&'a str>,
    methods: HashMap<String, usize>,
    environment: &'a mut Karel,
}

impl<'a> SyntaxParser<'a> {
    /// Create new syntax parser. It takes list of strings that represent
    /// program soRsult<SyntaxParser, Sult<SyntaxParserurce. They are read and methods are found and indexed.
    /// One can then run program with `run` or `step`.
    pub fn new(sources: Vec<String>, environment: &'a mut Karel) -> SyntaxParser {
        let mut sp = SyntaxParser {
            pointer: None,
            source: crate::syntax::SyntaxParser::preprocess(sources),
            methods: HashMap::new(),
            environment,
        };
        sp.index_methods();
        if sp.methods.contains_key("main") {
            sp.pointer = Some(sp.methods["main"]);
        }
        sp
    }

    /// Run method until the program ends or a an error is encountered.
    pub fn run(&mut self) -> Result<(), RuntimeError> {
        match self.pointer {
            None => Err(RuntimeError::NoEntryPointDefined),
            Some(_) => {
                while true {
                    let result = self.step();
                    if result.is_none(){
                        break;
                    }
                    if result.unwrap().is_err() {
                        return Err(result.unwrap().unwrap_err());
                    }
                }
                Ok(())
            }
        }
    }

    /// Run one step of program.
    pub fn step(&mut self) -> Option<Result<(), RuntimeError>> {}

    /// Run one command from user.
    pub fn interactive(&mut self, command: String) -> Result<(), RuntimeError> {}

    /// Take list of source file contents and preprocess it - trimming
    /// whitespaces, removing comments and empty lines.
    ///
    /// Result si list of lines.
    fn preprocess<'b>(source_files_content: Vec<String>) -> Vec<&'b str> {
        let lines: Vec<&str> = Vec::new();
        for source_file in source_files_content {
            for line in source_file.lines() {
                // Remove comments
                let comment_char = line.find("#");
                let parsed_line: &str;
                if let Some(comment_char) = comment_char {
                    parsed_line = &line[0..comment_char];
                } else {
                    parsed_line = &line;
                }
                // Remove whitespaces
                parsed_line = parsed_line.trim();
                if parsed_line.len() != 0 {
                    lines.push(parsed_line);
                }
            }
        }

        lines
    }

    fn index_methods(&mut self) {
        let mut current_index: usize = 0;
        for line in self.source {
            if line.len() > 3 && line.starts_with("def") {
                let method_name = line[3..].trim();
                self.methods[method_name] = current_index;
            }
            current_index += 1;
        }
    }
}

enum RuntimeError {
    /// Main was not found. Consider calling `interactive` instead
    NoEntryPointDefined,
    RuntimeActionError(crate::core::ActionError),
    RuntimeQueryError(crate::core::QueryError),
    RuntimeSyntaxError(SyntaxError),
}

enum SyntaxError {
    /// Method that was called is not defined
    /// (this method should be defined by user)
    MethodNotDefined,
    /// Non-user defined structure that was called is not defined
    /// (such as conditions, loops, and Karel commands)
    NotDefined,
    /// Wrong block end encountered. Make sure you didn't mix up
    /// `endif`, `enddef`, `endrepeat`, `endwhile`
    WrongBlockEnd,
    /// Unexpected end of file encountered. Make sure you included
    /// `endif`, `enddef`, `endrepeat`, or `endwhile`.
    UnexpectedEndOfFile,
}
