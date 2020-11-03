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
        //     - if current_power > max_power, max_power = current_power what
    }
}
