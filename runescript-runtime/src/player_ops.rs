use js_sys::Reflect;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{engine_find_uid, log};
use crate::script::ScriptState;

#[derive(Clone)]
pub struct Player {
    js: JsValue,
}

impl Player {
    const X: &'static str = "x";

    pub fn null() -> Player {
        return Player {
            js: JsValue::NULL,
        }
    }

    pub fn new(js: JsValue) -> Player {
        return Player {
            js
        }
    }

    pub fn get_x(&self) -> u16 {
        return Reflect::get(&self.js, &JsValue::from(Player::X))
            .ok()
            .and_then(|value| value.as_f64())
            .map_or(0, |n| n as u16)
    }

    pub fn set_x(&self, x: u16) {
        Reflect::set(&self.js, &JsValue::from(Player::X), &JsValue::from(x));
    }
}

#[wasm_bindgen]
extern {
    pub fn hint_player(player: &JsValue, pid: i32);
}

pub fn find_uid(state: &mut ScriptState) -> Result<(), String> {
    let uid: i32 = state.pop_int();
    // log(format!("{}" ,uid).as_str());

    unsafe {
        let player = engine_find_uid(uid);
        if player.is_null() {
            log("not found!");
            state.push_int(0);
        } else {
            state.set_active_player(Player::new(player));
            state.pointer_add(state.active_player());
            state.push_int(1);
        }

        let mut player2 = state.get_active_player();
        // log(format!("{}", player.x).as_str());
        Reflect::set(&player2.js, &JsValue::from(Player::X), &JsValue::from(3222)).expect("TODO: panic message");

        let player3 = state.get_active_player();
        Reflect::set(&player3.js, &JsValue::from(Player::X), &JsValue::from(3225)).expect("TODO: panic message");

        hint_player(&player3.js, 420);

        // log(format!("{}", player2.x()).as_str());
        // log(format!("{}", player3.x()).as_str());

        // log(format!("{:?}", Reflect::get(&player, &x).expect("")).as_str());
        log(format!("{}", player3.get_x()).as_str());
    }

    // unsafe {
    //     set_bas_readyanim()
    // }
    return Ok(());
}