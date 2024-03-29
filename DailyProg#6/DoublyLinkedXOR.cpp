#include <iostream>
// not fully implemented, only get and add, dynamically allocated memory is NOT freed
using namespace std;

class Node
{
private:

public:

int content;
Node* both;

void display()
{
    std::cout << "Node value: " << content << std::endl;
}


Node(int c, Node* both=0)
{
    content = c;
    this->both = both;
}


};

class DLList
{



private:
int size;
Node* root;
Node* last;

public:
DLList()
{
    size = 0;
    root = new Node(0);
    last = root;
}
int getSize() {return size;}
void add(int c)
{
    if (!size)
    {
        root = new Node(c);
    }
    else if (size == 1)
    {
        Node* newNode = new Node(c);
        newNode->both = (Node*)((long)root ^ (long)last);
        root->both = (Node*)((long)newNode ^ (long)last);
        last->both = (Node*)((long)root ^ (long)newNode);
    }
    else
    {
        Node* prev = (Node*)((long)root ^ (long)last->both);
        Node* newNode = new Node(c);
        newNode->both = (Node*)((long)last ^ (long)prev);
        last->both = (Node*)((long)newNode ^ (long)root);
        prev->both = (Node*)(((long)prev->both ^ (long)last) ^ (long)newNode);
    }
    size++;
}

Node* get(int index)
{
    if (!size) return nullptr;
    if (index == 0) return root;

    Node* Addr = root;
    if (index <= size/2)
    { 
        Node* nextAddr =(Node*)((long) root->both ^ (long)last);
        while (index > 0)
        {
            Node* temp = nextAddr;
            nextAddr = (Node*)(((long) nextAddr->both) ^ (long) Addr);
            Addr = temp;
            --index;
        }
    }
    else
    {
        Addr = last;
        Node* nextAddr = (Node*)(((long)last->both) ^ (long)root);
        while (index < size)
        {
            Node* temp = nextAddr;
            nextAddr = (Node*)(((long) nextAddr->both) ^ (long) Addr);
            Addr = temp;
            ++index;
        }
    }
    return Addr;
    
}
};




int main()
{
    DLList lista = DLList();
    lista.add(1);
    lista.add(2);
    lista.add(3);
    lista.add(6);

    for (int i = 0; i < lista.getSize(); i++)
        lista.get(i)->display();
    return 0;
}