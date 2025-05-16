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

    let mut prefix_len = 0;
    while prefix_len < len1 && prefix_len < len2 && bytes1[prefix_len] == bytes2[prefix_len] {
        prefix_len += 1;
    }

    let mut suffix_len = 0;
    while suffix_len + prefix_len < len1
        && suffix_len + prefix_len < len2
        && bytes1[len1 - 1 - suffix_len] == bytes2[len2 - 1 - suffix_len]
    {
        suffix_len += 1;
    }

    let s1 = &bytes1[prefix_len..len1 - suffix_len];
    let s2 = &bytes2[prefix_len..len2 - suffix_len];

    let m = s1.len();
    let n = s2.len();

    let mut memo = vec![None; (m + 1) * (n + 1)];
    dp(s1, s2, 0, 0, &mut memo, m.max(n), n)
}

#[inline(always)]
fn idx(i: usize, j: usize, n: usize) -> usize {
    i * (n + 1) + j
}

fn dp(
    s1: &[u8],
    s2: &[u8],
    i: usize,
    j: usize,
    memo: &mut [Option<usize>],
    min_distance_found: usize,
    n: usize,
) -> usize {
    let k = idx(i, j, n);
    if let Some(cached) = memo[k] {
        return cached;
    }

    let len1 = s1.len();
    let len2 = s2.len();

    // Manual abs_diff
    let remaining = if len1 - i > len2 - j {
        len1 - i - (len2 - j)
    } else {
        len2 - j - (len1 - i)
    };

    if remaining >= min_distance_found {
        return min_distance_found;
    }

    let result = if i == len1 {
        len2 - j
    } else if j == len2 {
        len1 - i
    } else if s1[i] == s2[j] {
        return dp(s1, s2, i + 1, j + 1, memo, min_distance_found, n);
    } else {
        let d_del = 1 + dp(
            s1,
            s2,
            i + 1,
            j,
            memo,
            min_distance_found.saturating_sub(1),
            n,
        );
        if d_del >= min_distance_found {
            memo[k] = Some(min_distance_found);
            return min_distance_found;
        }

        let d_ins = 1 + dp(s1, s2, i, j + 1, memo, d_del.saturating_sub(1), n);
        if d_ins >= d_del {
            memo[k] = Some(d_del);
            return d_del;
        }

        let d_sub = 1 + dp(s1, s2, i + 1, j + 1, memo, d_ins.saturating_sub(1), n);
        d_del.min(d_ins).min(d_sub)
    };

    memo[k] = Some(result);
    result
}
