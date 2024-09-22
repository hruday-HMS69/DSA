

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut ans = 1_i64; 
        let mut i = 1;

        while i < k {
            let gap = Self::get_gap(ans, ans + 1, n as i64);
            if i + gap as i32 <= k {
                i += gap as i32;
                ans += 1;
            } else {
                i += 1;
                ans *= 10;
            }
        }

        ans as i32 // Convert to i32 before returning
    }

    fn get_gap(mut a: i64, mut b: i64, n: i64) -> i64 {
        let mut gap = 0;
        while a <= n {
            gap += (b.min(n + 1)) - a;
            a *= 10;
            b *= 10;
        }
        gap
    }
}
