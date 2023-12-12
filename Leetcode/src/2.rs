// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sum = 0;
        let mut carry = 0;
        let mut l3 = Some(Box::new(ListNode { val: 0, next: None }));
        let mut head = l3.as_mut();

        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());

        while l1.is_some() || l2.is_some() {
            sum = 0;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next.as_ref();
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next.as_ref();
            }
            sum += carry;
            carry = sum / 10;
            sum %= 10;
            head.as_mut().unwrap().next = Some(Box::new(ListNode { val: sum, next: None }));
            head = head.unwrap().next.as_mut();
        }

        if carry != 0 {
            head.as_mut().unwrap().next = Some(Box::new(ListNode { val: carry, next: None }));
        }

        l3.unwrap().next
    }
}