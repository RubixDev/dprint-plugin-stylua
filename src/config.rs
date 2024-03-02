use dprint_core::configuration::NewLineKind;
use serde::Serialize;
use stylua_lib::{
    CallParenType, CollapseSimpleStatement, IndentType, LineEndings, QuoteStyle, SortRequiresConfig,
};

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
    pub sort_requires: bool,
}

impl From<&Configuration> for stylua_lib::Config {
    fn from(conf: &Configuration) -> Self {
        stylua_lib::Config {
            column_width: conf.line_width as usize,
            line_endings: match conf.new_line_kind {
                NewLineKind::CarriageReturnLineFeed => LineEndings::Windows,
                _ => LineEndings::Unix,
            },
            indent_type: match conf.use_tabs {
                true => IndentType::Tabs,
                false => IndentType::Spaces,
            },
            indent_width: conf.indent_width as usize,
            quote_style: conf.quote_style,
            call_parentheses: conf.call_parentheses,
            collapse_simple_statement: conf.collapse_simple_statement,
            sort_requires: SortRequiresConfig {
                enabled: conf.sort_requires,
            },
            ..Default::default()
        }
    }
}
