


def solution(N):
    bin_string = "{0:b}".format(N)
    max_num = 0;
    current_num = 0;
    for idx in range(1, len(bin_string)):
        if bin_string[idx] == '0':
            current_num += 1
        else:
            if current_num > max_num:
                max_num = current_num
            current_num = 0
    print(max_num)
    return max_num


solution(15)