
def solution(A):
    suma = sum(A) - 2*A[-1]
    retVal = abs(suma)
    for idx in range(len(A)-2, 0, -1):
        suma -= 2*A[idx]
        if abs(suma) < retVal:
            retVal = abs(suma)
    return retVal

if __name__ == '__main__':
    A = [3,1]
    print(solution(A))
