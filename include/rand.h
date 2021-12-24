#ifndef RAND_H
#define RAND_H

#include "vec.h"

float
randomFloat();

float
randomFloat(float min, float max);

float
jitter(float f);

Vec
randomUnitVector();

Vec
randomInUnitSphere();

Vec
randomInUnitDisk();

Color
randomColor();

int randomInt(int min, int max);

#endif