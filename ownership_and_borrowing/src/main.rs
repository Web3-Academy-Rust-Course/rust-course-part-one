#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() {
    let block_data = (String::from("Transaction 1"), String::from("Transaction 2"));

    let _block_data_2 = block_data.0;

    // we can still acces Transaction 1 becuase block_data is still the owner of it, however if we
    // try to acces Transaction 2 we would run into error becuase it's ownership is transferred to
    // the block_data_2
    println!("{:?}", block_data.1);
    // println!("{:?}", block_data.0);

    let transaction_2_data = &block_data.1;
    // transaction_2_data is borrowing transaction 2 from block_data by reference
    println!("The {} in block_data is still owned by block_data, however transaction_2_data is borrowing it by reference and it is shown here: {} ", block_data.1, transaction_2_data);

    // Moving:
    let x = 10; // x owns the value 10
    let y = x; // y gets a copy of 10, x still owns 10
    let z = x; // z gets another copy of 10, x still owns 10 (since 10 is `Copy`)
               // this is possible becuase integers are not moving ownership, they are implementing Copy trait,
               // therefore they do not transfer ownership

    let mut v: Vec<i32> = Vec::new(); // v owns an empty vector
    let w = v; // w gets ownership of the empty vector, v loses access (since Vec is not `Copy`)
    println!("w has empty vec {:?}", w);
    // Now, v can't be used anymore!
    // println!("v doesn't have empty vec anymore {:?}", v);
}
