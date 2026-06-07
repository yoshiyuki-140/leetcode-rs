pub struct Solution;
use crate::common::ListNode;

// https://leetcode.com/problems/merge-two-sorted-lists/solutions/2947855/simple-and-efficient-rust-8-liner-by-ral-kso9/
#[cfg(any())]
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // リストの先頭のメモリ番地はlist1に残しておく
        let mut r = &mut list1;
        // 値がSomeかどうかを判定する
        while list2.is_some() {
            // rがNone, もしくはlist1の値の方が大きかった場合
            // list2とrの値を入れ替える
            if r.is_none() || list2.as_ref()?.val < r.as_ref()?.val {
                std::mem::swap(r, &mut list2);
            }
            // メモリ番地を次に更新
            r = &mut r.as_mut()?.next;
        }
        // list1の最初のメモリ番地を返却
        list1
    }
}

impl Solution {
    pub fn merge_two_lists(
        mut li1: Option<Box<ListNode>>,
        mut li2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // リストの先頭のメモリ番地はlist1に残しておく
        let mut r = &mut li1;
        // 値がSomeかどうかを判定する
        while li2.is_some() {
            // rがNone,もしくはli1の値の方が大きかった場合
            if r.is_none() || r.as_ref()?.val < r.as_ref()?.val {
                std::mem::swap(r, &mut li2);
            }
            // メモリ番地を次に更新
            r = &mut r.as_mut()?.next;
        }
        li1
    }
}
