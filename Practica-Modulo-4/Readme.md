## Contrato inteligente:Escrow

## Inicio: Clonar el template para contratos inteligentes

**comando:**
```bash
git clone https://github.com/Vara-Lab/Smart-Contract-Template.git
```

## Directorio IO

## Librerias y dependencias necesarias
```rust
#![no_std]
use gmeta::{In, InOut, Metadata, Out}; // Agregamos la dependencia In en gmeta
use gstd::{prelude::*, ActorId};
use primitive_types::U256; 


pub type WalletId = U256;
```


### PASO 1 Definir las acciones para el contrato: .
**comando:**
```rust
#[derive(Clone, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum EscrowAction {
  
    Create {
        buyer: ActorId,
 
        seller: ActorId,
      
        amount: u128,
    },

    Deposit(
        WalletId,
    ),
    Confirm(
 
        WalletId,
    ),

    Refund(
 
        WalletId,
    ),
    Cancel(

        WalletId,
    ),
    Continue(
        u64,
    ),
}

```

### PASO 2 Definir las eventos para el contrato: .
**comando:**
```rust
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum EscrowEvent {
    Cancelled(
        WalletId,
    ),
    Refunded(
        /// Transaction id.
        u64,
        /// An ID of a refunded wallet.
        WalletId,
    ),
    Confirmed(
        /// Transaction id.
        u64,
        /// An ID of a wallet with a confirmed deal.
        WalletId,
    ),
    Deposited(
        /// Transaction id.
        u64,
        /// An ID of a deposited wallet.
        WalletId,
    ),
    Created(
        /// An ID of a created wallet.
        WalletId,
    ),
    TransactionProcessed,
    TransactionFailed,
}
```


### PASO 3 Definimos un Struct llamado Escrow y un enum para su estado
**comando:**
```rust
/// Escrow wallet information.
#[derive(Decode, Encode, TypeInfo, Clone, Copy, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Wallet {
    /// A buyer.
    pub buyer: ActorId,
    /// A seller.
    pub seller: ActorId,
    /// A wallet state.
    pub state: WalletState,
    /// An amount of tokens that a wallet can have. **Not** a current amount on a wallet balance!
    pub amount: u128,
}


/// An escrow wallet state.
#[derive(Decode, Encode, TypeInfo, PartialEq, Eq, Clone, Copy, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum WalletState {
    AwaitingDeposit,
    AwaitingConfirmation,
    Closed,
}

#[derive(Default, Encode, Decode, Clone, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct EscrowState {
    pub ft_program_id: ActorId,
    pub wallets: Vec<(WalletId, Wallet)>,
    pub id_nonce: WalletId,
    pub transaction_id: u64,
    pub transactions: Vec<(u64, Option<EscrowAction>)>,
}

/// Acciones y eventos del contrato a controlar
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTAction {
    Mint(u128),
    Burn(u128),
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        to: ActorId,
        amount: u128,
    },
    TotalSupply,
    BalanceOf(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTEvent {
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    TotalSupply(u128),
    Balance(u128),
    Ok
}

```

### PASO 4 Definimos un Struct para iniciar el programa
**comando:**
```rust
/// Initializes an escrow program.
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitEscrow {
    /// Address of a fungible token program.
    pub ft_program_id: ActorId,
}


```

### PASO 5 Definir las acciones, estado y eventos.
**comando:**
```rust
pub struct EscrowMetadata;

impl Metadata for EscrowMetadata {
    type Init = In<InitEscrow>;
    type Handle = InOut<EscrowAction, EscrowEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<EscrowState>;
}
```


## Directorio src


