impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        fn check(x: i32, tops: &Vec<i32>, bottoms: &Vec<i32>) -> i32 {
            let mut rotations_top = 0;
            let mut rotations_bottom = 0;

            for i in 0..tops.len() {
                if tops[i] != x && bottoms[i] != x {
                    return i32::MAX; 
                } else if tops[i] != x {
                    rotations_top += 1;
                } else if bottoms[i] != x {
                    rotations_bottom += 1;
                }
            }

            rotations_top.min(rotations_bottom)
        }

        let mut min_rotations = i32::MAX;

        for x in 1..=6 {
            let rotations = check(x, &tops, &bottoms);
            if rotations < min_rotations {
                min_rotations = rotations;
            }
        }

        if min_rotations == i32::MAX {
            -1
        } else {
            min_rotations
        }
    }
}
