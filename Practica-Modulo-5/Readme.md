## Contrato inteligente: gRC20

## Inicio: Clonar el template para contratos inteligentes

**comando:**
```bash
git clone https://github.com/Vara-Lab/Smart-Contract-Template.git
```

## Directorio IO

## Librerias y dependencias necesarias
```rust
#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub type TxId = u64;
pub type ValidUntil = u64;

```


### PASO 1 Definir las acciones para el contrato gRC20.
**comando:**
```rust
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTAction {
    TransferToUsers {
        amount: u128,
        to_users: Vec<ActorId>,
    },
    Mint {
        amount: u128,
        to: ActorId,
    },
    Burn {
        amount: u128,
    },
    Transfer {
        tx_id: Option<TxId>,
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        tx_id: Option<TxId>,
        to: ActorId,
        amount: u128,
    },
    BalanceOf(ActorId),
    AddAdmin {
        admin_id: ActorId,
    },
    DeleteAdmin {
        admin_id: ActorId,
    },
}

```

### PASO 2 Definir las eventos para el contrato: .
**comando:**
```rust
#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTReply {
    Initialized,
    TransferredToUsers {
        from: ActorId,
        to_users: Vec<ActorId>,
        amount: u128,
    },
    Transferred {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approved {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    AdminAdded {
        admin_id: ActorId,
    },
    AdminRemoved {
        admin_id: ActorId,
    },
    Balance(u128),
}
```


### PASO 3 Definimos la estrcutura del Estado parcial
**comando:**
```rust
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Query {
    Name,
    Symbol,
    Decimals,
    CurrentSupply,
    TotalSupply,
    Description,
    ExternalLinks,
    BalanceOf(ActorId),
    AllowanceOfAccount {
        account: ActorId,
        approved_account: ActorId,
    },
    Admins,
    GetTxValidityTime {
        account: ActorId,
        tx_id: TxId,
    },
    GetTxIdsForAccount {
        account: ActorId,
    },
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum QueryReply {
    Name(String),
    Symbol(String),
    Decimals(u8),
    Description(String),
    ExternalLinks(ExternalLinks),
    CurrentSupply(u128),
    TotalSupply(u128),
    Balance(u128),
    AllowanceOfAccount(u128),
    Admins(Vec<ActorId>),
    TxValidityTime(ValidUntil),
    TxIdsForAccount { tx_ids: Vec<TxId> },
}

```

### PASO 4 Definimos un Struct para iniciar el programa
**comando:**
```rust
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub description: String,
    pub external_links: ExternalLinks,
    pub initial_supply: u128,
    pub total_supply: u128,
    pub admin: ActorId,
    pub initial_capacity: Option<u32>,
    pub config: Config,
}

#[derive(Debug, Decode, Encode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct ExternalLinks {
    pub image: String,
    pub website: Option<String>,
    pub telegram: Option<String>,
    pub twitter: Option<String>,
    pub discord: Option<String>,
    pub tokenomics: Option<String>,
}

#[derive(Debug, Decode, Encode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Config {
    pub tx_storage_period: u64,
    pub tx_payment: u128,
}


```

### PASO 5 Definir las acciones, estado y eventos.
**comando:**
```rust
pub struct FungibleTokenMetadata;

impl Metadata for FungibleTokenMetadata {
    type Init = InOut<InitConfig, FTReply>;
    type Handle = InOut<FTAction, Result<FTReply, FTError>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<Query, QueryReply>;
}
```


## Directorio src


### PASO 1 Definimos el estado gRC-20.
**comando:**
```rust
#[derive(Debug, Clone, Default)]
struct FungibleToken {
    /// Name of the token.
    name: String,
    /// Symbol of the token.
    symbol: String,
    /// Token's decimals.
    decimals: u8,
    /// Description of the token
    description: String,
    /// ExternalLinks
    external_links: ExternalLinks,
    /// Current supply of the token.
    current_supply: u128,
    /// Total supply of the token.
    total_supply: u128,
    /// Map to hold balances of token holders.
    balances: HashMap<ActorId, u128>,
    /// Map to hold allowance information of token holders.
    allowances: HashMap<ActorId, HashMap<ActorId, u128>>,
    /// Mapping of executed transactions to the time they are valid.
    tx_ids: HashMap<(ActorId, TxId), ValidUntil>,
    /// Mapping of accounts to their transaction IDs.
    account_to_tx_ids: HashMap<ActorId, HashSet<TxId>>,
    /// Configuration parameters for the fungible token contract.
    config: Config,
    admins: Vec<ActorId>,
}

static mut FUNGIBLE_TOKEN: Option<FungibleToken> = None;
```


