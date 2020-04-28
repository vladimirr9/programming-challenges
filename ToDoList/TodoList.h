#pragma once
#include "Task.h"
#include <vector>

class TodoList
{
private:
	std::vector<Task> list;
	int size;
public:
	TodoList();
	void add(Task& t);
	void remove(int n);
	void finish(int n);
	bool isEmpty();
	void display();
	void save();
	void load();
	int getSize();
	static std::string getToday();
};