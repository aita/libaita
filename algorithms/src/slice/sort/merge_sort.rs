use core::cmp::Ordering;
use core::ptr;

fn merge<T, F>(v: &mut [T], buf: &mut Vec<T>, mid: usize, is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    unsafe {
        let len = v.len();
        buf.set_len(len);
        ptr::copy_nonoverlapping(v.get_unchecked(0), buf.get_unchecked_mut(0), len);

        let mut k = 0;
        let mut l = 0;
        let mut r = mid;
        while l < mid && r < len {
            if is_less(&buf[l], &buf[r]) {
                ptr::copy_nonoverlapping(buf.get_unchecked(l), v.get_unchecked_mut(k), 1);
                l += 1;
            } else {
                ptr::copy_nonoverlapping(buf.get_unchecked(r), v.get_unchecked_mut(k), 1);
                r += 1;
            }
            k += 1;
        }
        while l < mid {
            ptr::copy_nonoverlapping(buf.get_unchecked(l), v.get_unchecked_mut(k), 1);
            k += 1;
            l += 1;
        }
        while r < len {
            ptr::copy_nonoverlapping(buf.get_unchecked(r), v.get_unchecked_mut(k), 1);
            k += 1;
            r += 1;
        }
    }
}

fn recurse<T, F>(v: &mut [T], buf: &mut Vec<T>, is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    let len = v.len();
    if len > 1 {
        let mid = len / 2;
        recurse(&mut v[0..mid], buf, is_less);
        recurse(&mut v[mid..], buf, is_less);
        merge(v, buf, mid, is_less);
    }
}

pub fn merge_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut buf: Vec<_> = Vec::with_capacity(v.len());
    recurse(v, &mut buf, &mut |a, b| compare(a, b) == Ordering::Less);
}
