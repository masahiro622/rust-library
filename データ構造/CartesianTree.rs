/// T: Ord を満たすスライスから Cartesian Tree を構築する
/// 戻り値: (ノードのリスト, 根のインデックス)
pub fn cartesian_tree<T: Ord>(arr: &[T]) -> (Vec<Node>, usize) {
    let n = arr.len();
    assert!(n > 0, "Input array must be non-empty");
    let mut nodes = vec![Node::default(); n];
    let mut stack: Vec<usize> = Vec::with_capacity(n);

    for i in 0..n {
        let mut last_popped = None;

        // スタックのトップ（右端パスの末端）と現在の値を比較
        // Min-Heap構成: arr[top] > arr[i] ならば top は i の下（左部分木）に移動
        while let Some(&top_idx) = stack.last() {
            if arr[top_idx] > arr[i] {
                last_popped = stack.pop();
            } else {
                break;
            }
        }

        // 最後にポップされた要素（自分より大きかった要素）を自分の左の子にする
        if let Some(lp) = last_popped {
            nodes[i].left = Some(lp);
            nodes[lp].parent = Some(i);
        }

        // スタックに残っている要素（自分以下の要素）の右の子に自分がなる
        if let Some(&top_idx) = stack.last() {
            nodes[top_idx].right = Some(i);
            nodes[i].parent = Some(top_idx);
        }

        stack.push(i);
    }

    // 根を見つける（親がいないノードを探す。)
    let root = stack[0];

    (nodes, root)
}
