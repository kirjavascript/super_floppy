mod solver;

use solver::*;

/* bindings needed:
 *
 * set_pruning_table
 * get_solutions
 */


use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::{to_value, from_value};

#[wasm_bindgen]
pub struct SolverWrap {
    state: SuperFloppy,
}

#[wasm_bindgen]
impl SolverWrap {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        SolverWrap {
            state: SuperFloppy::solved(),
        }
    }

    #[wasm_bindgen]
    pub fn get_state(&self) -> JsValue {
        to_value(&self.state).unwrap()
    }

    #[wasm_bindgen]
    pub fn set_state(&mut self, state: JsValue) {
        self.state = from_value(state).unwrap();
    }

    #[wasm_bindgen]
    pub fn set_random_state(&mut self) {
        self.state = SuperFloppy::random_state();
    }

    #[wasm_bindgen]
    pub fn do_moves(&mut self, moves: String) {
        self.state.do_moves(parse(&moves));
    }
}
