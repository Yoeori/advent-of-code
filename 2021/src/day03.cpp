#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

void day03() {
    std::fstream input("../input/day03.txt", std::ios_base::in);

    vector<string> ratings;
    string line;
    while (input >> line) {
        ratings.push_back(line);
    }

    string gamma_string = "";
    string epsilon_string = "";

    for (int i = 0; i < ratings[0].length(); i++) {

        int a = 0;
        int b = 0;

        for (string l : ratings) {
            if (l.at(i) == '0') {
                a += 1;
            } else {
                b += 1;
            }
        }

        if (a > b) {
            gamma_string.append("0");
            epsilon_string.append("1");
        } else {
            gamma_string.append("1");
            epsilon_string.append("0");
        }
    }

    int gamma = stoi(gamma_string, 0, 2);
    int epsilon = stoi(epsilon_string, 0, 2);

    cout << "Answer to part 1: " << gamma * epsilon << "\n";

    vector<string> ratings_oxygen(ratings);

    // We filter based on gamma_string
    for (int i = 0; i < ratings[0].length() && ratings_oxygen.size() > 1; i++) {

        int a = 0;
        int b = 0;

        for (string l : ratings_oxygen) {
            if (l.at(i) == '0') {
                a += 1;
            } else {
                b += 1;
            }
        }

        ratings_oxygen.erase(
            std::remove_if(ratings_oxygen.begin(), ratings_oxygen.end(),
                           [&i, &a, &b](const string &val) { return val.at(i) != (a > b ? '0' : '1'); }),
            ratings_oxygen.end());
    }

    vector<string> ratings_co2(ratings);

    for (int i = 0; i < ratings[0].length() && ratings_co2.size() > 1; i++) {

        int a = 0;
        int b = 0;

        for (string l : ratings_co2) {
            if (l.at(i) == '0') {
                a += 1;
            } else {
                b += 1;
            }
        }

        ratings_co2.erase(
            std::remove_if(ratings_co2.begin(), ratings_co2.end(),
                           [&i, &a, &b](const string &val) { return val.at(i) != (b < a ? '1' : '0'); }),
            ratings_co2.end());
    }
    
    int oxygen = stoi(ratings_oxygen[0], 0, 2);
    int co2 = stoi(ratings_co2[0], 0, 2);

    cout << "Answer to part 2: " << oxygen * co2 << "\n";
}