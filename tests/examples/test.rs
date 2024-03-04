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
    // write test for internal_adder here
}
