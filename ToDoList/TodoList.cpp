#include "TodoList.h"
#include <iostream>
#include <fstream>
#include <ctime>
#include <direct.h>
#include <filesystem>

#define path2 "lists/"

TodoList::TodoList()
{
	size = 0;
}
int TodoList::getSize()
{
	return size;
}

void TodoList::add(Task& t)
{
	list.push_back(t);
	size++;
}

void TodoList::remove(int n)
{
	list.erase(list.begin() + n - 1);
	size--;

}

void TodoList::finish(int n)
{
	try
	{
		list.at(n-1).finish();
	}
	catch (std::exception & e)
	{
		std::cout << "n is out of range!" << std::endl;
	}
}
bool TodoList::isEmpty()
{
	return size==0;
}
void TodoList::display()
{
	if (isEmpty())
	{
		std::cout << "------------------------------------------------------------------" << std::endl;
		std::cout << "List is empty!" << std::endl;
		std::cout << "------------------------------------------------------------------" << std::endl;
		return;
	}
	std::cout << "------------------------------------------------------------------" << std::endl;
	for (int i = 0; i < size; i++)
	{
		std::cout << i+1 << ". " << list.at(i).getContent();
		if (list.at(i).isDone())
			std::cout << " ---DONE";
		std::cout << std::endl;
	}
	std::cout << "------------------------------------------------------------------" << std::endl;
}
void TodoList::save()
{
	
	std::filesystem::create_directory(path2);
	std::string filePath = path2 + getToday() +".txt";
	std::ofstream file;
	file.open(filePath);
	if (file.is_open())
	{
		for (int i = 0; i < size; i++)
		{
			if (list.at(i).isDone())
				file << list.at(i).getContent()+ " 1" + "\n";
			else
				file << list.at(i).getContent() + " 0" + "\n";
		}
		file.close();
	}
	else
		std::cout << "Unable to open file." << std::endl;
}
void TodoList::load()
{
	
	std::string filePath = path2 + getToday() +".txt";
	//std::cout << filePath << std::endl;
	std::ifstream file;
	std::string content;
	int n;
	bool done;
	file.open(filePath);
	if (file.is_open())
	{
		std::string line = "";
		while (getline(file, line))
		{
			n = line.find(' ');
			content = line.substr(0, n);
			done = (bool) stoi(line.substr(n + 1, 1));
			Task t = Task(content,done);
			add(t);
		}
		file.close();
	}
	else
		std::cout << "Unable to open file." << std::endl;

}

std::string TodoList::getToday()
{
	time_t now = time(0);
	now -= 60 * 60 * 5; //5 hour offset so the day is actually today_5AM - tomorrow_5AM
	struct tm lt;
	localtime_s(&lt, &now);
	std::string today = std::to_string((lt.tm_year + 1900)) + "_" + std::to_string((lt.tm_mon + 1)) + "_" + std::to_string(lt.tm_mday);
	return today;
}

