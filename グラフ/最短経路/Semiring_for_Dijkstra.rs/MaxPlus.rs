#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct MaxWeight(pub i64);

impl PathWeight for MaxWeight {
    fn start() -> Self {
        MaxWeight(0)
    }
    fn infinity() -> Self {
        MaxWeight(i64::MIN)
    }
    fn accumulate(self, edge: Self) -> Self {
        // 負の無限大との加算は負の無限大のままにする処理
        if self.0 == i64::MIN {
            self
        } else {
            MaxWeight(self.0 + edge.0)
        }
    }
    fn priority_cmp(&self, other: &Self) -> Ordering {
        // 値が「大きい」方が優先度が高い
        self.0.cmp(&other.0)
    }
}
