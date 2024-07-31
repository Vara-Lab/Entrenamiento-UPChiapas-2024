use gstd::{collections::BTreeMap, msg, prelude::*, ActorId};

use ping_no_wallet_io::*;

static mut STATE: Option<ContractState> = None;

#[no_mangle]
extern "C" fn init() {
    unsafe {
        STATE = Some(ContractState {
            last_who_call: ActorId::zero(),
            signless_accounts_by_address: BTreeMap::new(),
            signless_accounts_by_no_wallet: BTreeMap::new(),
            signless_data: BTreeMap::new()
        });
    };
    msg::reply(ContractEvent::ContractStarted, 0)
        .expect("Error sending reply");
}

#[no_mangle]
extern "C" fn handle() {
    let message = msg::load()
        .expect("Error loading message");

    let state = state_mut();

    match message {
        ContractAction::Ping { user_account } => {
            let (user_address, no_wallet) = user_account;

            if user_address.is_some() {

                let address = match state.get_address(msg::source(), user_address) {
                    Ok(address) => address,
                    Err(error_message) => {
                        msg::reply(error_message, 0)
                            .expect("Error sending reply");
                        return;
                    }
                };
                
                state.last_who_call = address;

            } else if no_wallet.is_some() {
                if let Err(error_message) = state.check_no_wallet(msg::source(), no_wallet.unwrap()) {
                    msg::reply(error_message, 0)
                            .expect("Error sending reply");
                    return;
                }

                state.last_who_call = msg::source();
            } else {
                state.last_who_call = msg::source();
            }

            msg::reply(ContractEvent::GetPing, 0)
                .expect("Cant send reply");
        },
        ContractAction::Pong { user_account } => {
            let (user_address, no_wallet) = user_account;

            if user_address.is_some() {

                let address = match state.get_address(msg::source(), user_address) {
                    Ok(address) => address,
                    Err(error_message) => {
                        msg::reply(error_message, 0)
                            .expect("Error sending reply");
                        return;
                    }
                };
                
                state.last_who_call = address;

            } else if no_wallet.is_some() {
                if let Err(error_message) = state.check_no_wallet(msg::source(), no_wallet.unwrap()) {
                    msg::reply(error_message, 0)
                            .expect("Error sending reply");
                    return;
                }

                state.last_who_call = msg::source();
            } else {
                state.last_who_call = msg::source();
            }

            msg::reply(ContractEvent::GetPong, 0)
                .expect("Error sending reply");
        },
        ContractAction::BindSignlessAccountWithAddress { 
            user_address, 
            signless_data 
        } => {
            if state.signless_accounts_by_address.contains_key(&user_address) {
                msg::reply(ContractEvent::AddressAlreadyHasSignlessAccount, 0)
                    .expect("Error sending reply");
                return;
            }

            let caller = msg::source();

            if state.signless_data.contains_key(&caller) {
                msg::reply(ContractEvent::SignlessAddressAlreadyEsists, 0)
                    .expect("Error sending reply");
                return;
            }

            state.signless_accounts_by_address.insert(user_address, caller);
            state.signless_data.insert(caller, signless_data);

            msg::reply(ContractEvent::SignlessAccountSet, 0)
                .expect("Error sending reply");
        },
        ContractAction::BindSignlessAccountWithNoWallet { 
            no_wallet, 
            signless_data 
        } => {
            if state.signless_accounts_by_no_wallet.contains_key(&no_wallet) {
                msg::reply(ContractEvent::NoWalletAlreadyHasSignlessAccount, 0)
                    .expect("Error sending reply");
                return;
            }

            let caller = msg::source();

            if state.signless_data.contains_key(&caller) {
                msg::reply(ContractEvent::SignlessAddressAlreadyEsists, 0)
                    .expect("Error sending reply");
                return;
            }

            state.signless_accounts_by_no_wallet.insert(no_wallet, caller);
            state.signless_data.insert(caller, signless_data);

            msg::reply(ContractEvent::SignlessAccountSet, 0)
                .expect("Error sending reply");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let message = msg::load()
        .expect("Error loading message");

    let state = state_mut();

    match message {
        ContractStateQuery::LastWhoCallContract => {
            msg::reply(ContractStateReply::LastWhoCallContract(state.last_who_call), 0) 
                .expect("Error sending state");
        },
        ContractStateQuery::SignlessAddressByAddress(address) => {
            let address = state.signless_accounts_by_address.get(&address);

            msg::reply(ContractStateReply::SignlessAddress(address.copied()), 0)
                .expect("Error sending reply in state");
        },
        ContractStateQuery::SignlessAddressByNoWallet(no_wallet) => {
            let address = state.signless_accounts_by_no_wallet.get(&no_wallet);

            msg::reply(ContractStateReply::SignlessAddress(address.copied()), 0)
                .expect("Error sending reply in state");
        },
        ContractStateQuery::SignlessData(signless_address) => {
            let signless_data = state.signless_data.get(&signless_address).cloned();

            msg::reply(ContractStateReply::SignlessData(signless_data), 0)
                .expect("Error sending reply in state");
        }
    }
}



fn state_mut() -> &'static mut ContractState {
    let state = unsafe { STATE.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}