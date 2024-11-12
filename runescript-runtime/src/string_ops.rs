use crate::script::{ScriptOpcode, ScriptState};

pub struct StringOps;

impl StringOps {
    #[inline(always)]
    pub fn new() -> StringOps {
        return StringOps;
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn push(
        &self,
        state: &mut ScriptState,
        code: ScriptOpcode,
    ) -> Result<(), String> {
        return match code {
            ScriptOpcode::AppendNum => self.append_num(state),
            ScriptOpcode::Append => self.append(state),
            ScriptOpcode::AppendSignNum => self.append_signnum(state),
            ScriptOpcode::Lowercase => self.lowercase(state),
            ScriptOpcode::TextGender => state.protect(&ScriptState::ACTIVE_PLAYER, |state| self.text_gender(state)),
            ScriptOpcode::ToString => self.to_string(state),
            ScriptOpcode::Compare => self.compare(state),
            ScriptOpcode::TextSwitch => self.text_switch(state),
            ScriptOpcode::AppendChar => self.append_char(state),
            ScriptOpcode::StringLength => self.string_length(state),
            ScriptOpcode::SubString => self.substring(state),
            ScriptOpcode::StringIndexOfChar => self.string_indexof_char(state),
            ScriptOpcode::StringIndexOfString => self.string_indexof_string(state),
            _ => Err(format!("Unrecognised string ops code: {:?}", code)),
        };
    }

    #[inline(always)]
    fn append_num(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: String = state.pop_string();
        let a: i32 = state.pop_int();
        state.push_string(b + &a.to_string());
        return Ok(());
    }

    #[inline(always)]
    fn append(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: String = state.pop_string();
        let a: String = state.pop_string();
        state.push_string(a + &b);
        return Ok(());
    }

    #[inline(always)]
    fn append_signnum(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: String = state.pop_string();
        let a: i32 = state.pop_int();
        if a >= 0 {
            state.push_string(b + "+" + &a.to_string());
        } else {
            state.push_string(b + &a.to_string());
        }
        return Ok(());
    }

    #[inline(always)]
    fn lowercase(&self, state: &mut ScriptState) -> Result<(), String> {
        let a: String = state.pop_string().to_ascii_lowercase();
        state.push_string(a);
        return Ok(());
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn text_gender(&self, state: &mut ScriptState) -> Result<(), String> {
        let female: String = state.pop_string();
        let male: String = state.pop_string();
        if state.get_active_player()?.gender() == 0 {
            state.push_string(male);
        } else {
            state.push_string(female);
        }
        return Ok(());
    }

    #[inline(always)]
    fn to_string(&self, state: &mut ScriptState) -> Result<(), String> {
        let a: i32 = state.pop_int();
        state.push_string(a.to_string());
        return Ok(());
    }

    #[inline(always)]
    fn compare(&self, state: &mut ScriptState) -> Result<(), String> {
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
                return Ok(());
            }
        }
        state.push_int((len1 - len2) as i32);
        return Ok(());
    }

    #[inline(always)]
    fn text_switch(&self, state: &mut ScriptState) -> Result<(), String> {
        let c: i32 = state.pop_int();
        let b: String = state.pop_string();
        let a: String = state.pop_string();
        if c == 1 {
            state.push_string(a);
        } else {
            state.push_string(b);
        }
        return Ok(());
    }

    #[inline(always)]
    fn append_char(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: String = state.pop_string();
        let a: i32 = state.pop_int();
        if a == -1 {
            return Err("null char".to_string());
        }
        if let Some(char) = std::char::from_u32((a & 0xffff) as u32) {
            state.push_string(b + &char.to_string());
            return Ok(());
        }
        return Err("bad char".to_string());
    }

    #[inline(always)]
    fn string_length(&self, state: &mut ScriptState) -> Result<(), String> {
        let a: String = state.pop_string();
        state.push_int(a.len() as i32);
        return Ok(());
    }

    #[inline(always)]
    fn substring(&self, state: &mut ScriptState) -> Result<(), String> {
        let string: String = state.pop_string();
        let end: usize = state.pop_int() as usize;
        let start: usize = state.pop_int() as usize;
        state.push_string(string[start..end].to_string());
        return Ok(());
    }

    #[inline(always)]
    fn string_indexof_char(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: String = state.pop_string();
        let a: i32 = state.pop_int();
        if a == -1 {
            return Err("null char".to_string());
        }
        if let Some(char) = std::char::from_u32((a & 0xffff) as u32) {
            state.push_int(
                b.chars()
                    .position(|c| c == char)
                    .map_or(-1, |index| index as i32), // return -1 if not found.
            );
            return Ok(());
        }
        return Err("bad char".to_string());
    }

    #[inline(always)]
    fn string_indexof_string(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: String = state.pop_string();
        let a: String = state.pop_string();
        state.push_int(b.find(&a).map_or(-1, |index| index as i32)); // return -1 if not found.
        return Ok(());
    }
}
