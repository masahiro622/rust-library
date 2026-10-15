fn run_length_encode<T: PartialEq + Clone>(v: &[T]) -> Vec<(T, usize)> {
    let mut res = Vec::new();
    if v.is_empty() {
        return res;
    }

    let mut cur = v[0].clone();
    let mut count = 1;

    for x in &v[1..] {
        if *x == cur {
            count += 1;
        } else {
            res.push((cur, count));
            cur = x.clone();
            count = 1;
        }
    }
    res.push((cur, count));
    res
}
