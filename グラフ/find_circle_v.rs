fn find_cycle(graph: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let n = graph.len();
    let mut visited = vec![0u8; n]; // 0: 未訪問, 1: 訪問中, 2: 訪問済み
    let mut prev = vec![None; n];
    let mut cycle = Vec::new();

    fn dfs(
        u: usize,
        graph: &Vec<Vec<usize>>,
        visited: &mut Vec<u8>,
        prev: &mut Vec<Option<usize>>,
        cycle: &mut Vec<usize>,
    ) -> bool {
        visited[u] = 1; // 訪問中
        for &v in &graph[u] {
            if visited[v] == 0 {
                prev[v] = Some(u);
                if dfs(v, graph, visited, prev, cycle) {
                    return true;
                }
            } else if visited[v] == 1 {
                // サイクル発見：v から u への back edge
                // prev をたどって cycle を復元
                cycle.push(v);
                let mut cur = u;
                while cur != v {
                    cycle.push(cur);
                    cur = prev[cur].unwrap();
                }
                cycle.reverse();
                return true;
            }
        }
        visited[u] = 2; // 訪問完了
        false
    }

    for u in 0..n {
        if visited[u] == 0 && dfs(u, graph, &mut visited, &mut prev, &mut cycle) {
            return Some(cycle);
        }
    }
    None
}

fn find_cycle_undirected(graph: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let n = graph.len();
    let mut visited = vec![false; n];
    let mut prev = vec![None; n];
    let mut cycle = Vec::new();

    fn dfs(
        u: usize,
        parent: Option<usize>,
        graph: &Vec<Vec<usize>>,
        visited: &mut Vec<bool>,
        prev: &mut Vec<Option<usize>>,
        cycle: &mut Vec<usize>,
    ) -> bool {
        visited[u] = true;
        for &v in &graph[u] {
            if Some(v) == parent {
                continue; // 無向グラフなので親への辺は無視
            }
            if visited[v] {
                // サイクル検出
                cycle.push(v);
                let mut cur = u;
                while cur != v {
                    cycle.push(cur);
                    cur = prev[cur].unwrap();
                }
                cycle.reverse();
                return true;
            } else {
                prev[v] = Some(u);
                if dfs(v, Some(u), graph, visited, prev, cycle) {
                    return true;
                }
            }
        }
        false
    }

    for u in 0..n {
        if !visited[u] && dfs(u, None, graph, &mut visited, &mut prev, &mut cycle) {
            return Some(cycle);
        }
    }

    None
}
