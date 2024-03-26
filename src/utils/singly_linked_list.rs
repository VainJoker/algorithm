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

impl From<Vec<i32>> for ListNode {
    fn from(value: Vec<i32>) -> Self {
        let mut dummy = ListNode::new(0);
        let ptr = &mut dummy;

        for &num in value.iter().rev() {
            let mut new_node = ListNode::new(num);
            new_node.next = ptr.next.take();
            ptr.next = Some(Box::new(new_node));
        }

        *dummy.next.unwrap().clone()
    }
}

impl Into<Vec<i32>> for ListNode {
    fn into(self) -> Vec<i32> {
        let mut value = Vec::new();
        let mut node = self.clone();
        while node.next.is_some() {
            value.push(node.val);
            node = *node.next.unwrap()
        }
        value.push(node.val);
        value
    }
}

fn helper(mut value: Vec<i32>, mut node:Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let elem = ListNode::new(value.remove(0));
    if let Some(ref mut n) = node {
        n.next = match value.is_empty() {
            true => Some(Box::new(elem)),
            false => helper(value, Some(Box::new(elem))),
        };
    }else {
        node = helper(value, Some(Box::new(elem)));
    }
    node
}

// fn helper2(value: Vec<i32>) -> Option<Box<ListNode>> {
    // let mut e0 = ListNode::new(value[0]);
    // let mut e1 = ListNode::new(value[1]);
    // let mut e2 = ListNode::new(value[2]);
    // // e2.next = None;
    // e1.next = Some(Box::new(e2));
    // e0.next = Some(Box::new(e1));
    // eprintln!("{:#?}",e0);
    // return Some(Box::new(e0));
// }

#[test]
fn test_helper() {
    let vec = vec![0,1,2,3,4,5];
    // let node = None;
    // let linked = helper(vec, node);
    let linked = ListNode::from(vec);
    eprintln!("{:#?}",linked);
    let v2 :Vec<i32> = linked.into();
    eprintln!("{:#?}",v2);
}
