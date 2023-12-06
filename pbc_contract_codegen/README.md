# Partisia Blockchain SDK Macros

Defines the ABI attribute macros:

- [`macro@state`] declares how the contract represents its state.
- [`macro@init`] declares the code run when the contract is initialized.
- [`macro@action`] declares an endpoint that the contract can be interacted with by.
- [`macro@callback`] declares a callback hook.

Additionally defines the zero-knowledge lifetime attribute macros:

- [`macro@zk_on_secret_input`] declares an endpoint that the contract can be interacted with to add secret variables.
- [`macro@zk_on_variable_inputted`] declares an automatic hook for when a variable is confirmed inputted.
- [`macro@zk_on_variable_rejected`] declares an automatic hook for when a variable is rejected.
- [`macro@zk_on_compute_complete`] declares an automatic hook for when the zero-knowledge computation is finished.
- [`macro@zk_on_variables_opened`] declares an automatic hook for when one of the contract's own secret variables is ready to be read.
- [`macro@zk_on_attestation_complete`] declares an automatic hook for when the contract have
asked nodes to attest a piece of data, and this process have completed.
- [`macro@zk_on_external_event`] declares an automatic hook for when the contract have subscribed to external events and nodes send events to the contract.


This crate can automatically produce [ABI files](https://partisiablockchain.gitlab.io/documentation/smart-contracts/smart-contract-binary-formats.html),
and serialization boilerplate for actions. Additionally, the crate will type check the
function signatures of the annotated functions, to guarantee that the contract can interact
correctly with the blockchain.


