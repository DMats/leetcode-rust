/*
 * Copyright 2020 David Hunt-Mateo (DMats)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut min_cost = std::i32::MAX;
        for (_, destination) in position.iter().enumerate() {
            let mut total_cost = 0;
            for (_, chip_position) in position.iter().enumerate() {
                let distance_to_destination = (*chip_position - *destination).abs();
                let cost = distance_to_destination % 2;
                total_cost += cost;
            }
            min_cost = std::cmp::min(min_cost, total_cost);
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

    #[test]
    fn eleven() {
        assert_eq!(0, Solution::min_cost_to_move_chips(vec![11]));
    }
}
