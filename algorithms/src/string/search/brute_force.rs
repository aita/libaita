pub fn brute_force_search(s: &str, pat: &str) -> Option<usize> {
    let s = s.as_bytes();
    let pat = pat.as_bytes();
    for i in 0..s.len() - pat.len() + 1 {
        let mut j = 0;
        for k in 0..pat.len() {
            if s[i + k] != pat[k] {
                break;
            }
            j += 1;
        }
        if j == pat.len() {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match() {
        assert_eq!(Some(1), brute_force_search("hello world", "ello"));
        assert_eq!(Some(6), brute_force_search("hello world", "world"));
    }

    #[test]
    fn test_unmatch() {
        assert_eq!(None, brute_force_search("hello world", "yellow"));
    }
}
