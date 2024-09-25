impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::new();
        
        for word in &words {
            trie.insert(word);
        }
        
        words.iter().map(|word| trie.query(word)).collect()
    }
}

struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    score: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),  // Create an array of `None`
            score: 0,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, s: &str) {
        let mut curr = &mut self.root;
        
        for c in s.chars() {
            let index = (c as usize) - ('a' as usize);
            if curr.children[index].is_none() {
                curr.children[index] = Some(Box::new(TrieNode::new()));
            }
            curr = curr.children[index].as_mut().unwrap();
            curr.score += 1;
        }
    }

    fn query(&self, s: &str) -> i32 {
        let mut result = 0;
        let mut curr = &self.root;
        
        for c in s.chars() {
            let index = (c as usize) - ('a' as usize);
            if let Some(node) = &curr.children[index] {
                result += node.score;
                curr = node;
            } else {
                break;
            }
        }
        result
    }
}
