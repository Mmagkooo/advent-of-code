#include <cstdio>
#include <cstdlib>
using namespace std;

const int ROCKY = 0;
const int WET = 1;
const int NARROW = 2;

const int MOD = 20183;
const int INF = 1e9;

const int MAX_X = 1000;
const int MAX_Y = 1000;

int xlim, ylim;

int dp[MAX_X][MAX_Y];
int field[MAX_X][MAX_Y];
int memo[MAX_X][MAX_Y][2][2];

//int rec = 0;
//int cnt = 0;

/**
    ovo je krivo, ne smiju bit i torch i gear, treba drukčije radit prijelaze:
    ne smije se dogodit prijelaz s bakljom iz rocky područja u wet područje bez baklje
    pogledati sol2nr.cpp
*/
void rek(int x, int y, int t, bool torch, bool gear) {
    if (x < 0 or y < 0 or x >= xlim or y >= ylim)
        return;
    int region = field[x][y];
    if (ROCKY == region and not torch and not gear)
        return;
    if (WET == region and torch)
        return;
    if (NARROW == region and gear)
        return;
    if (t >= memo[x][y][torch][gear])
        return;
    
    //if (++cnt > rec) {
     //   printf("%d\n", cnt);
      //  rec = cnt;
    //}
    
    memo[x][y][torch][gear] = t;
    
    rek(x-1, y, t+1, torch, gear);
    rek(x-1, y, t+8, not torch, gear);
    rek(x-1, y, t+8, torch, not gear);
    rek(x-1, y, t+8, not torch, not gear);
    
    rek(x+1, y, t+1, torch, gear);
    rek(x+1, y, t+8, not torch, gear);
    rek(x+1, y, t+8, torch, not gear);
    rek(x+1, y, t+8, not torch, not gear);
    
    rek(x, y-1, t+1, torch, gear);
    rek(x, y-1, t+8, not torch, gear);
    rek(x, y-1, t+8, torch, not gear);
    rek(x, y-1, t+8, not torch, not gear);
    
    rek(x, y+1, t+1, torch, gear);
    rek(x, y+1, t+8, not torch, gear);
    rek(x, y+1, t+8, torch, not gear);
    rek(x, y+1, t+8, not torch, not gear);
    
    //--cnt;
}

int main(int argc, char** argv){
    if (argc != 4) {
        printf("<DEPTH> <TARGET_X> <TARGET_Y>");
        return 1;
    }
    
    for (int x = 0; x < MAX_X; ++x)
        for (int y = 0; y < MAX_Y; ++y)
            for (int torch = 0; torch < 2; ++torch)
                for (int gear = 0; gear < 2; ++gear)
                    memo[x][y][torch][gear] = INF;
    
    int DEPTH = atoi(argv[1]), Tx = atoi(argv[2]), Ty = atoi(argv[3]);
    
    xlim = Tx+1+50;
    ylim = Ty+1+50;

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
    printf("%d %d\n", memo[Tx][Ty][true][false], memo[Tx][Ty][true][true]);
}