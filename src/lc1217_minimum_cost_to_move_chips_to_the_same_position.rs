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
///
/// Pseudocode:
///
/// - Build destinations vector
///   - destinations = empty vector of size (max value in position)
///   - for (chip_id, chip_position) in position
///     - slot[chip_position]++
///
/// - Calculate min cost
///   - min_cost = MAX
///   - for destination in position
///     - total_cost = 0
///     - for (current_location, chip_count) in position
///       - cost_to_move_one_chip = abs(current_location - destination) % 2
///       - cost_to_move_multiple_chips = chip_count * cost_to_move_one_chip
///       - total_cost += cost_to_move_multiple_chips
///     - min_cost = min(min_cost, total_cost)
///   - return min_cost
pub struct Solution;
impl Solution {
    #[allow(unused_variables)]
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        // Build destinations vector
        let max_position = *position.iter().max().unwrap() as usize;
        let mut destinations = vec![0; max_position];
        for (chip_id, chip_position) in position.iter().enumerate() {
            let chip_index = (*chip_position - 1) as usize;
            destinations[chip_index] += 1;
        }

        // Calculate min cost
        let mut min_cost = std::i32::MAX;
        dbg!(min_cost);
        for (destination, _) in destinations.iter().enumerate() {
            dbg!(destination);
            let mut total_cost = 0;
            dbg!(total_cost);
            for (current_location, chip_count) in position.iter().enumerate() {
                dbg!(current_location);
                dbg!(chip_count);
                let (distance_to_destination, overflow_occurred) =
                    current_location.overflowing_sub(destination);
                dbg!(distance_to_destination);
                dbg!(overflow_occurred);
                let cost_to_move_one_chip = (match overflow_occurred {
                    // If overflow occurred, reverse the subtraction
                    true => destination - current_location,
                    false => distance_to_destination,
                } % 2) as i32;
                dbg!(cost_to_move_one_chip);
                let cost_to_move_multiple_chips = chip_count * cost_to_move_one_chip;
                dbg!(cost_to_move_multiple_chips);
                total_cost += cost_to_move_multiple_chips;
                dbg!(total_cost);
            }
            min_cost = std::cmp::min(min_cost, total_cost);
            dbg!(min_cost);
        }
        min_cost
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
