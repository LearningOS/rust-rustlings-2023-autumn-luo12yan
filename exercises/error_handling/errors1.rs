// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct CustomError {
    message: String,
}

impl Error for CustomError {}

impl CustomError {
    fn new(message: &str) -> Self {
        CustomError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<&str> for CustomError {
    fn from(string: &str) -> Self {
        CustomError::new(string)
    }
}

pub fn generate_nametag_text(name: String) -> Result<String, CustomError> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err(CustomError::new("`name` was empty; it must be nonempty."))
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
