use druid::{Data, Lens};
use std::default::Default;
use bigdecimal::num_bigint::BigUint;

// Datenmodelle für Hauptmenü, Alice und Bob
#[derive(Clone, Data, Lens)]
pub struct MainMenuModel {
    pub(crate) modul_width: String,
    pub(crate) miller_rabin_iterations: String,
    pub(crate) basis_length: String,
    pub(crate) public_exponent_alice: String,
    pub(crate) public_exponent_bob: String,
}

#[derive(Clone, Data, Lens)]
pub struct AliceModel {
    pub(crate) plaintext: String,
    pub(crate) ciphertext: String,
    pub(crate) signature: String,
    pub(crate) signature_status: bool,
    pub(crate) private_exponent: String,
}

#[derive(Clone, Data, Lens)]
pub struct BobModel {
    pub(crate) plaintext: String,
    pub(crate) ciphertext: String,
    pub(crate) signature: String,
    pub(crate) signature_status: bool,
    pub(crate) private_exponent: String,
}

// Datenmodell für die gesamte App -- quasi die ebene über den einzelnen Datenmodellen
#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_view: View,
    pub main_menu: MainMenuModel,
    pub alice: AliceModel,
    pub bob: BobModel,
}

// Enum für verschiedene Ansichten
#[derive(Clone, Data, PartialEq)]
pub enum View {
    MainMenu,
    Alice,
    Bob,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            current_view: View::MainMenu,
            main_menu: MainMenuModel {
                modul_width: String::new(),
                miller_rabin_iterations: String::new(),
                basis_length: String::new(),
                public_exponent_alice: String::new(),
                public_exponent_bob: String::new(),
            },
            alice: AliceModel {
                plaintext: String::new(),
                ciphertext: String::new(),
                signature: String::new(),
                signature_status: false,
                private_exponent: String::new(),
            },
            bob: BobModel {
                plaintext: String::new(),
                ciphertext: String::new(),
                signature: String::new(),
                signature_status: false,
                private_exponent: String::new(),
            },
        }
    }
}
