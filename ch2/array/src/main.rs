fn main() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];

    let arrays = [one, two, blank1];

    let _x = &one[..];

    for a in arrays {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("{} ", n);
        }
        println!("");
    }

    println!("one: {:?}", one);
}
