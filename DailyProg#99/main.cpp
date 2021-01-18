#include <iostream>
#include <vector>
#include <unordered_map>

using namespace std;

int findLongest(vector<int> niz);

int main()
{
    vector<int> niz;
    niz.push_back(10);
    niz.push_back(4);
    niz.push_back(200);
    niz.push_back(1);
    niz.push_back(3);
    niz.push_back(2);    

    cout << findLongest(niz) << endl;
    return 0;
}


struct Node{
    Node* top = this;
    int size = 1;

};


int findLongest(vector<int> niz)
{
    unordered_map<int,Node> mapa;
    int maxDuz = 0;
    for (int i : niz)
    {
        Node* tmp = new Node();
        mapa[i] = *tmp;
        Node& tek = mapa[i];
        if (mapa.find(i-1) != mapa.end() && mapa.find(i+1) != mapa.end())
        {
            Node &pret = mapa[i-1];
            Node &vrh = (*mapa[i+1].top);
            tek.size += pret.size;
            pret.size = 0;
            pret.top = &vrh;
            tek.top = &vrh;
            vrh.size += tek.size;
            tek.size = 0;
            tek.top = &vrh;
            if (vrh.size > maxDuz)
                maxDuz = vrh.size;
        }
        else if (mapa.find(i-1) != mapa.end())
        {
            Node &pret = mapa[i-1];
            tek.size += pret.size;
            pret.size = 0;
            pret.top = &tek;
            if (tek.size > maxDuz)
                maxDuz = tek.size;
        }
        else if (mapa.find(i+1) != mapa.end())
        {
            Node &vrh = (*mapa[i+1].top);
            vrh.size += tek.size;
            tek.size = 0;
            tek.top = &vrh;
            if (vrh.size > maxDuz)
                maxDuz = vrh.size;
        }
    }
    for (unordered_map<int,Node>::iterator it = mapa.begin(); it != mapa.end(); it++)
    {
        delete (Node*)(&(*it).second);
    }
    return maxDuz;
}