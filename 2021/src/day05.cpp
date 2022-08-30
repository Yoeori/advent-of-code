#include <algorithm>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <sstream>
#include <tuple>
#include <vector>

using namespace std;

struct Point {
    int x;
    int y;
    Point(int x, int y) : x(x), y(y) {}
};

struct LineSegment {
    Point p1;
    Point p2;

    LineSegment(int x0, int y0, int x1, int y1): p1(Point(x0, y0)), p2(Point(x1, y1)) {}

    bool is_horizontal() { return p1.y == p2.y; }

    bool is_vertical() { return p1.x == p2.x; }

    vector<Point> points() {
        vector<Point> points;
        if (is_horizontal()) {
            for (int x = min(p1.x, p2.x); x <= max(p1.x, p2.x); x++) {
                points.push_back(Point(x, p1.y));
            }
        } else if(is_vertical()) {
            for (int y = min(p1.y, p2.y); y <= max(p1.y, p2.y); y++) {
                points.push_back(Point(p1.x, y));
            }
        } else {
            // assume: diagonal
            int x, y;
            bool dir;

            if (p1.x > p2.x) {
                x = p2.x;
                y = p2.y;

                dir = p1.y > p2.y;
            } else {
                x = p1.x;
                y = p1.y;

                dir = p2.y > p1.y;
            }

            for (; x <= max(p1.x, p2.x); x++) {
                points.push_back(Point(x, y));
                
                if (dir) {
                    y++;
                } else {
                    y--;
                }
            }
        }
        return points;
    }
};

void day05() {
    fstream input("../input/day05.txt", std::ios_base::in);

    vector<LineSegment> segments;
    int max_x = 0;
    int max_y = 0;

    int x0,y0,x1,y1;
    char c;
    while (input >> x0) {
        input >> c >> y0 >> c >> c >> x1 >> c >> y1;
        max_x = max({max_x, x0, x1});
        max_y = max({max_y, y0, y1});
        segments.push_back(LineSegment(x0, y0, x1, y1));
    }

    max_x += 1;
    max_y += 1;

    int* grid = new int[max_x * max_y]();
    for (LineSegment &segment : segments) {
        if (!(segment.is_horizontal() || segment.is_vertical())) {
            continue;
        }
        for (Point point : segment.points()) {
            grid[max_x * point.y + point.x] += 1;
        }
    }

    int count = 0;
    for (int i = 0; i < max_x * max_y; i++) {
        if (grid[i] > 1) {
            count++;
        }
    }

    cout << "Answer to part 1: " << count << "\n";

    for (LineSegment &segment : segments) {
        if (segment.is_horizontal() || segment.is_vertical()) {
            continue;
        }
        for (Point point : segment.points()) {
            grid[max_x * point.y + point.x] += 1;
        }
    }

    count = 0;
    for (int i = 0; i < max_x * max_y; i++) {
        if (grid[i] > 1) {
            count++;
        }
    }

    cout << "Answer to part 2: " << count << "\n";

    delete[] grid;
}