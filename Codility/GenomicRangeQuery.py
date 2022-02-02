def solution(S, P, Q):
    sol = []
    S = list(S)
    A_prefix_sum = [0] * (len(S) + 1)
    C_prefix_sum = [0] * (len(S) + 1)
    G_prefix_sum = [0] * (len(S) + 1)
    for idx, el in enumerate(S):
        if el == 'A':
            A_prefix_sum[idx+1] = 1
        elif el == 'C':
            C_prefix_sum[idx + 1] = 1
        elif el == 'G':
            G_prefix_sum[idx + 1] = 1
        A_prefix_sum[idx+1] += A_prefix_sum[idx]
        C_prefix_sum[idx+1] += C_prefix_sum[idx]
        G_prefix_sum[idx+1] += G_prefix_sum[idx]

    for left, right in zip(P,Q):
        left_idx = left
        right_idx = right + 1 #jagged array
        if A_prefix_sum[right_idx] > A_prefix_sum[left_idx]:
            sol.append(1)
        elif C_prefix_sum[right_idx] > C_prefix_sum[left_idx]:
            sol.append(2)
        elif G_prefix_sum[right_idx] > G_prefix_sum[left_idx]:
            sol.append(3)
        else:
            sol.append(4)
    return sol

if __name__ == '__main__':
    P = [2,5,0]
    Q = [4,5,6]
    S = 'CAGCCTA'
    print(solution(S,P,Q))