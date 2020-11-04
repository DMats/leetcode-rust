pub struct Solution;

/// Given a string s, the power of the string is the maximum length of a
/// non-empty substring that contains only one unique character. Return the
/// power of the string.
///
/// Pseudocode
/// - Return 0 if empty string
/// - track max_power, current_power, and current_char
/// - iterate over chars in string
///   - if c == current_char,
///     - increment current power
///     - if current_power > max_power
///       - max_power = current_power
///   - else
///     - update current_char
///     - reset current_power
impl Solution {
    pub fn max_power(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mut max_power = 1;
        let mut current_power = 1;
        let mut current_char: Option<char> = None;
        for c in s.chars() {
            match current_char {
                None => {
                    current_char = Some(c);
                }
                _ => {
                    if c == current_char.unwrap() {
                        current_power += 1;
                        if current_power > max_power {
                            max_power = current_power;
                        }
                    } else {
                        current_char = Some(c);
                        current_power = 1;
                    }
                }
            }
        }
        max_power
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
