use crate::utils::{singly_linked_list::ListNode, Solution};

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1,list2) {
            (None, None) => None,
            (None, Some(l2)) => Some(l2),
            (Some(l1), None) => Some(l1),
            (Some(l1), Some(l2)) => {
                if l1.val > l2.val {
                    Some(Box::new(
                        ListNode{
                            val:l2.val,
                            next:Self::merge_two_lists(Some(l1), l2.next)
                        }
                    ))
                }else {
                    Some(Box::new(
                        ListNode{
                            val:l1.val,
                            next:Self::merge_two_lists(l1.next, Some(l2))
                        }
                    ))
                }
            }
        }
    }
}


// 输入：l1 = [1,2,4], l2 = [1,3,4]
// 输出：[1,1,2,3,4,4]
//
//
#[test]
fn test_merge_two_lists() {
    let l1 = ListNode::from(vec![1,2,4]);
    let l2 = ListNode::from(vec![1,3,4]);
    let l3 = Solution::merge_two_lists(Some(Box::new(l1)), Some(Box::new(l2)));
    let l3 = *l3.unwrap();
    let res = vec![1,1,2,3,4,4];
    let v3 :Vec<i32> = l3.into();
    assert_eq!(res,v3);
}
