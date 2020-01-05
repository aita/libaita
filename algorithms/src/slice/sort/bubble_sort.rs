use core::cmp::Ordering;

pub fn bubble_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = v.len();
    for i in 0..len - 1 {
        for j in 0..len - i - 1 {
            if compare(&v[j + 1], &v[j]) == Ordering::Less {
                v.swap(j, j + 1)
            }
        }
    }
}
