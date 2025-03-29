pub fn quick_sort<T, F>(base: &mut [T], compar: &F)
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
}
