#ifndef SPHERE_H
#define SPHERE_H

#include "observable.h"

class Sphere : public Observable
{
public:
  Sphere(const Point& center, float radius, std::shared_ptr<Material> material);

  virtual std::optional<Observation> hit(const Ray& ray,
                                         float tMin,
                                         float tMax) const override;

private:
  Point center;
  float radius;
  std::shared_ptr<Material> material;
};

#endif