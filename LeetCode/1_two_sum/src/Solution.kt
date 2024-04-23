fun main (){
    val nums = intArrayOf(2,7,11,15)
    val target = 9
    println(twoSum(nums, target).contentToString())
}


fun twoSum(nums: IntArray, target: Int): IntArray {
    val map = nums.indices.associateBy { nums[it] }
    for ((i,value) in nums.withIndex()) {
        val candidate = map[target-value]
        if (candidate == i) {
            continue
        }
        if (candidate != null) {
            return intArrayOf(i, candidate)
        }
    }
    throw IllegalArgumentException("No solution")
}