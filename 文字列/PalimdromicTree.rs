//回文部分文字列を求めるための木構造
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    length: usize,
    next: HashMap<char, usize>,
    suffix_link: usize,
    count: usize,
}

#[derive(Debug, Clone)]
struct PalindromicTree {
    nodes: Vec<Node>,
    s: Vec<char>,
    last: usize,
}

impl PalindromicTree {
    fn new() -> Self {
        PalindromicTree {
            nodes: vec![
                Node {
                    length: -1 as usize,
                    next: HashMap::new(),
                    suffix_link: 0,
                    count: 0,
                },
                Node {
                    length: 0,
                    next: HashMap::new(),
                    suffix_link: 0,
                    count: 0,
                },
            ],
            s: vec![],
            last: 1,
        }
    }

    fn get_suffix_link(&self, mut p: usize, i: usize) -> usize {
        while i - 1 - self.nodes[p].length < 0
            || self.s[i - 1] != self.s[i - 1 - self.nodes[p].length - 1]
        {
            p = self.nodes[p].suffix_link;
        }
        p
    }
    //文字列に新しい文字を追加
    fn add(&mut self, c: char) {
        self.s.push(c);
        let i = self.s.len();
        let mut cur = self.get_suffix_link(self.last, i);
        if self.nodes[cur].next.contains_key(&c) {
            self.last = self.nodes[cur].next[&c];
            self.nodes[self.last].count += 1;
            return;
        }
        self.nodes.push(Node {
            length: self.nodes[cur].length + 2,
            next: HashMap::new(),
            suffix_link: 0,
            count: 1,
        });
        self.last = self.nodes.len() - 1;
        self.nodes[cur].next.insert(c, self.last);
        if self.nodes.len() == 2 {
            self.nodes[self.last].suffix_link = 0;
            return;
        }
        self.nodes[self.last].suffix_link =
            self.nodes[self.get_suffix_link(self.nodes[cur].suffix_link, i)].next[&c];
    }
    //文字列内の回文部分文字列の総数を返す
    fn count_palindromes(&self) -> usize {
        self.nodes.iter().map(|node| node.count).sum()
    }
}
