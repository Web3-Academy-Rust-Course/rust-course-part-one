#[derive(Debug)]
enum MyError {
    InvalidInput(String),
    ResourceNotFound(String),
    // ... (define custom error for input that is too short and implement it in the code)
}

fn validate_user_input(input: Option<String>) -> Result<(), MyError> {
    if input.is_none() {
        return Err(MyError::ResourceNotFound("Input is missing".to_string()));
    }

    let input_str = input.unwrap();

    // replace _ with custom error to make it work
    if input_str.len() < 5 {
        return Err(_("Input must be at least 5 characters long".to_string()));
    }

    if input_str.chars().any(|c| !c.is_alphanumeric()) {
        return Err(MyError::InvalidInput(
            "Input must only contain alphanumeric characters".to_string(),
        ));
    }

    Ok(())
}

fn main() {
    let _ = validate_user_input(Some("heyall".to_string()));
    println!("Succesiful!, you succesifuly validated user input!");
    let InputTooShortError = validate_user_input(Some("h".to_string()));
    println!(
        "You got error becuase this input is too short: {:?}",
        InputTooShortError
    );
}
