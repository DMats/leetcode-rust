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
    use super::*;

    #[test]
    fn leetcode() {
        assert_eq!(2, Solution::max_power(String::from("leetcode")));
    }

    #[test]
    fn abbcccddddeeeeedcba() {
        assert_eq!(5, Solution::max_power(String::from("abbcccddddeeeeedcba")));
    }

    #[test]
    fn triplepillooooow() {
        assert_eq!(5, Solution::max_power(String::from("triplepillooooow")));
    }

    #[test]
    fn hooraaaaaaaaaaay() {
        assert_eq!(11, Solution::max_power(String::from("hooraaaaaaaaaaay")));
    }

    #[test]
    fn tourist() {
        assert_eq!(1, Solution::max_power(String::from("tourist")));
    }

    #[test]
    fn empty_string() {
        assert_eq!(0, Solution::max_power(String::from("")));
    }

    #[test]
    fn five_hundred() {
        assert_eq!(
            500,
            Solution::max_power(String::from(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaa"
            ))
        );
    }

    #[test]
    fn five_hundred_and_one() {
        assert_eq!(
            501,
            Solution::max_power(String::from(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
                aaaa"
            ))
        );
    }
}
