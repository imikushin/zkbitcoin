//! zkBitcoin.

pub mod committee;
pub mod constants;
pub mod frost;
pub mod json_rpc_stuff;
pub mod plonk;
pub mod snarkjs;

/// 1. Alice signs a transaction to deploy a smart contract.
pub mod alice_sign_tx;

/// 2. Bob sends a request to the zkBitcoin committee to unlock funds from a smart contract.
/// The MPC committee can verify that request.
pub mod bob_request;

/// 3. The zkBitcoin committee produce a collaborative schnorr signature to unlock the funds for Bob.
pub mod mpc_sign_tx;

//
// Helpers
//

pub fn get_network() -> bitcoin::Network {
    if std::env::var("MAINNET").is_ok() {
        bitcoin::Network::Bitcoin
    } else {
        bitcoin::Network::Testnet
    }
}
