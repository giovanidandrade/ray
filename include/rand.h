#ifndef RAND_H
#define RAND_H

#include "vec.h"

float
randomFloat();

float
jitter(float f);

Vec
randomUnitVector();

Vec
randomInUnitSphere();

Vec
randomInUnitDisk();

#endif