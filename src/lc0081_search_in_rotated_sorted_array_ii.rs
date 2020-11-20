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

pub struct Solution;
impl Solution {
    /// Search in Rotated Sorted Array II
    ///
    /// Suppose an array sorted in ascending order is rotated at some pivot
    /// unknown to you beforehand.
    ///
    /// (i.e., [0,0,1,2,2,5,6] might become [2,5,6,0,0,1,2]).
    ///
    /// You are given a target value to search. If found in the array return
    /// true, otherwise return false.
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let nums_len = nums.len();
        if nums_len == 0 {
            return false;
        }
        let mut end = nums_len - 1;
        let mut start: usize = 0;

        while start <= end {
            let mid = start + (end - start) / 2;

            if nums[mid] == target {
                return true;
            }

            if !Solution::is_binary_search_helpful(&nums, start, nums[mid]) {
                start += 1;
                continue;
            }

            // Which array does pivot belong to?
            let pivot_array = Solution::exists_in_first(&nums, start, nums[mid]);

            // Which array does target belong to?
            let target_array = Solution::exists_in_first(&nums, start, target);

            // XOR ^ is true when operands are distinct.
            if pivot_array ^ target_array {
                // Pivot and target exist in different sorted arrays
                if pivot_array {
                    // Pivot in first, target in second
                    start = mid + 1;
                } else {
                    // Target in first, pivot in second
                    end = mid - 1;
                }
            } else {
                // Pivot and target exist in same sorted array
                if nums[mid] < target {
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
        }
        false
    }

    /// returns true if element exists in first array, false if it exists in
    /// second
    fn exists_in_first(nums: &Vec<i32>, start: usize, element: i32) -> bool {
        nums[start] <= element
    }

    /// returns true if we can reduce the search space in current binary search
    /// space
    fn is_binary_search_helpful(nums: &Vec<i32>, start: usize, element: i32) -> bool {
        nums[start] != element
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }
}
