// Generic struct representing a transaction with generic data and signature types
#[allow(dead_code)]
struct Transaction<T, H> {
    from: String,
    to: String,
    data: T,
    signature: H,
}

fn main() {
    // Transaction with integer data and string signature
    let transaction1 = Transaction {
        from: String::from("Alice"),
        to: String::from("Bob"),
        data: 100,
        signature: String::from("Signature1"),
    };

    // Transaction with string data and byte array signature
    let transaction2 = Transaction {
        from: String::from("Bob"),
        to: String::from("Charlie"),
        data: String::from("Transfer 50 tokens"),
        signature: vec![1, 2, 3, 4],
    };

    println!("Transaction1 data: {}", transaction1.data);
    println!("Transaction2 data: {}", transaction2.data);
}
