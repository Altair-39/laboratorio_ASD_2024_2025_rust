use std::cmp::Ordering;
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

#[inline]
pub fn compare_f64_quicksort(a: f64, b: f64) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

#[inline]
pub fn compare_str_lex(a: &str, b: &str) -> Ordering {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    // Compare byte-by-byte (LLVM optimizes this well)
    for (a_byte, b_byte) in a_bytes.iter().zip(b_bytes) {
        match a_byte.cmp(b_byte) {
            Ordering::Equal => continue,
            non_eq => return non_eq,
        }
    }

    // If all compared bytes are equal, the shorter string is "Less"
    a_bytes.len().cmp(&b_bytes.len())
}
