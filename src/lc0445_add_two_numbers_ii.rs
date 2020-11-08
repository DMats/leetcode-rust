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
        let decimal_1 = Solution::digits_list_to_decimal(l1);
        let decimal_2 = Solution::digits_list_to_decimal(l2);
        let sum = decimal_1 + decimal_2;
        Solution::decimal_to_digits_list(sum)
    }

    pub fn digits_list_to_decimal(digits: Option<Box<ListNode>>) -> i32 {
        let mut decimal = 0;
        let mut digits = digits;
        while let Some(digit) = digits {
            decimal = (decimal * 10) + digit.val;
            digits = digit.next;
        }
        decimal
    }

    pub fn decimal_to_digits_list(decimal: i32) -> Option<Box<ListNode>> {
        if decimal == 0 {
            return Some(Box::new(ListNode::new(0)));
        }
        let mut decimal = decimal;
        let mut prev_node = ListNode::new(0);
        let mut curr_node = Some(Box::new(ListNode::new(0)));
        while decimal > 0 {
            let digit = decimal % 10;
            decimal = (decimal - digit) / 10;
            curr_node.as_mut().unwrap().val = digit;
            prev_node.next = curr_node;
            curr_node = Some(Box::new(prev_node));
            prev_node = ListNode::new(0);
        }
        curr_node.unwrap().next
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

    #[test]
    fn test_0_plus_0() {
        assert_eq!(sll![0], Solution::add_two_numbers(sll![0], sll![0]))
    }
}
