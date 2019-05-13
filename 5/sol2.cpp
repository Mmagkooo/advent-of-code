#include <list>
#include <string>
#include <iostream>
#include <cmath>
#include <iterator>

using namespace std;

const int diff = 'a' - 'A';

int main() {
    
    string s; cin >> s;
    
    int sol = s.length();
    
    for(char cc = 'A'; cc <= 'Z'; ++cc) {
        list<char> l;
        for (char c: s) {
            if (c == cc || c == cc+diff) continue;
            l.push_back(c);
        }
        
        cout << "ok" << endl;
        
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
        
        sol = min(sol, (int) l.size());
    }

    cout << "SOL=" << sol << endl;
}