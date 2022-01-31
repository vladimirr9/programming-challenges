#Writing code under the assumption perfomance is not tested, so I am maximizing readability

def rotate_once(arr):
    if (len(arr) == 0):
        return arr
    last_element = arr[-1]
    for idx in range(len(arr) - 1, 0, -1):
        arr[idx] = arr[idx-1]
    arr[0] = last_element
    return arr

def rotate_k_times(arr, k):
    for idx in range(k):
        rotate_once(arr)
    return arr


def main():
    array = [1, 2, 3, 4 ,5]
    rotate_k_times(array, 3)
    print(array)



if __name__ == '__main__':
    main()

