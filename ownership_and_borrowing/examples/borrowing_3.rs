// Tip: You can only borrow mutable object as mutable, you can not borrow an immutable object as mutable
fn main() {
    // Fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}
#[allow(unused_variables)]
fn borrow_object(s: &mut String) {}
