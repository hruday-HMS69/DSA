impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        let mut seen = vec![vec![vec![false; 105]; n]; m];

        fn dfs(
            x: i32,
            y: i32,
            health: i32,
            grid: &Vec<Vec<i32>>,
            seen: &mut Vec<Vec<Vec<bool>>>,
        ) -> bool {
            let m = grid.len() as i32;
            let n = grid[0].len() as i32;

            if health <= 0 {
                return false;
            }

            let x = x as usize;
            let y = y as usize;
            let h = health as usize;

            if seen[x][y][h] {
                return false;
            }

            seen[x][y][h] = true;

            let mut curr = health;
            if grid[x][y] == 1 {
                curr -= 1;
            }

            if curr <= 0 {
                return false;
            }

            if x as i32 == m - 1 && y as i32 == n - 1 {
                return true;
            }

            let dirs = [(1,0),(-1,0),(0,1),(0,-1)];

            for (dx, dy) in dirs {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && ny >= 0 && nx < m && ny < n {
                    if dfs(nx, ny, curr, grid, seen) {
                        return true;
                    }
                }
            }

            false
        }

        dfs(0, 0, health, &grid, &mut seen)
    }
}