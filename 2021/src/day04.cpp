#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <tuple>
#include <vector>

using namespace std;

#define BINGO_SIZE 5

struct BingoSheet {
    int values[BINGO_SIZE * BINGO_SIZE];
    bool marked[BINGO_SIZE * BINGO_SIZE];
    bool won;

    BingoSheet(fstream &stream) : marked{false}, won(false) {
        for (int i = 0; i < BINGO_SIZE * BINGO_SIZE; i++) {
            stream >> values[i];
        }
    }

    bool mark(int value) {
        if (won) {
            return false;
        }

        for (int i = 0; i < BINGO_SIZE * BINGO_SIZE; i++) {
            if (values[i] == value) {
                marked[i] = true;
            }
        }

        if (has_bingo()) {
            won = true;
            return true;
        } else {
            return false;
        }
    }

    int score() {
        int score = 0;
        for (int i = 0; i < BINGO_SIZE * BINGO_SIZE; i++) {
            if (!marked[i]) {
                score += values[i];
            }
        }
        return score;
    }

    private:
    bool has_bingo() {
        // Check rows:
        for (int x = 0; x < BINGO_SIZE; x++) {
            bool found_negative = false;
            for (int y = 0; y < BINGO_SIZE; y++) {
                if (!marked[x * BINGO_SIZE + y]) {
                    found_negative = true;
                }
            }

            if (!found_negative) {
                return true;
            }
        }

        // Check cols
        for (int y = 0; y < BINGO_SIZE; y++) {
            bool found_negative = false;
            for (int x = 0; x < BINGO_SIZE; x++) {
                if (!marked[x * BINGO_SIZE + y]) {
                    found_negative = true;
                }
            }

            if (!found_negative) {
                return true;
            }
        }

        return false;
    }
};

void day04() {
    fstream input("../input/day04.txt", std::ios_base::in);

    vector<int> draws = {};
    string segment;

    string firstline;
    input >> firstline;

    stringstream firstlines(firstline);
    while (getline(firstlines, segment, ',')) {
        draws.push_back(stoi(segment));
    }

    vector<BingoSheet> sheets = {};

    while (input.peek() != EOF) {
        sheets.push_back(BingoSheet(input));
    }

    int last_score;
    bool someone_has_won;

    for (int draw : draws) {
        for (BingoSheet &sheet : sheets) {
            if (sheet.mark(draw)) {
                last_score = draw * sheet.score();
                if (!someone_has_won)
                    cout << "Answer to part 1: " << last_score << "\n";
                someone_has_won = true;
            }
        }
    }

    cout << "Answer to part 2: " << last_score << "\n";
}