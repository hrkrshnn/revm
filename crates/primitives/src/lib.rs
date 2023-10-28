#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unused_crate_dependencies)]

extern crate alloc;

pub mod bytecode;
pub mod constants;
pub mod db;
pub mod env;
#[cfg(feature = "c-kzg")]
pub mod kzg;
pub mod log;
pub mod precompile;
pub mod result;
pub mod specification;
pub mod state;
pub mod utilities;

pub use alloy_primitives::{
    self, address, b256, bytes, fixed_bytes, hex, hex_literal, ruint, uint, Address, Bytes,
    FixedBytes, B256, U256,
};
pub use bitvec;
pub use bytecode::*;
pub use constants::*;
pub use env::*;
pub use hashbrown::{hash_map, hash_set, HashMap, HashSet};
#[cfg(feature = "c-kzg")]
pub use kzg::{EnvKzgSettings, KzgSettings};
pub use log::Log;
pub use precompile::*;
pub use result::*;
pub use specification::*;
pub use state::*;
pub use utilities::*;

/// Call schemes.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CallScheme {
    /// `CALL`
    Call,
    /// `CALLCODE`
    CallCode,
    /// `DELEGATECALL`
    DelegateCall,
    /// `STATICCALL`
    StaticCall,
}

/// CallContext of the runtime.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CallContext {
    /// Execution address.
    pub address: Address,
    /// Caller of the EVM.
    pub caller: Address,
    /// The address the contract code was loaded from, if any.
    pub code_address: Address,
    /// Apparent value of the EVM.
    pub apparent_value: U256,
    /// The scheme used for the call.
    pub scheme: CallScheme,
}

impl Default for CallContext {
    fn default() -> Self {
        CallContext {
            address: Address::default(),
            caller: Address::default(),
            code_address: Address::default(),
            apparent_value: U256::default(),
            scheme: CallScheme::Call,
        }
    }
}

/// Config.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConfigKind {
    MultisigAddress = 1,
    RequiredGas = 2,
    SetBalance= 3,
    DumpState = 4,
    Unknown,
}

impl ConfigKind {
    pub fn from_u8(value: u8) -> ConfigKind {
        match value {
            1 => ConfigKind::MultisigAddress,
            2 => ConfigKind::RequiredGas,
            3 => ConfigKind::SetBalance,
            4 => ConfigKind::DumpState,
            _ => ConfigKind::Unknown,
        }
    }
}

/// Config.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AdminCallKind {
    EmergencyStop = 1,
    ReloadRuntimeConfig = 2,
    Mint = 3,
    Burn = 4,
    Unknown,
}

impl AdminCallKind {
    pub fn from_u8(value: u8) -> AdminCallKind{
        match value {
            1 => AdminCallKind::EmergencyStop,
            2 => AdminCallKind::ReloadRuntimeConfig,
            3 => AdminCallKind::Mint,
            4 => AdminCallKind::Burn,
            _ => AdminCallKind::Unknown,
        }
    }
}
