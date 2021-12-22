#ifndef OBSERVABLE_H
#define OBSERVABLE_H

#include "point.h"
#include "ray.h"
#include <optional>

struct Observation
{
  Point point;
  Vec normal;
  float t;
  bool frontFacing;

  inline void setFace(const Ray& ray, const Vec& outwardNormal)
  {
    frontFacing = ray.direction.dot(outwardNormal) < 0;
    normal = frontFacing ? outwardNormal : -outwardNormal;
  }
};

class Observable
{
public:
  virtual std::optional<Observation> hit(const Ray& ray,
                                         float tMin,
                                         float tMax) const = 0;
};

#endif