### PASO 1 Definimos el estado SCROW.
**comando:**
```rust
static mut ESCROW: Option<Escrow> =  None;
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


#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Escrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
    pub state: EscrowState,
}

async fn transfer_tokens(
    transaction_id: u64,
    token_address: &ActorId,
    from: &ActorId,
    to: &ActorId,
    amount_tokens: u128,
) -> Result<(), ()> {
    let reply = msg::send_for_reply_as::<_, FTEvent>(
        *token_address,
        FTAction::Transfer {
            from: *from,
            to: *to,
            amount: amount_tokens,
            },
        0,
        0,
    )
    .expect("Error in sending a message `FTokenAction::Message`")
    .await;

    match reply {
        Ok(FTEvent::Transfer {
            from,
            to,
            amount,
        }) => Ok(()),
        _ => Err(()),
    }
}

fn get_mut_wallet(wallets: &mut HashMap<WalletId, Wallet>, wallet_id: WalletId) -> &mut Wallet {
    wallets
        .get_mut(&wallet_id)
        .unwrap_or_else(|| panic_wallet_not_exist(wallet_id))
}

fn reply(escrow_event: EscrowEvent) {
    msg::reply(escrow_event, 0).expect("Error during a replying with EscrowEvent");
}

fn check_buyer_or_seller(buyer: ActorId, seller: ActorId) {
    if msg::source() != buyer && msg::source() != seller {
        panic!("msg::source() must be a buyer or seller");
    }
}

fn check_buyer(buyer: ActorId) {
    if msg::source() != buyer {
        panic!("msg::source() must be a buyer");
    }
}

fn check_seller(seller: ActorId) {
    if msg::source() != seller {
        panic!("msg::source() must be a seller");
    }
}

fn panic_wallet_not_exist(wallet_id: WalletId) -> ! {
    panic!("Wallet with the {wallet_id} ID doesn't exist");
}

#[derive(Default, Clone)]
pub struct Escrow {
    pub ft_program_id: ActorId,
    pub wallets: HashMap<WalletId, Wallet>,
    pub id_nonce: WalletId,
    pub transaction_id: u64,
    pub transactions: HashMap<u64, Option<EscrowAction>>,
}

impl Escrow {
    pub fn create(&mut self, buyer: ActorId, seller: ActorId, amount: u128) {
        if buyer == ActorId::zero() && seller == ActorId::zero() {
            panic!("A buyer or seller can't have the zero address")
        }
        check_buyer_or_seller(buyer, seller);

        let wallet_id = self.id_nonce;
        self.id_nonce = self.id_nonce.saturating_add(WalletId::one());

        if self.wallets.contains_key(&wallet_id) {
            panic!("Wallet with the {wallet_id} already exists");
        }
        self.wallets.insert(
            wallet_id,
            Wallet {
                buyer,
                seller,
                amount,
                state: WalletState::AwaitingDeposit,
            },
        );

        reply(EscrowEvent::Created(wallet_id));
    }

    pub async fn deposit(&mut self, transaction_id: Option<u64>, wallet_id: WalletId) {
        let current_transaction_id = self.get_transaction_id(transaction_id);

        let wallet = get_mut_wallet(&mut self.wallets, wallet_id);
        check_buyer(wallet.buyer);
        assert_eq!(wallet.state, WalletState::AwaitingDeposit);

        if transfer_tokens(
            current_transaction_id,
            &self.ft_program_id,
            &wallet.buyer,
            &exec::program_id(),
            wallet.amount,
        )
        .await
        .is_err()
        {
            self.transactions.remove(&current_transaction_id);
            reply(EscrowEvent::TransactionFailed);
            return;
        }

        wallet.state = WalletState::AwaitingConfirmation;

        self.transactions.remove(&current_transaction_id);

        reply(EscrowEvent::Deposited(current_transaction_id, wallet_id));
    }

    pub async fn confirm(&mut self, transaction_id: Option<u64>, wallet_id: WalletId) {
        let current_transaction_id = self.get_transaction_id(transaction_id);

        let wallet = get_mut_wallet(&mut self.wallets, wallet_id);
        check_buyer(wallet.buyer);
        assert_eq!(wallet.state, WalletState::AwaitingConfirmation);

        if transfer_tokens(
            current_transaction_id,
            &self.ft_program_id,
            &exec::program_id(),
            &wallet.seller,
            wallet.amount,
        )
        .await
        .is_ok()
        {
            wallet.state = WalletState::Closed;

            self.transactions.remove(&current_transaction_id);

            reply(EscrowEvent::Confirmed(current_transaction_id, wallet_id));
        } else {
            reply(EscrowEvent::TransactionFailed);
        }
    }

    pub async fn refund(&mut self, transaction_id: Option<u64>, wallet_id: WalletId) {
        let current_transaction_id = self.get_transaction_id(transaction_id);

        let wallet = get_mut_wallet(&mut self.wallets, wallet_id);
        check_seller(wallet.seller);
        assert_eq!(wallet.state, WalletState::AwaitingConfirmation);

        if transfer_tokens(
            current_transaction_id,
            &self.ft_program_id,
            &exec::program_id(),
            &wallet.buyer,
            wallet.amount,
        )
        .await
        .is_ok()
        {
            wallet.state = WalletState::AwaitingDeposit;

            self.transactions.remove(&current_transaction_id);

            reply(EscrowEvent::Refunded(current_transaction_id, wallet_id));
        } else {
            reply(EscrowEvent::TransactionFailed);
        }
    }

    pub async fn cancel(&mut self, wallet_id: WalletId) {
        let wallet = get_mut_wallet(&mut self.wallets, wallet_id);
        check_buyer_or_seller(wallet.buyer, wallet.seller);
        assert_eq!(wallet.state, WalletState::AwaitingDeposit);

        wallet.state = WalletState::Closed;

        reply(EscrowEvent::Cancelled(wallet_id));
    }

    pub async fn continue_transaction(&mut self, transaction_id: u64) {
        let transactions = self.transactions.clone();
        let payload = &transactions
            .get(&transaction_id)
            .expect("Transaction does not exist");
        if let Some(action) = payload {
            match action {
                EscrowAction::Deposit(wallet_id) => {
                    self.deposit(Some(transaction_id), *wallet_id).await
                }
                EscrowAction::Confirm(wallet_id) => {
                    self.confirm(Some(transaction_id), *wallet_id).await
                }
                EscrowAction::Refund(wallet_id) => {
                    self.refund(Some(transaction_id), *wallet_id).await
                }
                _ => unreachable!(),
            }
        } else {
            msg::reply(EscrowEvent::TransactionProcessed, 0)
                .expect("Error in a reply `EscrowEvent::TransactionProcessed`");
        }
    }

    pub fn get_transaction_id(&mut self, transaction_id: Option<u64>) -> u64 {
        match transaction_id {
            Some(transaction_id) => transaction_id,
            None => {
                let transaction_id = self.transaction_id;
                self.transaction_id = self.transaction_id.wrapping_add(1);
                transaction_id
            }
        }
    }
}

```

