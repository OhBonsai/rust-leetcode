use super::*;

struct Solution ();
impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.as_ref().unwrap();
        let mut slow = head.as_ref();

        if fast.next.is_none() { return head; }

        while !fast.next.is_none() && !fast.next.as_ref().unwrap().next.is_none() {
            fast = fast.next.as_ref().unwrap().next.as_ref().unwrap();
            if fast.next.is_none() {
                return slow.unwrap().next.clone();
            }
            slow = slow.unwrap().next.as_ref();

        }
        slow.unwrap().next.clone()


    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test] fn test_1() {
      assert_eq!(Solution::middle_node(linklist!(1,2,3,4,5)).unwrap().val, 3);
      assert_eq!(Solution::middle_node(linklist!(1,2,3,4,5, 6)).unwrap().val, 4);


  }
}