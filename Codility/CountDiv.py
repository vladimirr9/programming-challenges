import math

def find_closest_divisible(A, K):
    number = A
    while number % K != 0:
        number += 1
    return number



def solution(A, B, K):
    number = find_closest_divisible(A, K)
    if number > B:
        return 0
    return math.floor(((B-number) / K) + 1)
    pass

if __name__ == '__main__':
    A = 6
    B = 11
    K = 2
    print(solution(A,B,K))