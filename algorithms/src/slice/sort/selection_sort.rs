use core::cmp::Ordering;

pub fn selection_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = v.len();
    for i in 0..len - 1 {
        let mut k = i;
        for j in i..len {
            if compare(&v[j], &v[k]) == Ordering::Less {
                k = j;
            }
        }
        v.swap(i, k);
    }
}
