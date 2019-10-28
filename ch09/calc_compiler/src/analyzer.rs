use crate::parser::{
    ExprOperator, ParsedExpr, ParsedFactor, ParsedProgram, ParsedStatement, ParsedTerm,
    TermOperator,
};
use crate::symbol_table::SymbolTable;

extern crate nom;

#[derive(Debug, PartialEq)]
pub enum AnalyzedFactor {
    Literal(f64),
    Identifier(usize),
    SubExpression(Box<AnalyzedExpr>),
}

pub type AnalyzedTerm = (AnalyzedFactor, Vec<(TermOperator, AnalyzedFactor)>);

pub type AnalyzedExpr = (AnalyzedTerm, Vec<(ExprOperator, AnalyzedTerm)>);

#[derive(Debug)]
pub enum AnalyzedStatement {
    Declaration(usize),
    InputOperation(usize),
    OutputOperation(AnalyzedExpr),
    Assignment(usize, AnalyzedExpr),
}

pub type AnalyzedProgram = Vec<AnalyzedStatement>;

pub fn analyze_program(
    variables: &mut SymbolTable,
    parsed_program: &ParsedProgram,
) -> Result<AnalyzedProgram, String> {
    let mut analyzed_program = AnalyzedProgram::new();
    for statement in parsed_program {
        analyzed_program.push(analyze_statement(variables, statement)?);
    }
    Ok(analyzed_program)
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
    variables: &mut SymbolTable,
    parsed_term: &ParsedTerm,
) -> Result<AnalyzedTerm, String> {
    let first_factor = analyze_factor(variables, &parsed_term.0)?;
    let mut other_factors = Vec::<(TermOperator, AnalyzedFactor)>::new();
    for factor in &parsed_term.1 {
        other_factors.push((factor.0, analyze_factor(variables, &factor.1)?));
    }
    Ok((first_factor, other_factors))
}

fn analyze_expr(
    variables: &mut SymbolTable,
    parsed_expr: &ParsedExpr,
) -> Result<AnalyzedExpr, String> {
    let first_term = analyze_term(variables, &parsed_expr.0)?;
    let mut other_terms = Vec::<(ExprOperator, AnalyzedTerm)>::new();
    for term in &parsed_expr.1 {
        other_terms.push((term.0, analyze_term(variables, &term.1)?));
    }
    Ok((first_term, other_terms))
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
