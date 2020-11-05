use crate::util::singly_linked_list::ListNode;

pub struct Solution;

/// Given head which is a reference node to a singly-linked list. The value of
/// each node in the linked list is either 0 or 1. The linked list holds the
/// binary representation of a number. Return the decimal value of the number in
/// the linked list.
impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut decimal;
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

#[cfg(test)]
mod tests {
    use crate::sll;
    use crate::util::singly_linked_list::to_singly_linked_list;

    use super::*;

    #[test]
    fn five() {
        assert_eq!(5, Solution::get_decimal_value(sll![1, 0, 1,]));
    }

    #[test]
    fn zero() {
        assert_eq!(0, Solution::get_decimal_value(sll![0]));
    }

    #[test]
    fn one() {
        assert_eq!(1, Solution::get_decimal_value(sll![1]));
    }

    #[test]
    fn eighteen_thousand_eight_hundred_eighty() {
        assert_eq!(
            18_880,
            Solution::get_decimal_value(sll![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0])
        );
    }

    #[test]
    fn double_zero() {
        assert_eq!(0, Solution::get_decimal_value(sll![0, 0]));
    }
}
