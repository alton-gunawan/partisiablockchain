# rust-contract-sdk

Documentation is automatically updated, and is found [here](https://privacyblockchain.gitlab.io/language/rust-contract-sdk).

## A note on versioning

A blockchain is a complex piece of software, employing many components with many active versions. Here is an overview of the _types of versions_ associated with the SDK:

- **SDK crate version**: Version of the crate itself. Incremented for every
  change of the crate's code.
- **Binder ABI version**: Tracks which ABI version is used to communicate with the underlying PBC platform.
- **Client ABI version**: Tracks which ABI version is used to communicate with clients, for example PBC Dashboard.

Each of these versions can be incremented individually, and employ semantic versioning:

- Major versions represent backwards incompatible changes. A 2.1.3 binder cannot run a contract compiled with the 1.1.3 SDK.
- Minor versions represent forwards incompatible changes. A 2.1.3 binder cannot run a contract compiled with the 2.2.3 SDK.
- Patch versions represent backwards and forwards compatible changes. A 2.1.3 binder _can_ run a contract compiled with the 2.1.9 SDK.

The minor version constraint might seem counter-intuitive, but consider the case where a new feature `poke()` is added to the blockchain for binder version 2.3.0. The old 2.2.9 binder doesn't know how to poke, and if it ran a poking contract compiled for binder ABI 2.3.0, it would not know what to do, so it would crash rather than doing the wrong thing. Crashing is undesirable, so we prefer to reject all contracts for a higher minor version.

## Usage

For contract development, use the `Cargo partisia-contract` tool to create a toml with the correct dependencies and features. The tool can be installed using
```bash
  cargo install cargo-partisia-contract
```

### Cheap memcpy

The `pbc_lib` crate overwrites internal Rust functions with cheaper
alternatives that uses PBC internals. To overwrite these functions, you must
add following to your root module:

```rust
extern crate pbc_lib as _;
```

## Migration from REAL Binder 9.X.X to 10.X.X for ZK contracts

You will need to update your ZK contract code to compile for Binder 10.X.X.

The `10.0.0` version allows the contract full control over deletion, transfer
and opening of user variables, whereas the older binder versions prevented the
contract from these actions.  ZK contracts are now more flexible with respect
to user variables, regardless of the owner and the stage of the calculation,
which have made the below-mentioned items redundant.

The following changes have been made:

- [`CalculationStatus::Output`] merged into [`CalculationStatus::Waiting`].
- `zk_on_user_variables_opened` merged into [`zk_on_variables_opened`].
- [`ZkStateChange::OutputComplete`] replaced with [`ZkStateChange::DeleteVariables`].

These changes allows the contract to treat user variables equally to
contract owned variables.

You should update your ZK-contract code as follows:

- Checks for [`CalculationStatus::Output`] are no longer needed, and can be removed.
- Replace usages of `zk_on_user_variables_opened` with [`zk_on_variables_opened`].
- [`ZkStateChange::OutputComplete`] can be replaced with [`ZkStateChange::DeleteVariables`].

## Version History

| **Rust Crate** | **PUB Binder** | **REAL Binder** | **Client** | **SDK** | **Changes**                                                                                                                                                                    |
|---------------:|---------------:|----------------:|-----------:|--------:|:-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|          6.0.0 |          9.5.0 |          10.0.0 |      5.2.0 |  17.0.0 | Contracts can now delete, transfer and open any variable, regardless of owner and calculation stage. Removes `zk_on_user_variables_opened` and the output phase of ZK contracts. |
|          5.0.1 |          9.5.0 |           9.5.0 |      5.2.0 |  16.1.0 | Support for Rust 1.70. New wasm instructions - sign extension.                                                                                                             |
|          5.0.1 |          9.4.0 |           9.4.0 |      5.2.0 |  16.0.1 | Fixed bug whereby `start_computation` variants produced wrong RPC.                                                                                                         |
|          5.0.0 |          9.4.0 |           9.4.0 |      5.2.0 |  16.0.0 | `start_computation` and `start_computation_with_inputs` now requires shortnames for ZK Computations.                                                                       |
|          4.4.0 |          9.4.0 |           9.4.0 |      5.2.0 |  15.4.0 | Added `SortedVecSet` and `SortedVec` as gas-efficient implementations.                                                                                                     |
|          4.3.0 |          9.4.0 |           9.4.0 |      5.2.0 |  15.3.0 | Removed unused `log_external` function, as it caused linker errors in certain setups.                                                                                      |
|          4.2.0 |          9.4.0 |           9.4.0 |      5.2.0 |  15.2.0 | Added `with_cost_from_contract` to interaction builder for support for sending events using the contract's gas.                                                            |
|          4.1.0 |          9.0.0 |           9.3.0 |      5.2.0 |  15.1.0 | Added `secret_variable_ids()` to `pbc_zk`. Deprecated `num_secret_variables()`.                                                                                            |
|          4.0.0 |          9.0.0 |           9.3.0 |      5.2.0 |  15.0.0 | Removed the zk feature. Instead macros now take an optional argument zk. For an action or a callback to be zk the init must also be zk.                                    |
|          3.0.0 |          9.0.0 |           9.3.0 |      5.2.0 |  14.0.0 | Added new SortedVecMap with faster serialization. Removed support for BTreeMap. Derives automatically requires generics to implement the trait in derive implementation.   |
|          2.2.0 |          9.0.0 |           9.3.0 |      5.2.0 |  13.6.0 | Added support for user events in ZK contracts.                                                                                                                             |
|          2.2.0 |          9.0.0 |           9.2.0 |      5.2.0 |  13.5.0 | Added support for `on_secret_input` hook to take an arbitrary secret type as input.                                                                                        |
|          2.1.0 |          9.0.0 |           9.2.0 |      5.1.0 |  13.4.0 | Added more built-in types: U256, Hash, PublicKey, Signature, BlsPublicKey, BlsSignature. Distinguish between public and private contracts, when adding version.            |
|          2.1.0 |          9.2.0 |           9.2.0 |      5.0.0 |  13.3.0 | Added support for XOR, SUBTRACT and OUTPUT instructions.                                                                                                                   |
|          2.1.0 |          9.1.0 |           9.1.0 |      5.0.0 |  13.2.0 | Added support for adding gas to contract via an empty invocation.                                                                                                          |
|          2.1.0 |          9.0.0 |           9.0.0 |      5.0.0 |  13.1.0 | Added support for `#[derive(SecretBinary)]` for `pbc_zk`.                                                                                                                  |
|          2.0.0 |          9.0.0 |           9.0.0 |      5.0.0 |  13.0.0 | ReadWriteRPC trait split into two: ReadRPC and WriteRPC. The state macro no longer adds derive(Clone). The CreateTypeSpec derive only adds derive(ReadRPC) and not write.  |
|          1.7.0 |          9.0.0 |           9.0.0 |      5.0.0 |  12.0.0 | Disabled option to send from original sender. Added ping method to check if a contract is alive.                                                                           |
|          1.7.0 |          8.1.0 |           8.1.0 |      5.0.0 |  11.0.0 | Added support for enum with struct variants in RPC, State and ABI.                                                                                                         |
|          1.6.0 |          8.1.0 |           8.1.0 |      4.1.0 |  10.1.0 | ZK-computations can now be called with open (non-secret) inputs.                                                                                                           |
|          1.5.0 |          8.0.0 |           8.0.0 |      4.1.0 |  10.0.0 | Added support for return values from actions and callbacks. `pbc_contract_common::signature::Signature` and `std::collections::VecDeque` can now be used in RPC and State. |
|          1.4.6 |          7.0.0 |           7.0.0 |      4.1.0 |   9.1.2 | Added `pbc_zk` lib for testing contracts.                                                                                                                                  |
|          1.4.5 |          7.0.0 |           7.0.0 |      4.1.0 |   9.1.1 | Removed implicit circular reference that prevented linking with `CARGO_INCREMENTAL=0`. Non-path dependencies are now allowed.                                              |
|          1.4.4 |          7.0.0 |           7.0.0 |      4.1.0 |   9.1.0 | Added new `EventGroup::builder` API. Old `EventGroup` constructor API have been deprecated. Will be removed in a future version.                                           |
|          1.4.3 |          7.0.0 |           7.0.0 |      4.1.0 |   9.0.0 | Added `on_attestation_complete` function.                                                                                                                                  |
|          1.4.2 |          6.0.0 |           6.0.0 |      4.0.0 |   8.0.0 | Changed FFI between binder and contract result. The length of the result is now at the offset returned by the wrapped action/init functions.                               |
|          1.4.1 |          5.0.0 |           5.0.0 |      4.0.0 |   7.0.0 | Changed contract argument input ABI to be more easily extendable.                                                                                                          |
|          1.4.0 |          4.0.0 |           4.0.0 |      4.0.0 |   6.1.0 | Shortnames must now be specified as pre-encoded LEB128 hex literals.                                                                                                       |
|          1.3.3 |          4.0.0 |           4.0.0 |      4.0.0 |   6.0.0 | Changed ABI format to include function hook kind.                                                                                                                          |
|          1.3.2 |          4.0.0 |           4.0.0 |      3.0.0 |   5.0.0 | Switched FnKind ids for state and event sections.                                                                                                                          |
|          1.3.1 |          3.1.0 |           3.1.0 |      3.0.0 |   4.2.0 | Added hosted panic handler, allowing contracts to give better error messages.                                                                                              |
|          1.3.0 |          3.0.0 |           3.0.0 |      3.0.0 |   4.1.0 | Bugfixes and added ZK definitions.                                                                                                                                         |
|          1.2.1 |          3.0.0 |           3.0.0 |      3.0.0 |   4.0.0 | Changed contract result ABI to be more easily extendable.                                                                                                                  |
|          1.2.0 |          2.1.0 |           2.1.0 |      3.0.0 |   3.1.0 | Contracts can now receive callbacks, allowing them to differentiate between successful and failed RPCs.                                                                    |
|          1.1.0 |          2.0.0 |           2.0.0 |      3.0.0 |   3.0.0 | LEB128 action shortnames, to improve interoperability with Java system contracts.                                                                                          |
|          1.0.0 |          1.1.0 |           1.1.0 |      2.0.0 |   2.1.0 | Added hosted memmove, allowing constant-gas memcpy and memmove functionality.                                                                                              |
|          1.0.0 |          1.0.0 |           1.0.0 |      2.0.0 |   2.0.0 | Integers in state have been made little-endian, in order to lower serialization gas costs.                                                                                 |
|          1.0.0 |          1.0.0 |           1.0.0 |      1.0.0 |   1.0.0 | Initial                                                                                                                                                                    |
