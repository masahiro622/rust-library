struct SecondLargest;

impl Monoid for SecondLargest {
    type S = (i64, i64, i64, i64); // (max, max_cnt, second_max, second_max_cnt)

    fn identity() -> Self::S {
        (i64::MIN + 1, 0, i64::MIN, 0)
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let (mut a_max, mut a_cnt, mut a_sec, mut a_sec_cnt) = *a;
        let (mut b_max, mut b_cnt, mut b_sec, mut b_sec_cnt) = *b;

        // a の最大値が常に大きくなるようにスワップして場合分けを半分にする
        if a_max < b_max {
            std::mem::swap(&mut a_max, &mut b_max);
            std::mem::swap(&mut a_cnt, &mut b_cnt);
            std::mem::swap(&mut a_sec, &mut b_sec);
            std::mem::swap(&mut a_sec_cnt, &mut b_sec_cnt);
        }

        // 1. 最大値が同じ場合
        if a_max == b_max {
            let (sec, sec_cnt) = if a_sec == b_sec {
                (a_sec, a_sec_cnt + b_sec_cnt)
            } else if a_sec > b_sec {
                (a_sec, a_sec_cnt)
            } else {
                (b_sec, b_sec_cnt)
            };
            (a_max, a_cnt + b_cnt, sec, sec_cnt)
        }
        // 2. a_max > b_max が確定している場合（b_max が2番目の候補になる）
        else {
            let (sec, sec_cnt) = if a_sec == b_max {
                (a_sec, a_sec_cnt + b_cnt)
            } else if a_sec > b_max {
                (a_sec, a_sec_cnt)
            } else {
                (b_max, b_cnt)
            };
            (a_max, a_cnt, sec, sec_cnt)
        }
    }
}
