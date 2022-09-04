#include <bits/stdc++.h>
#include <climits>
#include <cstring>
#include <exception>
#include <fstream>
#include <iostream>
#include <numeric>
#include <stdexcept>
#include <vector>

using namespace std;

struct SingleSegment {
    bool segments[7];
    string inp;

    SingleSegment(string inp): inp(inp) {
        for (int i = 0; i < 7; i++) {
            segments[i] = false;
        }
        for (char &c : inp) {
            segments[int(c) - 97] = true;
        }
    }

    SingleSegment() {
        for (int i = 0; i < 7; i++) {
            segments[i] = false;
        }
    }

    SingleSegment operator - (SingleSegment const &other) {
        SingleSegment res = SingleSegment();
        memcpy(&res, &segments, 7);
        for (int i = 0; i < 7; i++) {
            if (other.segments[i]) {
                res.segments[i] = false;
            }
        }
        return res;
    }

    bool eq(SingleSegment const &other) {
        for (int i = 0; i < 7; i++) {
            if (!other.segments[i] != !segments[i]) {
                return false;
            }
        }
        return true;
    }

    int len() {
        int count = 0;
        for (auto segment : segments) {
            count += segment ? 1 : 0;
        }
        return count;
    }
};

struct Entry {
    SingleSegment signals[10];
    SingleSegment outputs[4];

    Entry(fstream &stream) {
        char c;
        string s;
        SingleSegment tmp_segment;
        vector<SingleSegment> tmp = {};

        for (int i = 0; i < 10; i++) {
            stream >> s;

            tmp_segment = SingleSegment(s);

            if (tmp_segment.len() == 2) { // One
                signals[1] = tmp_segment;
            } else if (tmp_segment.len() == 3) { // Seven
                signals[7] = tmp_segment;
            } else if (tmp_segment.len() == 4) { // Four
                signals[4] = tmp_segment;
            } else if (tmp_segment.len() == 7) { // Eight
                signals[8] = tmp_segment;
            } else {
                tmp.push_back(tmp_segment);
            }
            
        }

        // We should now have the number 1, 4, 7 & 8;
        // Use those to differentiate between the other numbers

        for (auto seg : tmp) {
            auto minusfour = seg - signals[4];
            if(seg.len() == 6 && (seg - signals[1]).len() == 5) { // Six
                signals[6] = seg;
            } else if (seg.len() == 6 &&  (seg - signals[4]).len() == 2) { // Nine
                signals[9] = seg;
            } else if (seg.len() == 6) { // Zero
                signals[0] = seg;
            }
        }

        for (auto seg : tmp) {
            if (seg.len() == 5 && (seg - signals[6]).len() == 0) { // Five
                signals[5] = seg;
            } else if (seg.len() == 5 && (seg - signals[4]).len() == 2) { // Three
                signals[3] = seg;
            }  else if (seg.len() == 5) { // Two
                signals[2] = seg;
            }
        }

        stream >> c;

        for (int i = 0; i < 4; i++) {
            stream >> s;
            outputs[i] = SingleSegment(s);
        }
    }

    int simple_count() {
        int count = 0;
        for (auto &out : outputs) {
            if (out.len() == 2 || out.len() == 4 || out.len() == 3 || out.len() == 7) {
                count += 1;
            }
        }
        return count;
    }

    int from_segment(SingleSegment &seg) {
        for (int i = 0; i < 10; i++) {
            if(signals[i].eq(seg)) {
                return i;
            }
        }
        throw std::bad_exception();
    }

    int complex_count() {
        int total = 0;
        for (auto &out : outputs) {
            total *= 10;
            total += from_segment(out);
        }
        return total;
    }
};

void day08() {
    fstream input("../input/day08.txt", ios_base::in);

    vector<Entry> entries = {};

    while (input.peek() != EOF) {
        entries.push_back(Entry(input));
    }

    int total = 0;
    for (Entry &entry : entries) {
        total += entry.simple_count();
    }

    cout << "Answer to part 1: " << total << "\n";

    total = 0;
    for (Entry &entry : entries) {
        total += entry.complex_count();
    }

    cout << "Answer to part 2: " << total << "\n";
}