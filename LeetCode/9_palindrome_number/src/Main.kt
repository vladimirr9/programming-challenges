fun main() {
    val solution = Solution()
    println(solution.isPalindrome(121))
    println(solution.isPalindrome(-121))
    println(solution.isPalindrome(10))
}




class Solution {
    fun isPalindrome(x: Int): Boolean {
        if (x < 0) return false
        val digits: MutableList<Int> = mutableListOf()
        var num = x
        while (num != 0) {
            digits.add(num % 10)
            num /= 10
        }
        for (i in 0..<digits.size / 2) {
            if (digits[i] != digits[digits.size - i - 1]) {
                return false
            }
        }
        return true
    }
}