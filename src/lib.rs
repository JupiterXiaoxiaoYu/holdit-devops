use wasm_bindgen::prelude::*;
use zkwasm_rest_abi::*;
use zkwasm_rest_convention::CommonState;
pub mod config;
pub mod state;
pub mod player;
pub mod error;
pub mod command;
pub mod settlement;

use crate::config::Config;
use crate::state::{State, Transaction};
zkwasm_rest_abi::create_zkwasm_apis!(Transaction, State, Config);
