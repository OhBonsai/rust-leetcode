use super::*;

struct Solution();
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode{
            val: val-1,
            next: head
        }));

        let mut node = &mut head;

        'outer:
        while let Some(ptr) = node {
            while let Some(ptr2) = &mut ptr.next {
                if ptr2.val == val {
                    ptr.next = ptr2.next.take();
                } else {
                    node = &mut ptr.next;
                    continue 'outer;
                }
            }
            break;
        }

        head.unwrap().next
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn common_test() {
        let mut v = linklist!(1,2,3, 3);
        println!("{:?}", Solution::remove_elements(v, 3))

    }


}
