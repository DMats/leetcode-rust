/// Given an array of integers `nums` and an integer `threshold`, we will choose
/// a positive integer divisor and divide all the array by it and sum the result
/// of the division. Find the **smallest** divisor such that the result
/// mentioned above is less than or equal to `threshold`.
///
/// Each result of division is rounded to the nearest integer greater than or
/// equal to that element. (For example: 7/3 = 3 and 10/2 = 5).
///
/// It is guaranteed that there will be an answer.
///
/// Constraints:
///
/// * 1 <= nums.length <= 5 * 10^4
/// * 1 <= nums[i] <= 10^6
/// * nums.length <= threshold <= 10^6
pub struct Solution;
impl Solution {
    #[allow(unused_variables)]
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_two_five_nine_thresh_six() {
        assert_eq!(5, Solution::smallest_divisor(vec![1, 2, 5, 9], 6));
    }

    #[test]
    fn two_three_five_seven_eleven_thresh_eleven() {
        assert_eq!(3, Solution::smallest_divisor(vec![2, 3, 5, 7, 11], 11));
    }

    #[test]
    fn nineteen_thresh_five() {
        assert_eq!(4, Solution::smallest_divisor(vec![19], 5));
    }
}
