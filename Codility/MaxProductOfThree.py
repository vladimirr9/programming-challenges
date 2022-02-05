def solution(A):
    A.sort()
    N = len(A)
    return max(A[0] * A[1] * A[N - 1], A[N - 1] * A[N - 2] * A[N - 3])

if __name__ == '__main__':
    A = [-3,1,2,-2,5,6]
    print(solution(A))
