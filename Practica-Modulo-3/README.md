## Contrato inteligente: Semáforo

## Inicio: Clonar el template para contratos inteligentes

**comando:**
```bash
git clone https://github.com/Vara-Lab/Smart-Contract-Template.git
```

## Directorio IO

### PASO 1 Definir las acciones para el semáforo.
```rust
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TrafficLightAction {
    Green,
    Yellow,
    Red
}

```

### PASO 2 Definir las eventos para el semáforo.
```rust
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TrafficLightEvent {
    Green,
    Yellow,
    Red
}

```

### PASO 3 Definir las acciones, estado y eventos como Metadata.
```rust

#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoTrafficLightState {
    pub current_light: String,
    pub all_users: Vec<(ActorId, String)>,
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<TrafficLightAction, TrafficLightEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<IoTrafficLightState>;
}

```


## Directorio src

### PASO 1 Definir el estado principal del semáforo dentro de la variable estática mutable STATE.
```rust

static mut STATE:Option<TrafficLightState> = None;

#[derive(Clone, Default)]
pub struct TrafficLightState {
    pub current_light: String,
    pub all_users: HashMap<ActorId, String>,
}

```

### PASO 2 Definir definir implementaciones de ser necesarias.
```rust
impl TrafficLightState {
    #[allow(dead_code)]
    async fn firstmethod(&mut self) {}
    #[allow(dead_code)]
    async fn secondmethod(&mut self) {}
    #[allow(dead_code)]
    async fn thirdmethod(&mut self) {}
}
```

### PASO 3 Definir en el interior de la función init() para inicializar el estado principal.
```rust
#[no_mangle]
extern "C" fn init() {
    
    let state = TrafficLightState {
        ..Default::default()
    };

    unsafe { STATE = Some(state) };
}

```

### PASO 4 Definir en el interior de la función Handle y definimos Acción->Transición->Evento.
**comando:**
```rust
#[async_main]
async fn main() {
    // We load the input message
    let action: TrafficLightAction = msg::load().expect("Could not load Action");

    let state = unsafe { STATE.as_mut().expect("Unexpected error in factory_state") };

    // We receive an action from the user and update the state. Example:
    let result: TrafficLightEvent = match action {
        TrafficLightAction::Green => {
            state.current_light = "Green".to_string();

            // Update your second field in state.
            state.all_users.insert(msg::source(), "Green".to_string());

            TrafficLightEvent::Green
        }
        TrafficLightAction::Yellow => {
            // Update your first field in state.
            state.current_light = "Yellow".to_string();

            // Update your second field in state.
            state.all_users.insert(msg::source(), "Yellow".to_string());

            TrafficLightEvent::Yellow
        }
        TrafficLightAction::Red => {
            // Update your first field in state.
            state.current_light = "Red".to_string();

            // Update your second field in state.
            state.all_users.insert(msg::source(), "Red".to_string());

            // Generate your event.
            TrafficLightEvent::Red
        }
    };

    msg::reply(result, 0).expect("Failed to encode or reply");
}

```
### PASO 5 Definir en el interior de la función state.

```rust
#[no_mangle]
extern "C" fn state() {
    let state = unsafe { STATE.take().expect("Unexpected error in taking state") };

    msg::reply::<IoTrafficLightState>(state.into(), 0).expect(
        "Failed to encode or reply with `<ContractMetadata as Metadata>::State` from `state()`",
    );
}

// Implementation of the From trait for converting CustomStruct to IoCustomStruct
impl From<TrafficLightState> for IoTrafficLightState {
    // Conversion method
    fn from(value: TrafficLightState) -> Self {
        // Destructure the CustomStruct object into its individual fields
        let TrafficLightState {
            current_light,
            all_users,
        } = value;

        // Perform some transformation on second field, cloning its elements (Warning: Just for HashMaps!!)
        let all_users = all_users.iter().map(|(k, v)| (*k, v.clone())).collect();

        // Create a new IoCustomStruct object using the destructured fields
        Self {
            current_light,
            all_users,
        }
    }
}
```

## Directorio State

### PASO 1 Definir las implementaciones para el estado.
```rust
#![no_std]

use io::*;
use gmeta::{ Metadata, metawasm};
use gstd::{ ActorId, prelude::*};

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;


    // Add your State functions

    pub fn state(state: State) -> IoTrafficLightState{
        state
    }

}

```

## Despliega el contrato en la plataforma IDEA e interactua con tu contrato.

## Integración con el Frontend: Semáforo

## Prerequisites

Make sure you have the following dependencies installed before starting:
- [Node.js](https://nodejs.org/)
- [npm](https://www.npmjs.com/) (Node.js package manager)

# 1. Inicio: Clonar el template para frontend

1. Clone this repository to your local machine:

    ```bash
    git clone https://github.com/Vara-Lab/Integration-Kit.git
    ```

2. Navigate to the project directory:

    ```bash
    cd Integration-Kit/Chakra-UI-Vite-Template
    ```

3. Install the dependencies:

    ```bash
    yarn install
    ```
4. Start the project on localhost:

    ```bash
    yarn start
   ```

# 2. Ingresar al home y actualizar la metadata y programID de los componentes de React:

```jsx
// Add your programID
  const programIDFT ="your programID";

// Add your metadata.txt
 const meta ="your metadata";

```

# 3. Usa los botones para interactuar con tu contrato inteligente en Vara Network.

# Felicidades ya tienes tu primer contrato inteligente integrado a Vara Network.  