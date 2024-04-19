use std::io;
use std::io::Write;

/// Prompts the user (without a newline) to input a value.
///
/// # Examples
///
/// ```
/// let input = akshually::io::prompt_line::<u8>("Your age: ");
/// match input {
///     Some(age) => {
///         println!("You are {age} years old.");
///     },
///     None => {
///         println!("You didn't enter an age.");
///     },
/// };
/// ```
pub fn prompt_line<T: std::str::FromStr>(prompt: &str) -> Option<T> {
    print!("{prompt}");
    if let Ok(_) = io::stdout().flush() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        return match input.trim().parse() {
            Ok(val) => Some(val),
            Err(_) => None,
        };
    }
    None
}
