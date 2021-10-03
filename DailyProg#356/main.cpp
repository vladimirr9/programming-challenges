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