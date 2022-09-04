#include <bits/stdc++.h>
#include <fstream>
#include <vector>
#include <set>

using namespace std;

int basin_size(const vector<vector<int>> map, array<int, 2> loc) {
    set<array<int, 2>> found;
    set<array<int, 2>> expand;
    expand.insert(loc);
    found.insert(loc);

    while (!expand.empty()) {
        // We expand to the four neighbours if position exists, not in our found set and it is not 9
        set<array<int, 2>> expand_new;
        for (auto lc : expand) {
            if (lc[0] - 1 >= 0 && found.find({lc[0] - 1, lc[1]}) == found.end() && map[lc[0] - 1][lc[1]] != 9) {
                expand_new.insert({lc[0] - 1, lc[1]});
                found.insert({lc[0] - 1, lc[1]});
            }

            if (lc[1] - 1 >= 0 && found.find({lc[0], lc[1] - 1}) == found.end() && map[lc[0]][lc[1] - 1] != 9) {
                expand_new.insert({lc[0], lc[1] - 1});
                found.insert({lc[0], lc[1] - 1});
            }

            if (lc[0] + 1 < map.size() && found.find({lc[0] + 1, lc[1]}) == found.end() &&
                map[lc[0] + 1][lc[1]] != 9) {
                expand_new.insert({lc[0] + 1, lc[1]});
                found.insert({lc[0] + 1, lc[1]});
            }

            if (lc[1] + 1 < map[lc[0]].size() && found.find({lc[0], lc[1] + 1}) == found.end() &&
                map[lc[0]][lc[1] + 1] != 9) {
                expand_new.insert({lc[0], lc[1] + 1});
                found.insert({lc[0], lc[1] + 1});
            }
        }

        expand = expand_new;
    }

    return found.size();
}

void day09() {
    fstream input("../input/day09.txt", ios_base::in);
    vector<vector<int>> map = {};

    string row;
    while (getline(input, row, '\n')) {
        map.push_back({});
        for (char c : row) {
            map.back().push_back((int)c - 48);
        }
    }

    int local_minima_count = 0;
    vector<array<int, 2>> local_minimas = {};

    for (int x = 0; x < map.size(); x++) {
        for (int y = 0; y < map[x].size(); y++) {
            int v = map[x][y];

            if (((x - 1) < 0 || map[x - 1][y] > v) && ((y - 1) < 0 || map[x][y - 1] > v) &&
                ((x + 1) >= map.size() || map[x + 1][y] > v) && ((y + 1) >= map[x].size() || map[x][y + 1] > v)) {
                local_minima_count += v + 1;
                local_minimas.push_back({x, y});
            }
        }
    }

    cout << "Answer to part 1: " << local_minima_count << "\n";

    vector<int> local_minima_size = {};
    for (auto minima : local_minimas) {
        local_minima_size.push_back(basin_size(map, minima));
    }

    sort(local_minima_size.begin(), local_minima_size.end(), greater<int>());

    cout << "Answer to part 2: " << local_minima_size[0] * local_minima_size[1] * local_minima_size[2] << "\n";
}