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
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        }

        let mut p = head.as_mut().unwrap();
        while let Some(next) = p.next.take() {
            let gcd = gcd(p.val, next.val);
            p.next = Some(Box::new(ListNode::new(gcd)));
            p.next.as_mut().unwrap().next = Some(next);
            p = p.next.as_mut().unwrap().next.as_mut().unwrap();
        }
        head
    }
}