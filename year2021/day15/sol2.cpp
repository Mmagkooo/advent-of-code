#include <fstream>
#include <iostream>
#include <vector>
#include <set>
#include <map>

using namespace std;

typedef pair<int, int> pii;

const int N_ADJ = 4;
const int adj[N_ADJ][2] = {
    {-1, 0},
    {1, 0},
    {0, -1},
    {0, 1},
};

const int INF = 1e9;
int HEIGHT;
int WIDTH;

pii find_min(set<pii>& remaining, map<pii, int>& dists) {
    pii min_pos;
    int min_dist = INF;
    for (set<pii>::iterator it = remaining.begin(); it != remaining.end(); ++it) {
        pii pos = *it;
        int dist = dists[pos];
        if (dist < min_dist) {
            min_dist = dist;
            min_pos = pos;
        }
    }
    return min_pos;
}

vector<pii> get_remaining_adj(pii& pos, set<pii>& remaining) {
    vector<pii> ret;
    for (int i = 0; i < N_ADJ; ++i) {
        int ni = pos.first + adj[i][0];
        int nj = pos.second + adj[i][1];
        pii npos = make_pair(ni, nj);

        if (ni < 0 || ni >= HEIGHT) {
            continue;
        } else if (nj < 0 || nj >= WIDTH) {
            continue;
        } else if (!remaining.count(npos)) {
            continue;
        }
        ret.push_back(npos);
    }
    return ret;
}

int main() {
    string s;
    vector<vector<int>> field;
    set<pii> remaining;
    map<pii, int> dists;
    for (int i = 0; cin >> s; ++i) {
        vector<int> row;
        for (int j = 0; j < s.size(); ++j) {
            row.push_back(s[j] - '0');
            pii current_pos = make_pair(i, j);
            remaining.insert(current_pos);
            dists[current_pos] = INF;
        }
        field.push_back(row);
    }

    HEIGHT = field.size();
    WIDTH = field[0].size();

    pii start_pos = make_pair(0, 0);
    dists[start_pos] = 0;
    dists[make_pair(0, 1)] = field[0][1];
    dists[make_pair(1, 0)] = field[1][0];
    remaining.erase(start_pos);

    while (remaining.size()) {
        pii min_pos = find_min(remaining, dists);
        remaining.erase(min_pos);
        int min_pos_dist = dists[min_pos];
        vector<pii> adj_poss = get_remaining_adj(min_pos, remaining);
        for (int i = 0; i < adj_poss.size(); ++i) {
            pii adj_pos = adj_poss[i];
            int old_dist = dists[adj_pos];
            int new_dist = min_pos_dist + field[adj_pos.first][adj_pos.second];
            if (new_dist < old_dist) {
                dists[adj_pos] = new_dist;
            }
        }
    }

    pii last_pos = make_pair(HEIGHT-1, WIDTH-1);
    cout << dists[last_pos] << endl;
}
