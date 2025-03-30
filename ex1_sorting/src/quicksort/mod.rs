use std::cmp::Ordering;
use std::cmp::PartialOrd;

pub fn quick_sort<T, F>(arr: &mut [T], compar: &F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> Ordering,
{
    if arr.len() <= 1 {
        return;
    }
    quick_sort_recursive(arr, compar);
}

fn quick_sort_recursive<T, F>(arr: &mut [T], compar: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut stack = Vec::with_capacity(32);
    stack.push((0, arr.len()));

    while let Some((start, end)) = stack.pop() {
        let len = end - start;

        if len <= 16 {
            insertion_sort(&mut arr[start..end], compar);
            continue;
        }

        let pivot_idx = choose_pivot(&mut arr[start..end], compar) + start;
        arr.swap(pivot_idx, end - 1);
        let pivot_pos = partition(&mut arr[start..end], compar) + start;

        let left_len = pivot_pos - start;
        let right_len = end - (pivot_pos + 1);

        if left_len > right_len {
            stack.push((start, pivot_pos));
            stack.push((pivot_pos + 1, end));
        } else {
            stack.push((pivot_pos + 1, end));
            stack.push((start, pivot_pos));
        }
    }
}

fn choose_pivot<T, F>(arr: &mut [T], compar: &F) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    if arr.len() <= 5 {
        return arr.len() / 2;
    }

    let mid = arr.len() / 2;
    let last = arr.len() - 1;

    if compar(&arr[0], &arr[mid]) == Ordering::Greater {
        arr.swap(0, mid);
    }
    if compar(&arr[0], &arr[last]) == Ordering::Greater {
        arr.swap(0, last);
    }
    if compar(&arr[mid], &arr[last]) == Ordering::Greater {
        arr.swap(mid, last);
    }

    mid
}

fn partition<T, F>(arr: &mut [T], compar: &F) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    let pivot = arr.len() - 1;
    let mut i = 0;

    for j in 0..pivot {
        if compar(&arr[j], &arr[pivot]) != Ordering::Greater {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot);
    i
}

fn insertion_sort<T, F>(arr: &mut [T], compar: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && compar(&arr[j - 1], &arr[j]) == Ordering::Greater {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
