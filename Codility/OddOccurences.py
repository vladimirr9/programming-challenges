def main():
    A = [2, 2, 3, 3, 5]
    solution(A)
    pass


def isOdd(N):
    return N & 1 == 1

def solution(A):
    occurences = dict()
    for element in A:
        if element not in occurences:
            occurences[element] = 0
        occurences[element] += 1
    for key, value in occurences.items():
        if isOdd(value):
            return key
    return -1


if __name__ == '__main__':
    main()


