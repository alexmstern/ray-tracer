#ifndef CANVAS_H
#define CANVAS_H

#include "../primitives/color.h"
#include <iostream>
#include <fstream>

class Canvas {
public:
    int w, h;
    Color* pixels;
    Canvas(int width, int height);
    Color pixelAt(Canvas c, int x, int y);
    void writePixel(Canvas c, int x, int y, Color pixel);
    void canvasToPPM(std::ofstream& os, Canvas c);
};

Canvas :: Canvas(int width, int height) {
    w = width;
    h = height;
    pixels = (Color *) calloc(w*h, sizeof(Color));
}

Color Canvas :: pixelAt(Canvas c, int x, int y) {
    return c.pixels[x + y*c.w];
}

void Canvas :: writePixel(Canvas c, int x, int y, Color pixel) {
    if (((x >= 0) && (x < c.w)) && ((y >= 0) && (y < c.h))) {
        c.pixels[x + y*c.w] = pixel;
    }
}

void Canvas :: canvasToPPM(std::ofstream& os, Canvas c) {
    os << "P3" << std::endl
       << c.w << " " << c.h << std::endl
       << "255" << std::endl;
    for (int i = 0; i < c.w*c.h; i++) {
        os << clampColor(c.pixels[i].x) << " "
           << clampColor(c.pixels[i].y) << " "
           << clampColor(c.pixels[i].z) << std::endl;
    }
}

#endif