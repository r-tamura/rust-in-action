use std::ops::Mul;

const BIAS: i32 = 127;
const RADIX: f32 = 2.0;
/// A simple floating point representation
///
/// +---+--------+-----------------+
/// | S |  E     | M               |
/// +---+--------+-----------------+
/// | 1 | 8 bits | 23 bits         |
/// +---+--------+-----------------+
///
pub struct MyF32 {
    sign: u8,
    exponent: u8,
    mantissa: u32,
}

impl MyF32 {
    pub fn sign(&self) -> f32 {
        (-1.0_f32).powf(self.sign as f32)
    }

    pub fn exponent(&self) -> f32 {
        RADIX.powf((self.exponent as i32 - BIAS) as f32)
    }

    pub fn mantissa(&self) -> f32 {
        let mut mantissa = 1.0_f32;
        for i in 0..23 {
            let b = self.mantissa & (1 << i);
            if b != 0 {
                mantissa += 2.0_f32.powf((i - 23) as f32);
            }
        }
        mantissa
    }
}

impl Into<f32> for MyF32 {
    fn into(self) -> f32 {
        self.sign() * self.exponent() * self.mantissa()
    }
}

impl From<f32> for MyF32 {
    fn from(f: f32) -> MyF32 {
        let bits = f.to_bits();
        let sign = (bits >> 31) as u8;

        let exponent = ((bits >> 23) & 0xFF) as u8;

        let mantissa = bits & 0x7FFFFF;

        MyF32 {
            sign,
            exponent,
            mantissa,
        }
    }
}

impl Mul for MyF32 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        let l: f32 = self.into();
        let r: f32 = rhs.into();
        l * r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_myf32() {
        let n = 42.42_f32;
        let myf32: MyF32 = n.into();
        assert_eq!(myf32.sign, 0);
        assert_eq!(myf32.exponent, 132);
        assert_eq!(myf32.mantissa, 2731540);
    }

    #[test]
    fn test_myf32_sign() {
        let n = 42.42_f32;
        let myf32: MyF32 = n.into();
        assert_eq!(myf32.sign(), 1.0);
    }

    #[test]
    fn test_myf32_exponent() {
        let n = 42.42_f32;
        let myf32: MyF32 = n.into();
        assert_eq!(myf32.exponent(), 32.0);
    }

    #[test]
    fn test_myf32_mantissa() {
        let n = 42.42_f32;
        let myf32: MyF32 = n.into();
        assert_eq!(myf32.mantissa(), 1.0 + 0.325624942779541);
    }

    #[test]
    fn 可逆性を満たす() {
        let n = 42.42_f32;
        let myf32: MyF32 = n.into();
        let n_ = myf32.sign() * myf32.exponent() * myf32.mantissa();
        assert_eq!(n, n_);
    }

    #[test]
    fn 掛け算の結果がf32型と等しくなる() {
        let n1 = 42.42_f32;
        let n2 = 42.42_f32;
        let myf32_1: MyF32 = n1.into();
        let myf32_2: MyF32 = n2.into();
        let n = n1 * n2;
        let n_ = myf32_1 * myf32_2;
        assert_eq!(n, n_);
    }
}
