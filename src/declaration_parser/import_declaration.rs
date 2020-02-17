/**********************************************************
 * --- Import Declaration ---
 *
 * Represents and parses an import statement.
 **********************************************************/

use crate::{
	declare_parse_required_whitespace,
	declare_parse_ascii,
	declare_parse_until_char
};

use crate::expression::variable_type::VariableType;
use crate::expression::variable_type::{ Type, VarStyle };

use crate::declaration_parser::declaration::{ Declaration, DeclarationResult };
use crate::declaration_parser::parser::Parser;

type ImportDeclarationResult = DeclarationResult<ImportDeclaration>;

pub struct ImportDeclaration {
	pub path: String,
	pub line: usize
}

impl Declaration<ImportDeclaration> for ImportDeclaration {
	fn out_of_space_error_msg() -> &'static str {
		return "unexpected end of import";
	}
}

impl ImportDeclaration {
	pub fn new(parser: &mut Parser) -> ImportDeclarationResult {
		let initial_line = parser.line;

		// Parse Var Style
		let mut import_keyword = "".to_string();
		declare_parse_ascii!(import_keyword, parser);
		if import_keyword != "import" {
			return ImportDeclarationResult::Err("Unexpected Keyword", "\"import\" keyword expected", parser.index - import_keyword.len(), parser.index);
		}

		declare_parse_required_whitespace!(parser);

		let content_start = parser.index;
		declare_parse_until_char!(';', parser);

		let mut import_path = parser.content[content_start..parser.index].to_string();

		return ImportDeclarationResult::Ok(ImportDeclaration {
			path: import_path,
			line: initial_line
		});
	}

	pub fn is_import_declaration(content: &str, index: usize) -> bool {
		let declare = &content[index..];
		return declare.starts_with("import ");
	}
}