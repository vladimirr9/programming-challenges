from collections import deque
#solution doesnt convert to string
class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False
        if x == 0:
            return True
        digits = deque()
        while x > 0:
            digits.append(x % 10)
            x = x // 10
        while len(digits) > 0:
            if digits[0] != digits[-1]:
                return False
            digits.popleft()
            if len(digits) > 0:
                digits.pop()
        return True