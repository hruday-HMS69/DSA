impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::new();
        
        for word in &words {
            trie.insert(word);
        }
        
        words.iter().map(|word| trie.query(word)).collect()
    }
}

struct Trie {
    nodes: Vec<Vec<i32>>,
}

impl Trie {
    fn new() -> Self {
        let mut nodes = Vec::new();
        nodes.push(vec![0; 26]); 
        Trie { nodes }
    }

    fn insert(&mut self, s: &String) {
        let mut curr = 0;
        for c in s.chars() {
            let index = (c as usize) - ('a' as usize);
            if self.nodes[curr][index] == 0 {
                self.nodes[curr][index] = self.create_node() as i32;
            }
            curr = self.nodes[curr][index] as usize;
            self.nodes[curr][26] += 1; 
        }
    }

    fn query(&self, s: &String) -> i32 {
        let mut result = 0;
        let mut curr = 0;
        for c in s.chars() {
            let index = (c as usize) - ('a' as usize);
            curr = self.nodes[curr][index] as usize;
            result += self.nodes[curr][26]; 
        }
        result
    }

    fn create_node(&mut self) -> usize {
        self.nodes.push(vec![0; 27]); 
        self.nodes.len() - 1
    }
}
