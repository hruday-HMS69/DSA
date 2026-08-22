impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut x = n;
        let mut sum = 0;
        let mut product = 1;

        while x > 0 {
            let digit = x % 10;
            sum += digit;
            product *= digit;
            x /= 10;
        }

        n % (sum + product) == 0
    }
}