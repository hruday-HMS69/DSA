impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut memo = vec![None; n + 1];

        fn dfs(i: usize, stones: &Vec<i32>, memo: &mut Vec<Option<i32>>) -> i32 {
            if i >= stones.len() {
                return 0;
            }

            if let Some(v) = memo[i] {
                return v;
            }

            let mut best = i32::MIN;
            let mut sum = 0;

            for k in 0..3 {
                if i + k >= stones.len() {
                    break;
                }

                sum += stones[i + k];
                best = best.max(sum - dfs(i + k + 1, stones, memo));
            }

            memo[i] = Some(best);
            best
        }

        let diff = dfs(0, &stone_value, &mut memo);

        if diff > 0 {
            "Alice".to_string()
        } else if diff < 0 {
            "Bob".to_string()
        } else {
            "Tie".to_string()
        }
    }
}