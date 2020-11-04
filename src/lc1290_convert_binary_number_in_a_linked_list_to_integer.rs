use crate::util::singly_linked_list::ListNode;

pub struct Solution;

/// Given head which is a reference node to a singly-linked list. The value of
/// each node in the linked list is either 0 or 1. The linked list holds the
/// binary representation of a number. Return the decimal value of the number in
/// the linked list.
impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut decimal = 0;
        match head {
            None => return 0,
            Some(node) => {
                decimal = (*node).val;
                let mut curr_node = (*node).next;
                for _i in 0..30 {
                    match curr_node {
                        None => return decimal,
                        Some(next_node) => {
                            decimal = (decimal * 2) + (*next_node).val;
                            curr_node = (*next_node).next;
                        }
                    }
                }
                return decimal;
            }
        }
    }
}