### PASO 4 Definimos la funcion Init()
**comando:**
```rust
#[no_mangle]
extern fn init() {
    let config: InitEscrow = msg::load().expect("Unable to decode InitEscrow");

    if config.ft_program_id.is_zero() {
        panic!("FT program address can't be 0");
    }

    let escrow = Escrow {
        ft_program_id: config.ft_program_id,
        ..Default::default()
    };
    unsafe {
        ESCROW = Some(escrow);
    }
}
```


### PASO 5 Definimos la funcion Handle()
**comando:**
```rust
#[async_main]
async fn main() {
    let action: EscrowAction = msg::load().expect("Unable to decode EscrowAction");
    let escrow = unsafe { ESCROW.get_or_insert(Default::default()) };
    match action {
        EscrowAction::Create {
            buyer,
            seller,
            amount,
        } => escrow.create(buyer, seller, amount),
        EscrowAction::Deposit(wallet_id) => {
            escrow
                .transactions
                .insert(escrow.transaction_id, Some(action));
            escrow.deposit(None, wallet_id).await
        }
        EscrowAction::Confirm(wallet_id) => {
            escrow
                .transactions
                .insert(escrow.transaction_id, Some(action));
            escrow.confirm(None, wallet_id).await
        }
        EscrowAction::Refund(wallet_id) => {
            escrow
                .transactions
                .insert(escrow.transaction_id, Some(action));
            escrow.refund(None, wallet_id).await
        }
        EscrowAction::Cancel(wallet_id) => escrow.cancel(wallet_id).await,
        EscrowAction::Continue(transaction_id) => escrow.continue_transaction(transaction_id).await,
    }
}
```

### PASO 5 Definimos la funcion State()
**comando:**
```rust
#[no_mangle]
extern fn state() {
    let escrow = unsafe { ESCROW.take().expect("Uninitialized Escrow state") };
    msg::reply::<EscrowState>(escrow.into(), 0).expect("Failed to share state");
}

impl From<Escrow> for EscrowState {
    fn from(state: Escrow) -> Self {
        Self {
            ft_program_id: state.ft_program_id,
            wallets: state
                .wallets
                .iter()
                .map(|(id, wallet)| (*id, *wallet))
                .collect(),
            id_nonce: state.id_nonce,
            transaction_id: state.transaction_id,
            transactions: state
                .transactions
                .iter()
                .map(|(a, b)| (*a, b.clone()))
                .collect(),
        }
    }
}
```


## Directorio state


### PASO 1 Definimos el estado y la metadata asociada en el directorio state
**comando:**
```rust
use io::*;
use gstd::prelude::*;
use primitive_types::U256;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = EscrowState;

    pub fn info(state: State, wallet_id: U256) -> Wallet {
        let (_, wallet) = *state
            .wallets
            .iter()
            .find(|(id, _)| id == &wallet_id)
            .unwrap_or_else(|| panic!("Wallet with the {wallet_id} ID doesn't exist"));

        wallet
    }

    pub fn created_wallets(state: State) -> Vec<(WalletId, Wallet)> {
        state
            .wallets
            .iter()
            .map(|(wallet_id, wallet)| (*wallet_id, *wallet))
            .collect()
    }
}
```

## Despliega el contrato en la plataforma IDEA e interactua con tu contrato.