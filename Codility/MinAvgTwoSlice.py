import sys


def solution(A):
    min_value = sys.maxsize
    min_it = -1
    for i in range(len(A)-1):
        value = (A[i] + A[i+1]) / 2
        if value < min_value:
            min_it = i
            min_value = value
    for i in range(len(A)-2):
        value = (A[i] + A[i+1] + A[i+2]) / 3
        if value < min_value:
            min_it = i
            min_value = value
    return min_it
