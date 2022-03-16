use std::collections::HashMap;

mod solution {
    struct Solution {}

    // @lc code=start
    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut complements: Hashmap<i32, i32> = HashMap::new();
            for (i, num) in nums.iter().enumerate() {
                match complements.get(num) {
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
            let value = Solution::two_sum([1, 2, 3], 4);
            assert_eq!([0, 2], value);
            assert_eq!([0, 2], two_sum([1, 2, 3, 4, 5], 5));
            assert_eq!(None, two_sum([1, 2], 5));
        }
    }
}
/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */
// pub struct Solution {}

// // @lc code=start
// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         let mut complements: Hashmap<i32, i32> = HashMap::new();
//         for (i, num) in nums.iter().enumerate() {
//             match complements.get(num) {
//                 Some(&index) => return vec![index, i as i32],
//                 None => complements.insert(target - num, i as i32),
//             };
//         }
//         vec![]
//     }
// }

// // @lc code=end

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let value = two_sum([1, 2, 3], 4);
//         assert_eq!([0, 2], value);
//         assert_eq!([0, 2], two_sum([1, 2, 3, 4, 5], 5));
//         assert_eq!(None, two_sum([1, 2], 5));
//     }
// }
