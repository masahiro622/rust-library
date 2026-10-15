/// Functional Graphからすべてのサイクルを検出して返す関数
/// `next_node[i]` は 頂点`i` の遷移先を表す
fn find_cycles(next_node: &[usize]) -> Vec<Vec<usize>> {
    let n = next_node.len();
    // 0: 未訪問, 1: 現在のパスで探索中, 2: 探索完了
    let mut state = vec![0; n];
    let mut cycles = Vec::new();

    for i in 0..n {
        if state[i] != 0 {
            continue; // 既に探索済みの頂点はスキップ
        }

        let mut path = Vec::new();
        let mut curr = i;

        // 未訪問の頂点を辿り続ける
        while state[curr] == 0 {
            state[curr] = 1; // 探索中に変更
            path.push(curr);
            curr = next_node[curr];
        }

        // 行き着いた先が「現在のパスで探索中の頂点」ならサイクルを検出
        if state[curr] == 1 {
            // pathの中からcurrが最初に現れた位置を探し、そこから後ろがサイクル
            let cycle_start = path.iter().position(|&x| x == curr).unwrap();
            let cycle = path[cycle_start..].to_vec();
            cycles.push(cycle);
        }

        // 現在のパスに含まれるすべての頂点を「探索完了」にマーク
        for &node in &path {
            state[node] = 2;
        }
    }

    cycles
}
