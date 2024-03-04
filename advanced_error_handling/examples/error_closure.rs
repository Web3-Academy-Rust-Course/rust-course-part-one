// Make this example work using closures
fn double_number(number_str: &str) -> Result<i32, String> {
    let number = number_str.parse::<i32>().map_err(|err| {
        eprintln!("Error: {}", err);
        err.to_string()
    })?;
    Ok(number * 2)
}

fn main() {
    println!("Result: {:?}", double_number("10").unwrap());
    println!("Result: {:?}", double_number("hello"));
}
