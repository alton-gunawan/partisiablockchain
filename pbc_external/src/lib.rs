//! External methods that are callable via import from within WASM.
#[cfg_attr(target_arch = "wasm32", link(wasm_import_module = "ext"))]
extern "C" {
    /// Log a message to stdout in development environments.
    pub fn log_external(message_ptr: i64, message_len: i32);
}

#[cfg(not(target_arch = "wasm32"))]
mod dummy {
    #[no_mangle]
    pub extern "C" fn log_external(_message_ptr: i64, _message_len: i32) {}
}
