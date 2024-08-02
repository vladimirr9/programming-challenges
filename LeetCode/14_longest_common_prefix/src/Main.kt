fun main() {
    val solution = Solution()
    println(solution.longestCommonPrefix(arrayOf("flower", "flow", "flight")))
    println(solution.longestCommonPrefix(arrayOf("dog", "racecar", "car")))
    solution.longestCommonPrefix(arrayOf(""))
}


class Solution {
    fun longestCommonPrefix(strs: Array<String>): String {
        val retVal: StringBuilder = StringBuilder()
        val minLength = strs.minOf { it.length }
        if (minLength == 0) {
            return ""
        }
        var p = 0
        outer@ while (p < minLength) {
            for (i in 0..<strs.size - 1) {
                if (strs[i][p] != strs[i + 1][p]) {
                    break@outer
                }
            }
            retVal.append(strs[0][p])
            p++
        }
        return retVal.toString()
    }
}