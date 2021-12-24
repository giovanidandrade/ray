#ifndef AXIS_BOX_H
#define AXIS_BOX_H

#include "point.h"
#include "ray.h"

class AxisBox
{
public:
  AxisBox();
  AxisBox(const Point& min, const Point& max);

  Point min, max;
  bool hit(const Ray& ray, float tMin, float tMax) const;
};

AxisBox
surroundingBox(const AxisBox& box0, const AxisBox& box1);

#endif