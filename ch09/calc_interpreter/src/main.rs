mod analyzer;
mod executor;
mod parser;
mod symbol_table;

fn main() {
    run_interpreter();
}

fn run_interpreter() {
    eprintln!("* Calc interactive interpreter *");
    let mut variables = symbol_table::SymbolTable::new();
    loop {
        let command = input_command();
        if command.len() == 0 {
            break;
        }
        match command.trim() {
            "q" => break,
            "c" => {
                variables = symbol_table::SymbolTable::new();
                eprintln!("Cleared variables.");
            }
            "v" => {
                eprintln!("Variables:");
                for v in variables.iter() {
                    eprintln!("  {}: {}", v.0, v.1);
                }
            }
            trimmed_command => match parser::parse_program(&trimmed_command) {
                Ok((rest, parsed_program)) => {
                    if rest.len() > 0 {
                        eprintln!("Unparsed input: `{}`.", rest)
                    } else {
                        match analyzer::analyze_program(&mut variables, &parsed_program) {
                            Ok(analyzed_program) => {
                                executor::execute_program(&mut variables, &analyzed_program)
                            }
                            Err(err) => eprintln!("Error: {}", err),
                        }
                    }
                }
                Err(err) => eprintln!("Error: {:?}", err),
            },
        }
    }
}

fn input_command() -> String {
    let mut text = String::new();
    eprint!("> ");
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line.");
    text
}
