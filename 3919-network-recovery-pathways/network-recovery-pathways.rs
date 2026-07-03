use std::collections::VecDeque;

impl Solution {
    pub fn find_max_path_score(
        edges: Vec<Vec<i32>>,
        online: Vec<bool>,
        k: i64,
    ) -> i32 {
        let n = online.len();
        let mut graph = vec![Vec::<(usize, i32)>::new(); n];
        let mut indegree = vec![0; n];
        let mut max_cost = 0;

        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2];
            graph[u].push((v, w));
            indegree[v] += 1;
            max_cost = max_cost.max(w);
        }

        let mut q = VecDeque::new();
        for i in 0..n {
            if indegree[i] == 0 {
                q.push_back(i);
            }
        }

        let mut topo = Vec::new();
        while let Some(u) = q.pop_front() {
            topo.push(u);
            for &(v, _) in &graph[u] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    q.push_back(v);
                }
            }
        }

        fn check(
            limit: i32,
            graph: &Vec<Vec<(usize, i32)>>,
            topo: &Vec<usize>,
            online: &Vec<bool>,
            k: i64,
        ) -> bool {
            let n = graph.len();
            let inf = i64::MAX / 4;
            let mut dist = vec![inf; n];
            dist[0] = 0;

            for &u in topo {
                if dist[u] == inf {
                    continue;
                }

                if u != 0 && u != n - 1 && !online[u] {
                    continue;
                }

                for &(v, w) in &graph[u] {
                    if w < limit {
                        continue;
                    }

                    if v != n - 1 && !online[v] {
                        continue;
                    }

                    let nd = dist[u] + w as i64;
                    if nd < dist[v] {
                        dist[v] = nd;
                    }
                }
            }

            dist[n - 1] <= k
        }

        let mut lo = 0;
        let mut hi = max_cost;
        let mut ans = -1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if check(mid, &graph, &topo, &online, k) {
                ans = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        ans
    }
}