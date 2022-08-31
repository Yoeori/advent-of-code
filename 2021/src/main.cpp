#include <iostream>
#include <string>

#include "day01.cpp"
#include "day02.cpp"
#include "day03.cpp"
#include "day04.cpp"
#include "day05.cpp"
#include "day06.cpp"
#include "day07.cpp"

using namespace std;

typedef void (*DayFunctions)();

int main(int argc, char *argv[]) {
    DayFunctions days[] = {day01, day02, day03, day04, day05, day06, day07};

    if (argc >= 2) {
        int day;
        try {
            day = stoi(argv[1]);
        } catch (const exception &e) {
          day = 0;
        }
        
        if (day >= 1 && day <= (sizeof(days) / sizeof(days[0]))) {
            days[day - 1]();
        } else {
            cout << "No valid day was given\n";
            return 1;
        }
    } else {
        days[(sizeof(days) / sizeof(days[0])) - 1]();
    }    

    return 0;
}