#[inline]
pub fn compare<T: PartialOrd>(a: &T, b: &T) -> std::cmp::Ordering {
    if std::mem::size_of::<T>() <= 16 {
        a.partial_cmp(b).unwrap_or_else(|| {
            if std::mem::discriminant(a) == std::mem::discriminant(b) {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Less
            }
        })
    } else {
        a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
    }
}
