#ifndef POINT_H
#define POINT_H

#include "vec.h"

class Point
{
public:
  Point();
  Point(float x, float y, float z);

  float x, y, z;

  float distance(const Point& p) const;

  Point operator+(const Vec& v) const;
  Point operator-(const Vec& v) const;

  Vec operator-(const Point& p) const;

  float operator[](int idx) const;
};

#endif