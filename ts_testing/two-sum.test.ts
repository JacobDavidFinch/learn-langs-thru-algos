import {twoSum} from './two-sum'

test('returns indecises that caclualte to sum', () => {
    expect(twoSum([1, 2, 3], 4)).toMatchObject([0, 2]);
    expect(twoSum([1, 2, 3, 4, 5], 5)).toMatchObject([1, 2]);
    expect(twoSum([1, 2], 5)).toBeNull();
  });
