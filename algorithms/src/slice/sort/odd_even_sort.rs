use core::cmp::Ordering;

pub fn odd_even_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = v.len();
    loop {
        let mut changed = false;
        for i in (0..len - 1).step_by(2) {
            if compare(&v[i + 1], &v[i]) == Ordering::Less {
                v.swap(i, i + 1);
                changed = true;
            }
        }
        for i in (1..len - 1).step_by(2) {
            if compare(&v[i + 1], &v[i]) == Ordering::Less {
                v.swap(i, i + 1);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}
