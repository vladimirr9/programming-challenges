def solution(A):
    noUnique = -1
    occurences = dict()
    for val in A:
        if val not in occurences:
            occurences[val] = 0
        occurences[val] += 1
    for val in A:
        if occurences[val] == 1:
            return val
    return  noUnique
