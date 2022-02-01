def solution(A):
    east = 0
    passings = 0
    for element in A:
        if element == 1:
            passings += east
        else:
            east += 1
        if passings > 1000000000:
            return -1
    return passings

if __name__ == '__main__':
    A = [0,1,0,1,1]
    print(solution(A))