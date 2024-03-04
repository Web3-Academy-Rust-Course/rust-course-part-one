use std::clone::Clone;
use std::collections::HashMap;

fn get_from_vec<T: Clone>(vec: &Vec<T>, index: usize) -> Option<T> {
    vec.get(index).cloned()
}

struct KVStore<K, V> {
    map: HashMap<K, V>,
}

fn main() {
    let names: Vec<String> = vec!["Alice".to_string(), "Bob".to_string()];
    match get_from_vec(&names, 5) {
        Some(value) => println!("Name is: {:?}", value),
        _ => println!("Could not find any name on that index!"),
    }

    // Initializing KVStore with u32 and u64
    let mut user_store: KVStore<u32, u64> = KVStore {
        map: HashMap::new(),
    };

    // Inserting value of 100 to the key 1
    user_store.map.insert(1, 100);
    let balance = user_store.map.get(&1);
    println!("Balance is of 1 is: {:?}", balance);

    // Making KVStore with i32 and String
    let mut other_store: KVStore<i32, String> = KVStore {
        map: HashMap::new(),
    };

    other_store.map.insert(3, "Alice".to_string());

    println!(
        "User on number 3 in store is: {:?}",
        other_store.map.get(&3)
    )
}
