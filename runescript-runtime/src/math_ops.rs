use crate::bits::Bits;
use crate::script::{ScriptOpcode, ScriptState};
use crate::trig::Trig;
use crate::{BITS, TRIG};
use rand::random;

#[inline(always)]
#[rustfmt::skip]
pub fn perform_math_operation(state: &mut ScriptState, code: ScriptOpcode) -> Result<(), String> {
    return match code {
        ScriptOpcode::Add => add(state),
        ScriptOpcode::Sub => sub(state),
        ScriptOpcode::Multiply => multiply(state),
        ScriptOpcode::Divide => divide(state),
        ScriptOpcode::Random => _random(state),
        ScriptOpcode::RandomInc => randominc(state),
        ScriptOpcode::Interpolate => interpolate(state),
        ScriptOpcode::AddPercent => addpercent(state),
        ScriptOpcode::SetBit => setbit(state),
        ScriptOpcode::ClearBit => clearbit(state),
        ScriptOpcode::TestBit => testbit(state),
        ScriptOpcode::Modulo => modulo(state),
        ScriptOpcode::Pow => pow(state),
        ScriptOpcode::InvPow => invpow(state),
        ScriptOpcode::And => and(state),
        ScriptOpcode::Or => or(state),
        ScriptOpcode::Min => min(state),
        ScriptOpcode::Max => max(state),
        ScriptOpcode::Scale => scale(state),
        ScriptOpcode::BitCount => bitcount(state),
        ScriptOpcode::ToggleBit => togglebit(state),
        ScriptOpcode::SetBitRange => setbit_range(state),
        ScriptOpcode::ClearBitRange => clearbit_range(state),
        ScriptOpcode::GetBitRange => getbit_range(state),
        ScriptOpcode::SetBitRangeToInt => setbit_range_toint(state),
        ScriptOpcode::SinDeg => sin_deg(state),
        ScriptOpcode::CosDeg => cos_deg(state),
        ScriptOpcode::Atan2Deg => atan2_deg(state),
        ScriptOpcode::Abs => abs(state),
        _ => Err(format!("Unrecognised math ops code: {:?}", code)),
    };
}

#[inline(always)]
fn add(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.wrapping_add(b));
    return Ok(());
}

#[inline(always)]
fn sub(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.wrapping_sub(b));
    return Ok(());
}

#[inline(always)]
fn multiply(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.wrapping_mul(b));
    return Ok(());
}

#[inline(always)]
fn divide(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.wrapping_div(b));
    return Ok(());
}

#[inline(always)]
fn _random(state: &mut ScriptState) -> Result<(), String> {
    let a: f64 = state.pop_int() as f64;
    state.push_int((random::<f64>() * a) as i32);
    return Ok(());
}

#[inline(always)]
fn randominc(state: &mut ScriptState) -> Result<(), String> {
    let a: f64 = state.pop_int().wrapping_add(1) as f64;
    state.push_int((random::<f64>() * a) as i32);
    return Ok(());
}

#[inline(always)]
fn interpolate(state: &mut ScriptState) -> Result<(), String> {
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
fn addpercent(state: &mut ScriptState) -> Result<(), String> {
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
fn setbit(state: &mut ScriptState) -> Result<(), String> {
    let bit: i32 = state.pop_int();
    let value: i32 = state.pop_int();
    state.push_int(value | (1i32.wrapping_shl(bit as u32)));
    return Ok(());
}

#[inline(always)]
fn clearbit(state: &mut ScriptState) -> Result<(), String> {
    let bit: i32 = state.pop_int();
    let value: i32 = state.pop_int();
    state.push_int(value & !1i32.wrapping_shl(bit as u32));
    return Ok(());
}

#[inline(always)]
fn testbit(state: &mut ScriptState) -> Result<(), String> {
    let bit: i32 = state.pop_int();
    let value: i32 = state.pop_int();
    state.push_int(((value & (1i32.wrapping_shl(bit as u32))) != 0) as i32);
    return Ok(());
}

#[inline(always)]
fn modulo(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.wrapping_rem(b));
    return Ok(());
}

#[inline(always)]
fn pow(state: &mut ScriptState) -> Result<(), String> {
    let exponent: i32 = state.pop_int();
    let base: i32 = state.pop_int();
    state.push_int(base.wrapping_pow(exponent as u32));
    return Ok(());
}

#[inline(always)]
fn invpow(state: &mut ScriptState) -> Result<(), String> {
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
fn and(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a & b);
    return Ok(());
}

#[inline(always)]
fn or(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a | b);
    return Ok(());
}

#[inline(always)]
fn min(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.min(b));
    return Ok(());
}

#[inline(always)]
fn max(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.max(b));
    return Ok(());
}

#[inline(always)]
fn scale(state: &mut ScriptState) -> Result<(), String> {
    let c: i32 = state.pop_int();
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.wrapping_mul(c).wrapping_div(b));
    return Ok(());
}

#[inline(always)]
fn bitcount(state: &mut ScriptState) -> Result<(), String> {
    let a: i32 = state.pop_int();
    state.push_int(Bits::bitcount(a));
    return Ok(());
}

#[inline(always)]
fn togglebit(state: &mut ScriptState) -> Result<(), String> {
    let bit: i32 = state.pop_int();
    let value: i32 = state.pop_int();
    state.push_int(value ^ (1i32.wrapping_shl(bit as u32)));
    return Ok(());
}

#[inline(always)]
fn setbit_range(state: &mut ScriptState) -> Result<(), String> {
    let end: i32 = state.pop_int();
    let start: i32 = state.pop_int();
    let num: i32 = state.pop_int();
    state.push_int(BITS.setbit_range(num, start, end));
    return Ok(());
}

#[inline(always)]
fn clearbit_range(state: &mut ScriptState) -> Result<(), String> {
    let end: i32 = state.pop_int();
    let start: i32 = state.pop_int();
    let num: i32 = state.pop_int();
    state.push_int(BITS.clearbit_range(num, start, end));
    return Ok(());
}

#[inline(always)]
fn getbit_range(state: &mut ScriptState) -> Result<(), String> {
    let end: i32 = state.pop_int();
    let start: i32 = state.pop_int();
    let num: i32 = state.pop_int();
    let r: i32 = 31i32.wrapping_sub(end);
    state.push_int(((num.wrapping_shl(r as u32) as u32) >> (start.wrapping_add(r) as u32)) as i32);
    return Ok(());
}

#[inline(always)]
fn setbit_range_toint(state: &mut ScriptState) -> Result<(), String> {
    let end: i32 = state.pop_int();
    let start: i32 = state.pop_int();
    let value: i32 = state.pop_int();
    let num: i32 = state.pop_int();
    state.push_int(BITS.setbit_range_toint(num, value, start, end));
    return Ok(());
}

#[inline(always)]
fn sin_deg(state: &mut ScriptState) -> Result<(), String> {
    let a: i32 = state.pop_int();
    state.push_int(TRIG.sin(a));
    return Ok(());
}

#[inline(always)]
fn cos_deg(state: &mut ScriptState) -> Result<(), String> {
    let a: i32 = state.pop_int();
    state.push_int(TRIG.cos(a));
    return Ok(());
}

#[inline(always)]
fn atan2_deg(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(Trig::atan2(b, a));
    return Ok(());
}

#[inline(always)]
fn abs(state: &mut ScriptState) -> Result<(), String> {
    let a: i32 = state.pop_int();
    state.push_int(a.abs());
    return Ok(());
}
