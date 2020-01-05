use core::cmp::Ordering;

fn heapify<T, F>(v: &mut [T], i: usize, compare: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = v.len();
    let mut j = i;
    let mut max = i;
    loop {
        let l = j * 2 + 1;
        let r = j * 2 + 2;
        if l < len && compare(&v[max], &v[l]) == Ordering::Less {
            max = l;
        }
        if r < len && compare(&v[max], &v[r]) == Ordering::Less {
            max = r;
        }
        if max == j {
            break;
        }
        v.swap(j, max);
        j = max;
    }
}

pub fn heap_sort<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = v.len();
    for i in (0..(len - 1) / 2 + 1).rev() {
        heapify(v, i, &mut compare);
    }
    for i in (0..len).rev() {
        v.swap(0, i);
        heapify(&mut v[0..i], 0, &mut compare);
    }
}
