import kotlin.math.pow

fun main() {
    val solution = Solution()
    println(solution.myAtoi("42"))
    println(solution.myAtoi("   -042"))
    println(solution.myAtoi("1337c0d3"))
    println(solution.myAtoi("0-1"))
    println(solution.myAtoi("words and 987"))
    println(solution.myAtoi("-91283472332"))
    println(solution.myAtoi("1"))
}


class Solution {
    fun myAtoi(s: String): Int {
        if (s.isEmpty()) {
            return 0
        }
        var i = 0
        while (s[i].isWhitespace()) {
            if (i >= s.length - 1) {
                return 0
            }
            i++
        }
        val negative: Boolean
        if (s[i] == '-') {
            negative = true
            i++
        } else if (s[i] == '+') {
            negative = false
            i++
        }  else {
            negative = false
        }
        if (i > s.length - 1 || !s[i].isDigit()) {
            return 0
        }
        var retVal = 0
        while (s[i].isDigit()) {
            try {
                retVal = Math.multiplyExact(retVal, 10)
                retVal = Math.addExact(retVal, s[i].digitToInt())
            } catch (e: ArithmeticException) {
                if (negative) {
                    return Int.MIN_VALUE
                } else {
                    return Int.MAX_VALUE
                }
            }

            if (i >= s.length - 1) {
                break
            }
            i++

        }
        return if (negative) {
            -retVal
        } else {
            retVal
        }
    }
}