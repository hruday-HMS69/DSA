use std::collections::HashMap;

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    freq_map: HashMap<i32, i32>,
}

impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut freq_map = HashMap::new();
        for &num in &nums2 {
            *freq_map.entry(num).or_insert(0) += 1;
        }
        FindSumPairs {
            nums1,
            nums2,
            freq_map,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let i = index as usize;
        let old_val = self.nums2[i];

        if let Some(count) = self.freq_map.get_mut(&old_val) {
            *count -= 1;
            if *count == 0 {
                self.freq_map.remove(&old_val);
            }
        }

        self.nums2[i] += val;
        *self.freq_map.entry(self.nums2[i]).or_insert(0) += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        let mut result = 0;
        for &num in &self.nums1 {
            let target = tot - num;
            if let Some(&count) = self.freq_map.get(&target) {
                result += count;
            }
        }
        result
    }
}
