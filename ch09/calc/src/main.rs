extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_while,
    character::complete::{alpha1, char},
    combinator::map,
    multi::many0,
    number::complete::double,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
enum ParsedFactor<'a> {
    Literal(f64),
    Identifier(&'a str),
    SubExpression(Box<ParsedExpr<'a>>),
}

#[derive(Debug, PartialEq)]
enum AnalyzedFactor {
    Literal(f64),
    Identifier(usize),
    SubExpression(Box<AnalyzedExpr>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TermOperator {
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ExprOperator {
    Add,
    Subtract,
}

type ParsedTerm<'a> = (ParsedFactor<'a>, Vec<(TermOperator, ParsedFactor<'a>)>);

type AnalyzedTerm = (AnalyzedFactor, Vec<(TermOperator, AnalyzedFactor)>);

type ParsedExpr<'a> = (ParsedTerm<'a>, Vec<(ExprOperator, ParsedTerm<'a>)>);

type AnalyzedExpr = (AnalyzedTerm, Vec<(ExprOperator, AnalyzedTerm)>);

#[derive(Debug)]
enum ParsedStatement<'a> {
    Declaration(&'a str),
    InputOperation(&'a str),
    OutputOperation(ParsedExpr<'a>),
    Assignment(&'a str, ParsedExpr<'a>),
}

#[derive(Debug)]
enum AnalyzedStatement {
    Declaration(usize),
    InputOperation(usize),
    OutputOperation(AnalyzedExpr),
    Assignment(usize, AnalyzedExpr),
}

type ParsedProgram<'a> = Vec<ParsedStatement<'a>>;

type AnalyzedProgram = Vec<AnalyzedStatement>;

fn parse_program(input: &str) -> IResult<&str, ParsedProgram> {
    many0(preceded(
        skip_spaces,
        alt((
            parse_declaration,
            parse_input_statement,
            parse_output_statement,
            parse_assignment,
        )),
    ))(input)
}

fn parse_declaration(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((char('@'), parse_identifier))(input)
        .map(|(input, output)| (input, ParsedStatement::Declaration(output.1)))
}

fn parse_input_statement(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((char('>'), parse_identifier))(input)
        .map(|(input, output)| (input, ParsedStatement::InputOperation(output.1)))
}

fn parse_output_statement(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((char('<'), parse_expr))(input)
        .map(|(input, output)| (input, ParsedStatement::OutputOperation(output.1)))
}

fn parse_assignment(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((
        parse_identifier,
        skip_spaces,
        tag(":="),
        skip_spaces,
        parse_expr,
    ))(input)
    .map(|(input, output)| (input, ParsedStatement::Assignment(output.0, output.4)))
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_subexpr(input: &str) -> IResult<&str, ParsedExpr> {
    delimited(
        preceded(skip_spaces, char('(')),
        parse_expr,
        preceded(skip_spaces, char(')')),
    )(input)
}

fn parse_factor(input: &str) -> IResult<&str, ParsedFactor> {
    preceded(
        skip_spaces,
        alt((
            map(parse_identifier, ParsedFactor::Identifier),
            map(double, ParsedFactor::Literal),
            map(parse_subexpr, |expr| {
                ParsedFactor::SubExpression(Box::new(expr))
            }),
        )),
    )(input)
}

fn parse_term(input: &str) -> IResult<&str, ParsedTerm> {
    tuple((
        parse_factor,
        many0(tuple((
            preceded(
                skip_spaces,
                alt((
                    map(char('*'), |_| TermOperator::Multiply),
                    map(char('/'), |_| TermOperator::Divide),
                )),
            ),
            parse_factor,
        ))),
    ))(input)
}

fn parse_expr(input: &str) -> IResult<&str, ParsedExpr> {
    tuple((
        parse_term,
        many0(tuple((
            preceded(
                skip_spaces,
                alt((
                    map(char('+'), |_| ExprOperator::Add),
                    map(char('-'), |_| ExprOperator::Subtract),
                )),
            ),
            parse_term,
        ))),
    ))(input)
}

fn evaluate_factor(variables: &SymbolTable, factor: &AnalyzedFactor) -> f64 {
    match factor {
        AnalyzedFactor::Literal(value) => *value,
        AnalyzedFactor::Identifier(handle) => variables.get_value(*handle),
        AnalyzedFactor::SubExpression(expr) => evaluate_expr(variables, expr),
    }
}

fn evaluate_term(variables: &SymbolTable, term: &AnalyzedTerm) -> f64 {
    let mut result = evaluate_factor(variables, &term.0);
    for factor in &term.1 {
        match factor.0 {
            TermOperator::Multiply => result *= evaluate_factor(variables, &factor.1),
            TermOperator::Divide => result /= evaluate_factor(variables, &factor.1),
        }
    }
    result
}

fn evaluate_expr(variables: &SymbolTable, expr: &AnalyzedExpr) -> f64 {
    let mut result = evaluate_term(variables, &expr.0);
    for term in &expr.1 {
        match term.0 {
            ExprOperator::Add => result += evaluate_term(variables, &term.1),
            ExprOperator::Subtract => result -= evaluate_term(variables, &term.1),
        }
    }
    result
}

fn execute_statement(variables: &mut SymbolTable, statement: &AnalyzedStatement) {
    match statement {
        AnalyzedStatement::Assignment(handle, expr) => {
            //println!("execute Assignment: {}.", handle);
            variables.set_value(*handle, evaluate_expr(variables, expr));
        }
        AnalyzedStatement::Declaration(handle) => {
            //println!("execute Declaration: {}.", handle);
            variables.set_value(*handle, 0.);
        }
        AnalyzedStatement::InputOperation(handle) => {
            dbg!();

            let mut text = String::new();
            use std::io::Write;
            eprint!("? ");
            std::io::stdout().flush().unwrap();
            std::io::stdin()
                .read_line(&mut text)
                .expect("Cannot read line.");
            let value = text.trim().parse::<f64>().unwrap_or(0.);
            dbg!();
            //println!("execute InputOperation: {}.", handle);
            variables.set_value(*handle, value);
        }
        AnalyzedStatement::OutputOperation(expr) => {
            println!("{}", evaluate_expr(variables, expr));
        }
    }
}

fn execute_program(mut variables: &mut SymbolTable, program: &AnalyzedProgram) {
    for statement in program {
        execute_statement(&mut variables, statement);
    }
}

fn translate_to_rust_factor(variables: &SymbolTable, analyzed_factor: &AnalyzedFactor) -> String {
    match analyzed_factor {
        AnalyzedFactor::Literal(value) => value.to_string() + "f64",
        AnalyzedFactor::Identifier(handle) => variables.get_name(*handle),
        AnalyzedFactor::SubExpression(expr) => {
            "(".to_string() + &translate_to_rust_expr(variables, expr) + ")"
        }
    }
}

fn translate_to_rust_term(variables: &SymbolTable, analyzed_term: &AnalyzedTerm) -> String {
    let mut result = translate_to_rust_factor(variables, &analyzed_term.0);
    for factor in &analyzed_term.1 {
        match factor.0 {
            TermOperator::Multiply => {
                result += " * ";
                result += &translate_to_rust_factor(variables, &factor.1);
            }
            TermOperator::Divide => {
                result += " / ";
                result += &translate_to_rust_factor(variables, &factor.1);
            }
        }
    }
    result
}

fn translate_to_rust_expr(variables: &SymbolTable, analyzed_expr: &AnalyzedExpr) -> String {
    let mut result = translate_to_rust_term(variables, &analyzed_expr.0);
    for term in &analyzed_expr.1 {
        match term.0 {
            ExprOperator::Add => {
                result += " + ";
                result += &translate_to_rust_term(variables, &term.1);
            }
            ExprOperator::Subtract => {
                result += " - ";
                result += &translate_to_rust_term(variables, &term.1);
            }
        }
    }
    result
}

fn translate_to_rust_statement(
    variables: &SymbolTable,
    analyzed_statement: &AnalyzedStatement,
) -> String {
    match analyzed_statement {
        AnalyzedStatement::Assignment(handle, expr) => format!(
            "{} = {}",
            variables.get_name(*handle),
            translate_to_rust_expr(&variables, expr)
        ),
        AnalyzedStatement::Declaration(handle) => {
            format!("let {}: f64", variables.get_name(*handle))
        }
        AnalyzedStatement::InputOperation(handle) => {
            format!("{} = input()", variables.get_name(*handle))
        }
        AnalyzedStatement::OutputOperation(expr) => format!(
            "println!(\"{}\", {})",
            "{}",
            translate_to_rust_expr(&variables, expr)
        ),
    }
}

fn translate_to_rust_program(
    variables: &SymbolTable,
    analyzed_program: &AnalyzedProgram,
) -> String {
    let mut rust_program = String::new();
    rust_program += "use std::io::Write;\n";
    rust_program += "\n";
    rust_program += "fn input() -> f64 {\n";
    rust_program += "    let mut text = String::new();\n";
    rust_program += "    eprint!(\"? \");\n";
    rust_program += "    std::io::stderr().flush().unwrap();\n";
    rust_program += "    std::io::stdin()\n";
    rust_program += "        .read_line(&mut text)\n";
    rust_program += "        .expect(\"Cannot read line.\");\n";
    rust_program += "    text.trim().parse::<f64>().unwrap_or(0.)\n";
    rust_program += "}\n";
    rust_program += "\n";
    rust_program += "fn main() {\n";
    for statement in analyzed_program {
        rust_program += "    ";
        rust_program += &translate_to_rust_statement(&variables, statement);
        rust_program += ";\n";
    }
    rust_program += "}\n";
    rust_program
}

type AnalysisResult = Result<(SymbolTable, AnalyzedProgram), String>;

fn analyze_program(parsed_program: &ParsedProgram) -> AnalysisResult {
    let mut symbols = SymbolTable::new();
    let mut analyzed_program = AnalyzedProgram::new();
    for statement in parsed_program {
        analyzed_program.push(analyze_statement(&mut symbols, statement)?);
    }
    Ok((symbols, analyzed_program))
}

#[derive(Debug)]
struct SymbolTable {
    entries: Vec<(String, f64)>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable {
            entries: Vec::<(String, f64)>::new(),
        }
    }
    fn insert_symbol(&mut self, identifier: &str) -> Result<usize, String> {
        if self
            .entries
            .iter()
            .find(|item| item.0 == identifier)
            .is_some()
        {
            Err(format!(
                "Error: Identifier '{}' declared several times.",
                identifier
            ))
        } else {
            self.entries.push((identifier.to_string(), 0.));
            Ok(self.entries.len() - 1)
        }
    }
    fn find_symbol(&self, identifier: &str) -> Result<usize, String> {
        if let Some(pos) = self.entries.iter().position(|item| item.0 == identifier) {
            Ok(pos)
        } else {
            Err(format!(
                "Error: Identifier '{}' used before having been declared.",
                identifier
            ))
        }
    }
    fn get_value(&self, handle: usize) -> f64 {
        //println!("get_value: {}", handle);
        self.entries[handle].1
    }
    fn set_value(&mut self, handle: usize, value: f64) {
        //println!("set_value: {} {}", handle, value);
        self.entries[handle].1 = value;
    }
    fn get_name(&self, handle: usize) -> String {
        //println!("get_name: {}", handle);
        self.entries[handle].0.clone()
    }
}

fn analyze_factor(
    variables: &mut SymbolTable,
    parsed_factor: &ParsedFactor,
) -> Result<AnalyzedFactor, String> {
    match parsed_factor {
        ParsedFactor::Literal(value) => Ok(AnalyzedFactor::Literal(*value)),
        ParsedFactor::Identifier(name) => {
            Ok(AnalyzedFactor::Identifier(variables.find_symbol(name)?))
        }
        ParsedFactor::SubExpression(expr) => Ok(AnalyzedFactor::SubExpression(
            Box::<AnalyzedExpr>::new(analyze_expr(variables, expr)?),
        )),
    }
}

fn analyze_term(
    mut variables: &mut SymbolTable,
    parsed_term: &ParsedTerm,
) -> Result<AnalyzedTerm, String> {
    let mut other_factors = Vec::<(TermOperator, AnalyzedFactor)>::new();
    for factor in &parsed_term.1 {
        other_factors.push((factor.0, analyze_factor(&mut variables, &factor.1)?));
    }
    Ok((
        analyze_factor(&mut variables, &parsed_term.0)?,
        other_factors,
    ))
}

fn analyze_expr(
    mut variables: &mut SymbolTable,
    parsed_expr: &ParsedExpr,
) -> Result<AnalyzedExpr, String> {
    let mut other_terms = Vec::<(ExprOperator, AnalyzedTerm)>::new();
    for term in &parsed_expr.1 {
        other_terms.push((term.0, analyze_term(&mut variables, &term.1)?));
    }
    Ok((analyze_term(&mut variables, &parsed_expr.0)?, other_terms))
}

fn analyze_statement(
    variables: &mut SymbolTable,
    parsed_statement: &ParsedStatement,
) -> Result<AnalyzedStatement, String> {
    match parsed_statement {
        ParsedStatement::Assignment(identifier, expr) => {
            let handle = variables.find_symbol(identifier)?;
            let analyzed_expr = analyze_expr(variables, expr)?;
            Ok(AnalyzedStatement::Assignment(handle, analyzed_expr))
        }
        ParsedStatement::Declaration(identifier) => {
            let handle = variables.insert_symbol(identifier)?;
            Ok(AnalyzedStatement::Declaration(handle))
        }
        ParsedStatement::InputOperation(identifier) => {
            let handle = variables.find_symbol(identifier)?;
            Ok(AnalyzedStatement::InputOperation(handle))
        }
        ParsedStatement::OutputOperation(expr) => {
            let analyzed_expr = analyze_expr(variables, expr)?;
            Ok(AnalyzedStatement::OutputOperation(analyzed_expr))
        }
    }
}

fn skip_spaces(input: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";
    take_while(move |ch| chars.contains(ch))(input)
}

fn main() {
    let parsed_program =
        parse_program("@data >data @computed computed := (data + 7) * 3 <computed")
            .unwrap()
            .1;
    dbg!(&parsed_program);
    let mut program_analysis_result = analyze_program(&parsed_program).unwrap();
    dbg!(&program_analysis_result);
    execute_program(&mut program_analysis_result.0, &program_analysis_result.1);
    print!(
        "{}",
        translate_to_rust_program(&program_analysis_result.0, &program_analysis_result.1)
    );
}

#[test]
fn parsing_variable_declaration() {
    let result = parse_program("@data");
    assert!(result.is_ok());
    assert!(result.unwrap().0.len() == 0);
}

#[test]
fn parsing_variable_starting_with_digits() {
    let result = parse_program("@2data");
    assert!(result.is_err());
}

#[test]
fn parsing_variable_with_internal_digits() {
    let result = parse_program("@data23");
    assert!(result.is_ok());
    assert!(result.unwrap().0.len() == 2);
}
