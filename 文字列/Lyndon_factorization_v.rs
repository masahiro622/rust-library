// Duval's algorithm (Lyndon factorization) in Rust
// Generic version for any sequence with elements T: Ord
// Returns indices a0..an such that seq[a_i..a_{i+1}] are the Lyndon factors.

fn duval_indices<T: Ord>(seq: &[T]) -> Vec<usize> {
    let n = seq.len();
    let mut res: Vec<usize> = Vec::new();
    let mut i = 0;
    res.push(0);

    while i < n {
        let mut j = i + 1;
        let mut k = i;
        while j < n && seq[k] <= seq[j] {
            if seq[k] < seq[j] {
                k = i;
            } else {
                k += 1;
            }
            j += 1;
        }
        let block_len = j - k;
        while i <= k {
            i += block_len;
            res.push(i);
        }
    }

    res
}
