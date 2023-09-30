fn mock_rand(n: u8) -> f32 {
    let base = 0b0_01111110_00000000000000000000000;
    let large_n = (n as u32) << 15;
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits);
    // mが 0.5 - 1.0の範囲なので0-1の範囲に正規化
    (m - 0.5) * 2.0
}

fn main() {
    println!("入力範囲の最大値: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("入力範囲の中央値: {:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    println!("入力範囲の最小値: {:08b} -> {:?}", 0x00, mock_rand(0x00));

    println!("{:?}, {:?}", (0.15_f64 + 0.15_f64), 0.3_f64);
    println!("{:032?}, {:032?}", (0.15_f64 + 0.15_f64).to_bits(), 0.3_f64.to_bits());
    assert_eq!((0.15_f64 + 0.15_f64), 0.3_f64);
}
