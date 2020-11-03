pub struct Solution;

/// Given a string s, the power of the string is the maximum length of a
/// non-empty substring that contains only one unique character. Return the
/// power of the string.
impl Solution {
    pub fn max_power(s: String) -> i32 {
        // Pseudocode
        // - Return 0 if empty string
        // - track max_power, current_power, and current_char
        // - iterate over chars in string
        //   - if c != current_char, update current_char, reset current_power
        //   - else increment current power
        //     - if current_power > max_power, max_power = current_power
        if s.is_empty() {
            return 0;
        }
        let mut max_power = 1;
        dbg!(max_power);
        let mut current_power = 1;
        dbg!(current_power);
        let mut current_char: Option<char> = None;
        dbg!(current_char);
        for c in s.chars() {
            match current_char {
                None => {
                    current_char = Some(c);
                }
                _ => {
                    if c == current_char.unwrap() {
                        current_power += 1;
                        dbg!(current_power);
                        if current_power > max_power {
                            max_power = current_power;
                            dbg!(max_power);
                        }
                    } else {
                        current_char = Some(c);
                        dbg!(current_char);
                        current_power = 1;
                        dbg!(current_power);
                    }
                }
            }
        }
        max_power
    }
}
