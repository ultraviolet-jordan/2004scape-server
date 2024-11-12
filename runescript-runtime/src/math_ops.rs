use crate::bits::Bits;
use crate::script::{ScriptOpcode, ScriptState};
use crate::trig::Trig;
use rand::random;

pub struct MathOps;

impl MathOps {
    #[inline(always)]
    pub fn new() -> MathOps {
        return MathOps;
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn push(&self, state: &mut ScriptState, code: ScriptOpcode, trig: &Trig, bits: &Bits) -> Result<(), String> {
        return match code {
            ScriptOpcode::Add => self.add(state),
            ScriptOpcode::Sub => self.sub(state),
            ScriptOpcode::Multiply => self.multiply(state),
            ScriptOpcode::Divide => self.divide(state),
            ScriptOpcode::Random => self.random(state),
            ScriptOpcode::RandomInc => self.randominc(state),
            ScriptOpcode::Interpolate => self.interpolate(state),
            ScriptOpcode::AddPercent => self.addpercent(state),
            ScriptOpcode::SetBit => self.setbit(state),
            ScriptOpcode::ClearBit => self.clearbit(state),
            ScriptOpcode::TestBit => self.testbit(state),
            ScriptOpcode::Modulo => self.modulo(state),
            ScriptOpcode::Pow => self.pow(state),
            ScriptOpcode::InvPow => self.invpow(state),
            ScriptOpcode::And => self.and(state),
            ScriptOpcode::Or => self.or(state),
            ScriptOpcode::Min => self.min(state),
            ScriptOpcode::Max => self.max(state),
            ScriptOpcode::Scale => self.scale(state),
            ScriptOpcode::BitCount => self.bitcount(state),
            ScriptOpcode::ToggleBit => self.togglebit(state),
            ScriptOpcode::SetBitRange => self.setbit_range(state, bits),
            ScriptOpcode::ClearBitRange => self.clearbit_range(state, bits),
            ScriptOpcode::GetBitRange => self.getbit_range(state),
            ScriptOpcode::SetBitRangeToInt => self.setbit_range_toint(state, bits),
            ScriptOpcode::SinDeg => self.sin_deg(state, trig),
            ScriptOpcode::CosDeg => self.cos_deg(state, trig),
            ScriptOpcode::Atan2Deg => self.atan2_deg(state),
            ScriptOpcode::Abs => self.abs(state),
            _ => Err(format!("Unrecognised math ops code: {:?}", code)),
        };
    }

    #[inline(always)]
    fn add(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a.wrapping_add(b));
        return Ok(());
    }

