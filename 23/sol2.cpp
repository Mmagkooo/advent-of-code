#include <cstdio>
#include <iostream>
#include <vector>
#include <cmath>
#include <set>
using namespace std;

typedef long long ll;

inline int dist(const auto& n1, const auto& n2) {
    return abs(n1.x-n2.x) + abs(n1.y-n2.y) + abs(n1.z-n2.z);
}

struct point {
    int x, y, z;
    point(int x, int y, int z) : x(x), y(y), z(z) {}
};

struct nanobot {
    int x, y, z, r;
    nanobot(int x, int y, int z, int r) : x(x), y(y), z(z), r(r) {}
    
    bool contains(const point &p) const {
        return dist(*this, p) <= this->r;
    }
};

bool operator<(const nanobot& n1, const nanobot& n2) {
    if (n1.x != n2.x) return n1.x < n2.x;
    if (n1.y != n2.y) return n1.y < n2.y;
    if (n1.z != n2.z) return n1.z < n2.z;
    return n1.r < n2.r;
}

bool operator==(const nanobot& n1, const nanobot& n2) {
    return n1.x == n2.x && n1.y == n2.y && n1.z == n2.z && n1.r == n2.r;
}

ostream& operator<<(ostream& os, nanobot& n) {
    os << "<" << n.x << "," << n.y << "," << n.z << ">, r=" << n.r;
    return os;
}

inline bool intersect(const nanobot &n1, const nanobot &n2) {
    return dist(n1, n2) <= n1.r + n2.r;
}

inline bool intersect(const set<nanobot> &nc, const nanobot &n2) {
    for (const nanobot& n1: nc) {
        if (!intersect(n1, n2)) {
            return false;
        }
    }
    return true;
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
    
    vector<set<nanobot>> nanobot_collections;
    for (const nanobot &n : nanobots) {
        set<nanobot> s {n};
        nanobot_collections.push_back(s);
    }
    
    int max_size = 1;
    int max_i = -1;
    for (int i = 0; i < nanobot_collections.size(); ++i) {
        set<nanobot> &nc = nanobot_collections[i];
        for (const nanobot &n : nanobots) {
            if (nc.count(n) == 0 && intersect(nc, n)) {
                nc.insert(n);
                int curr_size = nc.size();
                if (curr_size > max_size) {
                    max_size = curr_size;
                    max_i = i;
                }
            }
        }
    }
    printf("size of max intersection: %d\n", max_size);
    
    /**
        Ovaj postupak pretpostavlja da je tražena točka jedan od vrhova manhattan sfera.
        To ne mora biti slučaj (npr. ako pričamo o 2D: sjecište dva pravokutnika; pravokutnik je u općem slučaju sjecište dva kvadrata)
    */
    int max_contained_cnt = 0;
    int deltas[] = {-1, 0, 1};
    for (const auto &n: nanobot_collections[max_i]) {
        for (const auto &dx: deltas) {
            for (const auto &dy: deltas) {
                for (const auto &dz: deltas) {
                    bool contained_in_all = true;
                    point p(n.x + dx*n.r, n.y + dy*n.r, n.z + dz*n.r);
                    int contained_cnt = 0;
                    for (const auto &other_n : nanobot_collections[max_i]) {                            
                        if (other_n.contains(p)) {
                            ++contained_cnt;
                            // contained_in_all = false;
                            // break;
                        }
                    }
                    
                    max_contained_cnt = max(contained_cnt, max_contained_cnt);
                    // if (contained_in_all) {
                        // printf("<%d,%d,%d> -> %d\n", p.x, p.y, p.z, contained_cnt);
                    // }
                }
            }
        }
    }
    printf("max_contained_cnt: %d\n", max_contained_cnt);
}