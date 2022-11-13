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
        let mut root = Box::new(ListNode::new(0));
        let mut n = &mut root;
        let mut n1 = l1;
        let mut n2 = l2;
        let mut val = 0;
        while n1 != None || n2 != None {
            val /= 10;
            if n1 != None {
                val += n1.as_ref().unwrap().val;
                n1 = n1.unwrap().next;
            }
            if n2 != None {
                val += n2.as_ref().unwrap().val;
                n2 = n2.unwrap().next;
            }
            n.next = Some(Box::new(ListNode::new(val % 10)));
            n = n.next.as_mut().unwrap();
        }
        if val / 10 == 1 {
            n.next = Some(Box::new(ListNode::new(1)));
        }
        root.next
    }
}