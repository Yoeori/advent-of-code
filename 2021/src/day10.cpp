#include <bits/stdc++.h>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include <set>

using namespace std;

long completion_points(char c) {
    switch (c) {
        case ')':
            return 1;
        case ']':
            return 2;
        case '}':
            return 3;
        case '>':
            return 4;
        case ' ':
            return 0;
    }
    throw invalid_argument("");
}

long expect(istream &stream, char c) {
    long score = 0;
    while (true) {
        char next = ' ';
        stream >> next;

        if(next == ' ') {
            return score * 5 + completion_points(c); // This line is incomplete, we build up a
        }

        switch(next) {
            case '[':
                score = expect(stream, ']');
                break;
            case '(':
                score = expect(stream, ')');
                break;
            case '{':
                score = expect(stream, '}');
                break;
            case '<':
                score = expect(stream, '>');
                break;
            default:
                if (next == c) {
                    return 0; // Completed
                } else {
                    // Failure
                    throw next;
                }
        }
    }
}

void day10() {
    fstream input("../input/day10.txt", ios_base::in);

    long broken_score = 0;
    vector<long> completion_scores = {};

    string line;
    while (getline(input, line, '\n')) {
        stringstream line_stream = stringstream(line);

        try {
            completion_scores.push_back(expect(line_stream, ' ') / 5);
        } catch (char c) {
            switch (c) {
                case ')':
                    broken_score += 3;
                    break;
                case ']':
                    broken_score += 57;
                    break;
                case '}':
                    broken_score += 1197;
                    break;
                case '>':
                    broken_score += 25137;
                    break;
            }
        }
    }

    cout << "Answer to part 1: " << broken_score << "\n";

    sort(completion_scores.begin(), completion_scores.end());
    cout << "Answer to part 2: " << completion_scores[completion_scores.size() / 2] << "\n";
}