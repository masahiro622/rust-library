use std::collections::{HashMap, VecDeque};

/// トライ木の各ノードを表す構造体
#[derive(Default, Debug)]
struct Node {
    children: HashMap<char, usize>, // 次の文字とノードのインデックス
    fail: usize,                    // 失敗遷移先のノードインデックス
    outputs: Vec<usize>,            // このノードに到達した際にマッチするパターンのインデックス
}

/// Aho-Corasickオートマトン本体
pub struct AhoCorasick {
    nodes: Vec<Node>,
    patterns: Vec<String>,
}

/// 検索結果を格納する構造体
#[derive(Debug, PartialEq, Eq)]
pub struct Match<'a> {
    pub pattern: &'a str,
    pub pattern_index: usize,
    pub start: usize, // マッチした文字列の開始バイト位置
    pub end: usize,   // マッチした文字列の終了バイト位置
}

impl AhoCorasick {
    /// 検索対象のパターン（キーワード）のリストからオートマトンを構築します
    pub fn new<I, S>(patterns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut ac = AhoCorasick {
            nodes: vec![Node::default()], // インデックス0はルートノード
            patterns: Vec::new(),
        };

        // 1. トライ木の構築
        for (i, pattern) in patterns.into_iter().enumerate() {
            let pat = pattern.into();
            ac.insert(&pat, i);
            ac.patterns.push(pat);
        }

        // 2. 失敗遷移（Failure Link）の構築
        ac.build_failure_links();
        ac
    }

    /// パターンをトライ木に追加します
    fn insert(&mut self, pattern: &str, pattern_index: usize) {
        let mut current = 0;
        for c in pattern.chars() {
            if !self.nodes[current].children.contains_key(&c) {
                let new_node_idx = self.nodes.len();
                self.nodes.push(Node::default());
                self.nodes[current].children.insert(c, new_node_idx);
            }
            current = *self.nodes[current].children.get(&c).unwrap();
        }
        // 単語の終端ノードにパターンのインデックスを記録
        self.nodes[current].outputs.push(pattern_index);
    }

    /// 幅優先探索（BFS）を用いて失敗遷移を構築します
    fn build_failure_links(&mut self) {
        let mut queue = VecDeque::new();

        // ルート（深さ0）の直接の子（深さ1）の失敗遷移はルート(0)に設定
        let root_children: Vec<usize> = self.nodes[0].children.values().copied().collect();
        for child_idx in root_children {
            self.nodes[child_idx].fail = 0;
            queue.push_back(child_idx);
        }

        while let Some(current) = queue.pop_front() {
            let children = self.nodes[current].children.clone();

            for (c, &child_idx) in &children {
                let mut fail = self.nodes[current].fail;

                // 失敗遷移先を辿り、文字 `c` への遷移を持つノードを探す
                while fail != 0 && !self.nodes[fail].children.contains_key(c) {
                    fail = self.nodes[fail].fail;
                }

                // 見つかった場合はその子ノードを失敗遷移先に、見つからなければルート(0)を設定
                let fail_target = self.nodes[fail].children.get(c).copied().unwrap_or(0);
                self.nodes[child_idx].fail = fail_target;

                // 失敗遷移先のノードが持つマッチ情報（outputs）を現在のノードにもマージする
                // （例: "she" にマッチした時は暗黙的に "he" にもマッチしているため）
                let fail_node_idx = self.nodes[child_idx].fail;
                if child_idx != fail_node_idx {
                    let mut outputs_to_add = if child_idx < fail_node_idx {
                        let (_, right) = self.nodes.split_at_mut(fail_node_idx);
                        right[0].outputs.clone()
                    } else {
                        let (left, _) = self.nodes.split_at_mut(child_idx);
                        left[fail_node_idx].outputs.clone()
                    };

                    if child_idx < fail_node_idx {
                        let (left, _right) = self.nodes.split_at_mut(fail_node_idx);
                        left[child_idx].outputs.append(&mut outputs_to_add);
                    } else {
                        let (_left, right) = self.nodes.split_at_mut(child_idx);
                        right[0].outputs.append(&mut outputs_to_add);
                    }
                }

                queue.push_back(child_idx);
            }
        }
    }

    /// 与えられたテキストに対して検索を実行します
    pub fn search<'a>(&'a self, text: &'a str) -> Vec<Match<'a>> {
        let mut results = Vec::new();
        let mut current = 0;

        for (i, c) in text.char_indices() {
            // 現在のノードから文字 `c` で遷移できない場合、失敗遷移を辿る
            while current != 0 && !self.nodes[current].children.contains_key(&c) {
                current = self.nodes[current].fail;
            }

            // 遷移先があれば進む
            if let Some(&next) = self.nodes[current].children.get(&c) {
                current = next;
            } else {
                current = 0; // ルートに戻る
            }

            // 到達したノードに登録されているパターンを結果に追加
            for &pattern_idx in &self.nodes[current].outputs {
                let pattern = &self.patterns[pattern_idx];
                let end = i + c.len_utf8();
                let start = end - pattern.len(); // バイト長で計算

                results.push(Match {
                    pattern,
                    pattern_index: pattern_idx,
                    start,
                    end,
                });
            }
        }

        results
    }
}
