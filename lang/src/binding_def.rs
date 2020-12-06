use crate::expr::Expr;
use crate::utils;
use crate::env::Env;


#[derive(Debug, PartialEq)]
pub struct BindingDef {
	pub name: String,
	pub val: Expr,
}

impl BindingDef {
	pub fn new(s: &str) -> Result<(&str, Self), String> {
		let s = utils::tag("var", s)?;
		let (s, _) = utils::extract_whitespace1(s)?;

		let (s, name) = utils::extract_ident(s)?;
		let (s, _) = utils::extract_whitespace(s);

		let s = utils::tag("=", s)?;
		let (s, _) = utils::extract_whitespace(s);

		let (s, val) = Expr::new(s)?;

		Ok((
			s,
			Self {
				name: name.to_string(),
				val,
			}
		))
	}

	pub(crate) fn eval(&self, env: &mut Env) {
		env.store_binding(self.name.clone(), self.val.eval());
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::expr::{Number, Op};

	#[test]
	fn cannot_parse_binding_def_without_space_after_var() {
		assert_eq!(
			BindingDef::new("varaaa=1+2"),
			Err("expected a space".to_string()),
		);
	}
}