/// We have n chips, where the position of the ith chip is position[i].
///
/// We need to move all the chips to the same position. In one step, we can
/// change the position of the ith chip from position[i] to:
///
/// * position[i] + 2 or position[i] - 2 with cost = 0.
/// * position[i] + 1 or position[i] - 1 with cost = 1.
///
/// Return the minimum cost needed to move all the chips to the same position.
///
/// Constraints:
///
/// * 1 <= position.length <= 100
/// * 1 <= position[i] <= 10^9
pub struct Solution;
impl Solution {
    #[allow(unused_variables)]
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_distribution() {
        assert_eq!(1, Solution::min_cost_to_move_chips(vec![1, 2, 3]));
    }

    #[test]
    fn center_bias() {
        assert_eq!(2, Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]));
    }

    #[test]
    fn large_split() {
        assert_eq!(1, Solution::min_cost_to_move_chips(vec![1, 1000000000]));
    }
}
