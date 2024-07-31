## Contrato inteligente: PayLess y GasLess Transactions

## Inicio: Clonar el template para contratos inteligentes

**comando:**
```bash
git clone https://github.com/Vara-Lab/Smart-Contract-Template.git
```

## Directorio IO

### Agrega las siguientes dependencias.
**comando:**
```rust
#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{In, InOut, Metadata};
```



### PASO 1 Definir las acciones.
**comando:**
```rust
#[derive(Encode, Decode, TypeInfo, Debug)]
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

```

### PASO 2 Declarar los eventos 

**comando:**
```rust

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
```


### PASO 3 Consultas de Estado

**comando:**
```rust
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


```


### PASO 4 Declarar una estructura para el inicio del programa

**comando:**
```rust
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

```



### PASO 5 Definimos ContractMetadata y el estado

**comando:**
```rust

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<ContractAction, ContractEvent>;
    type Others = (); 
    type Reply = ();
    type Signal = ();
    type State = InOut<ContractStateQuery, ContractStateReply>;
}
```

## Directorio src

### Agrega las siguientes dependencias.
**comando:**
```rust
#![no_std]
use gstd::{
    async_main, collections::HashMap, msg, prelude::*, prog::ProgramGenerator, ActorId, CodeId,
};
use io::*;


#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
```

### PASO 1 Definimos una estructura Actors para incorporar implementaciones

**comando:**
```rust
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
```

### PASO 2 Definimos las variables a usar
**comando:**
```rust
// Definimos las varibles.
static mut STATE: Option<ContractState> = None;

```



### PASO 3 Agregamos la funcion init para inicializar las variables

**comando:**
```rust

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

```

### PASO 4 Definimos esta función main de forma asincrona usando el macro #[async_main].

**comando:**
```rust
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
```

### PASO 5 Definimos las variables a usar
**comando:**
```rust
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

```


```
