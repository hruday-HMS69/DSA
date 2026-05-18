use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();

        if n == 1 {
            return 0;
        }

        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();

        for i in 0..n {
            map.entry(arr[i]).or_insert(Vec::new()).push(i);
        }

        let mut queue = VecDeque::new();
        let mut visited = vec![false; n];

        queue.push_back(0usize);
        visited[0] = true;

        let mut steps = 0;

        while !queue.is_empty() {
            let size = queue.len();

            for _ in 0..size {
                let curr = queue.pop_front().unwrap();

                if curr == n - 1 {
                    return steps;
                }

                if curr + 1 < n && !visited[curr + 1] {
                    visited[curr + 1] = true;
                    queue.push_back(curr + 1);
                }

                if curr >= 1 && !visited[curr - 1] {
                    visited[curr - 1] = true;
                    queue.push_back(curr - 1);
                }

                if let Some(neighbors) = map.get_mut(&arr[curr]) {
                    for &idx in neighbors.iter() {
                        if !visited[idx] {
                            visited[idx] = true;
                            queue.push_back(idx);
                        }
                    }

                    neighbors.clear();
                }
            }

            steps += 1;
        }

        -1
    }
}