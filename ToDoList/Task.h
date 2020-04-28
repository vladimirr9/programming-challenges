#pragma once
#include <string>

class Task {
private:
	std::string content;
	bool done;

public:
	Task(std::string content);
	Task(std::string content, bool done);
	bool isDone();
	void finish();
	std::string &getContent();

};