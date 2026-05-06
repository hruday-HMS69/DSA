impl Solution {
    pub fn rotate_the_box(r#box: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let rows = r#box.len();
        let cols = r#box[0].len();
        let mut result = vec![vec!['.'; rows]; cols];

        for i in 0..rows {
            let mut empty_spot = cols - 1;
            for j in (0..cols).rev() {
                if r#box[i][j] == '#' {
                    result[empty_spot][rows - 1 - i] = '#';
                    empty_spot -= 1;
                } else if r#box[i][j] == '*' {
                    result[j][rows - 1 - i] = '*';
                    empty_spot = j - 1;
                }
            }
        }

        result
    }
}