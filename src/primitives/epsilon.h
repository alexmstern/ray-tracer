#ifndef EPSILON_H
#define EPSILON_H

#include <cstdlib>

#define EPSILON 0.00001
bool equal(float a, float b) {return abs(a - b) < EPSILON;}

#endif