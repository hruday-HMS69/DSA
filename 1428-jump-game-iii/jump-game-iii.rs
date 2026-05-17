impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {

        let mut visited = vec![false; arr.len()];

        Self::dfs(&arr, start, &mut visited)
    }

    fn dfs(arr: &Vec<i32>, index: i32, visited: &mut Vec<bool>) -> bool {

        if index < 0 || index >= arr.len() as i32 {
            return false;
        }

        let i = index as usize;

        if visited[i] {
            return false;
        }

        if arr[i] == 0 {
            return true;
        }

        visited[i] = true;

        Self::dfs(arr, index + arr[i], visited)
            || Self::dfs(arr, index - arr[i], visited)
    }
}