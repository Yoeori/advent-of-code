#include <fstream>
#include <iostream>
#include <numeric>

using namespace std;

void round(long long* lanternfish) {
    for (long long i = 0; i < 10; i++) {
        if (i == 0) {
            lanternfish[9] = lanternfish[i];
        } else {
            lanternfish[i - 1] = lanternfish[i];
        }
    }
    lanternfish[9] = 0;
    lanternfish[6] += lanternfish[8];
}

void day06() {
    fstream input("../input/day06.txt", std::ios_base::in);

    long long lanternfish[10] = {0};

    long long x;
    char c;
    while (input >> x) {
        lanternfish[x] += 1;
        input >> c;
    }

    for (long long i = 0; i < 80; i++) {
        round(lanternfish);
    }

    cout << "Answer to part 1: " << accumulate(begin(lanternfish), end(lanternfish), 0) << "\n";

    for (long long i = 0; i < 256 - 80; i++) {
        round(lanternfish);
    }

    cout << "Answer to part 2: " << accumulate(begin(lanternfish), end(lanternfish), 0) << "\n";
}