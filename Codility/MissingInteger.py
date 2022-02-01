def solution(A):
    occurs = set()
    for value in A:
        occurs.add(value)
    retval = 1
    while retval in occurs:
        retval += 1
    return retval


if __name__ == '__main__':
    # A = [1,3,6,4,1,2]
    A = [-1,-3]
    print(solution(A))