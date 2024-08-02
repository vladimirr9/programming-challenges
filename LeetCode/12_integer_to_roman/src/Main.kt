import kotlin.math.pow

fun main() {
    val solution = Solution()

    println(solution.intToRoman(49))
    println(solution.intToRoman(6))
    println(solution.intToRoman(4))
    println(solution.intToRoman(3749))
    println(solution.intToRoman(58))
    println(solution.intToRoman(1994))
}


class Solution {
    fun intToRoman(num: Int): String {
        if (num == 0) {
            return ""
        }
        if (num == 1) {
            return "I"
        }
        val numStr = num.toString()
        if (numStr[0] == '4' || numStr[0] == '9'){
            val numeral = getHighestThatFitsAndTimes(num + 10.0.pow(numStr.length - 1).toInt())
            val subtractingNumeral = getSubtractingNumeral(numeral)
            return subtractingNumeral.toString() + numeral.toString() + intToRoman(num - (numeral.value - subtractingNumeral.value))
        } else {
            val numeral = getHighestThatFitsAndTimes(num)
            return numeral.toString() + intToRoman(num - numeral.value)
        }
    }


    private inline fun getHighestThatFitsAndTimes(num: Int): RomanNumeral {
        for (numeral in RomanNumeral.entries.reversed()) {
            if (num / numeral.value > 0) {
                return numeral
            }
        }
        throw IllegalArgumentException()
    }

    private inline fun getSubtractingNumeral(numeral: RomanNumeral) : RomanNumeral {
        return when (numeral) {
            RomanNumeral.I -> throw IllegalArgumentException()
            RomanNumeral.V, RomanNumeral.X -> RomanNumeral.I
            RomanNumeral.L, RomanNumeral.C -> RomanNumeral.X
            RomanNumeral.D, RomanNumeral.M -> RomanNumeral.C
        }
    }


    enum class RomanNumeral(val value: Int) {
        I(1),
        V(5),
        X(10),
        L(50),
        C(100),
        D(500),
        M(1000);
    }
}