fn greet_world() {
    println!("Hello, world!");
    let southen_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southen_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
