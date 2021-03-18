import numpy as np
import random

def getMatrix(N,M, p):
    """
    :param N: num of rows
    :param M: num of cols
    :param p: chance that a field is 1
    :return: matrix NxM
    """
    mat = np.zeros((N,M))
    for i in range(N):
        for j in range(M):
            if random.uniform(0,1) < p:
                mat[i][j] = 1
    mat[0,0] = 0
    mat[N-1,M-1] = 0
    return mat

N = 10
M = 10
p = 0.3
Mat = getMatrix(N,M,p)
ways = np.zeros(Mat.shape)
ways[0,0] = 1
for i in range(1,M):
    if Mat[0,i] == 0 and ways[0,i-1] != 0:
        ways[0,i] = 1
for i in range(1,N):
    if Mat[i,0] == 0 and ways[i-1,0] != 0:
        ways[i,0] = 1
for i in range(1,M):
    for j in range(1,N):
        if (Mat[i,j] != 1):
            ways[j,i] = ways[j-1,i] + ways[j,i-1]
#print(Mat)
#print(ways)
print(int(ways[N-1,M-1]))