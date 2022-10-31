# dprint-plugin-stylua

Format Lua code through [dprint](https://dprint.dev/) using
[StyLua](https://github.com/JohnnyMorganz/StyLua)

## Install

Add the plugin to your config file by running
`dprint config add RubixDev/stylua`.

Don't forget to add `lua` to your `includes` pattern.

## Configuration

| Name                    | Type                    | Default                  | Possible values                                                      |
| ----------------------- | ----------------------- | ------------------------ | -------------------------------------------------------------------- |
| lineWidth               | u32                     | global config or `120`   | `0` — `4294967295`                                                   |
| useTabs                 | bool                    | global config or `2`     | `true`, `false`                                                      |
| indentWidth             | u8                      | global config or `false` | `0` — `255`                                                          |
| newLineKind             | NewLineKind             | global config or `lf`    | `auto`, `lf`, `crlf`, `system`                                       |
| verify                  | bool                    | `false`                  | `true`, `false`                                                      |
| quoteStyle              | QuoteStyle              | `AutoPreferDouble`       | `AutoPreferDouble`, `AutoPreferSingle`, `ForceDouble`, `ForceSingle` |
| callParanetheses        | CallParenType           | `Always`                 | `Always`, `NoSingleString`, `NoSingleTable`, `None`                  |
| collapseSimpleStatement | CollapseSimpleStatement | `Never`                  | `Never`, `FunctionOnly`, `ConditionalOnly`, `Always`                 |

Also have a look at the
[StyLua configuration](https://github.com/JohnnyMorganz/StyLua#options).
