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

/// Search in Rotated Sorted Array II
///
/// Suppose an array sorted in ascending order is rotated at some pivot unknown
/// to you beforehand.
///
/// (i.e., [0,0,1,2,2,5,6] might become [2,5,6,0,0,1,2]).
///
/// You are given a target value to search. If found in the array return true,
/// otherwise return false.
pub struct Solution;
impl Solution {
    #[allow(unused_variables)]
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        true
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
