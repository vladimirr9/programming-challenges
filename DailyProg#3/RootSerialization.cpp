#include <string>
#include <iostream>



using namespace std;

static const string EMPTYMARKER = "$";

class Node
{
private:
string val;
Node* left;
Node* right;

public:
Node(string val, Node* left = 0, Node* right = 0)
{
    this->val = val;
    this->left = left;
    this->right = right;
}
void display()
{
    cout<<val<<endl;
    if (left && right)
    {
        left->display();
        right->display();
    }
    else if (left)
        left->display();
    else if (right)
        right->display();
    else return;
}

static string serialize(Node* node)
{
    string retVal;
    retVal = "(" + node->val + ",";
    if (node->left)
        retVal += serialize(node->left) + ",";
    else
        retVal += EMPTYMARKER + ",";
    if (node->right)
        retVal += serialize(node->right);
    else
        retVal += EMPTYMARKER;
    retVal += ")";
    return retVal;
    
}
static Node* deserialize(string::iterator &it)
{
    string::iterator p = it+1;
    while (*it != ',')
        ++it;
    string tempVal = string(p,it);
    Node* tempLeft = 0;
    Node* tempRight = 0;
    ++it;
    if (*it == '(')
    {
        tempLeft = deserialize(it);
    }
    ++it;
    ++it;
    if (*it == '(')
    {
        tempRight = deserialize(it);
    }
    it++;
    Node* newNode = new Node(tempVal, tempLeft, tempRight);
    return newNode;
}
static void deleteNode(Node* node)
{
    if (node->left) deleteNode(node->left);
    if (node->right) deleteNode(node->right);
    delete node;
    return;
}

};



int main()
{
    Node v = Node("v");
    Node l = Node("l");
    Node vl = Node("vl",&v,&l);
    Node a = Node("a");
    Node d = Node("d");
    Node ad = Node("ad", &a, &d);
    Node vlad = Node("Vlad", &vl, &ad);

    Node* neko = new Node("neko", new Node("ne", new Node("n"), new Node("e")), new Node("ko", new Node("k"), new Node("o")));

    vlad.display();
    neko->display();
    cout << Node::serialize(&vlad) << endl;
    string vladSerial = Node::serialize(&vlad);
    string::iterator it = vladSerial.begin();
    Node::deserialize(it)->display();
    cout << Node::serialize(&vlad) << endl;

    cout << "----------------------------------" << endl;
    neko->display();
    Node::deleteNode(neko);


    return 0;
}