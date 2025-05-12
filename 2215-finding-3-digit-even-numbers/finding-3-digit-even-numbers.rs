use std::collections::HashSet;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut freq = [0; 10];
        for &d in &digits {
            freq[d as usize] += 1;
        }

        let mut result = HashSet::new();

        for i in 1..=9 { 
            for j in 0..=9 { 
                for k in (0..=9).step_by(2) { 
                    let mut count = [0; 10];
                    count[i] += 1;
                    count[j] += 1;
                    count[k] += 1;

                    let mut valid = true;
                    for d in 0..10 {
                        if count[d] > freq[d] {
                            valid = false;
                            break;
                        }
                    }

                    if valid {
                        let number = i * 100 + j * 10 + k;
                        result.insert(number as i32); 
                    }
                }
            }
        }

        let mut output: Vec<i32> = result.into_iter().collect();
        output.sort();
        output
    }
}
