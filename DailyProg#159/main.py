def firstRec(string):
    mapa = {}
    for i in range(256):
        mapa[i] = 0
    for char in string:
        if (mapa[ord(char)] != 0): return char
        mapa[ord(char)] += 1
    return None

input = "abcdef"
print(firstRec(input))