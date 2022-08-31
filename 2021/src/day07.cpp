#include <bits/stdc++.h>
#include <climits>
#include <fstream>
#include <iostream>
#include <numeric>
#include <vector>
#include <numeric>

using namespace std;

long long total_fuel(long long *lut, vector<long long> &crabs, long long mid) {
    long long total = 0;
    for (long long crab : crabs) {
        total += lut[abs(crab - mid)];
    }
    return total;
}

void day07() {
    fstream input("../input/day07.txt", std::ios_base::in);

    vector<long long> crabs;
    long long max_v = 0;

    char c;
    long long x;
    while (input >> x) {
        crabs.push_back(x);
        max_v = max(x, max_v);
        input >> c;
    }

    sort(crabs.begin(), crabs.end());

    long long mid = crabs[crabs.size() / 2];
    long long total = 0;
    for (long long crab : crabs) {
        total += abs(crab - mid);
    }

    cout << "Answer to part 1: " << total << "\n";

    // Mean would also work, but this is easier
    long long* lut = new long long[max_v];
    lut[0] = 0;
    for (long long i = 1; i < max_v; i++) {
        lut[i] = i + lut[i - 1];
    }

    long long min_fuel = LONG_LONG_MAX;
    for (long long i = 0; i < max_v; i++) {
        min_fuel = min(min_fuel, total_fuel(lut, crabs, i));
    }

    cout << "Answer to part 2: " << min_fuel << "\n";

    delete[] lut;
}