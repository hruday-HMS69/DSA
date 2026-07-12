impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;

            graph[a].push(b);
            graph[b].push(a);
        }

        let mut visited = vec![false; n];
        let mut ans = 0;

        for i in 0..n {
            if !visited[i] {
                let mut stack = vec![i];
                let mut vertices = 0;
                let mut degree_sum = 0;

                visited[i] = true;

                while let Some(node) = stack.pop() {
                    vertices += 1;
                    degree_sum += graph[node].len();

                    for &next in &graph[node] {
                        if !visited[next] {
                            visited[next] = true;
                            stack.push(next);
                        }
                    }
                }

                let edges_count = degree_sum / 2;
                if edges_count == vertices * (vertices - 1) / 2 {
                    ans += 1;
                }
            }
        }

        ans
    }
}