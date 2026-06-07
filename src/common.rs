// 共通で利用するデータ構造などを定義するファイル

// singly-linked list
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(unused)]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}
