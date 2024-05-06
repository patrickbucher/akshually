use std::time::Duration;

/// Formats `duration` as a compact string, indicating days, hours, minutes, seconds, and
/// milliseconds.
///
/// # Examples
///
/// ```
/// let duration = std::time::Duration::from_secs(3600 + 300 + 12);
/// assert_eq!(akshually::date::format(duration), "1h5m12s");
/// ```
pub fn format(duration: Duration) -> String {
    let units_in_millis = [
        ("d", 1000 * 60 * 60 * 24),
        ("h", 1000 * 60 * 60),
        ("m", 1000 * 60),
        ("s", 1000),
        ("ms", 1),
    ];
    let mut results: Vec<String> = Vec::new();
    let mut remainder = duration.as_millis();
    for (u, ms) in units_in_millis {
        let n = remainder / ms;
        if n > 0 {
            remainder -= n * ms;
            results.push(format!("{}{}", n, u));
        }
    }
    results
        .iter()
        .fold("".to_string(), |acc: String, v: &String| {
            format!("{acc}{v}")
        })
}

#[cfg(test)]
mod tests {
    use crate::date::*;

    #[test]
    fn test_format() {
        assert_eq!(format(Duration::from_secs(59)), "59s");
        assert_eq!(format(Duration::from_secs(61)), "1m1s");
        assert_eq!(format(Duration::from_secs(121)), "2m1s");
        assert_eq!(format(Duration::from_secs(3600 + 1)), "1h1s");
        assert_eq!(format(Duration::from_secs(24 * 3600 + 1)), "1d1s");
    }
}
