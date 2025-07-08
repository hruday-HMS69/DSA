impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::cmp::max;

        events.sort_by_key(|e| e[1]);
        let n = events.len();

        let ends: Vec<i32> = events.iter().map(|e| e[1]).collect();

        fn find_last_non_overlap(events: &Vec<Vec<i32>>, ends: &Vec<i32>, start: i32) -> usize {
            let mut left = 0;
            let mut right = ends.len();
            while left < right {
                let mid = (left + right) / 2;
                if ends[mid] < start {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            if left == 0 {
                0
            } else {
                left
            }
        }

        let mut dp = vec![vec![0; (k + 1) as usize]; n + 1];

        for i in 1..=n {
            let (start, end, value) = (events[i - 1][0], events[i - 1][1], events[i - 1][2]);
            let l = find_last_non_overlap(&events, &ends, start);
            for j in 1..=k as usize {
                dp[i][j] = max(
                    dp[i - 1][j],                          
                    dp[l][j - 1] + value                   
                );
            }
        }

        dp[n][k as usize]
    }
}
