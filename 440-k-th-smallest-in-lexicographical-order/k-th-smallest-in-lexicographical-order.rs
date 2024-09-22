impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut ans = 1_i64; // Start with the first number (1)
        let mut i = 1_i64;   // Change i to i64 to match the type of gap

        while i < k as i64 { // Cast k to i64 to match the type of i
            let gap = Self::get_gap(ans, ans + 1, n as i64); // Calculate the gap
            if i + gap <= k as i64 { // Ensure both i and k are i64 for comparison
                i += gap; // Skip the gap
                ans += 1; // Move to the next lexicographical number
            } else {
                i += 1; // Traverse deeper into the current subtree
                ans *= 10; // Move to the next lexicographical child
            }
        }

        ans as i32 // Return the result as i32
    }

    fn get_gap(mut a: i64, mut b: i64, n: i64) -> i64 {
        let mut gap = 0;
        while a <= n {
            gap += b.min(n + 1) - a; // Count the number of nodes between a and b
            a *= 10; // Move to the next depth level
            b *= 10;
        }
        gap
    }
}
