use dprint_core::configuration::NewLineKind;
use serde::Serialize;
use stylua_lib::{CallParenType, CollapseSimpleStatement, IndentType, QuoteStyle};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub line_width: u32,
    pub use_tabs: bool,
    pub indent_width: u8,
    pub new_line_kind: NewLineKind,

    pub verify: bool,

    pub quote_style: QuoteStyle,
    pub call_parentheses: CallParenType,
    pub collapse_simple_statement: CollapseSimpleStatement,
}

impl From<&Configuration> for stylua_lib::Config {
    fn from(conf: &Configuration) -> Self {
        stylua_lib::Config::new()
            .with_column_width(conf.line_width as usize)
            .with_indent_type(match conf.use_tabs {
                true => IndentType::Tabs,
                false => IndentType::Spaces,
            })
            .with_indent_width(conf.indent_width as usize)
            .with_quote_style(conf.quote_style)
            .with_call_parentheses(conf.call_parentheses)
            .with_collapse_simple_statement(conf.collapse_simple_statement)
    }
}
