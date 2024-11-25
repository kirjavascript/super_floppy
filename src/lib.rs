mod solver;


use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;


use solver::*;

#[wasm_bindgen]
pub struct Solver{}

#[wasm_bindgen]
impl Solver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Solver {}
    }

}
