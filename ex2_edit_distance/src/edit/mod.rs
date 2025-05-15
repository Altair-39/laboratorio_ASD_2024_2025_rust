pub fn edit_distance(s1: &str, s2: &str) -> usize {
    use std::cmp::min;

    if s1.is_empty() {
        return s2.chars().count();
    }
    if s2.is_empty() {
        return s1.chars().count();
    }

    let mut d_no_op = usize::MAX;

    let mut s1_chars = s1.chars();
    let mut s2_chars = s2.chars();

    let c1 = s1_chars.next().unwrap();
    let c2 = s2_chars.next().unwrap();

    let s1_rest = s1.get(c1.len_utf8()..).unwrap_or("");
    let s2_rest = s2.get(c2.len_utf8()..).unwrap_or("");

    if c1 == c2 {
        d_no_op = edit_distance(s1_rest, s2_rest);
    }

    let d_del = 1 + edit_distance(s1_rest, s2);
    let d_ins = 1 + edit_distance(s1, s2_rest);

    min(d_no_op, min(d_del, d_ins))
}

pub fn edit_distance_dyn(s1: &str, s2: &str) -> usize {
    let bytes1 = s1.as_bytes();
    let bytes2 = s2.as_bytes();

    let len1 = bytes1.len();
    let len2 = bytes2.len();

    let mut memo = vec![vec![None; len2 + 1]; len1 + 1];

    dp(bytes1, bytes2, 0, 0, &mut memo, usize::MAX)
}

fn dp(
    s1: &[u8],
    s2: &[u8],
    i: usize,
    j: usize,
    memo: &mut Vec<Vec<Option<usize>>>,
    min_distance_found: usize,
) -> usize {
    if let Some(cached) = memo[i][j] {
        return cached;
    }

    if (s1.len() - i).abs_diff(s2.len() - j) >= min_distance_found {
        return min_distance_found;
    }

    let result = if i == s1.len() {
        s2.len() - j
    } else if j == s2.len() {
        s1.len() - i
    } else if s1[i] == s2[j] {
        dp(s1, s2, i + 1, j + 1, memo, min_distance_found)
    } else {
        let d_no_op = usize::MAX;

        let d_del = 1 + dp(
            s1,
            s2,
            i + 1,
            j,
            memo,
            min_distance_found.min(d_no_op).saturating_sub(1),
        );
        if d_del >= min_distance_found {
            memo[i][j] = Some(min_distance_found);
            return min_distance_found;
        }

        let d_ins = 1 + dp(
            s1,
            s2,
            i,
            j + 1,
            memo,
            min_distance_found.min(d_del).saturating_sub(1),
        );

        let d_sub = 1 + dp(
            s1,
            s2,
            i + 1,
            j + 1,
            memo,
            min_distance_found.min(d_ins).saturating_sub(1),
        );

        d_del.min(d_ins).min(d_sub)
    };

    memo[i][j] = Some(result);
    result.min(min_distance_found)
}
