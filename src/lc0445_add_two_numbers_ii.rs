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
        // Safe to unwrap because linked lists are guaranteed non-empty
        let l1_len = l1.as_ref().unwrap().len();
        let l2_len = l2.as_ref().unwrap().len();

        // Recursively add with longer list as first parameter
        if l1_len < l2_len {
            Solution::recursive_add(l1, l2, l1_len, l2_len)
        } else {
            Solution::recursive_add(l2, l1, l2_len, l1_len)
        }
    }

    fn recursive_add(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        l1_len: i32,
        l2_len: i32,
    ) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode::new(0)))
    }
}

impl ListNode {
    fn len(&self) -> i32 {
        let mut count = 1;
        let mut next = &self.next;
        while let Some(node) = next {
            count += 1;
            next = &node.next;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::sll;
    use crate::util::singly_linked_list::to_singly_linked_list;

    use super::*;

    #[test]
    fn test_7_243_plus_564() {
        assert_eq!(
            sll![7, 8, 0, 7],
            Solution::add_two_numbers(sll![7, 2, 4, 3], sll![5, 6, 4])
        );
    }

    #[test]
    fn test_0_plus_0() {
        assert_eq!(sll![0], Solution::add_two_numbers(sll![0], sll![0]))
    }

    #[test]
    fn test_3_999_999_999_plus_7() {
        assert_eq!(
            sll![4, 0, 0, 0, 0, 0, 0, 0, 0, 6],
            Solution::add_two_numbers(sll![3, 9, 9, 9, 9, 9, 9, 9, 9, 9], sll![7])
        )
    }

    #[test]
    fn test_9_999_999_999_999_plus_0() {
        assert_eq!(
            sll![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
            Solution::add_two_numbers(sll![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9], sll![0])
        )
    }
}
