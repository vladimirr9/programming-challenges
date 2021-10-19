import math

num = 43435353

def get_digits(num: int):
    return math.floor(math.log(num, 10)) + 1

print(get_digits(num))