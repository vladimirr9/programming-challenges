from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        existing_values = dict()
        for idx, num in enumerate(nums):
            if num in existing_values:
                existing_values[num].append(idx)
            else:
                existing_values[num] = [idx]
        for idx, num in enumerate(nums):
            if (target-num) in existing_values:
                if (target-num) == num:
                    if len(existing_values[num]) == 2:
                        return existing_values[num]
                    else:
                        continue
                else:
                    return [existing_values[num][0], existing_values[target-num][0]]
        pass
