import kotlin.math.absoluteValue
import kotlin.math.min

fun main() {
    val solution = Solution()
    println(solution.maxArea(intArrayOf(1, 8, 6, 2, 5, 4, 8, 3, 7)))
}


class Solution {
    fun maxArea(height: IntArray): Int {
        var p = 0
        var q = height.size - 1
        var maxArea = -1
        while (p < q) {
            val area = getArea(height, p, q)
            if (area > maxArea) {
                maxArea = area
            }
            if (height[p] > height[q]) {
                q--
            } else {
                p++
            }
        }
        return maxArea
    }


    private inline fun getArea(height: IntArray, p: Int, q: Int): Int {
        return min(height[p], height[q]) * (p - q).absoluteValue
    }
}