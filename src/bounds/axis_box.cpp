#include "axis_box.h"
#include <cmath>
#include <cstdio>

AxisBox::AxisBox()
{
  this->min = Point(0, 0, 0);
  this->max = Point(0, 0, 0);
}

AxisBox::AxisBox(const Point& min, const Point& max)
{
  this->min = min;
  this->max = max;
}

bool
AxisBox::hit(const Ray& ray, float tMin, float tMax) const
{
  for (int i = 0; i < 3; ++i) {
    float inverseDirection = 1.0 / ray.direction[i];

    float t0 = (min[i] - ray.origin[i]) * inverseDirection;
    float t1 = (max[i] - ray.origin[i]) * inverseDirection;

    if (inverseDirection < 0) {
      std::swap(t0, t1);
    }

    tMin = t0 > tMin ? t0 : tMin;
    tMax = t1 < tMax ? t1 : tMax;

    if (tMax <= tMin) {
      return false;
    }
  }

  return true;
}

AxisBox
surroundingBox(const AxisBox& box0, const AxisBox& box1)
{
  Point min = Point(fmin(box0.min.x, box1.min.x),
                    fmin(box0.min.y, box1.min.y),
                    fmin(box0.min.z, box1.min.z));

  Point max = Point(fmax(box0.max.x, box1.max.x),
                    fmax(box0.max.y, box1.max.y),
                    fmax(box0.max.z, box1.max.z));

  return AxisBox(min, max);
}