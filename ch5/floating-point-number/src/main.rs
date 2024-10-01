const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn to_parts(n: f32) -> (u32, u32, u32) {
    let sign = n.to_bits() >> 31;
    let exponent = (n.to_bits() >> 23) & 0xFF;
    let mantissa = n.to_bits() & 0x7FFFFF;
    (sign, exponent, mantissa)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let sign_ = (-1.0_f32).powf(sign as f32);
    let exponent_ = RADIX.powf((exponent as i32 - BIAS) as f32);
    let mut mantissa_ = 1.0_f32;

    for i in 0..23 {
        let b = fraction & (1 << i);
        if b != 0 {
            mantissa_ += 2.0_f32.powf((i - 23) as f32);
        }
    }
    (sign_, exponent_, mantissa_)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

fn main() {
    let n = 42.42_f32;

    let (sign, exponent, fraction) = to_parts(n);
    let (sign_, exponent_, mantissa) = decode(sign, exponent, fraction);
    let n_ = from_parts(sign_, exponent_, mantissa);

    assert_eq!(n, n_);
}
