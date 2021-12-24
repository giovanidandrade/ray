#include "point.h"
#include <cstdlib>

Point::Point()
{
  this->x = 0;
  this->y = 0;
  this->z = 0;
}

Point::Point(float x, float y, float z)
{
  this->x = x;
  this->y = y;
  this->z = z;
}

float
Point::distance(const Point& p) const
{
  return ((*this) - p).len();
}

Point
Point::operator+(const Vec& v) const
{
  return Point(x + v.x, y + v.y, z + v.z);
}

Point
Point::operator-(const Vec& v) const
{
  return Point(x - v.x, y - v.y, z - v.z);
}

Vec
Point::operator-(const Point& p) const
{
  return Vec(x - p.x, y - p.y, z - p.z);
}

float
Point::operator[](int idx) const
{
  switch (idx) {
    case 0:
      return x;
    case 1:
      return y;
    case 2:
      return z;
  }

  exit(1);
}