use core::cmp::Ordering;

fn partition<T, F>(v: &mut [T], is_less: &mut F) -> (usize, usize)
where
    F: FnMut(&T, &T) -> bool,
{
    let len = v.len();
    let end = len - 1;
    if is_less(&v[end], &v[0]) {
        v.swap(0, end)
    }
    let mut j = 1;
    let mut g = end - 1;
    let mut k = 1;
    while k <= g {
        if is_less(&v[k], &v[0]) {
            v.swap(k, j);
            j += 1;
        } else if !is_less(&v[k], &v[end]) {
            while is_less(&v[end], &v[g]) && k < g {
                g -= 1;
            }
            v.swap(k, g);
            g -= 1;
            if is_less(&v[k], &v[0]) {
                v.swap(k, j);
                j += 1;
            }
        }
        k += 1;
    }
    j -= 1;
    g += 1;
    v.swap(0, j);
    v.swap(end, g);
    (j, g)
}

fn recurse<T, F>(v: &mut [T], is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    if v.len() < 2 {
        return;
    }
    let (l, r) = partition(v, is_less);
    recurse(&mut v[0..l], is_less);
    recurse(&mut v[l + 1..r], is_less);
    recurse(&mut v[r + 1..], is_less);
}

pub fn dual_quick_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    recurse(v, &mut |a, b| compare(a, b) == Ordering::Less);
}
