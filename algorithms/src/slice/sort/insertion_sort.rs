use core::cmp::Ordering;
use core::ptr;

pub fn insertion_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    unsafe {
        for i in 1..v.len() {
            let tmp = ptr::read(v.get_unchecked(i));
            let mut j = i;
            while j > 0 && compare(&tmp, &v[j - 1]) == Ordering::Less {
                ptr::copy_nonoverlapping(v.get_unchecked(j - 1), v.get_unchecked_mut(j), 1);
                j -= 1;
            }
            ptr::write(v.get_unchecked_mut(j), tmp);
        }
    }
}
