//! Optimism EVM System calls.

mod eip4788;
pub(crate) use eip4788::pre_block_beacon_root_contract_call;

mod canyon;
pub(crate) use canyon::ensure_create2_deployer_canyon;
