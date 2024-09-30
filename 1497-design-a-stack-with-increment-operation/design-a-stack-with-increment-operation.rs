struct CustomStack {
    stack: Vec<i32>,
    add: Vec<i32>,
    i: usize,
}

impl CustomStack {
    fn new(maxSize: i32) -> Self {
        CustomStack {
            stack: vec![0; maxSize as usize], 
            add: vec![0; maxSize as usize],   
            i: 0,
        }
    }
    fn push(&mut self, x: i32) { 
        if self.i < self.stack.len() {
            self.stack[self.i] = x;
            self.i += 1;
        }
    }
     fn pop(&mut self) -> i32 { 
        if self.i == 0 {
            return -1;
        }
        self.i -= 1;
        let ans = self.stack[self.i] + self.add[self.i];
        if self.i > 0 {
            self.add[self.i - 1] += self.add[self.i];
        }
        self.add[self.i] = 0;
        ans
    }

    fn increment(&mut self, k: i32, val: i32) { 
        if self.i > 0 {
            let idx = std::cmp::min(k as usize, self.i) - 1;
            self.add[idx] += val;
        }
    }
}