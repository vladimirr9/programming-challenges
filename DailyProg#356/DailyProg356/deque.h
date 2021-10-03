#pragma once
#include <iostream>
#include <vector>
using namespace std;

template<class T> class MyDeque {
private:
	int arraySize;
	int size;
	int numOfArrays;
public:
	vector<T*> vector1;

	MyDeque(int arraySize) {
		this->size = 0;
		this->arraySize = arraySize;
		numOfArrays = 0;
	}

	void enqueue(T item) {
		size++;
		if (size >= numOfArrays * arraySize) {
			T* newArray = new T[arraySize];
			vector1.push_back(newArray);
			numOfArrays++;
		}
		for (int i = size - 1; i > 0; i--) {
			int arrayNum = i / arraySize;
			int indInArray = i % arraySize;
			vector1.at(arrayNum)[indInArray] = get(i - 1);
		}
		vector1.at(0)[0] = item;

	}

	T dequeue() {
		if (size == 0)
			return NULL;
		int arrayNum = size - 1 / arraySize;
		int indInArray = size - 1 % arraySize;
		T item = get(size - 1);
		size--;
		if (indInArray == 0) {
			T* pt = vector1.at(arrayNum);
			delete[]pt;
			vector1.pop_back();
			numOfArrays--;
		}
		return item;

	}

	void printAll() {
		for (int i = 0; i < size; i++)
			cout << get(i) << " ";
		cout << endl;
	}

	T get(int i) {
		if (i >= size || i < 0)
			return NULL;
		int arrayNum = i / arraySize;
		int indInArray = i % arraySize;
		return vector1.at(arrayNum)[indInArray];
	}

	int getSize() {
		return size;
	}
};