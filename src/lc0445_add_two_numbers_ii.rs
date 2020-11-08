use crate::util::singly_linked_list::ListNode;

/// You are given two non-empty linked lists representing two non-negative
/// integers. The most significant digit comes first and each of their nodes
/// contain a single digit. Add the two numbers and return it as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except the
/// number 0 itself.
///
/// Follow up:
/// What if you cannot modify the input lists? In other words, reversing the
/// lists is not allowed.
pub struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
    }
}

#[cfg(test)]
mod tests {
    use crate::sll;
    use crate::util::singly_linked_list::to_singly_linked_list;

    use super::*;

    #[test]
    fn test_7243_plus_564() {
        assert_eq!(
            sll![7, 8, 0, 7],
            Solution::add_two_numbers(sll![7, 2, 4, 3], sll![5, 6, 4])
        );
    }
}
