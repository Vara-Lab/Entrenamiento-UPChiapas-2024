#![no_std]

use gstd::{prelude::*, ActorId, collections::BTreeMap};
use gmeta::{Metadata, InOut};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<ContractAction, ContractEvent>;
    type Others = (); 
    type Reply = ();
    type Signal = ();
    type State = InOut<ContractStateQuery, ContractStateReply>;
}

pub struct ContractState {
    pub last_who_call: ActorId,
    pub signless_accounts_by_address: BTreeMap<ActorId, ActorId>,
    pub signless_accounts_by_no_wallet: BTreeMap<String, ActorId>,
    pub signless_data: BTreeMap<ActorId, SignlessAccount>
}

impl ContractState {
    pub fn check_no_wallet(&self, caller: ActorId, no_wallet: String) -> Result<(), ContractEvent> {
        let no_wallet_signless_address = self
            .signless_accounts_by_no_wallet
            .get(&no_wallet);

        let Some(signless_address) = no_wallet_signless_address else {
            return Err(ContractEvent::BadSignlessSession);
        };

        if *signless_address != caller {
            return Err(ContractEvent::BadSignlessSession);
        }
    
        Ok(())
    }

    pub fn get_address(&self, caller: ActorId, user_address: Option<ActorId>) -> Result<ActorId, ContractEvent> {
        let address = match user_address {
            Some(address) => {
                let signless_address = self
                    .signless_accounts_by_address
                    .get(&address)
                    .ok_or(ContractEvent::BadSignlessSession)?;

                if *signless_address != caller {
                    return Err(ContractEvent::BadSignlessSession);
                }

                address
            },
            None => caller
        };

        Ok(address)
    }
}



#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractStateQuery {
    LastWhoCallContract,
    SignlessAddressByAddress(ActorId),
    SignlessAddressByNoWallet(String),
    SignlessData(ActorId)
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractStateReply {
    LastWhoCallContract(ActorId),
    SignlessAddress(Option<ActorId>),
    SignlessData(Option<SignlessAccount>)
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractAction {
    Ping {
        user_account: (Option<ActorId>, Option<String>),
    },
    Pong {
        user_account: (Option<ActorId>, Option<String>),
    },
    BindSignlessAccountWithAddress {
        user_address: ActorId,
        signless_data: SignlessAccount
    },
    BindSignlessAccountWithNoWallet {
        no_wallet: String,
        signless_data: SignlessAccount
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractEvent {
    GetPing,
    GetPong,
    ContractStarted,
    SignlessAccountSet,
    BadSignlessSession,
    AddressAlreadyHasSignlessAccount,
    NoWalletAlreadyHasSignlessAccount,
    SignlessAddressAlreadyEsists
}


#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct SignlessAccount {
    address: String,
    encoded: String,
    encoding: SignlessEncodingData,
    meta: SignlessMetaData
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct SignlessEncodingData {
    content: (String, String),
    encoding_type: (String, String),
    version: String
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct SignlessMetaData {
    name: String
}
