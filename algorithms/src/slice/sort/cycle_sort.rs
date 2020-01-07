use core::cmp::Ordering;
use core::mem;
use core::ptr;

pub fn cycle_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = v.len();
    unsafe {
        for start in 0..len - 1 {
            let mut elt = ptr::read(v.get_unchecked(start));
            let mut pos = start;
            for i in start + 1..len {
                if compare(&v[i], &elt) == Ordering::Less {
                    pos += 1;
                }
            }
            if pos == start {
                continue;
            }
            while compare(&v[pos], &elt) == Ordering::Equal {
                pos += 1;
            }
            if pos != start {
                mem::swap(v.get_unchecked_mut(pos), &mut elt);
            }
            while pos != start {
                pos = start;
                for i in start + 1..len {
                    if compare(&v[i], &elt) == Ordering::Less {
                        pos += 1;
                    }
                }
                while compare(&v[pos], &elt) == Ordering::Equal {
                    pos += 1;
                }
                if compare(&v[pos], &elt) != Ordering::Equal {
                    mem::swap(v.get_unchecked_mut(pos), &mut elt);
                }
            }
        }
    }
}
