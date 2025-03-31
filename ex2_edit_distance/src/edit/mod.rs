use std::cmp::min;
use std::collections::HashMap;

pub fn edit_distance(s1: &str, s2: &str) -> usize {
    if s1.is_empty() {
        return s2.len();
    };
    if s2.is_empty() {
        return s1.len();
    };

    let mut d_no_op = usize::MAX;
    if s1.chars().next() == s2.chars().next() {
        d_no_op = edit_distance(&s1[1..], &s2[1..]);
    }

    let d_del = 1 + edit_distance(&s1[1..], s2);
    let d_ins = 1 + edit_distance(s1, &s2[1..]);

    min(d_no_op, min(d_del, d_ins))
}

pub fn edit_distance_dyn(s1: &str, s2: &str) -> usize {
    let mut memo = HashMap::new();
    dp(s1, s2, &mut memo, usize::MAX)
}

fn dp<'a>(
    s1: &'a str,
    s2: &'a str,
    memo: &mut HashMap<(&'a str, &'a str), usize>,
    min_distance_found: usize,
) -> usize {
    if let Some(&result) = memo.get(&(s1, s2)) {
        return result;
    }

    if s1.len().abs_diff(s2.len()) >= min_distance_found {
        return min_distance_found;
    }

    let result = if s1.is_empty() {
        s2.len()
    } else if s2.is_empty() {
        s1.len()
    } else {
        let (first1, rest1) = s1.split_at(1);
        let (first2, rest2) = s2.split_at(1);

        let d_no_op = if first1 == first2 {
            dp(rest1, rest2, memo, min_distance_found)
        } else {
            usize::MAX
        };

        if d_no_op == 0 {
            return 0;
        }

        let d_del = 1 + dp(
            rest1,
            s2,
            memo,
            min_distance_found.min(d_no_op).saturating_sub(1),
        );

        if d_del >= min_distance_found {
            return min_distance_found;
        }

        let d_ins = 1 + dp(
            s1,
            rest2,
            memo,
            min_distance_found.min(d_del).saturating_sub(1),
        );

        d_no_op.min(d_del).min(d_ins)
    };

    memo.insert((s1, s2), result);

    if result < min_distance_found {
        result
    } else {
        min_distance_found
    }
}
