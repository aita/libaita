use core::cmp::Ordering;

fn recurse<T, F>(v: &mut [T], is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    let len = v.len();
    let end = len - 1;
    if is_less(&v[end], &v[0]) {
        v.swap(0, end);
    }
    if len >= 3 {
        let t = len / 3;
        recurse(&mut v[0..len - t], is_less);
        recurse(&mut v[t..len], is_less);
        recurse(&mut v[0..len - t], is_less);
    }
}

pub fn stooge_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    recurse(v, &mut |a, b| compare(a, b) == Ordering::Less);
}
