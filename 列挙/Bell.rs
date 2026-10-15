/// N個の要素（0 ~ N-1 のインデックス）をグループに分ける全パターンを列挙する
pub fn list_set_partitions(n: usize) -> Vec<Vec<Vec<usize>>> {
    let mut result = vec![];
    if n == 0 {
        result.push(Vec::new());
        return result;
    }

    let mut current_partition: Vec<Vec<usize>> = Vec::new();
    backtrack(0, n, &mut current_partition, &mut result);
    result
}

fn backtrack(
    current_item: usize,
    n: usize,
    current_partition: &mut Vec<Vec<usize>>,
    result: &mut Vec<Vec<Vec<usize>>>,
) {
    // すべての要素を振り分け終えたら結果に格納
    if current_item == n {
        result.push(current_partition.clone());
        return;
    }

    // パターン1: 既存のグループのいずれかに追加する
    for i in 0..current_partition.len() {
        current_partition[i].push(current_item);
        backtrack(current_item + 1, n, current_partition, result);
        current_partition[i].pop(); // バックトラック（状態を戻す）
    }

    // パターン2: 新しいグループを作成してそこに入れる
    // （重複を避けるため、常に新しいグループの先頭要素として追加する形になる）
    current_partition.push(vec![current_item]);
    backtrack(current_item + 1, n, current_partition, result);
    current_partition.pop(); // バックトラック（状態を戻す）
}
