use std::fmt::Write;
use std::ops::Deref;

use parser::{
	AstExpression,
	AstFunction,
	AstProgram,
	AstStatement,
	UnaryOperator,
};

fn generate_expression(expression: &AstExpression, output: &mut String) {
	match *expression {
		AstExpression::Constant { value } => {
			write!(output, "movl ${}, %eax\n", value).unwrap();
		},
		AstExpression::UnaryOperator { ref operator } => {
			match *operator.deref() {
				UnaryOperator::Negation { ref expression } => {
					generate_expression(expression, output);
					write!(output, "neg %eax\n").unwrap();
				},
				_ => {},
			}
		},
	}
}

fn generate_statement(statement: &AstStatement, output: &mut String) {
	match *statement {
		AstStatement::Return { ref expression } => {
			generate_expression(expression, output);
			write!(output, "ret\n").unwrap();
		},
	}
}

fn generate_function(function: &AstFunction, output: &mut String) {
	write!(output, ".globl {}\n{}:\n", function.name, function.name).unwrap();
	generate_statement(&function.statement, output);
}

pub fn generate_program(program: &AstProgram) -> String {
	let mut result = String::new();

	generate_function(&program.function, &mut result);

	result
}
