/// SWAG (Sliding Window Aggregation) を両端キューで実装したもの。
/// 両端キューの両端から要素を追加・削除でき、全体の積を O(1) で求められる。
/// fn new() -> Self: 空の両端キューを作る。
/// fn push_front(&mut self, value: M::S): 前に要素を追加する。
/// fn push_back(&mut self, value: M::S): 後ろに要素を追加する。
/// fn pop_front(&mut self) -> Option<M::S>: 前から要素を削除して返す。両端キューが空のときは None を返す。
/// fn pop_back(&mut self) -> Option<M::S>: 後ろから要素を削除して返す。両端キューが空のときは None を返す。
/// fn product(&self) -> M::S: 全体の積を返す。両端キューが空のときは単位元を返す。
pub struct SwagDeque<M: Monoid> {
    left: SwagStack<M>,
    right: SwagStack<M>,
}

impl<M: Monoid> Default for SwagDeque<M> {
    fn default() -> Self {
        Self {
            left: SwagStack::new(),
            right: SwagStack::new(),
        }
    }
}

impl<M: Monoid> SwagDeque<M> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_front(&mut self, value: M::S) {
        self.left.push(value);
    }

    pub fn push_back(&mut self, value: M::S) {
        self.right.push(value);
    }

    fn rebalance_to_left(&mut self) {
        let values = self.right.values_bottom_to_top();
        let left_size = (values.len() + 1) / 2;

        self.left.clear();
        self.right.clear();

        for value in values[..left_size].iter().rev() {
            self.left.push(value.clone());
        }
        for value in values[left_size..].iter() {
            self.right.push(value.clone());
        }
    }

    fn rebalance_to_right(&mut self) {
        let values = self.left.values_top_to_bottom();
        let left_size = values.len() / 2;

        self.left.clear();
        self.right.clear();

        for value in values[..left_size].iter().rev() {
            self.left.push(value.clone());
        }
        for value in values[left_size..].iter() {
            self.right.push(value.clone());
        }
    }

    pub fn pop_front(&mut self) -> Option<M::S> {
        if self.left.is_empty() {
            if self.right.is_empty() {
                return None;
            }
            self.rebalance_to_left();
        }
        self.left.pop()
    }

    pub fn pop_back(&mut self) -> Option<M::S> {
        if self.right.is_empty() {
            if self.left.is_empty() {
                return None;
            }
            self.rebalance_to_right();
        }
        self.right.pop()
    }

    pub fn product(&self) -> M::S {
        match (
            self.left.agg_top_to_bottom(),
            self.right.agg_bottom_to_top(),
        ) {
            (Some(left_agg), Some(right_agg)) => M::binary_operation(&left_agg, &right_agg),
            (Some(left_agg), None) => left_agg,
            (None, Some(right_agg)) => right_agg,
            (None, None) => M::identity(),
        }
    }
}

struct Entry<T> {
    value: T,
    agg_bottom_to_top: T,
    agg_top_to_bottom: T,
}

struct SwagStack<M: Monoid> {
    data: Vec<Entry<M::S>>,
    _marker: std::marker::PhantomData<M>,
}

impl<M: Monoid> SwagStack<M> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push(&mut self, value: M::S) {
        let (agg_bottom_to_top, agg_top_to_bottom) = if let Some(last) = self.data.last() {
            (
                M::binary_operation(&last.agg_bottom_to_top, &value),
                M::binary_operation(&value, &last.agg_top_to_bottom),
            )
        } else {
            (value.clone(), value.clone())
        };
        self.data.push(Entry {
            value,
            agg_bottom_to_top,
            agg_top_to_bottom,
        });
    }

    fn pop(&mut self) -> Option<M::S> {
        self.data.pop().map(|entry| entry.value)
    }

    fn agg_bottom_to_top(&self) -> Option<M::S> {
        self.data
            .last()
            .map(|entry| entry.agg_bottom_to_top.clone())
    }

    fn agg_top_to_bottom(&self) -> Option<M::S> {
        self.data
            .last()
            .map(|entry| entry.agg_top_to_bottom.clone())
    }

    fn values_bottom_to_top(&self) -> Vec<M::S> {
        self.data.iter().map(|entry| entry.value.clone()).collect()
    }

    fn values_top_to_bottom(&self) -> Vec<M::S> {
        self.data
            .iter()
            .rev()
            .map(|entry| entry.value.clone())
            .collect()
    }
}
