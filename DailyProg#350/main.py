import sys
import copy
import pickle
import os

def assembleSet(N):
    map = dict()
    i = 1
    while (i ** 2 <= N):
        map[i**2] = i**2
        i += 1
    reversed_map = dict()
    for i in map.values().__reversed__():
        reversed_map[i] = i;
    return reversed_map




def findPerfectSquares(N, map, solutions: dict):
    if N in map:
        return (1, [N])
    min = sys.maxsize
    bestvals = []
    for val in map:
        if val > N:
            continue
        if (N-val in solutions):
            p, vals = solutions.get(N-val)
        else:
            p, vals = findPerfectSquares(N-val, map, solutions)

        if p+1 < min:
            min = p+1
            vals = copy.deepcopy(vals)
            vals.append(val)
            bestvals = vals
    if N not in solutions:
        solutions[N] = (min, bestvals)
    return (min, bestvals)
#
# if (os.path.exists("./pickledSolutions")):
#     solutions = pickle.load(open('pickledSolutions', 'rb'))
# else:
solutions = dict()
for i in range(100, 10050):
    map = assembleSet(i)

    min, vals = findPerfectSquares(i, map, solutions)
    print (i, '  min: ', min, '  vals:', vals)

#pickle.dump(solutions, open('pickledSolutions', 'wb'))

