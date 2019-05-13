R2=1;
do {
    R1=1;
    do {
        if (R2*R1 == R3) {
            R0 += R2;
        }
        R1++;
    } while(R1 <= R3);

    R2++;
} while(R2 <= R1);

R