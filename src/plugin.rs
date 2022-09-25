use std::path::Path;

use dprint_core::{
    configuration::{
        self, ConfigKeyMap, GlobalConfiguration, ResolveConfigurationResult,
        RECOMMENDED_GLOBAL_CONFIGURATION,
    },
    plugins::{FormatResult, PluginInfo, SyncPluginHandler},
};
use stylua_lib::{LineEndings, OutputVerification};

use crate::config::Configuration;

pub struct StyluaPluginHandler;

impl SyncPluginHandler<Configuration> for StyluaPluginHandler {
    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        let mut config = config;
        let mut diagnostics = vec![];

        let default_config = stylua_lib::Config::default();

        let resolved_config = Configuration {
            line_width: configuration::get_value(
                &mut config,
                "lineWidth",
                global_config
                    .line_width
                    .unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.line_width),
                &mut diagnostics,
            ),
            use_tabs: configuration::get_value(
                &mut config,
                "useTabs",
                global_config
                    .use_tabs
                    .unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.use_tabs),
                &mut diagnostics,
            ),
            indent_width: configuration::get_value(
                &mut config,
                "indentWidth",
                global_config
                    .indent_width
                    .unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.indent_width),
                &mut diagnostics,
            ),
            new_line_kind: configuration::get_value(
                &mut config,
                "newLineKind",
                global_config
                    .new_line_kind
                    .unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.new_line_kind),
                &mut diagnostics,
            ),
            verify: configuration::get_value(&mut config, "verify", false, &mut diagnostics),
            quote_style: configuration::get_value(
                &mut config,
                "quoteStyle",
                default_config.quote_style(),
                &mut diagnostics,
            ),
            call_parentheses: configuration::get_value(
                &mut config,
                "callParentheses",
                default_config.call_parentheses(),
                &mut diagnostics,
            ),
            collapse_simple_statement: configuration::get_value(
                &mut config,
                "collapseSimpleStatement",
                default_config.collapse_simple_statement(),
                &mut diagnostics,
            ),
        };

        diagnostics.extend(configuration::get_unknown_property_diagnostics(config));

        ResolveConfigurationResult {
            diagnostics,
            config: resolved_config,
        }
    }

    fn plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_key: "stylua".to_string(),
            file_extensions: vec!["lua".to_string()],
            file_names: vec![],
            help_url: "https://github.com/RubixDev/dprint-plugin-stylua#readme".to_string(),
            config_schema_url: "".to_string(),
            update_url: None,
        }
    }

    fn license_text(&mut self) -> String {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE")).into()
    }

    fn format(
        &mut self,
        _file_path: &Path,
        file_text: &str,
        config: &Configuration,
        _format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> FormatResult,
    ) -> FormatResult {
        let stylua_config = stylua_lib::Config::from(config).with_line_endings(
            match configuration::resolve_new_line_kind(file_text, config.new_line_kind) {
                "\r\n" => LineEndings::Windows,
                "\n" => LineEndings::Unix,
                // Fall back to \n in case upstream function changes
                _ => LineEndings::Unix,
            },
        );

        let result = stylua_lib::format_code(
            file_text,
            stylua_config,
            None,
            match config.verify {
                true => OutputVerification::Full,
                false => OutputVerification::None,
            },
        )?;
        if result == file_text {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
dprint_core::generate_plugin_code!(StyluaPluginHandler, StyluaPluginHandler);
