#include <cstdio>
#include <iostream>
#include <vector>
#include <cmath>
using namespace std;

typedef long long ll;

struct nanobot {
    ll x, y, z, r;
    nanobot(ll x, ll y, ll z, ll r) : x(x), y(y), z(z), r(r) {}
};

ostream& operator<<(ostream& os, nanobot& n) {
    os << "<" << n.x << "," << n.y << "," << n.z << ">, r=" << n.r;
    return os;
}

inline int dist(nanobot& n1, nanobot& n2) {
    return abs(n1.x-n2.x) + abs(n1.y-n2.y) + abs(n1.z-n2.z);
}

int main() {
    string line;
    ll x, y, z, r;
    vector<nanobot> nanobots;
    while (getline(cin, line)) {
        sscanf(line.c_str(), "pos=<%I64d,%I64d,%I64d>, r=%I64d", &x, &y, &z, &r);
        nanobot n(x, y, z, r);
        nanobots.push_back(n);
        //cout << n << endl;
    }
    
    int maxi, maxr = 0;
    for (int i = 0; i < nanobots.size(); ++i) {
        if (nanobots[i].r > maxr) {
            maxr = nanobots[i].r;
            maxi = i;
        }
    }
    
    nanobot maxnanobot = nanobots[maxi];
    
    cout << "i=" << maxi << "maxr=" << maxr << endl;
    
    int cnt = 0;
    for (nanobot& n: nanobots) {
        if (dist(maxnanobot, n) <= maxr) {
            ++cnt;
        }
    }
    
    printf("%d\n", cnt);
}