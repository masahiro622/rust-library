//配列の連続部分和の最大値を求める
//O(n)

fn kadane(arr: &[i32]) -> i32 {
    let mut current_sum = arr[0];
    let mut max_sum = arr[0];

    for &x in &arr[1..] {
        current_sum = current_sum.max(current_sum + x);
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}
