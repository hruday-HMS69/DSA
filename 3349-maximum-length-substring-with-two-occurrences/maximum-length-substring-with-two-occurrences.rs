impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut count =[0;26];
        let mut left = 0;
        let mut ans = 0;
        for right in 0..bytes.len(){
            let idx = (bytes[right]-b'a')as usize;
            count[idx]+= 1;
             while count[idx] > 2 {
                let left_idx = (bytes[left] - b'a') as usize;
                count[left_idx] -= 1;
                left += 1;
            }
                        ans = ans.max((right - left + 1) as i32);

        }
        ans
    }
}