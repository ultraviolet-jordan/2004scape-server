pub struct Trig {
    sin: Vec<i32>,
    cos: Vec<i32>,
}

impl Trig {
    const DEGREES_RANGE: i32 = 16384;
    const DEGREES_MASK: i32 = Self::DEGREES_RANGE - 1;
    const RADIANS_TO_DEGREES_FACTOR: f64 = Self::DEGREES_RANGE as f64 / std::f64::consts::TAU;
    const SIZE: f64 = std::f64::consts::PI / (Self::DEGREES_RANGE as f64 / 2.0);

    pub fn new() -> Trig {
        let mut sin: Vec<i32> = vec![0; Self::DEGREES_RANGE as usize];
        let mut cos: Vec<i32> = vec![0; Self::DEGREES_RANGE as usize];
        for index in 0..Self::DEGREES_RANGE as usize {
            let (s, c) = f64::sin_cos(index as f64 * Trig::SIZE);
            sin[index] = (s * Self::DEGREES_RANGE as f64) as i32;
            cos[index] = (c * Self::DEGREES_RANGE as f64) as i32;
        }
        return Trig { sin, cos };
    }

    #[inline(always)]
    pub fn radians(x: i32) -> f64 {
        return (((x & Self::DEGREES_MASK) as f64) / Self::DEGREES_RANGE as f64) * std::f64::consts::TAU;
    }

    #[inline(always)]
    pub fn atan2(y: i32, x: i32) -> i32 {
        return ((f64::atan2(y as f64, x as f64) * Self::RADIANS_TO_DEGREES_FACTOR).round() as i32) & Self::DEGREES_MASK;
    }

    #[inline(always)]
    pub fn sin(&self, value: i32) -> i32 {
        return self.sin[(value & Self::DEGREES_MASK) as usize];
    }

    #[inline(always)]
    pub fn cos(&self, value: i32) -> i32 {
        return self.cos[(value & Self::DEGREES_MASK) as usize];
    }
}
