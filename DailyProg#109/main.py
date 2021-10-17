val = 0b11100010


def swap_digits(val: int) -> val:
    return ((val & 0b10101010) >> 1) | ((val & 0b01010101) << 1)


print("{0:b}".format(swap_digits(val)))
