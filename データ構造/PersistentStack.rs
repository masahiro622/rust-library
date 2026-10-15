use std::rc::Rc;

/// 永続スタックの本体
#[derive(Debug, Clone)]
pub struct PersistentStack<T> {
    head: Option<Rc<Node<T>>>,
}

/// スタックを構成するノード
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> PersistentStack<T> {
    /// 空のスタックを作成する
    pub fn new() -> Self {
        PersistentStack { head: None }
    }

    /// 新しい要素を追加し、新しいスタックを返す
    /// （元のスタックは変更されない）
    pub fn push(&self, value: T) -> Self {
        let new_node = Node {
            value,
            next: self.head.clone(), // 既存のノードへの参照（Rc）をクローンして共有
        };
        PersistentStack {
            head: Some(Rc::new(new_node)),
        }
    }

    /// 先頭要素を取り除いた新しいスタックを返す
    /// （元のスタックは変更されない）
    pub fn pop(&self) -> Self {
        PersistentStack {
            // headが存在すれば、その次のノードへの参照を新しいheadにする
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    /// 先頭要素への参照を取得する
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    /// スタックが空かどうかを判定する
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

impl<T> Default for PersistentStack<T> {
    fn default() -> Self {
        Self::new()
    }
}
