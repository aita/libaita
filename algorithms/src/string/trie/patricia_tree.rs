use core::mem;

#[derive(Debug)]
struct Node {
    prefix: Vec<u8>,
    children: Vec<Node>,
    is_end: bool,
}

fn longest_prefix_match(a: &[u8], b: &[u8]) -> usize {
    let len = a.len();
    for i in 0..len {
        if a[i] != b[i] {
            return i;
        }
    }
    len
}

impl Node {
    fn new(prefix: Vec<u8>, is_end: bool) -> Self {
        Node {
            prefix: prefix,
            children: Vec::new(),
            is_end: is_end,
        }
    }

    fn search(&self, word: &[u8]) -> bool {
        match self
            .children
            .binary_search_by(|node| node.prefix[0].cmp(&word[0]))
        {
            Err(_) => false,
            Ok(idx) => {
                let node = &self.children[idx];
                let m = longest_prefix_match(&node.prefix[..], word);
                if m < node.prefix.len() {
                    return false;
                }
                if m == word.len() {
                    return node.is_end;
                }
                node.search(&word[m..])
            }
        }
    }

    fn insert(&mut self, word: &[u8]) {
        if word.len() == 0 {
            self.is_end = true;
            return;
        }

        match self
            .children
            .binary_search_by(|node| node.prefix[0].cmp(&word[0]))
        {
            Err(idx) => {
                let new_node = Node::new(Vec::from(word), true);
                self.children.insert(idx, new_node);
            }
            Ok(idx) => {
                let node = &mut self.children[idx];
                let m = longest_prefix_match(&node.prefix[..], word);
                if m < node.prefix.len() {
                    // split the node
                    let mut new_node = Node::new(Vec::from(&node.prefix[m..]), node.is_end);
                    mem::swap(&mut new_node.children, &mut node.children);
                    node.prefix.truncate(m);
                    node.children.push(new_node);
                    node.is_end = false;
                }
                node.insert(&word[m..]);
            }
        }
    }
}

pub struct PatriciaTree {
    root: Node,
}

impl PatriciaTree {
    pub fn new() -> PatriciaTree {
        PatriciaTree {
            root: Node::new(Vec::new(), false),
        }
    }

    pub fn search(&self, word: &str) -> bool {
        self.root.search(word.as_bytes())
    }

    pub fn insert(&mut self, word: &str) {
        self.root.insert(word.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut trie = PatriciaTree::new();
        trie.insert("red");
        trie.insert("redbull");
        trie.insert("read");
        trie.insert("led");
        trie.insert("red");

        assert_eq!(trie.search("red"), true);
        assert_eq!(trie.search("read"), true);
        assert_eq!(trie.search("redbull"), true);
        assert_eq!(trie.search("re"), false);
        assert_eq!(trie.search("reddit"), false);
        assert_eq!(trie.search("led"), true);
    }
}
