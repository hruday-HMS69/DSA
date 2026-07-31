impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut freq = [0i32; 26];

        for b in word.bytes() {
            freq[(b - b'a') as usize] += 1;
        }
        let mut v: Vec<i32> = freq.into_iter().collect();
        v.sort_unstable_by(|a, b| b.cmp(a));

        let mut ans = 0;

        for (i, &f) in v.iter().enumerate() {
            if f == 0 {
                break;
            }
            let cost = (i / 8 + 1) as i32;
            ans += f * cost;
        }

        ans
    }
}