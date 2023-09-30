fn main() {
    let a: u16 = 50115;
    let b: i16 = -15421;
    let c = a as i16;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
    println!("c: {:016b} {}", c, c);
    assert_eq!(a, b as u16);
}
