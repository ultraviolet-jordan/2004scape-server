use crate::script::ScriptState;

#[inline(always)]
pub(crate) fn append_num(state: &mut ScriptState) {
    let b: String = state.pop_string();
    let a: i32 = state.pop_int();
    state.push_string(b + &a.to_string());
}

#[inline(always)]
pub(crate) fn append(state: &mut ScriptState) {
    let b: String = state.pop_string();
    let a: String = state.pop_string();
    state.push_string(a + &b);
}

#[inline(always)]
pub(crate) fn append_signnum(state: &mut ScriptState) {
    let b: String = state.pop_string();
    let a: i32 = state.pop_int();
    if a >= 0 {
        state.push_string(b + "+" + &a.to_string());
    } else {
        state.push_string(b + &a.to_string());
    }
}

#[inline(always)]
pub(crate) fn lowercase(state: &mut ScriptState) {
    let a: String = state.pop_string().to_ascii_lowercase();
    state.push_string(a);
}

#[rustfmt::skip]
#[inline(always)]
pub(crate)fn text_gender(state: &mut ScriptState) {
    let female: String = state.pop_string();
    let male: String = state.pop_string();
    match state.get_active_player() {
        Ok(player) => {
            if player.gender() == 0 {
                state.push_string(male);
            } else {
                state.push_string(female);
            }
        }
        Err(err) => state.abort(err),
    }
}

#[inline(always)]
pub(crate) fn to_string(state: &mut ScriptState) {
    let a: i32 = state.pop_int();
    state.push_string(a.to_string());
}

#[inline(always)]
pub(crate) fn compare(state: &mut ScriptState) {
    let b: String = state.pop_string();
    let a: String = state.pop_string();
    let len1: usize = a.len();
    let len2: usize = b.len();
    let limit: usize = len1.min(len2);
    for (index, (c1, c2)) in a.chars().zip(b.chars()).enumerate() {
        if index >= limit {
            break;
        }
        let code1: i32 = c1 as i32;
        let code2: i32 = c2 as i32;
        if code1 != code2 {
            state.push_int(code1 - code2);
            return;
        }
    }
    state.push_int((len1 - len2) as i32);
}

#[inline(always)]
pub(crate) fn text_switch(state: &mut ScriptState) {
    let c: i32 = state.pop_int();
    let b: String = state.pop_string();
    let a: String = state.pop_string();
    if c == 1 {
        state.push_string(a);
    } else {
        state.push_string(b);
    }
}

#[inline(always)]
pub(crate) fn append_char(state: &mut ScriptState) {
    let b: String = state.pop_string();
    let a: i32 = state.pop_int();
    if a == -1 {
        return state.abort(String::from("null char"));
    }
    match std::char::from_u32((a & 0xffff) as u32) {
        None => state.abort(String::from("bad char")),
        Some(char) => state.push_string(b + &char.to_string()),
    }
}

#[inline(always)]
pub(crate) fn string_length(state: &mut ScriptState) {
    let a: String = state.pop_string();
    state.push_int(a.len() as i32);
}

#[inline(always)]
pub(crate) fn substring(state: &mut ScriptState) {
    let string: String = state.pop_string();
    let end: usize = state.pop_int() as usize;
    let start: usize = state.pop_int() as usize;
    state.push_string(string[start..end].to_string());
}

#[inline(always)]
pub(crate) fn string_indexof_char(state: &mut ScriptState) {
    let b: String = state.pop_string();
    let a: i32 = state.pop_int();
    if a == -1 {
        return state.abort(String::from("null char"));
    }
    match std::char::from_u32((a & 0xffff) as u32) {
        None => state.abort(String::from("bad char")),
        Some(char) => {
            state.push_int(
                b.chars()
                    .position(|c| c == char)
                    .map_or(-1, |index| index as i32), // return -1 if not found.
            );
        }
    }
}

#[inline(always)]
pub(crate) fn string_indexof_string(state: &mut ScriptState) {
    let b: String = state.pop_string();
    let a: String = state.pop_string();
    state.push_int(b.find(&a).map_or(-1, |index| index as i32)); // return -1 if not found.
}
