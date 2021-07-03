import pytest
from typing import List


def search_range(nums: List[int], target: int) -> List[int]:
    if len(nums) == 0:
        return [-1, -1]

    if len(nums) == 1:
        if nums[0] == target:
            return [0, 0]
        else:
            return [-1, -1]


    left, right, pos = 0, len(nums) - 1, len(nums)

    while left <= right:
        mid = int((left + right) / 2 )

        if nums[mid] > target:
            right = mid - 1
        elif nums[mid] < target:
            left = mid + 1
        else:
            pos = mid
            break

    
    if pos > len(nums) - 1 or nums[pos] != target:
        return [-1, -1]

    
    start, end = pos, pos

    while start > 0 and nums[start] == target:
        start -= 1

    if nums[0] == target:
        start = 0
    else:
        start += 1

    while end < len(nums) and nums[end] == target:
        end += 1

    if nums[-1] == target:
        end = len(nums) - 1
    else:
        end -= 1

    return [start, end]


def test_1():
    assert search_range([5, 7, 7, 8, 8, 10], 8) == [3,4]
    assert search_range([5, 7, 7, 8, 8, 10], 6) == [-1, -1]
    assert search_range([2, 2], 1) == [-1, -1]
    assert search_range([1, 2, 3], 1) == [0, 0]
    assert search_range([1, 4], 4) == [1, 1]


if __name__ == '__main__':
    pytest.main(["./q34.py", "-v", "-s"])
