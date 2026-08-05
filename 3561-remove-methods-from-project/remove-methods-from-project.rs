impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;

        let mut graph = vec![Vec::new(); n];
        for e in invocations {
            graph[e[0] as usize].push(e[1] as usize);
        }
        let mut suspicious = vec![false; n];
        let mut stack = vec![k];
        suspicious[k] = true;

        while let Some(node) = stack.pop() {
            for &next in &graph[node] {
                if !suspicious[next] {
                    suspicious[next] = true;
                    stack.push(next);
                }
            }
        }

   
        for i in 0..n {
            if !suspicious[i] {
                for &next in &graph[i] {
                    if suspicious[next] {
                        return (0..n).map(|x| x as i32).collect();
                    }
                }
            }
        }

        let mut ans = Vec::new();
        for i in 0..n {
            if !suspicious[i] {
                ans.push(i as i32);
            }
        }

        ans
    }
}