//! Provides access to built-ins in the PBC WASM Interpreter
//!
//! Mainly for use on Partisia Blockchain.
//!
//! # Usage
//!
//! Beyond declaring this library as a dependency, Cargo needs to know it should link the library.
//! The easiest way to guarentee this is to include the following line somewhere in your toplevel
//! module:
//!
//! ```
//! extern crate pbc_lib as _;
//! ```
//!
//! After compilation, verify that the library is correctly linked by inspecting the generated
//! WASM. The `memcpy` function if present should be a tiny wrapper. [Twiggy](https://rustwasm.github.io/twiggy/index.html) is a useful
//! utility for verifying this. Calling in bash:
//!
//! ```sh
//! twiggy top $WASM_PATH | grep "memcpy"
//! ```
//!
//! Twiggy should report either nothing, indicating that memcpy is unneeded for your contract, or
//! it should output something like:
//!
//! ```txt
//!             16 ┊     0.00% ┊ memcpy
//!              9 ┊     0.00% ┊ export "memcpy"
//! ```
//!
//! Disregard the percentages, those will vary from contract to contract; the important factor is
//! that `memcpy` is small. A large `memcpy` (like shown in the following snippet) indicates that
//! `pbc_lib` wasn't correctly linked.
//!
//! ```txt
//!            326 ┊     0.02% ┊ memcpy
//! ```

#[cfg(all(not(feature = "abi"), any(target_arch = "wasm32", doc)))]
#[path = "wasm_mem.rs"]
pub mod mem;

#[cfg(all(not(feature = "abi"), any(target_arch = "wasm32", doc)))]
#[path = "wasm_exit.rs"]
pub mod exit;
