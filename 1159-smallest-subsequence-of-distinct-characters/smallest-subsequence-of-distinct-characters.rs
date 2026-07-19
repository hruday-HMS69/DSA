impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut last = [0usize; 26];
        for (i, &ch) in chars.iter().enumerate() {
            last[(ch as u8 - b'a') as usize] = i;
        }

        let mut visited = [false; 26];
        let mut stack: Vec<char> = Vec::new();

        for (i, &ch) in chars.iter().enumerate() {
            let idx = (ch as u8 - b'a') as usize;

            if visited[idx] {
                continue;
            }

            while let Some(&top) = stack.last() {
                let top_idx = (top as u8 - b'a') as usize;

                if top > ch && last[top_idx] > i {
                    stack.pop();
                    visited[top_idx] = false;
                } else {
                    break;
                }
            }

            stack.push(ch);
            visited[idx] = true;
        }

        stack.into_iter().collect()
    }
}