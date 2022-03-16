/*
 * @lc app=leetcode id=1 lang=golang
 *
 * [1] Two Sum
 */

// @lc code=start
m := make(map[int]int)

func twoSum(nums []int, target int) []int {
	var twoSumResult
	for i, v := range nums {
		elem, ok = m[target - num]
		if ok {
			twoSumResult = [elem, i]
		}
		m[i] = v
		}
	return twoSumResult
}
// @lc code=end

