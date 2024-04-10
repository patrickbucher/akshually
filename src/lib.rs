//! # Akshually
//!
//! The `akshually` crate contains various unrelated functions I kept on re-implementing for
//! different projects and examples. Therefore I started collecting them in a library that doesn't
//! take up any additional dependencies.

use std::io;
use std::io::Write;

/// Rounds the `value` with the specified `granularity`.
///
/// # Examples
///
/// ```
/// assert_eq!(akshually::round_to(1.93333, 0.05), 1.95);
/// ```
pub fn round_to(value: f64, granularity: f64) -> f64 {
    let scale_factor = 1.0 / granularity;
    let scaled_up = value * scale_factor;
    let rounded = scaled_up.round();
    let scaled_down = rounded / scale_factor;
    scaled_down
}

/// Prompts the user (without a newline) to input a value.
///
/// # Examples
///
/// ```
/// let input = akshually::prompt_line::<u8>("Your age: ");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_to_nickel() {
        let result = round_to(10.0 / 3.0, 0.05);
        assert_eq!(result, 3.35);
    }
}
