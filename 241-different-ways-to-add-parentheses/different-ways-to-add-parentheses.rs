impl Solution {
    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        let mut res = Vec::new();
        let len = input.len();

        for (i, c) in input.chars().enumerate() {
            if c == '*' || c == '-' || c == '+' {
                let left = Self::diff_ways_to_compute(input[..i].to_string());
                let right = Self::diff_ways_to_compute(input[i + 1..].to_string());

                for l in left.iter() {
                    for r in right.iter() {
                        match c {
                            '*' => res.push(l * r),
                            '+' => res.push(l + r),
                            '-' => res.push(l - r),
                            _ => (),
                        }
                    }
                }
            }
        }

        if res.is_empty() {
            if let Ok(num) = input.parse::<i32>() {
                res.push(num);
            }
        }

        res
    }
}
