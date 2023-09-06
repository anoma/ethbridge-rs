#![allow(dead_code)]
#![allow(unused_imports)]
use ::ethbridge_bridge_events::*;
use ::ethers::abi::AbiDecode;
use ::ethers::contract::{AbiError, EthEvent};
#[doc = r"[`EventCodec`] as a static reference trait object."]
pub type DynEventCodec = &'static (dyn EventCodec + Sync);
#[doc = r"Codec to deserialize Ethereum events."]
pub trait EventCodec {
    #[doc = r"ABI signature of the Ethereum event."]
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str>;
    #[doc = r"The kind of event."]
    fn kind(&self) -> EventKind;
    #[doc = r"Decode an Ethereum event."]
    fn decode(&self, log: &::ethers::abi::RawLog) -> Result<Events, AbiError>;
}
#[doc = "Event codec for [`TransferToChainFilter`]."]
pub static TRANSFER_TO_CHAIN_CODEC: DynEventCodec =
    &::std::marker::PhantomData::<TransferToChainFilter>;
impl TryFrom<Events> for TransferToChainFilter {
    type Error = ();
    fn try_from(ev: Events) -> Result<Self, ()> {
        match ev {
            Events::Bridge(BridgeEvents::TransferToChainFilter(ev)) => Ok(ev),
            _ => Err(()),
        }
    }
}
impl EventCodec for ::std::marker::PhantomData<TransferToChainFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        TransferToChainFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Bridge
    }
    fn decode(&self, log: &::ethers::abi::RawLog) -> Result<Events, AbiError> {
        let event = TransferToChainFilter::decode_log(log)?;
        Ok(Events::Bridge(BridgeEvents::TransferToChainFilter(event)))
    }
}
#[doc = "Event codec for [`TransferToErcFilter`]."]
pub static TRANSFER_TO_ERC_CODEC: DynEventCodec =
    &::std::marker::PhantomData::<TransferToErcFilter>;
impl TryFrom<Events> for TransferToErcFilter {
    type Error = ();
    fn try_from(ev: Events) -> Result<Self, ()> {
        match ev {
            Events::Bridge(BridgeEvents::TransferToErcFilter(ev)) => Ok(ev),
            _ => Err(()),
        }
    }
}
impl EventCodec for ::std::marker::PhantomData<TransferToErcFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        TransferToErcFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Bridge
    }
    fn decode(&self, log: &::ethers::abi::RawLog) -> Result<Events, AbiError> {
        let event = TransferToErcFilter::decode_log(log)?;
        Ok(Events::Bridge(BridgeEvents::TransferToErcFilter(event)))
    }
}
#[doc = "Event codec for [`ValidatorSetUpdateFilter`]."]
pub static VALIDATOR_SET_UPDATE_CODEC: DynEventCodec =
    &::std::marker::PhantomData::<ValidatorSetUpdateFilter>;
impl TryFrom<Events> for ValidatorSetUpdateFilter {
    type Error = ();
    fn try_from(ev: Events) -> Result<Self, ()> {
        match ev {
            Events::Bridge(BridgeEvents::ValidatorSetUpdateFilter(ev)) => Ok(ev),
            _ => Err(()),
        }
    }
}
impl EventCodec for ::std::marker::PhantomData<ValidatorSetUpdateFilter> {
    fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
        ValidatorSetUpdateFilter::abi_signature()
    }
    fn kind(&self) -> EventKind {
        EventKind::Bridge
    }
    fn decode(&self, log: &::ethers::abi::RawLog) -> Result<Events, AbiError> {
        let event = ValidatorSetUpdateFilter::decode_log(log)?;
        Ok(Events::Bridge(BridgeEvents::ValidatorSetUpdateFilter(
            event,
        )))
    }
}
#[doc = r"Return all Ethereum event codecs."]
pub fn event_codecs() -> [DynEventCodec; 3usize] {
    [
        TRANSFER_TO_CHAIN_CODEC,
        TRANSFER_TO_ERC_CODEC,
        VALIDATOR_SET_UPDATE_CODEC,
    ]
}
#[doc = r"The Ethereum events generated by `ethbridge-rs`."]
#[derive(Debug)]
pub enum Events {
    #[doc = r" Bridge events."]
    Bridge(BridgeEvents),
}
#[doc = r"The kinds of Ethereum events generated by `ethbridge-rs`."]
#[derive(Debug)]
pub enum EventKind {
    #[doc = r" Bridge events."]
    Bridge,
}
