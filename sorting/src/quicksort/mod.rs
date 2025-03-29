use std::cmp::Ordering;
pub fn quick_sort<T, F>(arr: &mut [T], compar: &F)
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr, compar);

    quick_sort(&mut arr[..pivot_index], compar);
    quick_sort(&mut arr[pivot_index + 1..], compar);
}

fn partition<T, F>(arr: &mut [T], compar: &F) -> usize
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    let pivot_index = arr.len() - 1;
    let pivot = arr[pivot_index].clone();
    let mut i = 0;

    for j in 0..pivot_index {
        if compar(&arr[j], &pivot) == Ordering::Less {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}
