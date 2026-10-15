use std::collections::HashMap;

// terminal_count: そのノードで終端する文字列の数
// prefix_count: そのノードをプレフィックスに持つ文字列の数
#[derive(Debug)]
struct TrieNode<T: Eq + std::hash::Hash + Clone> {
    item: T,
    next: HashMap<T, usize>,
    terminal_count: usize,
    prefix_count: usize,
}

impl<T: Eq + std::hash::Hash + Clone> TrieNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            next: HashMap::new(),
            terminal_count: 0,
            prefix_count: 0,
        }
    }
}

#[derive(Debug)]
struct TrieTree<T: Eq + std::hash::Hash + Clone> {
    nodes: Vec<TrieNode<T>>,
}

// artificial item: 登録する列には存在しないものを指定すること
impl<T: Eq + std::hash::Hash + Clone> TrieTree<T> {
    fn new(artificial_item: T) -> Self {
        Self {
            nodes: vec![TrieNode::new(artificial_item)],
        }
    }

    // 文字列 s を挿入
    fn insert(&mut self, s: &[T]) {
        let mut node_id = 0;
        self.nodes[node_id].prefix_count += 1;
        for x in s {
            let entry = self.nodes[node_id].next.get(x);
            let next_id = if let Some(&id) = entry {
                id
            } else {
                let id = self.nodes.len();
                self.nodes.push(TrieNode::new(x.clone()));
                self.nodes[node_id].next.insert(x.clone(), id);
                id
            };
            node_id = next_id;
            self.nodes[node_id].prefix_count += 1;
        }
        self.nodes[node_id].terminal_count += 1;
    }

    // 文字列 s を start_id から挿入
    fn insert_continuation(&mut self, s: &[T], start_id: usize) {
        let mut node_id = start_id;
        self.nodes[node_id].prefix_count += 1;
        for x in s {
            // Check if the child exists first
            let next_id = if let Some(&id) = self.nodes[node_id].next.get(x) {
                id
            } else {
                let id = self.nodes.len();
                self.nodes.push(TrieNode::new(x.clone()));
                self.nodes[node_id].next.insert(x.clone(), id);
                id
            };
            node_id = next_id;
            self.nodes[node_id].prefix_count += 1;
        }
        self.nodes[node_id].terminal_count += 1;
    }

    // 文字列 s の出現回数をカウント
    fn count(&self, s: &[T]) -> usize {
        let mut node_id = 0;
        for x in s {
            match self.nodes[node_id].next.get(x) {
                Some(&id) => node_id = id,
                None => return 0,
            }
        }
        self.nodes[node_id].terminal_count
    }

    // 文字列 s の出現回数を start_id からカウント
    fn count_continuation(&self, s: &[T], start_id: usize) -> usize {
        let mut node_id = start_id;
        for x in s {
            match self.nodes[node_id].next.get(x) {
                Some(&id) => node_id = id,
                None => return 0,
            }
        }
        self.nodes[node_id].terminal_count
    }

    // 文字列 s が存在するかどうか
    fn search(&self, s: &[T]) -> bool {
        self.count(s) > 0
    }

    // 文字列 s が start_id から存在するかどうか
    fn search_continuation(&self, s: &[T], start_id: usize) -> bool {
        self.count_continuation(s, start_id) > 0
    }

    // 文字列 s がプレフィックスとして存在するかどうか
    fn search_prefix(&self, s: &[T]) -> bool {
        let mut node_id = 0;
        for x in s {
            match self.nodes[node_id].next.get(x) {
                Some(&id) => node_id = id,
                None => return false,
            }
        }
        true
    }

    // 文字列 s が start_id からプレフィックスとして存在するかどうか
    fn search_prefix_continuation(&self, s: &[T], start_id: usize) -> bool {
        let mut node_id = start_id;
        for x in s {
            match self.nodes[node_id].next.get(x) {
                Some(&id) => node_id = id,
                None => return false,
            }
        }
        true
    }

    // 文字列 s をプレフィックスとする文字列の出現回数をカウント
    fn count_prefixing(&self, s: &[T], equal: bool) -> usize {
        let mut node_id = 0;
        for x in s {
            match self.nodes[node_id].next.get(x) {
                Some(&id) => node_id = id,
                None => return 0,
            }
        }
        if equal {
            self.nodes[node_id].prefix_count
        } else {
            self.nodes[node_id].prefix_count - self.nodes[node_id].terminal_count
        }
    }

    // 文字列 s をプレフィックスとする文字列の出現回数を start_id からカウント
    fn count_prefixing_continuation(&self, s: &[T], start_id: usize) -> usize {
        let mut node_id = start_id;
        for x in s {
            match self.nodes[node_id].next.get(x) {
                Some(&id) => node_id = id,
                None => return 0,
            }
        }
        self.nodes[node_id].prefix_count
    }

    // 文字列 s をプレフィックスに持つ文字列の出現回数をカウント
    fn count_prefixed(&self, s: &[T], equal: bool) -> usize {
        let mut node_id = 0;
        let mut count = self.nodes[node_id].terminal_count;
        for x in s {
            match self.nodes[node_id].next.get(x) {
                Some(&id) => {
                    node_id = id;
                    count += self.nodes[node_id].terminal_count;
                }
                None => return count,
            }
        }
        if equal {
            count
        } else {
            count - self.nodes[node_id].terminal_count
        }
    }

    // 登録されている全ての文字列の出現回数をカウント
    fn count_all(&self) -> usize {
        self.nodes[0].prefix_count
    }

    // ノード数を取得
    fn size(&self) -> usize {
        self.nodes.len()
    }

    // 文字列 s が登録されている場合、そのノードIDを返す。存在しない場合 -1 を返す
    fn prefix_node_id(&self, s: &[T]) -> isize {
        let mut node_id = 0;
        for x in s {
            match self.nodes[node_id].next.get(x) {
                Some(&id) => node_id = id,
                None => return -1,
            }
        }
        node_id as isize
    }
}
