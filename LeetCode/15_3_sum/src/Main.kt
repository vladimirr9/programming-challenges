fun main() {
    val solution = Solution()
    println(solution.threeSum(intArrayOf(0, 0, 0)))
    println(solution.threeSum(intArrayOf(-1, 0, 1, 2, -1, -4)))
    println(solution.threeSum(intArrayOf(0, 1, 1)))
}


class Solution {
    fun threeSum(nums: IntArray): List<List<Int>> {
        val map = HashMap<Int, MutableList<Int>>()
        val results: MutableSet<List<Int>> = mutableSetOf()
        for (i in nums.indices) {
            if (!map.contains(nums[i])) {
                map[nums[i]] = mutableListOf()
            }
            if (map[nums[i]]!!.size < 3) {
                map[nums[i]]?.add(i)
            }
        }
        for (i in nums.indices) {
            val x: Int = nums[i]
            for (j in (i + 1)..<nums.size) {
                val y = nums[j]
                val z = -(x + y)
                val mapEntry = map[z]
                if (mapEntry != null) {
                    for (validValue in mapEntry) {
                        if (j != validValue && i != validValue) {
                            results.add(listOf(x, y, z).sorted())
                        }
                    }
                }
            }
        }
        return results.toList()
    }
}