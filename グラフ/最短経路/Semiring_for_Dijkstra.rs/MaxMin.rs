#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct MaxMin(pub i64);

impl PathWeight for MaxMin {
    fn start() -> Self {
        MaxMin(i64::MAX)
    }
    fn infinity() -> Self {
        MaxMin(i64::MIN)
    }
    fn accumulate(self, edge: Self) -> Self {
        MaxMin(self.0.min(edge.0))
    }
    fn priority_cmp(&self, other: &Self) -> Ordering {
        // 値が「大きい」方が優先度が高い
        self.0.cmp(&other.0)
    }
}
