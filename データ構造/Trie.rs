use std::io::{self, BufRead, Write};

struct Trie {
    children: [Option<Box<Trie>>; 2],
    count: usize,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: [None, None],
            count: 0,
        }
    }

    fn insert(&mut self, x: u32) {
        let mut node = self;
        for i in (0..30).rev() {
            let bit = ((x >> i) & 1) as usize;
            node = node.children[bit].get_or_insert_with(|| Box::new(Trie::new()));
            node.count += 1;
        }
    }

    fn erase(&mut self, x: u32) {
        let mut node = self;
        for i in (0..30).rev() {
            let bit = ((x >> i) & 1) as usize;
            let next = node.children[bit].as_mut().unwrap();
            next.count -= 1;
            node = next;
        }
    }

    fn query(&self, x: u32) -> u32 {
        let mut node = self;
        let mut res = 0;
        for i in (0..30).rev() {
            let bit = ((x >> i) & 1) as usize;
            let preferred = bit;
            let alternative = 1 - bit;

            if let Some(child) = node.children[preferred].as_ref() {
                if child.count > 0 {
                    node = child;
                    // res += 0 << i (nothing to do)
                    continue;
                }
            }
            // fallback to alternative
            if let Some(child) = node.children[alternative].as_ref() {
                if child.count > 0 {
                    node = child;
                    res |= 1 << i;
                }
            }
        }
        res
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout());
    let mut lines = stdin.lock().lines();
    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut trie = Trie::new();
    let mut set = std::collections::HashSet::new();

    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut it = line.split_whitespace();
        let t: u8 = it.next().unwrap().parse().unwrap();
        let x: u32 = it.next().unwrap().parse().unwrap();

        match t {
            0 => {
                if set.insert(x) {
                    trie.insert(x);
                }
            }
            1 => {
                if set.remove(&x) {
                    trie.erase(x);
                }
            }
            2 => {
                let ans = trie.query(x);
                writeln!(stdout, "{}", ans).unwrap();
            }
            _ => unreachable!(),
        }
    }
}
