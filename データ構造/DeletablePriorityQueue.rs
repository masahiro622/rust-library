use std::cmp::Ord;
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
pub struct DeletableBinaryHeap<T> {
    heap: BinaryHeap<T>,
    erased: BinaryHeap<T>,
}

impl<T: Ord> DeletableBinaryHeap<T> {
    /// 新しい空のヒープを作成します
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            erased: BinaryHeap::new(),
        }
    }

    /// 値を追加します
    pub fn push(&mut self, item: T) {
        self.heap.push(item);
    }

    /// 値の削除を予約します
    /// 注意: 実際にキュー内に存在する値を指定してください
    pub fn remove(&mut self, item: T) {
        self.erased.push(item);
    }

    /// 内部の整合性を取ります（削除予約された最大値を実際に排出する）
    fn clean(&mut self) {
        while let (Some(h_top), Some(e_top)) = (self.heap.peek(), self.erased.peek()) {
            if h_top == e_top {
                self.heap.pop();
                self.erased.pop();
            } else {
                break;
            }
        }
    }

    /// 最大値を取り出して返します
    pub fn pop(&mut self) -> Option<T> {
        self.clean();
        self.heap.pop()
    }

    /// 最大値への参照を返します
    /// 内部で clean() を呼ぶため、&mut self が必要であることに注意してください
    pub fn peek(&mut self) -> Option<&T> {
        self.clean();
        self.heap.peek()
    }

    /// 要素が空かどうかを判定します
    pub fn is_empty(&mut self) -> bool {
        self.clean();
        self.heap.is_empty()
    }

    /// 現在の有効な要素数を返します
    /// 注意: 存在しない要素を remove していない前提であれば正確です
    pub fn len(&self) -> usize {
        self.heap.len().saturating_sub(self.erased.len())
    }
}

// --- 使用例 ---
fn main() {
    let mut pq = DeletableBinaryHeap::new();

    pq.push(10);
    pq.push(30);
    pq.push(20);

    println!("Current max: {:?}", pq.peek()); // Some(30)

    // 30 を削除（即座には消えず、erasedに入る）
    pq.remove(30);

    // 次に最大値を見ようとした瞬間に、内部で30が掃除され、20が見える
    println!("After removing 30, max: {:?}", pq.peek()); // Some(20)

    pq.push(15);
    pq.remove(20);

    println!("After pushing 15 and removing 20, max: {:?}", pq.peek()); // Some(15)

    // 取り出し
    while let Some(val) = pq.pop() {
        println!("Popped: {}", val);
    }
}
