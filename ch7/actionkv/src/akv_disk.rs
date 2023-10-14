use std::collections::HashMap;

use libactionkv::ActionKV;
use libactionkv::ByteStr;
use libactionkv::ByteString;

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

fn store_index_on_disk(store: &mut ActionKV, index_key: &ByteStr) {
    store.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&store.index).unwrap();
    store.index = std::collections::HashMap::new();
    store.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    const INDEX_KEY: &ByteStr = b"+index";

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
            let index_as_bytes = store.get(INDEX_KEY).unwrap().unwrap();
            let index: HashMap<ByteString, u64> = bincode::deserialize(&index_as_bytes).unwrap();
            let offset = *index.get(&key.as_bytes().to_vec()).expect("not found key");
            let value = store.get_at(offset).expect("unable to get value");
            println!("{:?}", value)
        }
        "delete" => {
            store.delete(key.as_bytes()).expect("unable to delete");
        }
        "update" => {
            let value = maybe_value.expect(USAGE).as_bytes();
            store
                .update(key.as_bytes(), value)
                .expect("unable to update");
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        "insert" => {
            let value = maybe_value.expect(USAGE).as_bytes();
            store
                .insert(key.as_bytes(), value)
                .expect("unable to insert");
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        _ => eprintln!("{}", USAGE),
    }
}
