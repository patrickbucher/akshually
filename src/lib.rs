use std::io;
use std::io::Error;
use std::io::Write;
use std::result::Result;
use std::str::FromStr;

pub fn round_to(value: f64, granularity: f64) -> f64 {
    let scale_factor = 1.0 / granularity;
    let scaled_up = value * scale_factor;
    let rounded = scaled_up.round();
    let scaled_down = rounded / scale_factor;
    scaled_down
}

pub fn prompt_line<T: FromStr<Err = std::io::Error>>(prompt: &str) -> Result<T, Error> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // FIXME: why does this fail?
    // input.parse()?

    match input.parse() {
        Ok(val) => Ok(val),
        Err(err) => Err(err),
    }
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
