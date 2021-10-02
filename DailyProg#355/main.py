
X = [5.7, 8.9, 6.5, 5.5, 1.5, 2.3, 1.2, 5.7, 9.9, 10.6, 10.4]
#X = [1.3, 2.3, 4.4]


if __name__ == '__main__':
    rounded = [round(x) for x in X]
    rounded_sum = round(sum(X))
    Y = [int(x) for x in X]
    sumdiff = abs(sum(Y) - rounded_sum)

    decimals = [(x % 1, ind) for ind, x in enumerate(X)]

    decimals = sorted(decimals, key=lambda x: (x[0]), reverse=True)
    for i in range(sumdiff):
        Y[decimals[i][1]] += 1
    print(X)
    print(Y)
    print(sum([abs(x-y) for x, y in zip(X, Y)]))
    assert(rounded_sum == sum(Y))






