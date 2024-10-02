// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 多分、加算器の実装をすればいいと思う
        // 桁上りに注意
        let mut keta_agari = 0;

        let mut l1 = l1;
        let mut l2 = l2;

        let mut result: Option<Box<ListNode>> = None;

        let mut current_node = &mut result;

        loop {
            // それぞれのリストの最下位の桁を取り出す
            let l1_val = match &l1 {
                Some(node) => node.val,
                None => 0,
            };
            let l2_val = match &l2 {
                Some(node) => node.val,
                None => 0,
            };

            // それぞれのリストの最下位の桁を足す
            let sum_over10 = l1_val + l2_val + keta_agari;

            // もしゼロなら終了
            // TODO: ここの条件が微妙な気がする
            if sum_over10 == 0 && l1.is_none() && l2.is_none() {
                return result;
            }

            // 桁上りがあれば次の桁に持ち越す
            let sum = sum_over10 % 10;
            keta_agari = sum_over10 / 10;

            // 結果のリストに足す
            current_node = match current_node {
                Some(node) => {
                    node.next = Some(Box::new(ListNode::new(sum)));
                    &mut node.next
                }
                None => {
                    result = Some(Box::new(ListNode::new(sum)));
                    &mut result
                }
            };

            // それぞれのリストの次の桁があればそれを取り出す
            l1 = match &l1 {
                Some(node) => node.next.clone(),
                None => None,
            };
            l2 = match &l2 {
                Some(node) => node.next.clone(),
                None => None,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(ListNode { val: 0, next: None }));
        let l2 = Some(Box::new(ListNode { val: 0, next: None }));
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, Some(Box::new(ListNode { val: 0, next: None })));
    }
}
