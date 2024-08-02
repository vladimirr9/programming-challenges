fun main() {
    val solution = Solution()
    println(solution.romanToInt("III"))
    println(solution.romanToInt("LVIII"))
    println(solution.romanToInt("MCMXCIV"))
}



class Solution {
    fun romanToInt(s: String): Int {
        var result = 0
        var p = 0

        while (p < s.length) {
            val symbol = RomanNumeral.valueOf(s[p].toString())
            p++
            val value : Int = when (symbol) {
                RomanNumeral.I -> {
                    if (p < s.length && RomanNumeral.valueOf(s[p].toString()) == RomanNumeral.V) {
                        p++
                        4
                    } else if (p < s.length && RomanNumeral.valueOf(s[p].toString()) == RomanNumeral.X) {
                        p++
                        9
                    } else {
                        1
                    }
                }
                RomanNumeral.V -> RomanNumeral.V.value
                RomanNumeral.X -> {
                    if (p < s.length && RomanNumeral.valueOf(s[p].toString()) == RomanNumeral.L) {
                        p++
                        40
                    } else if (p < s.length && RomanNumeral.valueOf(s[p].toString()) == RomanNumeral.C) {
                        p++
                        90
                    } else {
                        10
                    }
                }
                RomanNumeral.L -> RomanNumeral.L.value
                RomanNumeral.C -> {
                    if (p < s.length && RomanNumeral.valueOf(s[p].toString()) == RomanNumeral.D) {
                        p++
                        400
                    } else if (p < s.length && RomanNumeral.valueOf(s[p].toString()) == RomanNumeral.M) {
                        p++
                        900
                    } else {
                        100
                    }
                }
                RomanNumeral.D -> RomanNumeral.D.value
                RomanNumeral.M -> RomanNumeral.M.value
            }
            result += value
        }
    return result
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

