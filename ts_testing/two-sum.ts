
export function twoSum(nums: number[], target: number): number[] {
    const numIndex = new Map<number, number>();
    for (let i = 0; i < nums.length; i++) {
        const num = nums[i];
        if(numIndex.has(target - num)) return [numIndex.get(target - num), i];
        numIndex.set(num, i);
    }
    return null;
};