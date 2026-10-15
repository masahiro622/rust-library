struct DistanceCounter {
    n: usize,
    graph: Vec<Vec<usize>>,
    deleted: Vec<bool>,
    subtree_size: Vec<usize>,
    parent: Vec<usize>,
    component_nodes: Vec<usize>,
    ans: Vec<u64>,
}

impl DistanceCounter {
    fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Self {
            n,
            graph,
            deleted: vec![false; n],
            subtree_size: vec![0; n],
            parent: vec![n; n],
            component_nodes: Vec::new(),
            ans: vec![0; n],
        }
    }

    fn solve(&mut self) -> Vec<u64> {
        if self.n > 0 {
            self.decompose(0);
        }
        std::mem::take(&mut self.ans)
    }

    fn build_component_subtree_sizes(&mut self, root: usize) -> usize {
        let none_parent = self.n;
        self.component_nodes.clear();

        let mut stack = vec![(root, none_parent, false)];
        while let Some((u, p, visited)) = stack.pop() {
            if !visited {
                self.parent[u] = p;
                self.component_nodes.push(u);
                stack.push((u, p, true));
                for &v in &self.graph[u] {
                    if self.deleted[v] || v == p {
                        continue;
                    }
                    stack.push((v, u, false));
                }
            } else {
                let mut size = 1usize;
                for &v in &self.graph[u] {
                    if self.deleted[v] || self.parent[v] != u {
                        continue;
                    }
                    size += self.subtree_size[v];
                }
                self.subtree_size[u] = size;
            }
        }

        self.subtree_size[root]
    }

    fn find_centroid(&self, total_nodes: usize) -> usize {
        let mut centroid = self.component_nodes[0];
        for &u in &self.component_nodes {
            let mut max_part = total_nodes - self.subtree_size[u];
            for &v in &self.graph[u] {
                if self.deleted[v] {
                    continue;
                }
                if self.parent[v] == u {
                    max_part = max_part.max(self.subtree_size[v]);
                }
            }
            if max_part * 2 <= total_nodes {
                centroid = u;
                break;
            }
        }
        centroid
    }

    fn collect_depths(&self, start: usize, parent: usize) -> Vec<usize> {
        let mut depths = Vec::new();
        let mut stack = vec![(start, parent, 1usize)];

        while let Some((u, p, depth)) = stack.pop() {
            depths.push(depth);
            for &v in &self.graph[u] {
                if self.deleted[v] || v == p {
                    continue;
                }
                stack.push((v, u, depth + 1));
            }
        }

        depths
    }

    fn decompose(&mut self, root: usize) {
        let total_nodes = self.build_component_subtree_sizes(root);
        let centroid = self.find_centroid(total_nodes);

        self.deleted[centroid] = true;

        let mut all_freq = vec![1i64];
        let mut child_freqs: Vec<Vec<i64>> = Vec::new();
        let neighbors = self.graph[centroid].clone();
        for &v in &neighbors {
            if self.deleted[v] {
                continue;
            }

            let depths = self.collect_depths(v, centroid);
            if depths.is_empty() {
                continue;
            }

            let max_depth = *depths.iter().max().unwrap();
            let mut sub_freq = vec![0i64; max_depth + 1];
            for &d in &depths {
                sub_freq[d] += 1;
            }

            if all_freq.len() < sub_freq.len() {
                all_freq.resize(sub_freq.len(), 0);
            }
            for d in 1..sub_freq.len() {
                all_freq[d] += sub_freq[d];
            }
            child_freqs.push(sub_freq);
        }

        let conv_all = convolution_i64(&all_freq, &all_freq);
        let mut ordered = vec![0i128; conv_all.len()];
        for (i, &v) in conv_all.iter().enumerate() {
            ordered[i] = v as i128;
        }

        for freq in &child_freqs {
            let conv_sub = convolution_i64(freq, freq);
            for (i, &v) in conv_sub.iter().enumerate() {
                ordered[i] -= v as i128;
            }
        }

        for d in 1..ordered.len().min(self.n) {
            self.ans[d] += (ordered[d] / 2) as u64;
        }

        for &v in &neighbors {
            if !self.deleted[v] {
                self.decompose(v);
            }
        }
    }
}
