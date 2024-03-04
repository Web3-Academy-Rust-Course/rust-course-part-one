use std::collections::HashMap;
use std::collections::HashSet;

// Vector
fn vector_example() {
    let mut transaction_id = vec![1, 2, 3, 4, 5];
    let first_transaction = &transaction_id[0];
    println!("The first transaction is: {}", first_transaction);
    transaction_id.push(6);
    println!("New transaction added is {}", &transaction_id[5])
}

fn hashmap_example() {
    let mut balances: HashMap<String, u64> = HashMap::new();
    balances.insert("allice.account".to_string(), 100);
    let alice_balance = balances.get("allice.account");
    println!("Alice's balance: {:?}", alice_balance);
}

fn hashset_example() {
    let mut visited_urls: HashSet<String> = HashSet::new();
    visited_urls.insert("https://docs.rs/".to_string());

    if !visited_urls.contains("https://www.rust-lang.org/") {
        println!("You haven't visited the Rust homepage yet!");
    }
    visited_urls.insert("https://www.rust-lang.org/".to_string());
    if visited_urls.contains("https://www.rust-lang.org/") {
        println!("Now, that you have visited the Rust homepage, it is icluded in visited urls!");
    }
}

fn main() {
    vector_example();
    hashmap_example();
    hashset_example();
}
