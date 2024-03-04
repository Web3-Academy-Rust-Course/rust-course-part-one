use std::hash::{DefaultHasher, Hash, Hasher};

// Trait defining common properties for blocks in a blockchain
trait Block {
    fn get_hash(&self) -> String;
}

// Generic function to get the hash of a block, regardless of its specific type
fn get_block_hash<_: Block>(block: &_) -> String {
    block.get_hash()
}

// Concrete implementation of a block for a proof-of-work blockchain
#[derive(Debug)]
#[allow(dead_code)]
struct PoWBlock {
    hash: String,
    nonce: u64,
    data: Vec<u8>,
}

impl Block for PoWBlock {
    fn get_hash(&self) -> String {
        self.hash.clone() // Return the block's pre-calculated hash
    }
}

// Concrete implementation of a block for a proof-of-stake blockchain
#[derive(Debug)]
#[allow(dead_code)]
struct PoSBlock {
    validator: String,
    signature: String,
    data: Vec<u8>,
}

impl Block for PoSBlock {
    fn get_hash(&self) -> String {
        let mut hasher = DefaultHasher::new();
        let combined_bytes: Vec<u8> = self
            .validator
            .as_bytes()
            .iter()
            .cloned()
            .chain(self.signature.as_bytes().iter().cloned())
            .chain(self.data.iter().cloned())
            .collect::<Vec<u8>>();
        combined_bytes.hash(&mut hasher);
        let hash_value = hasher.finish() as u64;
        format!("0x{:016x}", hash_value)
    }
}

fn main() {
    let pow_block = PoWBlock {
        hash: String::from("0x12345..."),
        nonce: 100,
        data: vec![1, 2, 3],
    };

    let pos_block = PoSBlock {
        validator: String::from("Validator1"),
        signature: String::from("Signature..."),
        data: vec![4, 5, 6],
    };

    println!("PoW Block Hash: {}", get_block_hash(&pow_block));
    println!("PoS Block Hash: {}", get_block_hash(&pos_block));
}