### PASO 2 Creamos la función para volver mutable el estado.
**comando:**
```rust

fn scrow_state_mut() -> &'static mut Escrow {

    let state = unsafe {  ESCROW.as_mut()};

    unsafe { state.unwrap_unchecked() }


}
```

### PASO 3 Como el estado es un struct podemos hacerle implementaciones.
**comando:**
```rust
impl FungibleToken {
    fn transfer_to_users(
        &mut self,
        amount: u128,
        to_users: Vec<ActorId>,
    ) -> Result<FTReply, FTError> {
        let source = msg::source();
        assert!(self.admins.contains(&source), "Not admin");

        self.check_balance(&source, amount * to_users.len() as u128)?;

        for to in to_users.clone() {
            self.balances
                .entry(source)
                .and_modify(|balance| *balance -= amount);
            self.balances
                .entry(to)
                .and_modify(|balance| *balance += amount)
                .or_insert(amount);
        }

        Ok(FTReply::TransferredToUsers {
            from: source,
            to_users,
            amount,
        })
    }

    fn mint(&mut self, amount: u128, to: ActorId) -> Result<FTReply, FTError> {
        assert!(self.admins.contains(&msg::source()), "Not admin");

        if self.total_supply >= self.current_supply + amount {
            self.balances
                .entry(to)
                .and_modify(|balance| *balance += amount)
                .or_insert(amount);
            self.current_supply += amount;

            return Ok(FTReply::Transferred {
                from: ZERO_ID,
                to,
                amount,
            });
        } else {
            return Err(FTError::MaxSupplyReached);
        };
    }

    fn burn(&mut self, amount: u128) -> Result<FTReply, FTError> {
        let source = msg::source();
        if self.balances.get(&source).unwrap_or(&0) < &amount {
            return Err(FTError::NotEnoughBalance);
        }
        self.balances
            .entry(source)
            .and_modify(|balance| *balance -= amount);
        self.current_supply -= amount;
        self.total_supply -= amount;

        Ok(FTReply::Transferred {
            from: source,
            to: ZERO_ID,
            amount,
        })
    }

    fn add_admin(&mut self, admin_id: &ActorId) -> Result<FTReply, FTError> {
        let source = msg::source();
        if !self.admins.contains(&source) {
            return Err(FTError::NotAdmin);
        }
        if self.admins.contains(admin_id) {
            return Err(FTError::AdminAlreadyExists);
        }
        self.admins.push(*admin_id);
        Ok(FTReply::AdminAdded {
            admin_id: *admin_id,
        })
    }

    fn delete_admin(&mut self, admin_id: &ActorId) -> Result<FTReply, FTError> {
        let source = msg::source();
        if !self.admins.contains(&source) {
            return Err(FTError::NotAdmin);
        }

        if admin_id == &source {
            return Err(FTError::CantDeleteYourself);
        }

        self.admins.retain(|acc| acc != admin_id);
        Ok(FTReply::AdminRemoved {
            admin_id: *admin_id,
        })
    }
    fn transfer(
        &mut self,
        tx_id: Option<TxId>,
        from: &ActorId,
        to: &ActorId,
        amount: u128,
    ) -> Result<FTReply, FTError> {
        let msg_source = msg::source();
        let block_timestamp = exec::block_timestamp();
        if let Some(tx_id) = tx_id {
            self.clear_outdated_tx_ids(&msg_source, block_timestamp);
            self.check_tx_id(tx_id, &msg_source)?;
        }

        if *from == ActorId::zero() || *to == ActorId::zero() {
            return Err(FTError::ZeroAddress);
        };

        self.check_balance(from, amount)?;

        self.can_transfer(&msg_source, from, amount)?;

        self.balances
            .entry(*from)
            .and_modify(|balance| *balance -= amount);
        self.balances
            .entry(*to)
            .and_modify(|balance| *balance += amount)
            .or_insert(amount);

        self.set_tx_id_status(
            tx_id,
            &msg_source,
            block_timestamp + self.config.tx_storage_period,
        );

        Ok(FTReply::Transferred {
            from: *from,
            to: *to,
            amount,
        })
    }

    /// Executed on receiving `fungible-token-messages::ApproveInput`.
    fn approve(
        &mut self,
        tx_id: Option<TxId>,
        to: &ActorId,
        amount: u128,
    ) -> Result<FTReply, FTError> {
        if *to == ActorId::zero() {
            return Err(FTError::ZeroAddress);
        }
        let msg_source = msg::source();
        let block_timestamp = exec::block_timestamp();
        if let Some(tx_id) = tx_id {
            self.clear_outdated_tx_ids(&msg_source, block_timestamp);
            self.check_tx_id(tx_id, &msg_source)?;
        }
        self.allowances
            .entry(msg_source)
            .or_default()
            .insert(*to, amount);
        self.set_tx_id_status(
            tx_id,
            &msg_source,
            block_timestamp + self.config.tx_storage_period,
        );
        Ok(FTReply::Approved {
            from: msg_source,
            to: *to,
            amount,
        })
    }

    fn check_balance(&self, account: &ActorId, amount: u128) -> Result<(), FTError> {
        if *self.balances.get(account).unwrap_or(&0) < amount {
            return Err(FTError::NotEnoughBalance);
        }
        Ok(())
    }

    fn can_transfer(
        &mut self,
        source: &ActorId,
        from: &ActorId,
        amount: u128,
    ) -> Result<(), FTError> {
        if from != source {
            if let Some(allowed_amount) = self.allowances.get(from).and_then(|m| m.get(source)) {
                if allowed_amount >= &amount {
                    self.allowances.entry(*from).and_modify(|m| {
                        m.entry(*source).and_modify(|a| *a -= amount);
                    });
                } else {
                    return Err(FTError::NotAllowedToTransfer);
                }
            } else {
                return Err(FTError::NotAllowedToTransfer);
            }
        }
        Ok(())
    }

    fn set_tx_id_status(
        &mut self,
        tx_id: Option<TxId>,
        account: &ActorId,
        valid_until: ValidUntil,
    ) {
        if let Some(tx_id) = tx_id {
            self.tx_ids.insert((*account, tx_id), valid_until);
        }
    }

    fn check_tx_id(&self, tx_id: TxId, account: &ActorId) -> Result<(), FTError> {
        if self.tx_ids.get(&(*account, tx_id)).is_some() {
            return Err(FTError::TxAlreadyExists);
        }

        Ok(())
    }

    fn clear_outdated_tx_ids(&mut self, account: &ActorId, block_timestamp: u64) {
        if let Entry::Occupied(mut tx_ids) = self.account_to_tx_ids.entry(*account) {
            let tx_ids_cloned = tx_ids.get().clone();
            for tx_id in tx_ids_cloned {
                let valid_until = self.tx_ids.get(&(*account, tx_id)).expect("Cant be None");
                if block_timestamp > *valid_until {
                    self.tx_ids.remove(&(*account, tx_id));
                    tx_ids.get_mut().remove(&tx_id);
                }
            }
            if tx_ids.get().is_empty() {
                tx_ids.remove_entry();
            }
        }
    }
}

```

