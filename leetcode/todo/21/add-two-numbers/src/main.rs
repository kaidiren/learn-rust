#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub fn middle_node(head: &Option<Box<ListNode>>) -> &Option<Box<ListNode>> {
        let mut slow = head;
        let mut fast = head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &(slow.as_ref().unwrap().next);
            fast = &(fast.as_ref().unwrap().next.as_ref().unwrap().next);
        }
        slow
    }
}

fn arr_to_list(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for n in arr {
        let mut new_node = ListNode::new(*n);
        new_node.next = head;
        head = Some(Box::new(new_node));
    }
    head
}

fn main() {
    let list1 = arr_to_list(&[2, 4, 3]);
    let list2 = arr_to_list(&[5, 6, 4]);

    let mut fast = list1;
    println!("{:?}", fast);
    //    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
    //        println!("{:?}", fast.unwrap().val);
    //        fast = fast.as_ref().unwrap().next.as_ref().unwrap().next;
    //    }
}
