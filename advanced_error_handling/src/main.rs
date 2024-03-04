use std::fs::File;
use std::io::Read;

fn custom_error() {
    #[derive(Debug)]
    enum MyError {
        InvalidInput(String),
        ResourceNotFound(String),
    }

    fn validate_user_input(input: Option<String>) -> Result<(), MyError> {
        if input.is_none() {
            return Err(MyError::ResourceNotFound("Input is missing".to_string()));
        }

        let input_str = input.unwrap();

        if input_str.chars().any(|c| !c.is_alphanumeric()) {
            return Err(MyError::InvalidInput(
                "Input must only contain alphanumeric characters".to_string(),
            ));
        }

        Ok(())
    }

    // We will use mock input strings here
    let _ = validate_user_input(Some("heyall".to_string()));
    println!("Succesiful!, you succesifuly validated user input!");
    let input_not_valid = validate_user_input(Some("something is invalid @".to_string()));
    match input_not_valid {
        Ok(()) => println!("ok"),
        Err(error) => println!("error: {:?}", error),
    }
}

fn error_closure() {
    fn read_file() -> Result<String, String> {
        let mut file = File::open("example.txt")
            .map_err(|err| format!("While opening file this occured: {}", err))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| format!("Error reading file: {}", err))?;
        Ok(contents)
    }
    println!("Result for Error closure is : {:?}", read_file());
}

fn main() {
    custom_error();
    error_closure();
}