    #[inline(always)]
    fn sub(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a.wrapping_sub(b));
        return Ok(());
    }

    #[inline(always)]
    fn multiply(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a.wrapping_mul(b));
        return Ok(());
    }

    #[inline(always)]
    fn divide(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a.wrapping_div(b));
        return Ok(());
    }

    #[inline(always)]
    fn random(&self, state: &mut ScriptState) -> Result<(), String> {
        let a: f64 = state.pop_int() as f64;
        state.push_int((random::<f64>() * a) as i32);
        return Ok(());
    }

    #[inline(always)]
    fn randominc(&self, state: &mut ScriptState) -> Result<(), String> {
        let a: f64 = state.pop_int().wrapping_add(1) as f64;
        state.push_int((random::<f64>() * a) as i32);
        return Ok(());
    }

    #[inline(always)]
    fn interpolate(&self, state: &mut ScriptState) -> Result<(), String> {
        let x: i32 = state.pop_int();
        let x1: i32 = state.pop_int();
        let x0: i32 = state.pop_int();
        let y1: i32 = state.pop_int();
        let y0: i32 = state.pop_int();
        let floor: f64 = (y1.wrapping_sub(y0) as f64 / x1.wrapping_sub(x0) as f64).floor();
        state.push_int(((floor * x.wrapping_sub(x0) as f64) + y0 as f64) as i32);
        return Ok(());
    }

    #[inline(always)]
    fn addpercent(&self, state: &mut ScriptState) -> Result<(), String> {
        let percent: i32 = state.pop_int();
        let num: i32 = state.pop_int();
        state.push_int(
            num.wrapping_mul(percent)
                .wrapping_div(100)
                .wrapping_add(num),
        );
        return Ok(());
    }

    #[inline(always)]
    fn setbit(&self, state: &mut ScriptState) -> Result<(), String> {
        let bit: i32 = state.pop_int();
        let value: i32 = state.pop_int();
        state.push_int(value | (1i32.wrapping_shl(bit as u32)));
        return Ok(());
    }

    #[inline(always)]
    fn clearbit(&self, state: &mut ScriptState) -> Result<(), String> {
        let bit: i32 = state.pop_int();
        let value: i32 = state.pop_int();
        state.push_int(value & !1i32.wrapping_shl(bit as u32));
        return Ok(());
    }

    #[inline(always)]
    fn testbit(&self, state: &mut ScriptState) -> Result<(), String> {
        let bit: i32 = state.pop_int();
        let value: i32 = state.pop_int();
        state.push_int(((value & (1i32.wrapping_shl(bit as u32))) != 0) as i32);
        return Ok(());
    }

    #[inline(always)]
    fn modulo(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a.wrapping_rem(b));
        return Ok(());
    }

    #[inline(always)]
    fn pow(&self, state: &mut ScriptState) -> Result<(), String> {
        let exponent: i32 = state.pop_int();
        let base: i32 = state.pop_int();
        state.push_int(base.wrapping_pow(exponent as u32));
        return Ok(());
    }

    #[inline(always)]
    fn invpow(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        if a == 0 || b == 0 {
            state.push_int(0);
        } else {
            match b {
                1 => state.push_int(a),
                2 => state.push_int((a as f64).sqrt() as i32),
                3 => state.push_int((a as f64).cbrt() as i32),
                4 => state.push_int((a as f64).sqrt().sqrt() as i32),
                _ => state.push_int(a.pow((1.0 / b as f64) as u32)),
            }
        }
        return Ok(());
    }

    #[inline(always)]
    fn and(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a & b);
        return Ok(());
    }

    #[inline(always)]
    fn or(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a | b);
        return Ok(());
    }

    #[inline(always)]
    fn min(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a.min(b));
        return Ok(());
    }

    #[inline(always)]
    fn max(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a.max(b));
        return Ok(());
    }

    #[inline(always)]
    fn scale(&self, state: &mut ScriptState) -> Result<(), String> {
        let c: i32 = state.pop_int();
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a.wrapping_mul(c).wrapping_div(b));
        return Ok(());
    }

    #[inline(always)]
    fn bitcount(&self, state: &mut ScriptState) -> Result<(), String> {
        let a: i32 = state.pop_int();
        state.push_int(Bits::bitcount(a));
        return Ok(());
    }

    #[inline(always)]
    fn togglebit(&self, state: &mut ScriptState) -> Result<(), String> {
        let bit: i32 = state.pop_int();
        let value: i32 = state.pop_int();
        state.push_int(value ^ (1i32.wrapping_shl(bit as u32)));
        return Ok(());
    }

    #[inline(always)]
    fn setbit_range(&self, state: &mut ScriptState, bits: &Bits) -> Result<(), String> {
        let end: i32 = state.pop_int();
        let start: i32 = state.pop_int();
        let num: i32 = state.pop_int();
        state.push_int(bits.setbit_range(num, start, end));
        return Ok(());
    }

    #[inline(always)]
    fn clearbit_range(&self, state: &mut ScriptState, bits: &Bits) -> Result<(), String> {
        let end: i32 = state.pop_int();
        let start: i32 = state.pop_int();
        let num: i32 = state.pop_int();
        state.push_int(bits.clearbit_range(num, start, end));
        return Ok(());
    }

    #[inline(always)]
    fn getbit_range(&self, state: &mut ScriptState) -> Result<(), String> {
        let end: i32 = state.pop_int();
        let start: i32 = state.pop_int();
        let num: i32 = state.pop_int();
        let r: i32 = 31i32.wrapping_sub(end);
        state.push_int(
            ((num.wrapping_shl(r as u32) as u32) >> (start.wrapping_add(r) as u32)) as i32,
        );
        return Ok(());
    }

    #[inline(always)]
    fn setbit_range_toint(&self, state: &mut ScriptState, bits: &Bits) -> Result<(), String> {
        let end: i32 = state.pop_int();
        let start: i32 = state.pop_int();
        let value: i32 = state.pop_int();
        let num: i32 = state.pop_int();
        state.push_int(bits.setbit_range_toint(num, value, start, end));
        return Ok(());
    }

    #[inline(always)]
    fn sin_deg(&self, state: &mut ScriptState, trig: &Trig) -> Result<(), String> {
        let a: i32 = state.pop_int();
        state.push_int(trig.sin(a));
        return Ok(());
    }

    #[inline(always)]
    fn cos_deg(&self, state: &mut ScriptState, trig: &Trig) -> Result<(), String> {
        let a: i32 = state.pop_int();
        state.push_int(trig.cos(a));
        return Ok(());
    }

    #[inline(always)]
    fn atan2_deg(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(Trig::atan2(b, a));
        return Ok(());
    }

    #[inline(always)]
    fn abs(&self, state: &mut ScriptState) -> Result<(), String> {
        let a: i32 = state.pop_int();
        state.push_int(a.abs());
        return Ok(());
    }
}
