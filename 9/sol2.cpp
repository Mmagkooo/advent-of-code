#include <iostream>
#include <list>
#include <algorithm>

using namespace std;

const int N = 411, M = 7105800, Q = 23;

list<int> l;

void printi(list<int>::iterator &it) {
    if (it == l.end()) cout << "end" << endl;
    else cout << *it << endl;
}

inline void move_right(list<int>::iterator &it, int i) {
    while (i--) {
        it++;
        if (it == l.end()) {
            it = l.begin();
        }
    }
    if (it == l.begin()) it = l.end();
}

inline void move_left(list<int>::iterator &it, int i) {
    while (i--) {
        if (it == l.begin()) {
            it = l.end();
        }
        it--;
    }
    if (it == l.end()) it = l.begin();
}

void printl() {
    for (auto& i : l) {
        cout << i << " ";
    }
    cout << endl;
}

int main() {
    int index = 0, player = -1;
    long long score[N] = {0};
    l.push_back(0);
    
    list<int>::iterator it = l.begin();
    
    for (int marble = 1; marble <= M; ++marble) {
        player = (player + 1) % N;
        if (marble % Q == 0) {
            score[player] += marble;
            move_left(it, 7);
            score[player] += *it;
            l.erase(it++);
        } else {
            move_right(it, 2);
            l.insert(it, marble);
            --it;
        }
    }
    
    cout << *max_element(score, score+N) << endl;
}