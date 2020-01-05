use core::cmp::Ordering;

pub fn shaker_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = v.len();
    let mut swapped = true;
    let mut start = 0;
    let mut end = len - 1;
    while swapped {
        swapped = false;
        for i in start..end {
            if compare(&v[i + 1], &v[i]) == Ordering::Less {
                v.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        end -= 1;
        for i in (start..end).rev() {
            if compare(&v[i + 1], &v[i]) == Ordering::Less {
                v.swap(i, i + 1);
                swapped = true;
            }
        }
        start += 1;
    }
}
