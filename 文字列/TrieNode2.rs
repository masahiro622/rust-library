// 英小文字 a-z 26文字専用 Trie 実装
// terminal_count: そのノードで終端する文字列本数
// prefix_count  : そのノードをプレフィックスに持つ（＝通過する）文字列本数
#[derive(Debug)]
struct TrieNode {
    children: [Option<usize>; 26],
    terminal_count: usize,
    prefix_count: usize,
}

impl TrieNode {
    #[inline]
    fn new() -> Self {
        Self {
            children: [None; 26],
            terminal_count: 0,
            prefix_count: 0,
        }
    }
}

#[derive(Debug)]
struct TrieTree {
    nodes: Vec<TrieNode>,
}

impl TrieTree {
    fn new() -> Self {
        Self {
            nodes: vec![TrieNode::new()],
        }
    }

    #[inline]
    fn idx(c: u8) -> usize {
        let k = c.wrapping_sub(b'a');
        assert!(k < 26, "non-lowercase ascii: {}", c);
        k as usize
    }

    // s: &str (英小文字)
    fn insert(&mut self, s: &str) {
        let mut node = 0;
        self.nodes[node].prefix_count += 1;
        for &b in s.as_bytes() {
            let k = Self::idx(b);
            if self.nodes[node].children[k].is_none() {
                let nid = self.nodes.len();
                self.nodes[node].children[k] = Some(nid);
                self.nodes.push(TrieNode::new());
            }
            node = self.nodes[node].children[k].unwrap();
            self.nodes[node].prefix_count += 1;
        }
        self.nodes[node].terminal_count += 1;
    }

    // start_id から継続挿入
    fn insert_continuation(&mut self, s: &str, start_id: usize) {
        let mut node = start_id;
        self.nodes[node].prefix_count += 1;
        for &b in s.as_bytes() {
            let k = Self::idx(b);
            if self.nodes[node].children[k].is_none() {
                let nid = self.nodes.len();
                self.nodes[node].children[k] = Some(nid);
                self.nodes.push(TrieNode::new());
            }
            node = self.nodes[node].children[k].unwrap();
            self.nodes[node].prefix_count += 1;
        }
        self.nodes[node].terminal_count += 1;
    }

    fn count(&self, s: &str) -> usize {
        match self.walk(s) {
            Some(id) => self.nodes[id].terminal_count,
            None => 0,
        }
    }

    fn count_continuation(&self, s: &str, start_id: usize) -> usize {
        match self.walk_from(s, start_id) {
            Some(id) => self.nodes[id].terminal_count,
            None => 0,
        }
    }

    fn search(&self, s: &str) -> bool {
        self.count(s) > 0
    }
    fn search_continuation(&self, s: &str, start_id: usize) -> bool {
        self.count_continuation(s, start_id) > 0
    }
    fn search_prefix(&self, s: &str) -> bool {
        self.walk(s).is_some()
    }
    fn search_prefix_continuation(&self, s: &str, start_id: usize) -> bool {
        self.walk_from(s, start_id).is_some()
    }

    fn count_prefixing(&self, s: &str, equal: bool) -> usize {
        match self.walk(s) {
            Some(id) => {
                if equal {
                    self.nodes[id].prefix_count
                } else {
                    self.nodes[id].prefix_count - self.nodes[id].terminal_count
                }
            }
            None => 0,
        }
    }

    fn count_prefixing_continuation(&self, s: &str, start_id: usize) -> usize {
        match self.walk_from(s, start_id) {
            Some(id) => self.nodes[id].prefix_count,
            None => 0,
        }
    }

    fn count_prefixed(&self, s: &str, equal: bool) -> usize {
        let mut node = 0usize;
        let mut acc = self.nodes[node].terminal_count;
        for &b in s.as_bytes() {
            let k = Self::idx(b);
            match self.nodes[node].children[k] {
                Some(nid) => {
                    node = nid;
                    acc += self.nodes[node].terminal_count;
                }
                None => return acc,
            }
        }
        if equal {
            acc
        } else {
            acc - self.nodes[node].terminal_count
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
    fn prefix_node_id(&self, s: &[u8]) -> isize {
        let mut node_id = 0;
        for &b in s {
            let k = Self::idx(b);
            match self.nodes[node_id].children[k] {
                Some(id) => node_id = id,
                None => return -1,
            }
        }
        node_id as isize
    }

    // ================== 追加: 削除機能 ==================
    // 指定文字列を 1 回分削除（存在しなければ false）
    fn remove(&mut self, s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        // 経路記録: (parent_id, child_idx, child_id)
        let mut node = 0usize;
        let mut path_nodes: Vec<usize> = Vec::with_capacity(s.len() + 1);
        let mut path_edges: Vec<(usize, usize, usize)> = Vec::with_capacity(s.len());
        path_nodes.push(0);
        for &b in s.as_bytes() {
            let k = Self::idx(b);
            match self.nodes[node].children[k] {
                Some(nid) => {
                    path_edges.push((node, k, nid));
                    node = nid;
                    path_nodes.push(node);
                }
                None => return false, // 未登録
            }
        }
        if self.nodes[node].terminal_count == 0 {
            return false;
        }
        // カウント減算
        self.nodes[node].terminal_count -= 1;
        for &nid in &path_nodes {
            self.nodes[nid].prefix_count -= 1;
        }
        // 末端から pruning
        for &(parent, idx, child) in path_edges.iter().rev() {
            if self.can_prune(child) {
                self.nodes[parent].children[idx] = None;
            } else {
                // 上位は pruning 不可ならそのまま打ち切り
                // （prefix_count==0 の伝播は既に減算済のため不要）
                // ただし更に上も prefix_count==0 で prune 可能な場合があるので続行しないと
                // prune 漏れすることがある。→ ここでは最後まで見る。
            }
        }
        true
    }

    #[inline]
    fn can_prune(&self, id: usize) -> bool {
        self.nodes[id].prefix_count == 0
            && self.nodes[id].terminal_count == 0
            && self.nodes[id].children.iter().all(|c| c.is_none())
    }

    fn is_empty(&self) -> bool {
        self.nodes[0].prefix_count == 0
    }

    // ================== 内部ユーティリティ ==================
    #[inline]
    fn walk(&self, s: &str) -> Option<usize> {
        let mut node = 0usize;
        for &b in s.as_bytes() {
            let k = Self::idx(b);
            node = self.nodes[node].children[k]?;
        }
        Some(node)
    }

    #[inline]
    fn walk_from(&self, s: &str, start_id: usize) -> Option<usize> {
        let mut node = start_id;
        for &b in s.as_bytes() {
            let k = Self::idx(b);
            node = self.nodes[node].children[k]?;
        }
        Some(node)
    }
}

// 簡単なテスト (必要なら #[cfg(test)] へ)
#[allow(dead_code)]
fn _sample() {
    let mut tr = TrieTree::new();
    tr.insert("apple");
    tr.insert("app");
    assert_eq!(tr.search("app"), true);
    assert_eq!(tr.count("apple"), 1);
    assert_eq!(tr.count_prefixing("app", true), 2); // "app","apple"
    assert_eq!(tr.count_prefixed("apple", false), 1); // "app" のみ
}
