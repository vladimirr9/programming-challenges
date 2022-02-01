
def solution(X, A):
    leaves = set()
    for idx in range(len(A)):
        if A[idx] not in leaves:
            leaves.add(A[idx])
        if len(leaves) == X:
            return idx
    return -1



if __name__ == '__main__':
    pass
