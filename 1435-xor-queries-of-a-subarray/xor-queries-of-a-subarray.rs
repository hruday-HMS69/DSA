impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
       
        let mut prefix_xor = vec![0; arr.len() + 1];
        for i in 0..arr.len() {
            prefix_xor[i + 1] = prefix_xor[i] ^ arr[i];
        }
        
        
        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            let left = query[0] as usize;
            let right = query[1] as usize;
            let xor_result = prefix_xor[right + 1] ^ prefix_xor[left];
            result.push(xor_result);
        }
        
        result
    }
}
