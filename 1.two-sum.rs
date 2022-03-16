use std::collections::HashMap;
/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Ve , target: i32) -> Vec<i32> {
        let mut complements: Hashmap<i32, i32> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            match complements.get(&num) {
                Some(&index) => return vec![index, i as i32],
                None => complements.insert(target - num, i as i32),
            };
        }
        vec![]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let value = two_sum([1, 2, 3], 4);
        assert_eq!([0, 2], value);
    }
}
