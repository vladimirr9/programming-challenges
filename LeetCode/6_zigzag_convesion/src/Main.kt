fun main() {
    val solution = Solution()
    val res = solution.convert("PAYPALISHIRING", 3)
    println(res)
}

class Solution {
    fun convert(s: String, numRows: Int): String {
        if (numRows == 1) {
            return s
        }
        val chars: List<MutableList<Char>> = (1..numRows).map { mutableListOf() }
        var row = 0
        var col = 0
        var verticalSpeed = 1
        var horizontalSpeed = 0
        for (char in s) {
            chars[row].add(char)
            row += verticalSpeed
            col += horizontalSpeed
            if (row == 0) {
                verticalSpeed = 1
                horizontalSpeed = 0
            }
            if (row == numRows - 1) {
                verticalSpeed = -1
                horizontalSpeed = 1
            }
        }
        val finalString = StringBuilder()
        for (i in 0..<numRows) {
            chars[i].forEach {
                finalString.append(it)
            }
        }

        return finalString.toString()
    }

    data class Coordinate(val row: Int, val col: Int)
}