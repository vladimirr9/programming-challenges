import time
import random
from main import find_missing_int
iters = 10000000


initial= [random.randint(-10,1000000) for x in range(iters)]
first = time.perf_counter()
res = find_missing_int(initial)
second = time.perf_counter()
print(second - first)
print('-------')

initial= [random.randint(-10,1000000) for x in range(iters)]
first = time.perf_counter()
initial.sort()
second = time.perf_counter()
print(second - first)
print('-------')