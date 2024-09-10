impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if original.len() != m as usize * n as usize {
            return vec![];
        }

        let mut ans = vec![vec![0; n as usize]; m as usize];
        let mut idx = 0;
        for a in original {
            ans[idx / n as usize][idx % n as usize] = a;
            idx += 1;
        }

        ans
    }
}