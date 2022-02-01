def solution(A):
    present_numbers = set()
    for element in A:
        if element < 0 or element > len(A) or element in present_numbers:
            return 0
        else:
            present_numbers.add(element)
    return 1

if __name__ == '__main__':
    A = [4,1,3]
    print(solution(A))