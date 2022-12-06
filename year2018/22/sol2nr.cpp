#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <vector>
#include <queue>
using namespace std;

const int ROCKY = 0;
const int WET = 1;
const int NARROW = 2;

const int MOD = 20183;
const int INF = 1e9;

const int MAX_X = 1000;
const int MAX_Y = 1000;

int xlim, ylim, Tx, Ty;

int dp[MAX_X][MAX_Y];
int field[MAX_X][MAX_Y];
int memo[MAX_X][MAX_Y][2][2];

struct args {
    int x, y, t;
    bool torch, gear;
    args(int x, int y, int t, bool torch, bool gear): x(x), y(y), t(t), torch(torch), gear(gear) {}
};

void rek(int x, int y, int t, bool torch, bool gear) {
    queue<args> container;
    container.push(args(x, y, t, torch, gear));
    while (not container.empty()) {
        args tmp = container.front();
        container.pop();
        x = tmp.x; y = tmp.y; t = tmp.t; torch = tmp.torch; gear = tmp.gear;
        if (x < 0 or y < 0 or x >= xlim or y >= ylim)
            continue;
        if (torch and gear)
            continue;
        int region = field[x][y];
        if (ROCKY == region and not torch and not gear)
            continue;
        if (WET == region and torch)
            continue;
        if (NARROW == region and gear)
            continue;
        if (t >= memo[x][y][torch][gear])
            continue;
        
        memo[x][y][torch][gear] = t;
        
        if (x == Tx and y == Ty)
            continue;
        
        container.push(args(x, y, t+7, not torch, gear));
        container.push(args(x, y, t+7, torch, not gear));
        container.push(args(x, y, t+7, not torch, not gear));
        
        container.push(args(x-1, y, t+1, torch, gear));
        container.push(args(x+1, y, t+1, torch, gear));
        container.push(args(x, y-1, t+1, torch, gear));
        container.push(args(x, y+1, t+1, torch, gear));
    }
}

int main(int argc, char** argv){
    time_t start = time(0);
    if (argc != 6) {
        printf("<DEPTH> <TARGET_X> <TARGET_Y> <EXPAND_X> <EXPAND_Y>");
        return 1;
    }
    
    for (int x = 0; x < MAX_X; ++x)
        for (int y = 0; y < MAX_Y; ++y)
            for (int torch = 0; torch < 2; ++torch)
                for (int gear = 0; gear < 2; ++gear)
                    memo[x][y][torch][gear] = INF;
    
    int DEPTH = atoi(argv[1]);
    Tx = atoi(argv[2]), Ty = atoi(argv[3]);
    int Ex = atoi(argv[4]), Ey = atoi(argv[5]);
    
    xlim = Tx+1+Ex;
    ylim = Ty+1+Ey;

    for (int x = 0; x < xlim; ++x) {
        dp[x][0] = (16807*x + DEPTH) % MOD;
        field[x][0] = dp[x][0] % 3;
    }
    for (int y = 0; y < ylim; ++y) {
        dp[0][y] = (48271*y + DEPTH) % MOD;
        field[0][y] = dp[0][y] % 3;
    }

    for (int x = 1; x < xlim; ++x) {
        for (int y = 1; y < ylim; ++y) {
            if (x == Tx and y == Ty) {
                dp[x][y] = DEPTH % MOD;
                field[x][y] = DEPTH % MOD % 3;
            } else {                
                dp[x][y] = (dp[x-1][y]*dp[x][y-1] + DEPTH) % MOD;
                field[x][y] = dp[x][y] % 3;
            }
        }
    }

    dp[0][0] = dp[Tx][Ty] = DEPTH % MOD;
    field[0][0] = field[Tx][Ty] = DEPTH % MOD % 3;

    rek(0, 0, 0, true, false);
    printf("%d\n", memo[Tx][Ty][true][false]);
    printf("time=%d\n", time(0) - start);
}