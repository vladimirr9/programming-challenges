def get_num_of_digits(n):
    num_of_digits = 0
    while n > 0:
        num_of_digits += n & 1
        n = n >> 1
    return num_of_digits

def find_next_highest(n):
    n_digits = get_num_of_digits(n)
    i = n + 1
    while get_num_of_digits(i) != n_digits:
        i += 1
    return i

print(find_next_highest(6))