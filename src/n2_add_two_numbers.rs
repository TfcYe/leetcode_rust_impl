use super::infrastructure::linked_list::{to_list, ListNode};
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut ref_head = &mut head;
        let mut carry = 0;

        let mut l1 = l1.as_ref();
        let mut l2 = l2.as_ref();
        while l1.is_some() || l2.is_some() {
            let (x, y) = (l1.map(|n| n.val), l2.map(|n| n.val));
            let (x, y) = (x.unwrap_or(0), y.unwrap_or(0));

            let mut sum = x + y + carry;
            carry = 0;
            if sum >= 10 {
                sum -= 10;
                carry = 1;
            }

            ref_head.next = Some(Box::new(ListNode::new(sum)));
            ref_head = ref_head.next.as_mut().unwrap();

            l1 = l1.map(|n| n.next.as_ref()).unwrap_or(None);
            l2 = l2.map(|n| n.next.as_ref()).unwrap_or(None);
        }

        if carry > 0 {
            ref_head.next = Some(Box::new(ListNode::new(1)));
        }

        head.next
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}
