use crate::parser::{TermOperator, ExprOperator};
use crate::symbol_table::SymbolTable;
use crate::analyzer::{AnalyzedFactor, AnalyzedTerm, AnalyzedExpr, AnalyzedStatement, AnalyzedProgram};

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

pub fn translate_to_rust_program(
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
