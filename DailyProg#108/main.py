A = 'abcde'
B = 'cdeab'


def can_be_shifted(A: str, B: str) -> bool:
    if not A or not B:
        return False
    if len(A) != len(B):
        return False
    for i in range(len(B)):
        shiftable = True
        for k in range(len(B)):
            if A[(i + k) % len(A)] != B[k]:
                shiftable = False
                break
        if shiftable:
            return True
    return False


print(can_be_shifted(A, B))
