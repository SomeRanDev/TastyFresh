/**********************************************************
 * --- Return Parser ---
 *
 * Parses a return statement.
 **********************************************************/

use crate::{
	declare_parse_whitespace,
	declare_parse_ascii
};

use crate::config_management::ConfigData;

use crate::expression::Expression;
use crate::expression::expression_parser::ExpressionEndReason;
use crate::expression::variable_type::VariableType;

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;
use crate::declaration_parser::cpp_transpiler::CPPTranspiler;

use crate::context_management::context::Context;

use std::rc::Rc;

use regex::Regex;

lazy_static! {
	pub static ref RETURN_REGEX: Regex = Regex::new(r"^\b(?:return)\b").unwrap();
}

type ReturnParserResult = DeclarationResult<ReturnParser>;

pub struct ReturnParser {
	pub expression: Rc<Expression>,
	pub line: usize
}

impl Declaration<ReturnParser> for ReturnParser {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of return statement";
	}
}

impl CPPTranspiler for ReturnParser {
	fn to_cpp(&self) -> String {
		return "".to_string();
	}
}

impl ReturnParser {
	pub fn new(parser: &mut Parser, file_name: String, config_data: &ConfigData, context: &mut Context, expected_return_type: Option<VariableType>) -> ReturnParserResult {
		let initial_line = parser.line;

		let mut return_keyword = "".to_string();
		declare_parse_ascii!(return_keyword, parser);
		if return_keyword != "return" {
			return ReturnParserResult::Err("Unexpected Keyword", "\"return\" keyword expected", parser.index - return_keyword.len(), parser.index);
		}

		declare_parse_whitespace!(parser);

		/*parse_expression(&mut self, file_name: String, config_data: &ConfigData) -> ExpressionEndReason {
		let expr_parser = ExpressionParser::new(self, Position::new(file_name, Some(self.line), self.index, None), config_data, None);
		self.line += expr_parser.position.line_offset;
		return expr_parser.end_data.reason;
	}*/

		let mut reason = ExpressionEndReason::Unknown;
		let expression = parser.parse_expression(file_name, config_data, Some(context), &mut reason, expected_return_type);

		match reason {
			ExpressionEndReason::Unknown => return ReturnParserResult::Err("Unknown Error", "unknown expression parsing error", parser.index - 1, parser.index),
			ExpressionEndReason::EndOfContent =>  return ReturnParserResult::Err("Unexpected End of Expression", "unexpected end of expression", parser.index - 1, parser.index),
			ExpressionEndReason::NoValueError => return ReturnParserResult::Err("Value Expected", "expression value expected here", parser.index - 1, parser.index),
			ExpressionEndReason::EndOfExpression => {
				let old_index = parser.index;
				declare_parse_whitespace!(parser);
				if parser.get_curr() != ';' {
					return ReturnParserResult::Err("Semicolon Needed", "there should be a ; here", old_index - 1, old_index);
				}
			},
			_ => ()
		}

		return ReturnParserResult::Ok(ReturnParser {
			expression: expression,
			line: initial_line
		});
	}

	pub fn is_declaration(parser: &Parser) -> bool {
		return Self::is_return_declaration(&parser.content, parser.index);
	}

	pub fn is_return_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return RETURN_REGEX.is_match(declare);
	}
}
