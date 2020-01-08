struct Node {
    element: u8,
    children: Vec<Node>,
    is_end: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            element: 0,
            children: Vec::new(),
            is_end: false,
        }
    }
}

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Trie {
        Trie { root: Node::new() }
    }

    pub fn insert(&mut self, s: &str) {
        let mut node = &mut self.root;
        for c in s.as_bytes() {
            match node.children.binary_search_by(|node| node.element.cmp(&c)) {
                Ok(idx) => {
                    node = &mut node.children[idx];
                }
                Err(idx) => {
                    let mut new_node = Node::new();
                    new_node.element = *c;
                    node.children.insert(idx, new_node);
                    node = &mut node.children[idx];
                }
            }
        }
        node.is_end = true;
    }

    pub fn search(&self, s: &str) -> bool {
        let mut node = &self.root;
        for c in s.as_bytes() {
            match node.children.binary_search_by(|node| node.element.cmp(&c)) {
                Ok(idx) => {
                    node = &node.children[idx];
                }
                Err(_) => return false,
            }
        }
        return node.is_end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut trie = Trie::new();
        trie.insert("red");
        trie.insert("redbull");
        trie.insert("read");

        assert_eq!(trie.search("red"), true);
        assert_eq!(trie.search("read"), true);
        assert_eq!(trie.search("redbull"), true);
        assert_eq!(trie.search("re"), false);
        assert_eq!(trie.search("reddit"), false);
    }
}
