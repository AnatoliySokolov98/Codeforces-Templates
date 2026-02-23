pub struct SegmentTree {
    nums: Vec<i32>,
    length: usize,
}

impl SegmentTree {
    pub fn new(length: usize) -> Self {
        SegmentTree {
            nums: vec![0; length * 4],
            length: length,
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

    fn build(&mut self, nums: &[i32], curr_index: usize, left_bound: usize, right_bound: usize) {
        if left_bound == right_bound {
            self.nums[curr_index] = nums[left_bound];
            return;
        }

        let pivot = (left_bound + right_bound) / 2;
        let left_index = curr_index * 2 + 1;
        let right_index = curr_index * 2 + 2;

        self.build(nums, left_index, left_bound, pivot);
        self.build(nums, right_index, pivot + 1, right_bound);
        self.nums[curr_index] = self.nums[left_index].max(self.nums[right_index]);
    }

    pub fn update(&mut self, target_index: usize, val: i32) {
        self.update_helper(target_index, val, 0, 0, self.length - 1);
    }

    fn update_helper(
        &mut self,
        target_index: usize,
        val: i32,
        curr_index: usize,
        left_bound: usize,
        right_bound: usize,
    ) {
        if left_bound == right_bound {
            self.nums[curr_index] = val;
            return;
        }

        let pivot = (left_bound + right_bound) / 2;
        let left_index = curr_index * 2 + 1;
        let right_index = curr_index * 2 + 2;
        if target_index <= pivot {
            self.update_helper(target_index, val, left_index, left_bound, pivot);
        } else {
            self.update_helper(target_index, val, right_index, pivot + 1, right_bound);
        }
        self.nums[curr_index] = self.nums[left_index].max(self.nums[right_index]);
    }

    pub fn query(&self, left_target: usize, right_target: usize) -> i32 {
        self.query_helper(left_target, right_target, 0, 0, self.length - 1)
    }

    fn query_helper(
        &self,
        left_target: usize,
        right_target: usize,
        curr_index: usize,
        left_bound: usize,
        right_bound: usize,
    ) -> i32 {
        if left_target <= left_bound && right_target >= right_bound {
            return self.nums[curr_index];
        }
        if left_target > right_bound || right_target < left_bound {
            return 0;
        }

        let pivot = (left_bound + right_bound) / 2;
        let left_index = curr_index * 2 + 1;
        let right_index = curr_index * 2 + 2;

        self.query_helper(left_target, right_target, left_index, left_bound, pivot)
            .max(self.query_helper(
                left_target,
                right_target,
                right_index,
                pivot + 1,
                right_bound,
            ))
    }
}


