#ifndef MATERIAL_H
#define MATERIAL_H

#include "color.h"
#include "ray.h"
#include <optional>
#include <utility>

struct Observation;

using Pair = std::pair<Ray, Color>;

class Material
{
public:
  virtual std::optional<Pair> scatter(const Ray& ray,
                                      const Observation& obs) const = 0;
};

Vec
refract(const Vec& ray, const Vec& normal, float ratio);

Vec
reflect(const Vec& ray, const Vec& normal);

#endif