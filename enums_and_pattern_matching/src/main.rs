#[allow(dead_code)]
enum TransactionType {
    Transfer(String, String, u64), // From address, to address, amount
    MintNFT(String, u32),          // Owner address, NFT ID
    CallContract(String, Vec<u8>), // Contract address, function call data
}

#[derive(Debug)]
#[allow(dead_code)]
enum AccountState {
    Active,
    Suspended,
    Locked(String), // Reason for locking
}

fn process_transaction(transaction: TransactionType) {
    match transaction {
        TransactionType::Transfer(from, to, amount) => {
            println!("Transferring {} tokens from {} to {}", amount, from, to);
        }
        TransactionType::MintNFT(owner, nft_id) => {
            println!("Minting NFT #{} for {}", nft_id, owner);
        }
        TransactionType::CallContract(address, data) => {
            println!("Calling contract {} with data {:?}", address, data);
        }
    }
}

fn main() {
    let transaction_one = TransactionType::MintNFT("alice.account".to_string(), 1);
    let transaction_two =
        TransactionType::Transfer("alice.account".to_string(), "bob.account".to_string(), 100);
    let alice_account = AccountState::Locked("Out of funds".to_string());
    let bob_account = AccountState::Active;
    println!(
        "Aliice account state {:?}, however Bob's account is {:?}",
        alice_account, bob_account
    );
    process_transaction(transaction_one);
    process_transaction(transaction_two);
}
