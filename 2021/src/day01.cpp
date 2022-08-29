#include <bits/stdc++.h>
#include <fstream>
#include <iostream>

using namespace std;

void day01() {
    std::fstream input("../input/day01.txt", std::ios_base::in);

    int count = 0;
    int last = INT_MAX;
    int t;
    while (input >> t) {
        if (t > last) {
            count++;
        }
        last = t;
    }

    cout << "Answer to part 1: " << count << "\n";

    std::fstream input1("../input/day01.txt", std::ios_base::in);
    std::fstream input2("../input/day01.txt", std::ios_base::in);

    count = 0;
    int prev_cur;
    int cur = 0;

    int a,b;
    for(int i = 0; i < 3; i++) {
      input1 >> a;
      prev_cur += a;
    }
    
    cur = prev_cur;

    while (input1 >> a) {
        input2 >> b;
        cur -= b;
        cur += a;
        if (cur > prev_cur) {
            count += 1;
        }
        prev_cur = cur;
    }

    cout << "Answer to part 2: " << count << "\n";
}