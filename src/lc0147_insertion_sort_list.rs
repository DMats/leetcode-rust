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
use crate::util::singly_linked_list::ListNode;

pub struct Solution;

/// Sort a linked list using insertion sort.
///
/// Algorithm of Insertion Sort:
///
/// 1.  Insertion sort iterates, consuming one input element each repetition,
///     and growing a sorted output list.
/// 2.  At each iteration, insertion sort removes one element from the input
///     data, finds the location it belongs within the sorted list, and inserts
///     it there.
/// 3.  It repeats until no input elements remain.
impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(node) => {
                // Move node into unsorted
                let mut unsorted = Some(node);
                // Create dummy node to act as start of sorted list
                let mut sorted = ListNode::new(std::i32::MIN);
                // Iterate over unsorted nodes
                while let Some(mut node_to_insert) = unsorted {
                    // Move node_to_insert.next into unsorted and replace it with None
                    unsorted = node_to_insert.next.take();
                    // Borrow the first sorted node
                    let mut sorted_ref = &mut sorted;
                    // Iterate over sorted nodes. Check if sorted.val < node_to_insert.val
                    while sorted_ref.next.is_some()
                        && sorted_ref.next.as_ref().unwrap().val < node_to_insert.val
                    {
                        // Move the sorted_ref to the next sorted node
                        sorted_ref = sorted_ref.next.as_mut().unwrap()
                    }
                    // Insert node_to_insert by:
                    //  1. Move sorted_ref.next to node_to_insert.next and replace it with None
                    node_to_insert.next = sorted_ref.next.take();
                    //  2. Move node_to_insert to sorted_ref.next
                    sorted_ref.next = Some(node_to_insert);
                }
                sorted.next
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sll;
    use crate::util::singly_linked_list::to_singly_linked_list;

    use super::*;

    #[test]
    fn simple() {
        assert_eq!(
            sll![1, 2, 3, 4],
            Solution::insertion_sort_list(sll![4, 2, 1, 3])
        );
    }

    #[test]
    fn negative_one() {
        assert_eq!(
            sll![-1, 0, 3, 4, 5],
            Solution::insertion_sort_list(sll![-1, 5, 3, 4, 0])
        );
    }

    #[test]
    fn none() {
        assert_eq!(None, Solution::insertion_sort_list(None));
    }
}
