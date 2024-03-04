use std::fs; // Import the `fs` module for file operations
use std::io;

#[allow(unused_assignments)]
fn result() {
    fn read_file(filename: &str) -> Result<String, std::io::Error> {
        let mut contents = String::new();
        match fs::read_to_string(filename) {
            Ok(file_contents) => contents = file_contents,
            Err(error) => return Err(error),
        }
        Ok(contents)
    }
    let result = read_file("src/some.txt");
    println!("File content: {:?}", result);
    let result_2 = read_file("src/undefined.txt");
    println!("This gives error: {:?}", result_2);
}

#[allow(unused_variables)]
fn option_example() {
    fn get_user_input() -> Option<String> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Some(input.trim().to_string()), // Extract and trim the input
            Err(error) => return None,               // Return None on I/O error
        }
    }

    let input_result = get_user_input();
    println!("User input is: {:?}", input_result);
}

fn main() {
    result();
    option_example();
}
