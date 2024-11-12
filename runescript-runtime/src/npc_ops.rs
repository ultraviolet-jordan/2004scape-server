use crate::script::ScriptState;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub type Npc;

    // ---- properties

    #[wasm_bindgen(method, getter = nid)]
    pub fn uid(this: &Npc) -> i32;

    #[wasm_bindgen(method, setter = activeScript)]
    pub fn active_script(this: &Npc, state: ScriptState);

    // ---- functions
}
