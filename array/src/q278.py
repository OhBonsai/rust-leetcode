# The isBadVersion API is already defined for you.
# @param version, an integer
# @return a bool
# def isBadVersion(version):



def isBadVersion(version: int) -> bool:
    return version > 3


class Solution:
    def firstBadVersion(self, n: int) -> int:
        l = 1
        r = n

        while l < r:
            m = l + int( (r -l)/2)

            if isBadVersion(m):
                r = m
            else:
                l = m + 1


        return l
