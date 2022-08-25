//! Fast and cheap alternative common built-in functions for PBC WASM interpreter
//!
//! Specially built for PBC WASM Interpreter, and works by calling hosted functions in the `pbc`
//! namespace. At least on the WASM target, these alternative definitions will automatically
//! replace the default implementations, allowing all calls to memcpy and memmove to take advantage
//! of these speedups.

#[link(wasm_import_module = "pbc")]
extern "C" {
    #[link_name = "memmove"]
    fn pbc_memmove(dest: *mut u8, src: *const u8, len: usize) -> *mut u8;
}

/// Alternative memcpy implementation for PBC WASM Interpreter
///
/// The linker will prefer this definition to linking it's own version.
///
/// # Safety
///
/// Not designed to be called directly; copies bytes around without any concern for Rust's memory
/// model.
#[no_mangle]
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, len: usize) -> *mut u8 {
    pbc_memmove(dest, src, len)
}

/// Alternative memmove implementation for PBC WASM Interpreter
///
/// The linker will prefer this definition to linking it's own version.
///
/// # Safety
///
/// Not designed to be called directly; copies bytes around without any concern for Rust's memory
/// model.
#[no_mangle]
pub unsafe fn memmove(dest: *mut u8, src: *const u8, len: usize) -> *mut u8 {
    pbc_memmove(dest, src, len)
}
