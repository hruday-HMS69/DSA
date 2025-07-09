use std::collections::VecDeque;
impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, mut start_time: Vec<i32>, mut end_time: Vec<i32>) -> i32 {
        if event_time > *end_time.last().unwrap() {
            start_time.push(event_time);
            end_time.push(event_time);
        }

        let n = start_time.len();
        let mut max_sum = 0;
        let mut win_sum = 0;
        let mut pos = 0;
        let mut last_end = 0;
        let mut dq: VecDeque<i32> = VecDeque::new();

        while pos < n {
            let gap = start_time[pos] - last_end;
            win_sum += gap;
            max_sum = max_sum.max(win_sum);
            dq.push_back(gap);

            if dq.len() > k as usize {
                if let Some(front) = dq.pop_front() {
                    win_sum -= front;
                }
            }

            last_end = end_time[pos];
            pos += 1;
        }

        max_sum
    }
}