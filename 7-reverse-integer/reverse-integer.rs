impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut c_value = x;
        let mut sum = 0;
        let mut digit = c_value % 10;
        c_value /= 10;
        while c_value != 0 {
            sum *= 10;
            sum += digit;
            digit = c_value % 10;
            c_value /= 10;
            println!("{} {} \t\t sum {}", c_value, digit, sum);
        }
        if sum >  214748364 || sum ==  214748364 && digit >  7 
        || sum < -214748364 || sum == -214748364 && digit < -8 {
            0
        } else {
            sum * 10 + digit
        }
    }
}