impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut res = 0;

        let mut left_gaps = vec![0; n];
        left_gaps[0] = start_time[0];

        for meet in 1..n {
            left_gaps[meet] = std::cmp::max(
                left_gaps[meet - 1],
                start_time[meet] - end_time[meet - 1],
            );
        }

        let mut right_gaps = vec![0; n];
        right_gaps[n - 1] = event_time - end_time[n - 1];

        for meet in (0..n - 1).rev() {
            right_gaps[meet] = std::cmp::max(
                right_gaps[meet + 1],
                start_time[meet + 1] - end_time[meet],
            );
        }

        for meet in 0..n {
            let left_gap = if meet == 0 {
                left_gaps[meet]
            } else {
                start_time[meet] - end_time[meet - 1]
            };

            let right_gap = if meet == n - 1 {
                right_gaps[meet]
            } else {
                start_time[meet + 1] - end_time[meet]
            };

            let mut interval = 0;
            let duration = end_time[meet] - start_time[meet];

            if (meet != 0 && left_gaps[meet - 1] >= duration)
                || (meet != n - 1 && right_gaps[meet + 1] >= duration)
            {
                interval = duration;
            }

            res = std::cmp::max(res, left_gap + interval + right_gap);
        }

        res
    }
}
