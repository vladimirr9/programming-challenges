import kotlin.math.absoluteValue

fun main() {
    val solution = IntSolution()
    println(solution.reverse(123))
    println(solution.reverse(-123))
    println(solution.reverse(120))
}

class IntSolution {
    fun reverse(x: Int): Int {
        var num = x
        var y = 0
        while (num != 0) {
            val lastDigit = num % 10
            val newRes = y * 10 + lastDigit
            if ((newRes - lastDigit) / 10 != y) {
                return 0
            }
            y = newRes
            num /= 10
        }
        return y
    }
}


class StringSolution {
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