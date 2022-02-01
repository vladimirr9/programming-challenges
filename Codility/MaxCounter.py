# you can write to stdout for debugging purposes, e.g.
# print("this is a debug message")



def solution(N, A):
    counters = [0 for x in range(N)]
    equalized = False
    last_equalized = 0
    max_counter = 0
    for value in A:
        if value == len(counters) + 1:
            equalized = True
            last_equalized = max_counter
        else:
            if equalized:
                counters[value-1] = max(counters[value-1]+1, last_equalized+1)
            else:
                counters[value - 1] += 1
            if counters[value-1] > max_counter:
                max_counter = counters[value-1]
    return [max(x, last_equalized) for x in counters]

if __name__ == '__main__':
    A = [3,4,4,6,1,4,4]
    print(solution(5,A))