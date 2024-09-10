impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let total_elements = m * n;

        
        if original.len() as i32 != total_elements {
            return vec![];
        }

        
        let mut ans = vec![vec![0; n as usize]; m as usize];

        for (i, &value) in original.iter().enumerate() {
            ans[i / n as usize][i % n as usize] = value;
        }

        ans
    }
}
