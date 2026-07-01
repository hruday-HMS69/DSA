use std::collections::VecDeque;

impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
         let mut dist = vec![vec![-1; n]; n];
        let mut q = VecDeque::new();

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    dist[i][j] = 0;
                    q.push_back((i as i32, j as i32));
                }
            }
        }

        let dirs = [(1,0), (-1,0), (0,1), (0,-1)];

        while let Some((x, y)) = q.pop_front() {
            for (dx, dy) in dirs {
                let nx = x + dx;
                let ny = y + dy;

                if nx < 0 || ny < 0 || nx >= n as i32 || ny >= n as i32 {
                    continue;
                }

                let (ux, uy) = (nx as usize, ny as usize);

                if dist[ux][uy] != -1 {
                    continue;
                }

                dist[ux][uy] = dist[x as usize][y as usize] + 1;
                q.push_back((nx, ny));
            }
        }
        let mut low = 0;
        let mut high = 2 * n as i32;
        let mut ans = 0;

        while low <= high {
            let mid = (low + high) / 2;

            if Self::can_reach(&dist, mid) {
                ans = mid;
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        ans
    }
 fn can_reach(dist: &Vec<Vec<i32>>, safe: i32) -> bool {
        let n = dist.len();

        if dist[0][0] < safe || dist[n - 1][n - 1] < safe {
            return false;
        }

        let mut vis = vec![vec![false; n]; n];
        let mut q = VecDeque::new();

        q.push_back((0usize, 0usize));
        vis[0][0] = true;

        let dirs = [(1,0), (-1,0), (0,1), (0,-1)];

        while let Some((x, y)) = q.pop_front() {
            if x == n - 1 && y == n - 1 {
                return true;
            }

            for (dx, dy) in dirs {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx < 0 || ny < 0 || nx >= n as i32 || ny >= n as i32 {
                    continue;
                }

                let (ux, uy) = (nx as usize, ny as usize);

                if vis[ux][uy] {
                    continue;
                }

                if dist[ux][uy] < safe {
                    continue;
                }

                vis[ux][uy] = true;
                q.push_back((ux, uy));
            }
        }

        false
    }
}