## Contrato inteligente: Delayed Messages

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
#[derive(Encode, Decode, TypeInfo)]
pub enum Action {
    FTDelayedMessage_0s(u128),
    FTDelayedMessage_10s(u128),
    FTDelayedMessage_20s(u128),
    FTDelayedMessage_30s(u128),
    FTDelayedMessage_1m(u128),
    FTDelayedMessage_3m(u128),
    FTDelayedMessage_5m(u128),   
}

```

### PASO 2 Agregamos las acciones del Token Fungible

**comando:**
```rust
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

```

### PASO 3 Declarar los eventos 

**comando:**
```rust

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    SuccessfulCreate,
    SuccessfulDestroy,
    SuccessfulTransfer
}

```

### PASO 4 Declarar una estructura para el inicio del programa, aquí agregamos el programa de token fungible

**comando:**
```rust

#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitFT {
   
    pub ft_program_id: ActorId,
}
```

### PASO 5 Declararamos los eventos para token fungible

**comando:**
```rust

#[derive(Encode, Decode, TypeInfo)]
pub enum FTEvent {
    Ok,
    Err,
    Balance(u128),
    PermitId(u128),
}
```



### PASO 6 Definimos ContractMetadata y el estado

**comando:**
```rust
pub struct ContractMetadata;


impl Metadata for ContractMetadata{
    type Init = In<InitFT>;
     type Handle = InOut<Action,Event>;
     type Others = ();
     type Reply=();
     type Signal = ();
     type State = Vec<(ActorId, u128)>;

}


```

## Directorio src

### Agrega las siguientes dependencias.
**comando:**
```rust
#![no_std]
use gmeta::Metadata;
use hashbrown::HashMap;
use io::*;
use gstd::{async_main, msg, exec, prelude::*, ActorId};
```

### PASO 1 Definimos una estructura Actors para incorporar implementaciones

**comando:**
```rust
#[derive(Debug, Clone, Default)]
struct Actors {  
    actors: HashMap<ActorId, u128>,
}

impl Actors {

    async fn delayed_message_0s( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 0;     
        let delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);     
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

    }


    async fn delayed_message_10s( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 3;     
        let delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);
        
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

    }

    async fn delayed_message_20s( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 6;     
        let delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

        
    }

    async fn delayed_message_30s( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 10;     
        let delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);
        
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

        
    }


    async fn delayed_message_1m( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 20;     
        let delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);
        
        currentstate.entry(msg::source()).or_insert(amount_tokens);  
       
    }

    async fn delayed_message_3m( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 60;     
        let delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);
        
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

        
    }

    async fn delayed_message_5m( &mut self, amount_tokens: u128){
       
        let currentstate = state_mut();
        let address_ft = addresft_state_mut(); 
        let payload = FTAction::Mint(amount_tokens);
        let delay = 100;     
        let delete_message =msg::send_delayed(address_ft.ft_program_id, payload, 0, delay);
        
        currentstate.entry(msg::source()).or_insert(amount_tokens);  
    
    }
}
```

### PASO 2 Definimos las variables a usar
**comando:**
```rust
// Definimos las varibles.
static mut ACTORS:Option<Actors> = None;

static mut STATE:Option<HashMap<ActorId, u128>> = None;

static mut ADDRESSFT:Option<InitFT> = None;

```

### PASO 3 Convertimos las variables a mutables 
**comando:**
```rust
fn actors_state_mut() -> &'static mut Actors  {

    unsafe { ACTORS.get_or_insert(Default::default()) }


}

fn state_mut() -> &'static mut HashMap<ActorId,u128> {

    let state = unsafe { STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }


}

fn addresft_state_mut() -> &'static mut InitFT {


    let addressft = unsafe { ADDRESSFT.as_mut()};

    unsafe { addressft.unwrap_unchecked() }


}
```

### PASO 4 Agregamos la funcion init para inicializar las variables

**comando:**
```rust
#[no_mangle]
extern "C" fn init () {

    let config: InitFT = msg::load().expect("Unable to decode InitFT");

    let _actors = Actors {
        ..Default::default()
    };

    if config.ft_program_id.is_zero() {
        panic!("FT program address can't be 0");
    }

    let initft = InitFT {
        ft_program_id: config.ft_program_id
    };

    unsafe {
        ADDRESSFT = Some(initft);
    }

   unsafe { STATE = Some(HashMap::new())}

}

```

### PASO 5 Definimos esta función main de forma asincrona usando el macro #[async_main].

**comando:**
```rust
#[async_main]
async fn main(){

    let action: Action = msg::load().expect("Could not load Action");

    let actors = unsafe { ACTORS.get_or_insert(Actors::default()) };

    match action {

        Action::FTDelayedMessage_0s(amount) =>  {
         
            actors.delayed_message_0s(amount).await;   
        },
        Action::FTDelayedMessage_10s(amount) =>  {
        
            actors.delayed_message_10s(amount).await;    
        },
        Action::FTDelayedMessage_20s(amount) => {

                actors.delayed_message_20s(amount).await;             
        },
        Action::FTDelayedMessage_30s(amount) => {
     
                actors.delayed_message_30s(amount).await;
            },
        Action::FTDelayedMessage_1m(amount) => {
     
                actors.delayed_message_1m(amount).await;
            },
        Action::FTDelayedMessage_3m(amount) => {
     
                actors.delayed_message_3m(amount).await;
            },
        Action::FTDelayedMessage_5m(amount) => {
     
                actors.delayed_message_5m(amount).await;
            },
    
            };

}
```

### PASO 6 Definimos las variables a usar
**comando:**
```rust
 
    #[no_mangle]
    extern "C" fn state() {
     
        let state: <ContractMetadata as Metadata>::State =
            state_mut().iter().map(|(k, v)| (*k, *v)).collect();
         
        msg::reply(state, 0).expect("failed to encode or reply from `state()`");
    }

```


```
