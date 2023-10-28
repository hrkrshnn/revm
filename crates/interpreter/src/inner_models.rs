pub use crate::primitives::CreateScheme;
use crate::primitives::{Address, Bytes, U256, CallContext};

/// Inputs for a call.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CallInputs {
    /// The target of the call.
    pub contract: Address,
    /// The transfer, if any, in this call.
    pub transfer: Transfer,
    /// The call data of the call.
    pub input: Bytes,
    /// The gas limit of the call.
    pub gas_limit: u64,
    /// The context of the call.
    pub context: CallContext,
    /// Is static call
    pub is_static: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateInputs {
    pub caller: Address,
    pub scheme: CreateScheme,
    pub value: U256,
    pub init_code: Bytes,
    pub gas_limit: u64,
}

/// Transfer from source to target, with given value.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transfer {
    /// Source address.
    pub source: Address,
    /// Target address.
    pub target: Address,
    /// Transfer value.
    pub value: U256,
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelfDestructResult {
    pub had_value: bool,
    pub target_exists: bool,
    pub is_cold: bool,
    pub previously_destroyed: bool,
}
