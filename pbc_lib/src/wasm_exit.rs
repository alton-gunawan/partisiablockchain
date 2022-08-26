//! Exit-function for providing information in case of termination of a contract.
//!
#[link(wasm_import_module = "pbc")]
extern "C" {
    #[link_name = "exit"]
    fn pbc_exit(message_ptr: *const u8, message_len: usize);
}

/// Method to call during termination in case of a panic.
/// Sends the panic message to the
#[no_mangle]
pub fn wasm_exit(output_message: &str) {
    unsafe { pbc_exit(output_message.as_ptr(), output_message.len()) }
}

pub fn override_panic() {
    std::panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info.payload();
        let message: &str = if let Some(s) = payload.downcast_ref::<&str>() {
            s
        } else if let Some(s) = payload.downcast_ref::<String>() {
            &s
        } else {
            &""
        };
        let loc: String = if let Some(location) = panic_info.location() {
            format!("{}:{}", location.file(), location.line(),)
        } else {
            format!("unknown location")
        };
        let entire_message = format!("{}: {}", loc, message);
        wasm_exit(&entire_message);
    }));
}
