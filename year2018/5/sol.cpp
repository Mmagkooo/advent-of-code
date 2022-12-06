#include <list>
#include <string>
#include <iostream>
#include <cmath>

using namespace std;

int main() {
    string s; cin >> s;
    
    list<char> l;
    for (char c: s) {
        l.push_back(c);
    }
    
    int diff = 'a' - 'A';
    
    list<char>::iterator i = l.begin();
    list<char>::iterator j = ++l.begin();
    
    while (j != l.end()) {
        if (abs(*i - *j) == diff) {
                if (i != l.begin()) {
                    l.erase(i--);
                    l.erase(j++);
                } else {
                    auto tmpi = next(i, 2);
                    auto tmpj = next(j, 2);
                    l.erase(i);
                    l.erase(j);
                    i = tmpi;
                    j = tmpj;
                }
            } else {
                i++;
                j++;
            }
    }
    
    cout << l.size() << endl;
}