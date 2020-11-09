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
        let (sum, carry) = Solution::recursive_add(l1, l2, l1_len, l2_len);
        if carry == 1 {
            let mut sum_head = ListNode::new(1);
            sum_head.next = sum;
            Some(Box::new(sum_head))
        } else {
            sum
        }
    }

    fn recursive_add(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        l1_len: i32,
        l2_len: i32,
    ) -> (Option<Box<ListNode>>, i32) {
        use std::cmp::Ordering::{Equal, Greater, Less};
        match l1_len.cmp(&l2_len) {
            // l1 is shorter than l2 so move to the next node in l2 and recurse
            Less => {
                let l1_val = 0;
                let l2_val = l2.as_ref().unwrap().val;
                let (sum_of_lower_digits, carry_of_lower_digits) =
                    Solution::recursive_add(l1, l2.unwrap().next, l1_len, l2_len - 1);
                let (sum, carry) = Solution::add_and_prepend_digit(
                    sum_of_lower_digits,
                    carry_of_lower_digits,
                    l1_val,
                    l2_val,
                );
                (sum, carry)
            }
            // l2 is shorter than l1 so move to the next node in l1 and recurse
            Greater => {
                let l1_val = l1.as_ref().unwrap().val;
                let l2_val = 0;
                let (sum_of_lower_digits, carry_of_lower_digits) =
                    Solution::recursive_add(l1.unwrap().next, l2, l1_len - 1, l2_len);
                let (sum, carry) = Solution::add_and_prepend_digit(
                    sum_of_lower_digits,
                    carry_of_lower_digits,
                    l1_val,
                    l2_val,
                );
                (sum, carry)
            }
            // l1 and l2 are the same length
            Equal => {
                let l1_val = l1.as_ref().unwrap().val;
                let l2_val = l2.as_ref().unwrap().val;
                if l1_len == 1 {
                    // This is the least significant digit position so add the
                    // digits and create the first sum node
                    let (sum, carry) = Solution::add_digits_and_carry(l1_val, l2_val, 0);
                    (Some(Box::new(ListNode::new(sum))), carry)
                } else {
                    // This is not the least significant digith position so
                    // move to the next node in both l1 and l2 and recurse
                    let (sum_of_lower_digits, carry_of_lower_digits) = Solution::recursive_add(
                        l1.unwrap().next,
                        l2.unwrap().next,
                        l1_len - 1,
                        l2_len - 1,
                    );
                    let (sum, carry) = Solution::add_and_prepend_digit(
                        sum_of_lower_digits,
                        carry_of_lower_digits,
                        l1_val,
                        l2_val,
                    );
                    (sum, carry)
                }
            }
        }
    }

    /// Add two digits plus a carry. Prepend the resulting digit to the sum of
    /// the lower digits. Return the new sum head and the new carry.
    fn add_and_prepend_digit(
        sum_of_lower_digits: Option<Box<ListNode>>,
        carry_of_lower_digits: i32,
        digit1: i32,
        digit2: i32,
    ) -> (Option<Box<ListNode>>, i32) {
        let (sum, carry) = Solution::add_digits_and_carry(digit1, digit2, carry_of_lower_digits);
        let mut sum_head = ListNode::new(sum);
        sum_head.next = sum_of_lower_digits;
        (Some(Box::new(sum_head)), carry)
    }

    /// Add two digits plus a carry. Both digits must be less than or equal to 9
    /// Returns the resulting digit and new carry which will be either 0 or 1.
    fn add_digits_and_carry(digit1: i32, digit2: i32, carry: i32) -> (i32, i32) {
        let sum = digit1 + digit2 + carry;
        let new_carry = sum / 10;
        let digit = if new_carry == 0 { sum } else { sum % 10 };
        (digit, new_carry)
    }
}

impl ListNode {
    /// Calculate length of list
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

    #[test]
    fn test_5_plus_5() {
        assert_eq!(sll![1, 0], Solution::add_two_numbers(sll![5], sll![5]))
    }
}
