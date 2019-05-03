/// # [Add Two Numbers](https://leetcode.com/problems/add-two-numbers/)
///
/// You are given two non-empty linked lists representing two non-negative
/// integers. The digits are stored in reverse order and each of their nodes
/// contain a single digit. Add the two numbers and return it as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except
/// the number 0 itself.
mod add_two_numbers {
    struct Solution;

    // Definition for singly-linked list.
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }

    impl Solution {
        pub fn add_two_numbers(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            Solution::add_two_numbers_with_div(&l1, &l2, 0)
        }

        fn add_two_numbers_with_div(
            l1: &Option<Box<ListNode>>,
            l2: &Option<Box<ListNode>>,
            div: i32,
        ) -> Option<Box<ListNode>> {
            match (&l1, &l2) {
                (Some(d1), Some(d2)) => {
                    let (div, rem) = Solution::div_rem(d1.val + d2.val + div);
                    let mut result = Box::new(ListNode::new(rem));
                    result.next = Solution::add_two_numbers_with_div(
                        &d1.next, &d2.next, div,
                    );
                    Some(result)
                }
                (None, Some(d2)) => {
                    let (div, rem) = Solution::div_rem(d2.val + div);
                    let mut result = Box::new(ListNode::new(rem));
                    result.next = Solution::add_two_numbers_with_div(
                        &None, &d2.next, div,
                    );
                    Some(result)
                }
                (Some(d1), None) => {
                    let (div, rem) = Solution::div_rem(d1.val + div);
                    let mut result = Box::new(ListNode::new(rem));
                    result.next = Solution::add_two_numbers_with_div(
                        &d1.next, &None, div,
                    );
                    Some(result)
                }
                (None, None) => {
                    if div == 0 {
                        None
                    } else {
                        Some(Box::new(ListNode::new(div)))
                    }
                }
            }
        }

        fn div_rem(number: i32) -> (i32, i32) {
            let div = (number as f32 / 10.0).trunc() as i32;
            let rem = number - div * 10;
            (div, rem)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test() {
            let l1 = Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            }));

            let l2 = Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            }));

            let res = Solution::add_two_numbers(l1, l2).unwrap();
            let expected = Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 8, next: None })),
                })),
            });

            assert_eq!(expected, res);
        }
    }
}
