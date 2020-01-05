use core::cmp::Ordering;

pub fn gnome_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = v.len();
    let mut i = 1;
    while i < len {
        if compare(&v[i], &v[i - 1]) != Ordering::Less {
            i += 1;
        } else {
            v.swap(i - 1, i);
            i -= 1;
            if i == 0 {
                i += 1;
            }
        }
    }
}
