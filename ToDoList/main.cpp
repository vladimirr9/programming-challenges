#include "Task.h"
#include "TodoList.h"
#include <ctime>
#include <iostream>
#include <string>
#include <exception>
using namespace std;
void add(TodoList& list);
void remove(TodoList& list);
void reset(TodoList& list);
void quit(TodoList& list);





int main()
{
	TodoList list = TodoList();
	list.load();
	string choice;
	int n = 0;
	while (1)
	{
		list.display();
		
		cout << "What do you want to do?" << endl << endl;
		cout << ">add" << endl;
		cout << ">remove" << endl;
		cout << ">reset" << endl;
		cout << ">quit" << endl;
		cout << "Type out the number of the task you want to mark off as done" << endl << endl;
		cin >> choice;
		if (choice.compare("add") == 0)
			add(list);
		else if (choice.compare("remove") == 0)
			remove(list);
		else if (choice.compare("reset") == 0)
			reset(list);
		else if (choice.compare("quit") == 0)
		{
			quit(list);
			break;
		}
		else
		{
			try
			{
				n = stoi(choice);
			}
			catch (exception& e)
			{
				cout << "Bad input!" << endl;
				continue;
			}
			list.finish(n);
		}
	}
	return 0;
}



void add(TodoList& list)
{
	cout << endl;
	cout << endl;
	cout << "What do you want to add? Type \"done\" to return" << endl;
	string content;
	cin >> content;
	while (content.compare("done") != 0)
	{
		Task t = Task(content);
		list.add(t);
		cin >> content;
	}
	cout << endl;
}

void remove(TodoList& list)
{
	cout << endl;
	cout << "Which one do you want to remove? Type the task number" << endl;
	string str;
	int content = 0;
	cin >> str;
	try
	{
		content = stoi(str);
	}
	catch (exception& e)
	{
		cout << "Bad input!" << endl;
		return;
	}

	{
		if (content > 0 && content <= list.getSize())
		{
			list.remove(content);
			cout << "Removed!" << endl;
		}
		else
			cout << "A task with that number doesn't exist!" << endl;
	}
	cout << endl;
	cout << endl;
}

void reset(TodoList& list)
{
	list = TodoList();
	cout << endl;
	cout << endl;
}

void quit(TodoList& list)
{
	list.save();
}