use std::cmp::Ordering;

pub fn merge_sort<T, F>(base: &mut [T], compar: &F)
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    if base.len() <= 1 {
        return;
    }

    let mid = base.len() / 2;
    let (l, r) = base.split_at_mut(mid);

    merge_sort(l, compar);
    merge_sort(r, compar);

    let l_clone = l.to_vec();
    let r_clone = r.to_vec();
    merge(base, &l_clone, &r_clone, compar);
}

fn merge<T, F>(base: &mut [T], l: &[T], r: &[T], compar: &F)
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < l.len() && j < r.len() {
        if compar(&l[i], &r[j]) == Ordering::Less {
            base[k] = l[i].clone();
            i += 1;
        } else {
            base[k] = r[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < l.len() {
        base[k] = l[i].clone();
        i += 1;
        k += 1;
    }

    while j < r.len() {
        base[k] = r[j].clone();
        j += 1;
        k += 1;
    }
}
