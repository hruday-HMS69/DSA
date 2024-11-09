impl Solution {
    pub fn min_end(n: i32, mut x: i32) -> i64 {
        let mut n = n - 1;
        let mut ans: i64 = x as i64;
        
        for i in 0..31 {
            if (x >> i & 1) ^ 1 == 1 {
                ans |= ((n & 1) as i64) << i; 
                n >>= 1;
            }
        }
        
        ans |= (n as i64) << 31;
        ans
    }
}
