#include "Task.h"
#include <string>

Task::Task(std::string content)
{
	this->content = content;
	done = false;
}

Task::Task(std::string content, bool done)
{
	this->content = content;
	this->done = done;
}

bool Task::isDone()
{
	return done;
}

void Task::finish()
{
	done = true;
}


std::string& Task::getContent()
{
	return ref(content);
}
