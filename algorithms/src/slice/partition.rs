// Lomuto partitioning algorithm
pub fn partition<T, F>(v: &mut [T], mut pred: F) -> usize
where
    F: FnMut(&T) -> bool,
{
    let len = v.len();
    let mut i = 0;
    loop {
        if i == len {
            return i;
        }
        if !pred(&v[i]) {
            break;
        }
        i += 1;
    }
    for j in i..len {
        if pred(&v[j]) {
            v.swap(i, j);
            i += 1
        }
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        let mut v = [5, 1, 2, 3, 4];
        let result = partition(&mut v, |a| *a <= 3);
        assert_eq!(3, result);
    }

    #[test]
    fn test_0() {
        let mut v = [5, 1, 2, 3, 4];
        let result = partition(&mut v, |a| *a > 10);
        assert_eq!(0, result);
    }
}
