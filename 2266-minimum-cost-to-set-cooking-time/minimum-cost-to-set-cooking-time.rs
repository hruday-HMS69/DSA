impl Solution {
    pub fn min_cost_set_time(start_at: i32, move_cost: i32, push_cost: i32, target_seconds: i32) -> i32 {
        let m = target_seconds / 60;
        let s = target_seconds % 60;
        std::cmp::min(
            Solution::calculate_cost(m, s, start_at, move_cost, push_cost),
            Solution::calculate_cost(m - 1, s + 60, start_at, move_cost, push_cost),
        )
    }

    fn calculate_cost(m: i32, s: i32, mut prev: i32, move_cost: i32, push_cost: i32) -> i32 {
        if m < 0 || m > 99 || s < 0 || s > 99 {
            return i32::MAX;
        }

        let arr = vec![m / 10, m % 10, s / 10, s % 10];
        let mut i = 0;

        // Skip leading zeros
        while i < 4 && arr[i as usize] == 0 {
            i += 1;
        }

        let mut total_cost = 0;

        for j in i..4 {
            if arr[j as usize] != prev {
                total_cost += move_cost;
            }
            total_cost += push_cost;
            prev = arr[j as usize];
        }

        total_cost
    }
}