use std::mem::size_of;
use std::any::type_name;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 111, 114, 119, 0];

fn info<T>(v: T, name: &str) where T: std::fmt::Debug {
    println!("{} {}", name, type_name::<T>());
    println!(" 場所: {:p}", &v);
    println!(" サイズ: {:?} バイト", size_of::<T>());
    println!(" 値: {:?}", v);
}

fn info_ref<T>(v: T, name: &str) where T: std::fmt::Pointer {
    println!("{} {}", name, type_name::<T>());
    println!(" 場所: {:p}", &v);
    println!(" サイズ: {:?} バイト", size_of::<T>());
    println!(" 参照先: {:p}", v);
}


fn main() {
    let a: usize = 42;
    let b: &[u8; 10] = &B;
    let c: Box<[u8]> = Box::new(C);

    info(a, "a");
    info_ref(b, "b (10バイトの配列への参照)");
    info_ref(c, "c (11バイトの配列へのBox)");
    info(B, "B 10バイトの配列");
    println!("{}", "B 10バイトの配列");
    println!(" 場所: {:p}", &B);
    println!(" サイズ: {:?} バイト", size_of::<[u8; 10]>());
    println!(" 値: {:?}", B);

    println!("{}", "B 10バイトの配列");
    println!(" 場所: {:p}", &C);
    println!(" サイズ: {:?} バイト", size_of::<[u8; 11]>());
    println!(" 値: {:?}", C);
}
