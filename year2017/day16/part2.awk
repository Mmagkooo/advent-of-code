#!/usr/bin/awk -f

# ./part2.awk [-v PROGRAMS=<STR>] [-v ITERATIONS=<N>] [FILENAME]
# omitting the vars yields the task result

BEGIN{
    iterations = ITERATIONS ? ITERATIONS : 1e9;
    programs = PROGRAMS ? PROGRAMS : "abcdefghijklmnop";
}

END{
    i = 0;
    while (!(programs in seen)) {
        seen[programs] = 1;
        i_to_program[i++] = programs;
        cmd = "./part1.awk -v PROGRAMS=" programs " " FILENAME;
        cmd | getline programs;
        close(cmd);
    }
    
    period = i;
    print i_to_program[iterations % period];
}


