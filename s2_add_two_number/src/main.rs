// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        println!("l1: {:?}", l1);
        println!("l2: {:?}", l2);

        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut carry: i32 = 0;
        let mut result: Option<Box<ListNode>> = None;
        let mut cur = &mut result;

        while l1.is_some() || l2.is_some() || carry != 0{
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = &node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = &node.next;
            }

            carry = sum / 10;
            *cur = Some(Box::new(ListNode::new(sum % 10)));
            cur = &mut cur.as_mut().unwrap().next;
        }

        result
    }
}

fn convert_vec_to_nodes(vector: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut nodes = None;

    for ele in vector.iter().rev() {
        let mut node = Box::new(ListNode::new(*ele));
        node.next = nodes;
        nodes = Some(node);
    }

    nodes
}

fn main() {
    let l1 = convert_vec_to_nodes(&vec![2,4,3]);
    let l2 = convert_vec_to_nodes(&vec![5,6,4]);

    let result = Solution::add_two_numbers(l1, l2);

    println!("{:?}", result);
}