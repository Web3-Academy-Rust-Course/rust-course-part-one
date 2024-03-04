fn main() {
    //if/else
    let age = 25;


if age >= 18 {
    println!("You are eligible to learn about smart contracts!");
} else {
    println!("Come back when you're older and wiser!");
}

    //else if
let transaction_type = "payment";


if transaction_type == "payment" {
    println!("Processing payment transaction...");
} else if transaction_type == "transfer" {
    println!("Transferring funds...");
} else {
    println!("Unknown transaction type!");
}


}
