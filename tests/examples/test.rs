pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // we use super::*; to bring in private functions from parent into test scope
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(internal_adder(5, 4), 9)
    }
}

// Part of this test was to add main function, becuase it is essential to any rust program
fn main() {}
