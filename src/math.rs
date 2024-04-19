/// Rounds the `value` with the specified `granularity`.
///
/// # Examples
///
/// ```
/// assert_eq!(akshually::math::round_to(1.93333, 0.05), 1.95);
/// ```
pub fn round_to(value: f64, granularity: f64) -> f64 {
    let scale_factor = 1.0 / granularity;
    let scaled_up = value * scale_factor;
    let rounded = scaled_up.round();
    let scaled_down = rounded / scale_factor;
    scaled_down
}

#[cfg(test)]
mod tests {
    use crate::math::round_to;

    #[test]
    fn round_to_nickel() {
        let result = round_to(10.0 / 3.0, 0.05);
        assert_eq!(result, 3.35);
    }
}
