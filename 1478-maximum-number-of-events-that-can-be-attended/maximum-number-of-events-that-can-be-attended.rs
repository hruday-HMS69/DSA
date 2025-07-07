use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort(); 

        let mut min_heap = BinaryHeap::new(); 
        let mut day = 0;
        let mut i = 0;
        let n = events.len();
        let mut count = 0;

        
        let last_day = events.iter().map(|e| e[1]).max().unwrap();

        for day in 1..=last_day {
            
            while i < n && events[i][0] == day {
                min_heap.push(Reverse(events[i][1])); 
                i += 1;
            }

            while let Some(&Reverse(end)) = min_heap.peek() {
                if end < day {
                    min_heap.pop();
                } else {
                    break;
                }
            }

            if let Some(_) = min_heap.pop() {
                count += 1;
            }
        }

        count
    }
}
