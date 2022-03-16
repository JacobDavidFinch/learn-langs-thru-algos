/*
 * @lc app=leetcode id=1 lang=typescript
 *
 * [1] Two Sum
 */

// @lc code=start

const numIndex = new Map<number, number>();

function twoSum(nums: number[], target: number): number[] {
    let twoSumResult;
    nums.forEach((num, i) => {
        if(numIndex.has(target - num)){
            twoSumResult = [numIndex.get(target - num), i];
        }
        numIndex.set(num, i);
    })
    return twoSumResult;
};
// @lc code=end