### PASO 4 Definimos la funcion Init()
**comando:**
```rust
 let init_config: InitConfig = msg::load().expect("Unable to decode InitConfig");

    if init_config.initial_supply > init_config.total_supply {
        msg::reply(FTError::SupplyError, 0).expect("Error in sending a reply");
    }

    if init_config.description.chars().count() > 500 {
        msg::reply(FTError::DescriptionError, 0).expect("Error in sending a reply");
    }

    if init_config.decimals > 100 {
        msg::reply(FTError::DecimalsError, 0).expect("Error in sending a reply");
    }

    let mut balances = HashMap::new();
    balances.insert(init_config.admin, init_config.initial_supply);

    let ft = FungibleToken {
        name: init_config.name,
        symbol: init_config.symbol,
        decimals: init_config.decimals,
        description: init_config.description,
        external_links: init_config.external_links,
        current_supply: init_config.initial_supply,
        total_supply: init_config.total_supply,
        balances,
        admins: vec![init_config.admin],
        config: init_config.config,
        ..Default::default()
    };
    unsafe { FUNGIBLE_TOKEN = Some(ft) };

    msg::reply(FTReply::Initialized, 0).expect("Error in sending a reply");
```


### PASO 5 Definimos la funcion Handle()
**comando:**
```rust
#[no_mangle]
extern "C" fn handle() {
    let action: FTAction = msg::load().expect("Could not load Action");
    let ft: &mut FungibleToken = unsafe {
        FUNGIBLE_TOKEN
            .as_mut()
            .expect("The contract is not initialized")
    };
    let reply = match action {
        FTAction::TransferToUsers { amount, to_users } => ft.transfer_to_users(amount, to_users),
        FTAction::Mint { amount, to } => ft.mint(amount, to),
        FTAction::Burn { amount } => ft.burn(amount),
        FTAction::AddAdmin { admin_id } => ft.add_admin(&admin_id),
        FTAction::DeleteAdmin { admin_id } => ft.delete_admin(&admin_id),
        FTAction::Transfer {
            tx_id,
            from,
            to,
            amount,
        } => ft.transfer(tx_id, &from, &to, amount),
        FTAction::Approve { tx_id, to, amount } => ft.approve(tx_id, &to, amount),
        FTAction::BalanceOf(account) => {
            let balance = ft.balances.get(&account).unwrap_or(&0);
            Ok(FTReply::Balance(*balance))
        }
    };
    msg::reply(reply, 0).expect("Error in sending a reply");
}
```

