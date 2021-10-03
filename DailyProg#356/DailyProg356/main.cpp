/*
Good morning! Here's your coding interview problem for today.

This problem was asked by Netflix.

Implement a queue using a set of fixed-length arrays.

The queue should support enqueue, dequeue, and get_size operations.
*/

#include <iostream>
#include "deque.h"
using namespace std;

int main() {
	cout << "hello" << endl;
	MyDeque<int> *d = new MyDeque<int>(10);
	for (int i = 0; i <= 235; i++)
		d->enqueue(i);
	d->printAll();
	for (int i = 0; i <= 220; i++)
		d->dequeue();
	d->printAll();

}