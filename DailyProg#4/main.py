initial = [3, 4, -1, 1]


def find_missing_int(arr: list):
    no_positives = True
    i = 0
    while i < len(arr):
        if arr[i] > 0:
            no_positives = False
        if arr[i] <= 0 or arr[i] > len(arr) or arr[i] is True:
            i += 1
            continue
        if arr[i] <= i+1:
            arr[arr[i]-1] = True
            i += 1
            continue
        if arr[i] > i:
            val = arr[arr[i]-1]
            arr[arr[i]-1] = True
            arr[i] = val
            continue
    if no_positives:
        return 1
    for k in range(len(arr)):
        if type(arr[k]) is not bool:
            return k + 1
    return len(arr) + 1

print(find_missing_int(initial))