### PASO 5 Definimos la funcion State()
**comando:**
```rust
#[no_mangle]
extern "C" fn state() {
    let token = unsafe {
        FUNGIBLE_TOKEN
            .as_ref()
            .expect("Unexpected: Error in getting contract state")
    };
    let query: Query = msg::load().expect("Unable to decode the query");
    let reply = match query {
        Query::Name => QueryReply::Name(token.name.clone()),
        Query::Symbol => QueryReply::Symbol(token.symbol.clone()),
        Query::Decimals => QueryReply::Decimals(token.decimals.clone()),
        Query::Description => QueryReply::Description(token.description.clone()),
        Query::ExternalLinks => QueryReply::ExternalLinks(token.external_links.clone()),
        Query::CurrentSupply => QueryReply::CurrentSupply(token.current_supply.clone()),
        Query::TotalSupply => QueryReply::TotalSupply(token.total_supply.clone()),
        Query::BalanceOf(account) => {
            let balance = if let Some(balance) = token.balances.get(&account) {
                *balance
            } else {
                0
            };
            QueryReply::Balance(balance)
        }
        Query::AllowanceOfAccount {
            account,
            approved_account,
        } => {
            let allowance = if let Some(allowance) = token
                .allowances
                .get(&account)
                .and_then(|m| m.get(&approved_account))
            {
                *allowance
            } else {
                0
            };
            QueryReply::AllowanceOfAccount(allowance)
        }
        Query::Admins => QueryReply::Admins(token.admins.clone()),
        Query::GetTxValidityTime { account, tx_id } => {
            let valid_until = token.tx_ids.get(&(account, tx_id)).unwrap_or(&0);
            QueryReply::TxValidityTime(*valid_until)
        }
        Query::GetTxIdsForAccount { account } => {
            let tx_ids: Vec<TxId> =
                if let Some(tx_ids) = token.account_to_tx_ids.get(&account).cloned() {
                    tx_ids.into_iter().collect()
                } else {
                    Vec::new()
                };
            QueryReply::TxIdsForAccount { tx_ids }
        }
    };
    msg::reply(reply, 0).expect("Error on sharinf state");
}
```
# Deploy the Contract on the IDEA Platform and Interact with Your Contract

## Step 1: Open Contract on Gitpod

<p align="center">
  <a href="https://gitpod.io/#https://github.com/Vara-Lab/GRC20-Standard-Template.git" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>

## Step 2: Compile and Deploy the Smart Contract

### Compile the smart contract by running the following command:

```bash
cargo build --release
```

Once the compilation is complete, locate the `*.opt.wasm` file in the `target/wasm32-unknown-unknown/release` directory.

## Step 3: Interact with Your Contract on Vara Network

1. Access [Gear IDE](https://idea.gear-tech.io/programs?node=wss%3A%2F%2Frpc.vara.network) using your web browser.
2. Connect your Substrate wallet to Gear IDE.
3. Upload the `*.opt.wasm` and `metadata.txt` files by clicking the "Upload Program" button.
