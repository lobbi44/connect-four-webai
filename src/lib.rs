#![feature(crate_visibility_modifier)]

mod utils;
pub mod logic;
pub mod BitBoard;
mod Engine;
mod transposition;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}