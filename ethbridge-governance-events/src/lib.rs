#![allow(dead_code)]
#![allow(unused_imports)]
use ::ethbridge_structs::*;
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthEvent))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthDisplay))]
#[cfg_attr(
    feature = "ethers-derive",
    ethevent(name = "NewContract", abi = "NewContract(string,address)")
)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct NewContractFilter {
    #[cfg_attr(feature = "ethers-derive", ethevent(indexed))]
    pub name: ::ethabi::ethereum_types::H256,
    pub addr: ::ethabi::ethereum_types::Address,
}
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthEvent))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthDisplay))]
#[cfg_attr(
    feature = "ethers-derive",
    ethevent(name = "UpgradedContract", abi = "UpgradedContract(string,address)")
)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct UpgradedContractFilter {
    #[cfg_attr(feature = "ethers-derive", ethevent(indexed))]
    pub name: ::ethabi::ethereum_types::H256,
    pub addr: ::ethabi::ethereum_types::Address,
}
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthEvent))]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthDisplay))]
#[cfg_attr(
    feature = "ethers-derive",
    ethevent(
        name = "ValidatorSetUpdate",
        abi = "ValidatorSetUpdate(uint256,bytes32,bytes32)"
    )
)]
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiCodec))]
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct ValidatorSetUpdateFilter {
    #[cfg_attr(feature = "ethers-derive", ethevent(indexed))]
    pub validator_set_nonce: ::ethabi::ethereum_types::U256,
    pub bridge_validator_set_hash: [u8; 32],
    pub governance_validator_set_hash: [u8; 32],
}
#[cfg_attr(feature = "ethers-derive", derive(::ethers_contract::EthAbiType))]
#[doc = "Container type for all of the contract's events"]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GovernanceEvents {
    NewContractFilter(NewContractFilter),
    UpgradedContractFilter(UpgradedContractFilter),
    ValidatorSetUpdateFilter(ValidatorSetUpdateFilter),
}
#[cfg(feature = "ethers-derive")]
impl ::ethers_contract::EthLogDecode for GovernanceEvents {
    fn decode_log(
        log: &::ethers::core::abi::RawLog,
    ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
        if let Ok(decoded) = NewContractFilter::decode_log(log) {
            return Ok(GovernanceEvents::NewContractFilter(decoded));
        }
        if let Ok(decoded) = UpgradedContractFilter::decode_log(log) {
            return Ok(GovernanceEvents::UpgradedContractFilter(decoded));
        }
        if let Ok(decoded) = ValidatorSetUpdateFilter::decode_log(log) {
            return Ok(GovernanceEvents::ValidatorSetUpdateFilter(decoded));
        }
        Err(::ethers::core::abi::Error::InvalidData)
    }
}
#[cfg(feature = "ethers-derive")]
impl ::core::fmt::Display for GovernanceEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::NewContractFilter(element) => ::core::fmt::Display::fmt(element, f),
            Self::UpgradedContractFilter(element) => ::core::fmt::Display::fmt(element, f),
            Self::ValidatorSetUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
        }
    }
}
impl ::core::convert::From<NewContractFilter> for GovernanceEvents {
    fn from(value: NewContractFilter) -> Self {
        Self::NewContractFilter(value)
    }
}
impl ::core::convert::From<UpgradedContractFilter> for GovernanceEvents {
    fn from(value: UpgradedContractFilter) -> Self {
        Self::UpgradedContractFilter(value)
    }
}
impl ::core::convert::From<ValidatorSetUpdateFilter> for GovernanceEvents {
    fn from(value: ValidatorSetUpdateFilter) -> Self {
        Self::ValidatorSetUpdateFilter(value)
    }
}
