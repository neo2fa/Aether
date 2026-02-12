use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, Promise, json_types::Base64VecU8};
use serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SignRequest {
    pub payload: Vec<u8>,
    pub path: String,
    pub key_version: u32,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct AetherContract {
    pub owner_id: AccountId,
}

#[near_bindgen]
impl AetherContract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self { owner_id }
    }

    /// Requests a signature from the NEAR MPC service for cross-chain execution
    pub fn request_signature(&self, payload: Vec<u8>, path: String) -> Promise {
        let mpc_contract: AccountId = "v1.signer-prod.near".parse().unwrap();
        
        // Call the NEAR Multi-Party Computation contract
        Promise::new(mpc_contract).function_call(
            "sign".to_string(),
            near_sdk::serde_json::to_vec(&SignRequest {
                payload,
                path,
                key_version: 0,
            }).unwrap(),
            0, 
            near_sdk::Gas(50_000_000_000_000), // 50 TGas
        )
    }
}
