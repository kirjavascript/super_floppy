mod solver;

use solver::*;

/* bindings needed:
 *
 * get_solutions
 */

use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::{to_value, from_value};

#[wasm_bindgen]
pub struct SolverWrap {
    state: SuperFloppy,
    prune_table: PruningTable,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
impl SolverWrap {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        SolverWrap {
            state: SuperFloppy::solved(),
            prune_table: std::collections::HashMap::new(),
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
    pub fn solve(&mut self) -> String {
        for i in 0..20 {
            let moves = search(&self.state, i, &self.prune_table);
            if let Some(moves) = moves {
                return alg_string(Some(moves));
            }
        }

        return "err".to_string();
    }

    #[wasm_bindgen]
    pub fn set_pruning_table(&mut self, table: &[u8]) {
        self.prune_table = bincode::deserialize(&table).unwrap();
    }

    #[wasm_bindgen]
    pub fn set_random_state(&mut self) {
        self.state = SuperFloppy::random_state();
    }

    #[wasm_bindgen]
    pub fn set_solved_state(&mut self) {
        self.state = SuperFloppy::solved();
    }

    #[wasm_bindgen]
    pub fn do_moves(&mut self, moves: String) {
        self.state.do_moves(parse(&moves));
    }
}
