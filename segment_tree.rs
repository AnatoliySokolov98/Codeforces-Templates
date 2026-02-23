struct SegmentTree {
    nums: Vec<i32>,
    length: usize,
}

impl SegmentTree {
    pub fn new(length: usize) -> Self {
        SegmentTree {
            nums: vec![0; length * 4],
            length,
        }
    }

    pub fn from(nums: &[i32]) -> Self {
        let mut res = SegmentTree {
            nums: vec![0; nums.len() * 4],
            length: nums.len(),
        };
        res.build(nums, 0, 0, nums.len() - 1);
        res
    }

    fn build(
        &mut self,
        nums: &[i32],
        current_index: usize,
        left_bound: usize,
        right_bound: usize,
    ) {
        if left_bound == right_bound {
            self.nums[current_index] = nums[left_bound];
            return;
        }
        let pivot = (left_bound + right_bound) / 2;
        let left_index = current_index * 2 + 1;
        let right_index = current_index * 2 + 2;
        self.build(nums, left_index, left_bound, pivot);
        self.build(nums, right_index, pivot + 1, right_bound);
        self.nums[current_index] = self.nums[left_index] + self.nums[right_index];
    }

    pub fn update(&mut self, target_index: usize, val: i32) {
        self.update_helper(target_index, val, 0, 0, self.length - 1)
    }

    fn update_helper(
        &mut self,
        target_index: usize,
        val: i32,
        current_index: usize,
        left_bound: usize,
        right_bound: usize,
    ) {
        if left_bound == right_bound {
            self.nums[current_index] = val;
            return;
        }
        let pivot = (left_bound + right_bound) / 2;
        let left_index = current_index * 2 + 1;
        let right_index = current_index * 2 + 2;
        if target_index <= pivot {
            self.update_helper(target_index, val, left_index, left_bound, pivot);
        } else {
            self.update_helper(target_index, val, right_index, pivot + 1, right_bound);
        }
        self.nums[current_index] = self.nums[left_index] + self.nums[right_index];
    }

    pub fn query(&self, target_left: usize, target_right: usize) -> i32 {
        self.query_helper(target_left, target_right, 0, 0, self.length - 1)
    }

    fn query_helper(
        &self,
        target_left: usize,
        target_right: usize,
        current_index: usize,
        left_bound: usize,
        right_bound: usize,
    ) -> i32 {
        if target_left > right_bound || target_right < left_bound {
            return 0;
        }
        if target_left <= left_bound && target_right >= right_bound {
            return self.nums[current_index];
        }
        let pivot = (left_bound + right_bound) / 2;
        let left_index = current_index * 2 + 1;
        let right_index = current_index * 2 + 2;
        self.query_helper(target_left, target_right, left_index, left_bound, pivot)
            + self.query_helper(
                target_left,
                target_right,
                right_index,
                pivot + 1,
                right_bound,
            )
    }
}
