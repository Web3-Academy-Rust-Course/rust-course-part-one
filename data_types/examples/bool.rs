// Make println! work
fn main() {
    let f: bool = false;
    let t = true;
    if t && t != f {
        println!("Success!");
    }
}
