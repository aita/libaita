use core::cmp::Ordering;
use rand::Rng;

pub fn max<'a, T, F>(v: &'a [T], compare: F) -> &T
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut max = 0;
    for i in 1..v.len() {
        if compare(&v[max], &v[i]) == Ordering::Less {
            max = i;
        }
    }
    &v[max]
}

pub fn min<'a, T, F>(v: &'a [T], compare: F) -> &T
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut min = 0;
    for i in 1..v.len() {
        if compare(&v[i], &v[min]) == Ordering::Less {
            min = i;
        }
    }
    &v[min]
}

pub fn nth_element<T, F>(v: &mut [T], n: usize, mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = v.len();
    for i in 0..n {
        let mut k = i;
        for j in i..len {
            if compare(&v[j], &v[k]) == Ordering::Less {
                k = j;
            }
        }
        v.swap(i, k);
    }
}

// Reservoir sampling
pub fn reservoir_sampling<'a, T, R>(v: &'a [T], rng: &mut R, k: usize) -> Vec<&'a T>
where
    R: Rng,
{
    let mut reservoir: Vec<&'a T> = Vec::new();
    for i in 0..k {
        reservoir.push(&v[i]);
    }
    for i in k..v.len() {
        let j = rng.gen::<usize>() % (i + 1);
        if j < k {
            reservoir[j] = &v[i]
        }
    }
    reservoir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        let v = vec![10, 5, 4, 2, 0];
        assert_eq!(10, *max(&v, |a, b| a.cmp(b)));
    }

    #[test]
    fn test_min() {
        let v = vec![10, 5, 4, 2, 0];
        assert_eq!(0, *min(&v, |a, b| a.cmp(b)));
    }

    #[test]
    fn test_nth_element() {
        let mut v = vec![10, 5, -1, 4, 2, 0];
        nth_element(&mut v, 3, |a, b| a.cmp(b));
        assert_eq!([-1, 0, 2], v[0..3]);
    }
}
