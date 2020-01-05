use core::cmp::Ordering;
use core::ptr;

pub fn shell_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = v.len();
    let mut gap = len / 2;
    while gap > 0 {
        for i in gap..len {
            unsafe {
                let tmp = ptr::read(v.get_unchecked(i));
                let mut j = i;
                while j >= gap && compare(&tmp, &v[j - gap]) == Ordering::Less {
                    ptr::copy_nonoverlapping(v.get_unchecked(j - gap), v.get_unchecked_mut(j), 1);
                    j -= gap;
                }
                ptr::write(v.get_unchecked_mut(j), tmp);
            }
        }
        gap /= 2;
    }
}
