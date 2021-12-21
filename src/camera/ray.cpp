#include "ray.h"

Ray::Ray(const Point& origin, const Vec& direction)
{
  this->origin = origin;
  this->direction = direction;
}

Point
Ray::at(float t) const
{
  return origin + t * direction;
}