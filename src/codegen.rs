use std::fmt::Write;

use parser::{AstProgram, AstFunction, AstExpression, AstStatement};

fn generate_statement(statement: &AstStatement, output: &mut String) {
	match statement {
		&AstStatement::Return { ref expression } => {
			match expression {
				&AstExpression::Constant { value } => {
					write!(output, "movl ${}, %eax\nret", value).unwrap();
				},
				&AstExpression::UnaryOperator { .. } => {},
			}
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
