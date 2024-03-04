#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_ne!(add(1, 1), 3); // assert_ne checks for inequality
}

#[allow(dead_code)]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("Our test should pass!");
}
