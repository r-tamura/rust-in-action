use std::f32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f32> for Q7 {
    fn from(x: f32) -> Self {
        if x >= 1.0 {
            Q7(127)
        } else if x <= -1.0 {
            Q7(-128)
        } else {
            Q7((x * 128.0) as i8)
        }
    }
}

impl From<Q7> for f32 {
    fn from(x: Q7) -> Self {
        x.0 as f32 / 128.0
    }
}

fn main() {
    let x = Q7::from(0.5);
    assert_eq!(x, Q7::from(0.5));
}
