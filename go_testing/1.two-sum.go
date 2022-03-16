package main

import(
	"fmt"
	"testing"
)

/*
 * @lc app=leetcode id=1 lang=golang
 *
 * [1] Two Sum
 */

// @lc code=start

func twoSum(nums []int, target int) []int {
	m := make(map[int]int)
	for idx, num := range nums {
		if val, found := m[target - num]; found { return []int{val, idx}}

		m[num] = idx
	}
	return nil
}

// @lc code=end

func TestTwoSum(t *testing.T) {
    output := twoSum([]int{1,2,3}, 4)
    if output != []int{0, 2} {
        t.Errorf("TwoSum = %d; want 1", output)
    }
}