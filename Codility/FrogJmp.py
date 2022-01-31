import math

def solution(X, Y, D):
    total_distance = Y - X
    jumps = math.ceil(total_distance / D)
    return jumps


if __name__ == '__main__':
    print(solution(10,85,30))
    pass