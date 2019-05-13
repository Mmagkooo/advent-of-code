#include <stdio.h>
#include <stdlib.h>



int main(int argc, char argv[][]) {
    if (argc != 4) {
        printf("<DEPTH> <TARGET_X> <TARGET_Y>");
        return 1;
    }
    
    int depth = atoi(argv[1]), Tx = atoi(argv[2]), Ty = atoi(argv[3]);
    
}