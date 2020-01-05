use core::cmp::Ordering;

pub fn comb_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = v.len();
    let mut h = len;
    let mut swapped = true;
    while h != 1 || swapped {
        h = h * 10 / 13;
        if h < 1 {
            h = 1
        }
        swapped = false;
        for i in 0..len - h {
            if compare(&v[i + h], &v[i]) == Ordering::Less {
                v.swap(i, i + h);
                swapped = true;
            }
        }
    }
}
