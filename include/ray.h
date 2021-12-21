#ifndef RAY_H
#define RAY_H

#include "point.h"

class Ray
{
public:
  Ray();
  Ray(const Point& origin, const Vec& direction);

  Point origin;
  Vec direction;

  Point at(float t) const;
};

#endif