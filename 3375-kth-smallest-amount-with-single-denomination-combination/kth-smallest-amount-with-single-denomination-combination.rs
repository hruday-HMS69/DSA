impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        fn gcd(mut a: i64, mut b: i64) -> i64 {
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a
        }

        fn lcm(a: i64, b: i64) -> i64 {
            a / gcd(a, b) * b
        }

        fn count(x: i64, coins: &[i32]) -> i64 {
            let n = coins.len();
            let mut ans = 0i64;

            for mask in 1usize..(1usize << n) {
                let mut multiple = 1i64;
                let mut bits = 0;

                for i in 0..n {
                    if mask & (1 << i) != 0 {
                        bits += 1;
                        multiple = lcm(multiple, coins[i] as i64);

                        if multiple > x {
                            break;
                        }
                    }
                }

                if multiple <= x {
                    if bits % 2 == 1 {
                        ans += x / multiple;
                    } else {
                        ans -= x / multiple;
                    }
                }
            }

            ans
        }

        let k = k as i64;
        let mut lo = 1i64;
        let mut hi = *coins.iter().min().unwrap() as i64 * k;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            if count(mid, &coins) >= k {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        lo
    }
}