use alloy::{
    dyn_abi::SolType,
    primitives::{keccak256, Address, U256},
    sol,
};

pub(crate) const BOB_CHAIN_ID: u64 = 60808;
pub(crate) const BOB_SEPOLIA_TESTNET_CHAIN_ID: u64 = 808813;

pub struct OffRampOpReturnData {
    pub order_id: U256,
    pub registry_address: Address,
}

impl OffRampOpReturnData {
    pub fn hash(&self) -> [u8; 32] {
        sol! {
            struct OffRampOpReturnDataSolType {
                uint256 order_id;
                address registry_address;
            }
        }

        // Convert to Alloy-compatible struct
        let alloy_data = OffRampOpReturnDataSolType {
            order_id: self.order_id,
            registry_address: self.registry_address,
        };

        // ABI encode the data in the same order as Solidity
        let encoded = <OffRampOpReturnDataSolType as SolType>::abi_encode_params(&alloy_data);

        // Compute Keccak256 hash
        keccak256(encoded).0
    }
}
