use libactionkv::ActionKV;

#[cfg(target_as = "windows")]
const USAGE: &str = "
Usage:
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(USAGE);
    let action = args.get(2).expect(USAGE).as_str();
    let key = args.get(3).expect(USAGE).as_str();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load file");

    match action {
        "get" => {
            let maybe_value = store.get(key.as_bytes()).expect("unable to get value");
            match maybe_value {
                None => println!("Key not found"),
                Some(value) => {
                    let value = String::from_utf8(value).expect("invalid utf8");
                    println!("{}", value);
                }
            }
        }
        "delete" => {
            store.delete(key.as_bytes()).expect("unable to delete");
        }
        "update" => {
            let value = maybe_value.expect(USAGE).as_bytes();
            store
                .update(key.as_bytes(), value)
                .expect("unable to update");
        }
        "insert" => {
            let value = maybe_value.expect(USAGE).as_bytes();
            store
                .insert(key.as_bytes(), value)
                .expect("unable to insert");
        }
        _ => eprintln!("{}", USAGE),
    }
}
