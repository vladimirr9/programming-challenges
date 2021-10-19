a = [-6, 0, 2, 40]
b = [1, 5, 7, 8]


def get_fixed_point(arr: list):
    for i in range(len(arr)):
        if i == arr[i]:
            return arr[i]
    return False


print(get_fixed_point(a))
print(get_fixed_point(b))