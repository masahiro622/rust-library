use std::collections::VecDeque;

#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i64,
}

struct MaxFlow {
    size: usize,
    graph: Vec<Vec<Edge>>,
    level: Vec<i32>,
    iter: Vec<usize>,
}

impl MaxFlow {
    fn new(n: usize) -> Self {
        MaxFlow {
            size: n,
            graph: vec![vec![]; n],
            level: vec![-1; n],
            iter: vec![0; n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let rev = self.graph[to].len();
        let rev_back = self.graph[from].len();
        self.graph[from].push(Edge { to, rev, cap });
        self.graph[to].push(Edge {
            to: from,
            rev: rev_back,
            cap: 0,
        });
    }

    fn bfs(&mut self, s: usize) {
        self.level.fill(-1);
        let mut queue = VecDeque::new();
        self.level[s] = 0;
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            for e in &self.graph[v] {
                if e.cap > 0 && self.level[e.to] < 0 {
                    self.level[e.to] = self.level[v] + 1;
                    queue.push_back(e.to);
                }
            }
        }
    }

    fn dfs(&mut self, v: usize, t: usize, up_to: i64) -> i64 {
        if v == t {
            return up_to;
        }
        while self.iter[v] < self.graph[v].len() {
            let i = self.iter[v];
            let e = self.graph[v][i].clone();
            if e.cap > 0 && self.level[v] < self.level[e.to] {
                let d = self.dfs(e.to, t, up_to.min(e.cap));
                if d > 0 {
                    self.graph[v][i].cap -= d;
                    self.graph[e.to][e.rev].cap += d;
                    return d;
                }
            }
            self.iter[v] += 1;
        }
        0
    }

    fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0;
        loop {
            self.bfs(s);
            if self.level[t] < 0 {
                break;
            }
            self.iter.fill(0);
            loop {
                let f = self.dfs(s, t, i64::MAX);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
        flow
    }

    // 最小カット集合 S (s から到達可能な頂点集合)
    fn min_cut(&mut self, s: usize) -> Vec<usize> {
        self.bfs(s);
        (0..self.size).filter(|&v| self.level[v] >= 0).collect()
    }
}
