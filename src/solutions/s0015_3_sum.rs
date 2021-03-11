use crate::solutions::Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        if nums.len() < 3 { return result; }
        let mut nums = nums;
        nums.sort();
        let mut first = 0;
        while first < nums.len() - 2 && nums[first] <= 0 {
            let mut second = first + 1;
            let mut third = nums.len() - 1;
            while second < third {
                match nums[first] + nums[second] + nums[third] {
                    0 => {
                        result.push(vec![nums[first], nums[second], nums[third]]);
                        second = Solution::next_unique(&nums, second, true);
                    }
                    x if x < 0 => {
                        second = Solution::next_unique(&nums, second, true);
                    }
                    _ => {
                        third = Solution::next_unique(&nums, third, false);
                    }
                }
            }
            first = Solution::next_unique(&nums, first, true);
        }
        result
    }

    #[inline(always)]
    fn next_unique(nums: &Vec<i32>, index: usize, forward: bool) -> usize {
        let current = nums[index];
        let mut i = index;
        while i < nums.len() && nums[i] == current {
            i = if forward { i + 1 } else { i - 1 }
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn s15_test() {
        assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![[-1, -1, 2], [-1, 0, 1]]);
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(vec![]), empty);
        assert_eq!(Solution::three_sum(vec![0]), empty);
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![[0, 0, 0]]);
        assert_eq!(Solution::three_sum(vec![2, 2, 2, -4, 0, -2]), vec![[-4, 2, 2], [-2, 0, 2]]);
    }
}