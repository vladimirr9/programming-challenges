#include <stdio.h>
int checkIfStrobogrammatic(int n, int num, int *array);
int power(int base, int exponent);

int main()
{
    const int n = 5;
    int array[100];
    for (int i = power(10, n-1); i < power(10, n); i++)
    {
        if (checkIfStrobogrammatic(n, i, array))
            printf("%d\n", i);
    }
    return 0;
}

int checkIfStrobogrammatic(int n, int num, int *array)
{
    for (int i = 0; i < n; i++)
    {
        array[n - i - 1] = num % 10;
        num = num / 10;
    }
    for (int i = 0; i < (n+1) / 2; i++)
    {
        if (array[i] == 1 && array[n - i - 1] == 1 ||
            array[i] == 6 && array[n - i - 1] == 9 ||
            array[i] == 9 && array[n - i - 1] == 6 ||
            array[i] == 8 && array[n - i - 1] == 8 ||
            array[i] == 0 && array[n - i - 1] == 0)
        {
            continue;
        }
        else
        {
            return 0;
        }
    }
    return 1;
}

int power(int base, int exponent)
{
    if (exponent == 0) return 1;
    int res = base;
    for (int i = 1; i < exponent; i++)
        res = res * base;
    return res;
}