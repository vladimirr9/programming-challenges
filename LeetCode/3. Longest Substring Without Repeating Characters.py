class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        if s is None or len(s) == 0:
            return 0
        if len(s) == 1:
            return 1
        i = 0
        res = 0
        curr = 0
        characters = set()
        while i < len(s):
            if s[i] not in characters:
                characters.add(s[i])
                curr += 1
                i += 1
                if curr > res:
                    res = curr
            else:
                characters = set()
                curr = 0
        return res


if __name__ == '__main__':
    s = "pwwkew"
    sol = Solution()
    print(sol.lengthOfLongestSubstring(s))
