// Don't modify code in main! Bt make it print "Hello world" instead of "()"
fn main() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{:?}", s2);
}

// Only modify the code below! Tip: this function should return something!
fn take_ownership(s: String) -> String {
    println!("{}", &s);
    s
}
