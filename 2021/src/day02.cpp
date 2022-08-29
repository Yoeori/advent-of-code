#include <bits/stdc++.h>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

void day02() {
    std::fstream input("../input/day02.txt", std::ios_base::in);

    int depth = 0;
    int h_pos = 0;

    string dir;
    int length;

    while(input >> dir >> length) {
        if (dir == "forward") {
            h_pos += length;
        } else if (dir == "down") {
            depth += length;
        } else if (dir == "up") {
            depth -= length;
        }
    }

    cout << "Answer to part 1: " << depth * h_pos << "\n";

    input.clear();
    input.seekg(0);

    int aim = 0;
    h_pos = 0;
    depth = 0;

    while (input >> dir >> length) {
      if (dir == "forward") {
        h_pos += length;
        depth += aim * length;
      } else if (dir == "down") {
        aim += length;
      } else if (dir == "up") {
        aim -= length;
      }
    }

    cout << "Answer to part 2: " << depth * h_pos << "\n";
}