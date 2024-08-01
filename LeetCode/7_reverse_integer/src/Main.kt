import kotlin.math.absoluteValue

fun main() {
    val solution = Solution()
    println(solution.reverse(123))
    println(solution.reverse(-123))
    println(solution.reverse(120))
}


class Solution {
    fun reverse(x: Int): Int {
        val negative = x < 0
        val reversed = x.absoluteValue.toString().reversed()
        try {
            return if (negative) -reversed.toInt() else reversed.toInt()
        } catch (e: Exception) {
            return 0
        }
    }
}