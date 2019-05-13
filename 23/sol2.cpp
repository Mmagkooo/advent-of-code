#include <cstdio>
#include <iostream>
#include <vector>
#include <cmath>
using namespace std;

typedef long long ll;

struct nanobot {
    int x, y, z, r;
    nanobot(int x, int y, int z, int r) : x(x), y(y), z(z), r(r) {}
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
    int x, y, z, r;
    vector<nanobot> nanobots;
    while (getline(cin, line)) {
        sscanf(line.c_str(), "pos=<%d,%d,%d>, r=%d", &x, &y, &z, &r);
        nanobot n(x, y, z, r);
        nanobots.push_back(n);
    }
    
    int minx = 1e9, maxx = -1e9, miny = 1e9, maxy = -1e9, minz = 1e9, maxz = -1e9;
    for (nanobot& n: nanobots) {
        minx = min(minx, n.x);
        maxx = max(maxx, n.x);
        miny = min(miny, n.y);
        maxy = max(maxy, n.y);
        minz = min(minz, n.z);
        maxz = max(maxz, n.z);
    }
    
    cout << "minx, maxx: " << minx << ", " << maxx << endl;
    cout << "miny, maxy: " << miny << ", " << maxy << endl;
    cout << "minz, maxz: " << minz << ", " << maxz << endl;
}