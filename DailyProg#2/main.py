initial = [1, 2, 3, 4, 5]


# initial = [3,2,1]

def get_products(arr: list):
    left = [1] * len(arr)
    right = [1] * len(arr)
    res = [0] * len(arr)
    for i in range(1, len(arr)):
        left[i] = left[i - 1] * arr[i - 1]
    for i in range(len(arr) - 2, -1, -1):
        right[i] = right[i + 1] * arr[i + 1]
    for i in range(len(arr)):
        res[i] = left[i] * right[i]
    return res

print(get_products(initial))
