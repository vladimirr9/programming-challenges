fun main() {
    val cube = SolutionNCube()
    val square = SolutionNSquare()

    val test = square
    println(test.longestPalindrome("babad"))
    println(test.longestPalindrome("cbbd"))
}


// time: O(N)
// space: O(N)
class SolutionN {
    // See Manacher's algorithm
}


// time: O(N^2)
// space: O(N)
class SolutionNSquare {
    fun longestPalindrome(s: String): String {
        val paddedString = getPaddedString(s)
        var longest = s[0].toString()
        for (i in paddedString.indices) {
            var p = i
            var q = i
            while (true) {
                if (paddedString[p] != paddedString[q]) {
                    p++
                    q--
                    break
                }
                if (p == 0 || q == paddedString.length - 1) {
                    break
                }
                p--
                q++
            }
            val start = p /2
            val end = q / 2 - 1
            val potentialLongest = s.substring(start, end + 1)
            if (potentialLongest.length > longest.length) {
                longest = potentialLongest
            }
        }
        return longest
    }

    private fun getPaddedString(s: String): String {
        var newString = "|"
        for (i in s.indices) {
            newString += s.substring(i, i + 1) + "|"
        }
        return newString
    }
}


// time: O(N^3)
// space: O(1)
class SolutionNCube {
    fun longestPalindrome(s: String): String {
        var longest = s[0].toString()
        outer@ for (p in s.indices) {
            var q = s.length
            while (q - p > longest.length) {
                val substring = s.substring(p, q)
                if (isPalindrome(substring)) {
                    if (substring.length > longest.length) {
                        longest = substring
                        continue@outer
                    }
                } else {
                    q--
                }
            }
        }
        return longest
    }


    private fun isPalindrome(s: String): Boolean {
        var p = 0
        var q = s.length - 1
        while (p <= q) {
            if (s[p] != s[q]) {
                return false
            }
            p += 1
            q -= 1
        }
        return true;
    }
}
