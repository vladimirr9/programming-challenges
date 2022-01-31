def solution(A):
    occured = dict()
    for element in A:
        occured[element] = True
    for idx in range(1,len(A)+2):
        if idx not in occured:
            return idx

if __name__ == '__main__':
    A = [2,3,1,5]
    print(solution(A))