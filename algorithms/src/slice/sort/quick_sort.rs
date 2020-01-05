use core::cmp::Ordering;

fn partition<T, F>(v: &mut [T], is_less: &mut F) -> usize
where
    F: FnMut(&T, &T) -> bool,
{
    let mut mid = 0;
    let len = v.len();
    let end = len - 1;
    for i in 0..end {
        if is_less(&v[i], &v[end]) {
            v.swap(i, mid);
            mid += 1;
        }
    }
    v.swap(mid, end);
    mid
}

fn recurse<T, F>(v: &mut [T], is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    if v.len() < 2 {
        return;
    }
    let mid = partition(v, is_less);
    recurse(&mut v[0..mid], is_less);
    recurse(&mut v[mid + 1..], is_less);
}

pub fn quick_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    recurse(v, &mut |a, b| compare(a, b) == Ordering::Less);
}
