#include <iostream>
#include <string>

#include "day01.cpp"
#include "day02.cpp"

using namespace std;

typedef void (*DayFunctions)();

int main(int argc, char *argv[]) {
    DayFunctions days[] = {day01, day02};

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