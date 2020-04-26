/**********************************************************
 * --- Context ---
 *
 * Keeps track of the header files that need to be added
 * depending on the usage of certain classes and functions.
 **********************************************************/

use crate::context_management::typing_context::TypingContext;
use crate::context_management::header_context::HeaderContext;
use crate::context_management::static_extension::{ StaticExtensionContext, StaticExtension };
use crate::context_management::context_manager::ContextManager;

use crate::expression::variable_type::{ VariableType, Type, VarStyle };
use crate::expression::value_type::NumberType;

pub struct Context {
	pub typing: TypingContext,
	pub module: TypingContext,
	pub headers: HeaderContext,
	pub static_extends: StaticExtensionContext,
	pub shared_modules: Vec<String>,
	pub align_lines: bool,
	pub convert_this_to_self: bool
}

impl Context {
	pub fn new() -> Context {
		return Context {
			typing: TypingContext::new(false),
			module: TypingContext::new(true),
			headers: HeaderContext::new(),
			static_extends: StaticExtensionContext::new(),
			shared_modules: Vec::new(),
			align_lines: false,
			convert_this_to_self: false
		}
	}

	pub fn import_module(&mut self, ctx_module: String) {
		self.shared_modules.push(ctx_module);
	}

	pub fn add_header(&mut self, path: &str, is_system: bool) {
		self.headers.add_header(path, is_system);
	}

	pub fn register_type(&mut self, var_type: &VariableType) {
		self.register_type_only(&var_type.var_type);
		match &var_type.var_style {
			VarStyle::AutoPtr => self.add_header("memory", true),
			VarStyle::UniquePtr => self.add_header("memory", true),
			_ => ()
		}
	}

	pub fn register_type_only(&mut self, var_type: &Type) {
		match var_type {
			Type::Function(_) => self.add_header("functional", true),
			Type::Tuple(_) => self.add_header("tuple", true),
			Type::Number(num_type) => {
				match num_type {
					NumberType::Size | NumberType::WChar => self.add_header("stddef.h", true),
					_ => ()
				}
			},
			Type::Class(cls_type) => {
				for inc in &cls_type.required_includes {
					self.add_header(&inc.0, inc.1);
				}
			}
			_ => ()
		}
	}

	pub fn register_module_attribute(&mut self, attribute: &str) {
		if attribute == "TastyAlign" {
			self.align_lines = true;
		}
	}

	pub fn find_static_extension(&self, func_name: &str, t: &VariableType, manager: Option<&ContextManager>, recursive: bool) -> Option<StaticExtension> {
		let result = self.static_extends.find(func_name, t);
		if result.is_some() {
			return result;
		} else if result.is_none() {
			if !recursive && manager.is_some() {
				let manager_unwrap = manager.unwrap();
				for module in &self.shared_modules {
					let item = manager_unwrap.get_context_immut(module).find_static_extension(func_name, t, manager, true);
					if item.is_some() {
						return item;
					}
				}
			}
		}
		return None;
	}
}
