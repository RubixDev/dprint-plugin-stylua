mod config;
mod plugin;

#[link(wasm_import_module = "dprant")]
extern "C" {
    #[allow(dead_code)]
    fn test_123_lel(length: u32);
}
