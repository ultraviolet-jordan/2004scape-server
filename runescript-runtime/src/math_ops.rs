use crate::bits::Bits;
use crate::script::ScriptState;
use crate::trig::Trig;
use crate::{BITS, TRIG};

#[inline(always)]
pub(crate) fn add(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.wrapping_add(b));
}

#[inline(always)]
pub(crate) fn sub(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.wrapping_sub(b));
}

#[inline(always)]
pub(crate) fn multiply(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.wrapping_mul(b));
}

#[inline(always)]
pub(crate) fn divide(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.wrapping_div(b));
}

#[inline(always)]
pub(crate) fn random(state: &mut ScriptState) {
    let a: f64 = state.pop_int() as f64;
    state.push_int((rand::random::<f64>() * a) as i32);
}

#[inline(always)]
pub(crate) fn randominc(state: &mut ScriptState) {
    let a: f64 = state.pop_int().wrapping_add(1) as f64;
    state.push_int((rand::random::<f64>() * a) as i32);
}

#[inline(always)]
pub(crate) fn interpolate(state: &mut ScriptState) {
    let x: i32 = state.pop_int();
    let x1: i32 = state.pop_int();
    let x0: i32 = state.pop_int();
    let y1: i32 = state.pop_int();
    let y0: i32 = state.pop_int();
    let floor: f64 = (y1.wrapping_sub(y0) as f64 / x1.wrapping_sub(x0) as f64).floor();
    state.push_int(((floor * x.wrapping_sub(x0) as f64) + y0 as f64) as i32);
}

#[inline(always)]
pub(crate) fn addpercent(state: &mut ScriptState) {
    let percent: i32 = state.pop_int();
    let num: i32 = state.pop_int();
    state.push_int(
        num.wrapping_mul(percent)
            .wrapping_div(100)
            .wrapping_add(num),
    );
}

#[inline(always)]
pub(crate) fn setbit(state: &mut ScriptState) {
    let bit: i32 = state.pop_int();
    let value: i32 = state.pop_int();
    state.push_int(value | (1i32.wrapping_shl(bit as u32)));
}

#[inline(always)]
pub(crate) fn clearbit(state: &mut ScriptState) {
    let bit: i32 = state.pop_int();
    let value: i32 = state.pop_int();
    state.push_int(value & !1i32.wrapping_shl(bit as u32));
}

#[inline(always)]
pub(crate) fn testbit(state: &mut ScriptState) {
    let bit: i32 = state.pop_int();
    let value: i32 = state.pop_int();
    state.push_int(((value & (1i32.wrapping_shl(bit as u32))) != 0) as i32);
}

#[inline(always)]
pub(crate) fn modulo(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.wrapping_rem(b));
}

#[inline(always)]
pub(crate) fn pow(state: &mut ScriptState) {
    let exponent: i32 = state.pop_int();
    let base: i32 = state.pop_int();
    state.push_int(base.wrapping_pow(exponent as u32));
}

#[inline(always)]
pub(crate) fn invpow(state: &mut ScriptState) {
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
}

#[inline(always)]
pub(crate) fn and(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a & b);
}

#[inline(always)]
pub(crate) fn or(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a | b);
}

#[inline(always)]
pub(crate) fn min(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.min(b));
}

#[inline(always)]
pub(crate) fn max(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.max(b));
}

#[inline(always)]
pub(crate) fn scale(state: &mut ScriptState) {
    let c: i32 = state.pop_int();
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(a.wrapping_mul(c).wrapping_div(b));
}

#[inline(always)]
pub(crate) fn bitcount(state: &mut ScriptState) {
    let a: i32 = state.pop_int();
    state.push_int(Bits::bitcount(a));
}

#[inline(always)]
pub(crate) fn togglebit(state: &mut ScriptState) {
    let bit: i32 = state.pop_int();
    let value: i32 = state.pop_int();
    state.push_int(value ^ (1i32.wrapping_shl(bit as u32)));
}

#[inline(always)]
pub(crate) fn setbit_range(state: &mut ScriptState) {
    let end: i32 = state.pop_int();
    let start: i32 = state.pop_int();
    let num: i32 = state.pop_int();
    state.push_int(BITS.setbit_range(num, start, end));
}

#[inline(always)]
pub(crate) fn clearbit_range(state: &mut ScriptState) {
    let end: i32 = state.pop_int();
    let start: i32 = state.pop_int();
    let num: i32 = state.pop_int();
    state.push_int(BITS.clearbit_range(num, start, end));
}

#[inline(always)]
pub(crate) fn getbit_range(state: &mut ScriptState) {
    let end: i32 = state.pop_int();
    let start: i32 = state.pop_int();
    let num: i32 = state.pop_int();
    let r: i32 = 31i32.wrapping_sub(end);
    state.push_int(((num.wrapping_shl(r as u32) as u32) >> (start.wrapping_add(r) as u32)) as i32);
}

#[inline(always)]
pub(crate) fn setbit_range_toint(state: &mut ScriptState) {
    let end: i32 = state.pop_int();
    let start: i32 = state.pop_int();
    let value: i32 = state.pop_int();
    let num: i32 = state.pop_int();
    state.push_int(BITS.setbit_range_toint(num, value, start, end));
}

#[inline(always)]
pub(crate) fn sin_deg(state: &mut ScriptState) {
    let a: i32 = state.pop_int();
    state.push_int(TRIG.sin(a));
}

#[inline(always)]
pub(crate) fn cos_deg(state: &mut ScriptState) {
    let a: i32 = state.pop_int();
    state.push_int(TRIG.cos(a));
}

#[inline(always)]
pub(crate) fn atan2_deg(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    state.push_int(Trig::atan2(b, a));
}

#[inline(always)]
pub(crate) fn abs(state: &mut ScriptState) {
    let a: i32 = state.pop_int();
    state.push_int(a.abs());
}
