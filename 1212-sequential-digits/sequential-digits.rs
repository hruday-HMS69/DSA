impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let s = b"123456789";
        let mut ans = Vec::new();

        let low_len = low.to_string().len();
        let high_len = high.to_string().len().min(9);

        for len in low_len..=high_len {
            for start in 0..=(9 - len) {
                let num = std::str::from_utf8(&s[start..start + len])
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();

                if num >= low && num <= high {
                    ans.push(num);
                }
            }
        }

        ans
    }
}