use std::io;
use std::io::Write;

pub fn round_to(value: f64, granularity: f64) -> f64 {
    let scale_factor = 1.0 / granularity;
    let scaled_up = value * scale_factor;
    let rounded = scaled_up.round();
    let scaled_down = rounded / scale_factor;
    scaled_down
}

pub fn prompt_line<T: std::str::FromStr>(prompt: &str) -> Option<T> {
    print!("{prompt}");
    if let Ok(_) = io::stdout().flush() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        return match input.parse() {
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
