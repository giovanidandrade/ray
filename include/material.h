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

#endif