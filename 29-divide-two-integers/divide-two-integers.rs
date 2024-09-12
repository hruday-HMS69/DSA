impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::min_value() && divisor == -1
            { return i32::max_value() }
        let mut dvd = i64::abs(dividend as i64);
        let dvs = i64::abs(divisor as i64);
        let sign: i64 = if (dividend > 0) ^ (divisor > 0) { -1 } else { 1 };
        let mut ans = 0;
        while dvd >= dvs {
            let mut tmp = dvs;
            let mut shift: i64 = 1;
            while dvd >= (tmp << 1) {
                tmp <<= 1;
                shift <<= 1;
            }
            dvd -= tmp;
            ans += shift;
        }
        (sign * ans) as i32
    }
}
