//
// Copyright (c) The Holo Core Contributors
//
// See LICENSE for license details.
//

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::{Receiver, Sender};

use crate::bfd;
use crate::ip::AddressFamily;
use crate::keychain::Keychain;
use crate::policy::{MatchSets, Policy};
use crate::sr::SrCfg;

// Useful type definition(s).
pub type IbusReceiver = Receiver<IbusMsg>;
pub type IbusSender = Sender<IbusMsg>;

// Ibus message for communication among the different Holo components.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum IbusMsg {
    // BFD peer registration.
    BfdSessionReg {
        sess_key: bfd::SessionKey,
        client_id: bfd::ClientId,
        client_config: Option<bfd::ClientCfg>,
    },
    // BFD peer unregistration.
    BfdSessionUnreg {
        sess_key: bfd::SessionKey,
        client_id: bfd::ClientId,
    },
    // BFD peer state update.
    BfdStateUpd {
        sess_key: bfd::SessionKey,
        state: bfd::State,
    },
    // Keychain update notification.
    KeychainUpd(Arc<Keychain>),
    // Keychain delete notification.
    KeychainDel(String),
    // Policy match sets update notification.
    PolicyMatchSetsUpd(Arc<MatchSets>),
    // Policy definition update notification.
    PolicyUpd(Arc<Policy>),
    // Policy definition delete notification.
    PolicyDel(String),
    // Segment Routing configuration update.
    SrCfgUpd(Arc<SrCfg>),
    // Segment Routing configuration event.
    SrCfgEvent(SrCfgEvent),
}

// Type of Segment Routing configuration change.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SrCfgEvent {
    LabelRangeUpdate,
    PrefixSidUpdate(AddressFamily),
}
