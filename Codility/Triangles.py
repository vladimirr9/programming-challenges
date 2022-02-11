def solution(A):
    if len(A) < 3:
        return 0
    A.sort()
    for idx in range (len(A)-2):
        if A[idx] >= 0 and A[idx] > A[idx + 2] - A[idx + 1]:
            return 1
    return 0




