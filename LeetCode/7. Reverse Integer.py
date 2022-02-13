#int can hold more values than 32bit integers, so this solution isn't quite right
class Solution:
    def reverse(self, x: int) -> int:
        retVal = 0
        str_x = str(x)
        if str_x[0] == '-':
            minus = True
        else:
            minus = False
        abs_x = abs(x)
        str_abs_x = str(abs_x)
        for digit in range(len(str_abs_x)-1, -1, -1):
            retVal *= 10
            retVal += int(str_abs_x[digit])
        retVal = retVal * -1 if minus else retVal
        if 2**31 -1 <= retVal <= -2**32:
            return 0
        return retVal



if __name__ == '__main__':
    sol = Solution()
    print(sol.reverse(-123456789))