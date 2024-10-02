impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
         let mut unique_sorted: Vec<i32> = arr.clone().into_iter().collect();
        unique_sorted.sort();
        unique_sorted.dedup(); 
        let rank_map: std::collections::HashMap<i32, i32> = unique_sorted
            .iter()
            .enumerate()
            .map(|(index, &value)| (value, (index + 1) as i32)) 
            .collect();
        arr.into_iter().map(|x| *rank_map.get(&x).unwrap()).collect()
    }
}