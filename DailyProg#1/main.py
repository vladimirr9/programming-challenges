def find_nums(arr, k):
    vals = set()
    for el in arr:
        if k - el in vals:
            return True, el, k - el
        vals.add(el)
    return False, None, None


arr = [10, 15, 3, 7]
k = 17

res, val1, val2 = find_nums(arr,k)
if res:
    print(val1, val2)